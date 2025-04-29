use macroquad::prelude::*;

mod my_lib;
use my_lib::*;

pub struct GameState {
    pub jobs: Vec<Job>,
    pub total_money: i32, // Tracks total money earned
}

impl GameState {
    pub fn new() -> Self {
        Self {
            jobs: vec![
                Job::new("Burger", 50.0, 50.0, 1, 1, 10, 2.0),
                Job::new("Restaurant", 50.0, 290.0, 2, 1, 20, 3.0),
            ],
            total_money: 0,
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
    for job in &mut state.jobs {
        if job.control_button.is_clicked() {
            job.toggle_running();
        }

        if job.running {
            state.total_money += job.update_progress(dt);
        }
    }
}

// Return a vector of draw commands. Pure function
fn render(state: &GameState) -> Vec<DrawCommand> {
    let mut commands = vec![];
    let mut y_offset = 50.0;

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
        clear_background(BLACK);

        let dt = get_frame_time();
        step(&mut state, dt);

        let commands = render(&state);
        draw(&commands);

        next_frame().await;
    }
}
