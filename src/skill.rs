use crate::game::Progress;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::counts_actions::CountsActions;

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
    pub actions_counter: CountsActions,
}

impl SkillArchetypeInstance {
    pub fn new(skill_type: SkillArchetype) -> Self {
        let rate = |level: i64| Self::actions_to_level(level);

        Self {
            skill_type,
            actions_counter: CountsActions::new(rate),
        }
    }

    fn actions_to_level(level: i64) -> i64 {
        let first_portion = level * (level + 1) / 2;

        let a = 6.95622e-7;
        let b = 6.57881;
        let c = a * (level as f64).powf(b);

        first_portion + c as i64
    }

    pub fn increment_actions(&mut self) {
        self.actions_counter.increment_actions();
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