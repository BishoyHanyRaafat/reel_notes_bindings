use super::animation::Transition;
use super::common::{Color, Opacity};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Background ───────────────────────────────────────────────────────────────
// Used at root level (global default) and optionally per-segment to override.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum Background {
    Solid(SolidBackground),
    Gradient(GradientBackground),
    Image(ImageBackground),
    Video(VideoBackground),
}

impl Default for Background {
    fn default() -> Self {
        Background::Solid(SolidBackground {
            color: Color("#0f0f0f".to_string()),
            blur: 0.0,
            brightness: 1.0,
            vignette: 0.0,
            transition: None,
        })
    }
}

// ─── Solid ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(from_py_object)]
pub struct SolidBackground {
    pub color: Color,
    #[serde(default)]
    pub blur: f32,
    #[serde(default = "default_brightness")]
    pub brightness: f32,
    #[serde(default)]
    pub vignette: f32, // 0.0 = none, 1.0 = heavy vignette
    #[serde(default)]
    pub transition: Option<Transition>,
}

// ─── Gradient ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(from_py_object)]
pub struct GradientBackground {
    pub gradient: GradientDef,
    #[serde(default)]
    pub blur: f32,
    #[serde(default = "default_brightness")]
    pub brightness: f32,
    #[serde(default)]
    pub vignette: f32,
    #[serde(default)]
    pub transition: Option<Transition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct GradientDef {
    #[serde(default)]
    pub direction: GradientDirection,
    pub stops: Vec<GradientStop>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum GradientDirection {
    #[default]
    Vertical,
    Horizontal,
    DiagonalTl,
    DiagonalTr,
    Radial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct GradientStop {
    pub color: Color,
    pub position: f32, // 0.0–1.0
}

// ─── Image ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(from_py_object)]
pub struct ImageBackground {
    pub src: String, // asset key or path
    #[serde(default)]
    pub fit: BackgroundFit,
    #[serde(default)]
    pub overlay: Option<BackgroundOverlay>,
    #[serde(default)]
    pub blur: f32,
    #[serde(default = "default_brightness")]
    pub brightness: f32,
    #[serde(default)]
    pub vignette: f32,
    #[serde(default)]
    pub transition: Option<Transition>,
}

// ─── Video ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(from_py_object)]
pub struct VideoBackground {
    pub src: String,
    #[serde(default = "default_true")]
    pub r#loop: bool,
    #[serde(default)]
    pub overlay: Option<BackgroundOverlay>,
    #[serde(default)]
    pub blur: f32,
    #[serde(default = "default_brightness")]
    pub brightness: f32,
    #[serde(default)]
    pub vignette: f32,
    #[serde(default)]
    pub transition: Option<Transition>,
}

// ─── Shared ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum BackgroundFit {
    #[default]
    Cover,
    Contain,
    Stretch,
    Tile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct BackgroundOverlay {
    pub color: Color,
    pub opacity: Opacity,
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn default_brightness() -> f32 {
    1.0
}
fn default_true() -> bool {
    true
}
