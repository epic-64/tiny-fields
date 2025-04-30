use macroquad::color::{Color, BLUE, DARKGRAY, GRAY, GREEN, LIGHTGRAY, WHITE};
use crate::draw::DrawCommand;
use crate::GameState;
use crate::layout::JobLayout;
use crate::my_lib::{Button, Job};

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

    pub fn render(&self, job: &Job, layout: &JobLayout) -> Vec<DrawCommand> {
        let mut commands = vec![];

        // Card background
        commands.push(DrawCommand::Rectangle {
            x: layout.card_rect.x,
            y: layout.card_rect.y,
            width: layout.card_rect.width as f64,
            height: layout.card_rect.height as f64,
            color: Self::BACKGROUND_COLOR,
        });

        // Job name
        commands.push(DrawCommand::Text {
            content: format!("Job: {}", job.name),
            x: layout.card_rect.x + Self::CARD_PADDING,
            y: layout.card_rect.y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE,
            font_size: Self::TEXT_FONT_SIZE_LARGE,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Info Line
        commands.push(DrawCommand::Text {
            content: format!(
                "Lvl {} | ${} | {}s | Slots: {}",
                job.level, job.dollars_per_action(), job.action_duration, job.timeslot_cost
            ),
            x: layout.card_rect.x + Self::CARD_PADDING,
            y: layout.card_rect.y + Self::CARD_PADDING + Self::TEXT_FONT_SIZE_LARGE + Self::CARD_SPACING,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_SECONDARY,
        });

        // Action progress bar
        commands.push(DrawCommand::ProgressBar {
            x: layout.action_bar_rect.x,
            y: layout.action_bar_rect.y,
            width: layout.action_bar_rect.width,
            height: layout.action_bar_rect.height,
            progress: job.action_progress.progress.get(),
            background_color: Self::PROGRESS_BAR_BACKGROUND,
            foreground_color: Self::PROGRESS_BAR_FOREGROUND_ACTION,
        });

        // Text inside the action progress bar
        commands.push(DrawCommand::Text {
            content: format!("{:.1} / {:.1}", job.time_accumulator, job.action_duration),
            x: layout.action_bar_rect.x + 10.0,
            y: layout.action_bar_rect.y + 15.0,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Level-up progress bar
        commands.push(DrawCommand::ProgressBar {
            x: layout.level_bar_rect.x,
            y: layout.level_bar_rect.y,
            width: layout.level_bar_rect.width,
            height: layout.level_bar_rect.height,
            progress: job.level_up_progress.progress.get(),
            background_color: Self::PROGRESS_BAR_BACKGROUND,
            foreground_color: Self::PROGRESS_BAR_FOREGROUND_LEVEL,
        });

        // Text inside the level-up progress bar
        commands.push(DrawCommand::Text {
            content: format!("Level Up: {} / {}", job.actions_done, job.actions_to_level_up()),
            x: layout.level_bar_rect.x + 10.0,
            y: layout.level_bar_rect.y + 15.0,
            font_size: Self::TEXT_FONT_SIZE_SMALL,
            color: Self::TEXT_COLOR_PRIMARY,
        });

        // Control button using layout's button_rect
        commands.push(DrawCommand::Button {
            button: Button{
                rect: layout.button_rect.clone(),
                color: job.control_button.color,
                hover_color: job.control_button.hover_color,
                label: job.control_button.label.clone(),
            },
        });

        commands
    }
}

// Return a vector of draw commands. Pure function
pub fn render(state: &GameState, layout: &[JobLayout]) -> Vec<DrawCommand> {
    let mut commands = vec![];

    // Display top-level info
    commands.push(DrawCommand::Text {
        content: format!("Money: ${}", state.total_money),
        x: 20.0,
        y: 20.0,
        font_size: 30.0,
        color: WHITE,
    });

    // Display timeslots
    commands.push(DrawCommand::Text {
        content: format!("Timeslots: {} / {}", state.time_slots.get_free(), state.time_slots.total),
        x: 20.0,
        y: 60.0,
        font_size: 30.0,
        color: WHITE,
    });

    // Display FPS
    commands.push(DrawCommand::Text {
        content: format!("FPS: {}", state.game_meta.effective_fps),
        x: 20.0,
        y: 100.0,
        font_size: 30.0,
        color: WHITE,
    });

    // Display raw FPS
    commands.push(DrawCommand::Text {
        content: format!("Raw FPS: {:.2}", state.game_meta.raw_fps),
        x: 20.0,
        y: 140.0,
        font_size: 30.0,
        color: WHITE,
    });

    // Use JobRenderer for each job
    let job_renderer = JobRenderer {};
    for layout in layout {
        let job = &state.jobs[layout.job_index];
        commands.extend(job_renderer.render(job, layout));
    }

    commands
}