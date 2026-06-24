use super::code::AppearAt;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct AudioBlock {
    #[serde(default)]
    pub background_music: Option<BackgroundMusic>,
    #[serde(default)]
    pub sound_effect: Option<SoundEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct BackgroundMusic {
    pub src: String,
    #[serde(default = "default_bg_volume")]
    pub volume: f32,
    #[serde(default = "default_fade_ms")]
    pub fade_in_ms: u32,
    #[serde(default = "default_fade_ms")]
    pub fade_out_ms: u32,
    #[serde(default = "default_true")]
    pub r#loop: bool,
    #[serde(default)]
    pub start_at: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(skip_from_py_object)]
pub struct SoundEffect {
    pub src: String,
    #[serde(default = "default_sfx_volume")]
    pub volume: f32,
    #[serde(default)]
    pub trigger_at: AppearAt,
}

fn default_bg_volume() -> f32 {
    0.15
}
fn default_sfx_volume() -> f32 {
    0.8
}
fn default_fade_ms() -> u32 {
    500
}
fn default_true() -> bool {
    true
}
