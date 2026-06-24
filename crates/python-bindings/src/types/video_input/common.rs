use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Color ────────────────────────────────────────────────────────────────────
// Stored as a raw string so the LLM can pass "#hex", "rgb()", "rgba()" freely.
// Your renderer is responsible for parsing this into whatever Skia needs.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
#[pyclass(skip_from_py_object)]
pub struct Color(pub String);

impl Default for Color {
    fn default() -> Self {
        Color("#ffffff".to_string())
    }
}

// ─── Position ─────────────────────────────────────────────────────────────────
// Named anchors or a 0.0–1.0 relative value on that axis.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[pyclass(skip_from_py_object)]
pub enum AxisPosition {
    Named(NamedPosition),
    Relative(f32), // 0.0 = left/top edge, 1.0 = right/bottom edge
}

impl Default for AxisPosition {
    fn default() -> Self {
        AxisPosition::Named(NamedPosition::Center)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(from_py_object)]
pub enum NamedPosition {
    Left,
    Center,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct Position {
    #[serde(default)]
    pub x: AxisPosition,
    #[serde(default)]
    pub y: AxisPosition,
}

// ─── Size ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[pyclass(skip_from_py_object)]
pub enum SizeValue {
    Auto(),        // "auto"
    Fraction(f32), // 0.0–1.0 fraction of canvas dimension
    Pixels(u32),   // absolute pixel value
}

impl Default for SizeValue {
    fn default() -> Self {
        SizeValue::Auto()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct Size {
    #[serde(default)]
    pub width: SizeValue,
    #[serde(default)]
    pub height: SizeValue,
}

// ─── Padding ──────────────────────────────────────────────────────────────────
// [top, right, bottom, left] in pixels — same as CSS shorthand order.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[pyclass(skip_from_py_object)]
pub struct Padding(pub [u32; 4]);

#[allow(dead_code)]
impl Padding {
    pub fn top(&self) -> u32 {
        self.0[0]
    }
    pub fn right(&self) -> u32 {
        self.0[1]
    }
    pub fn bottom(&self) -> u32 {
        self.0[2]
    }
    pub fn left(&self) -> u32 {
        self.0[3]
    }
}

// ─── Shadow ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct Shadow {
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub blur: f32,
    #[serde(default)]
    pub offset_x: f32,
    #[serde(default)]
    pub offset_y: f32,
}

// ─── Border ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct Border {
    #[serde(default = "default_border_width")]
    pub width: u32,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub style: BorderStyle,
}

fn default_border_width() -> u32 {
    1
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum BorderStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

// ─── Background Fill ──────────────────────────────────────────────────────────
// Reused on text boxes, code blocks, table cells, etc.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct FillBackground {
    #[serde(default)]
    pub color: Color,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default)]
    pub border_radius: u32,
    #[serde(default)]
    pub padding: Padding,
}

fn default_opacity() -> f32 {
    1.0
}

// ─── Easing ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum Easing {
    Linear,
    EaseIn,
    #[default]
    EaseOut,
    EaseInOut,
    Spring,
    Bounce,
}

// ─── Opacity ──────────────────────────────────────────────────────────────────
// Newtype so we can enforce 0.0–1.0 at the type level if we add validation later.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
#[pyclass(skip_from_py_object)]
pub struct Opacity(pub f32);

impl Default for Opacity {
    fn default() -> Self {
        Opacity(1.0)
    }
}

// ─── Resolution ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct Resolution(pub u32, pub u32);

impl Default for Resolution {
    fn default() -> Self {
        Resolution(1920, 1080)
    }
}

// ─── Aspect Ratio ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub enum AspectRatio {
    #[serde(rename = "16:9")]
    #[default]
    Wide,
    #[serde(rename = "9:16")]
    Vertical,
    #[serde(rename = "1:1")]
    Square,
    #[serde(rename = "4:3")]
    Classic,
}

// ─── Part Appear At ───────────────────────────────────────────────────────────
// Controls when a CodePart becomes active within the segment.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum PartAppearAt {
    /// Becomes active at the very start of the segment.
    Start(),
    /// Becomes active when the narrator says the matched audio_cue word.
    WithAudioCue(String),
    /// Becomes active right after the part with this ID finishes.
    WithPartId(String),
    /// Becomes active at this many seconds into the segment.
    At(f32),
}

impl Default for PartAppearAt {
    fn default() -> Self {
        PartAppearAt::Start()
    }
}
