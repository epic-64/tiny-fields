use crate::assets::Assets;
use crate::draw::{BorderStyle, UiElement};
use crate::game::{Intent, UiRect};
use crate::job::{JobInstance, JOB_CARD_HEIGHT, JOB_CARD_WIDTH};
use crate::palette;
use crate::skill::{SkillArchetype, SkillCategory};
use strum::IntoEnumIterator;

#[derive(Clone, Debug)]
pub enum JobSlotState {
    Empty,
    PickingCategory,
    PickingSkill(SkillCategory),
    PickingProduct(SkillArchetype),
    RunningJob(JobInstance),
}

impl JobSlotState {
    pub fn build_ui(&self, job_slot_index: usize, assets: &Assets) -> Vec<UiElement> {
        match self {
            JobSlotState::Empty => empty_slot_ui(job_slot_index, assets),
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
    pub fn build_ui(&self, assets: &Assets) -> Vec<UiElement> {
        self.state.build_ui(self.index, assets)
    }
}

fn empty_slot_ui(job_slot_index: usize, assets: &Assets) -> Vec<UiElement> {
    let mut elements = vec![];

    elements.push(UiElement::Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
        color: palette::CARD_BACKGROUND.get_color(),
        border_style: BorderStyle::None,
    });

    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: 10.0,
            y: 10.0,
            w: JOB_CARD_WIDTH,
            h: JOB_CARD_HEIGHT,
        },
        font_size: 0.0,
        font: assets.fonts.text.clone(),
        text: "Click to pick category".to_string(),
        background_color: Default::default(),
        text_color: Default::default(),
        intent: Intent::ChangeJobSlotState(job_slot_index, JobSlotState::PickingCategory),
        parent_clip: None,
        border_style: BorderStyle::None,
    });

    elements
}