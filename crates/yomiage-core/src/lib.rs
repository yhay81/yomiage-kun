//! Shared domain logic for Yomiage-kun.

mod cache;
mod config;
mod error;
mod text;
mod tts;

pub use cache::CachedTtsProvider;
pub use config::{AppSettings, ProviderKind, VoiceSettings};
pub use error::{Error, Result};
pub use text::{NormalizeOptions, normalize_discord_text};
pub use tts::{
    AudioData, SynthesisRequest, TtsProvider, VoicevoxCompatibleProvider, build_provider,
};
