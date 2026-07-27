use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use bytes::Bytes;
use futures_util::StreamExt as _;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::{AppSettings, Error, Result, VoiceSettings};

const MAX_AUDIO_RESPONSE_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct SynthesisRequest {
    pub text: String,
    pub voice: VoiceSettings,
}

#[derive(Debug, Clone)]
pub struct AudioData {
    pub bytes: Bytes,
    pub content_type: String,
}

/// A human-readable voice exposed by a VOICEVOX-compatible engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VoiceOption {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct EngineSpeaker {
    name: String,
    styles: Vec<EngineStyle>,
}

#[derive(Debug, Deserialize)]
struct EngineStyle {
    id: i64,
    name: String,
}

fn voice_options(speakers: Vec<EngineSpeaker>) -> Vec<VoiceOption> {
    let mut voices = speakers
        .into_iter()
        .flat_map(|speaker| {
            speaker.styles.into_iter().map(move |style| VoiceOption {
                id: style.id,
                name: format!("{}（{}）", speaker.name, style.name),
            })
        })
        .collect::<Vec<_>>();
    voices.sort_by(|left, right| left.name.cmp(&right.name).then(left.id.cmp(&right.id)));
    voices
}

#[async_trait]
pub trait TtsProvider: Send + Sync {
    fn name(&self) -> &'static str;
    async fn healthcheck(&self) -> Result<String>;
    async fn voices(&self) -> Result<Vec<VoiceOption>>;
    async fn synthesize(&self, request: &SynthesisRequest) -> Result<AudioData>;
}

/// Client for VOICEVOX API-compatible engines, including `AivisSpeech`.
#[derive(Debug, Clone)]
pub struct VoicevoxCompatibleProvider {
    name: &'static str,
    endpoint: Url,
    client: Client,
}

impl VoicevoxCompatibleProvider {
    /// Creates a client for a VOICEVOX-compatible endpoint.
    ///
    /// # Errors
    ///
    /// Returns an error when the endpoint URL is malformed or the HTTP client cannot be built.
    pub fn new(name: &'static str, endpoint: &str) -> Result<Self> {
        let mut endpoint = Url::parse(endpoint)
            .map_err(|_| Error::InvalidSettings("TTSエンドポイントが正しくありません".into()))?;
        if !endpoint.path().ends_with('/') {
            endpoint.set_path(&format!("{}/", endpoint.path()));
        }
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(3))
            .timeout(Duration::from_secs(20))
            .build()?;
        Ok(Self { name, endpoint, client })
    }

    fn url(&self, path: &str) -> Result<Url> {
        self.endpoint
            .join(path)
            .map_err(|_| Error::InvalidSettings("TTS APIのURLを組み立てられません".into()))
    }
}

#[async_trait]
impl TtsProvider for VoicevoxCompatibleProvider {
    fn name(&self) -> &'static str {
        self.name
    }

    async fn healthcheck(&self) -> Result<String> {
        let response = self
            .client
            .get(self.url("version")?)
            .send()
            .await
            .map_err(|error| Error::ProviderUnavailable(error.to_string()))?;
        if !response.status().is_success() {
            return Err(Error::ProviderResponse(format!(
                "{}: HTTP {}",
                self.name,
                response.status()
            )));
        }
        let version = response.text().await?.trim_matches('"').to_owned();
        Ok(version)
    }

    async fn voices(&self) -> Result<Vec<VoiceOption>> {
        let speakers: Vec<EngineSpeaker> = self
            .client
            .get(self.url("speakers")?)
            .send()
            .await
            .map_err(|error| Error::ProviderUnavailable(error.to_string()))?
            .error_for_status()
            .map_err(|error| Error::ProviderResponse(error.to_string()))?
            .json()
            .await?;

        Ok(voice_options(speakers))
    }

    async fn synthesize(&self, request: &SynthesisRequest) -> Result<AudioData> {
        if request.text.trim().is_empty() {
            return Err(Error::EmptyText);
        }
        request.voice.validate()?;

        let speaker = request.voice.speaker_id.to_string();
        let mut query: Value = self
            .client
            .post(self.url("audio_query")?)
            .query(&[("speaker", speaker.as_str()), ("text", request.text.as_str())])
            .send()
            .await?
            .error_for_status()
            .map_err(|error| Error::ProviderResponse(error.to_string()))?
            .json()
            .await?;

        if let Some(object) = query.as_object_mut() {
            object.insert("speedScale".into(), request.voice.speed.into());
            object.insert("pitchScale".into(), request.voice.pitch.into());
            object.insert("intonationScale".into(), request.voice.intonation.into());
            object.insert("volumeScale".into(), request.voice.volume.into());
            object.insert("outputSamplingRate".into(), 48_000.into());
            object.insert("outputStereo".into(), true.into());
        }

        let response = self
            .client
            .post(self.url("synthesis")?)
            .query(&[("speaker", speaker.as_str())])
            .json(&query)
            .send()
            .await?
            .error_for_status()
            .map_err(|error| Error::ProviderResponse(error.to_string()))?;
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("audio/wav")
            .to_owned();
        if response.content_length().is_some_and(|length| length > MAX_AUDIO_RESPONSE_BYTES as u64)
        {
            return Err(Error::AudioResponseTooLarge(MAX_AUDIO_RESPONSE_BYTES / 1024 / 1024));
        }
        let mut stream = response.bytes_stream();
        let mut bytes = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            if bytes.len().saturating_add(chunk.len()) > MAX_AUDIO_RESPONSE_BYTES {
                return Err(Error::AudioResponseTooLarge(MAX_AUDIO_RESPONSE_BYTES / 1024 / 1024));
            }
            bytes.extend_from_slice(&chunk);
        }
        Ok(AudioData { bytes: Bytes::from(bytes), content_type })
    }
}

/// Creates the TTS provider selected by the application settings.
///
/// # Errors
///
/// Returns an error when the settings or endpoint are invalid.
pub fn build_provider(settings: &AppSettings) -> Result<Arc<dyn TtsProvider>> {
    settings.validate()?;
    let name = match settings.provider {
        crate::ProviderKind::AivisSpeech => "AivisSpeech",
        crate::ProviderKind::Voicevox => "VOICEVOX",
    };
    Ok(Arc::new(VoicevoxCompatibleProvider::new(name, &settings.endpoint)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_is_normalized_for_api_paths() {
        let provider =
            VoicevoxCompatibleProvider::new("VOICEVOX", "http://127.0.0.1:50021").unwrap();
        assert_eq!(provider.url("speakers").unwrap().as_str(), "http://127.0.0.1:50021/speakers");
    }

    #[test]
    fn speakers_are_flattened_into_named_voices() {
        let speakers: Vec<EngineSpeaker> = serde_json::from_str(
            r#"[{"name":"四国めたん","styles":[{"id":2,"name":"ノーマル"},{"id":0,"name":"あまあま"}]}]"#,
        )
        .unwrap();

        assert_eq!(
            voice_options(speakers),
            vec![
                VoiceOption { id: 0, name: "四国めたん（あまあま）".into() },
                VoiceOption { id: 2, name: "四国めたん（ノーマル）".into() },
            ]
        );
    }
}
