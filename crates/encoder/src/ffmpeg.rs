use std::path::{Path, PathBuf};
use std::process::Stdio;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, ChildStdin, Command};
use tracing::{debug, info};

#[derive(Debug, Error)]
pub enum EncoderError {
    #[error("ffmpeg not found — install ffmpeg and ensure it is on PATH")]
    FfmpegNotFound,

    #[error("ffmpeg process error: {0}")]
    Process(#[from] std::io::Error),

    #[error("ffmpeg exited with status {status}: {stderr}")]
    FfmpegFailed { status: i32, stderr: String },

    #[error("Encoder already finalised")]
    AlreadyFinalised,
}

// ── Config ────────────────────────────────────────────────────────────────────

pub struct EncoderConfig {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub video_bitrate: String, // e.g. "8M"
    pub audio_bitrate: String, // e.g. "192k"
    pub output_path: PathBuf,
}

/// One audio clip placed at an absolute time in the final mix.
#[derive(Debug, Clone)]
pub struct AudioSpan {
    pub path: PathBuf,
    pub start_ms: u64,
}

// ── FfmpegEncoder ─────────────────────────────────────────────────────────────

/// Wraps an ffmpeg child process.
///
/// Usage:
/// ```ignore
/// let mut enc = FfmpegEncoder::start(config).await?;
/// for frame in frames {
///     enc.write_frame(&frame.0).await?;
/// }
/// enc.finalise(audio_inputs).await?;
/// ```
pub struct FfmpegEncoder {
    child: Child,
    stdin: ChildStdin,
    config: EncoderConfig,
    done: bool,
}

impl FfmpegEncoder {
    /// Spawn an ffmpeg process that reads raw RGB24 frames from stdin
    /// and writes a video-only intermediate file.  Audio is muxed in
    /// `finalise()` via a second ffmpeg pass.
    pub async fn start(config: EncoderConfig) -> Result<Self, EncoderError> {
        // Verify ffmpeg exists
        which_ffmpeg()?;

        let intermediate = intermediate_path(&config.output_path);

        // ffmpeg command:
        //   -f rawvideo -pix_fmt rgb24 -s WxH -r FPS -i pipe:0
        //   -c:v libx264 -b:v BITRATE -pix_fmt yuv420p
        //   -movflags +faststart intermediate.mp4
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-y", // overwrite
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgb24",
            "-s",
            &format!("{}x{}", config.width, config.height),
            "-r",
            &config.fps.to_string(),
            "-i",
            "pipe:0", // stdin
            "-c:v",
            "libx264",
            "-b:v",
            &config.video_bitrate,
            "-pix_fmt",
            "yuv420p",
            "-movflags",
            "+faststart",
            intermediate.to_str().unwrap(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|_| EncoderError::FfmpegNotFound)?;
        let stdin = child.stdin.take().expect("stdin was piped");

        info!(
            width = config.width,
            height = config.height,
            fps = config.fps,
            "ffmpeg encoder started"
        );

        Ok(Self {
            child,
            stdin,
            config,
            done: false,
        })
    }

    /// Write one raw RGB24 frame (width × height × 3 bytes) to ffmpeg stdin.
    pub async fn write_frame(&mut self, rgb: &[u8]) -> Result<(), EncoderError> {
        self.stdin.write_all(rgb).await?;
        Ok(())
    }

    /// Close the video pipe and mux the audio tracks in a second ffmpeg pass.
    ///
    /// `audio_paths` — list of per-segment MP3 paths in playback order.
    /// They are concatenated and muxed into the final output.
    pub async fn finalise(
        mut self,
        audio_spans: &[AudioSpan],
        total_duration_ms: u64,
    ) -> Result<PathBuf, EncoderError> {
        if self.done {
            return Err(EncoderError::AlreadyFinalised);
        }
        self.done = true;

        // Close stdin so ffmpeg knows the stream is over
        drop(self.stdin);

        // Wait for the video-only pass to finish
        let video_status = self.child.wait().await?;
        if !video_status.success() {
            return Err(EncoderError::FfmpegFailed {
                status: video_status.code().unwrap_or(-1),
                stderr: "(check ffmpeg output above)".into(),
            });
        }

        let intermediate = intermediate_path(&self.config.output_path);
        let output = &self.config.output_path;

        // ── Mux audio ─────────────────────────────────────────────────────────
        if audio_spans.is_empty() {
            // No audio — just rename the intermediate
            tokio::fs::rename(&intermediate, output).await?;
        } else {
            // Mix delayed segment audio over a silent bed so all visual gaps,
            // holds, and pauses remain aligned with the final video duration.
            mux_audio(
                &intermediate,
                audio_spans,
                total_duration_ms,
                output,
                &self.config.audio_bitrate,
            )
            .await?;
            let _ = tokio::fs::remove_file(&intermediate).await;
        }

        info!(output = %output.display(), "Encoding complete");
        Ok(output.clone())
    }
}

// ── Audio mux helpers ─────────────────────────────────────────────────────────

/// Video intermediate + delayed audio mix → final MP4.
async fn mux_audio(
    video: &Path,
    audio_spans: &[AudioSpan],
    total_duration_ms: u64,
    output: &Path,
    audio_bitrate: &str,
) -> Result<(), EncoderError> {
    debug!("Muxing audio into final output");

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y")
        .arg("-i")
        .arg(video)
        .arg("-f")
        .arg("lavfi")
        .arg("-t")
        .arg(format!("{:.3}", total_duration_ms as f64 / 1000.0))
        .arg("-i")
        .arg("anullsrc=channel_layout=stereo:sample_rate=48000");

    for span in audio_spans {
        cmd.arg("-i").arg(&span.path);
    }

    let filter = build_audio_mix_filter(audio_spans);
    cmd.arg("-filter_complex").arg(filter);
    cmd.args([
        "-map",
        "0:v",
        "-map",
        "[aout]",
        "-c:v",
        "copy",
        "-c:a",
        "aac",
        "-b:a",
        audio_bitrate,
        "-shortest",
        output.to_str().unwrap(),
    ])
    .stdout(Stdio::null())
    .stderr(Stdio::piped());

    let output = cmd.output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(EncoderError::FfmpegFailed {
            status: output.status.code().unwrap_or(-1),
            stderr: if stderr.is_empty() {
                "(audio mux failed — ffmpeg returned no stderr)".into()
            } else {
                stderr
            },
        });
    }

    Ok(())
}

fn build_audio_mix_filter(audio_spans: &[AudioSpan]) -> String {
    let mut parts = Vec::new();
    let mut mix_inputs = Vec::new();

    // Input 0 is the video stream. Input 1 is the silent bed.
    mix_inputs.push("[1:a]".to_string());

    for (idx, span) in audio_spans.iter().enumerate() {
        let input_idx = idx + 2;
        let label = format!("[a{idx}]");
        parts.push(format!(
            "[{input_idx}:a]adelay={}:all=1{}",
            span.start_ms, label
        ));
        mix_inputs.push(label);
    }

    parts.push(format!(
        "{}amix=inputs={}:duration=longest:dropout_transition=0[aout]",
        mix_inputs.join(""),
        mix_inputs.len()
    ));

    parts.join(";")
}

// ── Utilities ─────────────────────────────────────────────────────────────────

fn intermediate_path(output: &Path) -> PathBuf {
    output.with_extension("_video_only.mp4")
}

fn which_ffmpeg() -> Result<(), EncoderError> {
    std::process::Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| EncoderError::FfmpegNotFound)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_mix_filter_includes_silent_bed_and_delays() {
        let filter = build_audio_mix_filter(&[
            AudioSpan {
                path: PathBuf::from("a.mp3"),
                start_ms: 500,
            },
            AudioSpan {
                path: PathBuf::from("b.mp3"),
                start_ms: 2500,
            },
        ]);

        assert!(filter.contains("[1:a]"));
        assert!(filter.contains("[2:a]adelay=500:all=1[a0]"));
        assert!(filter.contains("[3:a]adelay=2500:all=1[a1]"));
        assert!(filter.contains("amix=inputs=3:duration=longest:dropout_transition=0[aout]"));
    }

    #[test]
    fn audio_mix_filter_handles_no_spans() {
        let filter = build_audio_mix_filter(&[]);
        assert_eq!(
            filter,
            "[1:a]amix=inputs=1:duration=longest:dropout_transition=0[aout]"
        );
    }
}
