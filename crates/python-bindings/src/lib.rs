//! Python bindings for the video engine data model.
//!
//! This crate exposes the schema types to Python so scripts can be authored or
//! transformed outside Rust while still matching the engine's data model.

use pyo3::prelude::*;

mod core;
mod types;
mod utils;

#[pymodule]
pub mod video_engine {
    #[pymodule_export]
    pub use crate::core::{create_video, version};
    // #[pymodule_export]
    // pub use crate::types::video_input::animation::{
    //     AnimationIn, AnimationInType, AnimationOut, AnimationOutType, AnimationTarget,
    //     BodyAnimation, BodyAnimationType, RepeatMode, SegmentAnimation, Stagger, StaggerOrder,
    //     Transition, TransitionType,
    // };
    // #[pymodule_export]
    // pub use crate::types::video_input::audio::{AudioBlock, BackgroundMusic, SoundEffect};
    // #[pymodule_export]
    // pub use crate::types::video_input::background::{
    //     Background, BackgroundFit, BackgroundOverlay, GradientBackground, GradientDef,
    //     GradientDirection, GradientStop, ImageBackground, SolidBackground, VideoBackground,
    // };
    // #[pymodule_export]
    // pub use crate::types::video_input::code::{
    //     AnnotationPosition, AppearAt, CodeAnnotation, CodeBlock, CodeFont, CodeLanguage, CodePart,
    //     CodeRevealMode, CodeRevealSync, CodeSize, CodeTheme,
    // };
    // #[pymodule_export]
    // pub use crate::types::video_input::common::{
    //     AspectRatio, AxisPosition, Border, BorderStyle, Color, Easing, FillBackground,
    //     NamedPosition, Opacity, Padding, PartAppearAt, Position, Resolution, Shadow, Size,
    //     SizeValue,
    // };
    // #[pymodule_export]
    // pub use crate::types::video_input::elevenlabs::{
    //     ElevenLabsConfig, ElevenLabsModel, TtsOutput, WordTimestamp,
    // };
    // #[pymodule_export]
    // pub use crate::types::video_input::image::{
    //     ColorPalette, ImageBlock, ImageMood, ImageStyle, Placeholder, PlaceholderIcon,
    // };
    //
    // #[pymodule_export]
    // pub use crate::types::video_input::layout::{Layout, LayoutMode, SafeZone};
    // #[pymodule_export]
    // pub use crate::types::video_input::math::{
    //     MathBlock, MathRenderMode, MathRevealAt, MathRevealStyle,
    // };
    // #[pymodule_export]
    // pub use crate::types::video_input::parts::{PreviousPartHighlight, SegmentPart};
    // #[pymodule_export]
    // pub use crate::types::video_input::root::{
    //     AssetRegistry, AudioAsset, AudioCodec, FontAsset, ImageAsset, OutputConfig, OutputFormat,
    //     OutputProfile, SegmentDefaults, VideoCodec, VideoInput,
    // };
    //
    // #[pymodule_export]
    // pub use crate::types::video_input::segment::{
    //     LoopCount, Mood, Pacing, Segment, SegmentControl, SegmentLoop, SegmentType,
    // };
    //
    // #[pymodule_export]
    // pub use crate::types::video_input::shape::{
    //     ShapeAnimation, ShapeAnimationType, ShapeBlock, ShapeFill, ShapeStroke, ShapeType,
    // };
    //
    // #[pymodule_export]
    // pub use crate::types::video_input::table::{
    //     BetterIs, CaptionPosition, CellHighlight, CellValue, ColHighlight, ColumnType,
    //     ComparisonConfig, RowHighlight, TableBlock, TableCaption, TableColors, TableFont,
    //     TableRevealMode, TableStyle, TotalsRow,
    // };
    //
    // #[pymodule_export]
    // pub use crate::types::video_input::text::{
    //     FontConfig, FontStyle, FontWeight, KeyTerm, RevealSync, TextAlign, TextBlock,
    //     TextRevealMode,
    // };
}
