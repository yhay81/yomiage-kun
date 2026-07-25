use std::sync::Arc;

use dashmap::DashMap;
use serenity::model::id::{ChannelId, GuildId};
use songbird::Call;
use tokio::sync::{Mutex, mpsc};
use yomiage_core::{SynthesisRequest, TtsProvider, VoiceSettings};

#[derive(Debug)]
pub struct PlaybackRequest {
    pub text: String,
    pub voice: VoiceSettings,
}

#[derive(Clone)]
pub struct GuildSession {
    pub text_channel_id: ChannelId,
    sender: mpsc::Sender<PlaybackRequest>,
    queue_capacity: usize,
}

impl GuildSession {
    #[must_use]
    pub fn queued(&self) -> usize {
        self.queue_capacity.saturating_sub(self.sender.capacity())
    }

    pub fn try_enqueue(&self, request: PlaybackRequest) -> Result<(), EnqueueError> {
        self.sender.try_send(request).map_err(|error| match error {
            mpsc::error::TrySendError::Full(_) => EnqueueError::Full,
            mpsc::error::TrySendError::Closed(_) => EnqueueError::Closed,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnqueueError {
    Full,
    Closed,
}

pub type Sessions = Arc<DashMap<GuildId, GuildSession>>;

pub fn create_session(
    text_channel_id: ChannelId,
    queue_capacity: usize,
    call: Arc<Mutex<Call>>,
    provider: Arc<dyn TtsProvider>,
) -> GuildSession {
    let (sender, receiver) = mpsc::channel(queue_capacity);
    tokio::spawn(playback_worker(receiver, call, provider));
    GuildSession { text_channel_id, sender, queue_capacity }
}

async fn playback_worker(
    mut receiver: mpsc::Receiver<PlaybackRequest>,
    call: Arc<Mutex<Call>>,
    provider: Arc<dyn TtsProvider>,
) {
    while let Some(item) = receiver.recv().await {
        let request = SynthesisRequest { text: item.text, voice: item.voice };
        match provider.synthesize(&request).await {
            Ok(audio) => {
                tracing::debug!(
                    provider = provider.name(),
                    bytes = audio.bytes.len(),
                    content_type = audio.content_type,
                    "synthesized Discord message"
                );
                call.lock().await.enqueue_input(audio.bytes.to_vec().into()).await;
            }
            Err(error) => {
                tracing::warn!(provider = provider.name(), %error, "speech synthesis failed");
            }
        }
    }
}
