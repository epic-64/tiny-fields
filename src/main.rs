use macroquad::prelude::*;

mod my_lib;
use my_lib::*;

pub struct Job {
    pub name: String,
    pub progress: ProgressBar,
    pub resource: i32,
    pub resource_name: String,
    pub production_rate: i32,
    pub level: i32, // Job level
    pub money_per_action: i32, // Money produced per action
    pub action_duration: f32, // Seconds to complete one action
    pub time_accumulator: f32, // Tracks time for progress
}

impl Job {
    pub fn new(
        name: &str,
        x: f32,
        y: f32,
        resource_name: &str,
        production_rate: i32,
        level: i32,
        money_per_action: i32,
        action_duration: f32,
    ) -> Self {
        Self {
            name: name.to_string(),
            progress: ProgressBar::new(x, y, 300.0, 20.0, GRAY, GREEN),
            resource: 0,
            resource_name: resource_name.to_string(),
            production_rate,
            level,
            money_per_action,
            action_duration,
            time_accumulator: 0.0,
        }
    }

    pub fn tick(&mut self) {
        self.resource += self.production_rate * self.level;
    }

    pub fn update_progress(&mut self, dt: f32) -> i32 {
        self.time_accumulator += dt;
        self.progress.set_progress(self.time_accumulator / self.action_duration);

        if self.time_accumulator >= self.action_duration {
            self.time_accumulator -= self.action_duration;
            self.tick();
            return self.money_per_action * self.level; // Return money earned
        }

        0 // No money earned yet
    }
}

pub struct GameState {
    pub jobs: Vec<Job>,
    pub total_money: i32, // Tracks total money earned
    pub build_button: Button,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            jobs: vec![
                Job::new("Burger", 10.0, 200.0, "Burgers", 1, 1, 10, 2.0),
                Job::new("Restaurant", 10.0, 250.0, "Meals", 2, 1, 20, 3.0),
            ],
            total_money: 0,
            build_button: Button::new(10.0, 120.0, 240.0, 40.0, WHITE, GRAY, "Build Restaurant"),
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
    state.update_progress(dt);

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
