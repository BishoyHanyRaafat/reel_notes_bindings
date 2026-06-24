use super::code::AppearAt;
use super::common::{Color, Position, Shadow};
use super::text::FontWeight;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Table Block ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct TableBlock {
    pub headers: Vec<String>,

    /// Each row is a Vec of CellValue — one per column.
    pub rows: Vec<Vec<CellValue>>,

    #[serde(default)]
    pub style: TableStyle,

    /// Semantic type per column — tells your renderer how to format and align.
    /// Length must match headers.len(). Defaults to Text for all columns.
    #[serde(default)]
    pub column_types: Vec<ColumnType>,

    /// Width per column as fraction of table width. Must sum to ~1.0.
    /// If empty, columns are equal width.
    #[serde(default)]
    pub column_widths: Vec<f32>,

    /// Symbol prepended to currency-typed cells.
    #[serde(default = "default_currency")]
    pub currency_symbol: String,

    #[serde(default)]
    pub highlight_rows: Vec<RowHighlight>,

    #[serde(default)]
    pub highlight_cols: Vec<ColHighlight>,

    #[serde(default)]
    pub highlight_cells: Vec<CellHighlight>,

    /// Enables delta/arrow comparison between rows on a specific column.
    #[serde(default)]
    pub comparison: Option<ComparisonConfig>,

    /// Appends a totals row at the bottom.
    #[serde(default)]
    pub totals_row: Option<TotalsRow>,

    #[serde(default)]
    pub font: TableFont,

    #[serde(default)]
    pub colors: TableColors,

    #[serde(default)]
    pub position: Position,

    /// Width as fraction of canvas width.
    #[serde(default = "default_table_width")]
    pub width: f32,

    #[serde(default)]
    pub reveal: TableRevealMode,

    #[serde(default)]
    pub reveal_interval_ms: u32,

    #[serde(default)]
    pub appear_at: AppearAt,

    #[serde(default)]
    pub z_index: i32,

    #[serde(default)]
    pub caption: Option<TableCaption>,

    #[serde(default)]
    pub border_radius: u32,

    #[serde(default)]
    pub shadow: Option<Shadow>,
}

// ─── Cell Value ───────────────────────────────────────────────────────────────
// Flexible cell content — the column_type tells the renderer how to display it.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[pyclass(skip_from_py_object)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Bool(bool),
    Null(),
}

impl std::fmt::Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellValue::Text(s) => write!(f, "{}", s),
            CellValue::Number(n) => write!(f, "{}", n),
            CellValue::Bool(b) => write!(f, "{}", b),
            CellValue::Null() => write!(f, ""),
        }
    }
}

// ─── Column Type ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum ColumnType {
    #[default]
    Text,
    Number,
    Currency,   // formatted with currency_symbol, right-aligned
    Percentage, // appends %, right-aligned
    Boolean,    // renders ✓ / ✗ or Yes / No
    Badge,      // pill-shaped colored label
    Bar,        // horizontal progress bar (0.0–1.0 value)
}

// ─── Highlights ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct RowHighlight {
    pub row_index: usize,
    pub color: Color,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct ColHighlight {
    pub col_index: usize,
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct CellHighlight {
    pub row: usize,
    pub col: usize,
    pub color: Color,
    #[serde(default)]
    pub text_color: Option<Color>,
}

// ─── Comparison Config ────────────────────────────────────────────────────────
// Compares numeric values in a column and shows deltas / arrows.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct ComparisonConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Whether higher or lower values in compare_col_index are "better".
    #[serde(default)]
    pub better_is: BetterIs,

    /// Column index to compare values across rows.
    pub compare_col_index: usize,

    #[serde(default = "default_positive_color")]
    pub positive_color: Color,

    #[serde(default = "default_negative_color")]
    pub negative_color: Color,

    /// Append a delta column (e.g. "+12.3%") next to the comparison column.
    #[serde(default = "default_true")]
    pub show_delta: bool,

    /// Show ↑ / ↓ arrows in the delta column.
    #[serde(default = "default_true")]
    pub show_arrows: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum BetterIs {
    #[default]
    Higher,
    Lower,
}

// ─── Totals Row ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct TotalsRow {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_totals_label")]
    pub label: String,
    #[serde(default)]
    pub color: Option<Color>,
    #[serde(default = "default_true")]
    pub bold: bool,
}

// ─── Table Style ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum TableStyle {
    #[default]
    Minimal,
    Bordered,
    Striped,
    Accounting, // right-aligned numbers, currency formatting, totals emphasis
    Comparison, // wider columns, delta arrows, color-coded values
    DarkCard,   // rounded card with dark background per row
}

// ─── Table Reveal ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum TableRevealMode {
    #[default]
    AllAtOnce,
    RowByRow,
    ColByCol,
}

// ─── Table Font ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct TableFont {
    #[serde(default = "default_table_font")]
    pub family: String,
    #[serde(default = "default_header_size")]
    pub header_size: u32,
    #[serde(default = "default_cell_size")]
    pub cell_size: u32,
    #[serde(default)]
    pub header_weight: FontWeight,
    #[serde(default)]
    pub cell_weight: FontWeight,
}

impl Default for TableFont {
    fn default() -> Self {
        Self {
            family: default_table_font(),
            header_size: default_header_size(),
            cell_size: default_cell_size(),
            header_weight: FontWeight::Semibold,
            cell_weight: FontWeight::Regular,
        }
    }
}

// ─── Table Colors ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct TableColors {
    #[serde(default = "default_header_bg")]
    pub header_bg: Color,
    #[serde(default = "default_header_text")]
    pub header_text: Color,
    #[serde(default = "default_row_bg")]
    pub row_bg: Color,
    #[serde(default = "default_row_alt_bg")]
    pub row_alt_bg: Color,
    #[serde(default = "default_cell_text")]
    pub cell_text: Color,
    #[serde(default = "default_border_color")]
    pub border: Color,
}

impl Default for TableColors {
    fn default() -> Self {
        Self {
            header_bg: Color("#1e1e2e".to_string()),
            header_text: Color("#ffffff".to_string()),
            row_bg: Color("#161622".to_string()),
            row_alt_bg: Color("#1a1a2a".to_string()),
            cell_text: Color("#e0e0e0".to_string()),
            border: Color("#2a2a3a".to_string()),
        }
    }
}

// ─── Caption ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct TableCaption {
    pub text: String,
    #[serde(default)]
    pub position: CaptionPosition,
    #[serde(default = "default_caption_size")]
    pub font_size: u32,
    #[serde(default)]
    pub color: Color,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum CaptionPosition {
    Above,
    #[default]
    Below,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn default_true() -> bool {
    true
}
fn default_currency() -> String {
    "$".to_string()
}
fn default_table_width() -> f32 {
    0.85
}
fn default_table_font() -> String {
    "Inter".to_string()
}
fn default_header_size() -> u32 {
    24
}
fn default_cell_size() -> u32 {
    22
}
fn default_caption_size() -> u32 {
    18
}
fn default_totals_label() -> String {
    "Total".to_string()
}
fn default_positive_color() -> Color {
    Color("#4CAF50".to_string())
}
fn default_negative_color() -> Color {
    Color("#F44336".to_string())
}
fn default_header_bg() -> Color {
    Color("#1e1e2e".to_string())
}
fn default_header_text() -> Color {
    Color("#ffffff".to_string())
}
fn default_row_bg() -> Color {
    Color("#161622".to_string())
}
fn default_row_alt_bg() -> Color {
    Color("#1a1a2a".to_string())
}
fn default_cell_text() -> Color {
    Color("#e0e0e0".to_string())
}
fn default_border_color() -> Color {
    Color("#2a2a3a".to_string())
}
