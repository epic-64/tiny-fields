use macroquad::math::Vec2;
use crate::game::GameState;
use crate::my_lib::Rectangle;

pub struct JobLayout {
    pub card: Rectangle,
    pub toggle_button: Rectangle,
    pub action_bar: Rectangle,
    pub level_bar: Rectangle,
    pub job_index: usize,
    pub offset: Vec2,
}

impl JobLayout {
    pub fn new(job_index: usize, offset: Vec2, ) -> Self {
        let card_padding = 20.0;
        let card_x = offset.x;
        let card_y = offset.y;
        let card_w = 400.0;
        let card_h = 175.0;

        Self {
            job_index,
            card: Rectangle::new(card_x, card_y, card_w, card_h),
            toggle_button: Rectangle {
                x: card_x + card_w - card_padding - 90.0,
                y: card_y + card_padding,
                width: 90.0,
                height: 40.0,
            },
            action_bar: Rectangle {
                x: card_x + card_padding,
                y: card_y + card_padding + 80.0,
                width: card_w - card_padding * 2.0,
                height: 20.0,
            },
            level_bar: Rectangle {
                x: card_x + card_padding,
                y: card_y + card_padding + 110.0,
                width: card_w - card_padding * 2.0,
                height: 20.0,
            },
            offset,
        }
    }
}

pub fn layout(state: &GameState, offset: Vec2) -> Vec<JobLayout> {
    let mut layouts = vec![];
    let x_offset = 50.0 + offset.x;
    let mut y_offset = 50.0 + offset.y;

    for (i, _job) in state.jobs.iter().enumerate() {
        layouts.push(JobLayout::new(i, Vec2::new(x_offset, y_offset)));

        y_offset += 200.0;
    }

    layouts
}