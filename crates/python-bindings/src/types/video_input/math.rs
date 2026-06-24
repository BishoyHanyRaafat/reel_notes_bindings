use super::common::{Color, FillBackground, Position};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct MathBlock {
    /// Raw LaTeX string. Your renderer passes this to a LaTeX → raster pipeline.
    pub latex: String,

    #[serde(default)]
    pub render_mode: MathRenderMode,

    #[serde(default = "default_math_font_size")]
    pub font_size: u32,

    #[serde(default)]
    pub color: Color,

    #[serde(default)]
    pub background: Option<FillBackground>,

    #[serde(default)]
    pub position: Position,

    /// When within the segment this block becomes visible.
    #[serde(default)]
    pub reveal_at: MathRevealAt,

    /// If reveal_at = WithWord, this is the word in the segment text that triggers it.
    #[serde(default)]
    pub reveal_word: Option<String>,

    #[serde(default)]
    pub reveal_style: MathRevealStyle,

    /// Uniform scale applied after rendering. 1.0 = native size.
    #[serde(default = "default_scale")]
    pub scale: f32,
}

// ─── Render Mode ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum MathRenderMode {
    Inline, // fits within a line of text
    #[default]
    Block, // centered on its own line, larger
}

// ─── Reveal At ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum MathRevealAt {
    Start(),
    End(),
    WithWord(), // triggered by reveal_word match against word timestamps
    At(f32),    // exact second offset within the segment
}

impl Default for MathRevealAt {
    fn default() -> Self {
        MathRevealAt::Start()
    }
}

// ─── Reveal Style ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum MathRevealStyle {
    #[default]
    Appear,
    Draw, // left-to-right stroke draw effect
    Fade,
    ZoomIn,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn default_math_font_size() -> u32 {
    48
}
fn default_scale() -> f32 {
    1.0
}
