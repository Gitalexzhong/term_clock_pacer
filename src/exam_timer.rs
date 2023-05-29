use color_char::Character;
use chrono::{ Local, Timelike, DateTime, Duration };

pub(crate) struct ExamStatus {
    pub(crate) start: DateTime<Local>,
    pub(crate) duration_hour: i64,
    pub(crate) duration_min: i64,
}

fn display_status_bar(term_clock: &mut [[Character; 80]; 24], mut percent: f64) {
    if percent <= 0.0 {
        term_clock[10][1] = Character::new('w', 34);
        term_clock[10][2] = Character::new('a', 34);
        term_clock[10][3] = Character::new('i', 34);
        term_clock[10][4] = Character::new('t', 34);
        term_clock[10][28] = Character::new('^', 34);
        term_clock[10][29] = Character::new('^', 34);
        term_clock[10][30] = Character::new('^', 34);
        term_clock[10][31] = Character::new('^', 34);
        term_clock[10][32] = Character::new('^', 34);
        return;
    }

    let mut index = 1;

    while percent > 0.0 {
        if percent >= 0.03125 {
            term_clock[10][index] = Character::new('█', 32);
            percent -= 0.03125;
        } else {
            // it is the last block so we return a partial fill block
            match percent {
                p if p <= 0.00390625 => term_clock[10][index] = Character::new('▏', 32),
                p if p <= 0.0078125 => term_clock[10][index] = Character::new('▎', 32),
                p if p <= 0.01171875 => term_clock[10][index] = Character::new('▍', 32),
                p if p <= 0.015625 => term_clock[10][index] = Character::new('▌', 32),
                p if p <= 0.01953125 => term_clock[10][index] = Character::new('▊', 32),
                p if p <= 0.0234375 => term_clock[10][index] = Character::new('▊', 32),
                p if p <= 0.02734375 => term_clock[10][index] = Character::new('▉', 32),
                _ => {}
            }
        }

        index += 1;

        // Check out of bounce and return finished major state
        if index == 33 && percent >= 0.0 {
            // TODO change all char color to over max time 
            break;
        }
    }
}

fn add_start_end_duration(
    mut term_clock: &mut [[Character; 80]; 24],
    exam: &mut ExamStatus,
    color: u32
) {
    // Calculate start time
    let start_time = exam.start;
    let start_time_str = start_time.hour().to_string() + ":" + &start_time.minute().to_string();

    // Calculate end time
    let end_time =
        exam.start + Duration::hours(exam.duration_hour) + Duration::minutes(exam.duration_min);
    let end_time_str =
        end_time.hour().to_string() +
        ":" +
        &end_time.minute().to_string() +
        ":" +
        &end_time.second().to_string();

    // Combined statement
    let mut duration_time = start_time_str + " - " + &end_time_str;
    for i in 0..duration_time.len() {
        term_clock[9][i + 1] = Character::new(duration_time.remove(0), color);
    }

    // Calculate remaining time
    // TODO remove min added for testing
    let mut remaining_time_str;
    let counter_colour;
    if Local::now() < start_time  {
    // if Local::now() < start_time + Duration::minutes(1) {
        // TODO remove min added for testing
        let time_left = Local::now() - Duration::minutes(1) - start_time;
        remaining_time_str =
            "-".to_string() +
            &time_left.num_minutes().abs().to_string() +
            ":" +
            &(time_left.num_seconds().abs() % 60).to_string();
        counter_colour = 34;
        display_status_bar(&mut term_clock, -1.0);
    } else if Local::now() <= end_time {
        let time_left = end_time - Local::now();
        remaining_time_str =
            time_left.num_minutes().abs().to_string() +
            ":" +
            &(time_left.num_seconds().abs() % 60).to_string();

        counter_colour = 32;
        display_status_bar(&mut term_clock, 1.0);
    } else {
        let time_left = Local::now() - end_time;
        remaining_time_str =
            "+".to_string() +
            &time_left.num_minutes().abs().to_string() +
            ":" +
            &(time_left.num_seconds().abs() % 60).to_string();

        counter_colour = 31;
    }

    // term_clock[10][1] = Character::new('L', color);
    for i in 0..remaining_time_str.len() {
        term_clock[9][32 - i] = Character::new(remaining_time_str.pop().unwrap(), counter_colour);
    }
}

pub(crate) fn update_exam_time(term_clock: &mut [[Character; 80]; 24], exam: &mut ExamStatus) {
    let exam_time_color = 32;

    add_start_end_duration(term_clock, exam, exam_time_color);
}
