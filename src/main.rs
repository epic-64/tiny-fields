use macroquad::prelude::*;

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

struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rectangle {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    fn contains_point(&self, point: (f32, f32)) -> bool {
        point.0 >= self.x && point.0 <= self.x + self.width &&
        point.1 >= self.y && point.1 <= self.y + self.height
    }

    fn draw(&self, color: Color) {
        draw_rectangle(self.x, self.y, self.width, self.height, color);
    }
}

struct Button {
    rect: Rectangle,
    color: Color,
    hover_color: Color,
    label: String,
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, color: Color, hover_color: Color, label: &str) -> Self {
        Self {
            rect: Rectangle::new(x, y, width, height),
            color,
            hover_color,
            label: label.to_string(),
        }
    }

    fn draw(&self) {
        let color = if self.is_hovered() { self.hover_color } else { self.color };
        self.rect.draw(color);

        draw_text(&self.label, self.rect.x + 10.0, self.rect.y + 10.0, 20.0, BLACK);
    }

    fn is_hovered(&self) -> bool {
        let mouse = mouse_position();
        self.rect.contains_point(mouse)
    }

    fn is_clicked(&self) -> bool {
        self.is_hovered() && is_mouse_button_pressed(MouseButton::Left)
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
        draw_text(&format!("Wood: {}", state.wood), 20.0, 40.0, 30.0, WHITE);
        draw_text(&format!("Lumber Camps: {}", state.lumber_camps), 20.0, 80.0, 30.0, WHITE);

        // Build button
        let button = Button::new(10.0, 120.0, 240.0, 40.0, WHITE, GRAY, "BUTTON");
        button.draw();

        if button.is_clicked() {
            state.try_build_lumber_camp();
        }

        next_frame().await;
    }
}
