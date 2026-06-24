use crate::SegmentDef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The top-level document produced by the LLM.
/// Every field has a serde default so partial JSON is still valid during development.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LessonScript {
    #[serde(default = "default_version")]
    pub version: String,

    /// Frames per second for the output video.
    #[serde(default = "default_fps")]
    pub fps: u32,

    /// [width, height] in pixels.
    #[serde(default = "default_resolution")]
    pub resolution: [u32; 2],

    #[serde(default)]
    pub aspect_ratio: AspectRatio,

    pub output: Option<OutputConfig>,

    #[serde(default)]
    pub background: BackgroundDef,

    /// Segment-level defaults; merged with per-segment overrides at runtime.
    #[serde(default)]
    pub defaults: SegmentDefaults,

    /// Pre-declared assets (images, fonts, audio clips).
    #[serde(default)]
    pub assets: AssetRegistry,

    /// The ordered list of segments that form the lesson.
    #[serde(default)]
    pub segments: Vec<SegmentDef>,
}

fn default_version() -> String {
    "1.0".into()
}
fn default_fps() -> u32 {
    30
}
fn default_resolution() -> [u32; 2] {
    [1920, 1080]
}

// ─── Output ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_filename")]
    pub filename: String,

    #[serde(default)]
    pub format: VideoFormat,

    #[serde(default)]
    pub codec: VideoCodec,

    /// e.g. "8M", "4M", "auto"
    #[serde(default = "default_bitrate")]
    pub bitrate: String,

    #[serde(default)]
    pub audio_codec: AudioCodecKind,

    #[serde(default = "default_audio_bitrate")]
    pub audio_bitrate: String,
}

fn default_filename() -> String {
    "output".into()
}
fn default_bitrate() -> String {
    "8M".into()
}
fn default_audio_bitrate() -> String {
    "192k".into()
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            filename: default_filename(),
            format: VideoFormat::default(),
            codec: VideoCodec::default(),
            bitrate: default_bitrate(),
            audio_codec: AudioCodecKind::default(),
            audio_bitrate: default_audio_bitrate(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoFormat {
    #[default]
    Mp4,
    Webm,
    Mov,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoCodec {
    #[default]
    H264,
    H265,
    Vp9,
    Av1,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioCodecKind {
    #[default]
    Aac,
    Mp3,
    Opus,
}

// ─── Background ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum BackgroundDef {
    Solid {
        color: String,
    },
    Gradient {
        gradient: GradientDef,
    },
    Image {
        src: String,
        #[serde(default)]
        fit: FitMode,
        overlay: Option<OverlayDef>,
    },
}

impl Default for BackgroundDef {
    fn default() -> Self {
        BackgroundDef::Solid {
            color: "#0f0f0f".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientDef {
    #[serde(default)]
    pub direction: GradientDirection,
    pub stops: Vec<GradientStop>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GradientDirection {
    #[default]
    Vertical,
    Horizontal,
    DiagonalTl,
    DiagonalTr,
    Radial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientStop {
    pub color: String,
    pub position: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayDef {
    pub color: String,
    pub opacity: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FitMode {
    #[default]
    Cover,
    Contain,
    Stretch,
    Tile,
}

// ─── Aspect ratio ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum AspectRatio {
    #[serde(rename = "16:9")]
    #[default]
    Widescreen,
    #[serde(rename = "9:16")]
    Vertical,
    #[serde(rename = "1:1")]
    Square,
    #[serde(rename = "4:3")]
    Classic,
}

// ─── Defaults ────────────────────────────────────────────────────────────────

/// Merged into every segment at pipeline build time unless the segment overrides the field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentDefaults {
    pub elevenlabs: Option<crate::ElevenLabsConfig>,
    pub animation_in: Option<AnimationDef>,
    pub animation_out: Option<AnimationDef>,
    pub animation_duration_ms: Option<u32>,
    pub post_hold_seconds: Option<f32>,
}

// ─── Asset registry ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetRegistry {
    #[serde(default)]
    pub images: HashMap<String, ImageAsset>,
    #[serde(default)]
    pub fonts: HashMap<String, FontAsset>,
    #[serde(default)]
    pub audio: HashMap<String, AudioAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAsset {
    pub src: String,
    #[serde(default)]
    pub preload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontAsset {
    pub src: String,
    pub family: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAsset {
    pub src: String,
}

// ─── Animation (shared across segments and elements) ─────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDef {
    #[serde(rename = "type")]
    pub kind: AnimationType,
    #[serde(default = "default_anim_duration")]
    pub duration_ms: u32,
    #[serde(default)]
    pub easing: Easing,
    #[serde(default)]
    pub delay_ms: u32,
}

fn default_anim_duration() -> u32 {
    300
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimationType {
    #[default]
    Fade,
    SlideUp,
    SlideDown,
    SlideLeft,
    SlideRight,
    ZoomIn,
    ZoomOut,
    FlipX,
    FlipY,
    BlurIn,
    BlurOut,
    Bounce,
    None,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Easing {
    Linear,
    EaseIn,
    #[default]
    EaseOut,
    EaseInOut,
    Spring,
    BounceEase,
}
