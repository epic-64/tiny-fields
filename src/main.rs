use macroquad::prelude::*;
use std::time::Instant;

mod draw;
pub mod game;

use crate::draw::{draw_multiple, DrawCommand};
use crate::game::{Assets, GameState, Intent, Job, UiRect};

#[macroquad::main("Tiny Fields")]
async fn main() {
    set_pc_assets_folder("assets");
    request_new_screen_size(1600.0, 900.0);

    let wood_1: Texture2D = load_texture("ChopChop_1.png").await.expect("Couldn't load file");
    let wood_2: Texture2D = load_texture("ChopChop_2.png").await.expect("Couldn't load file");

    let assets = Assets {
        wood_1,
        wood_2,
    };

    let mut state = GameState::new();

    let mut ui = Ui2 {
        last_mouse_position: Vec2::new(0.0, 0.0),
        global_offset: Vec2::new(0.0, 0.0),
    };

    loop {
        let frame_start = Instant::now();
        let dt = get_frame_time();

        ui.update_offset();

        clear_background(ORANGE);
        let intents = ui.run(&state, &assets);

        // Update game state
        state.step(&intents, dt);

        // Keep track of FPS
        state.game_meta.raw_fps = 1.0 / frame_start.elapsed().as_secs_f32();
        state.game_meta.effective_fps = get_fps() as f32;

        next_frame().await;
    }
}

pub struct JobDrawContainer {
    job: usize,
    draw_commands: Vec<DrawCommand>,
}

impl JobDrawContainer {
    pub fn get_intents(&self) -> Vec<Intent> {
        let mut intents = vec![];

        for command in &self.draw_commands {
            match command {
                DrawCommand::Button { x, y, width, height, .. } => {
                    let rectangle = UiRect { x: *x, y: *y, width: *width, height: *height };

                    if rectangle.is_clicked() {
                        intents.push(Intent::ToggleJob(self.job));
                    }
                }
                _ => {}
            }
        }

        intents
    }
}

struct Ui2 {
    last_mouse_position: Vec2,
    global_offset: Vec2,
}

impl Ui2 {
    pub fn update_offset(&mut self) {
        if is_mouse_button_pressed(MouseButton::Right) {
            self.last_mouse_position = Vec2::from(mouse_position());
        }

        if is_mouse_button_down(MouseButton::Right) {
            let current_mouse_pos = Vec2::from(mouse_position());
            let delta = current_mouse_pos - self.last_mouse_position;

            if delta.length_squared() > 0.0 {
                let new_offset = {self.global_offset + delta}.clamp(
                    Vec2::new(-200.0, -600.0),
                    Vec2::new(1000.0, 600.0),
                );

                self.global_offset = new_offset;
            }

            self.last_mouse_position = current_mouse_pos;
        }
    }
}

impl Ui2 {
    pub fn run(&mut self, state: &GameState, assets: &Assets) -> Vec<Intent> {
        let mut intents = vec![];

        let job_draw_containers: Vec<JobDrawContainer> = self.get_job_draw_containers(state, assets);

        for container in &job_draw_containers {
            draw_multiple(&container.draw_commands); // side effects: draw to scene
            intents.extend(container.get_intents())  // collect inputs based on screen state
        }

        intents
    }

    fn get_job_draw_containers(&self, state: &GameState, assets: &Assets) -> Vec<JobDrawContainer>
    {
        let mut job_draw_containers = vec![];

        let mut job_offset = Vec2::new(50.0, 50.0);
        let card_height = 170.0;
        let card_spacing = 10.0;
        let card_padding = 26.0;

        for (id, job) in state.jobs.iter().enumerate() {
            let job_draw_container = get_job_draw_container(
                assets,
                id,
                job,
                self.global_offset + job_offset,
                card_height,
                card_padding,
                card_spacing,
            );
            job_draw_containers.push(job_draw_container);

            job_offset += Vec2::new(0.0, card_height as f32 + 15.0);
        }

        job_draw_containers
    }
}

pub fn get_job_draw_container(
    assets: &Assets,
    job_id: usize,
    job: &Job,
    offset: Vec2,
    card_height: f64,
    card_padding: f32,
    card_spacing: f32,
) -> JobDrawContainer
{
    let color_card = Color::from_rgba(50, 50, 50, 255);
    let color_primary = WHITE;
    let color_secondary = LIGHTGRAY;
    let color_button = DARKGRAY;
    let color_button_hover = SKYBLUE;

    let font_size_large = 24.0;
    let font_size_small = 20.0;

    let card_width = 500.0;
    let image_width = 100.0f32;
    let inner_x = offset.x + card_padding + image_width + card_spacing;
    let progress_bar_width = card_width - card_padding - image_width - card_spacing - card_padding;
    let button_width = 80.0;

    let chosen_image = if job.running && job.time_accumulator % 2.0 < 1.0 {
        assets.wood_2.clone()
    } else {
        assets.wood_1.clone()
    };

    let commands = vec![
        // Background
        DrawCommand::Rectangle {
            x: offset.x,
            y: offset.y,
            width: card_width as f64,
            height: card_height,
            color: color_card
        },

        // Job Animation
        DrawCommand::Image {
            x: offset.x + card_padding,
            y: offset.y,
            width: image_width as f64,
            height: card_height,
            texture: chosen_image,
        },

        // Title Bar
        DrawCommand::Text {
            content: job.name.clone() + " ",
            x: inner_x,
            y: offset.y + card_padding + 15.0,
            font_size: font_size_large,
            color: color_primary,
        },

        // Job Info
        DrawCommand::Text {
            content: format!("Lvl {} | ${} | {}s | {} Slots", job.level, job.money_per_action(), job.action_duration, job.timeslot_cost),
            x: inner_x,
            y: offset.y + 72.0,
            font_size: font_size_small,
            color: color_secondary,
        },

        // Action Progress Bar
        DrawCommand::ProgressBar {
            x: inner_x,
            y: offset.y + 96.0,
            width: progress_bar_width,
            height: 20.0,
            progress: job.action_progress.get(),
            background_color: GRAY,
            foreground_color: GREEN,
        },

        // Action Progress Text
        DrawCommand::Text {
            content: format!("{:.1} / {:.1}", job.time_accumulator, job.action_duration),
            x: inner_x + 10.0,
            y: offset.y + 111.0,
            font_size: 20.0,
            color: WHITE,
        },

        // Level Up Progress Bar
        DrawCommand::ProgressBar {
            x: inner_x,
            y: offset.y + 126.0,
            width: progress_bar_width,
            height: 20.0,
            progress: job.level_up_progress.get(),
            background_color: GRAY,
            foreground_color: BLUE,
        },

        // Level Up Progress Text
        DrawCommand::Text {
            content: format!("Level Up: {} / {}", job.actions_done, job.actions_to_level_up()),
            x: inner_x + 10.0,
            y: offset.y + 141.0,
            font_size: 20.0,
            color: WHITE,
        },

        // Start / Stop Button
        DrawCommand::Button {
            x: offset.x + card_width - button_width - card_padding,
            y: offset.y + card_padding,
            width: button_width,
            font_size: font_size_large,
            height: 46.0,
            text: if job.running { "Stop".to_string() } else { "Start".to_string() },
            color: color_button,
            hover_color: color_button_hover,
        }
    ];

    JobDrawContainer { job: job_id, draw_commands: commands }
}