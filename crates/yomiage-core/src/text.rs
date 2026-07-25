use std::sync::LazyLock;

use regex::Regex;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

static URL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bhttps?://[^\s<>()]+").expect("valid URL regex"));
static CUSTOM_EMOJI: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<a?:([A-Za-z0-9_]+):\d+>").expect("valid emoji regex"));
static USER_MENTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<@!?\d+>").expect("valid user mention regex"));
static ROLE_MENTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<@&\d+>").expect("valid role mention regex"));
static CHANNEL_MENTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<#\d+>").expect("valid channel mention regex"));
static MARKDOWN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[*_~`>|]+").expect("valid markdown regex"));
static REPEATED_W: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:w|ｗ){2,}").expect("valid repeated-w regex"));
static WHITESPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s+").expect("valid whitespace regex"));

#[derive(Debug, Clone, Copy)]
pub struct NormalizeOptions {
    pub max_characters: usize,
}

impl Default for NormalizeOptions {
    fn default() -> Self {
        Self { max_characters: 160 }
    }
}

/// Converts Discord markup into deterministic text suitable for Japanese TTS.
#[must_use]
pub fn normalize_discord_text(input: &str, options: NormalizeOptions) -> String {
    let mut output = input.nfkc().collect::<String>();
    output = URL.replace_all(&output, " URL ").into_owned();
    output = CUSTOM_EMOJI.replace_all(&output, " $1 ").into_owned();
    output = USER_MENTION.replace_all(&output, " メンション ").into_owned();
    output = ROLE_MENTION.replace_all(&output, " ロール ").into_owned();
    output = CHANNEL_MENTION.replace_all(&output, " チャンネル ").into_owned();
    output = REPEATED_W.replace_all(&output, " わら ").into_owned();
    output = MARKDOWN.replace_all(&output, " ").into_owned();
    let output = WHITESPACE.replace_all(&output, " ").trim().to_owned();

    let graphemes = output.graphemes(true).collect::<Vec<_>>();
    if graphemes.len() > options.max_characters {
        let keep = options.max_characters.saturating_sub(4);
        format!("{}、以下略", graphemes[..keep].concat())
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleans_discord_markup() {
        let normalized = normalize_discord_text(
            "**こんにちは** <@123> https://example.com <:party:456> www",
            NormalizeOptions::default(),
        );
        assert_eq!(normalized, "こんにちは メンション URL party わら");
    }

    #[test]
    fn truncates_on_grapheme_boundaries() {
        let normalized =
            normalize_discord_text("あいうえおかきくけこ", NormalizeOptions { max_characters: 8 });
        assert_eq!(normalized, "あいうえ、以下略");
    }

    #[test]
    fn normalizes_width() {
        let normalized = normalize_discord_text("ＡＢＣ　１２３", NormalizeOptions::default());
        assert_eq!(normalized, "ABC 123");
    }
}
