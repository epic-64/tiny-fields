use crate::assets::{AssetId, Assets};
use crate::draw::{pill, UiElement};
use crate::game::{Effect, GameState, Intent, Inventory, Item, Progress, UiRect};
use crate::palette;
use crate::palette::PaletteC;
use macroquad::math::Vec2;
use macroquad::prelude::Texture2D;
use crate::assets::AssetId::{AlchemyAnim1, AlchemyAnim2, CookingAnim1, CookingAnim2, HerbalismAnim1, HerbalismAnim2, Hunting1, Hunting2, Mining1, Mining2, Smithing1, Smithing2, WoodAnim1, WoodAnim2};
use crate::skill::{SkillType, Skills};

#[derive(Clone, PartialEq)]
pub enum JobType {
    Lumbering,
    Mining,
    Herbalism,
    Hunting,
    Foraging,
    Woodworking,
    Smithing,
    Cooking,
    Alchemy,
}

impl JobType {
    pub fn get_animation_images(&self, assets: &Assets) -> (Texture2D, Texture2D) {
        match self {
            JobType::Lumbering => (WoodAnim1.texture(assets), WoodAnim2.texture(assets)),
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
            JobType::Lumbering => "Lumbering".to_string(),
            JobType::Mining => "Mining".to_string(),
            JobType::Hunting => "Hunting".to_string(),
            JobType::Smithing => "Smithing".to_string(),
            JobType::Herbalism => "Herbalism".to_string(),
            JobType::Foraging => "Foraging".to_string(),
            JobType::Woodworking => "Woodworking".to_string(),
            JobType::Cooking => "Cooking".to_string(),
            JobType::Alchemy => "Alchemy".to_string(),
        }
    }

    pub fn get_product(&self) -> Item {
        match self {
            JobType::Lumbering   => Item::Wood,
            JobType::Mining      => Item::Iron,
            JobType::Hunting     => Item::Meat,
            JobType::Smithing    => Item::IronBar,
            JobType::Herbalism   => Item::Herb,
            JobType::Foraging    => Item::Berry,
            JobType::Woodworking => Item::Wood, // todo: change to correct item
            JobType::Cooking     => Item::Sandwich,
            JobType::Alchemy     => Item::ManaPotion, // todo: change to correct item
        }
    }

    pub fn get_required_items(&self) -> Vec<(Item, i64)>{
        match self {
            JobType::Lumbering => vec![(Item::Tree, 0)],
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

    pub fn get_skill_type(&self) -> SkillType {
        match self {
            JobType::Lumbering => SkillType::Lumbering,
            JobType::Mining => SkillType::Mining,
            JobType::Hunting => SkillType::Hunting,
            JobType::Smithing => SkillType::Smithing,
            JobType::Herbalism => SkillType::Herbalism,
            JobType::Foraging => SkillType::Foraging,
            JobType::Woodworking => SkillType::Woodworking,
            JobType::Cooking => SkillType::Cooking,
            JobType::Alchemy => SkillType::Alchemy,
        }
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

    pub fn update_progress(&mut self, inventory: &mut Inventory, dt: f32) -> Vec<Effect> {
        let duration = self.job_type.base_duration();

        if !self.has_paid_resources {
            // Check if we have the required items to start the job
            let required_items = self.job_type.get_required_items();

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

            vec![
                self.job_type.get_completion_effect(),
                Effect::IncrementActionsForSkill {
                    skill_type: self.job_type.get_skill_type(),
                    amount: 1,
                },
            ]
        } else {
            vec![]
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
}

pub const JOB_CARD_HEIGHT: f32 = 192.0;
pub const JOB_CARD_WIDTH: f32 = 404.0;
pub const JOB_CARD_SPACING_OUTER: f32 = 8.0;

pub fn build_job_cards(state: &GameState, assets: &Assets, offset: Vec2) -> Vec<UiElement>
{
    let mut elements: Vec<UiElement> = vec![];

    let mut container_offset = offset;
    let mut offset_x = offset.x;
    let mut offset_y = offset.y;

    let card_height = JOB_CARD_HEIGHT;
    let card_width = JOB_CARD_WIDTH;
    let card_spacing_inner = 6.0;
    let card_padding_x = 12.0;
    let card_padding_y = 12.0;

    for (id, job) in state.jobs.iter().enumerate() {
        let job_draw_container = build_job_card(
            state,
            &None, // No clipping for the job cards
            assets,
            job,
            id,
            container_offset,
            card_height,
            card_width,
            card_padding_x,
            card_padding_y,
            card_spacing_inner,
        );

        elements.extend(job_draw_container);

        if (id + 1) % 3 == 0 && id != 0 {
            offset_x = offset.x; // Reset horizontal offset for the new row
            offset_y += card_height + JOB_CARD_SPACING_OUTER;
        } else {
            offset_x += card_width + JOB_CARD_SPACING_OUTER;
        }

        container_offset = Vec2::new(offset_x, offset_y);
    }

    elements
}

pub fn build_job_card(
    state: &GameState,
    clip: &Option<(i32, i32, i32, i32)>,
    assets: &Assets,
    job: &JobInstance,
    job_id: usize,
    offset: Vec2,
    card_height: f32,
    card_width: f32,
    card_padding_x: f32,
    card_padding_y: f32,
    card_spacing: f32,
) -> Vec<UiElement>
{
    let color_primary = palette::TEXT.get_color();
    let color_secondary = palette::BORDER.get_color();
    let font_size_large = 16.0;
    let font_size_small = 14.0;

    let image_width = 90.0f32;
    let image_height = 120.0f32;
    let image_x = offset.x + card_padding_x;
    let image_y = offset.y + card_height - image_height - card_padding_y;
    let inner_x = offset.x + card_padding_x + image_width + card_spacing;

    let (image1, image2) = job.job_type.get_animation_images(assets);

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

    // Job Animation background
    // elements.push(UiElement::Rectangle {
    //     x: image_x,
    //     y: image_y,
    //     width: image_width,
    //     height: image_height,
    //     color: palette::IMAGE_BACKGROUND.get_color(),
    //     bordered: false,
    // });

    // Job Animation Image
    let image_padding = 0.0;
    elements.push(UiElement::Image {
        x: image_x + image_padding,
        y: image_y + image_padding,
        width: image_width - image_padding * 2.0,
        height: image_height - image_padding * 2.0,
        texture: chosen_image.clone(),
        color: PaletteC::White.get_color(),
    });

    let right_side_width = 64.0;

    // Draw HyperMode button on the right
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + card_width - right_side_width - card_padding_x,
            y: image_y,
            w: right_side_width,
            h: 30.0,
        },
        font: assets.fonts.text_bold.clone(),
        parent_clip: clip.clone(),
        font_size: font_size_small,
        text: "Hyper".to_string(),
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        intent: Intent::ToggleHyperMode(job_id),
    });

    // Draw Product Image on the right
    elements.push(UiElement::Rectangle {
        x: offset.x + card_width - right_side_width - card_padding_x,
        y: image_y + 40.0,
        width: right_side_width,
        height: right_side_width,
        color: palette::PRODUCT_COLOR.get_color(),
        bordered: true,
    });

    // Draw Product Image
    elements.push(UiElement::Image {
        x: offset.x + card_width - right_side_width - card_padding_x + 8.0,
        y: image_y + 40.0 + 8.0,
        width: right_side_width - 16.0,
        height: right_side_width - 16.0,
        texture: job.job_type.get_product().get_texture(&assets),
        color: PaletteC::White.get_color(),
    });

    // Draw Product Pill at the top of the rectangle
    elements.extend(
        pill(
            offset.x + card_width - right_side_width - card_padding_x + right_side_width / 2.0 - 24.0 / 2.0,
            image_y + 40.0 - 14.0 / 2.0,
            24.0,
            14.0,
            state.inventory.get_item_amount(&job.job_type.get_product()).to_string().as_str(),
            None,
            assets.fonts.mono.clone()
        )
    );

    // Draw 4 resource icons in the middle
    let resource_icon_size = 50.0;
    let resource_icon_padding = 4.0;
    let resource_icon_spacing = 4.0;

    let required_items = job.job_type.get_required_items();
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
            bordered: false,
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
                pill(
                    resource_x + resource_icon_size / 2.0 - 24.0 / 2.0,
                    offset.y + card_padding_y + 96.0 - 14.0 / 2.0,
                    24.0,
                    14.0,
                    state.inventory.get_item_amount(required_item).to_string().as_str(),
                    None,
                    assets.fonts.mono.clone()
                )
            );

            // draw pill at the bottom of the rectangle
            let pill_width = resource_icon_size - 24.0;
            let pill_height = 14.0;
            elements.extend(
                pill(
                    resource_x + resource_icon_size / 2.0 - pill_width / 2.0,
                    offset.y + card_padding_y + 96.0 + resource_icon_size - pill_height / 2.0,
                    pill_width,
                    pill_height,
                    required_amount.to_string().as_str(),
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
            bordered: false,
        });
    }

    // Title Bar
    elements.push(UiElement::Text {
        content: job.job_type.get_name(),
        font: assets.fonts.text_bold.clone(),
        x: offset.x + card_padding_x,
        y: offset.y + card_padding_y + font_size_large,
        font_size: font_size_large,
        color: color_primary,
    });

    // Job Info
    elements.push(UiElement::Text {
        content: format!("Lvl {} | {}s", job.level, job.job_type.base_duration()),
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
        with_border: true,
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
        intent: Intent::ToggleJob(job_id),
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
        intent: Intent::ToggleJob(job_id),
    });

    elements
}