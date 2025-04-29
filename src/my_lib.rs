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
    Rectangle { x: f32, y: f32, width: f64, height: f64, color: Color },
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
            DrawCommand::Rectangle { x, y, width, height, color } => {
                draw_rectangle(*x, *y, *width as f32, *height as f32, *color);
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

    pub fn reset(&mut self) {
        self.progress.reset();
    }
}

pub struct Job {
    pub name: String,
    pub progress: ProgressBar, // Progress for actions
    pub level_up_progress: ProgressBar, // Progress for leveling up
    pub production_rate: i32,
    pub level: i32,
    pub money_per_action: i32,
    pub action_duration: f32,
    pub time_accumulator: f32,
    pub running: bool,
    pub control_button: Button,
    pub actions_until_level_up: i32, // Remaining actions to level up
    pub actions_done: i32, // Tracks completed actions
}

impl Job {
    pub fn new(
        name: &str,
        x: f32,
        y: f32,
        production_rate: i32,
        level: i32,
        money_per_action: i32,
        action_duration: f32,
    ) -> Self {
        Self {
            name: name.to_string(),
            progress: ProgressBar::new(x, y, 300.0, 20.0, GRAY, GREEN),
            level_up_progress: ProgressBar::new(x, y + 30.0, 300.0, 20.0, GRAY, BLUE),
            production_rate,
            level,
            money_per_action,
            action_duration,
            time_accumulator: 0.0,
            running: false,
            control_button: Button::new(x + 320.0, y, 100.0, 30.0, WHITE, GRAY, "Start"),
            actions_until_level_up: 10, // Example: 10 actions to level up
            actions_done: 0,
        }
    }

    pub fn toggle_running(&mut self) {
        self.running = !self.running;
        self.control_button.label = if self.running { "Stop".to_string() } else { "Start".to_string() };
    }

    pub fn update_progress(&mut self, dt: f32) -> i32 {
        self.time_accumulator += dt;
        self.progress.set_progress(self.time_accumulator / self.action_duration);

        if self.time_accumulator >= self.action_duration {
            self.time_accumulator -= self.action_duration;
            self.actions_done += 1;
            self.level_up_progress.set_progress(self.actions_done as f32 / self.actions_until_level_up as f32);

            if self.actions_done >= self.actions_until_level_up {
                self.level_up();
            }

            return self.money_per_action * self.level;
        }

        0
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.money_per_action += 5; // Example: Increase money per action
        self.actions_done = 0;
        self.level_up_progress.reset();
    }

    pub fn dollars_per_second(&self) -> f32 {
        (self.money_per_action as f32 / self.action_duration) * self.level as f32
    }
}

pub struct JobRenderer {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl JobRenderer {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn render(&self, job: &Job) -> Vec<DrawCommand> {
        let mut commands = vec![];

        // Card background
        commands.push(DrawCommand::Rectangle {
            x: self.x,
            y: self.y,
            width: self.width as f64,
            height: self.height as f64,
            color: DARKGRAY,
        });

        // Job name
        commands.push(DrawCommand::Text {
            content: format!("Job: {}", job.name),
            x: self.x + 10.0,
            y: self.y + 20.0,
            font_size: 24.0,
            color: WHITE,
        });

        // Level and $ per action
        commands.push(DrawCommand::Text {
            content: format!("Level: {} | $/Action: {}", job.level, job.money_per_action),
            x: self.x + 10.0,
            y: self.y + 50.0,
            font_size: 20.0,
            color: LIGHTGRAY,
        });

        // Seconds per action and $ per second
        commands.push(DrawCommand::Text {
            content: format!("Sec/Action: {:.2} | $/Sec: {:.2}", job.action_duration, job.dollars_per_second()),
            x: self.x + 10.0,
            y: self.y + 80.0,
            font_size: 20.0,
            color: LIGHTGRAY,
        });

        // Actions until level up
        commands.push(DrawCommand::Text {
            content: format!("Actions to Level Up: {}", job.actions_until_level_up - job.actions_done),
            x: self.x + 10.0,
            y: self.y + 110.0,
            font_size: 20.0,
            color: LIGHTGRAY,
        });

        // Action progress bar
        commands.push(DrawCommand::ProgressBar {
            x: self.x + 10.0,
            y: self.y + 140.0,
            width: self.width - 20.0,
            height: 20.0,
            progress: job.progress.progress.get(),
            background_color: GRAY,
            foreground_color: GREEN,
        });

        // Level-up progress bar
        commands.push(DrawCommand::ProgressBar {
            x: self.x + 10.0,
            y: self.y + 170.0,
            width: self.width - 20.0,
            height: 20.0,
            progress: job.level_up_progress.progress.get(),
            background_color: GRAY,
            foreground_color: BLUE,
        });

        // Control button
        commands.push(DrawCommand::Button {
            button: Button::new(
                self.x + self.width - 110.0,
                self.y + 140.0,
                100.0,
                30.0,
                WHITE,
                GRAY,
                &job.control_button.label,
            ),
        });

        commands
    }
}