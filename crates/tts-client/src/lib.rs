//! ElevenLabs client and TTS result types.
//!
//! This crate is responsible for turning segment text into audio files and word
//! timestamps. It has no knowledge of rendering or timeline assembly.

mod client;
mod types;

pub use client::TtsClient;
pub use types::{TtsError, TtsResult, WordTimestamp};
