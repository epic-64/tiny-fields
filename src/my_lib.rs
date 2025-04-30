use macroquad::math::i64;
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
    pub action_progress: ProgressBar, // Progress for actions
    pub level_up_progress: ProgressBar, // Progress for leveling up
    pub production_rate: i32,
    pub level: i32,
    pub base_money_per_action: i32,
    pub action_duration: f32,
    pub time_accumulator: f32,
    pub running: bool,
    pub control_button: Button,
    pub actions_until_level_up: i32, // Remaining actions to level up
    pub actions_done: i32, // Tracks completed actions
    pub timeslot_cost: i32,
}

impl Job {
    pub fn new(
        name: &str,
        x: f32,
        y: f32,
        production_rate: i32,
        level: i32,
        base_money_per_action: i32,
        action_duration: f32,
        timeslot_cost: i32,
    ) -> Self {
        let button_x = x + 310.0;
        let button_y = y + 140.0;

        Self {
            name: name.to_string(),
            action_progress: ProgressBar::new(x + 10.0, y + 140.0, 300.0, 20.0, GRAY, GREEN),
            level_up_progress: ProgressBar::new(x + 10.0, y + 170.0, 300.0, 20.0, GRAY, BLUE),
            production_rate,
            level,
            base_money_per_action,
            action_duration,
            time_accumulator: 0.0,
            running: false,
            control_button: Button::new(button_x, button_y, 100.0, 30.0, WHITE, GRAY, "Start"),
            actions_until_level_up: 10,
            actions_done: 0,
            timeslot_cost,
        }
    }

    pub fn toggle_running(&mut self, free_timeslots: i32) -> Option<Event> {
        if self.running {
            self.running = false;
            self.control_button.label = "Start".to_string();
            Some(Event::TimeslotChanged)
        } else if free_timeslots >= self.timeslot_cost {
            self.running = true;
            self.control_button.label = "Stop".to_string();
            Some(Event::TimeslotChanged)
        } else {
            None
        }
    }

    pub fn update_progress(&mut self, dt: f32) -> i64 {
        self.time_accumulator += dt;
        self.action_progress.set_progress(self.time_accumulator / self.action_duration);

        if self.time_accumulator >= self.action_duration {
            self.time_accumulator -= self.action_duration;
            self.actions_done += 1;
            self.level_up_progress.set_progress(self.actions_done as f32 / self.actions_until_level_up as f32);

            if self.actions_done >= self.actions_until_level_up {
                self.level_up();
            }

            return self.dollars_per_action();
        }

        0
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.actions_done = 0;
        self.level_up_progress.reset();
    }

    pub fn dollars_multiplier(&self) -> f32 {
        let level_multiplier = 0.3;
        let level_portion = self.level - 1;

        1.0 + (level_portion as f32 * level_multiplier)
    }

    pub fn dollars_per_action(&self) -> i64 {
        (self.base_money_per_action as f32 * self.dollars_multiplier()) as i64
    }

    pub fn dollars_per_second(&self) -> i64 {
        (self.dollars_per_action() / self.action_duration as i64)
    }
}

pub struct JobRenderer {}

impl JobRenderer {
    const CARD_PADDING: f32 = 10.0;
    const CARD_SPACING: f32 = 30.0;
    const TEXT_FONT_SIZE_LARGE: f32 = 24.0;
    const TEXT_FONT_SIZE_SMALL: f32 = 20.0;
    const PROGRESS_BAR_HEIGHT: f32 = 20.0;
    const BACKGROUND_COLOR: Color = DARKGRAY;
    const TEXT_COLOR_PRIMARY: Color = WHITE;
    const TEXT_COLOR_SECONDARY: Color = LIGHTGRAY;
    const PROGRESS_BAR_BACKGROUND: Color = GRAY;
    const PROGRESS_BAR_FOREGROUND_ACTION: Color = GREEN;
    const PROGRESS_BAR_FOREGROUND_LEVEL: Color = BLUE;

    pub fn render(&self, job: &Job, x: f32, y: f32, width: f64, height: f64, ) -> Vec<DrawCommand> {
        let mut commands = vec![];

        // Card background
        commands.push(DrawCommand::Rectangle {
            x,
            y,
            width,
            height,
            color: Self::BACKGROUND_COLOR,
        });

        // Job name
        commands.push(DrawCommand::Text {
            content: format!("Job: {}", job.name),
            x: x + Self::CARD_PADDING,
            y: y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE,
            font_size: Self::TEXT_FONT_SIZE_LARGE,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Level and $ per action
        commands.push(DrawCommand::Text {
            content: format!("Level: {} | $/Action: {}", job.level, job.dollars_per_action()),
            x: x + Self::CARD_PADDING,
            y: y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE + Self::CARD_SPACING,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_SECONDARY,
        });

        // Seconds per action and $ per second
        commands.push(DrawCommand::Text {
            content: format!("Sec/Action: {:.2} | $/Sec: {:.2}", job.action_duration, job.dollars_per_second()),
            x: x + Self::CARD_PADDING,
            y: y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE + 2.0 * Self::CARD_SPACING,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_SECONDARY,
        });

        // Actions until level up
        commands.push(DrawCommand::Text {
            content: format!("Actions to Level Up: {}", job.actions_until_level_up - job.actions_done),
            x: x + Self::CARD_PADDING,
            y: y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE + 3.0 * Self::CARD_SPACING,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_SECONDARY,
        });

        // Action progress bar
        commands.push(DrawCommand::ProgressBar {
            x: x + Self::CARD_PADDING,
            y: y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE + 4.0 * Self::CARD_SPACING,
            width: width as f32 - 2.0 * Self::CARD_PADDING,
            height: Self::PROGRESS_BAR_HEIGHT,
            progress: job.action_progress.progress.get(),
            background_color: Self::PROGRESS_BAR_BACKGROUND,
            foreground_color: Self::PROGRESS_BAR_FOREGROUND_ACTION,
        });

        commands.push(DrawCommand::Text {
            content: format!("{:.1} / {:.1}", job.time_accumulator, job.action_duration),
            x: x + Self::CARD_PADDING + 10.0,
            y: y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE + 4.0 * Self::CARD_SPACING + 15.0,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Level-up progress bar
        commands.push(DrawCommand::ProgressBar {
            x: x + Self::CARD_PADDING,
            y: y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE + 5.0 * Self::CARD_SPACING,
            width: width as f32 - 2.0 * Self::CARD_PADDING,
            height: Self::PROGRESS_BAR_HEIGHT,
            progress: job.level_up_progress.progress.get(),
            background_color: Self::PROGRESS_BAR_BACKGROUND,
            foreground_color: Self::PROGRESS_BAR_FOREGROUND_LEVEL,
        });

        // Control button
        commands.push(DrawCommand::Button {
            button: job.control_button.clone(),
        });

        commands
    }
}

pub enum Event {
    TimeslotChanged,
}