use crate::counts_actions::CountsActions;
use crate::game::{Effect, Inventory, Item, Progress, WoodItem};
use crate::skill::SkillArchetype;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum LumberingJobArchetype {
    Craftwood,
    Graintree,
}

#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum MiningJobArchetype {
    Iron,
}

#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum HuntingJobArchetype {
    Deer,
}

#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum ForagingJobArchetype {
    Herb,
}

#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum CookingJobArchetype {
    Sandwich,
}

#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum AlchemyJobArchetype {
    ManaPotion,
}

#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum SmithingJobArchetype {
    IronBar,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum JobArchetype {
    Lumbering(LumberingJobArchetype),
    Mining(MiningJobArchetype),
    Hunting(HuntingJobArchetype),
    Foraging(ForagingJobArchetype),
    Smithing(SmithingJobArchetype),
    Cooking(CookingJobArchetype),
    Alchemy(AlchemyJobArchetype),
}

impl JobArchetype {
    pub fn base_duration(&self) -> f64 {
        match self {
            _ => 4.0,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            JobArchetype::Lumbering(LumberingJobArchetype::Craftwood) => "Craftwood".to_string(),
            JobArchetype::Lumbering(LumberingJobArchetype::Graintree) => "Graintree".to_string(),
            JobArchetype::Mining(MiningJobArchetype::Iron) => "Mining".to_string(),
            JobArchetype::Hunting(HuntingJobArchetype::Deer) => "Deer".to_string(),
            JobArchetype::Cooking(CookingJobArchetype::Sandwich) => "Sandwich".to_string(),
            JobArchetype::Alchemy(AlchemyJobArchetype::ManaPotion) => "Mana Potion".to_string(),
            JobArchetype::Foraging(ForagingJobArchetype::Herb) => "Herb".to_string(),
            JobArchetype::Smithing(SmithingJobArchetype::IronBar) => "Iron Bar".to_string(),
        }
    }

    pub fn get_product(&self) -> Item {
        match self {
            JobArchetype::Lumbering(LumberingJobArchetype::Craftwood) => Item::Wood(WoodItem::Craftwood),
            JobArchetype::Lumbering(LumberingJobArchetype::Graintree) => Item::Wood(WoodItem::Graintree),
            JobArchetype::Mining(MiningJobArchetype::Iron) => Item::IronOre,
            JobArchetype::Hunting(HuntingJobArchetype::Deer) => Item::Meat,
            JobArchetype::Cooking(CookingJobArchetype::Sandwich) => Item::Sandwich,
            JobArchetype::Alchemy(AlchemyJobArchetype::ManaPotion) => Item::ManaPotion,
            JobArchetype::Foraging(ForagingJobArchetype::Herb) => Item::Herb,
            JobArchetype::Smithing(SmithingJobArchetype::IronBar) => Item::IronBar,
        }
    }

    pub fn get_required_items(&self) -> Vec<(Item, i64)>{
        match self {
            JobArchetype::Cooking(CookingJobArchetype::Sandwich) => vec![
                (Item::Wood(WoodItem::Craftwood), 2),
                (Item::Meat, 2),
                (Item::Herb, 2),
            ],
            JobArchetype::Alchemy(AlchemyJobArchetype::ManaPotion) => vec![
                (Item::Herb, 2),
            ],
            JobArchetype::Smithing(SmithingJobArchetype::IronBar) => vec![
                (Item::IronOre, 2),
            ],
            _ => vec![],
        }
    }

    pub fn get_skill_type(&self) -> SkillArchetype {
        match self {
            JobArchetype::Lumbering(LumberingJobArchetype::Craftwood) => SkillArchetype::Lumbering,
            JobArchetype::Lumbering(LumberingJobArchetype::Graintree) => SkillArchetype::Lumbering,
            JobArchetype::Mining(MiningJobArchetype::Iron) => SkillArchetype::Mining,
            JobArchetype::Hunting(HuntingJobArchetype::Deer) => SkillArchetype::Hunting,
            JobArchetype::Foraging(ForagingJobArchetype::Herb) => SkillArchetype::Foraging,
            JobArchetype::Cooking(CookingJobArchetype::Sandwich) => SkillArchetype::Cooking,
            JobArchetype::Alchemy(AlchemyJobArchetype::ManaPotion) => SkillArchetype::Alchemy,
            JobArchetype::Smithing(SmithingJobArchetype::IronBar) => SkillArchetype::Smithing,
        }
    }
}

pub struct JobArchetypeInstance {
    pub job_archetype: JobArchetype,
    pub action_counter: CountsActions,
}

impl JobArchetypeInstance {
    pub fn new(job_archetype: JobArchetype) -> Self {
        Self {
            job_archetype,
            action_counter: CountsActions::new(Self::actions_cumulative, 1),
        }
    }

    fn actions_cumulative(level: i64) -> i64 {
        let first_portion = level * (level + 1) / 2;

        let a = 6.95622e-9;
        let b = 6.57881;
        let c = a * (level as f64).powf(b);

        first_portion + c as i64
    }

    pub fn increment_actions(&mut self) {
        self.action_counter.increment_actions()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct JobInstance {
    pub job_archetype: JobArchetype,
    pub action_progress: Progress,
    pub time_accumulator: f64,
    pub running: bool,
    pub timeslot_cost: i32,
    pub has_paid_resources: bool,
    pub hyper_mode: HyperMode,
}

pub struct JobParameters {
    pub job_archetype: JobArchetype,
}

#[derive(Clone, PartialEq, Debug)]
pub struct HyperMode {
    pub is_enabled: bool,
    pub hyper_time_accumulator: f64,
    pub hyper_duration_seconds: f64,
    pub hyper_multiplier: f64,
    pub actions_counter: i32,
    pub hyper_actions_cost: i32,
}

impl HyperMode {
    pub fn new() -> Self {
        Self {
            is_enabled: false,
            hyper_time_accumulator: 0.0,
            hyper_duration_seconds: 10.0,
            hyper_multiplier: 4.0,
            actions_counter: 0,
            hyper_actions_cost: 5,
        }
    }

    pub fn has_enough_actions(&self) -> bool {
        self.actions_counter >= self.hyper_actions_cost
    }

    pub fn increment_actions(&mut self) -> () {
        if !self.is_enabled {
            self.actions_counter += 1;
        }
    }

    pub fn enable(&mut self) {
        if self.has_enough_actions() {
            self.actions_counter = 0;
            self.is_enabled = true;
        }
    }

    pub fn update(&mut self, dt: f32) -> () {
        if !self.is_enabled {
            return;
        }

        self.hyper_time_accumulator += dt as f64;

        if self.hyper_time_accumulator >= self.hyper_duration_seconds {
            self.is_enabled = false;
            self.hyper_time_accumulator = 0.0;
        }
    }
}

impl JobInstance {
    pub fn new(p: JobParameters) -> Self {
        Self {
            job_archetype: p.job_archetype,
            running: true, // todo: change to false
            action_progress: Progress{value: 0.0},
            time_accumulator: 0.0,
            timeslot_cost: 1,
            has_paid_resources: false,
            hyper_mode: HyperMode::new(),
        }
    }

    pub fn toggle_running(&mut self) -> () {
        self.running = !self.running;
    }

    pub fn update_progress(&mut self, inventory: &mut Inventory, dt: f32) -> Vec<Effect> {
        let duration = self.job_archetype.base_duration();

        if !self.has_paid_resources {
            // Check if we have the required items to start the job
            let required_items = self.job_archetype.get_required_items();

            for (item, amount) in &required_items {
                if inventory.get_item_amount(&item) < *amount {
                    // Not enough resources to start the job
                    return vec![];
                }
            }

            // Deduct the required items from the inventory
            for (item, amount) in required_items {
                inventory.add_item(item, -amount);
            }

            self.has_paid_resources = true;
        }

        self.hyper_mode.update(dt);

        let dt_progress = if self.hyper_mode.is_enabled {
            dt * self.hyper_mode.hyper_multiplier as f32
        } else {
            dt
        };

        self.time_accumulator += dt_progress as f64;
        self.action_progress.set(self.time_accumulator / duration);

        if self.time_accumulator >= duration {
            self.time_accumulator -= duration;
            self.has_paid_resources = false;
            self.hyper_mode.increment_actions();

            vec![
                Effect::AddItem { item: self.job_archetype.get_product(), amount: 1 },
                Effect::IncrementActionsForSkill { skill_type: self.job_archetype.get_skill_type() },
                Effect::IncrementActionsForJobType { job_type: self.job_archetype.clone() },
            ]
        } else {
            vec![]
        }
    }
}

pub struct JobArchetypeInstances {
    pub instances: Vec<JobArchetypeInstance>,
}

impl JobArchetypeInstances {
    pub fn new() -> Self {
        let instances = LumberingJobArchetype::iter().map(JobArchetype::Lumbering)
            .chain(MiningJobArchetype::iter().map(JobArchetype::Mining))
            .chain(HuntingJobArchetype::iter().map(JobArchetype::Hunting))
            .chain(ForagingJobArchetype::iter().map(JobArchetype::Foraging))
            .chain(CookingJobArchetype::iter().map(JobArchetype::Cooking))
            .chain(AlchemyJobArchetype::iter().map(JobArchetype::Alchemy))
            .chain(SmithingJobArchetype::iter().map(JobArchetype::Smithing))
            .map(|archetype| JobArchetypeInstance::new(archetype))
            .collect();

        Self { instances }
    }

    pub fn get_archetype(&self, job_type: &JobArchetype) -> &JobArchetypeInstance {
        self.instances.iter().find(|i| i.job_archetype == *job_type).unwrap()
    }

    pub fn get_archetype_mut(&mut self, job_type: &JobArchetype) -> &mut JobArchetypeInstance {
        self.instances.iter_mut().find(|i| i.job_archetype == *job_type).unwrap()
    }
}