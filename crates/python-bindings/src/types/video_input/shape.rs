use super::code::AppearAt;
use super::common::{Color, Opacity, Position};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct ShapeBlock {
    #[serde(default)]
    pub r#type: ShapeType,
    pub position: Position,
    pub size: [f32; 2], // [width, height] in pixels
    #[serde(default)]
    pub fill: ShapeFill,
    #[serde(default)]
    pub stroke: Option<ShapeStroke>,
    #[serde(default)]
    pub border_radius: u32, // only meaningful for Rect
    #[serde(default)]
    pub z_index: i32,
    #[serde(default)]
    pub appear_at: AppearAt,
    #[serde(default)]
    pub animation: ShapeAnimation,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum ShapeType {
    #[default]
    Rect,
    Circle,
    Ellipse,
    Line,
    Arrow,
    Polygon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct ShapeFill {
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub opacity: Opacity,
}

impl Default for ShapeFill {
    fn default() -> Self {
        Self {
            color: Color("#ffffff".to_string()),
            opacity: Opacity(0.1),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct ShapeStroke {
    pub color: Color,
    #[serde(default = "default_stroke_width")]
    pub width: f32,
    #[serde(default)]
    pub opacity: Opacity,
    /// Dash pattern [dash_length, gap_length]. Empty = solid.
    #[serde(default)]
    pub dash: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct ShapeAnimation {
    #[serde(default)]
    pub r#type: ShapeAnimationType,
    #[serde(default = "default_shape_anim_duration")]
    pub duration_ms: u32,
}

impl Default for ShapeAnimation {
    fn default() -> Self {
        Self {
            r#type: ShapeAnimationType::Null,
            duration_ms: 400,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass(skip_from_py_object)]
pub enum ShapeAnimationType {
    Draw, // stroke draws itself left-to-right / path trace
    Fade,
    Grow, // scales from 0 to full size
    #[default]
    Null,
}

fn default_stroke_width() -> f32 {
    2.0
}
fn default_shape_anim_duration() -> u32 {
    400
}
