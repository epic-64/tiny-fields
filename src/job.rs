use crate::draw::UiElement;
use crate::game::{Assets, GameState, Intent, Job, MouseInput, UiRect};
use crate::ui::ScrollContainer;
use macroquad::color::{Color, BLUE, DARKBLUE, DARKGRAY, GRAY, GREEN, WHITE};
use macroquad::math::Vec2;

pub struct JobUi {
    scroll_container: ScrollContainer,
}

impl JobUi {
    pub fn new(rect: UiRect) -> Self {
        Self {
            scroll_container: ScrollContainer::new(rect),
        }
    }

    pub fn update(&mut self, mouse_input: &MouseInput) {
        self.scroll_container.update(mouse_input);
    }

    pub fn build(&self, state: &GameState, assets: &Assets) -> Vec<UiElement> {
        let mut elements: Vec<UiElement> = vec![];

        // add decorations
        let padding = 5.0;
        elements.push(UiElement::Rectangle {
            x: self.scroll_container.rect.x - padding,
            y: self.scroll_container.rect.y - padding,
            width: self.scroll_container.rect.w + padding * 2.0,
            height: self.scroll_container.rect.h + padding * 2.0,
            color: Color::from_rgba(0, 0, 0, 100),
        });

        elements.extend(self.scroll_container.build(state, assets, build_job_cards));

        elements
    }
}

fn build_job_cards(
    state: &GameState,
    assets: &Assets,
    clip_rect: &UiRect,
    offset: Vec2
) -> Vec<UiElement>
{
    let mut elements: Vec<UiElement> = vec![];

    let mut container_offset = offset;
    let card_height = 150.0;
    let card_width = clip_rect.w;
    let card_spacing = 15.0;
    let card_padding_x = 20.0;
    let card_padding_y = 20.0;

    let container_clip = Some((
        clip_rect.x as i32,
        clip_rect.y as i32,
        clip_rect.w as i32,
        clip_rect.h as i32,
    ));

    for (id, job) in state.jobs.iter().enumerate() {
        let job_draw_container = build_job_card(
            &container_clip,
            assets,
            job,
            id,
            container_offset,
            card_height,
            card_width,
            card_padding_x,
            card_padding_y,
            card_spacing,
        );

        elements.extend(job_draw_container);

        container_offset += Vec2::new(0.0, card_height + 5.0);
    }

    elements
}

pub fn build_job_card(
    clip: &Option<(i32, i32, i32, i32)>,
    assets: &Assets,
    job: &Job,
    job_id: usize,
    offset: Vec2,
    card_height: f32,
    card_width: f32,
    card_padding_x: f32,
    card_padding_y: f32,
    card_spacing: f32,
) -> Vec<UiElement>
{
    let color_primary = DARKBLUE;
    let color_secondary = DARKGRAY;
    let color_button = GRAY;

    let font_size_large = 20.0;
    let font_size_small = 14.0;

    let image_width = 90.0f32;
    let button_width = 90.0;
    let inner_x = offset.x + card_padding_x + image_width + card_spacing;

    let (image1, image2) = job.job_type.get_images(assets);

    let chosen_image = if job.running && job.time_accumulator % 2.0 < 1.0 {
        image1
    } else {
        image2
    };

    let mut elements = vec![];

    // Background
    elements.push(UiElement::Rectangle {
        x: offset.x,
        y: offset.y,
        width: card_width,
        height: card_height,
        color: Color::from_rgba(240, 240, 230, 255),
    });

    // Job Animation
    elements.push(UiElement::Image {
        x: offset.x + card_padding_x,
        y: offset.y + card_padding_y,
        width: image_width,
        height: card_height - card_padding_y * 2.0,
        texture: chosen_image.clone(),
        color: if job.running { WHITE } else { Color::from_rgba(90, 90, 90, 255) },
    });

    // Title Bar
    elements.push(UiElement::Text {
        content: job.name.clone() + " ",
        font: assets.fonts.main.clone(),
        x: inner_x,
        y: offset.y + card_padding_y + font_size_large,
        font_size: font_size_large,
        color: color_primary,
    });

    // Job Info
    elements.push(UiElement::Text {
        content: format!("Lvl {} | ${} | {}s", job.level, job.money_per_action(), job.action_duration),
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
        background_color: GRAY,
        foreground_color: GREEN,
    });

    // Action Progress Text
    elements.push(UiElement::Text {
        content: format!("{:.1} / {:.1}", job.time_accumulator, job.action_duration),
        font: assets.fonts.main.clone(),
        x: inner_x + 10.0,
        y: progress_bar_action_y + 15.0,
        font_size: font_size_small,
        color: WHITE,
    });

    let progress_bar_level_y = progress_bar_action_y + progress_bar_height + 5.0;

    // Level Up Progress Bar
    elements.push(UiElement::ProgressBar {
        x: inner_x,
        y: progress_bar_level_y,
        width: progress_bar_width,
        height: progress_bar_height,
        progress: job.level_up_progress.get(),
        background_color: GRAY,
        foreground_color: BLUE,
    });

    // Level Up Progress Text
    elements.push(UiElement::Text {
        content: format!("Level Up: {} / {}", job.actions_done, job.actions_to_level_up()),
        font: assets.fonts.main.clone(),
        x: inner_x + 10.0,
        y: progress_bar_level_y + 15.0,
        font_size: font_size_small,
        color: WHITE,
    });

    // Start / Stop Button
    elements.push(UiElement::Button {
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