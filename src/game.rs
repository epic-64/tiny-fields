use crate::assets::AssetId::*;
use crate::assets::{AssetId, Assets};
use crate::draw::UiElement;
use macroquad::color::Color;
use macroquad::input::MouseButton;
use macroquad::math::Vec2;
use macroquad::prelude::Texture2D;
use std::collections::HashMap;
use AssetId::{WoodAnim1, WoodAnim2};

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
            jobs: vec![],
            time_slots: TimeSlots { total: 9, used: 0, },
            performance_flags: PerformanceFlags::new(),
            game_meta: GameMeta::new(),
            inventory: Inventory::new(),
            text_particles: vec![],
        }
    }

    pub fn add_job_instance(&mut self, job_type: JobType) {
        self.jobs.push(
            JobInstance::new(JobParameters {
                instance_id: self.jobs.iter().map(|j| j.instance_id).max().unwrap_or(0) + 1,
                job_type: job_type.clone(),
            })
        );
    }

    // Step logic (tick + inputs)
    pub fn step(&mut self, actions: &[Intent], dt: f32) -> Vec<EffectWithSource>
    {
        let free_timeslots = self.time_slots.get_free();

        for action in actions {
            match action {
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
        let mut effects = vec![];

        for job in &mut self.jobs {
            if job.running {
                if let Some(effect) = job.update_progress(&mut self.inventory, dt) {
                    effects.push(EffectWithSource::JobSource {
                        job: job.clone(),
                        effect: effect.clone(),
                    });
                }
            }
        }

        // process side effects
        for effect in &effects {
            match effect {
                EffectWithSource::JobSource { effect, .. } => {
                    match effect {
                        Effect::AddItem { item, amount } => {
                            self.inventory.add_item(*item, *amount);
                        }
                    }
                }
            }
        }

        effects
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
    pub actions_until_level_up: i32,
}

#[derive(Clone, PartialEq)]
pub enum Effect {
    AddItem { item: Item, amount: i64 },
}

pub enum EffectWithSource {
    JobSource { job: JobInstance, effect: Effect },
}

#[derive(Clone, PartialEq)]
pub enum JobType {
    Lumberjacking,
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
    pub fn get_animation_images(&self, assets: &Assets) -> (Texture2D, Texture2D) {
        match self {
            JobType::Lumberjacking => (WoodAnim1.texture(assets), WoodAnim2.texture(assets)),
            JobType::Mining => (Mining1.texture(assets), Mining2.texture(assets)),
            JobType::Hunting => (Hunting1.texture(assets), Hunting2.texture(assets)),
            JobType::Smithing => (Smithing1.texture(assets), Smithing2.texture(assets)),
            JobType::Cooking => (CookingAnim1.texture(assets), CookingAnim2.texture(assets)),
            JobType::Herbalism => (HerbalismAnim1.texture(assets), HerbalismAnim2.texture(assets)),
            JobType::Alchemy => (AlchemyAnim1.texture(assets), AlchemyAnim2.texture(assets)),
            _ => (WoodAnim1.texture(assets), WoodAnim2.texture(assets)),
        }
    }

    pub fn base_actions_to_level_up(&self) -> i32 {
        10
    }

    pub fn base_duration(&self) -> f32 {
        match self {
            _ => 4.0,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            JobType::Lumberjacking => "Lumberjacking".to_string(),
            JobType::Mining => "Mining".to_string(),
            JobType::Hunting => "Hunting".to_string(),
            JobType::Smithing => "Smithing".to_string(),
            JobType::Herbalism => "Herbalism".to_string(),
            JobType::Foraging => "Foraging".to_string(),
            JobType::Woodworking => "Woodworking".to_string(),
            JobType::Cooking => "Cooking".to_string(),
            JobType::Alchemy => "Alchemy".to_string(),
            JobType::Selling => "Selling".to_string(),
        }
    }

    pub fn get_product(&self) -> Item {
        match self {
            JobType::Lumberjacking => Item::Wood,
            JobType::Mining      => Item::Iron,
            JobType::Hunting     => Item::Meat,
            JobType::Smithing    => Item::IronBar,
            JobType::Herbalism   => Item::Herb,
            JobType::Foraging    => Item::Berry,
            JobType::Woodworking => Item::Wood, // todo: change to correct item
            JobType::Cooking     => Item::Sandwich,
            JobType::Alchemy     => Item::ManaPotion, // todo: change to correct item
            JobType::Selling     => Item::Coin,
        }
    }

    pub fn get_required_items(&self) -> Vec<(Item, i64)>{
        match self {
            JobType::Lumberjacking => vec![(Item::Tree, 0)],
            JobType::Cooking => vec![(Item::Wood, 4), (Item::Meat, 1), (Item::Herb, 1), (Item::ManaPotion, 1)],
            JobType::Hunting => vec![(Item::Deer, 0)],
            JobType::Alchemy => vec![(Item::Herb, 1)],
            JobType::Herbalism => vec![(Item::Herb, 0)], // todo: change to correct item
            _ => vec![],
        }
    }

    pub fn get_completion_effect(&self) -> Effect {
        Effect::AddItem { item: self.get_product(), amount: 1 }
    }
}

#[derive(Clone, PartialEq)]
pub struct JobInstance {
    pub instance_id: i32,
    pub job_type: JobType,
    pub action_progress: Progress,
    pub level_up_progress: Progress,
    pub level: i32,
    pub time_accumulator: f32,
    pub running: bool,
    pub actions_done: i32,
    pub timeslot_cost: i32,
    pub has_paid_resources: bool,
}

pub struct JobParameters {
    pub instance_id: i32,
    pub job_type: JobType,
}

impl JobInstance {
    pub fn new(p: JobParameters) -> Self {
        Self {
            instance_id: p.instance_id,
            level: 1,
            running: false,
            action_progress: Progress{value: 0.0},
            level_up_progress: Progress{value: 0.0},
            time_accumulator: 0.0,
            actions_done: 0,
            timeslot_cost: 1,
            job_type: p.job_type,
            has_paid_resources: false,
        }
    }

    pub fn toggle_running(&mut self, free_timeslots: i32) -> () {
        if self.running {
            self.running = false;
        } else if free_timeslots >= self.timeslot_cost {
            self.running = true;
        }
    }

    pub fn update_progress(&mut self, inventory: &mut Inventory, dt: f32) -> Option<Effect> {
        let duration = self.job_type.base_duration();

        if !self.has_paid_resources {
            // Check if we have the required items to start the job
            let required_items = self.job_type.get_required_items();

            for (item, amount) in &required_items {
                if inventory.get_item_amount(&item) < *amount {
                    // Not enough resources to start the job
                    return None;
                }
            }

            // Deduct the required items from the inventory
            for (item, amount) in required_items {
                inventory.add_item(item, -amount);
            }

            self.has_paid_resources = true; // Mark that we've paid resources
        }

        self.time_accumulator += dt;
        self.action_progress.set(self.time_accumulator / duration);

        if self.time_accumulator >= duration {
            // reset job instance
            self.time_accumulator -= duration;
            self.has_paid_resources = false;
            self.actions_done += 1;

            // update level up progress bar
            self.level_up_progress.set(
                self.actions_done as f32 / self.actions_to_level_up() as f32
            );

            // level up if enough actions done
            if self.actions_done >= self.actions_to_level_up() {
                self.level_up();
            }

            Some(self.job_type.get_completion_effect())
        } else {
            None
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.actions_done = 0;
        self.level_up_progress.reset();
    }

    pub fn actions_to_level_up(&self) -> i32 {
        let base_actions = self.job_type.base_actions_to_level_up();
        let growth_factor: f32 = 1.5;

        (base_actions as f32 * growth_factor.powi(self.level - 1)) as i32
    }

    pub fn get_particle_marker(&self, elements: &Vec<UiElement>) -> (f32, f32) {
        let mut found_x = 0.0;
        let mut found_y = 0.0;

        for element in elements {
            match element {
                UiElement::JobParticleMarker { x, y, job } => {
                    if job.instance_id == self.instance_id {
                        found_x = *x;
                        found_y = *y;
                        break;
                    }
                }
                _ => {}
            }
        }

        (found_x, found_y)
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