use macroquad::prelude::*;

mod my_lib;
use my_lib::*;

struct GameState {
    wood: i32,
    lumber_camps: i32,
    time_accumulator: f32,
}

impl GameState {
    fn new() -> Self {
        Self {
            wood: 0,
            lumber_camps: 0,
            time_accumulator: 0.0,
        }
    }

    fn tick(&mut self) {
        self.wood += 1 + self.lumber_camps;
    }

    fn try_build_lumber_camp(&mut self) {
        let cost = 10;
        if self.wood >= cost {
            self.wood -= cost;
            self.lumber_camps += 1;
        }
    }
}

#[macroquad::main("Tiny Idle Game")]
async fn main() {
    let mut state = GameState::new();

    loop {
        clear_background(BLACK);

        // Timing
        let dt = get_frame_time();
        state.time_accumulator += dt;

        if state.time_accumulator >= 1.0 {
            state.tick();
            state.time_accumulator -= 1.0;
        }

        // Display resources
        draw_text_primary(&format!("Wood: {}", state.wood), 20.0, 40.0);
        draw_text_primary(&format!("Lumber Camps: {}", state.lumber_camps), 20.0, 80.0);

        // Build button
        let button = Button::new(10.0, 120.0, 240.0, 40.0, WHITE, GRAY, "BUTTON");
        button.draw();

        if button.is_clicked() {
            state.try_build_lumber_camp();
        }

        next_frame().await;
    }
}
