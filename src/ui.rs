use macroquad::prelude::*;
use crate::draw::UiElement;
use crate::game::{Assets, GameState, MouseInput, UiRect};

pub struct ScrollContainer {
    pub rect: UiRect,
    scroll_offset: Vec2,
    last_mouse_position: Vec2,
}

impl ScrollContainer {
    pub fn new(rect: UiRect) -> Self {
        Self {
            rect,
            scroll_offset: Vec2::new(0.0, 0.0),
            last_mouse_position: Vec2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self, mouse_input: &MouseInput) {
        if self.rect.is_hovered(mouse_input) {
            self.handle_scroll()
        }
    }

    fn handle_scroll(&mut self) {
        let mouse_wheel_delta = clamp(mouse_wheel().1, -1.0, 1.0);

        if mouse_wheel_delta.abs() > 0.0 {
            self.scroll_offset.y += mouse_wheel_delta * 40.0;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.last_mouse_position = Vec2::from(mouse_position());
        }

        if is_mouse_button_down(MouseButton::Right) {
            let current_mouse_pos = Vec2::from(mouse_position());
            let delta = current_mouse_pos - self.last_mouse_position;

            if delta.length_squared() > 0.0 {
                let new_offset = self.scroll_offset + Vec2::new(0.0, delta.y);

                self.scroll_offset = new_offset;
            }

            self.last_mouse_position = current_mouse_pos;
        }
    }

    pub fn build(
        &self,
        state: &GameState,
        assets: &Assets,
        mouse_input: &MouseInput,
        build_ui_elements: fn(&GameState, &Assets, &MouseInput, &UiRect, Vec2) -> Vec<UiElement>
    ) -> Vec<UiElement>
    {
        let mut elements: Vec<UiElement> = vec![];

        // Create a clipping area for the scroll container
        let clip = Some((
            self.rect.x as i32,
            self.rect.y as i32,
            self.rect.w as i32,
            self.rect.h as i32,
        ));

        // Create a scissor rectangle for the clipping area
        elements.push(UiElement::Scissor { clip });

        let scrollable_pos = Vec2::new(self.rect.x, self.rect.y) + self.scroll_offset;
        elements.extend(build_ui_elements(state, assets, mouse_input, &self.rect, scrollable_pos));

        // Remove the clipping area
        elements.push(UiElement::Scissor { clip: None });

        elements
    }
}