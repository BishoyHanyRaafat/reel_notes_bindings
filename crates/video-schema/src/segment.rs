use crate::{AnimationDef, AudioTrack, BackgroundDef, ElevenLabsConfig, TextDef};
use serde::{Deserialize, Serialize};

/// One segment of the lesson — the atomic unit of the pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentDef {
    pub id: u32,

    #[serde(default)]
    pub kind: SegmentKind,

    /// If set, overrides the duration calculated from audio.
    pub duration_override_seconds: Option<f32>,

    /// Main narration text.  Optional so pause/transition segments can omit it.
    pub text: Option<TextDef>,

    /// TTS + voice settings for this segment (merged with defaults).
    pub elevenlabs: Option<ElevenLabsConfig>,

    /// Background override for this segment only.
    pub background: Option<BackgroundDef>,

    /// How the segment enters.
    pub animation_in: Option<AnimationDef>,

    /// How the segment exits.
    pub animation_out: Option<AnimationDef>,

    /// Extra seconds to hold the frame after narration ends.
    #[serde(default)]
    pub post_hold_seconds: f32,

    /// Mood hint — your renderer may use this to pick accent colours.
    #[serde(default)]
    pub mood: Mood,

    /// Pacing hint — affects TTS speed default if not overridden.
    #[serde(default)]
    pub pacing: Pacing,

    /// Background music or sound effect for this segment.
    pub audio: Option<AudioTrack>,

    /// How to transition into the *next* segment.
    pub transition_to_next: Option<SegmentTransition>,

    /// Control flags (skip, disable, speed multiplier, etc.)
    #[serde(default)]
    pub control: SegmentControl,
}

// ─── Segment kind ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SegmentKind {
    /// Normal narrated segment — TTS runs, content displays.
    #[default]
    Speech,
    /// No TTS; holds screen for `duration_override_seconds`.
    Pause,
    /// No content; pure visual transition between chapters.
    Transition,
    /// Large centred title; automatically gets larger font.
    Title,
    /// Marks a structural chapter boundary; may trigger background change.
    Chapter,
    /// Final segment; triggers fade-to-black or end-card.
    Outro,
}

// ─── Mood + Pacing ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
#[serde(rename_all = "lowercase")]
pub enum Pacing {
    Slow,
    #[default]
    Normal,
    Fast,
}

// ─── Transition ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentTransition {
    #[serde(rename = "type", default)]
    pub kind: TransitionKind,
    #[serde(default = "default_transition_ms")]
    pub duration_ms: u32,
    #[serde(default)]
    pub easing: crate::Easing,
}

fn default_transition_ms() -> u32 {
    400
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionKind {
    Cut,
    #[default]
    Fade,
    Crossfade,
    WipeLeft,
    WipeRight,
    WipeUp,
    WipeDown,
    ZoomThrough,
    BlurThrough,
    None,
}

// ─── Control ─────────────────────────────────────────────────────────────────

/// Runtime control flags.  The renderer checks these before processing a segment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentControl {
    /// When false the segment is completely excluded from the render.
    #[serde(default = "bool_true")]
    pub enabled: bool,

    /// Intentional skip (different from disabled — for short-form cuts).
    #[serde(default)]
    pub skip: bool,

    /// Scale the entire segment duration (0.5 = half speed, 2.0 = double).
    #[serde(default = "one_f32")]
    pub speed_multiplier: f32,

    /// Extra silence before the segment starts (ms).
    #[serde(default)]
    pub delay_before_ms: u32,

    /// Extra silence after the segment ends (ms).
    #[serde(default)]
    pub delay_after_ms: u32,

    /// Output profile names this segment participates in.
    /// Empty = participates in all profiles.
    #[serde(default)]
    pub profiles: Vec<String>,
}

impl Default for SegmentControl {
    fn default() -> Self {
        Self {
            enabled: true,
            skip: false,
            speed_multiplier: 1.0,
            delay_before_ms: 0,
            delay_after_ms: 0,
            profiles: vec![],
        }
    }
}

fn bool_true() -> bool {
    true
}
fn one_f32() -> f32 {
    1.0
}
