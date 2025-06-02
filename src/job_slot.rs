use macroquad::prelude::Vec2;
use crate::assets::Assets;
use crate::draw::{BorderStyle, UiElement};
use crate::game::{GameState, Intent, UiRect};
use crate::job::{build_job_card, JobArchetype, JobInstance, JobParameters, JOB_CARD_HEIGHT, JOB_CARD_SPACING_OUTER, JOB_CARD_WIDTH};
use crate::palette;
use crate::skill::{SkillArchetype, SkillCategory};
use crate::assets::AssetId::BackgroundParchment;

#[derive(Clone, Debug)]
pub enum JobSlotState {
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

        match self {
            JobSlotState::Empty => category_selection_ui(job_slot_index, assets, offset),
            JobSlotState::PickingSkill(category) => {
                skill_selection_ui(job_slot_index, category, assets, offset)
            }
            JobSlotState::PickingProduct(skill_archetype) => {
                product_selection_ui(job_slot_index, skill_archetype, assets, offset)
            }
            JobSlotState::RunningJob(job_instance) => {
                job_ui(&state, assets, job_instance, job_slot_index, offset)
            }
            default => {
                // Handle other states if needed
                vec![]
            }
        }
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

fn category_selection_ui(job_slot_index: usize, assets: &Assets, offset: Vec2) -> Vec<UiElement> {
    let mut elements = vec![];

    elements.push(UiElement::Image {
        x: offset.x,
        y: offset.y,
        width: JOB_CARD_WIDTH,
        height: JOB_CARD_HEIGHT,
        texture: BackgroundParchment.texture(&assets),
        color: palette::CARD_BACKGROUND.get_color(),
    });

    // Add title: Select Category
    elements.push(UiElement::Text {
        content: "Select Category".to_string(),
        font: assets.fonts.text_bold.clone(),
        x: offset.x + 10.0,
        y: offset.y + 10.0 + 32.0,
        font_size: 32.0,
        color: palette::TEXT.get_color(),
    });

    // Add button for Gathering Category
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + 10.0,
            y: offset.y + 10.0 + 32.0 + 40.0,
            w: 100.0,
            h: 30.0,
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
            x: offset.x + 120.0,
            y: offset.y + 10.0 + 32.0 + 40.0,
            w: 100.0,
            h: 30.0,
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

    elements.push(UiElement::Image {
        x: offset.x,
        y: offset.y,
        width: JOB_CARD_WIDTH,
        height: JOB_CARD_HEIGHT,
        texture: BackgroundParchment.texture(&assets),
        color: palette::CARD_BACKGROUND.get_color(),
    });

    // Add title: Select Skill
    elements.push(UiElement::Text {
        content: format!("Select Skill for {}", category.as_str()),
        font: assets.fonts.text_bold.clone(),
        x: offset.x + 10.0,
        y: offset.y + 10.0 + 32.0,
        font_size: 32.0,
        color: palette::TEXT.get_color(),
    });

    // Add buttons for each skill in the category
    for (i, skill_archetype) in category.get_skill_archetypes().iter().enumerate() {
        elements.push(UiElement::RectButton {
            rectangle: UiRect {
                x: offset.x + 10.0,
                y: offset.y + 60.0 + (i as f32 * 40.0),
                w: JOB_CARD_WIDTH - 20.0,
                h: 30.0,
            },
            font_size: 16.0,
            font: assets.fonts.text.clone(),
            text: skill_archetype.as_str().to_string(),
            background_color: palette::BUTTON_BACKGROUND.get_color(),
            text_color: palette::BUTTON_TEXT.get_color(),
            intent: Intent::ChangeJobSlotState(
                job_slot_index,
                JobSlotState::PickingProduct(skill_archetype.clone()),
            ),
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

    elements.push(UiElement::Image {
        x: offset.x,
        y: offset.y,
        width: JOB_CARD_WIDTH,
        height: JOB_CARD_HEIGHT,
        texture: BackgroundParchment.texture(&assets),
        color: palette::CARD_BACKGROUND.get_color(),
    });

    // Add title: Select Product
    elements.push(UiElement::Text {
        content: format!("Select Product for {}", skill_archetype.as_str()),
        font: assets.fonts.text_bold.clone(),
        x: offset.x + 10.0,
        y: offset.y + 10.0 + 32.0,
        font_size: 32.0,
        color: palette::TEXT.get_color(),
    });

    // Here you would add buttons for each product related to the skill archetype
    // For now, we will just add a placeholder button
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + 10.0,
            y: offset.y + 60.0,
            w: JOB_CARD_WIDTH - 20.0,
            h: 30.0,
        },
        font_size: 16.0,
        font: assets.fonts.text.clone(),
        text: "Placeholder Product".to_string(),
        background_color: palette::BUTTON_BACKGROUND.get_color(),
        text_color: palette::BUTTON_TEXT.get_color(),
        intent: Intent::ChangeJobSlotState(
            job_slot_index,
            JobSlotState::RunningJob(JobInstance::new(JobParameters{
                instance_id: job_slot_index as i32,
                job_archetype: JobArchetype::LumberingWood,
            })),
        ),
        parent_clip: None,
        border_style: BorderStyle::None,
    });

    elements
}

pub fn job_ui(
    state: &GameState,
    assets: &Assets,
    job_instance: &JobInstance,
    job_id: usize,
    offset: Vec2,
) -> Vec<UiElement> {
    build_job_card(
        &state,
        &None,
        &assets,
        &job_instance,
        job_id,
        offset,
        JOB_CARD_HEIGHT,
        JOB_CARD_WIDTH,
        10.0,
        10.0,
        JOB_CARD_SPACING_OUTER,
    )
}