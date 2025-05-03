use crate::draw::DrawCommand;
use crate::layout::{layout, JobLayout};
use crate::my_lib::{Job, JobBaseValues, JobParameters};
use crate::render::JobRenderer;
use macroquad::color::WHITE;
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::math::Vec2;
use macroquad::prelude::Texture2D;

pub struct Assets {
    pub wood_1: Texture2D,
    pub wood_2: Texture2D,
}

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
    pub fn step(&mut self, actions: &[Intent], dt: f32) {
        let free_timeslots = self.time_slots.get_free();

        for action in actions {
            match action {
                Intent::ToggleJob(index) => {
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

pub enum Intent {
    ToggleJob(usize),
}

pub struct UserInterface {
    pub global_offset: Vec2,
    pub last_mouse_position: Vec2,
    pub job_layouts: Vec<JobLayout>,
}

impl UserInterface {
    pub fn new(state: &GameState) -> Self {
        Self {
            last_mouse_position: Vec2::new(0.0, 0.0),
            global_offset: Vec2::new(0.0, 0.0),
            job_layouts: layout(state, Vec2::new(0.0, 0.0)),
        }
    }

    pub fn recreate(&mut self, state: &GameState, offset: Vec2) -> Self {
        Self {
            last_mouse_position: self.last_mouse_position,
            global_offset: offset,
            job_layouts: layout(state, offset),
        }
    }

    pub fn process_input(&mut self) -> Vec<Intent> {
        let mut actions = vec![];

        for layout in &self.job_layouts {
            if layout.toggle_button.is_clicked() {
                actions.push(Intent::ToggleJob(layout.job_index));
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.last_mouse_position = Vec2::from(mouse_position());
        }

        actions
    }

    pub fn render(&self, state: &GameState, assets: &Assets) -> Vec<DrawCommand> {
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
        for layout in &self.job_layouts {
            let job = &state.jobs[layout.job_index];
            commands.extend(job_renderer.render(assets, job, layout));
        }

        commands
    }
}