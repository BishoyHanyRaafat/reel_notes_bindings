//! FFmpeg-backed video encoder.
//!
//! This crate owns the raw frame pipe, intermediate video file, and final audio
//! mux step. It does not know about segments, text reveal, or animations.

mod ffmpeg;
pub use ffmpeg::{AudioSpan, EncoderConfig, EncoderError, FfmpegEncoder};
