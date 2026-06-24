use animations::LayerAnimation;
use skia_safe::{
    Canvas, Color, FontMgr, Point,
    textlayout::{
        FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign as SkTextAlign, TextStyle,
    },
};
use video_schema::{TextAlign, TextDef, TextReveal};

use crate::context::{RendererError, parse_hex_color};

/// Draw the text layer for one frame.
///
/// `revealed_word_count` — how many words to show (usize::MAX = all).
/// `animation`           — layer transform/opacity sampled by the animation crate.
pub fn draw_text_layer(
    canvas: &Canvas,
    canvas_w: i32,
    canvas_h: i32,
    def: &TextDef,
    revealed_word_count: usize,
    animation: &LayerAnimation,
) -> Result<(), RendererError> {
    // ── Build word list ───────────────────────────────────────────────────────
    let words: Vec<&str> = def.content.split_whitespace().collect();

    let visible_words = match def.reveal {
        TextReveal::AllAtOnce => &words[..],
        TextReveal::WordByWord => {
            let n = revealed_word_count.min(words.len());
            &words[..n]
        }
        // Line-by-line and typewriter share the same count mechanic for v1.
        TextReveal::LineByLine | TextReveal::Typewriter => {
            let n = revealed_word_count.min(words.len());
            &words[..n]
        }
    };

    if visible_words.is_empty() {
        return Ok(());
    }

    // ── Font ─────────────────────────────────────────────────────────────────
    let font_def = def.font.as_ref();
    let font_size = font_def.map(|f| f.size).unwrap_or(64) as f32;

    // ── Colours ───────────────────────────────────────────────────────────────
    let base_color_str = def.color.as_deref().unwrap_or("#ffffff");
    let base_color = parse_hex_color(base_color_str)?;

    // Apply fade alpha from animation progress
    let alpha = (animation.opacity * def.opacity * 255.0) as u8;
    let draw_color = Color::from_argb(alpha, base_color.r(), base_color.g(), base_color.b());

    // ── Layout ────────────────────────────────────────────────────────────────
    let (cx, cy) = def
        .position
        .as_ref()
        .map(|p| p.resolve(canvas_w as f32, canvas_h as f32))
        .unwrap_or((canvas_w as f32 / 2.0, canvas_h as f32 / 2.0));

    let max_width_frac = def.max_width.unwrap_or(0.85);
    let max_width = canvas_w as f32 * max_width_frac;

    // ── Build paragraph using Skia textlayout ────────────────────────────────
    let font_mgr = FontMgr::new();
    let mut fc = FontCollection::new();
    fc.set_default_font_manager(font_mgr, None);

    let mut para_style = ParagraphStyle::new();
    para_style.set_text_align(map_text_align(&def.align));

    let mut ts = TextStyle::new();
    ts.set_font_size(font_size);
    ts.set_color(draw_color);

    // Bold weight from font def
    if let Some(fd) = font_def {
        use video_schema::FontWeight;
        let weight = match fd.weight {
            FontWeight::Thin => skia_safe::font_style::Weight::THIN,
            FontWeight::Light => skia_safe::font_style::Weight::LIGHT,
            FontWeight::Regular => skia_safe::font_style::Weight::NORMAL,
            FontWeight::Medium => skia_safe::font_style::Weight::MEDIUM,
            FontWeight::Semibold => skia_safe::font_style::Weight::SEMI_BOLD,
            FontWeight::Bold => skia_safe::font_style::Weight::BOLD,
            FontWeight::Black => skia_safe::font_style::Weight::BLACK,
        };
        ts.set_font_style(skia_safe::FontStyle::new(
            weight,
            skia_safe::font_style::Width::NORMAL,
            skia_safe::font_style::Slant::Upright,
        ));
    }

    let mut builder = ParagraphBuilder::new(&para_style, &fc);

    // ── Word-by-word building with key term styling ───────────────────────────
    for (i, word) in visible_words.iter().enumerate() {
        let suffix = if i + 1 < visible_words.len() { " " } else { "" };

        // Check if this word matches a key term
        let key_term = def
            .key_terms
            .iter()
            .find(|kt| kt.word.split_whitespace().next() == Some(word));

        if let Some(kt) = key_term {
            // Apply key term overrides
            let mut kts = ts.clone();

            if let Some(ref color_str) = kt.color {
                if let Ok(c) = parse_hex_color(color_str) {
                    let kc = Color::from_argb(alpha, c.r(), c.g(), c.b());
                    kts.set_color(kc);
                }
            }

            if kt.bold {
                kts.set_font_style(skia_safe::FontStyle::bold());
            }

            builder.push_style(&kts);
            builder.add_text(&format!("{word}{suffix}"));
            builder.pop();
        } else {
            builder.push_style(&ts);
            builder.add_text(&format!("{word}{suffix}"));
            builder.pop();
        }
    }

    let mut paragraph = builder.build();
    paragraph.layout(max_width);

    canvas.save();
    canvas.translate((
        animation.translate_x * canvas_w as f32,
        animation.translate_y * canvas_h as f32,
    ));
    canvas.translate((cx, cy));
    canvas.scale((animation.scale_x, animation.scale_y));
    canvas.translate((-cx, -cy));

    // ── Shadow (draw first, behind text) ─────────────────────────────────────
    if let Some(shadow) = &def.shadow {
        if let Ok(sc) = parse_hex_color(&shadow.color) {
            let sa = Color::from_argb(alpha, sc.r(), sc.g(), sc.b());
            let shadow_x = cx - (paragraph.max_width() / 2.0) + shadow.offset_x;
            let shadow_y = cy - (paragraph.height() / 2.0) + shadow.offset_y;

            let mut shadow_ts = ts.clone();
            shadow_ts.set_color(sa);

            let mut sb = ParagraphBuilder::new(&para_style, &fc);
            sb.push_style(&shadow_ts);
            sb.add_text(&visible_words.join(" "));
            let mut sp = sb.build();
            sp.layout(max_width);

            if animation.blur_px > 0.0 {
                let blur_steps = [
                    (-animation.blur_px * 0.2, 0.0),
                    (animation.blur_px * 0.2, 0.0),
                    (0.0, -animation.blur_px * 0.2),
                    (0.0, animation.blur_px * 0.2),
                ];
                for (dx, dy) in blur_steps {
                    sp.paint(canvas, Point::new(shadow_x + dx, shadow_y + dy));
                }
            } else {
                sp.paint(canvas, Point::new(shadow_x, shadow_y));
            }
        }
    }

    // ── Draw paragraph centred at (cx, cy) ───────────────────────────────────
    let x = cx - (paragraph.max_width() / 2.0);
    let y = cy - (paragraph.height() / 2.0);
    paragraph.paint(canvas, Point::new(x, y));
    canvas.restore();

    Ok(())
}

fn map_text_align(align: &TextAlign) -> SkTextAlign {
    match align {
        TextAlign::Left => SkTextAlign::Left,
        TextAlign::Center => SkTextAlign::Center,
        TextAlign::Right => SkTextAlign::Right,
        TextAlign::Justify => SkTextAlign::Justify,
    }
}
