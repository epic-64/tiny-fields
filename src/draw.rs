use crate::render::Button;
use macroquad::color::{Color, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_rectangle, draw_text, draw_texture_ex, DrawTextureParams, Texture2D};

pub enum DrawCommand {
    Text {
        content: String,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
    },
    Button {
        button: Button,
    },
    Button2 {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
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
        DrawCommand::Button { button } => {
            button.draw();
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
        DrawCommand::Button2 { x, y, width, height, text, color, hover_color } => {
            let is_hovered = false; // Replace with actual hover detection logic
            let current_color = if is_hovered { *hover_color } else { *color };
            draw_rectangle(*x, *y, *width, *height, current_color);
            draw_text(text, *x + 10.0, *y + 10.0, 20.0, WHITE);
        }
    }
}