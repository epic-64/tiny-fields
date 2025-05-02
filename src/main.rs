use std::time::Instant;
use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets::{self, Group}, Drag, DrawList, Ui};
use macroquad::audio::{load_sound, play_sound, play_sound_once, PlaySoundParams};

mod my_lib;
mod layout;
mod draw;
mod render;

use crate::my_lib::*;
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

fn define_jobs() -> Vec<Job> {
    vec![
        Job::new(JobParameters {
            name: "Burger".to_string(),
            action_duration: 10.0,
            timeslot_cost: 1,
            base_values: JobBaseValues {
                money_per_action: 10,
                actions_until_level_up: 10,
            },
        }),

        Job::new(JobParameters {
            name: "Pizza".to_string(),
            action_duration: 15.0,
            timeslot_cost: 2,
            base_values: JobBaseValues {
                money_per_action: 80,
                actions_until_level_up: 10,
            },
        }),

        Job::new(JobParameters {
            name: "Sushi".to_string(),
            action_duration: 20.0,
            timeslot_cost: 3,
            base_values: JobBaseValues {
                money_per_action: 250,
                actions_until_level_up: 10,
            },
        }),

        Job::new(JobParameters {
            name: "Tacos".to_string(),
            action_duration: 25.0,
            timeslot_cost: 3,
            base_values: JobBaseValues {
                money_per_action: 500,
                actions_until_level_up: 10,
            },
        }),
    ]
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
            jobs: define_jobs(),
            total_money: 0,
            time_slots: TimeSlots { total: 3, used: 0, },
            performance_flags: PerformanceFlags { timeslots_changed: false, },
            game_meta: GameMeta { effective_fps: 0.0, raw_fps: 0.0, },
        }
    }

    // Step logic (tick + inputs)
    fn step(&mut self, actions: &[Action], dt: f32) {
        let free_timeslots = self.time_slots.get_free();

        for action in actions {
            match action {
                Action::ToggleJob(index) => {
                    if let Some(job) = self.jobs.get_mut(*index) {
                        job.toggle_running(free_timeslots);
                        self.performance_flags.timeslots_changed = true;
                    }
                }
            }
        }

        self.update_progress(dt);

        if self.performance_flags.timeslots_changed {
            self.time_slots.used = get_used_timeslots(&self.jobs);
        }
    }

    fn update_progress(&mut self, dt: f32) {
        for job in &mut self.jobs {
            if job.running {
                self.total_money += job.update_progress(dt);
            }
        }
    }
}

fn get_used_timeslots(jobs: &[Job]) -> i32 {
    jobs.iter().filter(|j| j.running).map(|j| j.timeslot_cost).sum()
}

enum Action {
    ToggleJob(usize),
}

struct UserInterface {
    global_offset: Vec2,
    layouts: Vec<JobLayout>,
    last_mouse_position: Vec2,
}

impl UserInterface {
    fn new(state: &GameState) -> Self {
        Self {
            last_mouse_position: Vec2::new(0.0, 0.0),
            global_offset: Vec2::new(0.0, 0.0),
            layouts: layout(state),
        }
    }

    fn process_input(&mut self) -> Vec<Action> {
        let mut actions = vec![];

        for layout in &self.layouts {
            if layout.button_rect.is_clicked() {
                actions.push(Action::ToggleJob(layout.job_index));
            }
        }

        // move global offset with right-click drag
        if is_mouse_button_down(MouseButton::Right) {
            self.global_offset = Vec2::from(mouse_position()) - self.last_mouse_position;
        }

        self.last_mouse_position = Vec2::from(mouse_position());

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

fn my_ui(state: &mut GameState) -> Vec<Action> {
    let mut actions = vec![];

    widgets::Window::new(hash!(), vec2(480., 200.), vec2(340., 360.))
        .label("Shop")
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            for job in &state.jobs {
                Group::new(hash!("shop", &job.name), Vec2::new(300., 100.)).ui(ui, |ui| {
                    ui.label(Vec2::new(10., 10.), &format!("Job: {}", job.name));
                    ui.label(Vec2::new(10., 30.), &format!("Level: {}", job.level));
                    ui.label(Vec2::new(10., 50.), &format!("Money per action: {}", job.base_values.money_per_action));
                    ui.label(Vec2::new(10., 70.), &format!("Actions until level up: {}", job.base_values.actions_until_level_up));

                    let label = if job.running { "Stop" } else { "Start" };
                    if ui.button(Vec2::new(240., 10.), label) {
                        let job_index = state.jobs.iter().position(|j| j.name == job.name).unwrap();
                        actions.push(Action::ToggleJob(job_index));
                    }
                });
            }
        });

    actions
}


#[macroquad::main("Tiny Fields")]
async fn main() {
    request_new_screen_size(1600.0, 900.0);

    let mut state = GameState::new();
    let mut ui = UserInterface::new(&state);

    loop {
        let frame_start = Instant::now();
        let dt = get_frame_time();

        clear_background(ORANGE);
        let mut actions = ui.process_input();
        actions.extend(my_ui(&mut state));

        // Update game state
        state.step(&actions, dt);

        // Compile list of draw commands
        let commands = ui.render(&state);

        // Draw the game
        draw(&commands);

        // Keep track of FPS
        state.game_meta.raw_fps = 1.0 / frame_start.elapsed().as_secs_f32();
        state.game_meta.effective_fps = get_fps() as f32;

        next_frame().await;
    }
}