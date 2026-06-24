//! Segment timing and word reveal math.
//!
//! This crate translates the schema and TTS output into frame-accurate timing
//! data. The CLI can orchestrate around it, but the reveal math itself lives
//! here.

use tts_client::WordTimestamp;
use video_schema::{LessonScript, SegmentDef, TextReveal};

use tts_client::TtsResult;

/// A segment with full timing resolved.
#[derive(Debug)]
pub struct TimedSegment<'a> {
    pub def: &'a SegmentDef,
    /// Absolute frame index where this segment starts.
    pub start_frame: u64,
    /// Absolute frame index where this segment ends (exclusive).
    pub end_frame: u64,
    /// Path to the MP3 for this segment.
    pub audio_path: std::path::PathBuf,
    /// Per-word appear frame (relative to start_frame = 0).
    pub word_frames: Vec<WordReveal>,
}

/// When a specific word should become visible, in absolute frame numbers.
#[derive(Debug)]
pub struct WordReveal {
    pub word: String,
    pub frame: u64,
}

impl<'a> TimedSegment<'a> {
    /// Total number of frames for this segment.
    pub fn frame_count(&self) -> u64 {
        self.end_frame - self.start_frame
    }

    /// Progress 0.0-1.0 for a given absolute frame number.
    pub fn progress_at(&self, frame: u64) -> f32 {
        if self.frame_count() == 0 {
            return 1.0;
        }
        ((frame - self.start_frame) as f32 / self.frame_count() as f32).clamp(0.0, 1.0)
    }

    /// How many words should be revealed at a given absolute frame.
    pub fn revealed_words_at(&self, frame: u64) -> usize {
        self.word_frames.iter().filter(|w| w.frame <= frame).count()
    }
}

/// Build the full timeline from TTS results.
pub fn build_timeline<'a>(
    script: &'a LessonScript,
    tts_results: &[SegmentAudio],
) -> Vec<TimedSegment<'a>> {
    let fps = script.fps as f64;
    let mut cursor_frame: u64 = 0;
    let mut timed = Vec::new();

    for seg in &script.segments {
        if !seg.control.enabled || seg.control.skip {
            continue;
        }

        let tts = tts_results.iter().find(|r| r.segment_id == seg.id);

        let base_duration_secs: f64 = if let Some(ov) = seg.duration_override_seconds {
            ov as f64
        } else if let Some(t) = tts {
            t.result.duration_secs
        } else {
            2.0
        };

        let hold = seg.post_hold_seconds as f64;
        let effective_duration = (base_duration_secs / seg.control.speed_multiplier as f64) + hold;

        let delay_start = (seg.control.delay_before_ms as f64 / 1000.0 * fps) as u64;
        cursor_frame += delay_start;

        let start_frame = cursor_frame;
        let frame_count = (effective_duration * fps).round() as u64;
        let end_frame = start_frame + frame_count;

        let word_frames = if let Some(t) = tts {
            build_word_frames(
                &t.result.word_timestamps,
                start_frame,
                fps,
                seg.text
                    .as_ref()
                    .map(|tx| &tx.reveal)
                    .unwrap_or(&TextReveal::AllAtOnce),
            )
        } else {
            vec![]
        };

        let audio_path = tts.map(|t| t.result.audio_path.clone()).unwrap_or_default();

        timed.push(TimedSegment {
            def: seg,
            start_frame,
            end_frame,
            audio_path,
            word_frames,
        });

        let delay_end = (seg.control.delay_after_ms as f64 / 1000.0 * fps) as u64;
        cursor_frame = end_frame + delay_end;
    }

    timed
}

/// Convert word timestamps (seconds) to absolute frame numbers.
pub fn build_word_frames(
    timestamps: &[WordTimestamp],
    start_frame: u64,
    fps: f64,
    reveal: &TextReveal,
) -> Vec<WordReveal> {
    match reveal {
        TextReveal::AllAtOnce => timestamps
            .iter()
            .map(|w| WordReveal {
                word: w.word.clone(),
                frame: start_frame,
            })
            .collect(),
        TextReveal::WordByWord | TextReveal::Typewriter => timestamps
            .iter()
            .map(|w| WordReveal {
                word: w.word.clone(),
                frame: start_frame + (w.start_secs * fps).round() as u64,
            })
            .collect(),
        TextReveal::LineByLine => timestamps
            .iter()
            .map(|w| WordReveal {
                word: w.word.clone(),
                frame: start_frame + (w.start_secs * fps).round() as u64,
            })
            .collect(),
    }
}

/// TTS synthesis result attached to its segment.
#[derive(Debug)]
pub struct SegmentAudio {
    pub segment_id: u32,
    pub result: TtsResult,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tts_client::{TtsResult, WordTimestamp};
    use video_schema::{AudioTrack, BackgroundDef, OutputConfig, SegmentControl};

    fn segment(reveal: TextReveal) -> SegmentDef {
        SegmentDef {
            id: 1,
            kind: Default::default(),
            duration_override_seconds: None,
            text: Some(video_schema::TextDef {
                content: "hello world".into(),
                key_terms: vec![],
                reveal,
                reveal_sync: Default::default(),
                reveal_interval_ms: None,
                font: None,
                color: None,
                opacity: 1.0,
                align: Default::default(),
                position: None,
                max_width: None,
                padding: None,
                shadow: None,
                background: None,
            }),
            elevenlabs: None,
            background: None::<BackgroundDef>,
            animation_in: None,
            animation_out: None,
            post_hold_seconds: 0.0,
            mood: Default::default(),
            pacing: Default::default(),
            audio: None::<AudioTrack>,
            transition_to_next: None,
            control: SegmentControl::default(),
        }
    }

    fn tts_result() -> TtsResult {
        TtsResult {
            audio_path: std::path::PathBuf::from("a.mp3"),
            duration_secs: 2.0,
            word_timestamps: vec![
                WordTimestamp {
                    word: "hello".into(),
                    start_secs: 0.0,
                    end_secs: 0.5,
                },
                WordTimestamp {
                    word: "world".into(),
                    start_secs: 0.5,
                    end_secs: 1.0,
                },
            ],
        }
    }

    #[test]
    fn all_at_once_reveals_everything_on_start() {
        let frames = build_word_frames(
            &tts_result().word_timestamps,
            12,
            30.0,
            &TextReveal::AllAtOnce,
        );
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].frame, 12);
        assert_eq!(frames[1].frame, 12);
    }

    #[test]
    fn word_by_word_uses_timestamps() {
        let frames = build_word_frames(
            &tts_result().word_timestamps,
            10,
            30.0,
            &TextReveal::WordByWord,
        );
        assert_eq!(frames[0].frame, 10);
        assert_eq!(frames[1].frame, 25);
    }

    #[test]
    fn timeline_applies_hold_and_delays() {
        let script = LessonScript {
            version: "1.0".into(),
            fps: 30,
            resolution: [1920, 1080],
            aspect_ratio: Default::default(),
            output: Some(OutputConfig::default()),
            background: Default::default(),
            defaults: Default::default(),
            assets: Default::default(),
            segments: vec![segment(TextReveal::WordByWord)],
        };

        let seg_audio = SegmentAudio {
            segment_id: 1,
            result: tts_result(),
        };

        let timeline = build_timeline(&script, &[seg_audio]);
        assert_eq!(timeline.len(), 1);
        assert_eq!(timeline[0].start_frame, 0);
        assert_eq!(timeline[0].end_frame, 60);
        assert_eq!(timeline[0].revealed_words_at(0), 1);
    }
}
