use animations::evaluate_segment_animation;
use encoder::{AudioSpan, EncoderConfig, FfmpegEncoder};
use renderer::{FrameState, RenderContext};
use std::path::PathBuf;
use tracing::{debug, info};
use video_schema::LessonScript;

use timing::TimedSegment;

/// Render every frame in the timeline and encode to MP4.
///
/// Returns the path to the final video file.
/// Render the entire timeline as a sequence of frames and encode the result to an MP4 video file.
///
/// This function orchestrates the rendering and encoding process. It iterates through every frame
/// defined in the timeline, evaluates animation states, calculates segment properties, draws frames,
/// and encodes them using FFmpeg. Additionally, audio segments are collected and muxed with the video
/// during the finalization stage.
///
/// # Arguments
///
/// * `script` - A reference to the [`LessonScript`] struct containing global rendering configurations,
///   such as resolution, frames per second, and output settings.
/// * `timeline` - A slice of [`TimedSegment`] structs representing the timeline of segments to be
///   rendered. Each segment contains definitions for animations, text, and other frame-specific details.
/// * `output` - A [`PathBuf`] specifying the output path where the final MP4 video file will be saved.
///
/// # Returns
///
/// * `anyhow::Result<PathBuf>` - Returns the file path of the successfully generated video as a
///   [`PathBuf`] if the operation completed without errors. Otherwise, it returns an error wrapped
///   in an [`anyhow::Result`].
///
/// # Errors
///
/// * This function will return an error if the encoder fails to initialize, if frame rendering or
///   encoding encounters an issue, or if the audio muxing process fails.
///
/// # Examples
///
/// ```
/// let script = LessonScript::default();
/// let timeline = vec![/* TimedSegments */];
/// let output_path = PathBuf::from("output/video.mp4");
/// let result = run_render_pass(&script, &timeline, output_path).await;
/// ```
pub async fn run_render_pass(
    script: &LessonScript,
    timeline: &[TimedSegment<'_>],
    output: PathBuf,
) -> anyhow::Result<PathBuf> {
    // ── Encoder setup ─────────────────────────────────────────────────────────
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

    // ── Renderer setup ────────────────────────────────────────────────────────
    let mut ctx = RenderContext::new(script.resolution[0], script.resolution[1])?;

    // ── Total frame count ─────────────────────────────────────────────────────
    let total_frames = timeline.last().map(|s| s.end_frame).unwrap_or(0);

    info!(total_frames, fps = script.fps, "Starting render pass");

    // ── Frame loop ────────────────────────────────────────────────────────────
    let mut current_segment_idx = 0;

    for frame_num in 0..total_frames {
        // Advance segment pointer
        while current_segment_idx + 1 < timeline.len()
            && frame_num >= timeline[current_segment_idx + 1].start_frame
        {
            current_segment_idx += 1;
        }

        let seg = &timeline[current_segment_idx];

        // ── Compute animation state ───────────────────────────────────────────
        let animation = evaluate_segment_animation(
            seg.def,
            seg.start_frame,
            frame_num,
            script.fps,
            seg.frame_count(),
        );

        // ── Compute revealed word count ───────────────────────────────────────
        let revealed_word_count = seg.revealed_words_at(frame_num);

        // ── Background: prefer segment override, fall back to global ──────────
        let background = seg.def.background.as_ref().unwrap_or(&script.background);

        // ── Build frame state ─────────────────────────────────────────────────
        let state = FrameState {
            background,
            text: seg.def.text.as_ref(),
            revealed_word_count,
            animation,
        };

        // ── Draw ──────────────────────────────────────────────────────────────
        let frame = ctx.draw_frame(&state)?;
        encoder.write_frame(&frame.0).await?;

        if frame_num % (script.fps as u64 * 5) == 0 {
            let secs = frame_num / script.fps as u64;
            debug!(frame = frame_num, time_secs = secs, "Rendering...");
        }
    }

    info!("All frames rendered — finalising audio mux");

    // ── Collect audio paths in segment order ──────────────────────────────────
    let audio_spans: Vec<AudioSpan> = timeline
        .iter()
        .filter(|s| !s.audio_path.as_os_str().is_empty())
        .map(|s| AudioSpan {
            path: s.audio_path.clone(),
            start_ms: ((s.start_frame as f64 / script.fps as f64) * 1000.0).round() as u64,
        })
        .collect();

    let total_duration_ms = ((total_frames as f64 / script.fps as f64) * 1000.0).round() as u64;

    // ── Finalise (mux audio, close ffmpeg) ───────────────────────────────────
    let final_path = encoder.finalise(&audio_spans, total_duration_ms).await?;
    Ok(final_path)
}
