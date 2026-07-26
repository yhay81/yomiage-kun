#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("設定が正しくありません: {0}")]
    InvalidSettings(String),
    #[error("読み上げるテキストが空です")]
    EmptyText,
    #[error("TTSエンジンに接続できません: {0}")]
    ProviderUnavailable(String),
    #[error("TTSエンジンがエラーを返しました: {0}")]
    ProviderResponse(String),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
