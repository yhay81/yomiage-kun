use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use serenity::{
    all::{
        ChannelId, CommandInteraction, Context, CreateInteractionResponse,
        CreateInteractionResponseMessage, EventHandler, GuildId, Interaction, Message, Permissions,
        Ready, UserId,
    },
    async_trait,
};
use tokio::sync::RwLock;
use yomiage_core::{NormalizeOptions, TtsProvider, VoiceSettings, normalize_discord_text};

use crate::{
    commands,
    service::{BotState, BotStatus},
    session::{EnqueueError, PlaybackRequest, Sessions, create_session},
};

pub struct Handler {
    pub sessions: Sessions,
    pub provider: Arc<dyn TtsProvider>,
    pub voice: VoiceSettings,
    pub queue_capacity: usize,
    pub max_characters: usize,
    pub status: Arc<RwLock<BotStatus>>,
    pub commands_registered: AtomicBool,
}

impl Handler {
    fn has_manage_permission(permissions: Option<Permissions>) -> bool {
        permissions.is_some_and(|permissions| {
            permissions.intersects(Permissions::MANAGE_GUILD | Permissions::ADMINISTRATOR)
        })
    }

    fn command_has_manage_permission(command: &CommandInteraction) -> bool {
        Self::has_manage_permission(command.member.as_ref().and_then(|member| member.permissions))
    }

    fn user_voice_channel(ctx: &Context, guild_id: GuildId, user_id: UserId) -> Option<ChannelId> {
        ctx.cache
            .guild(guild_id)
            .and_then(|guild| guild.voice_states.get(&user_id).and_then(|state| state.channel_id))
    }

    async fn respond(command: &CommandInteraction, ctx: &Context, content: impl Into<String>) {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content).ephemeral(true),
        );
        if let Err(error) = command.create_response(&ctx.http, response).await {
            tracing::warn!(%error, "failed to respond to Discord interaction");
        }
    }

    async fn join(&self, ctx: &Context, command: &CommandInteraction) -> String {
        let Some(guild_id) = command.guild_id else {
            return "サーバー内で実行してください。".into();
        };
        if !Self::command_has_manage_permission(command) {
            return "この操作には「サーバーの管理」権限が必要です。".into();
        }
        let voice_channel = Self::user_voice_channel(ctx, guild_id, command.user.id);
        let Some(voice_channel) = voice_channel else {
            return "先にボイスチャンネルへ参加してください。".into();
        };
        let Some(manager) = songbird::get(ctx).await else {
            return "音声サービスを初期化できませんでした。".into();
        };

        if self.sessions.contains_key(&guild_id) {
            return "すでに読み上げ中です。`/leave`後にもう一度お試しください。".into();
        }

        match manager.join(guild_id, voice_channel).await {
            Ok(call) => {
                self.sessions.insert(
                    guild_id,
                    create_session(
                        command.channel_id,
                        voice_channel,
                        self.queue_capacity,
                        call,
                        Arc::clone(&self.provider),
                    ),
                );
                format!(
                    "<#{}> の投稿を読み上げます。キュー上限は{}件です。",
                    command.channel_id, self.queue_capacity
                )
            }
            Err(error) => {
                tracing::warn!(%error, %guild_id, "failed to join voice channel");
                format!("ボイスチャンネルに参加できませんでした: {error}")
            }
        }
    }

    async fn leave(&self, ctx: &Context, command: &CommandInteraction) -> String {
        let Some(guild_id) = command.guild_id else {
            return "サーバー内で実行してください。".into();
        };
        if !Self::command_has_manage_permission(command) {
            return "この操作には「サーバーの管理」権限が必要です。".into();
        }
        if let Some((_, session)) = self.sessions.remove(&guild_id) {
            session.stop().await;
        }
        let Some(manager) = songbird::get(ctx).await else {
            return "音声サービスを取得できませんでした。".into();
        };
        match manager.remove(guild_id).await {
            Ok(()) => "読み上げを終了しました。".into(),
            Err(error) => {
                tracing::warn!(%error, %guild_id, "failed to leave voice channel");
                "接続済みのボイスチャンネルがありません。".into()
            }
        }
    }

    async fn skip(&self, ctx: &Context, command: &CommandInteraction) -> String {
        let Some(guild_id) = command.guild_id else {
            return "サーバー内で実行してください。".into();
        };
        let Some(session) = self.sessions.get(&guild_id).map(|entry| entry.clone()) else {
            return "現在は読み上げていません。".into();
        };
        if Self::user_voice_channel(ctx, guild_id, command.user.id)
            != Some(session.voice_channel_id)
        {
            return "読み上げくんと同じボイスチャンネルに参加してから実行してください。".into();
        }
        let Some(manager) = songbird::get(ctx).await else {
            return "音声サービスを取得できませんでした。".into();
        };
        let Some(call) = manager.get(guild_id) else {
            return "現在は読み上げていません。".into();
        };
        match call.lock().await.queue().skip() {
            Ok(()) => "現在の読み上げをスキップしました。".into(),
            Err(_) => "スキップできる読み上げがありません。".into(),
        }
    }

    fn guild_status(&self, guild_id: Option<GuildId>) -> String {
        let Some(guild_id) = guild_id else {
            return "サーバー内で実行してください。".into();
        };
        self.sessions.get(&guild_id).map_or_else(
            || "Botはオンラインです。現在このサーバーでは読み上げていません。".into(),
            |session| {
                format!(
                    "読み上げ中です。対象: <#{}>、合成待ち: {}件",
                    session.text_channel_id,
                    session.queued()
                )
            },
        )
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!(user = %ready.user.name, guilds = ready.guilds.len(), "Discord bot ready");
        {
            let mut status = self.status.write().await;
            status.state = BotState::Running;
            status.username = Some(ready.user.name.clone());
            status.guild_count = ready.guilds.len();
            status.last_error = None;
        }
        if self
            .commands_registered
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
            && let Err(error) = commands::register(&ctx).await
        {
            self.commands_registered.store(false, Ordering::Release);
            tracing::warn!(%error, "failed to register application commands");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command) = interaction else {
            return;
        };
        let content = match command.data.name.as_str() {
            "join" => self.join(&ctx, &command).await,
            "leave" => self.leave(&ctx, &command).await,
            "skip" => self.skip(&ctx, &command).await,
            "status" => self.guild_status(command.guild_id),
            _ => return,
        };
        Self::respond(&command, &ctx, content).await;
    }

    async fn message(&self, _ctx: Context, message: Message) {
        if message.author.bot {
            return;
        }
        let Some(guild_id) = message.guild_id else {
            return;
        };
        let Some(session) = self.sessions.get(&guild_id).map(|entry| entry.clone()) else {
            return;
        };
        if session.text_channel_id != message.channel_id {
            return;
        }

        let source = if message.content.trim().is_empty() && !message.attachments.is_empty() {
            "添付ファイル"
        } else {
            &message.content
        };
        let text = normalize_discord_text(
            source,
            NormalizeOptions { max_characters: self.max_characters },
        );
        if text.is_empty() {
            return;
        }
        let request = PlaybackRequest { text, voice: self.voice.clone() };
        match session.try_enqueue(request) {
            Ok(()) => {}
            Err(EnqueueError::Full) => {
                tracing::warn!(%guild_id, "guild speech queue is full");
            }
            Err(EnqueueError::Closed) => {
                if let Some((_, session)) = self.sessions.remove(&guild_id) {
                    session.stop().await;
                }
                tracing::warn!(%guild_id, "guild speech queue closed unexpectedly");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn management_commands_require_manage_guild_or_administrator() {
        assert!(!Handler::has_manage_permission(None));
        assert!(!Handler::has_manage_permission(Some(Permissions::SEND_MESSAGES)));
        assert!(Handler::has_manage_permission(Some(Permissions::MANAGE_GUILD)));
        assert!(Handler::has_manage_permission(Some(Permissions::ADMINISTRATOR)));
    }
}
