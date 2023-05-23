use std::{ thread::sleep, time::Duration };
use chrono::{ Local, Timelike, DateTime, Duration as ChDuration };
use color_char::Character;

mod clock;
use clock::update_time;

fn display(term_clock: &[[Character; 80]; 24]) {
    print!("{}[2J", 27 as char);

    for i in 0..24 {
        for j in 0..80 {
            print!("\x1b[0;{}m{}\x1b[0m", term_clock[i][j].get_color(), term_clock[i][j]);
        }
        println!();
    }
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