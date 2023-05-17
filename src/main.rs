use chrono::{ Local, Timelike };

enum Line {
    Top,
    Middle,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn add_number_line(display: &mut [[char; 80]; 24], start: usize, line: Line) {
    match line {
        Line::Top => {
            display[1][start] = '█';
            display[1][start + 1] = '█';
            display[1][start + 2] = '█';
            display[1][start + 3] = '█';
            display[1][start + 4] = '█';
            display[1][start + 5] = '█';
        }
        Line::Middle => {
            display[3][start] = '█';
            display[3][start + 1] = '█';
            display[3][start + 2] = '█';
            display[3][start + 3] = '█';
            display[3][start + 4] = '█';
            display[3][start + 5] = '█';
        }
        Line::Bottom => {
            display[5][start] = '█';
            display[5][start + 1] = '█';
            display[5][start + 2] = '█';
            display[5][start + 3] = '█';
            display[5][start + 4] = '█';
            display[5][start + 5] = '█';
        }
        Line::TopLeft => {
            display[1][start] = '█';
            display[2][start] = '█';
            display[3][start] = '█';
            display[1][start + 1] = '█';
            display[2][start + 1] = '█';
            display[3][start + 1] = '█';
        }
        Line::TopRight => {
            display[1][start + 4] = '█';
            display[2][start + 4] = '█';
            display[3][start + 4] = '█';
            display[1][start + 5] = '█';
            display[2][start + 5] = '█';
            display[3][start + 5] = '█';
        }
        Line::BottomLeft => {
            display[3][start] = '█';
            display[4][start] = '█';
            display[5][start] = '█';
            display[3][start + 1] = '█';
            display[4][start + 1] = '█';
            display[5][start + 1] = '█';
        }
        Line::BottomRight => {
            display[3][start + 4] = '█';
            display[4][start + 4] = '█';
            display[5][start + 4] = '█';
            display[3][start + 5] = '█';
            display[4][start + 5] = '█';
            display[5][start + 5] = '█';
        }
    }
}

fn add_divider(display: &mut [[char; 80]; 24]) {
    display[2][16] = '█';
    display[4][16] = '█';
    display[2][17] = '█';
    display[4][17] = '█';
}

fn add_big_number(display: &mut [[char; 80]; 24], start: usize, digit: u32) {
    match digit {
        0 => {
            add_number_line(display, start, Line::Top);
            add_number_line(display, start, Line::Bottom);
            add_number_line(display, start, Line::BottomRight);
            add_number_line(display, start, Line::TopRight);
            add_number_line(display, start, Line::BottomLeft);
            add_number_line(display, start, Line::TopLeft);
        }
        1 => {
            add_number_line(display, start, Line::TopRight);
            add_number_line(display, start, Line::BottomRight);
        }
        2 => {
            add_number_line(display, start, Line::Top);
            add_number_line(display, start, Line::Bottom);
            add_number_line(display, start, Line::Middle);
            add_number_line(display, start, Line::TopRight);
            add_number_line(display, start, Line::BottomLeft);
        }
        3 => {
            add_number_line(display, start, Line::Top);
            add_number_line(display, start, Line::Bottom);
            add_number_line(display, start, Line::Middle);
            add_number_line(display, start, Line::BottomRight);
            add_number_line(display, start, Line::TopRight);
        }
        4 => {
            add_number_line(display, start, Line::Middle);
            add_number_line(display, start, Line::BottomRight);
            add_number_line(display, start, Line::TopRight);
            add_number_line(display, start, Line::TopLeft);
        }
        5 => {
            add_number_line(display, start, Line::Top);
            add_number_line(display, start, Line::Bottom);
            add_number_line(display, start, Line::Middle);
            add_number_line(display, start, Line::BottomRight);
            add_number_line(display, start, Line::TopLeft);
        }
        6 => {
            add_number_line(display, start, Line::Top);
            add_number_line(display, start, Line::Bottom);
            add_number_line(display, start, Line::Middle);
            add_number_line(display, start, Line::BottomRight);
            add_number_line(display, start, Line::BottomLeft);
            add_number_line(display, start, Line::TopLeft);
        }
        7 => {
            add_number_line(display, start, Line::Top);

            add_number_line(display, start, Line::BottomRight);
            add_number_line(display, start, Line::TopRight);
        }
        8 => {
            add_number_line(display, start, Line::Top);
            add_number_line(display, start, Line::Bottom);
            add_number_line(display, start, Line::Middle);
            add_number_line(display, start, Line::BottomRight);
            add_number_line(display, start, Line::TopRight);
            add_number_line(display, start, Line::BottomLeft);
            add_number_line(display, start, Line::TopLeft);
        }
        9 => {
            add_number_line(display, start, Line::Top);
            add_number_line(display, start, Line::Bottom);
            add_number_line(display, start, Line::Middle);
            add_number_line(display, start, Line::BottomRight);
            add_number_line(display, start, Line::TopRight);
            add_number_line(display, start, Line::TopLeft);
        }
        _ => todo!(),
    }
}

fn display(term_clock: &[[char; 80]; 24]) {
    print!("{}[2J", 27 as char);

    for i in 0..24 {
        for j in 0..80 {
            print!("\x1b[0;32m{}\x1b[0m", term_clock[i][j]);
        }
        println!();
    }
}

fn update(term_clock: &mut [[char; 80]; 24]) {
    let dt = Local::now();
    let hour = dt.hour();
    let minutes = dt.minute();

    add_big_number(term_clock, 1, hour / 10);
    add_big_number(term_clock, 8, hour % 10);

    add_divider(term_clock);

    add_big_number(term_clock, 20, minutes / 10);
    add_big_number(term_clock, 27, minutes % 10);
}

fn main() {
    let mut term_clock = [[' '; 80]; 24];

    update(&mut term_clock);
    display(&term_clock);
}