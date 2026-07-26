use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use moka::future::Cache;
use sha2::{Digest, Sha256};

use crate::{AudioData, Result, SynthesisRequest, TtsProvider};

/// Bounded, in-memory cache for fully synthesized audio.
pub struct CachedTtsProvider {
    inner: Arc<dyn TtsProvider>,
    cache: Cache<String, AudioData>,
}

impl CachedTtsProvider {
    /// Wraps a provider with a byte-bounded, in-memory audio cache.
    #[must_use]
    pub fn new(inner: Arc<dyn TtsProvider>, max_bytes: u64) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_bytes)
            .weigher(|_key, audio: &AudioData| u32::try_from(audio.bytes.len()).unwrap_or(u32::MAX))
            .time_to_idle(Duration::from_hours(1))
            .build();
        Self { inner, cache }
    }

    fn key(&self, request: &SynthesisRequest) -> String {
        let voice = &request.voice;
        let source = format!(
            "{}\0{}\0{}\0{}\0{}\0{}\0{}",
            self.inner.name(),
            request.text,
            voice.speaker_id,
            voice.speed,
            voice.pitch,
            voice.intonation,
            voice.volume
        );
        format!("{:x}", Sha256::digest(source.as_bytes()))
    }
}

#[async_trait]
impl TtsProvider for CachedTtsProvider {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    async fn healthcheck(&self) -> Result<String> {
        self.inner.healthcheck().await
    }

    async fn synthesize(&self, request: &SynthesisRequest) -> Result<AudioData> {
        let key = self.key(request);
        if let Some(audio) = self.cache.get(&key).await {
            tracing::debug!(provider = self.name(), "TTS cache hit");
            return Ok(audio);
        }
        let audio = self.inner.synthesize(request).await?;
        self.cache.insert(key, audio.clone()).await;
        Ok(audio)
    }
}
