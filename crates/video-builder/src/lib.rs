//! Fluent builder for composing `video_schema::LessonScript` values.
//!
//! The runtime pipeline still operates on schema structs. This crate exists to
//! make script construction more expressive without changing that core model.

use serde::{Deserialize, Serialize};
use video_schema::{
    AnimationDef, AnimationType, AspectRatio, AudioAsset, AudioTrack, BackgroundDef, FitMode,
    LessonScript, OutputConfig, SegmentControl, SegmentDef, SegmentDefaults, SegmentKind, TextDef,
};

/// Fluent builder for a complete video script.
#[derive(Debug, Clone)]
pub struct Video {
    script: LessonScript,
    next_segment_id: u32,
}

impl Video {
    /// Start a new script with the schema defaults.
    pub fn new() -> Self {
        Self {
            script: LessonScript {
                version: "1.0".to_string(),
                fps: 30,
                resolution: [1920, 1080],
                aspect_ratio: AspectRatio::default(),
                output: None,
                background: BackgroundDef::default(),
                defaults: SegmentDefaults::default(),
                assets: Default::default(),
                segments: Vec::new(),
            },
            next_segment_id: 1,
        }
    }

    /// Override the default background for the whole video.
    pub fn background(mut self, background: BackgroundDef) -> Self {
        self.script.background = background;
        self
    }

    /// Override the output config for the whole video.
    pub fn output(mut self, output: OutputConfig) -> Self {
        self.script.output = Some(output);
        self
    }

    /// Override the frame rate.
    pub fn fps(mut self, fps: u32) -> Self {
        self.script.fps = fps;
        self
    }

    /// Override the resolution.
    pub fn resolution(mut self, width: u32, height: u32) -> Self {
        self.script.resolution = [width, height];
        self
    }

    /// Override the aspect ratio.
    pub fn aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.script.aspect_ratio = aspect_ratio;
        self
    }

    /// Register an image asset for reuse.
    pub fn asset_image<S1, S2>(mut self, key: S1, src: S2, preload: bool) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.script.assets.images.insert(
            key.into(),
            video_schema::ImageAsset {
                src: src.into(),
                preload,
            },
        );
        self
    }

    /// Register a font asset for reuse.
    pub fn asset_font<S1, S2>(mut self, key: S1, src: S2, family: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.script.assets.fonts.insert(
            key.into(),
            video_schema::FontAsset {
                src: src.into(),
                family: family.into(),
            },
        );
        self
    }

    /// Register an audio asset for reuse.
    pub fn asset_audio<S1, S2>(mut self, key: S1, src: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.script
            .assets
            .audio
            .insert(key.into(), AudioAsset { src: src.into() });
        self
    }

    /// Add a new segment with primary content.
    pub fn add<C>(self, content: C) -> SegmentDraft
    where
        C: Into<SegmentContent>,
    {
        let segment = build_segment(self.next_segment_id, content.into());
        SegmentDraft {
            video: self,
            segment,
        }
    }

    /// Append a fully-specified segment without going through the fluent helpers.
    ///
    /// This is the escape hatch for schema features that do not yet have a
    /// dedicated builder method.
    pub fn segment(mut self, segment: SegmentDef) -> Self {
        self.script.segments.push(segment);
        self.next_segment_id += 1;
        self
    }

    /// Finish building and return the schema object.
    pub fn build(self) -> LessonScript {
        self.script
    }

    fn push_segment(mut self, segment: SegmentDef) -> Self {
        self.script.segments.push(segment);
        self.next_segment_id += 1;
        self
    }
}

impl Default for Video {
    fn default() -> Self {
        Self::new()
    }
}

impl From<LessonScript> for Video {
    fn from(script: LessonScript) -> Self {
        let next_segment_id = script.segments.iter().map(|seg| seg.id).max().unwrap_or(0) + 1;
        Self {
            script,
            next_segment_id,
        }
    }
}

impl From<Video> for LessonScript {
    fn from(video: Video) -> Self {
        video.script
    }
}

/// A pending segment awaiting a modifier like audio or animation.
#[derive(Debug, Clone)]
pub struct SegmentDraft {
    video: Video,
    segment: SegmentDef,
}

impl SegmentDraft {
    /// Attach a modifier and commit the segment back into the video.
    pub fn with<M>(self, modifier: M) -> Video
    where
        M: SegmentModifier,
    {
        let mut segment = self.segment;
        modifier.apply(&mut segment);
        self.video.push_segment(segment)
    }

    /// Commit the segment without any extra modifiers.
    pub fn done(self) -> Video {
        self.video.push_segment(self.segment)
    }
}

/// Primary content for a segment.
#[derive(Debug, Clone)]
pub enum SegmentContent {
    Text(TextDef),
    Image(ImageContent),
}

impl From<TextDef> for SegmentContent {
    fn from(value: TextDef) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for SegmentContent {
    fn from(value: &str) -> Self {
        Self::Text(TextDef {
            content: value.to_string(),
            ..default_text()
        })
    }
}

impl From<String> for SegmentContent {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<ImageContent> for SegmentContent {
    fn from(value: ImageContent) -> Self {
        Self::Image(value)
    }
}

/// Fluent image content helper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    pub src: String,
    #[serde(default)]
    pub fit: FitMode,
    pub overlay: Option<video_schema::OverlayDef>,
}

impl ImageContent {
    pub fn new<S: Into<String>>(src: S) -> Self {
        Self {
            src: src.into(),
            fit: FitMode::default(),
            overlay: None,
        }
    }

    pub fn fit(mut self, fit: FitMode) -> Self {
        self.fit = fit;
        self
    }

    pub fn overlay(mut self, overlay: video_schema::OverlayDef) -> Self {
        self.overlay = Some(overlay);
        self
    }
}

/// Modifier applied to a pending segment.
pub trait SegmentModifier {
    fn apply(self, segment: &mut SegmentDef);
}

impl SegmentModifier for AudioTrack {
    fn apply(self, segment: &mut SegmentDef) {
        segment.audio = Some(self);
    }
}

impl SegmentModifier for AnimationDirective {
    fn apply(self, segment: &mut SegmentDef) {
        match self.placement {
            AnimationPlacement::In => segment.animation_in = Some(self.def),
            AnimationPlacement::Out => segment.animation_out = Some(self.def),
        }
    }
}

/// The side of the segment an animation applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPlacement {
    In,
    Out,
}

/// Animation helper with an explicit placement.
#[derive(Debug, Clone)]
pub struct AnimationDirective {
    def: AnimationDef,
    placement: AnimationPlacement,
}

impl AnimationDirective {
    pub fn new(def: AnimationDef) -> Self {
        Self {
            def,
            placement: AnimationPlacement::In,
        }
    }

    pub fn out(def: AnimationDef) -> Self {
        Self {
            def,
            placement: AnimationPlacement::Out,
        }
    }
}

/// Create a default text animation definition.
pub fn animation(kind: AnimationType) -> AnimationDirective {
    AnimationDirective::new(AnimationDef {
        kind,
        duration_ms: 300,
        easing: video_schema::Easing::default(),
        delay_ms: 0,
    })
}

/// Create a text segment helper from raw content.
pub fn text<S: Into<String>>(content: S) -> TextDef {
    TextDef {
        content: content.into(),
        ..default_text()
    }
}

/// Create an image segment helper.
pub fn image<S: Into<String>>(src: S) -> ImageContent {
    ImageContent::new(src)
}

/// Create a music track helper.
pub fn music<S: Into<String>>(src: S) -> AudioTrack {
    AudioTrack {
        background_music: Some(video_schema::BackgroundMusic {
            src: src.into(),
            volume: 1.0,
            fade_in_ms: 0,
            fade_out_ms: 0,
            loop_audio: false,
            start_at: 0.0,
        }),
        sound_effect: None,
    }
}

/// Create a sound-effect track helper.
pub fn audio<S: Into<String>>(src: S) -> AudioTrack {
    AudioTrack {
        background_music: None,
        sound_effect: Some(video_schema::SoundEffect {
            src: src.into(),
            volume: 1.0,
            trigger_at: Default::default(),
        }),
    }
}

fn build_segment(id: u32, content: SegmentContent) -> SegmentDef {
    let mut segment = SegmentDef {
        id,
        kind: SegmentKind::Speech,
        duration_override_seconds: None,
        text: None,
        elevenlabs: None,
        background: None,
        animation_in: None,
        animation_out: None,
        post_hold_seconds: 0.0,
        mood: Default::default(),
        pacing: Default::default(),
        audio: None,
        transition_to_next: None,
        control: SegmentControl::default(),
    };

    match content {
        SegmentContent::Text(def) => {
            segment.text = Some(def);
        }
        SegmentContent::Image(def) => {
            segment.background = Some(BackgroundDef::Image {
                src: def.src,
                fit: def.fit,
                overlay: def.overlay,
            });
        }
    }

    segment
}

fn default_text() -> TextDef {
    TextDef {
        content: String::new(),
        key_terms: Vec::new(),
        reveal: Default::default(),
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use video_schema::{AnimationType, BackgroundDef, Easing, FitMode, SegmentKind};

    #[test]
    fn builder_starts_with_schema_defaults() {
        let video = Video::new();
        let script = video.build();

        assert_eq!(script.version, "1.0");
        assert_eq!(script.fps, 30);
        assert_eq!(script.resolution, [1920, 1080]);
        assert!(script.segments.is_empty());
    }

    #[test]
    fn adding_text_commits_a_segment() {
        let script = Video::new().add(text("hello")).done().build();

        assert_eq!(script.segments.len(), 1);
        let seg = &script.segments[0];
        assert_eq!(seg.id, 1);
        assert_eq!(
            std::mem::discriminant(&seg.kind),
            std::mem::discriminant(&SegmentKind::Speech)
        );
        assert_eq!(seg.text.as_ref().map(|t| t.content.as_str()), Some("hello"));
    }

    #[test]
    fn adding_text_and_audio_is_chainable() {
        let script = Video::new()
            .add(text("hello"))
            .with(audio("voice.wav"))
            .build();

        let seg = &script.segments[0];
        assert!(seg.audio.is_some());
        assert!(seg.background.is_none());
        assert_eq!(seg.text.as_ref().unwrap().content, "hello");
    }

    #[test]
    fn adding_text_and_music_uses_background_music() {
        let script = Video::new().add("hello").with(music("bed.mp3")).build();

        let seg = &script.segments[0];
        assert!(seg.audio.as_ref().unwrap().background_music.is_some());
        assert!(seg.audio.as_ref().unwrap().sound_effect.is_none());
    }

    #[test]
    fn animation_defaults_to_entering_motion() {
        let script = Video::new()
            .add("hello")
            .with(animation(AnimationType::Fade))
            .build();

        let seg = &script.segments[0];
        assert_eq!(
            std::mem::discriminant(&seg.animation_in.as_ref().unwrap().kind),
            std::mem::discriminant(&AnimationType::Fade)
        );
        assert!(seg.animation_out.is_none());
    }

    #[test]
    fn explicit_exit_animation_targets_animation_out() {
        let script = Video::new()
            .add("hello")
            .with(AnimationDirective::out(AnimationDef {
                kind: AnimationType::Fade,
                duration_ms: 250,
                easing: Easing::EaseOut,
                delay_ms: 0,
            }))
            .build();

        let seg = &script.segments[0];
        assert!(seg.animation_in.is_none());
        assert_eq!(seg.animation_out.as_ref().unwrap().duration_ms, 250);
    }

    #[test]
    fn image_becomes_background_image_segment() {
        let script = Video::new()
            .add(image("diagram.png").fit(FitMode::Contain))
            .done()
            .build();

        let seg = &script.segments[0];
        match seg.background.as_ref().unwrap() {
            BackgroundDef::Image { src, fit, overlay } => {
                assert_eq!(src, "diagram.png");
                assert_eq!(
                    std::mem::discriminant(fit),
                    std::mem::discriminant(&FitMode::Contain)
                );
                assert!(overlay.is_none());
            }
            other => panic!("unexpected background: {other:?}"),
        }
    }

    #[test]
    fn image_overlay_is_preserved() {
        let script = Video::new()
            .add(image("cover.png").overlay(video_schema::OverlayDef {
                color: "#000000".to_string(),
                opacity: 0.4,
            }))
            .done()
            .build();

        let seg = &script.segments[0];
        match seg.background.as_ref().unwrap() {
            BackgroundDef::Image { overlay, .. } => {
                assert_eq!(overlay.as_ref().unwrap().opacity, 0.4);
            }
            _ => panic!("expected image background"),
        }
    }

    #[test]
    fn multiple_segments_keep_order_and_ids() {
        let script = Video::new()
            .add("first")
            .with(audio("a.wav"))
            .add("second")
            .with(audio("b.wav"))
            .add(image("third.png"))
            .done()
            .build();

        assert_eq!(script.segments.len(), 3);
        assert_eq!(script.segments[0].id, 1);
        assert_eq!(script.segments[1].id, 2);
        assert_eq!(script.segments[2].id, 3);
        assert_eq!(script.segments[0].text.as_ref().unwrap().content, "first");
        assert_eq!(script.segments[1].text.as_ref().unwrap().content, "second");
        assert!(matches!(
            script.segments[2].background,
            Some(BackgroundDef::Image { .. })
        ));
    }

    #[test]
    fn raw_segment_escape_hatch_preserves_full_schema_data() {
        let segment = SegmentDef {
            id: 42,
            kind: SegmentKind::Outro,
            duration_override_seconds: Some(3.5),
            text: Some(TextDef {
                content: "raw".to_string(),
                ..default_text()
            }),
            elevenlabs: None,
            background: Some(BackgroundDef::Solid {
                color: "#222222".to_string(),
            }),
            animation_in: None,
            animation_out: None,
            post_hold_seconds: 1.0,
            mood: Default::default(),
            pacing: Default::default(),
            audio: Some(audio("fx.wav")),
            transition_to_next: None,
            control: Default::default(),
        };

        let script = Video::new().segment(segment).build();
        assert_eq!(script.segments.len(), 1);
        assert_eq!(script.segments[0].id, 42);
        assert!(matches!(script.segments[0].kind, SegmentKind::Outro));
        assert_eq!(script.segments[0].post_hold_seconds, 1.0);
    }

    #[test]
    fn can_customize_global_video_settings() {
        let script = Video::new()
            .fps(60)
            .resolution(1280, 720)
            .aspect_ratio(video_schema::AspectRatio::Vertical)
            .background(BackgroundDef::Solid {
                color: "#101010".to_string(),
            })
            .output(OutputConfig {
                filename: "out".to_string(),
                format: video_schema::VideoFormat::Mp4,
                codec: video_schema::VideoCodec::H264,
                bitrate: "4M".to_string(),
                audio_codec: video_schema::AudioCodecKind::Aac,
                audio_bitrate: "128k".to_string(),
            })
            .build();

        assert_eq!(script.fps, 60);
        assert_eq!(script.resolution, [1280, 720]);
        assert!(matches!(script.background, BackgroundDef::Solid { .. }));
        assert_eq!(script.output.unwrap().filename, "out");
    }
}
