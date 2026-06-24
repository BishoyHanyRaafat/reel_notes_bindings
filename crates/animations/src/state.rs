use std::ops::Add;

use serde::{Deserialize, Serialize};

/// Visual state for one rendered layer after animation math is applied.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct LayerAnimation {
    pub opacity: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation_deg: f32,
    pub blur_px: f32,
}

impl LayerAnimation {
    pub const fn neutral() -> Self {
        Self {
            opacity: 1.0,
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation_deg: 0.0,
            blur_px: 0.0,
        }
    }

    pub(crate) fn combine(self, other: Self) -> Self {
        Self {
            opacity: self.opacity * other.opacity,
            translate_x: self.translate_x + other.translate_x,
            translate_y: self.translate_y + other.translate_y,
            scale_x: self.scale_x * other.scale_x,
            scale_y: self.scale_y * other.scale_y,
            rotation_deg: self.rotation_deg + other.rotation_deg,
            blur_px: self.blur_px.max(other.blur_px),
        }
    }
}

impl Add for LayerAnimation {
    type Output = LayerAnimation;

    fn add(self, other: LayerAnimation) -> LayerAnimation {
        self.combine(other)
    }
}
