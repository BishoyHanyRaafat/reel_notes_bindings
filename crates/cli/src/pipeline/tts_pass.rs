use timing::SegmentAudio;
use tracing::info;
use tts_client::TtsClient;
use video_schema::{ElevenLabsConfig, LessonScript, SegmentDef};

/// Run TTS for every active segment that has text, concurrently.
/// Returns results in the same order as `script.segments`.
pub async fn run_tts_pass(
    script: &LessonScript,
    tts: &TtsClient,
) -> anyhow::Result<Vec<SegmentAudio>> {
    let active: Vec<&SegmentDef> = script
        .segments
        .iter()
        .filter(|s| s.control.enabled && !s.control.skip)
        .filter(|s| s.text.is_some())
        .collect();

    info!(count = active.len(), "Starting TTS pass");

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

    // TODO: In Production, we will want to run these concurrently, but we need to be careful about
    // overloading the TTS service. For now, let's run them sequentially.
    // Replace it with
    // let results: Vec<anyhow::Result<SegmentAudio>> = futures::future::join_all(futures).await;
    let results: Vec<anyhow::Result<SegmentAudio>> = join_onebyone(futures).await;

    // Collect, surfacing the first error if any
    results.into_iter().collect()
}

pub async fn join_onebyone<I, F>(futures: I) -> Vec<F::Output>
where
    I: IntoIterator<Item = F>,
    F: Future,
{
    let mut results = Vec::new();

    for future in futures {
        results.push(future.await);
    }

    results
}

/// Merge default ElevenLabs config with a per-segment override.
/// Per-segment fields take precedence; defaults fill in missing fields.
fn merge_elevenlabs_config(
    defaults: Option<&ElevenLabsConfig>,
    override_: Option<&ElevenLabsConfig>,
) -> ElevenLabsConfig {
    // If an override exists, use it wholesale (simple strategy for v1).
    // Future: field-level merge.
    if let Some(ov) = override_ {
        return ov.clone();
    }
    if let Some(def) = defaults {
        return def.clone();
    }
    // Fallback — no voice configured; pipeline will error at TTS call.
    ElevenLabsConfig {
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
