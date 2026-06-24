use super::common::{Border, Color, FillBackground, PartAppearAt, Position, Shadow, Size};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Code Block ───────────────────────────────────────────────────────────────
#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    #[serde(default)]
    pub language: CodeLanguage,

    /// The full source code string. Use \n for newlines.
    pub source: String,

    #[serde(default)]
    pub theme: CodeTheme,

    #[serde(default)]
    pub font: CodeFont,

    #[serde(default)]
    pub position: Position,

    #[serde(default)]
    pub size: CodeSize,

    #[serde(default)]
    pub line_numbers: bool,

    /// Lines to highlight from the start (1-indexed).
    /// Use parts for time-sequenced highlighting instead.
    #[serde(default)]
    pub highlight_lines: Vec<u32>,

    /// How the code content initially appears.
    #[serde(default)]
    pub reveal: CodeRevealMode,

    #[serde(default)]
    pub reveal_sync: CodeRevealSync,

    /// Used when reveal_sync = Timed.
    #[serde(default = "default_reveal_interval")]
    pub reveal_interval_ms: u32,

    /// Sequential code parts revealed in sync with narration.
    /// Each part highlights specific lines and optionally shows an annotation.
    #[serde(default)]
    pub parts: Vec<CodePart>,

    #[serde(default)]
    pub background: Option<FillBackground>,

    #[serde(default)]
    pub border: Option<Border>,

    #[serde(default)]
    pub border_radius: u32,

    /// Renders a fake macOS/VS Code window chrome above the code block.
    #[serde(default)]
    pub window_chrome: bool,

    #[serde(default)]
    pub window_title: Option<String>,

    #[serde(default)]
    pub shadow: Option<Shadow>,

    /// When during the segment this block appears. Default = start of segment.
    #[serde(default)]
    pub appear_at: AppearAt,

    #[serde(default)]
    pub z_index: i32,
}

// ─── Code Part ────────────────────────────────────────────────────────────────
// Represents one step in a sequenced code explanation.
// Parts appear in order, each highlighting specific lines and optionally
// adding an annotation arrow pointing to that region.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct CodePart {
    /// Unique identifier — referenced by other parts via appear_at = WithPartId.
    pub id: String,

    /// Optional display label shown as a subtitle or step marker.
    #[serde(default)]
    pub label: Option<String>,

    /// Line range to highlight. [start, end] inclusive, 1-indexed.
    pub lines: [u32; 2],

    /// Semi-transparent highlight color over the selected lines.
    #[serde(default)]
    pub highlight_color: Option<Color>,

    /// When this part becomes active.
    #[serde(default)]
    pub appear_at: PartAppearAt,

    #[serde(default)]
    pub annotation: Option<CodeAnnotation>,
}

// ─── Code Annotation ──────────────────────────────────────────────────────────
// A text callout with optional arrow pointing at the highlighted lines.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct CodeAnnotation {
    pub text: String,
    #[serde(default)]
    pub position: AnnotationPosition,
    #[serde(default)]
    pub color: Color,
    #[serde(default = "default_true")]
    pub arrow: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum AnnotationPosition {
    #[default]
    Right,
    Left,
    Above,
    Below,
}

// ─── Appear At (block level) ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum AppearAt {
    Start(),
    End(),
    At(f32),
}

impl Default for AppearAt {
    fn default() -> Self {
        AppearAt::Start()
    }
}

// ─── Reveal Mode ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum CodeRevealMode {
    #[default]
    AllAtOnce,
    LineByLine,
    BlockByBlock,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum CodeRevealSync {
    #[default]
    Audio,
    Timed,
}

// ─── Code Size ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct CodeSize {
    /// Fraction of canvas width. 0.6 = 60% of canvas.
    #[serde(default = "default_code_width")]
    pub width: f32,
    /// "auto" stretches to fit content; or explicit fraction.
    #[serde(default)]
    pub height: Size,
}

impl Default for CodeSize {
    fn default() -> Self {
        Self {
            width: default_code_width(),
            height: Size::default(),
        }
    }
}

// ─── Code Font ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct CodeFont {
    #[serde(default = "default_code_font_family")]
    pub family: String,
    #[serde(default = "default_code_font_size")]
    pub size: u32,
    #[serde(default = "default_line_height")]
    pub line_height: f32,
}

impl Default for CodeFont {
    fn default() -> Self {
        Self {
            family: default_code_font_family(),
            size: default_code_font_size(),
            line_height: default_line_height(),
        }
    }
}

// ─── Language ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum CodeLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Cpp,
    C,
    Java,
    Kotlin,
    Swift,
    Sql,
    Bash,
    Json,
    Yaml,
    Toml,
    Markdown,
    Pseudocode,
    #[default]
    Plaintext,
}

// ─── Theme ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum CodeTheme {
    #[default]
    OneDark,
    Dark,
    Light,
    Monokai,
    Dracula,
    GithubDark,
    SolarizedDark,
    Nord,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn default_reveal_interval() -> u32 {
    300
}
fn default_true() -> bool {
    true
}
fn default_code_width() -> f32 {
    0.65
}
fn default_code_font_family() -> String {
    "JetBrains Mono".to_string()
}
fn default_code_font_size() -> u32 {
    28
}
fn default_line_height() -> f32 {
    1.6
}
