use colored::Colorize;

#[macro_export]
macro_rules! color_format {
    ($input:expr, $($color:expr),*) => {{
        let mut colors = vec![$(Color::from($color)),*];
        color_fmt($input, colors)
    }};
}

#[repr(u32)]
#[derive(Debug, Clone)]
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

fn count_single_occurence(input: &str, target: char) -> usize {
    let mut count = 0;
    let mut consecutive = 0;

    for char in input.chars() {
        if char == target {
            consecutive += 1;
            continue;
        }

        if consecutive == 1 {
            count += 1;
        }

        consecutive = 0;
    }

    if consecutive == 1 {
        count += 1;
    }

    count
}

pub fn color_fmt(input: &str, mut colors: Vec<Color>) -> String {
    let open_brackets = count_single_occurence(input, '{');
    let close_brackets = count_single_occurence(input, '}');
    assert_eq!(open_brackets, colors.len(), "There are not as many colors as brackets in the given string.");
    assert_eq!(open_brackets, close_brackets, "Opening brackets don't match the closing brackets.");

    let mut result = String::new();



    // for part in input.split_terminator('{') {
    //     println!("{:?}", part);
    //
    //     let mut brace_parts: Vec<&str> = part.splitn(2, '}').collect();
    //
    //     // The first part will be the word contained in the curly braces
    //     // and the second part is the text after the closing curly brace.
    //     if brace_parts.len() == 2 {
    //         let brace_text = brace_parts.remove(0);
    //         let text_color = colors.remove(0);
    //
    //         let (r, g, b) = hex_to_rgb(text_color as u32);
    //         let text_color = colored::Color::TrueColor { r, g, b };
    //
    //         result.push_str(&brace_text.color(text_color).to_string());
    //     }
    //
    //     let text_after = brace_parts.remove(0);
    //     result.push_str(&text_after.color(colored::Color::White).to_string());
    // }

    result
}