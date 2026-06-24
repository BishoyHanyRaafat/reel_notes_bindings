use super::animation::AnimationIn;
use super::code::CodeBlock;
use super::common::Color;
use super::common::PartAppearAt;
use super::image::ImageBlock;
use super::math::MathBlock;
use super::table::TableBlock;
use super::text::TextBlock;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Segment Part ─────────────────────────────────────────────────────────────
// A sub-unit within a segment that appears at a specific moment.
// Parts are revealed sequentially in sync with narration — either by
// audio cue word matching or by explicit time offset.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct SegmentPart {
    /// Unique identifier within this segment. Referenced by other parts
    /// via PartAppearAt::WithPartId.
    pub id: String,

    /// Optional display label — can be shown as a step counter or subtitle.
    #[serde(default)]
    pub label: Option<String>,

    // ── Content — at most one primary content type per part ──────────────────
    #[serde(default)]
    pub text: Option<TextBlock>,
    #[serde(default)]
    pub code: Option<CodeBlock>,
    #[serde(default)]
    pub math: Option<MathBlock>,
    #[serde(default)]
    pub table: Option<TableBlock>,
    #[serde(default)]
    pub images: Vec<ImageBlock>,

    /// When this part becomes visible within the segment.
    #[serde(default)]
    pub appear_at: PartAppearAt,

    /// How this part animates in.
    #[serde(default)]
    pub transition_in: AnimationIn,

    /// If true, the previous part remains visible while this one appears.
    /// If false, the previous part animates out before this part appears.
    #[serde(default = "default_true")]
    pub hold_previous: bool,

    /// Dims the previous part while this one is active — draws focus.
    #[serde(default)]
    pub highlight_previous_part: Option<PreviousPartHighlight>,

    /// Seconds to hold this part fully visible before it either exits
    /// or the next part begins (whichever is first).
    #[serde(default)]
    pub post_hold_seconds: f32,
}

// ─── Previous Part Highlight ──────────────────────────────────────────────────
// When active, dims the previous part and optionally draws a border around it.
// Used to keep context visible while shifting focus to the new part.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct PreviousPartHighlight {
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// How much to dim the previous part. 0.0 = invisible, 1.0 = unchanged.
    #[serde(default = "default_dim_opacity")]
    pub dim_opacity: f32,
    /// Optional border drawn around the previous part's bounding box.
    #[serde(default)]
    pub border_color: Option<Color>,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn default_true() -> bool {
    true
}
fn default_dim_opacity() -> f32 {
    0.35
}
