use video_schema::Easing;

pub(crate) fn apply_easing(t: f32, easing: &Easing) -> f32 {
    match easing {
        Easing::Linear => t,
        Easing::EaseIn => t * t,
        Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
        Easing::EaseInOut => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - ((-2.0 * t + 2.0).powi(2)) / 2.0
            }
        }
        Easing::Spring => spring(t),
        Easing::BounceEase => ease_out_bounce(t),
    }
}

pub(crate) fn lerp(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t
}

pub(crate) fn spring(t: f32) -> f32 {
    let damped = (1.0 - t).powf(2.0);
    let oscillation = (t * 12.0).sin() * 0.06;
    (1.0 - damped + oscillation).clamp(0.0, 1.2)
}

pub(crate) fn ease_out_bounce(t: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;
    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use video_schema::Easing;

    #[test]
    fn linear_easing_passes_through() {
        assert_eq!(apply_easing(0.42, &Easing::Linear), 0.42);
    }

    #[test]
    fn ease_out_reaches_one_at_the_end() {
        assert_eq!(apply_easing(1.0, &Easing::BounceEase), 1.0);
    }
}
