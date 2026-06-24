use serde::{Deserialize, Serialize};

/// Per-segment audio tracks (background music, sound effects).
/// TTS audio is handled separately by the TTS pass.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    pub background_music: Option<BackgroundMusic>,
    pub sound_effect: Option<SoundEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundMusic {
    /// Asset key or file path.
    pub src: String,

    #[serde(default = "default_volume")]
    pub volume: f32,

    #[serde(default)]
    pub fade_in_ms: u32,

    #[serde(default)]
    pub fade_out_ms: u32,

    #[serde(default)]
    pub loop_audio: bool,

    /// Start offset within the audio file, in seconds.
    #[serde(default)]
    pub start_at: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffect {
    pub src: String,

    #[serde(default = "default_volume")]
    pub volume: f32,

    /// When to play the effect within the segment.
    #[serde(default)]
    pub trigger_at: SoundTrigger,
}

fn default_volume() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SoundTrigger {
    /// Named trigger point.
    Named(SoundTriggerNamed),
    /// Exact second within the segment.
    Seconds(f32),
}

impl Default for SoundTrigger {
    fn default() -> Self {
        SoundTrigger::Named(SoundTriggerNamed::Start)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SoundTriggerNamed {
    #[default]
    Start,
    End,
}
