use std::ops::Range;

use colored::Colorize;
use serde::Serialize;
use serdebug::SerDebug;

#[repr(u32)]
#[derive(SerDebug, Serialize, Clone)]
pub enum Color {
    Red = 0xE42667,
    Green = 0xAFFF5F,
    White = 0xDCEEEB,
    Grey = 0x949494,
    Beach = 0x7DC7A4,
    LightGrey = 0xAAADB0,
    Blue = 0x0074D9,
    Orange = 0xFF851B,
    Yellow = 0xFFDC00,
    Aqua = 0x7FDBFF,
}

fn hex_to_rgb(color: u32) -> (u8, u8, u8) {
    let red = ((color >> 16) & 0xFF) as u8;
    let green = ((color >> 8) & 0xFF) as u8;
    let blue = (color & 0xFF) as u8;

    (red, green, blue)
}

pub(crate) fn color_fmt(input: &str, colors: &[Color]) -> String {
    let format_ranges = find_format_ranges(input);
    if format_ranges.iter().find(|range| range.formatted).is_none() {
        return input.to_string();
    }

    let mut colors = colors.to_vec();
    let mut result = String::new();

    for format_range in format_ranges {
        let text = match format_range.formatted {
            true => {
                &input[format_range.range.start + format_range.consecutive
                    ..format_range.range.end - format_range.consecutive]
            }
            false => &input[format_range.range.start..format_range.range.end],
        };

        if !format_range.formatted {
            result.push_str(&text);
            continue;
        }

        if colors.is_empty() {
            panic!("The amount of colors don't match the patterns found in the string.");
        }

        let (r, g, b) = hex_to_rgb(colors.remove(0) as u32);
        let text_color = colored::Color::TrueColor { r, g, b };

        result.push_str(&text.color(text_color).to_string());
    }

    result
}

#[derive(Debug)]
struct FormatRange {
    range: Range<usize>,
    consecutive: usize,
    formatted: bool,
}

fn find_format_ranges(input: &str) -> Vec<FormatRange> {
    let mut format_ranges = Vec::new();
    if input.len() == 1 {
        return format_ranges;
    }

    let mut consecutive_open = 0usize;
    let mut normal_start = 0usize;
    let mut index = 0usize;
    while index <= input.len() - 1 {
        let current = input.chars().nth(index).unwrap();
        if current == '[' {
            consecutive_open += 1;
        } else if consecutive_open % 2 != 0 {
            format_ranges.push(FormatRange {
                range: normal_start..index - consecutive_open,
                consecutive: 0,
                formatted: false,
            });

            let start_index = index - consecutive_open;

            let mut consecutive_close = 0usize;
            while index < input.len() - 1 {
                let current = input.chars().nth(index).unwrap();
                if current == ']' {
                    consecutive_close += 1;
                } else if consecutive_close != 0 {
                    break;
                }

                index += 1;
            }

            if consecutive_open != consecutive_close {
                panic!("Cannot have less consecutive '[' than consecutive ']'.");
            }

            format_ranges.push(FormatRange {
                range: start_index..index,
                consecutive: consecutive_close,
                formatted: true,
            });

            normal_start = index;
            consecutive_open = 0;
        } else if index == input.len() - 1 {
            format_ranges.push(FormatRange {
                range: normal_start..input.len(),
                consecutive: 0,
                formatted: false,
            });
        }

        index += 1;
    }

    format_ranges
}
