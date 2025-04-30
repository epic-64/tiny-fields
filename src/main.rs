use std::time::Instant;
use macroquad::prelude::*;

mod my_lib;
use my_lib::*;

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
                Job::new("Burger", 50.0, 50.0, 1, 2.0, 2, JobBaseValues {
                    money_per_action: 10,
                    actions_until_level_up: 5,
                }),
                Job::new("Pizza", 50.0, 50.0, 1, 5.0, 2, JobBaseValues {
                    money_per_action: 80,
                    actions_until_level_up: 10,
                }),
                Job::new("Sushi", 50.0, 50.0, 1, 10.0, 2, JobBaseValues {
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
fn step(state: &mut GameState, dt: f32) {
    let free_timeslots = state.time_slots.get_free();

    for job in &mut state.jobs {
        if job.control_button.is_clicked() {
            job.toggle_running(free_timeslots);
            state.performance_flags.timeslots_changed = true;
        }

        if job.running {
            state.total_money += job.update_progress(dt);
        }
    }

    // Recalculate `used_timeslots` after the mutable borrow ends
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
fn render(state: &GameState) -> Vec<DrawCommand> {
    let mut commands = vec![];

    // Display resources
    commands.push(DrawCommand::Text {
        content: format!("Money: ${}", state.total_money),
        x: 20.0,
        y: 20.0,
        font_size: 30.0,
        color: WHITE,
    });

    commands.push(DrawCommand::Text {
        content: format!("Free Timeslots: {}", state.time_slots.get_free()),
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

    // Add raw FPS to the draw commands
    commands.push(DrawCommand::Text {
        content: format!("Raw FPS: {:.2}", state.game_meta.raw_fps),
        x: 20.0,
        y: 140.0,
        font_size: 30.0,
        color: WHITE,
    });

    let mut y_offset = 200.0;
    let job_renderer = JobRenderer{};

    for job in &state.jobs {
        commands.extend(job_renderer.render(job, 50.0, y_offset, 400.0, 180.0));
        y_offset += 240.0; // Adjust spacing between cards
    }

    commands
}

// Main draw loop
#[macroquad::main("Tiny Fields")]
async fn main() {
    let mut state = GameState::new();

    loop {
        let frame_start = Instant::now(); // Start measuring raw frame time

        clear_background(ORANGE);

        let dt = get_frame_time();
        step(&mut state, dt);

        // Calculate raw FPS based on rendering and logic time
        let raw_frame_time = frame_start.elapsed().as_secs_f32();
        state.game_meta.raw_fps = 1.0 / raw_frame_time;

        // Calculate effective FPS
        state.game_meta.effective_fps = get_fps() as f32;

        let commands = render(&state);
        draw(&commands);

        next_frame().await;
    }
}