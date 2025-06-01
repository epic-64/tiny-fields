use crate::game::{Intent, MouseInput, UiRect};
use crate::palette;
use macroquad::color::{Color, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_rectangle, draw_text_ex, draw_texture_ex, get_internal_gl, measure_text, DrawTextureParams, QuadGl, Texture2D};
use macroquad::shapes::{draw_circle, draw_line, draw_rectangle_lines};
use macroquad::text::{Font, TextParams};

const BORDER_STRENGTH: f32 = 2.0;

#[derive(Clone, Eq, PartialEq)]
pub enum BorderStyle {
    None,
    Solid,
    Dotted,
}

#[derive(Clone)]
pub enum UiElement {
    Text {
        content: String,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
        font: Font,
    },
    RectButton {
        rectangle: UiRect,
        font_size: f32,
        font: Font,
        text: String,
        background_color: Color,
        text_color: Color,
        intent: Intent,
        parent_clip: Option<(i32, i32, i32, i32)>,
    },
    ImgButton {
        rectangle: UiRect,
        intent: Intent,
        texture: Texture2D,
        parent_clip: Option<(i32, i32, i32, i32)>,
    },
    ProgressBar {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        progress: f64,
        background_color: Color,
        foreground_color: Color,
        with_border: bool,
    },
    Circle { x: f32, y: f32, radius: f32, color: Color },
    Rectangle { x: f32, y: f32, width: f32, height: f32, color: Color, border_style: BorderStyle },
    Image { x: f32, y: f32, width: f32, height: f32, texture: Texture2D, color: Color },
    Scissor { clip: Option<(i32, i32, i32, i32)> },
}

pub fn is_hovered(command: &UiElement, mouse_input: &MouseInput) -> bool {
    match command {
        UiElement::RectButton { rectangle, parent_clip, .. } |
        UiElement::ImgButton { rectangle, parent_clip, .. } => {
            let clip_is_hovered = if let Some(clip) = parent_clip {
                let (x, y, w, h) = clip;
                UiRect {
                    x: *x as f32,
                    y: *y as f32,
                    w: *w as f32,
                    h: *h as f32
                }.is_hovered(mouse_input)
            } else {
                true
            };

            rectangle.is_hovered(mouse_input) && clip_is_hovered
        }
        _ => false,
    }
}

pub fn draw(command: &UiElement, mouse_input: &MouseInput) {
    let gl: &mut QuadGl = unsafe { get_internal_gl() }.quad_gl ;

    match command {
        UiElement::Text { content, x, y, font_size, color, font } => {
            draw_text_ex(content, *x, *y, TextParams{
                font: Some(font),
                font_size: *font_size as u16,
                color: *color,
                ..Default::default()
            });
        }
        UiElement::ProgressBar { x, y, width, height, progress, background_color, foreground_color, with_border } => {
            draw_rectangle(*x, *y, *width, *height, *background_color);
            draw_rectangle(*x, *y, *width * *progress as f32, *height, *foreground_color);

            if *with_border {
                let strength = BORDER_STRENGTH;
                draw_rectangle_lines(*x, *y, *width, *height, strength * 2.0, palette::BORDER.get_color());
            }
        }
        UiElement::Rectangle { x, y, width, height, color, border_style } => {
            draw_rectangle(*x, *y, *width, *height, *color);

            match border_style {
                BorderStyle::None => {}
                BorderStyle::Solid => {
                    let strength = BORDER_STRENGTH;
                    draw_rectangle_lines(*x, *y, *width, *height, strength * 2.0, palette::BORDER.get_color());
                }
                BorderStyle::Dotted => {
                    let strength = BORDER_STRENGTH;
                    draw_dotted_rectangle(
                        (*x + strength / 2.0).round(),
                        (*y + strength / 2.0).round(),
                        *width - strength,
                        *height - strength,
                        palette::BORDER.get_color(),
                        strength,
                        14,
                    );
                }
            }
        }
        UiElement::Circle { x, y, radius, color } => {
            draw_circle(*x, *y, *radius, *color);
        }
        UiElement::Image { x, y, width, height, texture, color } => {
            let params = DrawTextureParams {
                dest_size: Some(Vec2::new(*width, *height)),
                ..Default::default()
            };
            draw_texture_ex(texture, *x, *y, *color, params);
        }
        UiElement::RectButton { rectangle: r, font_size, text, background_color, text_color, font, .. } => {
            if is_hovered(command, mouse_input) {
                draw_rectangle(r.x - 2.0, r.y - 2.0, r.w + 4.0, r.h + 4.0, palette::BUTTON_HOVER.get_color());
            }

            draw_rectangle(r.x, r.y, r.w, r.h, *background_color);

            let the_font = Some(font);
            let text_measure = measure_text(text, the_font, *font_size as u16, 1.);
            let text_x = (r.x + (r.w - text_measure.width) / 2.0).round();
            let text_y = (r.y + (r.h + text_measure.height / 2.0) / 2.0).round();

            draw_text_ex(text, text_x, text_y, TextParams {
                font: the_font,
                font_size: *font_size as u16,
                color: *text_color,
                ..Default::default()
            });
        }
        UiElement::ImgButton { rectangle: r, texture, .. } => {
            if is_hovered(command, mouse_input) {
                draw_rectangle(r.x - 2.0, r.y - 2.0, r.w + 4.0, r.h + 4.0, palette::BUTTON_HOVER.get_color());
            }

            let params = DrawTextureParams {
                dest_size: Some(Vec2::new(r.w, r.h)),
                ..Default::default()
            };
            draw_texture_ex(texture, r.x, r.y, WHITE, params);
        }
        UiElement::Scissor { clip } => {
            gl.scissor(*clip)
        }
    }
}

pub fn pill(x: f32, y: f32, w: f32, h: f32, text: &str, text_color: Option<Color>, font: Font) -> Vec<UiElement> {
    let mut elements = Vec::new();
    let text_color = text_color.unwrap_or_else(|| palette::PILL_TEXT_COLOR.get_color());

    let radius = h / 2.0;

    // add circle on the left
    elements.push(UiElement::Circle {
        x: x,
        y: y + radius,
        radius: radius,
        color: palette::PILL_COLOR.get_color(),
    });

    // add rectangle in the middle
    elements.push(UiElement::Rectangle {
        x: x,
        y: y,
        width: w,
        height: h,
        color: palette::PILL_COLOR.get_color(),
        border_style: BorderStyle::None,
    });

    // add circle on the right
    elements.push(UiElement::Circle {
        x: x + w,
        y: y + radius,
        radius: radius,
        color: palette::PILL_COLOR.get_color(),
    });

    // add text in the middle
    let font_size = 15.0;
    let height = measure_text("8", Some(&font), font_size as u16, 1.0).height;
    let text_measure = measure_text(text, Some(&font), font_size as u16, 1.0);
    elements.push(UiElement::Text {
        content: text.to_string(),
        x: (x + w / 2.0 - text_measure.width / 2.0).round(),
        y: (y + h / 2.0 + height / 2.0).round() - 1.0,
        font_size: font_size,
        color: text_color,
        font: font,
    });

    elements
}

fn draw_dotted_line(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    color: Color,
    strength: f32,
    segments: u8,
) {
    let dx = (x2 - x1) / segments as f32;
    let dy = (y2 - y1) / segments as f32;

    let draw_len = 2; // draw 2 segments
    let gap_len = 1;  // skip 1 segment
    let pattern_len = draw_len + gap_len;

    let mut i = 0;
    while i < segments {
        // draw for `draw_len` segments, if there's enough remaining
        for j in 0..draw_len {
            if i + j >= segments {
                break;
            }

            draw_line(
                x1 + (i + j) as f32 * dx,
                y1 + (i + j) as f32 * dy,
                x1 + (i + j + 1) as f32 * dx,
                y1 + (i + j + 1) as f32 * dy,
                strength,
                color,
            );
        }

        i += pattern_len;
    }
}

fn draw_dotted_rectangle(x: f32, y: f32, width: f32, height: f32, color: Color, strength: f32, segments: u8) {
    draw_dotted_line(x, y, x + width, y, color, strength, segments);
    draw_dotted_line(x + width, y, x + width, y + height, color, strength, segments);
    draw_dotted_line(x + width, y + height, x, y + height, color, strength, segments);
    draw_dotted_line(x, y + height, x, y, color, strength, segments);
}