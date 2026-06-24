use super::common::{AspectRatio, Color};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[pyclass(skip_from_py_object)]
pub struct ImageBlock {
    pub description: String,
    #[serde(default)]
    pub aspect_ratio: AspectRatio,
    #[serde(default)]
    pub style: ImageStyle,
    #[serde(default)]
    pub mood: ImageMood,
    #[serde(default)]
    pub color_palette: ColorPalette,
    pub negative_prompt: Option<String>,
    pub placeholder: Option<Placeholder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[pyclass(skip_from_py_object)]
pub struct Placeholder {
    pub show: bool,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub icon: PlaceholderIcon,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[pyclass(skip_from_py_object)]
pub enum ImageStyle {
    #[default]
    Realistic,
    Illustration,
    Diagram,
    FlatIcon,
    Render3D,
    Sketch,
    Infographic,
    Chart,
    Whiteboard,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[pyclass(skip_from_py_object)]
pub enum ImageMood {
    #[default]
    Neutral,
    Professional,
    Friendly,
    Serious,
    Technical,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[pyclass(skip_from_py_object)]
pub enum ColorPalette {
    MatchBackground,
    Vibrant,
    Muted,
    Monochrome,
    Warm,
    #[default]
    Cool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[pyclass(skip_from_py_object)]
pub enum PlaceholderIcon {
    #[default]
    Image,
    Chart,
    Diagram,
    Null,
}
