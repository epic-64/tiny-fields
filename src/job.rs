use crate::draw::UiElement;
use crate::game::{Assets, GameState, Intent, JobInstance, UiRect};
use macroquad::math::Vec2;
use crate::palette;
use crate::palette::Palette;

pub const JOB_CARD_HEIGHT: f32 = 192.0;
pub const JOB_CARD_WIDTH: f32 = 384.0;
pub const JOB_CARD_SPACING_OUTER: f32 = 5.0;

pub fn build_job_cards(state: &GameState, assets: &Assets, offset: Vec2) -> Vec<UiElement>
{
    let mut elements: Vec<UiElement> = vec![];

    let mut container_offset = offset;
    let mut offset_x = offset.x;
    let mut offset_y = offset.y;

    let card_height = JOB_CARD_HEIGHT;
    let card_width = JOB_CARD_WIDTH;
    let card_spacing_inner = 8.0;
    let card_padding_x = 8.0;
    let card_padding_y = 8.0;

    for (id, job) in state.jobs.iter().enumerate() {
        let job_draw_container = build_job_card(
            &None, // No clipping for the job cards
            assets,
            job,
            id,
            container_offset,
            card_height,
            card_width,
            card_padding_x,
            card_padding_y,
            card_spacing_inner,
        );

        elements.extend(job_draw_container);

        if (id + 1) % 3 == 0 && id != 0 {
            offset_x = offset.x; // Reset horizontal offset for the new row
            offset_y += card_height + JOB_CARD_SPACING_OUTER;
        } else {
            offset_x += card_width + card_spacing_inner;
        }

        container_offset = Vec2::new(offset_x, offset_y);
    }

    elements
}

pub fn build_job_card(
    clip: &Option<(i32, i32, i32, i32)>,
    assets: &Assets,
    job: &JobInstance,
    job_id: usize,
    offset: Vec2,
    card_height: f32,
    card_width: f32,
    card_padding_x: f32,
    card_padding_y: f32,
    card_spacing: f32,
) -> Vec<UiElement>
{
    let color_primary = palette::TEXT.get_color();
    let color_secondary = palette::BORDER.get_color();
    let color_button = palette::BUTTON_BACKGROUND.get_color();

    let font_size_large = 20.0;
    let font_size_small = 14.0;

    let image_width = 90.0f32;
    let image_height = 120.0f32;
    let image_x = offset.x + card_padding_x;
    let image_y = offset.y + card_height - image_height - card_padding_y;

    let button_width = 90.0;
    let inner_x = offset.x + card_padding_x + image_width + card_spacing;

    let (image1, image2) = job.job_type.get_images(assets);

    let chosen_image = if job.running && job.time_accumulator % 2.0 < 1.0 {
        image1
    } else {
        image2
    };

    let mut elements = vec![];

    // Job Marker
    elements.push(UiElement::JobParticleMarker {
        x: offset.x + 320.0,
        y: offset.y + 75.0,
        job: job.clone(),
    });
    
    // Background
    elements.push(UiElement::Rectangle {
        x: offset.x,
        y: offset.y,
        width: card_width,
        height: card_height,
        color: palette::CARD_BACKGROUND.get_color(),
        bordered: false,
    });

    // Job Animation
    // Job animation background
    elements.push(UiElement::Rectangle {
        x: image_x,
        y: image_y,
        width: image_width,
        height: image_height,
        color: palette::IMAGE_BACKGROUND.get_color(),
        bordered: true,
    });

    let image_padding = 12.0;
    elements.push(UiElement::Image {
        x: image_x + image_padding,
        y: image_y + image_padding,
        width: image_width - image_padding * 2.0,
        height: image_height - image_padding * 2.0,
        texture: chosen_image.clone(),
        color: Palette::White.get_color(),
    });

    // Title Bar
    elements.push(UiElement::Text {
        content: job.job_type.get_name(),
        font: assets.fonts.main.clone(),
        x: inner_x,
        y: offset.y + card_padding_y + font_size_large,
        font_size: font_size_large,
        color: color_primary,
    });

    // Job Info
    elements.push(UiElement::Text {
        content: format!("Lvl {} | {}s", job.level, job.job_type.base_duration()),
        font: assets.fonts.main.clone(),
        x: inner_x,
        y: offset.y + card_padding_y + font_size_large + 28.0,
        font_size: font_size_small,
        color: color_secondary,
    });


    let progress_bar_width = card_width - card_padding_x - image_width - card_spacing
        - button_width - card_spacing - card_padding_x;
    let progress_bar_height = 20.0;
    let progress_bar_action_y = offset.y + 85.0;
    // Action Progress Bar
    elements.push(UiElement::ProgressBar {
        x: inner_x,
        y: progress_bar_action_y,
        width: progress_bar_width,
        height: progress_bar_height,
        progress: job.action_progress.get(),
        background_color: palette::BAR_BACKGROUND.get_color(),
        foreground_color: palette::PROGRESS_COLOR.get_color(),
    });

    // Action Progress Text
    elements.push(UiElement::Text {
        content: format!("{:.1} / {:.1}", job.time_accumulator, job.job_type.base_duration()),
        font: assets.fonts.main.clone(),
        x: inner_x + 10.0,
        y: progress_bar_action_y + 15.0,
        font_size: font_size_small,
        color: palette::TEXT.get_color(),
    });

    let progress_bar_level_y = progress_bar_action_y + progress_bar_height + 5.0;

    // Level Up Progress Bar
    elements.push(UiElement::ProgressBar {
        x: inner_x,
        y: progress_bar_level_y,
        width: progress_bar_width,
        height: progress_bar_height,
        progress: job.level_up_progress.get(),
        background_color: palette::BAR_BACKGROUND.get_color(),
        foreground_color: palette::PROGRESS_COLOR.get_color(),
    });

    // Level Up Progress Text
    elements.push(UiElement::Text {
        content: format!("Level Up: {} / {}", job.actions_done, job.actions_to_level_up()),
        font: assets.fonts.main.clone(),
        x: inner_x + 10.0,
        y: progress_bar_level_y + 15.0,
        font_size: font_size_small,
        color: palette::TEXT.get_color(),
    });

    // Start / Stop Button
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + card_width - button_width - card_padding_x,
            y: offset.y + card_padding_y,
            w: button_width,
            h: 50.0,
        },
        font: assets.fonts.main.clone(),
        parent_clip: clip.clone(),
        font_size: font_size_small,
        text: if job.running { "Stop".to_string() } else { "Start".to_string() },
        color: color_button,
        intent: Intent::ToggleJob(job_id),
    });

    elements
}