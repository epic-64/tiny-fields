use macroquad::math::Rect;
use macroquad::prelude::{Color, Vec2};
use macroquad::texture::{draw_texture_ex, DrawTextureParams, Texture2D};

pub fn draw_nine_patch(
    texture: &Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    corner_size: f32,
    color: Color,
) {
    let tex_w = texture.width();
    let tex_h = texture.height();
    let third_w = tex_w / 3.0;
    let third_h = tex_h / 3.0;

    let stretch_w = width - 2.0 * corner_size;
    let stretch_h = height - 2.0 * corner_size;

    let draw_patch = |dst_x, dst_y, dst_w, dst_h, src_x, src_y| {
        draw_texture_ex(
            texture,
            dst_x,
            dst_y,
            color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(dst_w, dst_h)),
                source: Some(Rect::new(src_x, src_y, third_w, third_h)),
                ..Default::default()
            },
        );
    };

    // Corners
    draw_patch(x, y, corner_size, corner_size, 0.0, 0.0); // Top-left
    draw_patch(x + corner_size + stretch_w, y, corner_size, corner_size, 2.0 * third_w, 0.0); // Top-right
    draw_patch(x, y + corner_size + stretch_h, corner_size, corner_size, 0.0, 2.0 * third_h); // Bottom-left
    draw_patch(x + corner_size + stretch_w, y + corner_size + stretch_h, corner_size, corner_size, 2.0 * third_w, 2.0 * third_h); // Bottom-right

    // Edges
    draw_patch(x + corner_size, y, stretch_w, corner_size, third_w, 0.0); // Top
    draw_patch(x + corner_size, y + corner_size + stretch_h, stretch_w, corner_size, third_w, 2.0 * third_h); // Bottom
    draw_patch(x, y + corner_size, corner_size, stretch_h, 0.0, third_h); // Left
    draw_patch(x + corner_size + stretch_w, y + corner_size, corner_size, stretch_h, 2.0 * third_w, third_h); // Right

    // Center
    draw_patch(x + corner_size, y + corner_size, stretch_w, stretch_h, third_w, third_h); // Center
}