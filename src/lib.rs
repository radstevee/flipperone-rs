use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::ffi::TextSubtext;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::RaylibDrawHandle;
use raylib::text::RaylibFont;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;

pub const BUTTON_TEXT: &str = "LOGS";
pub const TEXT_SIZE: f32 = 20f32;
pub const TEXT_SPACING: f32 = 2f32;
pub const ROUNDNESS: f32 = 0.03f32;
pub const GRID_SPACING: f32 = 25f32;
pub const SEGMENTS: i32 = 30;
pub const GRID_THICKNESS: f32 = 1.5f32;
pub const BLINK_TIME: f32 = 0.35f32;
pub const SMOOTHING: f32 = 0.1f32;
pub const NAVIGATION_BUTTON_SIZE: f32 = 30f32;
#[cfg(target_os = "windows")]
pub const SUBTITLE: &str = "FLIPPERONE.EXE";
#[cfg(not(target_os = "windows"))]
pub const SUBTITLE: &str = "FLIPPERONE";

pub fn draw_smooth_rounded_rect_lines(
    draw: &mut RaylibDrawHandle,
    rect: Rectangle,
    roundness: f32,
    segments: i32,
    base_thickness: f32,
    color: Color,
) {
    for idx in 0..5 {
        let thickness = base_thickness + (idx as f32) * 0.4;
        let mut layer_color = color;
        layer_color.a = color.a / (idx + 2);

        draw.draw_rectangle_rounded_lines(
            rect,
            roundness,
            segments,
            thickness,
            layer_color,
        )
    }
}

pub fn draw_text_with_colors(
    draw: &mut RaylibDrawHandle,
    text: String,
    position: Vector2,
    font_size: f32,
    font: &impl RaylibFont,
    default_color: Color,
    outline_color: Color,
) {
    let mut x_pos = position.x;
    let mut y_pos = position.y;
    let mut word = String::new();
    let mut word_color = default_color;
    let quote_color = outline_color;
    let mut inside_quotes = false;

    for char in text.chars() {
        if char == '"' {
            if !word.is_empty() {
                draw.draw_text_ex(
                    font,
                    &word,
                    Vector2::new(x_pos, y_pos),
                    font_size,
                    2f32,
                    word_color,
                );

                x_pos += font.measure_text(&text, font_size, 2f32).x + 2f32;
                word.clear();
            }

            inside_quotes = !inside_quotes;

            if inside_quotes {
                word_color = if inside_quotes {
                    quote_color
                } else {
                    default_color
                };
            }

            let quote_str = "'";
            draw.draw_text_ex(
                font,
                quote_str,
                Vector2::new(x_pos, y_pos),
                font_size,
                2f32,
                word_color,
            );

            x_pos += font.measure_text(quote_str, font_size, 2f32).x + 2f32;

            if !inside_quotes {
                word_color = default_color;
            }
        } else if char == ' ' || char == '\n' {
            if !word.is_empty() {
                draw.draw_text_ex(
                    font,
                    &word,
                    Vector2::new(x_pos, y_pos),
                    font_size,
                    2f32,
                    word_color,
                );

                x_pos += font.measure_text(&word, font_size, 2f32).x + 2f32;
                word.clear();

                if !inside_quotes {
                    word_color = default_color;
                }
            }

            if char == ' ' {
                x_pos += 8f32;
            } else if char == '\n' {
                x_pos = position.x;
                y_pos += font_size + 4f32;
            }
        } else {
            word.push(char);
        }
    }

    if !word.is_empty() {
        draw.draw_text_ex(
            font,
            &word,
            Vector2::new(x_pos, y_pos),
            font_size,
            2f32,
            word_color
        );
    }
}

pub fn current_line_drawing_width(font: &impl RaylibFont, str: &str) -> i32 {
    let mut relative_width = 0;

    for char in str.chars().rev() {
        if char == '\n' {
            break;
        }

        if char == ' ' {
            relative_width += 8;
        } else {
            let character_width = font.measure_text(&char.to_string(), TEXT_SIZE, TEXT_SPACING).x as i32;
            relative_width += character_width + 2;
        }
    }

    relative_width
}

pub fn count_chars_until_newline_backwards(str: &str) -> i32 {
    let mut count = 0;

    for char in str.chars().rev() {
        if char == '\n' {
            break;
        }

        count += 1;
    }

    count
}

pub fn can_tab_complete(str: &str) -> bool {
    let mut count = 0;

    for char in str.chars().rev() {
        if char == '\n' {
            return false
        }

        if char == ' ' {
            count += 1;
        } else {
            return false;
        }

        if count == 4 {
            return true;
        }
    }

    false
}

pub fn count_spacing_chars_until_newline_backwards(str: &str, other_chars_permitted: bool) -> i32 {
    let mut count = 0;

    for char in str.chars().rev() {
        if char == '\n' {
            break;
        }

        if char == ' ' {
            count += 1;
        } else if !other_chars_permitted {
            return -1;
        }
    }

    count
}

pub fn text_subtext(
    text: String,
    position: i32,
    length: i32
) -> String {
    unsafe {
        let cstr = CString::new(text).expect("invalid C string");
        let ptr = cstr.into_raw();

        let subtext_ptr = TextSubtext(ptr, position as c_int, length as c_int);

        let cstr = CStr::from_ptr(subtext_ptr);

        let result = cstr.to_string_lossy().into_owned();

        let _ = CString::from_raw(ptr);

        result
    }
}
