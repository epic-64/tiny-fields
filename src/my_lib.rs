use macroquad::math::i64;
use macroquad::prelude::*;
use crate::draw::DrawCommand;
use crate::layout::JobLayout;

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

pub struct JobBaseValues {
    pub money_per_action: i32,
    pub actions_until_level_up: i32,
}

pub struct Job {
    pub name: String,
    pub action_progress: ProgressBar, // Progress for actions
    pub level_up_progress: ProgressBar, // Progress for leveling up
    pub level: i32,
    pub action_duration: f32,
    pub time_accumulator: f32,
    pub running: bool,
    pub control_button: Button,
    pub actions_done: i32, // Tracks completed actions
    pub timeslot_cost: i32,
    pub base_values: JobBaseValues,
}

impl Job {
    pub fn new(
        name: &str,
        x: f32,
        y: f32,
        level: i32,
        action_duration: f32,
        timeslot_cost: i32,
        base_values: JobBaseValues,
    ) -> Self {
        Self {
            name: name.to_string(),
            action_progress: ProgressBar::new(x + 10.0, y + 140.0, 300.0, 20.0, GRAY, GREEN),
            level_up_progress: ProgressBar::new(x + 10.0, y + 170.0, 300.0, 20.0, GRAY, BLUE),
            level,
            action_duration,
            time_accumulator: 0.0,
            running: false,
            control_button: Button{
                rect: Rectangle::new(x + 180.0, y, 100.0, 30.0),
                color: Color{ r: 0.2, g: 0.5, b: 0.8, a: 1.0, },
                hover_color: Color{ r: 0.3, g: 0.6, b: 0.9, a: 1.0, },
                label: "Start".to_string(),
            },
            actions_done: 0,
            timeslot_cost,
            base_values,
        }
    }

    pub fn toggle_running(&mut self, free_timeslots: i32) -> () {
        if self.running {
            self.running = false;
            self.control_button.label = "Start".to_string();
        } else if free_timeslots >= self.timeslot_cost {
            self.running = true;
            self.control_button.label = "Stop".to_string();
        }
    }

    pub fn update_progress(&mut self, dt: f32) -> i64 {
        self.time_accumulator += dt;
        self.action_progress.set_progress(self.time_accumulator / self.action_duration);

        if self.time_accumulator >= self.action_duration {
            self.time_accumulator -= self.action_duration;
            self.actions_done += 1;
            self.level_up_progress.set_progress(self.actions_done as f32 / self.actions_to_level_up() as f32);

            if self.actions_done >= self.actions_to_level_up() {
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
        (self.base_values.money_per_action as f32 * self.dollars_multiplier()) as i64
    }

    pub fn dollars_per_second(&self) -> i64 {
        self.dollars_per_action() / self.action_duration as i64
    }

    pub fn actions_to_level_up(&self) -> i32 {
        let base_actions = 10;        // Base number of actions for level 1
        let growth_factor: f32 = 1.5; // Exponential growth factor
        (base_actions as f32 * growth_factor.powi(self.level - 1)) as i32
    }
}

pub struct JobRenderer {}

impl JobRenderer {
    const CARD_PADDING: f32 = 20.0;
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

    pub fn render(&self, job: &Job, layout: &JobLayout) -> Vec<DrawCommand> {
        let mut commands = vec![];

        // Card background
        commands.push(DrawCommand::Rectangle {
            x: layout.card_rect.x,
            y: layout.card_rect.y,
            width: layout.card_rect.width as f64,
            height: layout.card_rect.height as f64,
            color: Self::BACKGROUND_COLOR,
        });

        // Job name
        commands.push(DrawCommand::Text {
            content: format!("Job: {} ({})", job.name, job.level),
            x: layout.card_rect.x + Self::CARD_PADDING,
            y: layout.card_rect.y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE,
            font_size: Self::TEXT_FONT_SIZE_LARGE,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Info Line
        commands.push(DrawCommand::Text {
            content: format!("$: {} | $/s: {}", job.dollars_per_action(), job.dollars_per_second()),
            x: layout.card_rect.x + Self::CARD_PADDING,
            y: layout.card_rect.y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE + Self::CARD_SPACING,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_SECONDARY,
        });

        // Action progress bar
        commands.push(DrawCommand::ProgressBar {
            x: layout.action_bar_rect.x,
            y: layout.action_bar_rect.y,
            width: layout.action_bar_rect.width,
            height: layout.action_bar_rect.height,
            progress: job.action_progress.progress.get(),
            background_color: Self::PROGRESS_BAR_BACKGROUND,
            foreground_color: Self::PROGRESS_BAR_FOREGROUND_ACTION,
        });

        // Text inside the action progress bar
        commands.push(DrawCommand::Text {
            content: format!("{:.1} / {:.1}", job.time_accumulator, job.action_duration),
            x: layout.action_bar_rect.x + 10.0,
            y: layout.action_bar_rect.y + 15.0,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Level-up progress bar
        commands.push(DrawCommand::ProgressBar {
            x: layout.level_bar_rect.x,
            y: layout.level_bar_rect.y,
            width: layout.level_bar_rect.width,
            height: layout.level_bar_rect.height,
            progress: job.level_up_progress.progress.get(),
            background_color: Self::PROGRESS_BAR_BACKGROUND,
            foreground_color: Self::PROGRESS_BAR_FOREGROUND_LEVEL,
        });

        // Text inside the level-up progress bar
        commands.push(DrawCommand::Text {
            content: format!("Level Up: {} / {}", job.actions_done, job.actions_to_level_up()),
            x: layout.level_bar_rect.x + 10.0,
            y: layout.level_bar_rect.y + 15.0,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Control button using layout's button_rect
        commands.push(DrawCommand::Button {
            button: Button::new(
                layout.button_rect.x,
                layout.button_rect.y,
                layout.button_rect.width,
                layout.button_rect.height,
                job.control_button.color,
                job.control_button.hover_color,
                &job.control_button.label,
            ),
        });

        commands
    }
}