use super::common::{Color, FillBackground, Opacity, Padding, Position, Shadow};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Text Block ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct TextBlock {
    pub content: String,

    #[serde(default)]
    pub key_terms: Vec<KeyTerm>,

    #[serde(default)]
    pub reveal: TextRevealMode,

    /// Only meaningful when reveal is WordByWord or Typewriter.
    /// "audio" = driven by ElevenLabs word timestamps (preferred).
    /// "timed" = evenly spaced by reveal_interval_ms.
    #[serde(default)]
    pub reveal_sync: RevealSync,

    /// Used when reveal_sync = Timed. Ignored otherwise.
    #[serde(default = "default_reveal_interval")]
    pub reveal_interval_ms: u32,

    #[serde(default)]
    pub font: FontConfig,

    #[serde(default)]
    pub color: Color,

    #[serde(default)]
    pub opacity: Opacity,

    #[serde(default)]
    pub align: TextAlign,

    #[serde(default)]
    pub position: Position,

    /// Max width as fraction of canvas width. 1.0 = full width.
    #[serde(default = "default_max_width")]
    pub max_width: f32,

    #[serde(default)]
    pub padding: Padding,

    #[serde(default)]
    pub shadow: Option<Shadow>,

    /// Background box behind the text (pill/card style).
    #[serde(default)]
    pub background: Option<FillBackground>,
}

// ─── Key Term ─────────────────────────────────────────────────────────────────
// Words or phrases in the text content that receive special visual treatment.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct KeyTerm {
    /// Must match a substring of TextBlock.content exactly.
    pub word: String,
    #[serde(default)]
    pub color: Option<Color>,
    #[serde(default = "default_key_term_scale")]
    pub scale: f32,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub background: Option<FillBackground>,
}

// ─── Font Config ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct FontConfig {
    #[serde(default = "default_font_family")]
    pub family: String,
    #[serde(default = "default_font_size")]
    pub size: u32,
    #[serde(default)]
    pub weight: FontWeight,
    #[serde(default)]
    pub style: FontStyle,
    #[serde(default = "default_letter_spacing")]
    pub letter_spacing: f32,
    #[serde(default = "default_line_height")]
    pub line_height: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: default_font_family(),
            size: default_font_size(),
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
            letter_spacing: 0.0,
            line_height: 1.4,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum FontWeight {
    Thin,
    Light,
    #[default]
    Regular,
    Medium,
    Semibold,
    Bold,
    Black,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
}

// ─── Reveal ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum TextRevealMode {
    #[default]
    AllAtOnce,
    WordByWord,
    LineByLine,
    Typewriter,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum RevealSync {
    /// Driven by ElevenLabs word timestamps — tight audio-visual sync.
    #[default]
    Audio,
    /// Evenly distributed over reveal_interval_ms — no audio dependency.
    Timed,
}

// ─── Align ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum TextAlign {
    Left,
    #[default]
    Center,
    Right,
    Justify,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn default_font_family() -> String {
    "Inter".to_string()
}
fn default_font_size() -> u32 {
    48
}
fn default_letter_spacing() -> f32 {
    0.0
}
fn default_line_height() -> f32 {
    1.4
}
fn default_max_width() -> f32 {
    0.85
}
fn default_reveal_interval() -> u32 {
    80
}
fn default_key_term_scale() -> f32 {
    1.0
}
