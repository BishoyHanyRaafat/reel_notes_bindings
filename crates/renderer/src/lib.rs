//! Frame renderer for the video pipeline.
//!
//! This crate turns a `FrameState` into raw pixels using Skia. It knows how to
//! draw backgrounds and text, but it does not decide timing or audio.

mod background;
mod context;
mod text;

pub use context::{FrameState, RenderContext, RendererError};

// Re-export the pixel buffer type so encoder doesn't depend on skia-safe directly.
pub use context::RgbFrame;
