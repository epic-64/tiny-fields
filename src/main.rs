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

pub struct GameState {
    pub jobs: Vec<Job>,
    pub total_money: i32,
    pub time_slots: TimeSlots,
    pub performance_flags: PerformanceFlags,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            jobs: vec![
                Job::new("Burger", 50.0, 50.0, 1, 1, 10, 2.0, 2),
                Job::new("Restaurant", 50.0, 290.0, 2, 1, 20, 3.0, 3),
                Job::new("Car Wash", 50.0, 530.0, 3, 1, 30, 4.0, 4),
            ],
            total_money: 0,
            time_slots: TimeSlots { total: 3, used: 0, },
            performance_flags: PerformanceFlags { timeslots_changed: false, },
        }
    }

    pub fn free_timeslots(&self) -> i32 {
        self.time_slots.total - self.time_slots.used
    }

    pub fn update_progress(&mut self, dt: f32) {
        for job in &mut self.jobs {
            self.total_money += job.update_progress(dt);
        }
    }
}

// Step logic (tick + inputs)
fn step(state: &mut GameState, dt: f32) {
    let free_timeslots = state.free_timeslots(); // Calculate free timeslots before the loop

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
        content: format!("Free Timeslots: {}", state.free_timeslots()),
        x: 20.0,
        y: 60.0,
        font_size: 30.0,
        color: WHITE,
    });

    let mut y_offset = 100.0;

    for job in &state.jobs {
        let renderer = JobRenderer::new(50.0, y_offset, 400.0, 220.0);
        commands.extend(renderer.render(job));
        y_offset += 240.0; // Adjust spacing between cards
    }

    commands
}

// Main draw loop
#[macroquad::main("Tiny Fields")]
async fn main() {
    let mut state = GameState::new();

    loop {
        clear_background(ORANGE);

        let dt = get_frame_time();
        step(&mut state, dt);

        let commands = render(&state);
        draw(&commands);

        next_frame().await;
    }
}
