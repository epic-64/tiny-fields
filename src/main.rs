use macroquad::prelude::*;
use std::time::Instant;

mod my_lib;
mod layout;
mod draw;
mod render;
pub mod game;

use crate::draw::{draw, draw_multiple, DrawCommand};
use crate::game::{Assets, GameState, Intent, UserInterface};
use crate::my_lib::{Job, Rectangle};

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
    // let mut ui = UserInterface::new(&state);
    let mut ui = Ui2 {
        last_mouse_position: Vec2::new(0.0, 0.0),
        global_offset: Vec2::new(0.0, 0.0),
    };

    loop {
        let frame_start = Instant::now();
        let dt = get_frame_time();

        clear_background(ORANGE);
        let intents = ui.run(&state);
        // let intents = ui.process_input();

        // if is_mouse_button_down(MouseButton::Right) {
        //     let current_mouse_pos = Vec2::from(mouse_position());
        //     let delta = current_mouse_pos - ui.last_mouse_position;
        //
        //     if delta.length_squared() > 0.0 {
        //         let new_offset = {ui.global_offset + delta}.clamp(
        //             Vec2::new(-200.0, -600.0),
        //             Vec2::new(1000.0, 600.0),
        //         );
        //
        //         ui = ui.recreate(&state, new_offset);
        //     }
        //
        //     ui.last_mouse_position = current_mouse_pos;
        // }

        // Update game state
        state.step(&intents, dt);

        // Compile list of draw commands
        //let commands = ui.render(&state, &assets);

        // Draw the game
        //draw_multiple(&commands);

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
                DrawCommand::Button2 { x, y, width, height, .. } => {
                    let rectangle = Rectangle { x: *x, y: *y, width: *width, height: *height };

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
    pub fn run(&mut self, state: &GameState) -> Vec<Intent> {
        let mut intents = vec![];

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

        let mut job_draw_containers: Vec<JobDrawContainer> = vec![];

        let mut job_offset = Vec2::new(0.0, 0.0);
        for (id, job) in state.jobs.iter().enumerate() {
            let job_draw_container = get_job_draw_container(id, job, self.global_offset + job_offset);
            job_draw_containers.push(job_draw_container);

            job_offset += Vec2::new(0.0, 200.0);
        }

        for container in &job_draw_containers {
            draw_multiple(&container.draw_commands); // side effects: draw to scene
            intents.extend(container.get_intents())  // collect inputs based on screen state
        }

        intents
    }
}

pub fn get_job_draw_container(job_id: usize, job: &Job, offset: Vec2) -> JobDrawContainer {
    let commands = vec![
        DrawCommand::Rectangle {
            x: offset.x,
            y: offset.y,
            width: 500.0,
            height: 175.0,
            color: Color::from_rgba(20, 20, 20, 200),
        },
        DrawCommand::Text {
            content: job.name.clone(),
            x: offset.x,
            y: offset.y,
            font_size: 20.0,
            color: WHITE,
        },
        DrawCommand::Text {
            content: format!("Level: {}", job.level),
            x: offset.x + 10.0,
            y: offset.y + 30.0,
            font_size: 20.0,
            color: WHITE,
        },
        DrawCommand::ProgressBar {
            x: offset.x + 10.0,
            y: offset.y + 60.0,
            width: 480.0,
            height: 20.0,
            progress: job.action_progress.get(),
            background_color: Color::from_rgba(200, 200, 200, 255),
            foreground_color: Color::from_rgba(0, 255, 0, 255),
        },
        DrawCommand::Button2 {
            x: offset.x + 10.0,
            y: offset.y + 90.0,
            width: 100.0,
            height: 30.0,
            text: if job.running { "Stop".to_string() } else { "Start".to_string() },
            color: Color::from_rgba(0, 255, 0, 255),
            hover_color: Color::from_rgba(0, 200, 0, 255),
        }
    ];

    JobDrawContainer { job: job_id, draw_commands: commands }
}

pub fn get_job_intents(commands: &[(usize, Vec<DrawCommand>)]) -> Vec<Intent> {
    let mut intents = vec![];

    for (job_id, commands) in commands {
        for command in commands {
            match command {
                DrawCommand::Button2 { x, y, width, height, .. } => {
                    let rectangle = Rectangle { x: *x, y: *y, width: *width, height: *height };

                    if rectangle.is_clicked() {
                        intents.push(Intent::ToggleJob(*job_id));
                    }
                }
                _ => {}
            }
        }
    }

    intents
}