use thiserror::Error;

#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("Failed to read script file: {0}")]
    ScriptRead(#[from] std::io::Error),

    #[error("Failed to parse script JSON: {0}")]
    ScriptParse(#[from] serde_json::Error),

    #[error("TTS error: {0}")]
    Tts(#[from] tts_client::TtsError),

    #[error("Renderer error: {0}")]
    Renderer(#[from] renderer::RendererError),

    #[error("Encoder error: {0}")]
    Encoder(#[from] encoder::EncoderError),

    #[error("No segments in script")]
    EmptyScript,

    #[error("Script validation error: {0}")]
    Validation(String),
}
