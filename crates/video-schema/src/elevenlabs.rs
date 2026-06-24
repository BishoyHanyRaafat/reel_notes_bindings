use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElevenLabsConfig {
    pub voice_id: String,

    #[serde(default)]
    pub model: ElevenLabsModel,

    /// 0.0 = very expressive, 1.0 = very stable/robotic.
    /// Recommended: 0.3–0.5 for educational content.
    #[serde(default = "default_stability")]
    pub stability: f32,

    #[serde(default = "default_similarity")]
    pub similarity_boost: f32,

    /// Style exaggeration. 0.0 = neutral, 1.0 = max expression.
    #[serde(default = "default_style")]
    pub style: f32,

    #[serde(default = "bool_true")]
    pub use_speaker_boost: bool,

    /// Speech rate multiplier: 0.7 (slow) – 1.3 (fast).
    #[serde(default = "default_speed")]
    pub speed: f32,

    /// Request character-level timestamps from ElevenLabs.
    /// Your pipeline groups these into word timestamps automatically.
    #[serde(default = "bool_true")]
    pub request_word_timestamps: bool,
}

fn default_stability() -> f32 {
    0.4
}
fn default_similarity() -> f32 {
    0.8
}
fn default_style() -> f32 {
    0.5
}
fn default_speed() -> f32 {
    0.95
}
fn bool_true() -> bool {
    true
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ElevenLabsModel {
    #[serde(rename = "eleven_monolingual_v1")]
    Monolingual,
    #[serde(rename = "eleven_multilingual_v2")]
    #[default]
    MultilingualV2,
    #[serde(rename = "eleven_turbo_v2")]
    TurboV2,
    #[serde(rename = "eleven_turbo_v2_5")]
    TurboV2_5,
}

impl ElevenLabsModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Monolingual => "eleven_monolingual_v1",
            Self::MultilingualV2 => "eleven_multilingual_v2",
            Self::TurboV2 => "eleven_turbo_v2",
            Self::TurboV2_5 => "eleven_turbo_v2_5",
        }
    }
}
