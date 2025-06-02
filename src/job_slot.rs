use macroquad::prelude::Vec2;
use crate::assets::Assets;
use crate::draw::{BorderStyle, UiElement};
use crate::game::{Intent, UiRect};
use crate::job::{JobInstance, JOB_CARD_HEIGHT, JOB_CARD_SPACING_OUTER, JOB_CARD_WIDTH};
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
    pub fn build_ui(&self, job_slot_index: usize, assets: &Assets, offset: Vec2) -> Vec<UiElement> {
        match self {
            JobSlotState::Empty => empty_slot_ui(job_slot_index, assets, offset),
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
    pub fn build_ui(&self, assets: &Assets, offset: Vec2) -> Vec<UiElement> {
        self.state.build_ui(self.index, assets, offset)
    }
}

fn empty_slot_ui(job_slot_index: usize, assets: &Assets, offset: Vec2) -> Vec<UiElement> {
    let mut elements = vec![];

    let column = job_slot_index % 3;
    let row = job_slot_index / 3;

    let offset = Vec2::new(
        offset.x + (column as f32 * JOB_CARD_WIDTH) + JOB_CARD_SPACING_OUTER * (column as f32),
        offset.y + (row as f32 * JOB_CARD_HEIGHT) + JOB_CARD_SPACING_OUTER * (row as f32),
    );

    elements.push(UiElement::Image {
        x: offset.x,
        y: offset.y,
        width: JOB_CARD_WIDTH,
        height: JOB_CARD_HEIGHT,
        texture: assets.textures.get(&BackgroundParchment).unwrap().clone(),
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
        intent: Intent::ChangeJobSlotState(job_slot_index, JobSlotState::PickingCategory),
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
        intent: Intent::ChangeJobSlotState(job_slot_index, JobSlotState::PickingCategory),
        parent_clip: None,
        border_style: BorderStyle::None,
    });

    elements
}