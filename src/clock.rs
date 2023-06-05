use chrono::{ Local, Timelike };
use color_char::Character;

use crate::{ type_defines::StdTerm };

enum Line {
    Top,
    Middle,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn add_number_line(display: &mut StdTerm, start: usize, line: Line, color_code: u32) {
    match line {
        Line::Top => {
            display[1][start] = Character::new('█', color_code);
            display[1][start + 1] = Character::new('█', color_code);
            display[1][start + 2] = Character::new('█', color_code);
            display[1][start + 3] = Character::new('█', color_code);
            display[1][start + 4] = Character::new('█', color_code);
            display[1][start + 5] = Character::new('█', color_code);
        }
        Line::Middle => {
            display[3][start] = Character::new('█', color_code);
            display[3][start + 1] = Character::new('█', color_code);
            display[3][start + 2] = Character::new('█', color_code);
            display[3][start + 3] = Character::new('█', color_code);
            display[3][start + 4] = Character::new('█', color_code);
            display[3][start + 5] = Character::new('█', color_code);
        }
        Line::Bottom => {
            display[5][start] = Character::new('█', color_code);
            display[5][start + 1] = Character::new('█', color_code);
            display[5][start + 2] = Character::new('█', color_code);
            display[5][start + 3] = Character::new('█', color_code);
            display[5][start + 4] = Character::new('█', color_code);
            display[5][start + 5] = Character::new('█', color_code);
        }
        Line::TopLeft => {
            display[1][start] = Character::new('█', color_code);
            display[2][start] = Character::new('█', color_code);
            display[3][start] = Character::new('█', color_code);
            display[1][start + 1] = Character::new('█', color_code);
            display[2][start + 1] = Character::new('█', color_code);
            display[3][start + 1] = Character::new('█', color_code);
        }
        Line::TopRight => {
            display[1][start + 4] = Character::new('█', color_code);
            display[2][start + 4] = Character::new('█', color_code);
            display[3][start + 4] = Character::new('█', color_code);
            display[1][start + 5] = Character::new('█', color_code);
            display[2][start + 5] = Character::new('█', color_code);
            display[3][start + 5] = Character::new('█', color_code);
        }
        Line::BottomLeft => {
            display[3][start] = Character::new('█', color_code);
            display[4][start] = Character::new('█', color_code);
            display[5][start] = Character::new('█', color_code);
            display[3][start + 1] = Character::new('█', color_code);
            display[4][start + 1] = Character::new('█', color_code);
            display[5][start + 1] = Character::new('█', color_code);
        }
        Line::BottomRight => {
            display[3][start + 4] = Character::new('█', color_code);
            display[4][start + 4] = Character::new('█', color_code);
            display[5][start + 4] = Character::new('█', color_code);
            display[3][start + 5] = Character::new('█', color_code);
            display[4][start + 5] = Character::new('█', color_code);
            display[5][start + 5] = Character::new('█', color_code);
        }
    }
}

fn add_divider(display: &mut StdTerm, color_code: u32) {
    display[2][16] = Character::new('█', color_code);
    display[4][16] = Character::new('█', color_code);
    display[2][17] = Character::new('█', color_code);
    display[4][17] = Character::new('█', color_code);
}

fn add_big_number(display: &mut StdTerm, start: usize, digit: u32, color_code: u32) {
    match digit {
        0 => {
            add_number_line(display, start, Line::Top, color_code);
            add_number_line(display, start, Line::Bottom, color_code);
            add_number_line(display, start, Line::BottomRight, color_code);
            add_number_line(display, start, Line::TopRight, color_code);
            add_number_line(display, start, Line::BottomLeft, color_code);
            add_number_line(display, start, Line::TopLeft, color_code);
        }
        1 => {
            add_number_line(display, start, Line::TopRight, color_code);
            add_number_line(display, start, Line::BottomRight, color_code);
        }
        2 => {
            add_number_line(display, start, Line::Top, color_code);
            add_number_line(display, start, Line::Bottom, color_code);
            add_number_line(display, start, Line::Middle, color_code);
            add_number_line(display, start, Line::TopRight, color_code);
            add_number_line(display, start, Line::BottomLeft, color_code);
        }
        3 => {
            add_number_line(display, start, Line::Top, color_code);
            add_number_line(display, start, Line::Bottom, color_code);
            add_number_line(display, start, Line::Middle, color_code);
            add_number_line(display, start, Line::BottomRight, color_code);
            add_number_line(display, start, Line::TopRight, color_code);
        }
        4 => {
            add_number_line(display, start, Line::Middle, color_code);
            add_number_line(display, start, Line::BottomRight, color_code);
            add_number_line(display, start, Line::TopRight, color_code);
            add_number_line(display, start, Line::TopLeft, color_code);
        }
        5 => {
            add_number_line(display, start, Line::Top, color_code);
            add_number_line(display, start, Line::Bottom, color_code);
            add_number_line(display, start, Line::Middle, color_code);
            add_number_line(display, start, Line::BottomRight, color_code);
            add_number_line(display, start, Line::TopLeft, color_code);
        }
        6 => {
            add_number_line(display, start, Line::Top, color_code);
            add_number_line(display, start, Line::Bottom, color_code);
            add_number_line(display, start, Line::Middle, color_code);
            add_number_line(display, start, Line::BottomRight, color_code);
            add_number_line(display, start, Line::BottomLeft, color_code);
            add_number_line(display, start, Line::TopLeft, color_code);
        }
        7 => {
            add_number_line(display, start, Line::Top, color_code);

            add_number_line(display, start, Line::BottomRight, color_code);
            add_number_line(display, start, Line::TopRight, color_code);
        }
        8 => {
            add_number_line(display, start, Line::Top, color_code);
            add_number_line(display, start, Line::Bottom, color_code);
            add_number_line(display, start, Line::Middle, color_code);
            add_number_line(display, start, Line::BottomRight, color_code);
            add_number_line(display, start, Line::TopRight, color_code);
            add_number_line(display, start, Line::BottomLeft, color_code);
            add_number_line(display, start, Line::TopLeft, color_code);
        }
        9 => {
            add_number_line(display, start, Line::Top, color_code);
            add_number_line(display, start, Line::Bottom, color_code);
            add_number_line(display, start, Line::Middle, color_code);
            add_number_line(display, start, Line::BottomRight, color_code);
            add_number_line(display, start, Line::TopRight, color_code);
            add_number_line(display, start, Line::TopLeft, color_code);
        }
        _ => {}
    }
}

fn add_date(display: &mut StdTerm, date: String, color_code: u32) {
    let mut date_str = date.chars();

    for i in 0..date.len() {
        display[7][12 + i] = Character::new(date_str.next().unwrap(), color_code);
    }
}

pub(crate) fn update_time(term_clock: &mut StdTerm) {
    let dt = Local::now();
    let hour = dt.hour();
    let minutes = dt.minute();
    let date = dt.date_naive();
    let clock_color = 32;

    add_big_number(term_clock, 1, hour / 10, clock_color);
    add_big_number(term_clock, 8, hour % 10, clock_color);

    add_divider(term_clock, clock_color);

    add_big_number(term_clock, 20, minutes / 10, clock_color);
    add_big_number(term_clock, 27, minutes % 10, clock_color);

    add_date(term_clock, date.to_string(), clock_color);
}
