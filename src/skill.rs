use crate::game::Progress;

pub fn cumulative_actions_to_level(level: u8) -> i64 {
    let first_portion = level * (level + 1) / 2;

    let a = 6.95622e-7;
    let b = 6.57881;
    let c = a * (level as f64).powf(b);

    first_portion as i64 + c as i64
}

pub fn actions_to_reach(current_level: u8, target_level: u8) -> i64 {
    if target_level <= current_level {
        return 0;
    }

    let current_actions = cumulative_actions_to_level(current_level);
    let target_actions = cumulative_actions_to_level(target_level);

    target_actions - current_actions
}

pub enum SkillCategory {
    Gathering,
    Crafting,
    Selling,
}

#[derive(Clone, PartialEq)]
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

    pub fn actions_to_next_level(&self) -> i64 {
        actions_to_reach(self.level as u8, self.level as u8 + 1)
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

pub struct GatheringSkills {
    pub lumbering: SkillArchetypeInstance,
    pub mining: SkillArchetypeInstance,
    pub fishing: SkillArchetypeInstance,
    pub hunting: SkillArchetypeInstance,
    pub foraging: SkillArchetypeInstance,
    pub herbalism: SkillArchetypeInstance,
    pub thieving: SkillArchetypeInstance,
}

impl GatheringSkills {
    pub fn new() -> Self {
        Self {
            lumbering: SkillArchetypeInstance::new(SkillArchetype::Lumbering),
            mining: SkillArchetypeInstance::new(SkillArchetype::Mining),
            fishing: SkillArchetypeInstance::new(SkillArchetype::Fishing),
            hunting: SkillArchetypeInstance::new(SkillArchetype::Hunting),
            foraging: SkillArchetypeInstance::new(SkillArchetype::Foraging),
            herbalism: SkillArchetypeInstance::new(SkillArchetype::Herbalism),
            thieving: SkillArchetypeInstance::new(SkillArchetype::Thieving),
        }
    }
}

pub struct CraftingSkills {
    pub woodworking: SkillArchetypeInstance,
    pub smithing: SkillArchetypeInstance,
    pub tailoring: SkillArchetypeInstance,
    pub alchemy: SkillArchetypeInstance,
    pub cooking: SkillArchetypeInstance,
}

impl CraftingSkills {
    pub fn new() -> Self {
        Self {
            woodworking: SkillArchetypeInstance::new(SkillArchetype::Woodworking),
            smithing: SkillArchetypeInstance::new(SkillArchetype::Smithing),
            tailoring: SkillArchetypeInstance::new(SkillArchetype::Tailoring),
            alchemy: SkillArchetypeInstance::new(SkillArchetype::Alchemy),
            cooking: SkillArchetypeInstance::new(SkillArchetype::Cooking),
        }
    }
}

pub struct SkillArchetypeInstances {
    pub gathering: GatheringSkills,
    pub crafting: CraftingSkills,
}

impl SkillArchetypeInstances {
    pub fn new() -> Self {
        Self {
            gathering: GatheringSkills::new(),
            crafting: CraftingSkills::new(),
        }
    }

    pub fn get_skill_by_type_mut(&mut self, skill_type: &SkillArchetype) -> &mut SkillArchetypeInstance {
        match skill_type {
            SkillArchetype::Lumbering => &mut self.gathering.lumbering,
            SkillArchetype::Mining => &mut self.gathering.mining,
            SkillArchetype::Fishing => &mut self.gathering.fishing,
            SkillArchetype::Hunting => &mut self.gathering.hunting,
            SkillArchetype::Foraging => &mut self.gathering.foraging,
            SkillArchetype::Herbalism => &mut self.gathering.herbalism,
            SkillArchetype::Thieving => &mut self.gathering.thieving,
            SkillArchetype::Woodworking => &mut self.crafting.woodworking,
            SkillArchetype::Smithing => &mut self.crafting.smithing,
            SkillArchetype::Tailoring => &mut self.crafting.tailoring,
            SkillArchetype::Alchemy => &mut self.crafting.alchemy,
            SkillArchetype::Cooking => &mut self.crafting.cooking,
        }
    }
    
    pub fn get_skill_by_type(&self, skill_type: &SkillArchetype) -> &SkillArchetypeInstance {
        match skill_type {
            SkillArchetype::Lumbering => &self.gathering.lumbering,
            SkillArchetype::Mining => &self.gathering.mining,
            SkillArchetype::Fishing => &self.gathering.fishing,
            SkillArchetype::Hunting => &self.gathering.hunting,
            SkillArchetype::Foraging => &self.gathering.foraging,
            SkillArchetype::Herbalism => &self.gathering.herbalism,
            SkillArchetype::Thieving => &self.gathering.thieving,
            SkillArchetype::Woodworking => &self.crafting.woodworking,
            SkillArchetype::Smithing => &self.crafting.smithing,
            SkillArchetype::Tailoring => &self.crafting.tailoring,
            SkillArchetype::Alchemy => &self.crafting.alchemy,
            SkillArchetype::Cooking => &self.crafting.cooking,
        }
    }
}