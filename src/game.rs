use crate::assets::AssetId::*;
use crate::assets::{AssetId, Assets};
use crate::job::{JobInstance, JobParameters, JobArchetype, JobArchetypeInstance, JobArchetypeInstances};
use crate::skill::{SkillArchetype, SkillArchetypeInstances};
use macroquad::color::Color;
use macroquad::input::MouseButton;
use macroquad::math::Vec2;
use macroquad::prelude::Texture2D;
use std::collections::HashMap;

pub struct MouseInput {
    pub pressed: Vec<MouseButton>,
    pub released: Vec<MouseButton>,
    pub down: Vec<MouseButton>,
    pub position: (f32, f32),
    pub scroll_y: f32,
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

pub struct GameState {
    pub skill_archetype_instances: SkillArchetypeInstances,
    pub job_archetype_instances: JobArchetypeInstances,
    pub jobs: Vec<JobInstance>,
    pub time_slots: TimeSlots,
    pub performance_flags: PerformanceFlags,
    pub game_meta: GameMeta,
    pub inventory: Inventory,
    pub text_particles: Vec<TextParticle>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            skill_archetype_instances: SkillArchetypeInstances::new(),
            job_archetype_instances: JobArchetypeInstances::new(),
            jobs: vec![],
            time_slots: TimeSlots { total: 9, used: 0, },
            performance_flags: PerformanceFlags::new(),
            game_meta: GameMeta::new(),
            inventory: Inventory::new(),
            text_particles: vec![],
        }
    }

    pub fn add_job_instance(&mut self, job_type: JobArchetype) {
        self.jobs.push(
            JobInstance::new(JobParameters {
                instance_id: self.jobs.iter().map(|j| j.instance_id).max().unwrap_or(0) + 1,
                job_type: job_type.clone(),
            })
        );
    }

    // Step logic (tick + inputs)
    pub fn step(&mut self, intents: &[Intent], dt: f32) -> Vec<EffectWithSource>
    {
        let free_timeslots = self.time_slots.get_free();

        for intent in intents {
            match intent {
                Intent::ToggleJob(index) => {
                    if let Some(job) = self.jobs.get_mut(*index) {
                        job.toggle_running(free_timeslots);
                        self.performance_flags.timeslots_changed = true;
                    }
                }
                Intent::ToggleHyperMode(index) => {
                    // todo: implement hyper mode toggle
                },
                Intent::BuyTimeSlot => {
                    let upgrade_cost = self.time_slots.get_upgrade_cost();

                    if self.inventory.get_item_amount(&Item::Coin) >= upgrade_cost {
                        self.inventory.add_item(Item::Coin, -upgrade_cost);
                        self.time_slots.total += 1;
                        self.performance_flags.timeslots_changed = true;
                    }
                },
                Intent::SkipSeconds(seconds) => {
                    for _ in 0..*seconds {
                        // skip capturing effects because we don't want to draw millions of events
                        self.update_progress(1.0);
                    }
                }
            }
        }

        if self.performance_flags.timeslots_changed {
            self.time_slots.used = get_used_timeslots(&self.jobs);
        }

        let effects = self.update_progress(dt);

        effects
    }

    fn update_progress(&mut self, dt: f32) -> Vec<EffectWithSource>
    {
        let mut effects_with_source = vec![];

        for job in &mut self.jobs {
            if job.running {
                let effects = job.update_progress(&mut self.inventory, dt);

                for effect in effects {
                    // execute side effects
                    match &effect {
                        Effect::AddItem { item, amount } => {
                            self.inventory.add_item(*item, *amount);
                        }
                        Effect::IncrementActionsForSkill { skill_type } => {
                            self.skill_archetype_instances.get_skill_by_type_mut(skill_type).increment_actions();
                        }
                        Effect::IncrementActionsForJobType { job_type } => {
                            self.job_archetype_instances.get_archetype_mut(job_type).increment_actions();
                        }
                    }

                    // collect effects with source
                    effects_with_source.push(EffectWithSource::JobSource {
                        job: job.clone(),
                        effect: effect.clone(),
                    });
                }
            }
        }

        effects_with_source
    }
}

fn get_used_timeslots(jobs: &[JobInstance]) -> i32 {
    jobs.iter().filter(|j| j.running).map(|j| j.timeslot_cost).sum()
}

#[derive(Clone)]
pub enum Intent {
    ToggleJob(usize),
    BuyTimeSlot,
    SkipSeconds(i32),
    ToggleHyperMode(usize),
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

#[derive(Clone, PartialEq)]
pub struct Progress {
    pub value: f32, // Value between 0.0 and 1.0
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
    pub actions_until_level_up: i32,
}

#[derive(Clone, PartialEq)]
pub enum Effect {
    AddItem { item: Item, amount: i64 },
    IncrementActionsForSkill { skill_type: SkillArchetype },
    IncrementActionsForJobType { job_type: JobArchetype },
}

pub enum EffectWithSource {
    JobSource { job: JobInstance, effect: Effect },
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
    IronBar,
    Sandwich,
    Bread,
    Tree,
    Deer,
    ManaPotion,
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
            Item::IronBar => "Iron Bar".to_string(),
            Item::Sandwich => "Sandwich".to_string(),
            Item::Bread => "Bread".to_string(),
            Item::Tree => "Tree".to_string(),
            Item::Deer => "Deer".to_string(),
            Item::ManaPotion => "Mana Potion".to_string(),
        }
    }

    pub fn get_texture(&self, assets: &Assets) -> Texture2D {
        match self {
            Item::Wood => Wood.texture(assets),
            Item::Meat => MeatGame.texture(assets),
            Item::Coin => Coin.texture(assets),
            Item::Bread => Bread.texture(assets),
            Item::Herb => Herbs.texture(assets),
            Item::Sandwich => Sandwich.texture(assets),
            Item::Tree => Tree.texture(assets),
            Item::Deer => Deer.texture(assets),
            Item::ManaPotion => ManaPotion.texture(assets),
            _ => Wood.texture(assets),
        }
    }
}

pub struct Inventory {
    pub item_amounts: HashMap<Item, i64>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            item_amounts: HashMap::from([
                (Item::Coin, 0),
            ]),
        }
    }

    pub fn add_item(&mut self, item: Item, amount: i64) -> () {
        if let Some(count) = self.item_amounts.get_mut(&item) {
            *count += amount;
        } else {
            self.item_amounts.insert(item, amount);
        }
    }

    pub fn get_item_amount(&self, item: &Item) -> i64 {
        *self.item_amounts.get(item).unwrap_or(&0)
    }
}

pub struct TextParticle {
    pub text: String,
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub color: Color,
}

impl TextParticle {
    pub fn step(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.lifetime -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}