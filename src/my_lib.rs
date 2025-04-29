use macroquad::prelude::*;

#[derive(Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains_point(&self, point: (f32, f32)) -> bool {
        point.0 >= self.x && point.0 <= self.x + self.width &&
        point.1 >= self.y && point.1 <= self.y + self.height
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle(self.x, self.y, self.width, self.height, color);
    }
}

#[derive(Clone)]
pub struct Button {
    pub rect: Rectangle,
    pub color: Color,
    pub hover_color: Color,
    pub label: String,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color, hover_color: Color, label: &str) -> Self {
        Self {
            rect: Rectangle::new(x, y, width, height),
            color,
            hover_color,
            label: label.to_string(),
        }
    }

    pub fn draw(&self) {
        let color = if self.is_hovered() { self.hover_color } else { self.color };
        self.rect.draw(color);

        draw_text(&self.label, self.rect.x + 10.0, self.rect.y + 10.0, 20.0, BLACK);
    }

    pub fn is_hovered(&self) -> bool {
        let mouse = mouse_position();
        self.rect.contains_point(mouse)
    }

    pub fn is_clicked(&self) -> bool {
        self.is_hovered() && is_mouse_button_pressed(MouseButton::Left)
    }
}

pub const DEFAULT_FONT_SIZE: f32 = 30.0;
pub const DEFAULT_FONT_COLOR: Color = WHITE;

pub fn draw_text_primary(text: &str, x: f32, y: f32) {
    draw_text(text, x, y, DEFAULT_FONT_SIZE, DEFAULT_FONT_COLOR);
}

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
    ProgressBar {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        progress: f32,
        background_color: Color,
        foreground_color: Color,
    },
}

pub fn draw(commands: &[DrawCommand]) {
    for command in commands {
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
        }
    }
}

pub struct Progress {
    value: f32, // Value between 0.0 and 1.0
}

impl Progress {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    pub fn set(&mut self, value: f32) {
        self.value = value.clamp(0.0, 1.0);
    }

    pub fn get(&self) -> f32 {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
    }
}

pub struct ProgressBar {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub progress: Progress,
    pub background_color: Color,
    pub foreground_color: Color,
}

impl ProgressBar {
    pub fn new(x: f32, y: f32, width: f32, height: f32, background_color: Color, foreground_color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            progress: Progress::new(),
            background_color,
            foreground_color,
        }
    }

    pub fn set_progress(&mut self, value: f32) {
        self.progress.set(value);
    }

    pub fn draw(&self) {
        // Draw background
        draw_rectangle(self.x, self.y, self.width, self.height, self.background_color);

        // Draw foreground (progress)
        draw_rectangle(self.x, self.y, self.width * self.progress.get(), self.height, self.foreground_color);
    }
}

