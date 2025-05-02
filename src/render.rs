use crate::draw::DrawCommand;
use crate::layout::JobLayout;
use crate::my_lib::{Job, Rectangle};
use macroquad::color::{Color, BLACK, BLUE, DARKGRAY, GRAY, GREEN, LIGHTGRAY, PINK, WHITE};
use macroquad::input::mouse_position;
use macroquad::math::Vec2;
use macroquad::prelude::{draw_text, draw_texture, draw_texture_ex, DrawTextureParams};
use crate::game::Assets;

#[derive(Clone)]
pub struct Button {
    pub rect: Rectangle,
    pub color: Color,
    pub hover_color: Color,
    pub label: String,
}

impl Button {
    pub fn draw(&self) {
        let color = if self.is_hovered() { self.hover_color } else { self.color };
        self.rect.draw(color);

        draw_text(&self.label, self.rect.x + 10.0, self.rect.y + 10.0, 20.0, BLACK);
    }

    pub fn is_hovered(&self) -> bool {
        let mouse = mouse_position();
        self.rect.contains_point(mouse)
    }
}

pub struct JobRenderer {}

impl JobRenderer {
    const CARD_PADDING: f32 = 20.0;
    const CARD_SPACING: f32 = 30.0;
    const TEXT_FONT_SIZE_LARGE: f32 = 24.0;
    const TEXT_FONT_SIZE_SMALL: f32 = 20.0;
    const BACKGROUND_COLOR: Color = DARKGRAY;
    const TEXT_COLOR_PRIMARY: Color = WHITE;
    const TEXT_COLOR_SECONDARY: Color = LIGHTGRAY;
    const PROGRESS_BAR_BACKGROUND: Color = GRAY;
    const PROGRESS_BAR_FOREGROUND_ACTION: Color = GREEN;
    const PROGRESS_BAR_FOREGROUND_LEVEL: Color = BLUE;

    pub fn render(&self, assets: &Assets, job: &Job, layout: &JobLayout) -> Vec<DrawCommand> {
        let mut commands = vec![];
        let image_width = 128.0f32;
        let card_inner_x = layout.card.x + Self::CARD_PADDING + image_width + Self::CARD_PADDING;
        let progress_bar_text_padding = 10.0;

        // Card background
        commands.push(DrawCommand::Rectangle {
            x: layout.card.x,
            y: layout.card.y,
            width: layout.card.width as f64,
            height: layout.card.height as f64,
            color: Self::BACKGROUND_COLOR,
        });

        commands.push(DrawCommand::Image {
            x: layout.card.x + Self::CARD_PADDING,
            y: layout.card.y,
            width: image_width as f64,
            height: layout.card.height as f64,
            texture: assets.wood_cutting_image_1.clone(),
        });

        // Job name
        commands.push(DrawCommand::Text {
            content: format!("Job: {}", job.name),
            x: card_inner_x,
            y: layout.card.y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE,
            font_size: Self::TEXT_FONT_SIZE_LARGE,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Info Line
        commands.push(DrawCommand::Text {
            content: format!(
                "Lvl {} | ${} | {}s | Slots: {}",
                job.level, job.dollars_per_action(), job.action_duration, job.timeslot_cost
            ),
            x: card_inner_x,
            y: layout.card.y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE + Self::CARD_SPACING,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_SECONDARY,
        });

        // Action progress bar
        commands.push(DrawCommand::ProgressBar {
            x: card_inner_x,
            y: layout.action_bar.y,
            width: layout.action_bar.width,
            height: layout.action_bar.height,
            progress: job.action_progress.get(),
            background_color: Self::PROGRESS_BAR_BACKGROUND,
            foreground_color: Self::PROGRESS_BAR_FOREGROUND_ACTION,
        });

        // Text inside the action progress bar
        commands.push(DrawCommand::Text {
            content: format!("{:.1} / {:.1}", job.time_accumulator, job.action_duration),
            x: card_inner_x + progress_bar_text_padding,
            y: layout.action_bar.y + 15.0,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Level-up progress bar
        commands.push(DrawCommand::ProgressBar {
            x: card_inner_x,
            y: layout.level_bar.y,
            width: layout.level_bar.width,
            height: layout.level_bar.height,
            progress: job.level_up_progress.get(),
            background_color: Self::PROGRESS_BAR_BACKGROUND,
            foreground_color: Self::PROGRESS_BAR_FOREGROUND_LEVEL,
        });

        // Text inside the level-up progress bar
        commands.push(DrawCommand::Text {
            content: format!("Level Up: {} / {}", job.actions_done, job.actions_to_level_up()),
            x: card_inner_x + progress_bar_text_padding,
            y: layout.level_bar.y + 15.0,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Control button using layout's button_rect
        commands.push(DrawCommand::Button {
            button: Button{
                rect: layout.toggle_button.clone(),
                color: PINK,
                hover_color: BLUE,
                label: if job.running { "Stop" } else { "Start" }.to_string(),
            },
        });

        commands
    }
}