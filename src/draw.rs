use crate::game::{Intent, Job, MouseInput, UiRect};
use macroquad::color::{Color, SKYBLUE, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_rectangle, draw_text_ex, draw_texture_ex, get_internal_gl, measure_text, DrawTextureParams, QuadGl, Texture2D};
use macroquad::text::{Font, TextParams};

#[derive(Clone)]
pub enum UiElement {
    Text {
        content: String,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
        font: Option<Font>,
    },
    Button {
        rectangle: UiRect,
        font_size: f32,
        font: Option<Font>,
        text: String,
        color: Color,
        intent: Intent,
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
    },
    Rectangle { x: f32, y: f32, width: f32, height: f32, color: Color },
    Image { x: f32, y: f32, width: f32, height: f32, texture: Texture2D, color: Color },
    Scissor { clip: Option<(i32, i32, i32, i32)> },
    JobParticleMarker { x: f32, y: f32, job: Job},
}

pub fn is_hovered(command: &UiElement, mouse_input: &MouseInput) -> bool {
    match command {
        UiElement::Button { rectangle, parent_clip, .. } => {
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
                font: if let Some(f) = font { Some(f) } else { None },
                font_size: *font_size as u16,
                color: *color,
                ..Default::default()
            });
        }
        UiElement::ProgressBar { x, y, width, height, progress, background_color, foreground_color } => {
            draw_rectangle(*x, *y, *width, *height, *background_color);
            draw_rectangle(*x, *y, *width * *progress, *height, *foreground_color);
        }
        UiElement::Rectangle { x, y, width, height, color } => {
            draw_rectangle(*x, *y, *width as f32, *height as f32, *color);
        }
        UiElement::Image { x, y, width, height, texture, color } => {
            let params = DrawTextureParams {
                dest_size: Some(Vec2::new(*width as f32, *height as f32)),
                ..Default::default()
            };
            draw_texture_ex(texture, *x, *y, *color, params);
        }
        UiElement::Button { rectangle: r, font_size, text, color, font, .. } => {
            if is_hovered(command, mouse_input) {
                draw_rectangle(r.x - 2.0, r.y - 2.0, r.w + 4.0, r.h + 4.0, SKYBLUE);
            }

            draw_rectangle(r.x, r.y, r.w, r.h, *color);

            let the_font = if let Some(f) = font { Some(f) } else { None };
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
        UiElement::Scissor { clip } => {
            gl.scissor(*clip)
        }
        UiElement::JobParticleMarker { .. } => {}
    }
}