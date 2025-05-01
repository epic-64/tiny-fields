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

    pub fn is_clicked(&self) -> bool {
        let mouse = mouse_position();
        self.contains_point(mouse) && is_mouse_button_pressed(MouseButton::Left)
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
        self.rect.is_clicked()
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

pub struct JobBaseValues {
    pub money_per_action: i32,
    pub actions_until_level_up: i32,
}

pub struct Job {
    pub name: String,
    pub action_progress: Progress, // Progress for actions
    pub level_up_progress: Progress, // Progress for leveling up
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
            action_progress: Progress{value: 0.0},
            level_up_progress: Progress{value: 0.0},
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
        self.action_progress.set(self.time_accumulator / self.action_duration);

        if self.time_accumulator >= self.action_duration {
            self.time_accumulator -= self.action_duration;
            self.actions_done += 1;
            self.level_up_progress.set(self.actions_done as f32 / self.actions_to_level_up() as f32);

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

    pub fn actions_to_level_up(&self) -> i32 {
        let base_actions = 10;        // Base number of actions for level 1
        let growth_factor: f32 = 1.5; // Exponential growth factor
        (base_actions as f32 * growth_factor.powi(self.level - 1)) as i32
    }
}