use super::animation::{SegmentAnimation, Transition};
use super::audio::AudioBlock;
use super::background::Background;
use super::code::CodeBlock;
use super::elevenlabs::ElevenLabsConfig;
use super::image::ImageBlock;
use super::layout::Layout;
use super::math::MathBlock;
use super::parts::SegmentPart;
use super::shape::ShapeBlock;
use super::table::TableBlock;
use super::text::TextBlock;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Segment ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct Segment {
    pub id: u32,

    #[serde(default)]
    pub r#type: SegmentType,

    /// Overrides the audio-derived duration. Use for pause/title segments
    /// that have no TTS. In seconds.
    #[serde(default)]
    pub duration_override_seconds: Option<f32>,

    // ── Primary content elements ───────────────────────────────────────────────
    #[serde(default)]
    pub text: Option<TextBlock>,

    #[serde(default)]
    pub math: Option<MathBlock>,

    #[serde(default)]
    pub code: Option<CodeBlock>,

    #[serde(default)]
    pub table: Option<TableBlock>,

    #[serde(default)]
    pub images: Vec<ImageBlock>,

    #[serde(default)]
    pub shapes: Vec<ShapeBlock>,

    // ── Sub-parts — sequential reveals within this segment ────────────────────
    /// Use parts when the narration walks through multiple distinct steps.
    /// Each part has its own content and appear_at trigger.
    #[serde(default)]
    pub parts: Vec<SegmentPart>,

    // ── Configuration ─────────────────────────────────────────────────────────
    #[serde(default)]
    pub elevenlabs: Option<ElevenLabsConfig>,

    #[serde(default)]
    pub audio: Option<AudioBlock>,

    #[serde(default)]
    pub animation: SegmentAnimation,

    /// Overrides the root-level background for this segment only.
    #[serde(default)]
    pub background: Option<Background>,

    #[serde(default)]
    pub layout: Layout,

    #[serde(default)]
    pub control: SegmentControl,

    // ── Pacing hints ──────────────────────────────────────────────────────────
    #[serde(default)]
    pub mood: Mood,

    #[serde(default)]
    pub pacing: Pacing,

    /// Seconds to hold the fully-rendered segment before transitioning out.
    #[serde(default)]
    pub post_hold_seconds: f32,

    /// How to transition from this segment to the next.
    #[serde(default)]
    pub transition_to_next: Option<Transition>,
}

// ─── Segment Type ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum SegmentType {
    /// Normal narrated segment — TTS runs and content displays.
    #[default]
    Speech,
    /// No TTS. Holds the canvas for duration_override_seconds.
    Pause,
    /// Visual-only transition segment. No content, no TTS.
    Transition,
    /// Large centered text. TTS optional. Usually chapter names.
    Title,
    /// Structural chapter boundary marker. Can trigger background change.
    Chapter,
    /// Final segment. Can trigger end cards, fade to black.
    Outro,
}

// ─── Segment Control ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct SegmentControl {
    /// If false, segment is fully excluded from rendering.
    /// Useful during development — easier than deleting and re-numbering.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Intentionally skipped from this render (distinct from disabled).
    /// Use this for output profile exclusions.
    #[serde(default)]
    pub skip: bool,

    /// Which output profiles this segment appears in.
    /// Empty = all profiles.
    #[serde(default)]
    pub profiles: Vec<String>,

    /// Scales the entire segment duration: audio + animation timing.
    /// 0.5 = half speed, 2.0 = double speed.
    #[serde(default = "default_speed")]
    pub speed_multiplier: f32,

    /// Milliseconds of silence before this segment's TTS begins.
    #[serde(default)]
    pub delay_before_ms: u32,

    /// Milliseconds of silence after this segment's TTS ends, before post_hold.
    #[serde(default)]
    pub delay_after_ms: u32,

    #[serde(default)]
    pub r#loop: Option<SegmentLoop>,
}

impl Default for SegmentControl {
    fn default() -> Self {
        Self {
            enabled: true,
            skip: false,
            profiles: vec![],
            speed_multiplier: 1.0,
            delay_before_ms: 0,
            delay_after_ms: 0,
            r#loop: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct SegmentLoop {
    pub count: LoopCount,
    #[serde(default)]
    pub pause_between_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum LoopCount {
    Count(u32),
    Infinite(),
}

// ─── Mood / Pacing ────────────────────────────────────────────────────────────
// Semantic hints your renderer can use to adjust automatic behavior
// (subtle background tint, animation easing, default hold duration).

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum Mood {
    #[default]
    Neutral,
    Analytical,
    Excited,
    Serious,
    Warm,
    Tense,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum Pacing {
    Slow,
    #[default]
    Normal,
    Fast,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn default_true() -> bool {
    true
}
fn default_speed() -> f32 {
    1.0
}
