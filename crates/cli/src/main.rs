//! Command-line entrypoint for the video engine.
//!
//! The CLI loads lesson scripts, runs TTS and timing, renders frames, and
//! finalizes the encoded output video.

mod error;
mod pipeline;

use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::EnvFilter;

use pipeline::{render_pass::run_render_pass, tts_pass::run_tts_pass};
use timing::build_timeline;
use tts_client::TtsClient;
use video_builder::Video as VideoBuilder;
use video_language::describe_script;

/// Video engine CLI
#[derive(Parser, Debug)]
#[command(
    name = "video-engine",
    version,
    about = "AI educational video generator"
)]
struct Args {
    /// Path to the lesson JSON file
    #[arg(short, long)]
    input: PathBuf,

    /// Output MP4 path (overrides filename in JSON)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Directory for intermediate files (audio, concat lists)
    #[arg(long, default_value = "./tmp")]
    work_dir: PathBuf,

    /// Skip TTS — reuse audio files already in work_dir (useful for fast re-renders)
    #[arg(long)]
    skip_tts: bool,

    /// Print a human-readable description of the script and exit.
    #[arg(long)]
    describe: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ── Logging ───────────────────────────────────────────────────────────────
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("video_engine=debug".parse()?)
                .add_directive("tts_client=debug".parse()?)
                .add_directive("renderer=debug".parse()?)
                .add_directive("encoder=debug".parse()?)
                .add_directive("cli=debug".parse()?),
        )
        .init();

    let args = Args::parse();

    // ── Load + parse JSON ─────────────────────────────────────────────────────
    info!(path = %args.input.display(), "Loading lesson script");
    let json = tokio::fs::read_to_string(&args.input)
        .await
        .with_context(|| format!("Cannot read {}", args.input.display()))?;

    let script: video_schema::LessonScript =
        serde_json::from_str(&json).context("Failed to parse lesson JSON")?;

    let script = VideoBuilder::from(script).build();

    validate_script(&script)?;

    if args.describe {
        print!("{}", describe_script(&script));
        return Ok(());
    }

    // ── Determine output path ─────────────────────────────────────────────────
    let output_path = args.output.unwrap_or_else(|| {
        let name = script
            .output
            .as_ref()
            .map(|o| o.filename.as_str())
            .unwrap_or("output");
        PathBuf::from(format!("{}.mp4", name))
    });

    // ── TTS pass ──────────────────────────────────────────────────────────────
    let audio_dir = args.work_dir.join("audio");
    tokio::fs::create_dir_all(&audio_dir).await?;

    let tts_client = TtsClient::from_env(audio_dir).context("Failed to create TTS client")?;

    let tts_results = if args.skip_tts {
        info!("--skip-tts set; skipping TTS pass");
        vec![]
    } else {
        run_tts_pass(&script, &tts_client).await?
    };

    // ── Timeline build ────────────────────────────────────────────────────────
    info!("Building timeline");
    let timeline = build_timeline(&script, &tts_results);

    if timeline.is_empty() {
        anyhow::bail!("Timeline is empty — check that segments have text and are enabled");
    }

    let total_secs: f64 = timeline
        .last()
        .map(|s| s.end_frame as f64 / script.fps as f64)
        .unwrap_or(0.0);

    info!(
        segments = timeline.len(),
        total_secs = format!("{:.1}", total_secs),
        "Timeline ready"
    );

    // ── Render + encode ───────────────────────────────────────────────────────
    let final_path = run_render_pass(&script, &timeline, output_path).await?;

    info!(output = %final_path.display(), "Done ✓");
    Ok(())
}

/// Basic sanity checks on the script before the pipeline runs.
fn validate_script(script: &video_schema::LessonScript) -> anyhow::Result<()> {
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

    // Ensure every speech segment has an ElevenLabs voice configured
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
