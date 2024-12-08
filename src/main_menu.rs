use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    text::Font,
};

pub fn draw_background_grid(
    draw: &mut RaylibDrawHandle,
    rect: &Rectangle,
    grid_spacing: f32,
    grid_thickness: f32,
    grid_color: &Color,
) {
    for x in (rect.x as i32..(rect.x + rect.width) as i32).step_by(grid_spacing as usize) {
        if (x as f32) > rect.x && (x as f32) < rect.x + rect.width {
            let start = Vector2::new(x as f32, rect.y);
            let end = Vector2::new(x as f32, rect.y + rect.height);
            draw.draw_line_ex(start, end, grid_thickness, grid_color);
        }
    }

    for y in (rect.y as i32..(rect.y + rect.height) as i32).step_by(grid_spacing as usize) {
        if (y as f32) > rect.y && (y as f32) < rect.y + rect.height {
            let start = Vector2::new(rect.x, y as f32);
            let end = Vector2::new(rect.x + rect.width, y as f32);
            draw.draw_line_ex(start, end, grid_thickness, grid_color);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn draw_top_bar(
    mouse_pos: Vector2,
    draw: &mut RaylibDrawHandle,
    font: &Font,
    subtitle: String,
    minimize_button: Rectangle,
    maximize_button: Rectangle,
    close_button: Rectangle,
    outline_color: Color,
) {
    let mut rng = SmallRng::from_entropy();
    let rand6 = rng.gen_range(1..=40);

    let subtitle_x = 210.0 + if rand6 == 40 { 5.0 } else { 0.0 };
    let subtitle_position = Vector2::new(subtitle_x, 30.0);
    draw.draw_text_ex(font, &subtitle, subtitle_position, 20.0, 2.0, outline_color);

    // draw close button
    if close_button.check_collision_point_rec(mouse_pos) {
        draw.draw_rectangle_rec(close_button, Color::RED);
    }
    draw.draw_text("X", (close_button.x + 8.9) as i32, (close_button.y + 5.0) as i32, 20, outline_color);

    // draw maximize button
    if maximize_button.check_collision_point_rec(mouse_pos) {
        draw.draw_rectangle_rec(maximize_button, Color::RED);
    }
    draw.draw_text("O", (maximize_button.x + 8.9) as i32, (maximize_button.y + 5.0) as i32, 20, outline_color);

    // draw minimize button
    if minimize_button.check_collision_point_rec(mouse_pos) {
        draw.draw_rectangle_rec(minimize_button, Color::RED);
    }
    draw.draw_text("_", (minimize_button.x + 8.9) as i32, (minimize_button.y + 5.0) as i32, 20, outline_color);
}
