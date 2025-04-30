use std::time::Instant;
use macroquad::prelude::*;

mod my_lib;
mod layout;
mod draw;

use my_lib::*;
use crate::layout::{layout, JobLayout};
use crate::draw::{draw, DrawCommand};

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
fn step(state: &mut GameState, layouts: &[JobLayout], dt: f32) {
    let free_timeslots = state.time_slots.get_free();
    let mouse = mouse_position();

    for layout in layouts {
        let job = &mut state.jobs[layout.job_index];

        if layout.button_rect.contains_point(mouse) && is_mouse_button_pressed(MouseButton::Left) {
            job.toggle_running(free_timeslots);
            state.performance_flags.timeslots_changed = true;
        }

        if job.running {
            state.total_money += job.update_progress(dt);
        }
    }

    if state.performance_flags.timeslots_changed {
        state.time_slots.used = get_used_timeslots(&state.jobs);
    }
}

fn get_used_timeslots(jobs: &[Job]) -> i32 {
    jobs.iter().filter(|j| j.running).map(|j| j.timeslot_cost).sum()
}

pub struct GameMeta {
    pub effective_fps: f32,
    pub raw_fps: f32,
}

// Return a vector of draw commands. Pure function
fn render(state: &GameState, layout: &[JobLayout]) -> Vec<DrawCommand> {
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
    for layout in layout {
        let job = &state.jobs[layout.job_index];
        commands.extend(job_renderer.render(job, layout));
    }

    commands
}

#[macroquad::main("Tiny Fields")]
async fn main() {
    let mut state = GameState::new();

    loop {
        let frame_start = Instant::now();
        let dt = get_frame_time();

        // Create layout, so we can use it in step and render
        let job_layouts = layout(&state);

        // Update game state
        step(&mut state, &job_layouts, dt);

        // Compile list of draw commands
        let commands = render(&state, &job_layouts);

        // Draw the game
        clear_background(ORANGE);
        draw(&commands);

        // Keep track of FPS
        state.game_meta.raw_fps = 1.0 / frame_start.elapsed().as_secs_f32();
        state.game_meta.effective_fps = get_fps() as f32;

        next_frame().await;
    }
}
