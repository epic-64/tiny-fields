use std::time::Instant;
use macroquad::prelude::*;

mod my_lib;
mod layout;

use my_lib::*;
use layout::*;

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

fn layout(state: &GameState) -> Vec<JobLayout> {
    let mut layouts = vec![];
    let mut y_offset = 200.0;

    for (i, _job) in state.jobs.iter().enumerate() {
        let card_x = 50.0;
        let card_y = y_offset;
        let card_w = 400.0;
        let card_h = 180.0;

        layouts.push(JobLayout {
            job_index: i,
            card_rect: Rectangle::new(card_x, card_y, card_w, card_h),
            button_rect: Rectangle::new(card_x + 180.0, card_y, 100.0, 30.0),
            action_bar_rect: Rectangle::new(card_x + 10.0, card_y + 140.0, 300.0, 20.0),
            level_bar_rect: Rectangle::new(card_x + 10.0, card_y + 170.0, 300.0, 20.0),
        });

        y_offset += 240.0;
    }

    layouts
}

// Step logic (tick + inputs)
fn step(state: &mut GameState, layout: &[JobLayout], dt: f32) {
    let free_timeslots = state.time_slots.get_free();
    let mouse = mouse_position();

    for layout in layout {
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

    commands.push(DrawCommand::Text {
        content: format!("Free Timeslots: {}", state.time_slots.get_free()),
        x: 20.0,
        y: 60.0,
        font_size: 30.0,
        color: WHITE,
    });

    commands.push(DrawCommand::Text {
        content: format!("FPS: {}", state.game_meta.effective_fps),
        x: 20.0,
        y: 100.0,
        font_size: 30.0,
        color: WHITE,
    });

    commands.push(DrawCommand::Text {
        content: format!("Raw FPS: {:.2}", state.game_meta.raw_fps),
        x: 20.0,
        y: 140.0,
        font_size: 30.0,
        color: WHITE,
    });

    for layout in layout {
        let job = &state.jobs[layout.job_index];

        // Background
        commands.push(DrawCommand::Rectangle {
            x: layout.card_rect.x,
            y: layout.card_rect.y,
            width: layout.card_rect.width as f64,
            height: layout.card_rect.height as f64,
            color: DARKGRAY,
        });

        // Job name
        commands.push(DrawCommand::Text {
            content: format!("Job: {} ({})", job.name, job.level),
            x: layout.card_rect.x + 20.0,
            y: layout.card_rect.y + 44.0,
            font_size: 24.0,
            color: WHITE,
        });

        // Money info
        commands.push(DrawCommand::Text {
            content: format!("$: {} | $/s: {}", job.dollars_per_action(), job.dollars_per_second()),
            x: layout.card_rect.x + 20.0,
            y: layout.card_rect.y + 74.0,
            font_size: 20.0,
            color: LIGHTGRAY,
        });

        // Action progress bar
        commands.push(DrawCommand::ProgressBar {
            x: layout.action_bar_rect.x,
            y: layout.action_bar_rect.y,
            width: layout.action_bar_rect.width,
            height: layout.action_bar_rect.height,
            progress: job.action_progress.progress.get(),
            background_color: GRAY,
            foreground_color: GREEN,
        });

        // Level-up progress bar
        commands.push(DrawCommand::ProgressBar {
            x: layout.level_bar_rect.x,
            y: layout.level_bar_rect.y,
            width: layout.level_bar_rect.width,
            height: layout.level_bar_rect.height,
            progress: job.level_up_progress.progress.get(),
            background_color: GRAY,
            foreground_color: BLUE,
        });

        // Button
        commands.push(DrawCommand::Button {
            button: Button::new(
                layout.button_rect.x,
                layout.button_rect.y,
                layout.button_rect.width,
                layout.button_rect.height,
                WHITE,
                GRAY,
                if job.running { "Stop" } else { "Start" },
            ),
        });
    }

    commands
}


// Main draw loop
#[macroquad::main("Tiny Fields")]
async fn main() {
    let mut state = GameState::new();

    loop {
        let frame_start = Instant::now();

        clear_background(ORANGE);
        let dt = get_frame_time();

        let job_layouts = layout(&state);           // NEW
        step(&mut state, &job_layouts, dt);         // UPDATED

        let commands = render(&state, &job_layouts); // UPDATED
        draw(&commands);

        state.game_meta.raw_fps = 1.0 / frame_start.elapsed().as_secs_f32();
        state.game_meta.effective_fps = get_fps() as f32;

        next_frame().await;
    }
}
