use skia_safe::{Canvas, Paint, Point, Rect, TileMode, gradient_shader};
use video_schema::{BackgroundDef, GradientDirection};

use crate::context::{RendererError, parse_hex_color};

/// Clears the canvas and draws the background layer.
pub fn draw_background(
    canvas: &Canvas,
    width: i32,
    height: i32,
    bg: &BackgroundDef,
) -> Result<(), RendererError> {
    match bg {
        BackgroundDef::Solid { color } => {
            let c = parse_hex_color(color)?;
            canvas.clear(c);
        }

        BackgroundDef::Gradient { gradient } => {
            let stops: Vec<skia_safe::Color> = gradient
                .stops
                .iter()
                .map(|s| parse_hex_color(&s.color))
                .collect::<Result<_, _>>()?;

            let positions: Vec<f32> = gradient.stops.iter().map(|s| s.position).collect();

            let rect = Rect::from_wh(width as f32, height as f32);

            let (start, end) = gradient_endpoints(&gradient.direction, width, height);

            let shader = gradient_shader::linear(
                (start, end),
                stops.as_slice(),
                Some(positions.as_slice()),
                TileMode::Clamp,
                None,
                None,
            );

            let mut paint = Paint::default();
            paint.set_shader(shader);
            canvas.draw_rect(rect, &paint);
        }

        // Image background — stub for v1; falls back to solid black.
        BackgroundDef::Image { overlay, .. } => {
            canvas.clear(skia_safe::Color::BLACK);

            if let Some(ov) = overlay {
                let c = parse_hex_color(&ov.color)?;
                let alpha = (ov.opacity * 255.0) as u8;
                let ov_color = skia_safe::Color::from_argb(alpha, c.r(), c.g(), c.b());
                let mut paint = Paint::default();
                paint.set_color(ov_color);
                canvas.draw_rect(Rect::from_wh(width as f32, height as f32), &paint);
            }
        }
    }

    Ok(())
}

fn gradient_endpoints(dir: &GradientDirection, width: i32, height: i32) -> (Point, Point) {
    let (w, h) = (width as f32, height as f32);
    match dir {
        GradientDirection::Horizontal => (Point::new(0.0, 0.0), Point::new(w, 0.0)),
        GradientDirection::Vertical => (Point::new(0.0, 0.0), Point::new(0.0, h)),
        GradientDirection::DiagonalTl => (Point::new(0.0, 0.0), Point::new(w, h)),
        GradientDirection::DiagonalTr => (Point::new(w, 0.0), Point::new(0.0, h)),
        // Radial is handled separately; fall back to vertical for now.
        GradientDirection::Radial => (Point::new(0.0, 0.0), Point::new(0.0, h)),
    }
}
