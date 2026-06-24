//! High-level Python entrypoints for the video engine.

use std::path::{Path, PathBuf};

use anyhow::Context;
use encoder::{AudioSpan, EncoderConfig, FfmpegEncoder};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use renderer::{FrameState, RenderContext};
use timing::{SegmentAudio, build_timeline};
use tokio::runtime::Runtime;
use tts_client::TtsClient;
use video_builder::Video as VideoBuilder;
use video_schema::{LessonScript, SegmentDef};

use animations::evaluate_segment_animation;

/// Return the binding version.
#[pyfunction]
pub fn version() -> &'static str {
    "1.0.0"
}

/// Create a video from JSON using the Rust pipeline.
///
/// `json_text` is the full lesson JSON.
/// `audio_temp_dir` is where intermediate audio files are written.
/// `output_path` is the final video file path.
///
/// For now we gonna use JSON, Later we can expose more structured APIs to Python, but this is
/// a good start.
#[pyfunction]
pub fn create_video(
    json_text: String,
    audio_temp_dir: String,
    output_path: String,
) -> PyResult<String> {
    let script = parse_and_normalize(&json_text).map_err(pyerr)?;
    let output = PathBuf::from(output_path);
    let audio_dir = PathBuf::from(audio_temp_dir);

    let rt = Runtime::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let result = rt.block_on(async move {
        run_pipeline(&script, &audio_dir, output)
            .await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    })?;

    Ok(result.display().to_string())
}

fn parse_and_normalize(json_text: &str) -> anyhow::Result<LessonScript> {
    let script: LessonScript =
        serde_json::from_str(json_text).context("Failed to parse lesson JSON")?;
    Ok(VideoBuilder::from(script).build())
}

async fn run_pipeline(
    script: &LessonScript,
    audio_dir: &Path,
    output_path: PathBuf,
) -> anyhow::Result<PathBuf> {
    validate_script(script)?;

    tokio::fs::create_dir_all(audio_dir).await?;
    let tts_client =
        TtsClient::from_env(audio_dir.to_path_buf()).context("Failed to create TTS client")?;

    let tts_results = run_tts_pass(script, &tts_client).await?;
    let timeline = build_timeline(script, &tts_results);
    if timeline.is_empty() {
        anyhow::bail!("Timeline is empty — check that segments have text and are enabled");
    }

    run_render_pass(script, &timeline, output_path).await
}

async fn run_tts_pass(script: &LessonScript, tts: &TtsClient) -> anyhow::Result<Vec<SegmentAudio>> {
    let active: Vec<&SegmentDef> = script
        .segments
        .iter()
        .filter(|s| s.control.enabled && !s.control.skip)
        .filter(|s| s.text.is_some())
        .collect();

    let futures = active.iter().map(|seg| {
        let cfg =
            merge_elevenlabs_config(script.defaults.elevenlabs.as_ref(), seg.elevenlabs.as_ref());
        async move {
            let text = seg.text.as_ref().unwrap().content.as_str();
            let result = tts.synthesise(seg.id, text, &cfg).await?;
            Ok::<_, anyhow::Error>(SegmentAudio {
                segment_id: seg.id,
                result,
            })
        }
    });

    let mut results = Vec::new();
    for future in futures {
        results.push(future.await);
    }
    results.into_iter().collect()
}

async fn run_render_pass(
    script: &LessonScript,
    timeline: &[timing::TimedSegment<'_>],
    output: PathBuf,
) -> anyhow::Result<PathBuf> {
    let out_cfg = script.output.as_ref().cloned().unwrap_or_default();

    let enc_cfg = EncoderConfig {
        width: script.resolution[0],
        height: script.resolution[1],
        fps: script.fps,
        video_bitrate: out_cfg.bitrate.clone(),
        audio_bitrate: out_cfg.audio_bitrate.clone(),
        output_path: output.clone(),
    };

    let mut encoder = FfmpegEncoder::start(enc_cfg).await?;
    let mut ctx = RenderContext::new(script.resolution[0], script.resolution[1])?;
    let total_frames = timeline.last().map(|s| s.end_frame).unwrap_or(0);
    let mut current_segment_idx = 0;

    for frame_num in 0..total_frames {
        while current_segment_idx + 1 < timeline.len()
            && frame_num >= timeline[current_segment_idx + 1].start_frame
        {
            current_segment_idx += 1;
        }

        let seg = &timeline[current_segment_idx];
        let animation = evaluate_segment_animation(
            seg.def,
            seg.start_frame,
            frame_num,
            script.fps,
            seg.frame_count(),
        );
        let revealed_word_count = seg.revealed_words_at(frame_num);
        let background = seg.def.background.as_ref().unwrap_or(&script.background);

        let state = FrameState {
            background,
            text: seg.def.text.as_ref(),
            revealed_word_count,
            animation,
        };

        let frame = ctx.draw_frame(&state)?;
        encoder.write_frame(&frame.0).await?;
    }

    let audio_spans: Vec<AudioSpan> = timeline
        .iter()
        .filter(|s| !s.audio_path.as_os_str().is_empty())
        .map(|s| AudioSpan {
            path: s.audio_path.clone(),
            start_ms: ((s.start_frame as f64 / script.fps as f64) * 1000.0).round() as u64,
        })
        .collect();

    let total_duration_ms = ((total_frames as f64 / script.fps as f64) * 1000.0).round() as u64;
    encoder
        .finalise(&audio_spans, total_duration_ms)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to finalise video: {}", e))
}

fn validate_script(script: &LessonScript) -> anyhow::Result<()> {
    if script.segments.is_empty() {
        anyhow::bail!("Script has no segments");
    }

    let active_count = script
        .segments
        .iter()
        .filter(|s| s.control.enabled && !s.control.skip)
        .count();

    if active_count == 0 {
        anyhow::bail!("All segments are disabled or skipped");
    }

    for seg in &script.segments {
        if !seg.control.enabled {
            continue;
        }
        if seg.text.is_some() && seg.elevenlabs.is_none() && script.defaults.elevenlabs.is_none() {
            anyhow::bail!(
                "Segment {} has text but no ElevenLabs config (set in defaults or segment)",
                seg.id
            );
        }
    }

    Ok(())
}

fn merge_elevenlabs_config(
    defaults: Option<&video_schema::ElevenLabsConfig>,
    override_: Option<&video_schema::ElevenLabsConfig>,
) -> video_schema::ElevenLabsConfig {
    if let Some(ov) = override_ {
        return ov.clone();
    }
    if let Some(def) = defaults {
        return def.clone();
    }
    video_schema::ElevenLabsConfig {
        voice_id: "CONFIGURE_ME".into(),
        model: Default::default(),
        stability: 0.4,
        similarity_boost: 0.8,
        style: 0.5,
        use_speaker_boost: true,
        speed: 0.95,
        request_word_timestamps: true,
    }
}

fn pyerr(err: anyhow::Error) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}
