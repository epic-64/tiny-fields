use crate::game::{Intent, UiRect};
use macroquad::color::{Color, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_rectangle, draw_text, draw_texture_ex, get_internal_gl, measure_text, DrawTextureParams, QuadGl, Texture2D};

#[derive(Clone)]
pub enum UiElement {
    Text {
        content: String,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
    },
    Button {
        rectangle: UiRect,
        font_size: f32,
        text: String,
        color: Color,
        hover_color: Color,
        is_hovered: bool,
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
    Rectangle { x: f32, y: f32, width: f64, height: f64, color: Color },
    Image { x: f32, y: f32, width: f32, height: f32, texture: Texture2D, color: Color },
    Scissor { clip: Option<(i32, i32, i32, i32)> },
}

pub fn draw(command: &UiElement) {
    let gl: &mut QuadGl = unsafe { get_internal_gl() }.quad_gl ;

    match command {
        UiElement::Text { content, x, y, font_size, color } => {
            draw_text(content, *x, *y, *font_size, *color);
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
        UiElement::Button { rectangle: r, font_size, text, color, is_hovered, .. } => {
            if *is_hovered {
                draw_rectangle(r.x - 2.0, r.y - 2.0, r.w + 4.0, r.h + 4.0, WHITE);
            }

            draw_rectangle(r.x, r.y, r.w, r.h, *color);

            let text_measure = measure_text(text, None, *font_size as u16, 1.);
            let text_x = r.x + (r.w - text_measure.width) / 2.0;
            let text_y = r.y + (r.h - text_measure.height) / 2.0 + font_size / 2.0;

            draw_text(text, text_x, text_y, *font_size, WHITE);
        }
        UiElement::Scissor { clip } => {
            gl.scissor(*clip)
        }
    }
}