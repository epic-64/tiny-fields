use crate::assets::AssetId::{AlchemyAnim1, AlchemyAnim2, CookingAnim1, CookingAnim2, CookingIcon, HerbalismAnim1, HerbalismAnim2, HerbalismIcon, HuntingAnim1, HuntingAnim2, HuntingIcon, LumberingIcon, MiningAnim1, MiningAnim2, MiningIcon, SmithingAnim1, SmithingAnim2, WoodAnim1, WoodAnim2};
use crate::assets::Assets;
use crate::counts_actions::CountsActions;
use crate::job::{AlchemyJobArchetype, CookingJobArchetype, HerbalismJobArchetype, HuntingJobArchetype, JobArchetype, LumberingJobArchetype, MiningJobArchetype, SmithingJobArchetype};
use macroquad::prelude::Texture2D;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use SkillArchetype::{Alchemy, Cooking, Herbalism, Hunting, Lumbering, Mining, Smithing};

#[derive(EnumIter, Clone, Debug)]
pub enum SkillCategory {
    Gathering,
    Crafting,
}

impl SkillCategory {
    pub fn as_str(&self) -> &str {
        match self {
            SkillCategory::Gathering => "Gathering",
            SkillCategory::Crafting => "Crafting",
        }
    }

    pub fn get_skill_archetypes(&self) -> Vec<SkillArchetype> {
        match self {
            SkillCategory::Gathering => vec![
                Lumbering,
                Mining,
                Hunting,
                Herbalism,
            ],
            SkillCategory::Crafting => vec![
                Smithing,
                Alchemy,
                Cooking,
            ],
        }
    }
}

#[derive(EnumIter, Clone, PartialEq, Debug)]
pub enum SkillArchetype {
    // Gathering Skills
    Lumbering,
    Mining,
    Hunting,
    Herbalism,

    // Crafting Skills
    Smithing,
    Alchemy,
    Cooking,
}

impl SkillArchetype {
    pub fn get_name(&self) -> &str {
        match self {
            Lumbering => "Lumbering",
            Mining => "Mining",
            Hunting => "Hunting",
            Herbalism => "Herbalism",
            Smithing => "Smithing",
            Alchemy => "Alchemy",
            Cooking => "Cooking",
        }
    }

    pub fn get_job_archetypes(&self) -> Vec<JobArchetype> {
        match self {
            Alchemy => vec![
                JobArchetype::Alchemy(AlchemyJobArchetype::ManaPotion),
            ],

            Cooking => vec![
                JobArchetype::Cooking(CookingJobArchetype::Sandwich),
            ],

            Lumbering => vec![
                JobArchetype::Lumbering(LumberingJobArchetype::Craftwood),
                JobArchetype::Lumbering(LumberingJobArchetype::Graintree),
            ],

            Mining => vec![
                JobArchetype::Mining(MiningJobArchetype::Iron),
            ],

            Hunting => vec![
                JobArchetype::Hunting(HuntingJobArchetype::Deer),
            ],

            Herbalism => vec![
                JobArchetype::Herbalism(HerbalismJobArchetype::Herb),
            ],

            Smithing => vec![
                JobArchetype::Smithing(SmithingJobArchetype::IronBar),
            ],
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
        }
    }

    pub fn get_icon_texture(&self, assets: &Assets) -> Texture2D {
        match self {
            Lumbering => LumberingIcon.texture(assets),
            Mining => MiningIcon.texture(assets),
            Hunting => HuntingIcon.texture(assets),
            Herbalism => HerbalismIcon.texture(assets),
            Cooking => CookingIcon.texture(assets),
            _default => Texture2D::empty(),
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
            actions_counter: CountsActions::new(Self::actions_to_level, 10),
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