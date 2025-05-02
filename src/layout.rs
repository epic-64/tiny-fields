use macroquad::math::Vec2;
use crate::GameState;
use crate::my_lib::Rectangle;

pub struct JobLayout {
    pub card_rect: Rectangle,
    pub button_rect: Rectangle,
    pub action_bar_rect: Rectangle,
    pub level_bar_rect: Rectangle,
    pub job_index: usize,
    pub offset: Vec2,
}

impl JobLayout {
    pub fn new(
        job_index: usize,
        offset: Vec2,
    ) -> Self {
        let card_padding = 20.0;
        let card_x = 50.0 + offset.x;
        let card_y = 0.0 + offset.y;
        let card_w = 400.0;
        let card_h = 175.0;

        Self {
            job_index,
            card_rect: Rectangle::new(card_x, card_y, card_w, card_h),
            button_rect: Rectangle {
                x: card_x + card_w - card_padding - 90.0,
                y: card_y + card_padding,
                width: 90.0,
                height: 40.0,
            },
            action_bar_rect: Rectangle {
                x: card_x + card_padding,
                y: card_y + card_padding + 80.0,
                width: card_w - card_padding * 2.0,
                height: 20.0,
            },
            level_bar_rect: Rectangle {
                x: card_x + card_padding,
                y: card_y + card_padding + 110.0,
                width: card_w - card_padding * 2.0,
                height: 20.0,
            },
            offset,
        }
    }
}

pub fn layout(state: &GameState) -> Vec<JobLayout> {
    let mut layouts = vec![];
    let mut y_offset = 200.0;

    for (i, _job) in state.jobs.iter().enumerate() {
        layouts.push(JobLayout::new(i, Vec2::new(0., y_offset)));

        y_offset += 200.0;
    }

    layouts
}