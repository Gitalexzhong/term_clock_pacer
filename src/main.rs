use std::{ thread::sleep, time::Duration };
use chrono::{ Local, Timelike, DateTime, Duration as ChDuration };
use color_char::Character;

enum Line {
    Top,
    Middle,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn add_number_line(display: &mut [[Character; 80]; 24], start: usize, line: Line, color_code: u32) {
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

fn add_divider(display: &mut [[Character; 80]; 24], color_code: u32) {
    display[2][16] = Character::new('█', color_code);
    display[4][16] = Character::new('█', color_code);
    display[2][17] = Character::new('█', color_code);
    display[4][17] = Character::new('█', color_code);
}

fn add_big_number(display: &mut [[Character; 80]; 24], start: usize, digit: u32, color_code: u32) {
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

fn add_date(display: &mut [[Character; 80]; 24], date: String, color_code: u32) {
    let mut date_str = date.chars();

    for i in 0..date.len() {
        display[7][12 + i] = Character::new(date_str.next().unwrap(), color_code);
    }
}

fn display(term_clock: &[[Character; 80]; 24]) {
    print!("{}[2J", 27 as char);

    for i in 0..24 {
        for j in 0..80 {
            print!("\x1b[0;{}m{}\x1b[0m", term_clock[i][j].get_color(), term_clock[i][j]);
        }
        println!();
    }
}

fn update_time(term_clock: &mut [[Character; 80]; 24]) {
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

fn update_exam_time(term_clock: &mut [[Character; 80]; 24], exam: &mut exam_status) {
    let exam_time_color = 32;

    // Display start time
    let mut start_time = exam.start.hour().to_string() + ":" + &exam.start.minute().to_string();
    for i in 0..start_time.len() {
        term_clock[9][i + 1] = Character::new(start_time.remove(0), exam_time_color);
    }

    // Display end time
    let end_time = exam.start + ChDuration::hours(exam.duration_hour) + ChDuration::minutes(exam.duration_min);
    let mut end_time = end_time.hour().to_string() + ":" + &end_time.minute().to_string();
    for i in 0..end_time.len() {
        term_clock[9][32 - i] = Character::new(end_time.pop().unwrap(), exam_time_color);
    }
}

struct exam_status {
    start: DateTime<Local>,
    duration_hour: i64,
    duration_min: i64,
}

fn main() {
    // Initial end and start times
    let mut exam = exam_status { duration_hour: 2, duration_min: 30, start: Local::now() };

    // Initial display
    let mut term_clock = [[Character::default(); 80]; 24];
    update_time(&mut term_clock);
    update_exam_time(&mut term_clock, &mut exam);
    display(&term_clock);

    // Check time endlessly
    let mut last_minute = Local::now().minute();
    loop {
        sleep(Duration::from_millis(500));
        let current_minute = Local::now().minute();
        if current_minute != last_minute {
            // On successful time change update display
            term_clock = [[Character::default(); 80]; 24];
            update_time(&mut term_clock);
            update_exam_time(&mut term_clock, &mut exam);

            display(&term_clock);

            last_minute = current_minute;
        }
    }
}