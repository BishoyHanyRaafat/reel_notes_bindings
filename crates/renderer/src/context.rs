use animations::LayerAnimation;
use skia_safe::{Color, ColorType, ImageInfo, Surface};
use thiserror::Error;
use video_schema::BackgroundDef;

use crate::background::draw_background;
use crate::text::draw_text_layer;

// ── Public types ─────────────────────────────────────────────────────────────

/// Raw RGB pixel buffer for one frame (width × height × 3 bytes).
pub struct RgbFrame(pub Vec<u8>);

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("Failed to create Skia surface")]
    SurfaceCreation,

    #[error("Failed to read pixels from surface")]
    PixelReadback,

    #[error("Font load error: {0}")]
    FontLoad(String),

    #[error("Color parse error: {0}")]
    ColorParse(String),
}

// ── RenderContext ─────────────────────────────────────────────────────────────

/// Owns a Skia CPU-raster surface sized to the video resolution.
/// Re-used across all frames — we clear and redraw each time.
pub struct RenderContext {
    surface: Surface,
    width: i32,
    height: i32,
}

impl RenderContext {
    /// Create a new CPU-raster surface.
    pub fn new(width: u32, height: u32) -> Result<Self, RendererError> {
        let info = ImageInfo::new(
            (width as i32, height as i32),
            ColorType::RGBA8888,
            skia_safe::AlphaType::Premul,
            None,
        );
        let surface =
            Surface::new_raster(&info, None, None).ok_or(RendererError::SurfaceCreation)?;

        Ok(Self {
            surface,
            width: width as i32,
            height: height as i32,
        })
    }

    /// Draw one frame and return its raw RGB pixels.
    ///
    /// `frame_state` carries everything needed to draw this specific frame:
    /// which segment, animation progress, which words are revealed, etc.
    pub fn draw_frame(&mut self, state: &FrameState<'_>) -> Result<RgbFrame, RendererError> {
        let canvas = self.surface.canvas();
        canvas.clear(Color::BLACK);

        // ── 1. Background ─────────────────────────────────────────────────────
        draw_background(canvas, self.width, self.height, &state.background)?;

        // ── 2. Text ───────────────────────────────────────────────────────────
        if let Some(text_def) = state.text {
            draw_text_layer(
                canvas,
                self.width,
                self.height,
                text_def,
                state.revealed_word_count,
                &state.animation,
            )?;
        }

        // ── 3. Read pixels ────────────────────────────────────────────────────
        let pixel_count = (self.width * self.height * 4) as usize; // RGBA
        let mut rgba = vec![0u8; pixel_count];

        let info = ImageInfo::new(
            (self.width, self.height),
            ColorType::RGBA8888,
            skia_safe::AlphaType::Premul,
            None,
        );

        self.surface
            .read_pixels(&info, &mut rgba, (self.width * 4) as usize, (0, 0))
            .then_some(())
            .ok_or(RendererError::PixelReadback)?;

        // Convert RGBA → RGB (ffmpeg rawvideo expects RGB24 by default)
        let rgb = rgba_to_rgb(&rgba);
        Ok(RgbFrame(rgb))
    }

    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
}

// ── FrameState ────────────────────────────────────────────────────────────────

/// Everything the renderer needs to draw a single frame.
/// Built by the pipeline's render_pass for each frame number.
pub struct FrameState<'a> {
    /// Which background to draw (may differ per segment).
    pub background: &'a BackgroundDef,

    /// Text definition for the active segment.
    pub text: Option<&'a video_schema::TextDef>,

    /// How many words have been revealed so far (for word-by-word reveal).
    /// For AllAtOnce reveal this equals `usize::MAX`.
    pub revealed_word_count: usize,

    /// Layer animation applied to the active segment.
    pub animation: LayerAnimation,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn rgba_to_rgb(rgba: &[u8]) -> Vec<u8> {
    let mut rgb = Vec::with_capacity(rgba.len() / 4 * 3);
    for chunk in rgba.chunks_exact(4) {
        rgb.push(chunk[0]); // R
        rgb.push(chunk[1]); // G
        rgb.push(chunk[2]); // B
        // drop chunk[3] (alpha)
    }
    rgb
}

/// Parse a hex color string like `#0f0f0f` or `#fff` into a Skia Color.
pub fn parse_hex_color(hex: &str) -> Result<Color, RendererError> {
    let h = hex.trim_start_matches('#');
    let (r, g, b, a) = match h.len() {
        3 => {
            let r = u8::from_str_radix(&h[0..1].repeat(2), 16)?;
            let g = u8::from_str_radix(&h[1..2].repeat(2), 16)?;
            let b = u8::from_str_radix(&h[2..3].repeat(2), 16)?;
            (r, g, b, 255)
        }
        6 => {
            let r = u8::from_str_radix(&h[0..2], 16)?;
            let g = u8::from_str_radix(&h[2..4], 16)?;
            let b = u8::from_str_radix(&h[4..6], 16)?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&h[0..2], 16)?;
            let g = u8::from_str_radix(&h[2..4], 16)?;
            let b = u8::from_str_radix(&h[4..6], 16)?;
            let a = u8::from_str_radix(&h[6..8], 16)?;
            (r, g, b, a)
        }
        _ => return Err(RendererError::ColorParse(hex.to_owned())),
    };
    Ok(Color::from_argb(a, r, g, b))
}

impl From<std::num::ParseIntError> for RendererError {
    fn from(e: std::num::ParseIntError) -> Self {
        RendererError::ColorParse(e.to_string())
    }
}
