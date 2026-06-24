use super::common::Easing;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Animate In ───────────────────────────────────────────────────────────────

#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationIn {
    #[serde(default)]
    pub r#type: AnimationInType,
    #[serde(default = "default_anim_duration")]
    pub duration_ms: u32,
    #[serde(default)]
    pub easing: Easing,
    #[serde(default)]
    pub delay_ms: u32,
    #[serde(default)]
    pub target: AnimationTarget,
}

impl Default for AnimationIn {
    fn default() -> Self {
        Self {
            r#type: AnimationInType::Fade,
            duration_ms: 300,
            easing: Easing::EaseOut,
            delay_ms: 0,
            target: AnimationTarget::WholeSegment,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum AnimationInType {
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
    Bounce,
    Swing,
    Null,
}

// ─── Animate Out ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct AnimationOut {
    #[serde(default)]
    pub r#type: AnimationOutType,
    #[serde(default = "default_anim_duration")]
    pub duration_ms: u32,
    #[serde(default)]
    pub easing: Easing,
    #[serde(default)]
    pub target: AnimationTarget,
}

impl Default for AnimationOut {
    fn default() -> Self {
        Self {
            r#type: AnimationOutType::Fade,
            duration_ms: 300,
            easing: Easing::EaseIn,
            target: AnimationTarget::WholeSegment,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
#[serde(rename_all = "snake_case")]
pub enum AnimationOutType {
    #[default]
    Fade,
    SlideUp,
    SlideDown,
    SlideLeft,
    SlideRight,
    ZoomOut,
    ZoomIn,
    BlurOut,
    Null,
}

// ─── Body Animation ───────────────────────────────────────────────────────────
// Runs while the segment is fully visible — looping or one-shot subtle motion.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct BodyAnimation {
    #[serde(default)]
    pub r#type: BodyAnimationType,
    #[serde(default = "default_body_duration")]
    pub duration_ms: u32,
    #[serde(default = "default_amplitude")]
    pub amplitude: f32,
    #[serde(default)]
    pub repeat: RepeatMode,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum BodyAnimationType {
    #[default]
    Null,
    Pulse,
    Float,
    Breathe,
    Shake,
    Glitch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
#[serde(rename_all = "snake_case")]
pub enum RepeatMode {
    Once(),
    Loop(),
    Count(u32),
}

impl Default for RepeatMode {
    fn default() -> Self {
        RepeatMode::Loop()
    }
}

// ─── Stagger ──────────────────────────────────────────────────────────────────
// Controls how multiple elements inside a segment animate relative to each other.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct Stagger {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_stagger_delay")]
    pub delay_between_elements_ms: u32,
    #[serde(default)]
    pub order: StaggerOrder,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum StaggerOrder {
    #[default]
    Sequential,
    Random,
    CenterOut,
    OutsideIn,
}

// ─── Segment Animation ────────────────────────────────────────────────────────
// Top-level animation block on a segment, composing all of the above.

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct SegmentAnimation {
    #[serde(rename = "in", default)]
    pub animation_in: AnimationIn,
    #[serde(rename = "out", default)]
    pub animation_out: AnimationOut,
    #[serde(default)]
    pub body: Option<BodyAnimation>,
    #[serde(default)]
    pub stagger: Option<Stagger>,
}

// ─── Transition ───────────────────────────────────────────────────────────────
// Applied between this segment and the next one.

#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    #[serde(default)]
    pub r#type: TransitionType,
    #[serde(default = "default_transition_duration")]
    pub duration_ms: u32,
    #[serde(default)]
    pub easing: Easing,
}

#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionType {
    Cut,
    #[default]
    Crossfade,
    Fade,
    WipeLeft,
    WipeRight,
    WipeUp,
    WipeDown,
    ZoomThrough,
    BlurThrough,
    Null,
}

// ─── Animation Target ─────────────────────────────────────────────────────────

#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimationTarget {
    #[default]
    WholeSegment,
    TextOnly,
    MediaOnly,
    PerElement,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn default_anim_duration() -> u32 {
    300
}
fn default_body_duration() -> u32 {
    2000
}
fn default_amplitude() -> f32 {
    0.5
}
fn default_stagger_delay() -> u32 {
    80
}
fn default_transition_duration() -> u32 {
    500
}
