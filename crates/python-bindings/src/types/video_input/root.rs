use super::animation::{AnimationIn, AnimationOut};
use super::background::Background;
use super::common::AspectRatio;
use super::common::{Color, Resolution};
use super::elevenlabs::ElevenLabsConfig;
use super::segment::Segment;
use super::text::FontConfig;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Video Input ──────────────────────────────────────────────────────────────
// Root type. This is exactly what the LLM outputs

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct VideoInput {
    #[serde(default = "default_version")]
    pub version: String,

    #[serde(default = "default_fps")]
    pub fps: u32,

    #[serde(default)]
    pub resolution: Resolution,

    #[serde(default)]
    pub aspect_ratio: AspectRatio,

    /// Which profile to render. Must match a key in profiles, or "default".
    #[serde(default = "default_profile")]
    pub profile: String,

    #[serde(default)]
    pub output: OutputConfig,

    #[serde(default)]
    pub background: Background,

    /// Segment-level defaults. Applied before per-segment overrides.
    #[serde(default)]
    pub defaults: SegmentDefaults,

    /// Pre-declared assets. Keys referenced by segments.
    #[serde(default)]
    pub assets: AssetRegistry,

    /// Output profile definitions for multi-cut rendering.
    #[serde(default)]
    pub profiles: HashMap<String, OutputProfile>,

    pub segments: Vec<Segment>,
}

// ─── Output Config ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct OutputConfig {
    #[serde(default = "default_filename")]
    pub filename: String,
    #[serde(default)]
    pub format: OutputFormat,
    #[serde(default)]
    pub codec: VideoCodec,
    #[serde(default = "default_bitrate")]
    pub bitrate: String,
    #[serde(default)]
    pub audio_codec: AudioCodec,
    #[serde(default = "default_audio_bitrate")]
    pub audio_bitrate: String,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            filename: default_filename(),
            format: OutputFormat::Mp4,
            codec: VideoCodec::H264,
            bitrate: default_bitrate(),
            audio_codec: AudioCodec::Aac,
            audio_bitrate: default_audio_bitrate(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum OutputFormat {
    #[default]
    Mp4,
    Webm,
    Mov,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum VideoCodec {
    #[default]
    H264,
    H265,
    Vp9,
    Av1,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum AudioCodec {
    #[default]
    Aac,
    Mp3,
    Opus,
}

// ─── Segment Defaults ─────────────────────────────────────────────────────────
// LLM sets these once. Your pipeline merges them into every segment
// before processing. Per-segment values always win.

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct SegmentDefaults {
    #[serde(default)]
    pub font: Option<FontConfig>,
    #[serde(default)]
    pub text_color: Option<Color>,
    #[serde(default)]
    pub animation_in: Option<AnimationIn>,
    #[serde(default)]
    pub animation_out: Option<AnimationOut>,
    #[serde(default)]
    pub animation_duration_ms: Option<u32>,
    #[serde(default)]
    pub post_hold_seconds: Option<f32>,
    #[serde(default)]
    pub elevenlabs: Option<ElevenLabsConfig>,
}

// ─── Asset Registry ───────────────────────────────────────────────────────────
// Pre-declares named assets so segments can reference them by key string.

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct AssetRegistry {
    #[serde(default)]
    pub images: HashMap<String, ImageAsset>,
    #[serde(default)]
    pub fonts: HashMap<String, FontAsset>,
    #[serde(default)]
    pub audio: HashMap<String, AudioAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct ImageAsset {
    pub src: String,
    #[serde(default = "default_true")]
    pub preload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct FontAsset {
    pub src: String,
    pub family: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct AudioAsset {
    pub src: String,
}

// ─── Output Profiles ──────────────────────────────────────────────────────────
// Defines multiple render cuts from a single JSON file.

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct OutputProfile {
    /// Maximum duration in seconds. None = no limit.
    #[serde(default)]
    pub max_duration_seconds: Option<f32>,

    /// Segment IDs to exclude in this profile.
    #[serde(default)]
    pub excluded_segment_ids: Vec<u32>,

    /// Override resolution for this profile (e.g. square for social).
    #[serde(default)]
    pub resolution: Option<Resolution>,

    /// Override aspect ratio for this profile.
    #[serde(default)]
    pub aspect_ratio: Option<AspectRatio>,

    /// Override output filename for this profile.
    #[serde(default)]
    pub filename: Option<String>,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn default_version() -> String {
    "1.0.0".to_string()
}
fn default_fps() -> u32 {
    30
}
fn default_profile() -> String {
    "default".to_string()
}
fn default_filename() -> String {
    "output".to_string()
}
fn default_bitrate() -> String {
    "8M".to_string()
}
fn default_audio_bitrate() -> String {
    "192k".to_string()
}
fn default_true() -> bool {
    true
}
