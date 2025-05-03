use crate::my_lib::{Job, JobBaseValues, JobParameters};
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
            name: "Pine".to_string(),
            action_duration: 10.0,
            timeslot_cost: 1,
            base_values: JobBaseValues {
                money_per_action: 10,
                actions_until_level_up: 10,
            },
        }),

        Job::new(JobParameters {
            name: "Spruce".to_string(),
            action_duration: 15.0,
            timeslot_cost: 2,
            base_values: JobBaseValues {
                money_per_action: 80,
                actions_until_level_up: 10,
            },
        }),

        Job::new(JobParameters {
            name: "Rosewood".to_string(),
            action_duration: 20.0,
            timeslot_cost: 3,
            base_values: JobBaseValues {
                money_per_action: 250,
                actions_until_level_up: 10,
            },
        }),

        Job::new(JobParameters {
            name: "Oak".to_string(),
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