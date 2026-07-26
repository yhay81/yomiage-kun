use serde::{Deserialize, Serialize};
use url::Url;

use crate::{Error, Result};

/// Supported text-to-speech backends.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    #[default]
    AivisSpeech,
    Voicevox,
}

impl ProviderKind {
    #[must_use]
    pub const fn default_endpoint(self) -> &'static str {
        match self {
            Self::AivisSpeech => "http://127.0.0.1:10101",
            Self::Voicevox => "http://127.0.0.1:50021",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::AivisSpeech => "AivisSpeech",
            Self::Voicevox => "VOICEVOX",
        }
    }
}

/// Voice controls shared by VOICEVOX-compatible engines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VoiceSettings {
    pub speaker_id: i64,
    pub speed: f32,
    pub pitch: f32,
    pub intonation: f32,
    pub volume: f32,
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self { speaker_id: 1, speed: 1.0, pitch: 0.0, intonation: 1.0, volume: 1.0 }
    }
}

impl VoiceSettings {
    /// Validates all voice parameters against the supported engine ranges.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidSettings`] when any parameter is out of range.
    pub fn validate(&self) -> Result<()> {
        if self.speaker_id < 0 {
            return Err(Error::InvalidSettings("話者IDは0以上にしてください".into()));
        }
        if !(0.5..=2.0).contains(&self.speed) {
            return Err(Error::InvalidSettings("読み上げ速度は0.5〜2.0にしてください".into()));
        }
        if !(-0.15..=0.15).contains(&self.pitch) {
            return Err(Error::InvalidSettings("ピッチは-0.15〜0.15にしてください".into()));
        }
        if !(0.0..=2.0).contains(&self.intonation) {
            return Err(Error::InvalidSettings("抑揚は0.0〜2.0にしてください".into()));
        }
        if !(0.0..=2.0).contains(&self.volume) {
            return Err(Error::InvalidSettings("音量は0.0〜2.0にしてください".into()));
        }
        Ok(())
    }
}

/// Non-secret desktop settings. The Discord token is stored in the OS keychain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub provider: ProviderKind,
    pub endpoint: String,
    pub voice: VoiceSettings,
    pub max_characters: usize,
    pub queue_capacity: usize,
    pub autostart: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        let provider = ProviderKind::default();
        Self {
            provider,
            endpoint: provider.default_endpoint().into(),
            voice: VoiceSettings::default(),
            max_characters: 160,
            queue_capacity: 32,
            autostart: false,
        }
    }
}

impl AppSettings {
    /// Validates the TTS endpoint, voice parameters, and queue limits.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidSettings`] when a setting is malformed or out of range.
    pub fn validate(&self) -> Result<()> {
        self.voice.validate()?;
        let endpoint = Url::parse(&self.endpoint).map_err(|_| {
            Error::InvalidSettings("TTSエンドポイントが正しいURLではありません".into())
        })?;
        if endpoint.scheme() != "http" && endpoint.scheme() != "https" {
            return Err(Error::InvalidSettings(
                "TTSエンドポイントはhttpまたはhttpsにしてください".into(),
            ));
        }
        if !(16..=2_000).contains(&self.max_characters) {
            return Err(Error::InvalidSettings("最大文字数は16〜2000にしてください".into()));
        }
        if !(1..=256).contains(&self.queue_capacity) {
            return Err(Error::InvalidSettings("キュー容量は1〜256にしてください".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_valid() {
        AppSettings::default().validate().unwrap();
    }

    #[test]
    fn provider_metadata_matches_each_engine() {
        assert_eq!(ProviderKind::AivisSpeech.display_name(), "AivisSpeech");
        assert_eq!(ProviderKind::AivisSpeech.default_endpoint(), "http://127.0.0.1:10101");
        assert_eq!(ProviderKind::Voicevox.display_name(), "VOICEVOX");
        assert_eq!(ProviderKind::Voicevox.default_endpoint(), "http://127.0.0.1:50021");
    }

    #[test]
    fn rejects_non_http_endpoint() {
        let settings = AppSettings { endpoint: "file:///tmp/engine".into(), ..Default::default() };
        assert!(settings.validate().is_err());
    }

    #[test]
    fn rejects_out_of_range_voice_parameters() {
        let settings = AppSettings {
            voice: VoiceSettings { speed: 3.0, ..Default::default() },
            ..Default::default()
        };
        assert!(settings.validate().is_err());
    }
}
