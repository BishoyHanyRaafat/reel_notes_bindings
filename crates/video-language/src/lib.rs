//! Human-readable descriptions of video scripts.
//!
//! This crate turns schema objects into plain English so the CLI can explain
//! what a script contains without rendering it.

use std::fmt::Write;

use video_schema::{
    AnimationDef, BackgroundDef, LessonScript, SegmentDef, SegmentKind, TextDef, TransitionKind,
};

/// Build a human-readable description of a full script.
pub fn describe_script(script: &LessonScript) -> String {
    let mut out = String::new();

    let _ = writeln!(
        out,
        "Video script: {}x{} at {} fps with {} segments.",
        script.resolution[0],
        script.resolution[1],
        script.fps,
        script.segments.len()
    );

    let _ = writeln!(
        out,
        "Default background: {}.",
        describe_background(&script.background)
    );

    if let Some(output) = &script.output {
        let _ = writeln!(out, "Output target: {}.", output.filename);
    }

    if !script.assets.images.is_empty()
        || !script.assets.audio.is_empty()
        || !script.assets.fonts.is_empty()
    {
        let _ = writeln!(
            out,
            "Assets declared: {} images, {} fonts, {} audio tracks.",
            script.assets.images.len(),
            script.assets.fonts.len(),
            script.assets.audio.len()
        );
    }

    for seg in &script.segments {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "Segment {}: {}",
            seg.id,
            describe_segment_kind(&seg.kind)
        );
        let _ = writeln!(out, "{}", describe_segment(seg));
    }

    out
}

/// Describe a single segment in plain language.
pub fn describe_segment(seg: &SegmentDef) -> String {
    let mut parts = Vec::new();

    if let Some(text) = &seg.text {
        parts.push(describe_text(text));
    } else {
        parts.push("No narration text.".to_string());
    }

    if let Some(bg) = &seg.background {
        parts.push(format!("Background: {}.", describe_background(bg)));
    }

    if let Some(anim_in) = &seg.animation_in {
        parts.push(format!("Enters with {}.", describe_animation(anim_in)));
    }

    if let Some(anim_out) = &seg.animation_out {
        parts.push(format!("Exits with {}.", describe_animation(anim_out)));
    }

    if let Some(transition) = &seg.transition_to_next {
        parts.push(format!(
            "Transitions to the next segment using {} over {} ms.",
            describe_transition_kind(&transition.kind),
            transition.duration_ms
        ));
    }

    if seg.post_hold_seconds > 0.0 {
        parts.push(format!(
            "Holds the final frame for {:.1} seconds.",
            seg.post_hold_seconds
        ));
    }

    if seg.audio.is_some() {
        parts.push("Includes additional audio tracks.".to_string());
    }

    parts.join(" ")
}

fn describe_segment_kind(kind: &SegmentKind) -> &'static str {
    match kind {
        SegmentKind::Speech => "spoken narration",
        SegmentKind::Pause => "pause",
        SegmentKind::Transition => "transition",
        SegmentKind::Title => "title card",
        SegmentKind::Chapter => "chapter marker",
        SegmentKind::Outro => "outro",
    }
}

fn describe_text(text: &TextDef) -> String {
    let mut out = String::new();
    let _ = write!(out, "Text: {:?}.", text.content);
    if !text.key_terms.is_empty() {
        let _ = write!(out, " Highlights {} key terms.", text.key_terms.len());
    }
    let _ = write!(out, " Reveal mode is {:?}.", text.reveal);
    out
}

fn describe_background(bg: &BackgroundDef) -> String {
    match bg {
        BackgroundDef::Solid { color } => format!("solid color {}", color),
        BackgroundDef::Gradient { gradient } => format!(
            "{:?} gradient with {} stops",
            gradient.direction,
            gradient.stops.len()
        ),
        BackgroundDef::Image { src, fit, .. } => {
            format!("image {} fitted with {:?} mode", src, fit)
        }
    }
}

fn describe_animation(anim: &AnimationDef) -> String {
    format!(
        "{:?} animation lasting {} ms with {:?} easing and {} ms delay",
        anim.kind, anim.duration_ms, anim.easing, anim.delay_ms
    )
}

fn describe_transition_kind(kind: &TransitionKind) -> &'static str {
    match kind {
        TransitionKind::Cut => "a cut",
        TransitionKind::Fade => "a fade",
        TransitionKind::Crossfade => "a crossfade",
        TransitionKind::WipeLeft => "a left wipe",
        TransitionKind::WipeRight => "a right wipe",
        TransitionKind::WipeUp => "an upward wipe",
        TransitionKind::WipeDown => "a downward wipe",
        TransitionKind::ZoomThrough => "a zoom-through",
        TransitionKind::BlurThrough => "a blur-through",
        TransitionKind::None => "no transition",
    }
}
