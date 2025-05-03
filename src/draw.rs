use crate::game::UiRect;
use macroquad::color::{Color, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_rectangle, draw_text, draw_texture_ex, measure_text, DrawTextureParams, Texture2D};

pub enum DrawCommand {
    Text {
        content: String,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
    },
    Button {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        font_size: f32,
        text: String,
        color: Color,
        hover_color: Color,
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
    Image { x: f32, y: f32, width: f64, height: f64, texture: Texture2D },
}

pub fn draw_multiple(commands: &[DrawCommand]) -> () {
    for command in commands {
        draw(command);
    }
}

pub fn draw(command: &DrawCommand) {
    match command {
        DrawCommand::Text { content, x, y, font_size, color } => {
            draw_text(content, *x, *y, *font_size, *color);
        }
        DrawCommand::ProgressBar { x, y, width, height, progress, background_color, foreground_color } => {
            draw_rectangle(*x, *y, *width, *height, *background_color);
            draw_rectangle(*x, *y, *width * *progress, *height, *foreground_color);
        }
        DrawCommand::Rectangle { x, y, width, height, color } => {
            draw_rectangle(*x, *y, *width as f32, *height as f32, *color);
        }
        DrawCommand::Image { x, y, width, height, texture } => {
            let params = DrawTextureParams {
                dest_size: Some(Vec2::new(*width as f32, *height as f32)),
                ..Default::default()
            };
            draw_texture_ex(texture, *x, *y, WHITE, params);
        }
        DrawCommand::Button { x, y, width, height, font_size, text, color, hover_color } => {
            let rect = UiRect {
                x: *x,
                y: *y,
                width: *width,
                height: *height,
            };
            let current_color = if rect.is_hovered() { *hover_color } else { *color };
            draw_rectangle(*x, *y, *width, *height, current_color);

            let text_measure = measure_text(text, None, *font_size as u16, 1.);
            let text_x = *x + (*width - text_measure.width) / 2.0;
            let text_y = *y + (*height - text_measure.height) / 2.0 + font_size / 2.0;

            draw_text(text, text_x, text_y, *font_size, WHITE);
        }
    }
}