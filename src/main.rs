use macroquad::prelude::*;

mod my_lib;
use my_lib::*;

pub struct Job {
    pub name: String,
    pub progress: ProgressBar,
    pub resource: i32,
    pub resource_name: String,
    pub production_rate: i32,
}

impl Job {
    pub fn new(name: &str, x: f32, y: f32, resource_name: &str, production_rate: i32) -> Self {
        Self {
            name: name.to_string(),
            progress: ProgressBar::new(x, y, 300.0, 20.0, GRAY, GREEN),
            resource: 0,
            resource_name: resource_name.to_string(),
            production_rate,
        }
    }

    pub fn tick(&mut self) {
        self.resource += self.production_rate;
    }

    pub fn update_progress(&mut self, dt: f32) {
        self.progress.set_progress(dt);
    }
}

pub struct GameState {
    pub jobs: Vec<Job>,
    pub time_accumulator: f32,
    pub build_button: Button,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            jobs: vec![
                Job::new("Burger", 10.0, 200.0, "Burgers", 1),
                Job::new("Restaurant", 10.0, 250.0, "Meals", 2),
            ],
            time_accumulator: 0.0,
            build_button: Button::new(10.0, 120.0, 240.0, 40.0, WHITE, GRAY, "Build Restaurant"),
        }
    }

    pub fn tick(&mut self) {
        for job in &mut self.jobs {
            job.tick();
        }
    }

    pub fn update_progress(&mut self, dt: f32) {
        for job in &mut self.jobs {
            job.update_progress(dt);
        }
    }
}

// Step logic (tick + inputs)
fn step(state: &mut GameState, dt: f32) {
    state.time_accumulator += dt;
    state.update_progress(dt);

    if state.time_accumulator >= 1.0 {
        state.tick();
        state.time_accumulator -= 1.0;
    }

    if state.build_button.is_clicked() {
        // Add logic for building restaurants or other jobs
    }
}

// Return a vector of draw commands. Pure function
fn render(state: &GameState) -> Vec<DrawCommand> {
    let mut commands = vec![
        DrawCommand::Button {
            button: state.build_button.clone(),
        },
    ];

    for job in &state.jobs {
        commands.push(DrawCommand::Text {
            content: format!("{}: {}", job.resource_name, job.resource),
            x: 20.0,
            y: job.progress.y - 30.0,
            font_size: 30.0,
            color: WHITE,
        });

        commands.push(DrawCommand::ProgressBar {
            x: job.progress.x,
            y: job.progress.y,
            width: job.progress.width,
            height: job.progress.height,
            progress: job.progress.progress.get(),
            background_color: job.progress.background_color,
            foreground_color: job.progress.foreground_color,
        });
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
