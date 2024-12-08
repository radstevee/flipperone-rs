use flipperone_rs::{can_tab_complete, count_chars_until_newline_backwards, current_line_drawing_width, draw_smooth_rounded_rect_lines, draw_text_with_colors, text_subtext, BLINK_TIME, GRID_SPACING, GRID_THICKNESS, NAVIGATION_BUTTON_SIZE, ROUNDNESS, SEGMENTS, SMOOTHING, SUBTITLE, TEXT_SIZE, TEXT_SPACING};
use raylib::color::Color;
use raylib::consts::{KeyboardKey, MouseButton};
use raylib::math::{Rectangle, Vector2};
use raylib::text::RaylibFont;
use std::error::Error;
use raylib::drawing::RaylibDraw;

pub mod button;
pub mod main_menu;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut ray, thread) = raylib::init()
        .size(1280, 720)
        .title("FLIPPERONE")
        .build();
    
    ray.set_target_fps(60);

    let default_font = ray.get_font_default();
    let custom_font = ray.load_font(&thread, "fonts/haxrcorp-4089.ttf")?;
    let mut subtitle;
    let mut frames= 0i64;
    let button_text = "LOGS";

    // let text_box_color = Color::new(30, 30, 30, 255);
    // let text_box_border_color = Color::new(255, 165, 0, 255);
    let text_color = Color::new(255, 255, 255, 255);

    let grid_color = Color::new(34, 16, 3, 255);
    let background_color = Color::new(10, 4, 0, 255);
    let outline_color = Color::new(254, 138, 44, 255);

    let offset_x = 10.0f32;
    let mut cursor_x = 70.0f32;
    let mut cursor_y = 122f32;
    let offset_y = 60f32;

    let mut input = String::new();

    let mut cursor_blink_timer = 0f32;
    let mut cursor_visible = false;
    let mut writing_text = true;
    let mut dragging = false;
    let mut mouse_offset = Vector2::default();
    let mut window_pos_x = 100;
    let mut window_pos_y = 100;

    let now = std::time::SystemTime::now();
    let mut last_backspace_time = now.duration_since(std::time::UNIX_EPOCH)
        .expect("looks like somebody time-travelled")
        .as_millis();

    while !&ray.window_should_close() {
        let width = ray.get_screen_width() as f32;
        let height = ray.get_screen_height() as f32;

        let text_pos = Vector2::new(30f32, height - 40.0);

        let button_rect = Rectangle::new(
            text_pos.x - 10.0,
            text_pos.y - 10.0,
            custom_font.measure_text(button_text, TEXT_SIZE, TEXT_SPACING).x + 20.0,
            TEXT_SIZE + 20.0,
        );

        let rect = Rectangle::new(
            offset_x,
            offset_y,
            width - offset_x * 2.0,
            height - offset_y * 2.0,
        );

        let text_box = Rectangle::new(
            rect.x + 52.0,
            rect.y + 52.0,
            rect.width - 54.0,
            rect.height - 54.0,
        );

        let close_button = Rectangle::new(
            width - NAVIGATION_BUTTON_SIZE - 10.0,
            10.0,
            NAVIGATION_BUTTON_SIZE,
            NAVIGATION_BUTTON_SIZE,
        );

        let maximize_button = Rectangle::new(
            width - NAVIGATION_BUTTON_SIZE - 45.0,
            10.0,
            NAVIGATION_BUTTON_SIZE,
            NAVIGATION_BUTTON_SIZE,
        );

        let minimize_button = Rectangle::new(
            width - NAVIGATION_BUTTON_SIZE - 80.0,
            10.0,
            NAVIGATION_BUTTON_SIZE,
            NAVIGATION_BUTTON_SIZE,
        );

        let mouse_pos = ray.get_mouse_position();

        if ray.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            && !text_box.check_collision_point_rec(mouse_pos) {
            dragging = true;

            mouse_offset = Vector2::new(
                (ray.get_mouse_x() - window_pos_x) as f32,
                (ray.get_mouse_y() - window_pos_y) as f32,
            );
        }

        if dragging {
            let target_pos_x = ray.get_mouse_x() as f32 - mouse_offset.x;
            let target_pos_y = ray.get_mouse_y() as f32 - mouse_offset.y;

            window_pos_x += ((target_pos_x as i32 - window_pos_x) as f32 * SMOOTHING) as i32; // skull
            window_pos_y += ((target_pos_y as i32 - window_pos_y) as f32 * SMOOTHING) as i32; // skull
        }

        if ray.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
            || ray.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT)
            || !ray.is_window_focused() {
            dragging = false;
        }

        ray.set_window_position(window_pos_x, window_pos_y);

        let close_button_clicked =
            close_button.check_collision_point_rec(mouse_pos)
                && ray.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        if close_button_clicked {
            break;
        }

        let maximize_button_clicked =
            maximize_button.check_collision_point_rec(mouse_pos)
                && ray.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        if maximize_button_clicked {
            ray.toggle_fullscreen();
        }

        let minimize_button_clicked =
            minimize_button.check_collision_point_rec(mouse_pos)
                && ray.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        if minimize_button_clicked {
            ray.set_window_state(
                ray.get_window_state()
                    .set_window_minimized(true)
            )
        }

        let key = ray.get_char_pressed();

        if let Some(char) = key {
            input.push(char);

            if char == ' ' {
                cursor_x += 8.0;
            } else {
                cursor_x += ray.measure_text(&char.to_string(), 20) as f32 + 2.0;
            }
        }

        if ray.is_key_pressed(KeyboardKey::KEY_TAB) {
            cursor_x += 8f32 * 4f32;
            input += "    ";
        }

        if ray.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && ray.is_key_pressed(KeyboardKey::KEY_V) {
            if let Ok(clip) = ray.get_clipboard_text() {
                let clipboard = clip.to_string();
                let dt = clipboard.lines().map(&str::to_string).collect::<Vec<String>>();

                for (idx, line) in dt.iter().enumerate() {
                    if idx == 0 {
                        cursor_x += current_line_drawing_width(&default_font, line) as f32;
                    } else {
                        cursor_x = 70.0 + current_line_drawing_width(&default_font, line) as f32;
                        cursor_y += 20.0;
                    }
                }

                input.push_str(&clipboard);
            }
        }

        let hovered = button_rect.check_collision_point_rec(mouse_pos);
        let clicked = hovered && ray.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        if ray.is_key_pressed(KeyboardKey::KEY_ENTER) {
            frames = 0;
            input.push('\n');
            cursor_x = 70.0;
            cursor_y += 20.0;
        }

        cursor_blink_timer += ray.get_frame_time();
        if cursor_blink_timer >= BLINK_TIME {
            cursor_visible = !cursor_visible;
            cursor_blink_timer = 0.0;
        }

        {
            let draw = &mut ray.begin_drawing(&thread);

            if draw.is_key_down(KeyboardKey::KEY_BACKSPACE) {
                let now = std::time::SystemTime::now();
                let current_time = now.duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis();

                if current_time - last_backspace_time >= 110 && !input.is_empty() {
                    last_backspace_time = current_time;

                    let mut key_text = String::new();
                    let is_new_line = input.ends_with('\n');
                    let is_space = input.ends_with(' ');

                    if let Some(last_char) = input.pop() {
                        key_text.push(last_char);
                    }

                    let ctrl = draw.is_key_down(KeyboardKey::KEY_LEFT_CONTROL);
                    let mut tab_delete = false;

                    if ctrl {
                        // Destroy the entire line
                        while !input.is_empty() {
                            if let Some(last_char) = input.pop() {
                                if last_char == '\n' {
                                    break;
                                }
                            }
                        }
                    } else if can_tab_complete(&input) && count_chars_until_newline_backwards(&input) != 0 {
                        tab_delete = true;
                        println!("Yay!");
                        for _ in 0..4 {
                            if let Some(last_char) = input.pop() {
                                if last_char == ' ' {
                                    cursor_x -= 8.0;
                                } else {
                                    cursor_x -= draw.measure_text(&last_char.to_string(), 20) as f32 + 2.0;
                                }
                            }
                        }
                    } else {
                        input.pop();
                    }

                    if is_new_line {
                        cursor_y -= 20.0;
                        cursor_x = 70.0 + current_line_drawing_width(&default_font, &input) as f32;
                    } else if tab_delete {
                        // tab delete is handled above
                    } else if ctrl {
                        cursor_x = 70.0;
                    } else if is_space {
                        cursor_x -= 8.0;
                    } else {
                        cursor_x -= draw.measure_text(&key_text, 20) as f32 + 2.0;
                    }
                }
            }

            if clicked {
                println!("logs button clicked");
            }

            if writing_text {
                frames += 1;
            } else {
                frames -= 1;
            }

            if frames > 60 * 3 {
                writing_text = false;
            } else if frames <= -30 {
                writing_text = true;
            }

            draw.clear_background(Color::BLANK);
            draw_smooth_rounded_rect_lines(draw, rect, ROUNDNESS, SEGMENTS, 3.0, outline_color);
            draw.draw_rectangle_rounded(rect, ROUNDNESS, SEGMENTS, background_color);

            main_menu::draw_background_grid(draw, &rect, GRID_SPACING, GRID_THICKNESS, &grid_color);
            
            subtitle = text_subtext(SUBTITLE.to_string(), 0, (frames / 10) as i32);

            draw.draw_rectangle_rec(text_box, Color::BLACK);
            draw_smooth_rounded_rect_lines(draw, text_box, ROUNDNESS, SEGMENTS, 2.0, background_color);

            let line_height = 20.0f32;
            let mut y_offset = text_box.y + 10.0;

            for line in input.lines() {
                draw_text_with_colors(
                    draw,
                    line.to_string(),
                    Vector2::new(text_box.x + 10.0, y_offset),
                    20.0,
                    &default_font,
                    Color::WHITE,
                    outline_color
                );

                y_offset += line_height;
            }

            if cursor_visible {
                draw.draw_line(
                    cursor_x as i32,
                    cursor_y as i32,
                    cursor_x as i32,
                    cursor_y as i32 + 20,
                    text_color
                );
            }

            draw.draw_text_ex(
                &custom_font,
                button_text,
                text_pos,
                TEXT_SIZE,
                TEXT_SPACING,
                outline_color
            );

            if clicked {
                draw.draw_rectangle_rec(
                    button_rect,
                    Color::new(255, 165, 0, 255)
                );
            } else if hovered {
                draw.draw_rectangle_rec(
                    button_rect,
                    Color::new(255, 165, 0, 50)
                );
            }

            draw.draw_rectangle_lines_ex(
                button_rect,
                2.0,
                outline_color
            );

            main_menu::draw_top_bar(mouse_pos, draw, &custom_font, subtitle, minimize_button, maximize_button, close_button, outline_color);
        }
    }

    ray.unload_font(custom_font.make_weak());

    Ok(())
}
