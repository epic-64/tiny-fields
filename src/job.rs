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

    pub fn build(&self, state: &GameState, assets: &Assets, mouse_input: &MouseInput) -> Vec<UiElement> {
        let mut elements: Vec<UiElement> = vec![];

        // add decorations
        let padding = 5.0;
        elements.push(UiElement::Rectangle {
            x: self.scroll_container.rect.x - padding,
            y: self.scroll_container.rect.y - padding,
            width: self.scroll_container.rect.w as f64 + padding as f64 * 2.0,
            height: self.scroll_container.rect.h as f64 + padding as f64 * 2.0,
            color: Color::from_rgba(0, 0, 0, 100),
        });

        elements.extend(self.scroll_container.build(state, assets, mouse_input, build_job_cards));

        elements
    }
}

fn build_job_cards(
    state: &GameState,
    assets: &Assets,
    mouse_input: &MouseInput,
    clip_rect: &UiRect,
    offset: Vec2
) -> Vec<UiElement>
{
    let mut elements: Vec<UiElement> = vec![];

    let mut container_offset = offset;
    let card_height = 180.0;
    let card_width = clip_rect.w;
    let card_spacing = 15.0;
    let card_padding_x = 55.0;
    let card_padding_y = 40.0;

    let container_clip = Some((
        clip_rect.x as i32,
        clip_rect.y as i32,
        clip_rect.w as i32,
        clip_rect.h as i32,
    ));

    // background
    elements.push(UiElement::Rectangle {
        x: clip_rect.x,
        y: clip_rect.y,
        width: clip_rect.w as f64 + 10.0,
        height: clip_rect.h as f64 + 10.0,
        color: Color::from_rgba(0, 0, 0, 100),
    });

    for (id, job) in state.jobs.iter().enumerate() {
        let job_draw_container = build_job_card(
            mouse_input,
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

        container_offset += Vec2::new(0.0, card_height + 15.0);
    }

    elements
}

pub fn build_job_card(
    mouse_input: &MouseInput,
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
    let color_button = DARKGRAY;

    let font_size_large = 24.0;
    let font_size_small = 20.0;

    let image_width = 90.0f32;
    let inner_x = offset.x + card_padding_x + image_width + card_spacing;
    let progress_bar_width = card_width - card_padding_x - image_width - card_spacing - card_padding_x;
    let button_width = 90.0;

    let chosen_image = if job.running && job.time_accumulator % 2.0 < 1.0 {
        &assets.textures.wood_2
    } else {
        &assets.textures.wood_1
    };

    let mut elements = vec![];

    // Background
    elements.push(UiElement::Image {
        x: offset.x,
        y: offset.y,
        width: card_width,
        height: card_height,
        texture: assets.textures.frame1.clone(),
        color: WHITE
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
        x: inner_x,
        y: offset.y + card_padding_y + 15.0,
        font_size: font_size_large,
        color: color_primary,
    });

    for i in 0..job.timeslot_cost {
        elements.push(UiElement::Image {
            x: inner_x +  i as f32 * (20.0 + 5.0) + 100.0,
            y: offset.y + card_padding_y,
            width: 20.0,
            height: 20.0,
            texture: assets.textures.time.clone(),
            color: if job.running { WHITE } else { GRAY },
        });
    }

    // Job Info
    elements.push(UiElement::Text {
        content: format!("Lvl {} | ${} | {}s", job.level, job.money_per_action(), job.action_duration),
        x: inner_x,
        y: offset.y + 80.0,
        font_size: font_size_small,
        color: color_secondary,
    });

    // Action Progress Bar
    elements.push(UiElement::ProgressBar {
        x: inner_x,
        y: offset.y + 96.0,
        width: progress_bar_width - 120.0,
        height: 20.0,
        progress: job.action_progress.get(),
        background_color: GRAY,
        foreground_color: GREEN,
    });

    // Action Progress Text
    elements.push(UiElement::Text {
        content: format!("{:.1} / {:.1}", job.time_accumulator, job.action_duration),
        x: inner_x + 10.0,
        y: offset.y + 111.0,
        font_size: 20.0,
        color: WHITE,
    });

    // Level Up Progress Bar
    elements.push(UiElement::ProgressBar {
        x: inner_x,
        y: offset.y + 126.0,
        width: progress_bar_width - 120.0,
        height: 20.0,
        progress: job.level_up_progress.get(),
        background_color: GRAY,
        foreground_color: BLUE,
    });

    // Level Up Progress Text
    elements.push(UiElement::Text {
        content: format!("Level Up: {} / {}", job.actions_done, job.actions_to_level_up()),
        x: inner_x + 10.0,
        y: offset.y + 141.0,
        font_size: 20.0,
        color: WHITE,
    });

    // Start / Stop Button
    let button_rect = UiRect {
        x: offset.x + card_width - button_width - 30.0,
        y: offset.y + 25.0,
        w: button_width,
        h: 132.0,
    };

    // Check if the clip area of the scroll container is hovered
    let clip_is_hovered = if let Some(clip) = clip {
        let (x, y, w, h) = clip;
        UiRect {
            x: *x as f32,
            y: *y as f32,
            w: *w as f32,
            h: *h as f32
        }.is_hovered(mouse_input)
    } else {
        true
    };

    let button_is_hovered = button_rect.is_hovered(mouse_input) && clip_is_hovered;

    elements.push(UiElement::Button {
        rectangle: button_rect,
        parent_clip: clip.clone(),
        font_size: font_size_large,
        text: if job.running { "Stop".to_string() } else { "Start".to_string() },
        color: color_button,
        intent: Intent::ToggleJob(job_id),
        is_hovered: button_is_hovered,
    });

    elements
}