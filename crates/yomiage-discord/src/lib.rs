//! Discord gateway, commands, and guild-scoped playback queues.

mod commands;
mod handler;
mod service;
mod session;

pub use service::{BotConfig, BotService, BotState, BotStatus, validate_token};
