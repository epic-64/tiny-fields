use crate::my_lib::Rectangle;

pub struct JobLayout {
    pub card_rect: Rectangle,
    pub button_rect: Rectangle,
    pub action_bar_rect: Rectangle,
    pub level_bar_rect: Rectangle,
    pub job_index: usize,
}