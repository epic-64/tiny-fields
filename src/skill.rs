use macroquad::prelude::Texture2D;
use crate::counts_actions::CountsActions;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use SkillArchetype::{Alchemy, Cooking, Herbalism, Hunting, Lumbering, Mining, Smithing};
use crate::assets::AssetId::{AlchemyAnim1, AlchemyAnim2, CookingAnim1, CookingAnim2, HerbalismAnim1, HerbalismAnim2, HuntingAnim1, HuntingAnim2, MiningAnim1, MiningAnim2, SmithingAnim1, SmithingAnim2, WoodAnim1, WoodAnim2};
use crate::assets::Assets;
use crate::job::{JobArchetype, LumberingJobArchetype};

#[derive(EnumIter, Clone, Debug)]
pub enum SkillCategory {
    Gathering,
    Crafting,
    Selling,
}

impl SkillCategory {
    pub fn as_str(&self) -> &str {
        match self {
            SkillCategory::Gathering => "Gathering",
            SkillCategory::Crafting => "Crafting",
            SkillCategory::Selling => "Selling",
        }
    }

    pub fn get_skill_archetypes(&self) -> Vec<SkillArchetype> {
        match self {
            SkillCategory::Gathering => vec![
                Lumbering,
                Mining,
                SkillArchetype::Fishing,
                Hunting,
                SkillArchetype::Foraging,
                Herbalism,
                SkillArchetype::Thieving,
            ],
            SkillCategory::Crafting => vec![
                SkillArchetype::Woodworking,
                Smithing,
                SkillArchetype::Tailoring,
                Alchemy,
                Cooking,
            ],
            SkillCategory::Selling => vec![], // No specific skills for selling
        }
    }
}

#[derive(EnumIter, Clone, PartialEq, Debug)]
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

impl SkillArchetype {
    pub fn get_name(&self) -> &str {
        match self {
            Lumbering => "Lumbering",
            Mining => "Mining",
            SkillArchetype::Fishing => "Fishing",
            Hunting => "Hunting",
            SkillArchetype::Foraging => "Foraging",
            Herbalism => "Herbalism",
            SkillArchetype::Thieving => "Thieving",
            SkillArchetype::Woodworking => "Woodworking",
            Smithing => "Smithing",
            SkillArchetype::Tailoring => "Tailoring",
            Alchemy => "Alchemy",
            Cooking => "Cooking",
        }
    }

    pub fn get_job_archetypes(&self) -> Vec<JobArchetype> {
        match self {
            Lumbering => vec![
                JobArchetype::Lumbering(LumberingJobArchetype::Kindlewood),
                JobArchetype::Lumbering(LumberingJobArchetype::Craftwood),
                JobArchetype::Lumbering(LumberingJobArchetype::Graintree),
            ],

            Mining => vec![
                JobArchetype::MiningIron,
            ],
            
            default => vec![],
        }
    }

    pub fn get_animation_images(&self, assets: &Assets) -> (Texture2D, Texture2D) {
        match self {
            Lumbering => (WoodAnim1.texture(assets), WoodAnim2.texture(assets)),
            Mining => (MiningAnim1.texture(assets), MiningAnim2.texture(assets)),
            Hunting => (HuntingAnim1.texture(assets), HuntingAnim2.texture(assets)),
            Herbalism => (HerbalismAnim1.texture(assets), HerbalismAnim2.texture(assets)),
            Smithing => (SmithingAnim1.texture(assets), SmithingAnim2.texture(assets)),
            Cooking => (CookingAnim1.texture(assets), CookingAnim2.texture(assets)),
            Alchemy => (AlchemyAnim1.texture(assets), AlchemyAnim2.texture(assets)),
            default => (Texture2D::empty(), Texture2D::empty()), // todo: handle other skills
        }
    }
}

pub struct SkillArchetypeInstance {
    pub skill_type: SkillArchetype,
    pub actions_counter: CountsActions,
}

impl SkillArchetypeInstance {
    pub fn new(skill_type: SkillArchetype) -> Self {
        Self {
            skill_type,
            actions_counter: CountsActions::new(Self::actions_to_level, 5),
        }
    }

    fn actions_to_level(level: i64) -> i64 {
        let first_portion = level * (level + 1) / 2;

        let a = 6.95622e-7;
        let b = 6.57881;
        let c = a * (level as f64).powf(b);

        5 + first_portion + c as i64
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