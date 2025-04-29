use macroquad::prelude::*;

mod my_lib;
use my_lib::*;

pub struct Job {
    pub name: String,
    pub progress: ProgressBar, // Progress for actions
    pub level_up_progress: ProgressBar, // Progress for leveling up
    pub production_rate: i32,
    pub level: i32,
    pub money_per_action: i32,
    pub action_duration: f32,
    pub time_accumulator: f32,
    pub running: bool,
    pub control_button: Button,
    pub actions_until_level_up: i32, // Remaining actions to level up
    pub actions_done: i32, // Tracks completed actions
}

impl Job {
    pub fn new(
        name: &str,
        x: f32,
        y: f32,
        production_rate: i32,
        level: i32,
        money_per_action: i32,
        action_duration: f32,
    ) -> Self {
        Self {
            name: name.to_string(),
            progress: ProgressBar::new(x, y, 300.0, 20.0, GRAY, GREEN),
            level_up_progress: ProgressBar::new(x, y + 30.0, 300.0, 20.0, GRAY, BLUE),
            production_rate,
            level,
            money_per_action,
            action_duration,
            time_accumulator: 0.0,
            running: false,
            control_button: Button::new(x + 320.0, y, 100.0, 30.0, WHITE, GRAY, "Start"),
            actions_until_level_up: 10, // Example: 10 actions to level up
            actions_done: 0,
        }
    }

    pub fn toggle_running(&mut self) {
        self.running = !self.running;
        self.control_button.label = if self.running { "Stop".to_string() } else { "Start".to_string() };
    }

    pub fn update_progress(&mut self, dt: f32) -> i32 {
        self.time_accumulator += dt;
        self.progress.set_progress(self.time_accumulator / self.action_duration);

        if self.time_accumulator >= self.action_duration {
            self.time_accumulator -= self.action_duration;
            self.actions_done += 1;
            self.level_up_progress.set_progress(self.actions_done as f32 / self.actions_until_level_up as f32);

            if self.actions_done >= self.actions_until_level_up {
                self.level_up();
            }

            return self.money_per_action * self.level;
        }

        0
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.money_per_action += 5; // Example: Increase money per action
        self.actions_done = 0;
        self.level_up_progress.reset();
    }

    pub fn dollars_per_second(&self) -> f32 {
        (self.money_per_action as f32 / self.action_duration) * self.level as f32
    }
}

pub struct GameState {
    pub jobs: Vec<Job>,
    pub total_money: i32, // Tracks total money earned
}

impl GameState {
    pub fn new() -> Self {
        Self {
            jobs: vec![
                Job::new("Burger", 10.0, 200.0, 1, 1, 10, 2.0),
                Job::new("Restaurant", 10.0, 250.0, 2, 1, 20, 3.0),
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

    for job in &state.jobs {
        // Card background
        commands.push(DrawCommand::Rectangle {
            x: job.progress.x - 10.0,
            y: job.progress.y - 80.0,
            width: 400.0,
            height: 200.0,
            color: DARKGRAY,
        });

        // Job name
        commands.push(DrawCommand::Text {
            content: format!("Job: {}", job.name),
            x: job.progress.x,
            y: job.progress.y - 60.0,
            font_size: 24.0,
            color: WHITE,
        });

        // Level and $ per action
        commands.push(DrawCommand::Text {
            content: format!("Level: {} | $/Action: {}", job.level, job.money_per_action),
            x: job.progress.x,
            y: job.progress.y - 40.0,
            font_size: 20.0,
            color: LIGHTGRAY,
        });

        // Seconds per action and $ per second
        commands.push(DrawCommand::Text {
            content: format!("Sec/Action: {:.2} | $/Sec: {:.2}", job.action_duration, job.dollars_per_second()),
            x: job.progress.x,
            y: job.progress.y - 20.0,
            font_size: 20.0,
            color: LIGHTGRAY,
        });

        // Actions until level up
        commands.push(DrawCommand::Text {
            content: format!("Actions to Level Up: {}", job.actions_until_level_up - job.actions_done),
            x: job.progress.x,
            y: job.progress.y,
            font_size: 20.0,
            color: LIGHTGRAY,
        });

        // Action progress bar
        commands.push(DrawCommand::ProgressBar {
            x: job.progress.x,
            y: job.progress.y + 20.0,
            width: job.progress.width,
            height: job.progress.height,
            progress: job.progress.progress.get(),
            background_color: GRAY,
            foreground_color: GREEN,
        });

        // Level-up progress bar
        commands.push(DrawCommand::ProgressBar {
            x: job.level_up_progress.x,
            y: job.level_up_progress.y + 40.0,
            width: job.level_up_progress.width,
            height: job.level_up_progress.height,
            progress: job.level_up_progress.progress.get(),
            background_color: GRAY,
            foreground_color: BLUE,
        });

        // Control button
        commands.push(DrawCommand::Button {
            button: job.control_button.clone(),
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
