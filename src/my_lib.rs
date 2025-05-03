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

#[derive(Clone)]
pub struct JobBaseValues {
    pub money_per_action: i32,
    pub actions_until_level_up: i32,
}

#[derive(Clone)]
pub struct Job {
    pub name: String,
    pub action_progress: Progress,
    pub level_up_progress: Progress,
    pub level: i32,
    pub action_duration: f32,
    pub time_accumulator: f32,
    pub running: bool,
    pub actions_done: i32,
    pub timeslot_cost: i32,
    pub base_values: JobBaseValues,
}

pub struct JobParameters {
    pub name: String,
    pub action_duration: f32,
    pub timeslot_cost: i32,
    pub base_values: JobBaseValues,
}

impl Job {
    pub fn new(p: JobParameters) -> Self {
        Self {
            level: 1,
            running: false,
            action_progress: Progress{value: 0.0},
            level_up_progress: Progress{value: 0.0},
            time_accumulator: 0.0,
            actions_done: 0,

            name: p.name,
            timeslot_cost: p.timeslot_cost,
            base_values: p.base_values,
            action_duration: p.action_duration,
        }
    }

    pub fn toggle_running(&mut self, free_timeslots: i32) -> () {
        if self.running {
            self.running = false;
        } else if free_timeslots >= self.timeslot_cost {
            self.running = true;
        }
    }

    pub fn update_progress(&mut self, dt: f32) -> i64 {
        self.time_accumulator += dt;
        self.action_progress.set(self.time_accumulator / self.action_duration);

        if self.time_accumulator >= self.action_duration {
            self.time_accumulator -= self.action_duration;
            self.actions_done += 1;
            self.level_up_progress.set(
                self.actions_done as f32 / self.actions_to_level_up() as f32
            );

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

    pub fn dollars_per_action(&self) -> i64 {
        let base_money_per_action = self.base_values.money_per_action;
        let growth_factor: f32 = 1.3;

        (base_money_per_action as f32 * growth_factor.powi(self.level - 1)) as i64
    }

    pub fn actions_to_level_up(&self) -> i32 {
        let base_actions = self.base_values.actions_until_level_up;
        let growth_factor: f32 = 1.5;

        (base_actions as f32 * growth_factor.powi(self.level - 1)) as i32
    }
}