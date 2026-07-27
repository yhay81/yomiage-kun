use std::sync::Arc;

use dashmap::DashMap;
use serenity::model::id::{ChannelId, GuildId};
use songbird::Call;
use tokio::{
    sync::{Mutex, mpsc, watch},
    task::JoinHandle,
};
use yomiage_core::{SynthesisRequest, TtsProvider, VoiceSettings};

#[derive(Debug)]
pub struct PlaybackRequest {
    pub text: String,
    pub voice: VoiceSettings,
}

#[derive(Clone)]
pub struct GuildSession {
    pub text_channel_id: ChannelId,
    pub voice_channel_id: ChannelId,
    sender: mpsc::Sender<PlaybackRequest>,
    queue_capacity: usize,
    cancellation: watch::Sender<bool>,
    worker: Arc<Mutex<Option<JoinHandle<()>>>>,
    call: Arc<Mutex<Call>>,
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

    pub async fn stop(&self) {
        let _ = self.cancellation.send(true);
        self.call.lock().await.queue().stop();
        if let Some(worker) = self.worker.lock().await.take() {
            let _ = worker.await;
        }
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
    voice_channel_id: ChannelId,
    queue_capacity: usize,
    call: Arc<Mutex<Call>>,
    provider: Arc<dyn TtsProvider>,
) -> GuildSession {
    let (sender, receiver) = mpsc::channel(queue_capacity);
    let (cancellation, cancellation_rx) = watch::channel(false);
    let worker =
        tokio::spawn(playback_worker(receiver, Arc::clone(&call), provider, cancellation_rx));
    GuildSession {
        text_channel_id,
        voice_channel_id,
        sender,
        queue_capacity,
        cancellation,
        worker: Arc::new(Mutex::new(Some(worker))),
        call,
    }
}

async fn playback_worker(
    mut receiver: mpsc::Receiver<PlaybackRequest>,
    call: Arc<Mutex<Call>>,
    provider: Arc<dyn TtsProvider>,
    mut cancellation: watch::Receiver<bool>,
) {
    loop {
        let item = tokio::select! {
            changed = cancellation.changed() => {
                if changed.is_err() || *cancellation.borrow() {
                    break;
                }
                continue;
            }
            item = receiver.recv() => {
                let Some(item) = item else {
                    break;
                };
                item
            }
        };
        let request = SynthesisRequest { text: item.text, voice: item.voice };
        let result = tokio::select! {
            changed = cancellation.changed() => {
                if changed.is_err() || *cancellation.borrow() {
                    break;
                }
                continue;
            }
            result = provider.synthesize(&request) => result,
        };
        match result {
            Ok(audio) => {
                tracing::debug!(
                    provider = provider.name(),
                    bytes = audio.bytes.len(),
                    content_type = audio.content_type,
                    "synthesized Discord message"
                );
                if *cancellation.borrow() {
                    break;
                }
                call.lock().await.enqueue_input(audio.bytes.to_vec().into()).await;
            }
            Err(error) => {
                tracing::warn!(provider = provider.name(), %error, "speech synthesis failed");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> PlaybackRequest {
        PlaybackRequest { text: "こんにちは".into(), voice: VoiceSettings::default() }
    }

    fn test_session(sender: mpsc::Sender<PlaybackRequest>) -> GuildSession {
        let (cancellation, _cancellation_rx) = watch::channel(false);
        GuildSession {
            text_channel_id: ChannelId::new(1),
            voice_channel_id: ChannelId::new(2),
            sender,
            queue_capacity: 1,
            cancellation,
            worker: Arc::new(Mutex::new(None)),
            call: Arc::new(Mutex::new(Call::standalone(
                GuildId::new(1),
                serenity::model::id::UserId::new(1),
            ))),
        }
    }

    #[tokio::test]
    async fn bounded_queue_reports_full_without_waiting() {
        let (sender, _receiver) = mpsc::channel(1);
        let session = test_session(sender);

        assert_eq!(session.queued(), 0);
        assert_eq!(session.try_enqueue(request()), Ok(()));
        assert_eq!(session.queued(), 1);
        assert_eq!(session.try_enqueue(request()), Err(EnqueueError::Full));
    }

    #[tokio::test]
    async fn closed_queue_is_removed_by_the_caller() {
        let (sender, receiver) = mpsc::channel(1);
        drop(receiver);
        let session = test_session(sender);

        assert_eq!(session.try_enqueue(request()), Err(EnqueueError::Closed));
    }
}
