use crate::easing::{apply_easing, ease_out_bounce, lerp};
use crate::state::LayerAnimation;
use video_schema::{AnimationDef, AnimationType, SegmentDef};

/// Evaluates the animation state of a specific segment at a given frame.
///
/// This function calculates the visual state of a segment's animation for a specific frame
/// based on its defined "animation_in" and "animation_out" properties, the segment's start frame,
/// and the given frame's timing within the animation range. It considers the easing, progress,
/// and animation type to produce a resulting `LayerAnimation` state.
///
/// # Arguments
///
/// * `seg` - A reference to a `SegmentDef` containing animation definitions for the segment.
/// * `segment_start_frame` - A `u64` representing the frame at which the segment starts.
/// * `frame` - A `u64` representing the current frame for which the animation state is being evaluated.
/// * `fps` - A `u32` indicating the frames per second for the animation, used to convert durations to frame counts.
/// * `segment_frame_count` - A `u64` representing the total frame count within the segment, to determine animation boundaries.
///
/// # Returns
///
/// Returns a `LayerAnimation` struct that represents the combined visual state of the segment's
/// "animation_in" and "animation_out" at the given frame. If no animations are defined or the frame
/// does not fall within an animation range, a neutral `LayerAnimation` is returned.
pub fn evaluate_segment_animation(
    seg: &SegmentDef,
    segment_start_frame: u64,
    frame: u64,
    fps: u32,
    segment_frame_count: u64,
) -> LayerAnimation {
    let local = frame.saturating_sub(segment_start_frame);
    let mut state = LayerAnimation::neutral();

    if let Some(anim_in) = seg.animation_in.as_ref() {
        let in_frames = ms_to_frames(anim_in.duration_ms, fps);
        if in_frames > 0 && local < in_frames {
            let progress = local as f32 / in_frames as f32;
            state = state.combine(sample_animation(anim_in, progress, true));
        }
    }

    if let Some(anim_out) = seg.animation_out.as_ref() {
        let out_frames = ms_to_frames(anim_out.duration_ms, fps);
        if out_frames > 0 {
            let out_start = segment_frame_count.saturating_sub(out_frames);
            if local >= out_start {
                let out_local = local - out_start;
                let progress = (out_local as f32 / out_frames as f32).clamp(0.0, 1.0);
                state = state.combine(sample_animation(anim_out, progress, false));
            }
        }
    }

    state
}

fn ms_to_frames(duration_ms: u32, fps: u32) -> u64 {
    ((duration_ms as f64 / 1000.0) * fps as f64).round() as u64
}

fn sample_animation(def: &AnimationDef, progress: f32, entering: bool) -> LayerAnimation {
    let t = apply_easing(progress.clamp(0.0, 1.0), &def.easing);
    match def.kind {
        AnimationType::Fade => LayerAnimation {
            opacity: if entering { t } else { 1.0 - t },
            ..LayerAnimation::neutral()
        },
        AnimationType::SlideUp => slide(0.0, if entering { 0.10 } else { -0.10 }, t),
        AnimationType::SlideDown => slide(0.0, if entering { -0.10 } else { 0.10 }, t),
        AnimationType::SlideLeft => slide(if entering { 0.10 } else { -0.10 }, 0.0, t),
        AnimationType::SlideRight => slide(if entering { -0.10 } else { 0.10 }, 0.0, t),
        AnimationType::ZoomIn => zoom(
            if entering { 0.88 } else { 1.0 },
            if entering { 1.0 } else { 1.12 },
            t,
        ),
        AnimationType::ZoomOut => zoom(
            if entering { 1.12 } else { 1.0 },
            if entering { 1.0 } else { 0.88 },
            t,
        ),
        AnimationType::FlipX => flip(true, entering, t),
        AnimationType::FlipY => flip(false, entering, t),
        AnimationType::BlurIn => blur(
            if entering { 24.0 } else { 0.0 },
            if entering { 0.0 } else { 24.0 },
            t,
        ),
        AnimationType::BlurOut => blur(
            if entering { 0.0 } else { 24.0 },
            if entering { 24.0 } else { 0.0 },
            t,
        ),
        AnimationType::Bounce => bounce(entering, t),
        AnimationType::None => LayerAnimation::neutral(),
    }
}

fn slide(from_x: f32, from_y: f32, t: f32) -> LayerAnimation {
    LayerAnimation {
        opacity: 1.0,
        translate_x: lerp(from_x, 0.0, t),
        translate_y: lerp(from_y, 0.0, t),
        ..LayerAnimation::neutral()
    }
}

fn zoom(from: f32, to: f32, t: f32) -> LayerAnimation {
    let s = lerp(from, to, t);
    LayerAnimation {
        opacity: 1.0,
        scale_x: s,
        scale_y: s,
        ..LayerAnimation::neutral()
    }
}

fn flip(horizontal: bool, entering: bool, t: f32) -> LayerAnimation {
    let from = -1.0;
    let to = 1.0;
    let scale = lerp(from, to, t);
    let mut out = LayerAnimation::neutral();
    if horizontal {
        out.scale_x = if entering { scale } else { -scale };
    } else {
        out.scale_y = if entering { scale } else { -scale };
    }
    out
}

fn blur(from: f32, to: f32, t: f32) -> LayerAnimation {
    LayerAnimation {
        blur_px: lerp(from, to, t).max(0.0),
        ..LayerAnimation::neutral()
    }
}

fn bounce(entering: bool, t: f32) -> LayerAnimation {
    let eased = ease_out_bounce(t);
    LayerAnimation {
        opacity: 1.0,
        translate_y: if entering {
            lerp(0.12, 0.0, eased)
        } else {
            lerp(0.0, -0.12, eased)
        },
        ..LayerAnimation::neutral()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use video_schema::{AnimationType, Easing, Mood, Pacing, SegmentKind};

    fn bare_segment(
        animation_in: Option<AnimationDef>,
        animation_out: Option<AnimationDef>,
    ) -> SegmentDef {
        SegmentDef {
            id: 1,
            kind: SegmentKind::Speech,
            duration_override_seconds: None,
            text: None,
            elevenlabs: None,
            background: None,
            animation_in,
            animation_out,
            post_hold_seconds: 0.0,
            mood: Mood::Neutral,
            pacing: Pacing::Normal,
            audio: None,
            transition_to_next: None,
            control: Default::default(),
        }
    }

    #[test]
    fn fade_in_starts_transparent() {
        let seg = bare_segment(
            Some(AnimationDef {
                kind: AnimationType::Fade,
                duration_ms: 1000,
                easing: Easing::Linear,
                delay_ms: 0,
            }),
            None,
        );

        let state = evaluate_segment_animation(&seg, 0, 0, 30, 30);
        assert_eq!(state.opacity, 0.0);
    }

    #[test]
    fn fade_out_finishes_transparent() {
        let seg = bare_segment(
            None,
            Some(AnimationDef {
                kind: AnimationType::Fade,
                duration_ms: 1000,
                easing: Easing::Linear,
                delay_ms: 0,
            }),
        );

        let state = evaluate_segment_animation(&seg, 0, 29, 30, 30);
        assert!(state.opacity < 0.1);
    }
}
