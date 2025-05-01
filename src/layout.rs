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
        let card_padding = 20.0;
        let card_x = 50.0;
        let card_y = y_offset;
        let card_w = 400.0;
        let card_h = 175.0;

        layouts.push(JobLayout {
            job_index: i,
            card_rect: Rectangle::new(card_x, card_y, card_w, card_h),
            button_rect: Rectangle{
                x: card_x + card_w - card_padding - 90.0,
                y: card_y + card_padding,
                width: 90.0,
                height: 40.0,
            },
            action_bar_rect: Rectangle{
                x: card_x + card_padding,
                y: card_y + card_padding + 80.0,
                width: card_w - card_padding * 2.0,
                height: 20.0,
            },
            level_bar_rect: Rectangle{
                x: card_x + card_padding,
                y: card_y + card_padding + 110.0,
                width: card_w - card_padding * 2.0,
                height: 20.0,
            }
        });

        y_offset += 200.0;
    }

    layouts
}