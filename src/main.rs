use macroquad::prelude::*;

mod my_lib;
use my_lib::*;

struct GameState {
    wood: i32,
    lumber_camps: i32,
    time_accumulator: f32,
    build_button: Button,
}

impl GameState {
    fn new() -> Self {
        Self {
            wood: 0,
            lumber_camps: 0,
            time_accumulator: 0.0,
            build_button: Button::new(10.0, 120.0, 240.0, 40.0, WHITE, GRAY, "Build Lumber Camp (10)"),
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

// Step logic (tick + inputs)
fn step(state: &mut GameState, dt: f32) {
    state.time_accumulator += dt;
    if state.time_accumulator >= 1.0 {
        state.tick();
        state.time_accumulator -= 1.0;
    }

    if state.build_button.is_clicked() {
        state.try_build_lumber_camp();
    }
}

// Render into draw commands
fn render(state: &GameState) -> Vec<DrawCommand> {
    vec![
        DrawCommand::Text {
            content: format!("Wood: {}", state.wood),
            x: 20.0,
            y: 40.0,
            font_size: 30.0,
            color: WHITE,
        },
        DrawCommand::Text {
            content: format!("Lumber Camps: {}", state.lumber_camps),
            x: 20.0,
            y: 80.0,
            font_size: 30.0,
            color: WHITE,
        },
        DrawCommand::Button {
            button: state.build_button.clone(),
        },
    ]
}

// Main draw loop
#[macroquad::main("Tiny Fields")]
async fn main() {
    let mut state = GameState::new();

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        step(&mut state, dt);

        let commands = render(&state);
        draw(&commands);

        next_frame().await;
    }
}
