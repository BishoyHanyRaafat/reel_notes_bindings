use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ─── ElevenLabs Config ────────────────────────────────────────────────────────
// Per-segment TTS configuration. Merges with defaults from root-level defaults block.
// Your pipeline should merge root defaults → segment override before calling the API.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct ElevenLabsConfig {
    /// ElevenLabs voice ID string
    pub voice_id: String,

    #[serde(default)]
    pub model: ElevenLabsModel,

    /// 0.0 = very expressive/variable, 1.0 = very stable/monotone
    /// Recommended: 0.35–0.50 for educational content
    #[serde(default = "default_stability")]
    pub stability: f32,

    /// How closely the voice matches the original voice profile
    #[serde(default = "default_similarity")]
    pub similarity_boost: f32,

    /// Style exaggeration. 0.0 = neutral, 1.0 = very stylized
    /// Adds latency — keep at 0.0–0.6 for production
    #[serde(default = "default_style")]
    pub style: f32,

    /// Enhances speaker clarity at the cost of some speed
    #[serde(default = "default_true")]
    pub use_speaker_boost: bool,

    /// Playback speed multiplier. 0.7 = slower, 1.3 = faster
    /// Affects natural rhythm — don't go below 0.85 for educational content
    #[serde(default = "default_speed")]
    pub speed: f32,

    /// Whether to request word-level timestamps in the response.
    /// Required for word-by-word text reveal and audio_cue matching.
    /// Always true in production — only false for quick drafts.
    #[serde(default = "default_true")]
    pub request_word_timestamps: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum ElevenLabsModel {
    ElevenMonolingualV1,
    #[default]
    ElevenMultilingualV2,
    ElevenTurboV2,
    ElevenTurboV25,
}

// ─── Word Timestamp ───────────────────────────────────────────────────────────
// Output from ElevenLabs alignment response — not part of the input schema,
// but lives here since it's tightly coupled to this module.
// Your TTS stage fills this and attaches it to each segment before timeline building.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct WordTimestamp {
    pub word: String,
    pub start_seconds: f32,
    pub end_seconds: f32,
}

// ─── Resolved TTS Output ──────────────────────────────────────────────────────
// What your TTS stage produces after calling ElevenLabs.
// Passed downstream to the timeline builder and renderer.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct TtsOutput {
    pub segment_id: u32,
    pub audio_path: String, // path to the generated .mp3 / .wav file
    pub duration_seconds: f32,
    pub word_timestamps: Vec<WordTimestamp>,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn default_stability() -> f32 {
    0.40
}
fn default_similarity() -> f32 {
    0.80
}
fn default_style() -> f32 {
    0.50
}
fn default_speed() -> f32 {
    0.95
}
fn default_true() -> bool {
    true
}
