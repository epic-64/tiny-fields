use crate::game::{MouseInput, UiRect};
use macroquad::prelude::*;

pub struct ScrollContainer {
    pub rect: UiRect,
    total_height: f32,
    scroll_offset: Vec2,
    last_mouse_position: Vec2,
}

impl ScrollContainer {
    pub fn new(rect: UiRect) -> Self {
        let height = rect.h;
        Self {
            rect,
            total_height: height,
            scroll_offset: Vec2::new(0.0, 0.0),
            last_mouse_position: Vec2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self, mouse_input: &MouseInput) {
        if self.rect.is_hovered(mouse_input) {
            self.handle_scroll()
        }

        // Move the container back to its boundary over time if it was scrolled too far
        // using exponential decay. Only if the mouse is not pressed.
        if !is_mouse_button_down(MouseButton::Left) {
            let factor = 0.1;
            let upper_bound = 0.0;

            if self.scroll_offset.y > upper_bound {
                self.scroll_offset.y = self.scroll_offset.y.lerp(upper_bound, factor);
                if (self.scroll_offset.y - upper_bound).abs() < 1.0 {
                    self.scroll_offset.y = upper_bound;
                }
            }

            let lower_bound = if self.total_height > self.rect.h {
                -self.total_height + self.rect.h
            } else {
                0.0
            };

            if self.scroll_offset.y < lower_bound {
                self.scroll_offset.y = self.scroll_offset.y.lerp(lower_bound, factor);
                if (self.scroll_offset.y - lower_bound).abs() < 1.0 {
                    self.scroll_offset.y = lower_bound;
                }
            }
        }
    }

    fn handle_scroll(&mut self) {
        let mouse_wheel_delta = clamp(mouse_wheel().1, -1.0, 1.0);

        if mouse_wheel_delta.abs() > 0.0 {
            self.scroll_offset.y += mouse_wheel_delta * 40.0;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.last_mouse_position = Vec2::from(mouse_position());
        }

        if is_mouse_button_down(MouseButton::Left) {
            let current_mouse_pos = Vec2::from(mouse_position());
            let delta = current_mouse_pos - self.last_mouse_position;

            if delta.length_squared() > 0.0 {
                let new_offset = self.scroll_offset + Vec2::new(0.0, delta.y);

                self.scroll_offset = new_offset;
            }

            self.last_mouse_position = current_mouse_pos;
        }
    }
}