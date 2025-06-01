use crate::game::Progress;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub enum SkillCategory {
    Gathering,
    Crafting,
    Selling,
}

#[derive(EnumIter, Clone, PartialEq)]
pub enum SkillArchetype {
    // Gathering Skills
    Lumbering,
    Mining,
    Fishing,
    Hunting,
    Foraging,
    Herbalism,
    Thieving,

    // Crafting Skills
    Woodworking,
    Smithing,
    Tailoring,
    Alchemy,
    Cooking,
}

impl SkillCategory {
    pub fn as_str(&self) -> &str {
        match self {
            SkillCategory::Gathering => "Gathering",
            SkillCategory::Crafting => "Crafting",
            SkillCategory::Selling => "Selling",
        }
    }

    pub fn get_skills(&self) -> Vec<SkillArchetype> {
        match self {
            SkillCategory::Gathering => vec![
                SkillArchetype::Lumbering,
                SkillArchetype::Mining,
                SkillArchetype::Fishing,
                SkillArchetype::Hunting,
                SkillArchetype::Foraging,
                SkillArchetype::Herbalism,
                SkillArchetype::Thieving,
            ],
            SkillCategory::Crafting => vec![
                SkillArchetype::Woodworking,
                SkillArchetype::Smithing,
                SkillArchetype::Tailoring,
                SkillArchetype::Alchemy,
                SkillArchetype::Cooking,
            ],
            SkillCategory::Selling => vec![], // No specific skills for selling
        }
    }
}

impl SkillArchetype {
    pub fn as_str(&self) -> &str {
        match self {
            SkillArchetype::Lumbering => "Lumbering",
            SkillArchetype::Mining => "Mining",
            SkillArchetype::Fishing => "Fishing",
            SkillArchetype::Hunting => "Hunting",
            SkillArchetype::Foraging => "Foraging",
            SkillArchetype::Herbalism => "Herbalism",
            SkillArchetype::Thieving => "Thieving",
            SkillArchetype::Woodworking => "Woodworking",
            SkillArchetype::Smithing => "Smithing",
            SkillArchetype::Tailoring => "Tailoring",
            SkillArchetype::Alchemy => "Alchemy",
            SkillArchetype::Cooking => "Cooking",
        }
    }
}

pub struct SkillArchetypeInstance {
    pub skill_type: SkillArchetype,
    pub actions_done_current_level: i32,
    pub level: u32,
    pub level_up_progress: Progress,
}

impl SkillArchetypeInstance {
    pub fn new(skill_type: SkillArchetype) -> Self {
        Self {
            skill_type,
            level: 1,
            actions_done_current_level: 0,
            level_up_progress: Progress::new(),
        }
    }

    fn actions_to_level(level: u8) -> i64 {
        let first_portion = level * (level + 1) / 2;

        let a = 6.95622e-7;
        let b = 6.57881;
        let c = a * (level as f64).powf(b);

        first_portion as i64 + c as i64
    }

    fn actions_to_reach(current_level: u8, target_level: u8) -> i64 {
        if target_level <= current_level {
            return 0;
        }

        let current_actions = Self::actions_to_level(current_level);
        let target_actions = Self::actions_to_level(target_level);

        target_actions - current_actions
    }

    pub fn actions_to_next_level(&self) -> i64 {
        Self::actions_to_reach(self.level as u8, self.level as u8 + 1)
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.actions_done_current_level = 0; // Reset actions after leveling up
        self.level_up_progress.reset();
    }

    pub fn increment_actions(&mut self) {
        self.actions_done_current_level += 1;

        // update level up progress bar
        self.level_up_progress.set(
            self.actions_done_current_level as f32 / self.actions_to_next_level() as f32
        );

        if self.actions_done_current_level as i64 >= self.actions_to_next_level() {
            self.level_up();
        }
    }
}

pub struct SkillArchetypeInstances {
    pub instances: Vec<SkillArchetypeInstance>,
}

impl SkillArchetypeInstances {
    pub fn new() -> Self {
        Self {
            instances: SkillArchetype::iter()
                .map(SkillArchetypeInstance::new)
                .collect(),
        }
    }

    pub fn get_skill_by_type_mut(&mut self, skill_type: &SkillArchetype) -> &mut SkillArchetypeInstance {
        self.instances.iter_mut().find(|s| s.skill_type == *skill_type)
            .expect("Skill type not found in instances")
    }
    
    pub fn get_skill_by_type(&self, skill_type: &SkillArchetype) -> &SkillArchetypeInstance {
        self.instances.iter().find(|s| s.skill_type == *skill_type)
            .expect("Skill type not found in instances")
    }
}