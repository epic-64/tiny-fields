use crate::draw::{pill, UiElement};
use crate::game::{Assets, GameState, Intent, JobInstance, UiRect};
use macroquad::math::Vec2;
use crate::palette;
use crate::palette::Palette;

pub const JOB_CARD_HEIGHT: f32 = 192.0;
pub const JOB_CARD_WIDTH: f32 = 404.0;
pub const JOB_CARD_SPACING_OUTER: f32 = 8.0;

pub fn build_job_cards(state: &GameState, assets: &Assets, offset: Vec2) -> Vec<UiElement>
{
    let mut elements: Vec<UiElement> = vec![];

    let mut container_offset = offset;
    let mut offset_x = offset.x;
    let mut offset_y = offset.y;

    let card_height = JOB_CARD_HEIGHT;
    let card_width = JOB_CARD_WIDTH;
    let card_spacing_inner = 6.0;
    let card_padding_x = 12.0;
    let card_padding_y = 12.0;

    for (id, job) in state.jobs.iter().enumerate() {
        let job_draw_container = build_job_card(
            state,
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
            offset_x += card_width + JOB_CARD_SPACING_OUTER;
        }

        container_offset = Vec2::new(offset_x, offset_y);
    }

    elements
}

pub fn build_job_card(
    state: &GameState,
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
    let font_size_large = 16.0;
    let font_size_small = 14.0;

    let image_width = 90.0f32;
    let image_height = 120.0f32;
    let image_x = offset.x + card_padding_x;
    let image_y = offset.y + card_height - image_height - card_padding_y;
    let inner_x = offset.x + card_padding_x + image_width + card_spacing;

    let (image1, image2) = job.job_type.get_animation_images(assets);

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

    // Job Animation background
    elements.push(UiElement::Rectangle {
        x: image_x,
        y: image_y,
        width: image_width,
        height: image_height,
        color: palette::IMAGE_BACKGROUND.get_color(),
        bordered: true,
    });

    // Job Animation Image
    let image_padding = 12.0;
    elements.push(UiElement::Image {
        x: image_x + image_padding,
        y: image_y + image_padding,
        width: image_width - image_padding * 2.0,
        height: image_height - image_padding * 2.0,
        texture: chosen_image.clone(),
        color: Palette::White.get_color(),
    });

    let right_side_width = 64.0;

    // Draw HyperMode button on the right
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + card_width - right_side_width - card_padding_x,
            y: image_y,
            w: right_side_width,
            h: 30.0,
        },
        font: assets.fonts.text_bold.clone(),
        parent_clip: clip.clone(),
        font_size: font_size_small,
        text: "Hyper".to_string(),
        color: palette::TEXT.get_color(),
        intent: Intent::ToggleHyperMode(job_id),
    });

    // Draw Product Image on the right
    elements.push(UiElement::Rectangle {
        x: offset.x + card_width - right_side_width - card_padding_x,
        y: image_y + 40.0,
        width: right_side_width,
        height: right_side_width,
        color: palette::PRODUCT_COLOR.get_color(),
        bordered: true,
    });

    // Draw Product Image
    elements.push(UiElement::Image {
        x: offset.x + card_width - right_side_width - card_padding_x + 8.0,
        y: image_y + 40.0 + 8.0,
        width: right_side_width - 16.0,
        height: right_side_width - 16.0,
        texture: job.job_type.get_product().get_texture(&assets),
        color: Palette::White.get_color(),
    });

    // Draw Product Pill at the top of the rectangle
    elements.extend(
        pill(
            offset.x + card_width - right_side_width - card_padding_x + right_side_width / 2.0 - 24.0 / 2.0,
            image_y + 40.0 - 14.0 / 2.0,
            24.0,
            14.0,
            state.inventory.get_item_amount(job.job_type.get_product()).to_string().as_str(),
            Palette::White.get_color(),
        )
    );

    // Draw 4 resource icons in the middle
    let resource_icon_size = 50.0;
    let resource_icon_spacing = 4.0;

    for i in 0..4 {
        let resource_x = inner_x + (i as f32 * (resource_icon_size + resource_icon_spacing));
        elements.push(UiElement::Rectangle {
            x: resource_x,
            y: offset.y + card_padding_y + 96.0,
            width: resource_icon_size,
            height: resource_icon_size,
            color: palette::IMAGE_BACKGROUND.get_color(),
            bordered: true,
        });

        // draw pill at the top of the rectangle
        elements.extend(
            pill(
                resource_x + resource_icon_size / 2.0 - 24.0 / 2.0,
                offset.y + card_padding_y + 96.0 - 14.0 / 2.0,
                24.0,
                14.0,
                "1234",
                Palette::White.get_color(),
            )
        );

        // draw pill at the bottom of the rectangle
        let pill_width = resource_icon_size - 24.0;
        let pill_height = 14.0;
        elements.extend(
            pill(
                resource_x + resource_icon_size / 2.0 - pill_width / 2.0,
                offset.y + card_padding_y + 96.0 + resource_icon_size - pill_height / 2.0,
                pill_width,
                pill_height,
                "1234", // Placeholder for resource amount
                Palette::Peach.get_color()
            )
        )
    }

    // Title Bar
    elements.push(UiElement::Text {
        content: job.job_type.get_name(),
        font: assets.fonts.text_bold.clone(),
        x: offset.x + card_padding_x,
        y: offset.y + card_padding_y + font_size_large,
        font_size: font_size_large,
        color: color_primary,
    });

    // Job Info
    elements.push(UiElement::Text {
        content: format!("Lvl {} | {}s", job.level, job.job_type.base_duration()),
        font: assets.fonts.text.clone(),
        x: offset.x + card_padding_x,
        y: offset.y + card_padding_y + 36.,
        font_size: font_size_small,
        color: color_secondary,
    });


    let progress_bar_width = card_width - card_padding_x - image_width - card_spacing - card_padding_x;
    let progress_bar_height = 10.0;
    let progress_bar_action_y = offset.y + card_height - progress_bar_height - card_padding_y;
    // Action Progress Bar
    elements.push(UiElement::ProgressBar {
        x: inner_x,
        y: progress_bar_action_y,
        width: progress_bar_width,
        height: progress_bar_height,
        progress: job.action_progress.get(),
        background_color: palette::BAR_BACKGROUND.get_color(),
        foreground_color: palette::PROGRESS_COLOR.get_color(),
        with_border: true,
    });

    // Delete Button
    let button_dimensions = 30.0;
    let button_spacing = 4.0;
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + card_width - button_dimensions - card_padding_x,
            y: offset.y + card_padding_y,
            w: button_dimensions,
            h: button_dimensions,
        },
        font: assets.fonts.text_bold.clone(),
        parent_clip: clip.clone(),
        font_size: font_size_small,
        text: "x".to_string(),
        color: palette::TEXT.get_color(),
        intent: Intent::ToggleJob(job_id),
    });

    // Start / Stop Button
    elements.push(UiElement::RectButton {
        rectangle: UiRect {
            x: offset.x + card_width - button_dimensions * 2.0 - button_spacing - card_padding_x,
            y: offset.y + card_padding_y,
            w: button_dimensions,
            h: button_dimensions,
        },
        font: assets.fonts.text_bold.clone(),
        parent_clip: clip.clone(),
        font_size: font_size_small,
        text: if job.running { "||".to_string() } else { ">".to_string() },
        color: palette::TEXT.get_color(),
        intent: Intent::ToggleJob(job_id),
    });

    elements
}