use crate::assets::AssetId::{BackgroundParchment, LockIcon};
use crate::assets::{AssetId, Assets};
use crate::draw::{number_pill, BorderStyle, UiElement};
use crate::game::{GameState, Intent, UiRect};
use crate::job::{JobInstance, JobParameters};
use crate::palette;
use crate::palette::PaletteC;
use crate::skill::{SkillArchetype, SkillCategory};
use macroquad::prelude::Vec2;
use strum::IntoEnumIterator;

pub const JOB_CARD_HEIGHT: f32 = 192.0;
pub const JOB_CARD_WIDTH: f32 = 404.0;
pub const JOB_CARD_SPACING_OUTER: f32 = 8.0;

#[derive(Clone, Debug)]
pub enum JobSlotState {
    Locked,
    Empty,
    PickingCategory,
    PickingSkill(SkillCategory),
    PickingProduct(SkillArchetype),
    RunningJob(JobInstance),
}

impl JobSlotState {
    pub fn build_ui(&self, job_slot_index: usize, state: &GameState, assets: &Assets, offset: Vec2) -> Vec<UiElement> {
        let column = job_slot_index % 3;
        let row = job_slot_index / 3;

        let offset = Vec2::new(
            offset.x + (column as f32 * JOB_CARD_WIDTH) + JOB_CARD_SPACING_OUTER * (column as f32),
            offset.y + (row as f32 * JOB_CARD_HEIGHT) + JOB_CARD_SPACING_OUTER * (row as f32),
        );

        let mut elements = vec![];

        elements.push(UiElement::Image {
            x: offset.x,
            y: offset.y,
            width: JOB_CARD_WIDTH,
            height: JOB_CARD_HEIGHT,
            texture: BackgroundParchment.texture(&assets),
            color: palette::CARD_BACKGROUND.get_color(),
        });

        let layout = CardLayout::new(10.0, 10.0, 10.0, 10.0);

        let state_specific_elements = match self {
            JobSlotState::Locked => locked_job_slot_ui(job_slot_index, assets, offset),
            JobSlotState::Empty => empty_job_slot_ui(job_slot_index, assets, offset),
            JobSlotState::PickingCategory => category_selection_ui(job_slot_index, assets, offset, &layout),
            JobSlotState::PickingSkill(category) => skill_selection_ui(job_slot_index, category, assets, offset),
            JobSlotState::PickingProduct(skill_archetype) => product_selection_ui(job_slot_index, skill_archetype, assets, offset),
            JobSlotState::RunningJob(job_instance) => job_card_ui(&state, assets, job_instance, job_slot_index, offset),
        };

        elements.extend(state_specific_elements);

        elements
    }
}

#[derive(Clone, Debug)]
pub struct JobSlot {
    pub index: usize,
    pub state: JobSlotState,
}

impl JobSlot {
    pub fn build_ui(&self, game_state: &GameState, assets: &Assets, offset: Vec2) -> Vec<UiElement> {
        self.state.build_ui(self.index, game_state, assets, offset)
    }
}

struct CardLayout {
    padding_x: f32,
    padding_y: f32,
    spacing_x: f32,
    spacing_y: f32,
}

impl CardLayout {
    pub fn new(padding_x: f32, padding_y: f32, spacing_x: f32, spacing_y: f32) -> Self {
        Self { padding_x, padding_y, spacing_x, spacing_y, }
    }
}

fn locked_job_slot_ui(index: usize, assets: &Assets, offset: Vec2) -> Vec<UiElement> {
    let mut elements = vec![];

    let icon_size = 64.0;

    // Add image button of a lock in the middle of the card
    elements.push(UiElement::ImgButton {
        rectangle: UiRect::new(
            offset.x + JOB_CARD_WIDTH / 2.0 - icon_size / 2.0,
            offset.y + JOB_CARD_HEIGHT / 2.0 - icon_size / 2.0,
            icon_size,
            icon_size,
        ),
        intent: Intent::ChangeJobSlotState(index, JobSlotState::PickingCategory),
        texture: LockIcon.texture(&assets),
        parent_clip: None,
        border_style: BorderStyle::None,
    });

    elements
}

fn empty_job_slot_ui(job_slot_index: usize, assets: &Assets, offset: Vec2) -> Vec<UiElement> {
    let mut elements = vec![];

    // Add title: Empty Slot
    elements.push(UiElement::Text {
        content: "Empty Slot".to_string(),
        font: assets.fonts.text_bold.clone(),
        x: offset.x + 10.0,
        y: offset.y + 10.0 + 32.0,
        font_size: 32.0,
        color: palette::TEXT.get_color(),
    });

    // Add button to select category
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + 10.0,
            y: offset.y + 10.0 + 32.0 + 40.0,
            w: JOB_CARD_WIDTH - 20.0,
            h: 30.0,
        },
        font_size: 16.0,
        font: assets.fonts.text.clone(),
        text: "Select Category".to_string(),
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        intent: Intent::ChangeJobSlotState(
            job_slot_index,
            JobSlotState::PickingCategory,
        ),
        parent_clip: None,
        border_style: BorderStyle::None,
    });

    elements
}

fn category_selection_ui(job_slot_index: usize, assets: &Assets, offset: Vec2, layout: &CardLayout) -> Vec<UiElement> {
    let mut elements = vec![];

    let CardLayout {padding_x, padding_y, spacing_x, spacing_y} = layout;

    // Add title: Select Category
    let title_font_size = 32.0;

    elements.push(UiElement::Text {
        content: "Select Category".to_string(),
        font: assets.fonts.text_bold.clone(),
        x: offset.x + padding_y,
        y: offset.y + padding_y + title_font_size,
        font_size: title_font_size,
        color: palette::TEXT.get_color(),
    });

    let number_of_categories = SkillCategory::iter().count();
    let button_height = JOB_CARD_HEIGHT - padding_y - title_font_size - padding_y - spacing_y;
    let button_width = (JOB_CARD_WIDTH - padding_x * 2.0 - spacing_x * (number_of_categories as f32 - 1.0)) / number_of_categories as f32;

    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + padding_x,
            y: offset.y + padding_y + title_font_size + spacing_y,
            w: button_width,
            h: button_height,
        },
        font_size: 16.0,
        font: assets.fonts.text.clone(),
        text: "Gathering".to_string(),
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        intent: Intent::ChangeJobSlotState(
            job_slot_index,
            JobSlotState::PickingSkill(SkillCategory::Gathering),
        ),
        parent_clip: None,
        border_style: BorderStyle::None,
    });

    // Add button for Crafting Category
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + padding_x + button_width + spacing_x,
            y: offset.y + padding_y + title_font_size + spacing_y,
            w: button_width,
            h: button_height,
        },
        font_size: 16.0,
        font: assets.fonts.text.clone(),
        text: "Crafting".to_string(),
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        intent: Intent::ChangeJobSlotState(
            job_slot_index,
            JobSlotState::PickingSkill(SkillCategory::Crafting),
        ),
        parent_clip: None,
        border_style: BorderStyle::None,
    });

    elements
}

pub fn skill_selection_ui(
    job_slot_index: usize,
    category: &SkillCategory,
    assets: &Assets,
    offset: Vec2,
) -> Vec<UiElement> {
    let mut elements = vec![];

    let title_font_size = 20.0;
    // Add title: Select Skill
    elements.push(UiElement::Text {
        content: format!("Select {} Skill", category.as_str()),
        font: assets.fonts.text_bold.clone(),
        x: offset.x + 10.0,
        y: offset.y + 10.0 + title_font_size,
        font_size: title_font_size,
        color: palette::TEXT.get_color(),
    });

    // buttons are quadratic. 4 should fit in a row.
    let button_spacing = 10.0;
    let padding_x = 10.0;
    let button_size = (JOB_CARD_WIDTH - padding_x * 2.0 - button_spacing * 3.0) / 4.0;


    // Add buttons for each skill in the category
    for (i, skill_archetype) in category.get_skill_archetypes().iter().enumerate() {
        // add small text above the button
        elements.push(UiElement::Text {
            content: skill_archetype.get_name().to_string(),
            font: assets.fonts.text.clone(),
            x: offset.x + padding_x + (i as f32 * (button_size + button_spacing)),
            y: offset.y + 60.0,
            font_size: 12.0,
            color: palette::TEXT.get_color(),
        });

        elements.push(UiElement::ImgButton {
            rectangle: UiRect::new(
                offset.x + padding_x + (i as f32 * (button_size + button_spacing)),
                offset.y + 64.0,
                button_size,
                button_size,
            ),
            intent: Intent::ChangeJobSlotState(
                job_slot_index,
                JobSlotState::PickingProduct(skill_archetype.clone()),
            ),
            texture: skill_archetype.get_icon_texture(assets),
            parent_clip: None,
            border_style: BorderStyle::None,
        });
    }

    elements
}

pub fn product_selection_ui(
    job_slot_index: usize,
    skill_archetype: &SkillArchetype,
    assets: &Assets,
    offset: Vec2,
) -> Vec<UiElement> {
    let mut elements = vec![];

    // Add title: Select Product
    elements.push(UiElement::Text {
        content: format!("Select Product for {}", skill_archetype.get_name()),
        font: assets.fonts.text_bold.clone(),
        x: offset.x + 10.0,
        y: offset.y + 10.0 + 32.0,
        font_size: 32.0,
        color: palette::TEXT.get_color(),
    });

    // Here you would add buttons for each product related to the skill archetype
    for (i, job_archetype) in skill_archetype.get_job_archetypes().iter().enumerate() {
        elements.push(UiElement::RectButton {
            rectangle: UiRect {
                x: offset.x + 10.0,
                y: offset.y + 60.0 + (i as f32 * 40.0),
                w: JOB_CARD_WIDTH - 20.0,
                h: 30.0,
            },
            font_size: 16.0,
            font: assets.fonts.text.clone(),
            text: job_archetype.get_name().clone(),
            background_color: palette::BUTTON_BACKGROUND.get_color(),
            text_color: palette::BUTTON_TEXT.get_color(),
            intent: Intent::ChangeJobSlotState(
                job_slot_index,
                JobSlotState::RunningJob(JobInstance::new(JobParameters{
                    job_archetype: job_archetype.clone(),
                })),
            ),
            parent_clip: None,
            border_style: BorderStyle::None,
        });
    }

    elements
}

pub fn job_card_ui(
    state: &GameState,
    assets: &Assets,
    job_instance: &JobInstance,
    job_slot_id: usize,
    offset: Vec2,
) -> Vec<UiElement> {
    let job = job_instance;
    let card_padding_x = 10.0;
    let card_padding_y = 10.0;
    let card_spacing = 10.0;
    let clip = None;

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

    let (image1, image2) = job.job_archetype.get_skill_type().get_animation_images(assets);

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
    if job.hyper_mode.has_enough_actions() {
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
            intent: Intent::EnableHyperMode(job_slot_id),
            border_style: BorderStyle::Solid,
        });
    }

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

    // Draw Skill instance level up progress bar
    let skill_progress_bar_width = card_width - card_padding_x - card_padding_x - right_side_width - card_spacing;
    let skill_progress_bar_height = 12.0;
    elements.push(UiElement::ProgressBar {
        x: offset.x + card_padding_x,
        y: offset.y + card_padding_y + 2.0,
        width: skill_progress_bar_width,
        height: font_size_large + 2.0,
        progress: skill_instance.actions_counter.level_up_progress.get(),
        background_color: palette::BAR_BACKGROUND.get_color(),
        foreground_color: palette::SKILL_COLOR.get_color(),
        border_style: BorderStyle::None,
    });

    // Draw Job instance level up progress bar
    elements.push(UiElement::ProgressBar {
        x: offset.x + card_padding_x,
        y: offset.y + card_padding_y + font_size_large + 6.0,
        width: skill_progress_bar_width,
        height: font_size_large + 2.0,
        progress: job_archetype_instance.action_counter.level_up_progress.get(),
        background_color: palette::BAR_BACKGROUND.get_color(),
        foreground_color: palette::PRODUCT_COLOR.get_color(),
        border_style: BorderStyle::None,
    });

    // Skill Type and Level
    elements.push(UiElement::Text {
        content: format!(
            "{} Lv. {} ({} / {})",
            skill_instance.skill_type.get_name(),
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
        intent: Intent::ChangeJobSlotState(job_slot_id, JobSlotState::Locked),
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