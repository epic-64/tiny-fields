use std::time::Instant;
use macroquad::prelude::*;

mod my_lib;
mod layout;
mod draw;
mod render;

use my_lib::*;
use crate::layout::{layout, JobLayout};
use crate::draw::{draw, DrawCommand};
use crate::render::{JobRenderer};

pub struct PerformanceFlags {
    pub timeslots_changed: bool,
}

pub struct TimeSlots {
    pub total: i32,
    pub used: i32,
}

impl TimeSlots {
    pub fn get_free(&self) -> i32 {
        self.total - self.used
    }
}

pub struct GameMeta {
    pub effective_fps: f32,
    pub raw_fps: f32,
}

pub struct GameState {
    pub jobs: Vec<Job>,
    pub total_money: i64,
    pub time_slots: TimeSlots,
    pub performance_flags: PerformanceFlags,
    pub game_meta: GameMeta,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            jobs: vec![
                Job::new("Burger", 50.0, 50.0, 1, 2.0, 1, JobBaseValues {
                    money_per_action: 10,
                    actions_until_level_up: 5,
                }),
                Job::new("Pizza", 50.0, 50.0, 1, 5.0, 2, JobBaseValues {
                    money_per_action: 80,
                    actions_until_level_up: 10,
                }),
                Job::new("Sushi", 50.0, 50.0, 1, 10.0, 3, JobBaseValues {
                    money_per_action: 250,
                    actions_until_level_up: 15,
                }),
            ],
            total_money: 0,
            time_slots: TimeSlots { total: 3, used: 0, },
            performance_flags: PerformanceFlags { timeslots_changed: false, },
            game_meta: GameMeta { effective_fps: 0.0, raw_fps: 0.0, },
        }
    }

    pub fn update_progress(&mut self, dt: f32) {
        for job in &mut self.jobs {
            self.total_money += job.update_progress(dt);
        }
    }
}

// Step logic (tick + inputs)
fn step(state: &mut GameState, actions: &[Action], dt: f32) {
    let free_timeslots = state.time_slots.get_free();

    for action in actions {
        match action {
            Action::ToggleJob(index) => {
                if let Some(job) = state.jobs.get_mut(*index) {
                    job.toggle_running(free_timeslots);
                    state.performance_flags.timeslots_changed = true;
                }
            }
        }
    }

    for job in &mut state.jobs {
        if job.running {
            job.update_progress(dt);
        }
    }

    if state.performance_flags.timeslots_changed {
        state.time_slots.used = get_used_timeslots(&state.jobs);
    }
}

fn get_used_timeslots(jobs: &[Job]) -> i32 {
    jobs.iter().filter(|j| j.running).map(|j| j.timeslot_cost).sum()
}

enum Action {
    ToggleJob(usize),
}

fn process_input(layouts: &[JobLayout]) -> Vec<Action> {
    let mouse = mouse_position();

    let mut actions = vec![];
    for layout in layouts {
        if layout.button_rect.contains_point(mouse) && is_mouse_button_pressed(MouseButton::Left) {
            actions.push(Action::ToggleJob(layout.job_index));
        }
    }

    actions
}

struct UserInterface {
    layouts: Vec<JobLayout>,
}

impl UserInterface {
    fn new(state: &GameState) -> Self {
        Self {
            layouts: layout(state),
        }
    }

    fn process_input(&self) -> Vec<Action> {
        let mut actions = vec![];

        for layout in &self.layouts {
            if layout.button_rect.is_clicked() {
                actions.push(Action::ToggleJob(layout.job_index));
            }
        }

        actions
    }

    fn render(&self, state: &GameState) -> Vec<DrawCommand> {
        let mut commands = vec![];

        // Display top-level info
        commands.push(DrawCommand::Text {
            content: format!("Money: ${}", state.total_money),
            x: 20.0,
            y: 20.0,
            font_size: 30.0,
            color: WHITE,
        });

        // Display timeslots
        commands.push(DrawCommand::Text {
            content: format!("Timeslots: {} / {}", state.time_slots.get_free(), state.time_slots.total),
            x: 20.0,
            y: 60.0,
            font_size: 30.0,
            color: WHITE,
        });

        // Display FPS
        commands.push(DrawCommand::Text {
            content: format!("FPS: {}", state.game_meta.effective_fps),
            x: 20.0,
            y: 100.0,
            font_size: 30.0,
            color: WHITE,
        });

        // Display raw FPS
        commands.push(DrawCommand::Text {
            content: format!("Raw FPS: {:.2}", state.game_meta.raw_fps),
            x: 20.0,
            y: 140.0,
            font_size: 30.0,
            color: WHITE,
        });

        // Use JobRenderer for each job
        let job_renderer = JobRenderer {};
        for layout in &self.layouts {
            let job = &state.jobs[layout.job_index];
            commands.extend(job_renderer.render(job, layout));
        }

        commands
    }
}

#[macroquad::main("Tiny Fields")]
async fn main() {
    let mut state = GameState::new();
    let mut ui = UserInterface::new(&state);

    loop {
        let frame_start = Instant::now();
        let dt = get_frame_time();

        // Process input
        let actions = ui.process_input();

        // Update game state
        step(&mut state, &actions, dt);

        // Compile list of draw commands
        let commands = ui.render(&state);

        // Draw the game
        clear_background(ORANGE);
        draw(&commands);

        // Keep track of FPS
        state.game_meta.raw_fps = 1.0 / frame_start.elapsed().as_secs_f32();
        state.game_meta.effective_fps = get_fps() as f32;

        next_frame().await;
    }
}
