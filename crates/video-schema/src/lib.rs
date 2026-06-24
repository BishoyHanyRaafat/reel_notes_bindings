//! Core data model for video scripts.
//!
//! This crate defines the serializable schema shared by the CLI, builder,
//! renderer, timing, and Python bindings. It intentionally stays free of I/O
//! and runtime pipeline logic.

pub mod audio;
pub mod elevenlabs;
pub mod root;
pub mod segment;
pub mod text;

// Flat re-exports so callers write `video_schema::LessonScript` not the full path
pub use audio::*;
pub use elevenlabs::*;
pub use root::*;
pub use segment::*;
pub use text::*;
