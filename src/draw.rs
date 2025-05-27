use crate::game::{Intent, JobInstance, MouseInput, UiRect};
use crate::palette;
use macroquad::color::{Color, SKYBLUE, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_rectangle, draw_text_ex, draw_texture, draw_texture_ex, get_internal_gl, measure_text, DrawTextureParams, QuadGl, Texture2D};
use macroquad::shapes::draw_circle;
use macroquad::text::{Font, TextParams};
use macroquad::texture::set_default_filter_mode;

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
        color: Color,
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
        progress: f32,
        background_color: Color,
        foreground_color: Color,
        with_border: bool,
    },
    Circle { x: f32, y: f32, radius: f32, color: Color },
    Rectangle { x: f32, y: f32, width: f32, height: f32, color: Color, bordered: bool },
    Image { x: f32, y: f32, width: f32, height: f32, texture: Texture2D, color: Color },
    Scissor { clip: Option<(i32, i32, i32, i32)> },
    JobParticleMarker { x: f32, y: f32, job: JobInstance },
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
            if *with_border {
                let strength = 2.0;
                draw_rectangle(*x, *y, *width, *height, palette::BORDER.get_color());
                draw_rectangle(*x + strength, *y + strength, *width - strength * 2., *height - strength * 2., *background_color);
                draw_rectangle(*x + strength, *y + strength, (*width - strength * 2.) * *progress, *height - strength * 2., *foreground_color);
            } else {
                draw_rectangle(*x, *y, *width, *height, *background_color);
                draw_rectangle(*x, *y, *width * *progress, *height, *foreground_color);
            }
        }
        UiElement::Rectangle { x, y, width, height, color, bordered } => {
            if *bordered {
                let strength = 2.0;
                draw_rectangle(*x, *y, *width, *height, palette::BORDER.get_color());
                draw_rectangle(*x + strength, *y + strength, *width - strength * 2., *height - strength * 2., *color);
            } else {
                draw_rectangle(*x, *y, *width, *height, *color);
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
        UiElement::RectButton { rectangle: r, font_size, text, color, font, .. } => {
            if is_hovered(command, mouse_input) {
                draw_rectangle(r.x - 2.0, r.y - 2.0, r.w + 4.0, r.h + 4.0, SKYBLUE);
            }

            draw_rectangle(r.x, r.y, r.w, r.h, *color);

            let the_font = Some(font);
            let text_measure = measure_text(text, the_font, *font_size as u16, 1.);
            let text_x = (r.x + (r.w - text_measure.width) / 2.0).round();
            let text_y = (r.y + (r.h + text_measure.height / 2.0) / 2.0).round();

            draw_text_ex(text, text_x, text_y, TextParams {
                font: the_font,
                font_size: *font_size as u16,
                color: WHITE,
                ..Default::default()
            });
        }
        UiElement::ImgButton { rectangle: r, texture, .. } => {
            if is_hovered(command, mouse_input) {
                draw_rectangle(r.x - 2.0, r.y - 2.0, r.w + 4.0, r.h + 4.0, SKYBLUE);
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
        UiElement::JobParticleMarker { .. } => {}
    }
}

pub fn pill(x: f32, y: f32, w: f32, h: f32, text: &str, text_color: Color, font: Font) -> Vec<UiElement> {
    let mut elements = Vec::new();

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
        bordered: false,
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