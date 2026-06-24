use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct Layout {
    #[serde(default)]
    pub mode: LayoutMode,
    /// Only for SplitLeft / SplitRight. Fraction of canvas for the primary pane.
    #[serde(default = "default_split_ratio")]
    pub split_ratio: f32,
    /// Only for Grid mode.
    #[serde(default = "default_columns")]
    pub columns: u32,
    #[serde(default = "default_rows")]
    pub rows: u32,
    #[serde(default = "default_gap")]
    pub gap: u32,
    /// Safe zone margins as fraction of canvas dimension.
    #[serde(default)]
    pub safe_zone: SafeZone,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            mode: LayoutMode::Free,
            split_ratio: default_split_ratio(),
            columns: default_columns(),
            rows: default_rows(),
            gap: default_gap(),
            safe_zone: SafeZone::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum LayoutMode {
    #[default]
    Free, // elements positioned by their own position fields
    SplitLeft,  // text left, media right
    SplitRight, // media left, text right
    Overlay,    // media fills canvas, text overlaid
    Grid,       // elements arranged in a column/row grid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct SafeZone {
    #[serde(default = "default_safe")]
    pub top: f32,
    #[serde(default = "default_safe")]
    pub right: f32,
    #[serde(default = "default_safe")]
    pub bottom: f32,
    #[serde(default = "default_safe")]
    pub left: f32,
}

impl Default for SafeZone {
    fn default() -> Self {
        Self {
            top: 0.05,
            right: 0.05,
            bottom: 0.05,
            left: 0.05,
        }
    }
}

fn default_split_ratio() -> f32 {
    0.5
}
fn default_columns() -> u32 {
    2
}
fn default_rows() -> u32 {
    1
}
fn default_gap() -> u32 {
    32
}
fn default_safe() -> f32 {
    0.05
}
