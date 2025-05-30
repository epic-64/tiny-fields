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
pub enum SkillType {
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

    pub fn get_skills(&self) -> Vec<SkillType> {
        match self {
            SkillCategory::Gathering => vec![
                SkillType::Lumbering,
                SkillType::Mining,
                SkillType::Fishing,
                SkillType::Hunting,
                SkillType::Foraging,
                SkillType::Herbalism,
                SkillType::Thieving,
            ],
            SkillCategory::Crafting => vec![
                SkillType::Woodworking,
                SkillType::Smithing,
                SkillType::Tailoring,
                SkillType::Alchemy,
                SkillType::Cooking,
            ],
            SkillCategory::Selling => vec![], // No specific skills for selling
        }
    }
}

impl SkillType {
    pub fn as_str(&self) -> &str {
        match self {
            SkillType::Lumbering => "Lumbering",
            SkillType::Mining => "Mining",
            SkillType::Fishing => "Fishing",
            SkillType::Hunting => "Hunting",
            SkillType::Foraging => "Foraging",
            SkillType::Herbalism => "Herbalism",
            SkillType::Thieving => "Thieving",
            SkillType::Woodworking => "Woodworking",
            SkillType::Smithing => "Smithing",
            SkillType::Tailoring => "Tailoring",
            SkillType::Alchemy => "Alchemy",
            SkillType::Cooking => "Cooking",
        }
    }
}

pub struct SkillInstance {
    pub skill_type: SkillType,
    pub actions_done_current_level: i32,
    pub level: u32,
    pub level_up_progress: Progress,
}

impl SkillInstance {
    pub fn new(skill_type: SkillType) -> Self {
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

    pub fn increment_actions(&mut self, amount: i32) {
        self.actions_done_current_level += amount;

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
    pub lumbering: SkillInstance,
    pub mining: SkillInstance,
    pub fishing: SkillInstance,
    pub hunting: SkillInstance,
    pub foraging: SkillInstance,
    pub herbalism: SkillInstance,
    pub thieving: SkillInstance,
}

impl GatheringSkills {
    pub fn new() -> Self {
        Self {
            lumbering: SkillInstance::new(SkillType::Lumbering),
            mining: SkillInstance::new(SkillType::Mining),
            fishing: SkillInstance::new(SkillType::Fishing),
            hunting: SkillInstance::new(SkillType::Hunting),
            foraging: SkillInstance::new(SkillType::Foraging),
            herbalism: SkillInstance::new(SkillType::Herbalism),
            thieving: SkillInstance::new(SkillType::Thieving),
        }
    }
}

pub struct CraftingSkills {
    pub woodworking: SkillInstance,
    pub smithing: SkillInstance,
    pub tailoring: SkillInstance,
    pub alchemy: SkillInstance,
    pub cooking: SkillInstance,
}

impl CraftingSkills {
    pub fn new() -> Self {
        Self {
            woodworking: SkillInstance::new(SkillType::Woodworking),
            smithing: SkillInstance::new(SkillType::Smithing),
            tailoring: SkillInstance::new(SkillType::Tailoring),
            alchemy: SkillInstance::new(SkillType::Alchemy),
            cooking: SkillInstance::new(SkillType::Cooking),
        }
    }
}

pub struct Skills {
    pub gathering: GatheringSkills,
    pub crafting: CraftingSkills,
}

impl Skills {
    pub fn new() -> Self {
        Self {
            gathering: GatheringSkills::new(),
            crafting: CraftingSkills::new(),
        }
    }

    pub fn get_skill_by_type_mut(&mut self, skill_type: &SkillType) -> &mut SkillInstance {
        match skill_type {
            SkillType::Lumbering => &mut self.gathering.lumbering,
            SkillType::Mining => &mut self.gathering.mining,
            SkillType::Fishing => &mut self.gathering.fishing,
            SkillType::Hunting => &mut self.gathering.hunting,
            SkillType::Foraging => &mut self.gathering.foraging,
            SkillType::Herbalism => &mut self.gathering.herbalism,
            SkillType::Thieving => &mut self.gathering.thieving,
            SkillType::Woodworking => &mut self.crafting.woodworking,
            SkillType::Smithing => &mut self.crafting.smithing,
            SkillType::Tailoring => &mut self.crafting.tailoring,
            SkillType::Alchemy => &mut self.crafting.alchemy,
            SkillType::Cooking => &mut self.crafting.cooking,
        }
    }
    
    pub fn get_skill_by_type(&self, skill_type: &SkillType) -> &SkillInstance {
        match skill_type {
            SkillType::Lumbering => &self.gathering.lumbering,
            SkillType::Mining => &self.gathering.mining,
            SkillType::Fishing => &self.gathering.fishing,
            SkillType::Hunting => &self.gathering.hunting,
            SkillType::Foraging => &self.gathering.foraging,
            SkillType::Herbalism => &self.gathering.herbalism,
            SkillType::Thieving => &self.gathering.thieving,
            SkillType::Woodworking => &self.crafting.woodworking,
            SkillType::Smithing => &self.crafting.smithing,
            SkillType::Tailoring => &self.crafting.tailoring,
            SkillType::Alchemy => &self.crafting.alchemy,
            SkillType::Cooking => &self.crafting.cooking,
        }
    }
}