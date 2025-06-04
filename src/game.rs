use crate::assets::AssetId::*;
use crate::assets::Assets;
use crate::draw::UiElement;
use crate::job::{JobArchetype, JobArchetypeInstances, JobInstance};
use crate::job_slot::{JobSlot, JobSlotState};
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
    pub game_meta: GameMeta,
    pub inventory: Inventory,
    pub text_particles: Vec<TextParticle>,
    pub job_slots: Vec<JobSlot>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            skill_archetype_instances: SkillArchetypeInstances::new(),
            job_archetype_instances: JobArchetypeInstances::new(),
            game_meta: GameMeta::new(),
            inventory: Inventory::new(),
            text_particles: vec![],
            job_slots: (0..9).map(|i| JobSlot { index: i, state: JobSlotState::Empty }).collect()
        }
    }

    pub fn get_job_slot_ui(&self, state: &GameState, assets: &Assets, offset: Vec2) -> Vec<UiElement> {
        self.job_slots.iter()
            .flat_map(|job_slot| { job_slot.build_ui(&state, &assets, offset) })
            .collect::<Vec<_>>()
    }

    // Step logic (tick + inputs)
    pub fn step(&mut self, intents: &[Intent], dt: f32) -> Vec<EffectWithSource>
    {
        // Process intents
        intents.iter().for_each(|intent| intent.execute(self));

        // update game progress and collect effects
        let effects = self.update_progress(dt);

        effects
    }

    fn update_progress(&mut self, dt: f32) -> Vec<EffectWithSource>
    {
        let mut effects_with_source = vec![];

        let mut running_jobs: Vec<&mut JobInstance> = self.job_slots.iter_mut()
            .filter_map(|slot| {
                if let JobSlotState::RunningJob(job_instance) = &mut slot.state {
                    Some(job_instance)
                } else {
                    None
                }
            })
            .collect();

        for job_instance in &mut running_jobs {
            if !job_instance.running {
                continue;
            }

            let effects = job_instance.update_progress(&mut self.inventory, dt);

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
                    job: job_instance.clone(),
                    effect: effect.clone(),
                });
            }
        }

        effects_with_source
    }
}

#[derive(Clone)]
pub enum Intent {
    ToggleJob(usize),
    SkipSeconds(i32),
    ToggleHyperMode(usize),
    ChangeJobSlotState(usize, JobSlotState),
}

impl Intent {
    pub fn execute(&self, game_state: &mut GameState) {
        match self {
            Intent::ToggleJob(index) => {
                if let Some(JobSlot { state: JobSlotState::RunningJob(job_instance), .. }) = game_state.job_slots.get_mut(*index) {
                    job_instance.toggle_running();
                }
            }
            Intent::ToggleHyperMode(_index) => {
                // todo: implement hyper mode toggle
            }
            Intent::SkipSeconds(seconds) => {
                for _ in 0..*seconds {
                    // skip capturing effects because we don't want to draw millions of events
                    game_state.update_progress(1.0);
                }
            }
            Intent::ChangeJobSlotState(index, new_state) => {
                if let Some(slot) = game_state.job_slots.get_mut(*index) {
                    slot.state = new_state.clone();
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct UiRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl UiRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

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

#[derive(Clone, PartialEq, Debug)]
pub struct Progress {
    pub value: f64, // Value between 0.0 and 1.0
}

impl Progress {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    pub fn set(&mut self, value: f64) {
        self.value = value.clamp(0.0, 1.0);
    }

    pub fn get(&self) -> f64 {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
    }
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
pub enum WoodItem {
    Kindlewood,
    Craftwood,
    Graintree,
}

trait GetName {
    fn get_name(&self) -> String;
}

impl GetName for WoodItem {
    fn get_name(&self) -> String {
        match self {
            WoodItem::Kindlewood => "Kindlewood".to_string(),
            WoodItem::Craftwood => "Craftwood".to_string(),
            WoodItem::Graintree => "Graintree".to_string(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum WoodWorkingItem {
    Plank,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Item {
    Wood(WoodItem),
    Woodworking(WoodWorkingItem),
    Coin,
    Iron,
    Herb,
    Meat,
    Berry,
    IronBar,
    Sandwich,
    ManaPotion,
}

impl Item {
    pub fn get_name(&self) -> String {
        match self {
            Item::Coin => "Coin".to_string(),
            Item::Wood(item) => item.get_name(),
            Item::Woodworking(WoodWorkingItem::Plank) => "Plank".to_string(),
            Item::Iron => "Iron".to_string(),
            Item::Herb => "Herb".to_string(),
            Item::Meat => "Meat".to_string(),
            Item::Berry => "Berry".to_string(),
            Item::IronBar => "Iron Bar".to_string(),
            Item::Sandwich => "Sandwich".to_string(),
            Item::ManaPotion => "Mana Potion".to_string(),
        }
    }

    pub fn get_texture(&self, assets: &Assets) -> Texture2D {
        match self {
            Item::Wood(WoodItem::Kindlewood) => Kindlewood.texture(assets),
            Item::Wood(WoodItem::Craftwood) => Craftwood.texture(assets),
            Item::Wood(WoodItem::Graintree) => Graintree.texture(assets),
            Item::Meat => MeatGame.texture(assets),
            Item::Coin => Coin.texture(assets),
            Item::Herb => Herbs.texture(assets),
            Item::Sandwich => Sandwich.texture(assets),
            Item::ManaPotion => ManaPotion.texture(assets),
            _default => Texture2D::empty(),
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