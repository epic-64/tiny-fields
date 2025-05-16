use std::collections::HashMap;
use macroquad::input::MouseButton;
use macroquad::prelude::Texture2D;
use macroquad::text::Font;

pub struct MouseInput {
    pub pressed: Vec<MouseButton>,
    pub released: Vec<MouseButton>,
    pub down: Vec<MouseButton>,
    pub position: (f32, f32),
    pub scroll_y: f32,
}

pub struct Textures {
    pub wood_1: Texture2D,
    pub wood_2: Texture2D,
    pub mining_1: Texture2D,
    pub mining_2: Texture2D,
}

pub struct Fonts {
    pub main: Option<Font>
}

pub struct Assets {
    pub fonts: Fonts,
    pub textures: Textures,
}

pub struct PerformanceFlags {
    pub timeslots_changed: bool,
}

impl PerformanceFlags {
    pub fn new() -> Self {
        Self { timeslots_changed: false }
    }
}

pub struct TimeSlots {
    pub total: i32,
    pub used: i32,
}

impl TimeSlots {
    pub fn get_free(&self) -> i32 {
        self.total - self.used
    }
    pub fn get_upgrade_cost(&self) -> i64 { 10_i64.pow(self.total as u32 - 1) }
}

pub struct GameMeta {
    pub effective_fps: f64,
    pub raw_fps: f64,
    pub frame_time: f64,
}

impl GameMeta {
    pub fn new() -> Self {
        Self {
            effective_fps: 0.0,
            raw_fps: 0.0,
            frame_time: 0.0,
        }
    }
}

fn define_jobs() -> Vec<Job> {
    let duration = 10.0;

    vec![
        Job::new(JobParameters {
            job_type: JobType::Woodcutting,
            name: "Woodcutting".to_string(),
            action_duration: duration,
            timeslot_cost: 1,
            base_values: JobBaseValues {
                money_per_action: 3,
                actions_until_level_up: 10,
            },
            completion_effect: Effect::AddItem {
                item: Item::Wood,
                amount: 1,
            },
        }),

        Job::new(JobParameters {
            job_type: JobType::Mining,
            name: "Mining".to_string(),
            action_duration: duration,
            timeslot_cost: 1,
            base_values: JobBaseValues {
                money_per_action: 30,
                actions_until_level_up: 10,
            },
            completion_effect: Effect::AddItem {
                item: Item::Iron,
                amount: 1,
            },
        }),

        Job::new(JobParameters {
            job_type: JobType::Herbalism,
            name: "Herbalism".to_string(),
            action_duration: duration,
            timeslot_cost: 1,
            base_values: JobBaseValues {
                money_per_action: 100,
                actions_until_level_up: 10,
            },
            completion_effect: Effect::AddItem {
                item: Item::Herb,
                amount: 1,
            },
        }),

        Job::new(JobParameters {
            job_type: JobType::Hunting,
            name: "Hunting".to_string(),
            action_duration: duration,
            timeslot_cost: 1,
            base_values: JobBaseValues {
                money_per_action: 500,
                actions_until_level_up: 10,
            },
            completion_effect: Effect::AddItem {
                item: Item::Meat,
                amount: 1,
            },
        }),
        Job::new(JobParameters {
            job_type: JobType::Foraging,
            name: "Foraging".to_string(),
            action_duration: duration,
            timeslot_cost: 1,
            base_values: JobBaseValues {
                money_per_action: 500,
                actions_until_level_up: 10,
            },
            completion_effect: Effect::AddItem {
                item: Item::Berry,
                amount: 1,
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
    pub inventory: Inventory,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            jobs: define_jobs(),
            total_money: 0,
            time_slots: TimeSlots { total: 3, used: 0, },
            performance_flags: PerformanceFlags::new(),
            game_meta: GameMeta::new(),
            inventory: Inventory::new(),
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
                Intent::BuyTimeSlot => {
                    let upgrade_cost = self.time_slots.get_upgrade_cost();

                    if self.total_money >= upgrade_cost {
                        self.total_money -= upgrade_cost;
                        self.time_slots.total += 1;
                        self.performance_flags.timeslots_changed = true;
                    }
                },
                Intent::SkipSeconds(seconds) => {
                    for _ in 0..*seconds {
                        self.update_progress(1.0);
                    }
                }
            }
        }

        self.update_progress(dt);

        if self.performance_flags.timeslots_changed {
            self.time_slots.used = get_used_timeslots(&self.jobs);
        }
    }

    fn update_progress(&mut self, dt: f32) -> () {
        let mut effects = vec![];

        for job in &mut self.jobs {
            if job.running {
                if let Some(effect) = job.update_progress(dt) {
                    effects.push(effect);
                }
            }
        }

        for effect in effects {
            match effect {
                Effect::AddItem { item, amount } => {
                    self.inventory.add_item(item, amount);
                }
            }
        }
    }
}

fn get_used_timeslots(jobs: &[Job]) -> i32 {
    jobs.iter().filter(|j| j.running).map(|j| j.timeslot_cost).sum()
}

#[derive(Clone)]
pub enum Intent {
    ToggleJob(usize),
    BuyTimeSlot,
    SkipSeconds(i32),
}

#[derive(Clone)]
pub struct UiRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl UiRect {
    pub fn contains_point(&self, point: (f32, f32)) -> bool {
        point.0 >= self.x && point.0 <= self.x + self.w &&
            point.1 >= self.y && point.1 <= self.y + self.h
    }

    pub fn is_hovered(&self, mouse_input: &MouseInput) -> bool {
        self.contains_point(mouse_input.position)
    }

    pub fn is_clicked(&self, mouse_input: &MouseInput) -> bool {
        self.is_hovered(mouse_input) && mouse_input.pressed.contains(&MouseButton::Left)
    }
}

#[derive(Clone)]
pub struct Progress {
    value: f32, // Value between 0.0 and 1.0
}

impl Progress {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    pub fn set(&mut self, value: f32) {
        self.value = value.clamp(0.0, 1.0);
    }

    pub fn get(&self) -> f32 {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
    }
}

#[derive(Clone)]
pub struct JobBaseValues {
    pub money_per_action: i32,
    pub actions_until_level_up: i32,
}

#[derive(Clone)]
pub enum Effect {
    AddItem { item: Item, amount: i64 },
}

#[derive(Clone)]
pub enum JobType {
    Woodcutting,
    Mining,
    Herbalism,
    Hunting,
    Foraging,
    Woodworking,
    Smithing,
    Cooking,
    Alchemy,
    Selling,
}

impl JobType {
    pub fn get_images(&self, assets: &Assets) -> (Texture2D, Texture2D) {
        let textures = &assets.textures;

        match self {
            JobType::Woodcutting => (textures.wood_1.clone(), textures.wood_2.clone()),
            JobType::Mining => (textures.mining_1.clone(), textures.mining_2.clone()),
            _ => (textures.wood_1.clone(), textures.wood_2.clone()),
        }
    }
}

#[derive(Clone)]
pub struct Job {
    pub job_type: JobType,
    pub name: String,
    pub action_progress: Progress,
    pub level_up_progress: Progress,
    pub level: i32,
    pub action_duration: f32,
    pub time_accumulator: f32,
    pub running: bool,
    pub actions_done: i32,
    pub timeslot_cost: i32,
    pub base_values: JobBaseValues,
    pub completion_effect: Effect,
}

pub struct JobParameters {
    pub job_type: JobType,
    pub name: String,
    pub action_duration: f32,
    pub timeslot_cost: i32,
    pub base_values: JobBaseValues,
    pub completion_effect: Effect,
}

impl Job {
    pub fn new(p: JobParameters) -> Self {
        if p.action_duration % 2.0 != 0.0 {
            panic!("action_duration must be divisible by 2.0");
        }

        Self {
            level: 1,
            running: false,
            action_progress: Progress{value: 0.0},
            level_up_progress: Progress{value: 0.0},
            time_accumulator: 0.0,
            actions_done: 0,
            job_type: p.job_type,
            name: p.name,
            timeslot_cost: p.timeslot_cost,
            base_values: p.base_values,
            action_duration: p.action_duration,
            completion_effect: p.completion_effect,
        }
    }

    pub fn toggle_running(&mut self, free_timeslots: i32) -> () {
        if self.running {
            self.running = false;
        } else if free_timeslots >= self.timeslot_cost {
            self.running = true;
        }
    }

    pub fn update_progress(&mut self, dt: f32) -> Option<Effect> {
        self.time_accumulator += dt;
        self.action_progress.set(self.time_accumulator / self.action_duration);

        if self.time_accumulator >= self.action_duration {
            self.time_accumulator -= self.action_duration;
            self.actions_done += 1;

            // update level up progress bar
            self.level_up_progress.set(
                self.actions_done as f32 / self.actions_to_level_up() as f32
            );

            // level up if enough actions done
            if self.actions_done >= self.actions_to_level_up() {
                self.level_up();
            }

            Some(self.completion_effect.clone())
        } else {
            None
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.actions_done = 0;
        self.level_up_progress.reset();
    }

    pub fn money_per_action(&self) -> i64 {
        let base_money_per_action = self.base_values.money_per_action;
        let growth_factor: f32 = 1.3;

        (base_money_per_action as f32 * growth_factor.powi(self.level - 1)) as i64
    }

    pub fn actions_to_level_up(&self) -> i32 {
        let base_actions = self.base_values.actions_until_level_up;
        let growth_factor: f32 = 1.5;

        (base_actions as f32 * growth_factor.powi(self.level - 1)) as i32
    }
}

pub fn pretty_number(num: i64) -> String {
    let (num, suffix) = match num {
        n if n >= 1_000_000_000 => (n as f64 / 1_000_000_000.0, "b"),
        n if n >= 1_000_000 => (n as f64 / 1_000_000.0, "m"),
        n if n >= 10_000 => (n as f64 / 1_000.0, "k"),
        _ => return num.to_string(),
    };

    format!("{:.2}{suffix}", num)
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Item {
    Coin,
    Wood,
    Iron,
    Herb,
    Meat,
    Berry,
}

impl Item {
    pub fn to_string(&self) -> String {
        match self {
            Item::Coin => "Coin".to_string(),
            Item::Wood => "Wood".to_string(),
            Item::Iron => "Iron".to_string(),
            Item::Herb => "Herb".to_string(),
            Item::Meat => "Meat".to_string(),
            Item::Berry => "Berry".to_string(),
        }
    }
}

pub struct Inventory {
    pub items: HashMap<Item, i32>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::from([
                (Item::Coin, 0),
            ]),
        }
    }

    pub fn add_item(&mut self, item: Item, amount: i64) -> () {
        if let Some(count) = self.items.get_mut(&item) {
            *count += amount as i32;
        } else {
            self.items.insert(item, amount as i32);
        }
    }
}