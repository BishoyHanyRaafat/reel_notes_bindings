use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDef {
    pub content: String,

    /// Words/phrases to style differently when spoken.
    #[serde(default)]
    pub key_terms: Vec<KeyTerm>,

    #[serde(default)]
    pub reveal: TextReveal,

    #[serde(default)]
    pub reveal_sync: RevealSync,

    /// Used when reveal_sync = Timed. Gap between words/lines in ms.
    pub reveal_interval_ms: Option<u32>,

    pub font: Option<FontDef>,

    pub color: Option<String>,

    #[serde(default = "one_f32")]
    pub opacity: f32,

    #[serde(default)]
    pub align: TextAlign,

    pub position: Option<Position2D>,

    /// Fraction of canvas width, 0.0–1.0.
    pub max_width: Option<f32>,

    /// [top, right, bottom, left] in pixels.
    pub padding: Option<[u32; 4]>,

    pub shadow: Option<ShadowDef>,

    /// Background pill/card behind the text.
    pub background: Option<TextBackground>,
}

fn one_f32() -> f32 {
    1.0
}

// ─── Key terms ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyTerm {
    pub word: String,
    pub color: Option<String>,
    #[serde(default = "one_f32")]
    pub scale: f32,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub underline: bool,
}

// ─── Reveal ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextReveal {
    #[default]
    AllAtOnce,
    WordByWord,
    LineByLine,
    Typewriter,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevealSync {
    /// Word appearance is tied to ElevenLabs word timestamps.
    #[default]
    Audio,
    /// Word appearance uses a fixed interval (reveal_interval_ms).
    Timed,
}

// ─── Font ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontDef {
    pub family: Option<String>,
    #[serde(default = "default_font_size")]
    pub size: u32,
    #[serde(default)]
    pub weight: FontWeight,
    #[serde(default)]
    pub style: FontStyle,
    pub letter_spacing: Option<f32>,
    #[serde(default = "default_line_height")]
    pub line_height: f32,
}

fn default_font_size() -> u32 {
    64
}
fn default_line_height() -> f32 {
    1.4
}

impl Default for FontDef {
    fn default() -> Self {
        Self {
            family: None,
            size: default_font_size(),
            weight: FontWeight::default(),
            style: FontStyle::default(),
            letter_spacing: None,
            line_height: default_line_height(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
#[serde(rename_all = "lowercase")]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
}

// ─── Layout primitives ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AxisPosition {
    Named(NamedPosition),
    /// 0.0 = left/top edge, 1.0 = right/bottom edge.
    Relative(f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NamedPosition {
    Left,
    Center,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position2D {
    pub x: AxisPosition,
    pub y: AxisPosition,
}

impl Position2D {
    /// Resolve to pixel coordinates given canvas width and height.
    pub fn resolve(&self, canvas_w: f32, canvas_h: f32) -> (f32, f32) {
        let x = match &self.x {
            AxisPosition::Named(NamedPosition::Left) => 0.0,
            AxisPosition::Named(NamedPosition::Center) => canvas_w / 2.0,
            AxisPosition::Named(NamedPosition::Right) => canvas_w,
            AxisPosition::Relative(v) => canvas_w * v,
            _ => canvas_w / 2.0,
        };
        let y = match &self.y {
            AxisPosition::Named(NamedPosition::Top) => 0.0,
            AxisPosition::Named(NamedPosition::Center) => canvas_h / 2.0,
            AxisPosition::Named(NamedPosition::Bottom) => canvas_h,
            AxisPosition::Relative(v) => canvas_h * v,
            _ => canvas_h / 2.0,
        };
        (x, y)
    }
}

// ─── Visual effects ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowDef {
    pub color: String,
    #[serde(default)]
    pub blur: f32,
    #[serde(default)]
    pub offset_x: f32,
    #[serde(default)]
    pub offset_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBackground {
    pub color: String,
    #[serde(default = "one_f32")]
    pub opacity: f32,
    #[serde(default)]
    pub border_radius: f32,
    pub padding: Option<[u32; 4]>,
}

// ─── Alignment ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextAlign {
    Left,
    #[default]
    Center,
    Right,
    Justify,
}
