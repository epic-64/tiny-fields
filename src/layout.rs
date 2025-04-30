use crate::GameState;
use crate::my_lib::Rectangle;

pub struct JobLayout {
    pub card_rect: Rectangle,
    pub button_rect: Rectangle,
    pub action_bar_rect: Rectangle,
    pub level_bar_rect: Rectangle,
    pub job_index: usize,
}

pub fn layout(state: &GameState) -> Vec<JobLayout> {
    let mut layouts = vec![];
    let mut y_offset = 200.0;

    for (i, _job) in state.jobs.iter().enumerate() {
        let card_x = 50.0;
        let card_y = y_offset;
        let card_w = 400.0;
        let card_h = 180.0;

        layouts.push(JobLayout {
            job_index: i,
            card_rect: Rectangle::new(card_x, card_y, card_w, card_h),
            button_rect: Rectangle::new(card_x + 200.0, card_y + 30.0, 100.0, 30.0),
            action_bar_rect: Rectangle::new(card_x + 10.0, card_y + 140.0, 300.0, 20.0),
            level_bar_rect: Rectangle::new(card_x + 10.0, card_y + 170.0, 300.0, 20.0),
        });

        y_offset += 240.0;
    }

    layouts
}