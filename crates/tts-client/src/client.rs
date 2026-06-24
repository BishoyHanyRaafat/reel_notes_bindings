use crate::types::{TtsError, TtsResult, WordTimestamp};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{debug, info};
use video_schema::ElevenLabsConfig;

const BASE_URL: &str = "https://api.elevenlabs.io/v1";

pub struct TtsClient {
    http: Client,
    api_key: String,
    /// Directory where audio files are written.
    out_dir: PathBuf,
}

impl TtsClient {
    /// Create a client.  Reads `ELEVENLABS_API_KEY` from the environment.
    pub fn from_env(out_dir: PathBuf) -> Result<Self, TtsError> {
        let api_key = std::env::var("ELEVENLABS_API_KEY").map_err(|_| TtsError::MissingApiKey)?;
        Ok(Self {
            http: Client::new(),
            api_key,
            out_dir,
        })
    }

    /// Synthesise `text` and write audio to `{out_dir}/{segment_id}.mp3`.
    /// Returns timing metadata and the path.
    pub async fn synthesise(
        &self,
        segment_id: u32,
        text: &str,
        cfg: &ElevenLabsConfig,
    ) -> Result<TtsResult, TtsError> {
        info!(segment_id, "TTS request");

        // ── Build request body ────────────────────────────────────────────────
        let body = TtsRequest {
            text: text.to_owned(),
            model_id: cfg.model.as_str().to_owned(),
            voice_settings: VoiceSettings {
                stability: cfg.stability,
                similarity_boost: cfg.similarity_boost,
                style: cfg.style,
                use_speaker_boost: cfg.use_speaker_boost,
                speed: cfg.speed,
            },
        };

        // ── Choose endpoint ───────────────────────────────────────────────────
        // If word timestamps are requested we call the /with-timestamps endpoint
        // which returns JSON. Otherwise we call the plain streaming endpoint.
        if cfg.request_word_timestamps {
            self.synthesise_with_timestamps(segment_id, body, cfg).await
        } else {
            self.synthesise_plain(segment_id, body, cfg).await
        }
    }

    // ── Plain audio (no timestamps) ──────────────────────────────────────────

    async fn synthesise_plain(
        &self,
        segment_id: u32,
        body: TtsRequest,
        cfg: &ElevenLabsConfig,
    ) -> Result<TtsResult, TtsError> {
        let url = format!("{BASE_URL}/text-to-speech/{}", cfg.voice_id);

        let resp = self
            .http
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Accept", "audio/mpeg")
            .json(&body)
            .send()
            .await?;

        let status = resp.status().as_u16();
        if status != 200 {
            let text = resp.text().await.unwrap_or_default();
            return Err(TtsError::Api { status, body: text });
        }

        let audio_bytes = resp.bytes().await?;
        let audio_path = self.write_audio(segment_id, &audio_bytes)?;
        let duration_secs = estimate_mp3_duration(&audio_bytes);

        debug!(segment_id, duration_secs, "TTS plain done");

        Ok(TtsResult {
            audio_path,
            duration_secs,
            word_timestamps: vec![],
        })
    }

    // ── With timestamps ──────────────────────────────────────────────────────

    async fn synthesise_with_timestamps(
        &self,
        segment_id: u32,
        body: TtsRequest,
        cfg: &ElevenLabsConfig,
    ) -> Result<TtsResult, TtsError> {
        let url = format!("{BASE_URL}/text-to-speech/{}/with-timestamps", cfg.voice_id);

        let resp = self
            .http
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Accept", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status().as_u16();
        if status != 200 {
            let text = resp.text().await.unwrap_or_default();
            return Err(TtsError::Api { status, body: text });
        }

        let resp_body: TimestampResponse = resp.json().await?;

        // ── Decode base64 audio ───────────────────────────────────────────────
        let audio_bytes = base64_decode(&resp_body.audio_base64)?;
        let audio_path = self.write_audio(segment_id, &audio_bytes)?;

        // ── Build word-level timestamps ───────────────────────────────────────
        let word_timestamps = build_word_timestamps(&resp_body.alignment);

        let duration_secs = word_timestamps
            .last()
            .map(|w| w.end_secs)
            .unwrap_or_else(|| estimate_mp3_duration(&audio_bytes));

        debug!(
            segment_id,
            duration_secs,
            words = word_timestamps.len(),
            "TTS with-timestamps done"
        );

        Ok(TtsResult {
            audio_path,
            duration_secs,
            word_timestamps,
        })
    }

    // ── Helpers ──────────────────────────────────────────────────────────────

    fn write_audio(&self, segment_id: u32, bytes: &[u8]) -> Result<PathBuf, TtsError> {
        std::fs::create_dir_all(&self.out_dir)?;
        let path = self.out_dir.join(format!("segment_{:04}.mp3", segment_id));
        std::fs::write(&path, bytes)?;
        Ok(path)
    }
}

// ── Wire types (ElevenLabs JSON schema) ─────────────────────────────────────

#[derive(Serialize)]
struct TtsRequest {
    text: String,
    model_id: String,
    voice_settings: VoiceSettings,
}

#[derive(Serialize)]
struct VoiceSettings {
    stability: f32,
    similarity_boost: f32,
    style: f32,
    use_speaker_boost: bool,
    speed: f32,
}

#[derive(Deserialize)]
struct TimestampResponse {
    audio_base64: String,
    alignment: Alignment,
}

#[derive(Deserialize)]
struct Alignment {
    characters: Vec<String>,
    character_start_times_seconds: Vec<f64>,
    character_end_times_seconds: Vec<f64>,
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Groups character-level timestamps into word-level timestamps.
/// ElevenLabs returns individual characters; we merge on whitespace boundaries.
fn build_word_timestamps(align: &Alignment) -> Vec<WordTimestamp> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut word_start = 0.0_f64;

    for (i, ch) in align.characters.iter().enumerate() {
        if ch == " " || ch == "\n" {
            if !current.is_empty() {
                let end = align
                    .character_end_times_seconds
                    .get(i.saturating_sub(1))
                    .copied()
                    .unwrap_or(word_start);
                words.push(WordTimestamp {
                    word: current.clone(),
                    start_secs: word_start,
                    end_secs: end,
                });
                current.clear();
            }
        } else {
            if current.is_empty() {
                word_start = align
                    .character_start_times_seconds
                    .get(i)
                    .copied()
                    .unwrap_or(0.0);
            }
            current.push_str(ch);
        }
    }

    // Flush last word
    if !current.is_empty() {
        let end = align
            .character_end_times_seconds
            .last()
            .copied()
            .unwrap_or(word_start);
        words.push(WordTimestamp {
            word: current,
            start_secs: word_start,
            end_secs: end,
        });
    }

    words
}

/// Rough MP3 duration estimate: assumes 128 kbps CBR.
/// The with-timestamps path derives duration from the alignment data instead.
fn estimate_mp3_duration(bytes: &[u8]) -> f64 {
    (bytes.len() as f64 * 8.0) / 128_000.0
}

fn base64_decode(s: &str) -> Result<Vec<u8>, TtsError> {
    use base64::Engine;
    let clean = s.replace(['\n', '\r', ' '], "");
    base64::engine::general_purpose::STANDARD
        .decode(&clean)
        .map_err(|e| {
            TtsError::Json(serde_json::from_str::<serde_json::Value>(&e.to_string()).unwrap_err())
        })
}
