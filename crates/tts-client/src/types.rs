use thiserror::Error;

/// One word and the time range it was spoken, in seconds.
#[derive(Debug, Clone)]
pub struct WordTimestamp {
    pub word: String,
    pub start_secs: f64,
    pub end_secs: f64,
}

/// Everything the pipeline needs from one TTS call.
#[derive(Debug)]
pub struct TtsResult {
    /// Path to the generated MP3/PCM audio file on disk.
    pub audio_path: std::path::PathBuf,

    /// Duration of the audio in seconds (derived from timestamps or file).
    pub duration_secs: f64,

    /// Per-word timing, empty if timestamps were not requested.
    pub word_timestamps: Vec<WordTimestamp>,
}

#[derive(Debug, Error)]
pub enum TtsError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("ElevenLabs API error {status}: {body}")]
    Api { status: u16, body: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Missing API key — set ELEVENLABS_API_KEY env var")]
    MissingApiKey,
}
