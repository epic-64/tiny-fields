use crate::assets::AssetId::{AlchemyAnim1, AlchemyAnim2, CookingAnim1, CookingAnim2, HerbalismAnim1, HerbalismAnim2, Hunting1, Hunting2, Mining1, Mining2, Smithing1, Smithing2, WoodAnim1, WoodAnim2};
use crate::assets::{AssetId, Assets};
use crate::counts_actions::CountsActions;
use crate::draw::{number_pill, BorderStyle, UiElement};
use crate::game::{Effect, GameState, Intent, Inventory, Item, Progress, UiRect};
use crate::palette;
use crate::palette::PaletteC;
use crate::skill::SkillArchetype;
use macroquad::math::Vec2;
use macroquad::prelude::Texture2D;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum JobArchetype {
    LumberingWood,
    MiningIron,
    HerbalismChamomile,
    HuntingDeer,
    Foraging,
    WoodworkingPlanks,
    SmithingIronBar,
    CookingSandwich,
    AlchemyManaPotion,
}

impl JobArchetype {
    pub fn get_animation_images(&self, assets: &Assets) -> (Texture2D, Texture2D) {
        match self {
            JobArchetype::LumberingWood => (WoodAnim1.texture(assets), WoodAnim2.texture(assets)),
            JobArchetype::MiningIron => (Mining1.texture(assets), Mining2.texture(assets)),
            JobArchetype::HuntingDeer => (Hunting1.texture(assets), Hunting2.texture(assets)),
            JobArchetype::SmithingIronBar => (Smithing1.texture(assets), Smithing2.texture(assets)),
            JobArchetype::CookingSandwich => (CookingAnim1.texture(assets), CookingAnim2.texture(assets)),
            JobArchetype::HerbalismChamomile => (HerbalismAnim1.texture(assets), HerbalismAnim2.texture(assets)),
            JobArchetype::AlchemyManaPotion => (AlchemyAnim1.texture(assets), AlchemyAnim2.texture(assets)),
            _ => (WoodAnim1.texture(assets), WoodAnim2.texture(assets)),
        }
    }

    pub fn base_duration(&self) -> f64 {
        match self {
            _ => 4.0,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            JobArchetype::LumberingWood => "LumberingWood".to_string(),
            JobArchetype::MiningIron => "Mining".to_string(),
            JobArchetype::HuntingDeer => "Hunting".to_string(),
            JobArchetype::SmithingIronBar => "Smithing".to_string(),
            JobArchetype::HerbalismChamomile => "Herbalism".to_string(),
            JobArchetype::Foraging => "Foraging".to_string(),
            JobArchetype::WoodworkingPlanks => "Woodworking".to_string(),
            JobArchetype::CookingSandwich => "Cooking".to_string(),
            JobArchetype::AlchemyManaPotion => "Alchemy".to_string(),
        }
    }

    pub fn get_product(&self) -> Item {
        match self {
            JobArchetype::LumberingWood => Item::Wood,
            JobArchetype::MiningIron => Item::Iron,
            JobArchetype::HuntingDeer => Item::Meat,
            JobArchetype::SmithingIronBar => Item::IronBar,
            JobArchetype::HerbalismChamomile => Item::Herb,
            JobArchetype::Foraging    => Item::Berry,
            JobArchetype::WoodworkingPlanks => Item::Wood, // todo: change to correct item
            JobArchetype::CookingSandwich => Item::Sandwich,
            JobArchetype::AlchemyManaPotion => Item::ManaPotion, // todo: change to correct item
        }
    }

    pub fn get_required_items(&self) -> Vec<(Item, i64)>{
        match self {
            JobArchetype::LumberingWood => vec![(Item::Tree, 0)],
            JobArchetype::CookingSandwich => vec![(Item::Wood, 4), (Item::Meat, 1), (Item::Herb, 1), (Item::ManaPotion, 1)],
            JobArchetype::HuntingDeer => vec![(Item::Deer, 0)],
            JobArchetype::AlchemyManaPotion => vec![(Item::Herb, 1)],
            JobArchetype::HerbalismChamomile => vec![(Item::Herb, 0)], // todo: change to correct item
            _ => vec![],
        }
    }

    pub fn get_completion_effect(&self) -> Effect {
        Effect::AddItem { item: self.get_product(), amount: 1 }
    }

    pub fn get_skill_type(&self) -> SkillArchetype {
        match self {
            JobArchetype::LumberingWood => SkillArchetype::Lumbering,
            JobArchetype::MiningIron => SkillArchetype::Mining,
            JobArchetype::HuntingDeer => SkillArchetype::Hunting,
            JobArchetype::SmithingIronBar => SkillArchetype::Smithing,
            JobArchetype::HerbalismChamomile => SkillArchetype::Herbalism,
            JobArchetype::Foraging => SkillArchetype::Foraging,
            JobArchetype::WoodworkingPlanks => SkillArchetype::Woodworking,
            JobArchetype::CookingSandwich => SkillArchetype::Cooking,
            JobArchetype::AlchemyManaPotion => SkillArchetype::Alchemy,
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
    pub instance_id: i32,
    pub job_archetype: JobArchetype,
    pub action_progress: Progress,
    pub time_accumulator: f64,
    pub running: bool,
    pub timeslot_cost: i32,
    pub has_paid_resources: bool,
}

pub struct JobParameters {
    pub instance_id: i32,
    pub job_archetype: JobArchetype,
}

impl JobInstance {
    pub fn new(p: JobParameters) -> Self {
        Self {
            job_archetype: p.job_archetype,
            instance_id: p.instance_id,
            running: true, // todo: change to false
            action_progress: Progress{value: 0.0},
            time_accumulator: 0.0,
            timeslot_cost: 1,
            has_paid_resources: false,
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

            self.has_paid_resources = true; // Mark that we've paid resources
        }

        self.time_accumulator += dt as f64;
        self.action_progress.set(self.time_accumulator / duration);

        if self.time_accumulator >= duration {
            // reset job instance
            self.time_accumulator -= duration;
            self.has_paid_resources = false;

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

pub const JOB_CARD_HEIGHT: f32 = 192.0;
pub const JOB_CARD_WIDTH: f32 = 404.0;
pub const JOB_CARD_SPACING_OUTER: f32 = 8.0;

pub fn build_job_card(
    state: &GameState,
    clip: &Option<(i32, i32, i32, i32)>,
    assets: &Assets,
    job: &JobInstance,
    job_slot_id: usize,
    offset: Vec2,
    card_padding_x: f32,
    card_padding_y: f32,
    card_spacing: f32,
) -> Vec<UiElement>
{
    let skill_instance = state.skill_archetype_instances.get_skill_by_type(&job.job_archetype.get_skill_type());
    let job_archetype_instance = state.job_archetype_instances.get_archetype(&job.job_archetype);

    let color_primary = palette::TEXT.get_color();
    let color_secondary = palette::BORDER.get_color();
    let font_size_large = 16.0;
    let font_size_small = 14.0;

    let card_height = JOB_CARD_HEIGHT;
    let card_width = JOB_CARD_WIDTH;

    let image_width = 90.0f32;
    let image_height = 120.0f32;
    let image_x = offset.x + card_padding_x;
    let image_y = offset.y + card_height - image_height - card_padding_y;
    let inner_x = offset.x + card_padding_x + image_width + card_spacing;

    let (image1, image2) = job.job_archetype.get_animation_images(assets);

    let chosen_image = if job.running && job.time_accumulator % 2.0 < 1.0 {
        image1
    } else {
        image2
    };

    let mut elements = vec![];

    // background image (parchment)
    elements.push(UiElement::Image {
        x: offset.x,
        y: offset.y,
        width: card_width,
        height: card_height,
        texture: assets.textures.get(&AssetId::BackgroundParchment).unwrap().clone(),
        color: PaletteC::White.get_color(),
    });

    // Job Animation Rectangle
    elements.push(UiElement::Rectangle {
        x: image_x,
        y: image_y,
        width: image_width,
        height: image_height,
        color: palette::IMAGE_BACKGROUND.get_color(),
        border_style: BorderStyle::Solid,
    });

    // Job Animation Image
    let image_padding = 6.0;
    elements.push(UiElement::Image {
        x: image_x + image_padding,
        y: image_y + image_padding,
        width: image_width - image_padding * 2.0,
        height: image_height - image_padding * 2.0,
        texture: chosen_image.clone(),
        color: PaletteC::White.get_color(),
    });

    let right_side_width = 64.0;

    // Draw the HyperMode button on the right
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + card_width - right_side_width - card_padding_x,
            y: image_y,
            w: right_side_width,
            h: 24.0,
        },
        font: assets.fonts.text_bold.clone(),
        parent_clip: clip.clone(),
        font_size: font_size_small,
        text: "Hyper".to_string(),
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        intent: Intent::ToggleHyperMode(job_slot_id),
        border_style: BorderStyle::Solid,
    });

    // Draw Product Image on the right
    elements.push(UiElement::Rectangle {
        x: offset.x + card_width - right_side_width - card_padding_x,
        y: image_y + 40.0,
        width: right_side_width,
        height: right_side_width,
        color: palette::PRODUCT_COLOR.get_color(),
        border_style: BorderStyle::Solid,
    });

    // Draw Product Image
    elements.push(UiElement::Image {
        x: offset.x + card_width - right_side_width - card_padding_x + 8.0,
        y: image_y + 40.0 + 8.0,
        width: right_side_width - 16.0,
        height: right_side_width - 16.0,
        texture: job.job_archetype.get_product().get_texture(&assets),
        color: PaletteC::White.get_color(),
    });

    // Draw Product Pill at the top of the rectangle
    elements.extend(
        number_pill(
            offset.x + card_width - right_side_width - card_padding_x + right_side_width / 2.0 - 24.0 / 2.0,
            image_y + 40.0 - 14.0 / 2.0,
            24.0,
            14.0,
            state.inventory.get_item_amount(&job.job_archetype.get_product()),
            None,
            assets.fonts.mono.clone()
        )
    );

    // Draw Skill instance level up progress bar
    let skill_progress_bar_width = card_width - card_padding_x - image_width - card_spacing - card_padding_x - right_side_width - card_spacing;
    let skill_progress_bar_height = 10.0;
    let skill_progress_bar_y = image_y;
    elements.push(UiElement::ProgressBar {
        x: inner_x,
        y: skill_progress_bar_y,
        width: skill_progress_bar_width,
        height: skill_progress_bar_height,
        progress: skill_instance.actions_counter.level_up_progress.get(),
        background_color: palette::BAR_BACKGROUND.get_color(),
        foreground_color: palette::SKILL_COLOR.get_color(),
        border_style: BorderStyle::Solid,
    });

    // Draw Job instance level up progress bar
    elements.push(UiElement::ProgressBar {
        x: inner_x,
        y: skill_progress_bar_y + skill_progress_bar_height + 4.0,
        width: skill_progress_bar_width,
        height: skill_progress_bar_height,
        progress: job_archetype_instance.action_counter.level_up_progress.get(),
        background_color: palette::BAR_BACKGROUND.get_color(),
        foreground_color: palette::PRODUCT_COLOR.get_color(),
        border_style: BorderStyle::Solid,
    });

    // Draw 4 resource icons in the middle
    let resource_icon_size = 50.0;
    let resource_icon_padding = 4.0;
    let resource_icon_spacing = 4.0;

    let required_items = job.job_archetype.get_required_items();
    let item_slots = required_items.len();
    let empty_slots = 4 - item_slots;

    for (i, (required_item, required_amount)) in required_items.iter().enumerate() {
        let resource_x = inner_x + (i as f32 * (resource_icon_size + resource_icon_spacing));
        let draw_pills = *required_amount > 0;
        let player_has_enough =
            *required_amount <= state.inventory.get_item_amount(required_item)
            || job.has_paid_resources;

        // draw background rectangle
        elements.push(UiElement::Rectangle {
            x: resource_x,
            y: offset.y + card_padding_y + 96.0,
            width: resource_icon_size,
            height: resource_icon_size,
            color: if player_has_enough { palette::IMAGE_BACKGROUND.get_color() } else { PaletteC::Coral.get_color() },
            border_style: BorderStyle::Solid,
        });

        // draw resource icon
        let inner_size = resource_icon_size - resource_icon_padding * 2.0;
        elements.push(UiElement::Image {
            x: resource_x + resource_icon_size / 2.0 - inner_size / 2.0,
            y: offset.y + card_padding_y + 96.0 + resource_icon_size / 2.0 - inner_size / 2.0,
            width: inner_size,
            height: inner_size,
            texture: required_item.get_texture(&assets),
            color: PaletteC::White.get_color(),
        });

        if draw_pills {
            // draw pill at the top of the rectangle
            elements.extend(
                number_pill(
                    resource_x + resource_icon_size / 2.0 - 24.0 / 2.0,
                    offset.y + card_padding_y + 96.0 - 14.0 / 2.0 - 2.0,
                    24.0,
                    14.0,
                    state.inventory.get_item_amount(required_item),
                    None,
                    assets.fonts.mono.clone()
                )
            );

            // draw pill at the bottom of the rectangle
            let pill_width = resource_icon_size - 24.0;
            let pill_height = 14.0;
            elements.extend(
                number_pill(
                    resource_x + resource_icon_size / 2.0 - pill_width / 2.0,
                    offset.y + card_padding_y + 96.0 + resource_icon_size - pill_height / 2.0 + 2.0,
                    pill_width,
                    pill_height,
                    *required_amount,
                    if player_has_enough { Some(PaletteC::Peach.get_color()) } else { Some(PaletteC::Coral.get_color()) },
                    assets.fonts.mono.clone(),
                )
            )
        }
    }

    // Draw empty slots for resources
    for i in 0..empty_slots {
        let resource_x = inner_x + (item_slots as f32 + i as f32) * (resource_icon_size + resource_icon_spacing);

        // draw background rectangle
        elements.push(UiElement::Rectangle {
            x: resource_x,
            y: offset.y + card_padding_y + 96.0,
            width: resource_icon_size,
            height: resource_icon_size,
            color: palette::IMAGE_BACKGROUND.get_color(),
            border_style: BorderStyle::Dotted,
        });
    }

    // Skill Type and Level
    elements.push(UiElement::Text {
        content: format!(
            "{} Lv. {} ({} / {})",
            skill_instance.skill_type.as_str(),
            skill_instance.actions_counter.level.to_string(),
            skill_instance.actions_counter.actions_done_current_level,
            skill_instance.actions_counter.actions_to_next_level(),
        ),
        font: assets.fonts.text_bold.clone(),
        x: offset.x + card_padding_x,
        y: offset.y + card_padding_y + font_size_large,
        font_size: font_size_large,
        color: color_primary,
    });

    // Job Type and Level
    elements.push(UiElement::Text {
        content: format!(
            "{} Lv. {} ({} / {})",
            job.job_archetype.get_name(),
            job_archetype_instance.action_counter.level,
            job_archetype_instance.action_counter.actions_done_current_level,
            job_archetype_instance.action_counter.actions_to_next_level(),
        ),
        font: assets.fonts.text.clone(),
        x: offset.x + card_padding_x,
        y: offset.y + card_padding_y + 36.,
        font_size: font_size_small,
        color: color_secondary,
    });


    let progress_bar_width = card_width - card_padding_x - image_width - card_spacing - card_padding_x;
    let progress_bar_height = 10.0;
    let progress_bar_action_y = offset.y + card_height - progress_bar_height - card_padding_y;
    // Action Progress Bar
    elements.push(UiElement::ProgressBar {
        x: inner_x,
        y: progress_bar_action_y,
        width: progress_bar_width,
        height: progress_bar_height,
        progress: job.action_progress.get(),
        background_color: palette::BAR_BACKGROUND.get_color(),
        foreground_color: palette::PROGRESS_COLOR.get_color(),
        border_style: BorderStyle::Solid,
    });

    // Delete Button
    let button_dimensions = 30.0;
    let button_spacing = 4.0;
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + card_width - button_dimensions - card_padding_x,
            y: offset.y + card_padding_y,
            w: button_dimensions,
            h: button_dimensions,
        },
        font: assets.fonts.text_bold.clone(),
        parent_clip: clip.clone(),
        font_size: font_size_small,
        text: "x".to_string(),
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        intent: Intent::ToggleJob(job_slot_id),
        border_style: BorderStyle::Solid,
    });

    // Start / Stop Button
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + card_width - button_dimensions * 2.0 - button_spacing - card_padding_x,
            y: offset.y + card_padding_y,
            w: button_dimensions,
            h: button_dimensions,
        },
        font: assets.fonts.text_bold.clone(),
        parent_clip: clip.clone(),
        font_size: font_size_small,
        text: if job.running { "||".to_string() } else { ">".to_string() },
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        intent: Intent::ToggleJob(job_slot_id),
        border_style: BorderStyle::Solid,
    });

    elements
}

pub struct JobArchetypeInstances {
    pub instances: Vec<JobArchetypeInstance>,
}

impl JobArchetypeInstances {
    pub fn new() -> Self {
        let instances = JobArchetype::iter()
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