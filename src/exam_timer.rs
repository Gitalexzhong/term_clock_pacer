use color_char::Character;
use chrono::{ Local, Timelike, DateTime, Duration };

pub(crate) struct ExamStatus {
    pub(crate) start: DateTime<Local>,
    pub(crate) duration_hour: i64,
    pub(crate) duration_min: i64,
}

fn add_start_end_duration(
    term_clock: &mut [[Character; 80]; 24],
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
    if  Local::now() <=  start_time  +  Duration::minutes(1) {
        term_clock[10][1] = Character::new('B', color);
    } else if Local::now() <= end_time {
        term_clock[10][1] = Character::new('I', color);
    } else {
        term_clock[10][1] = Character::new('A', color);
    }
    // term_clock[10][1] = Character::new('L', color);
    // for i in 0..end_time.len() {
    //     term_clock[9][32 - i] = Character::new(end_time.pop().unwrap(), color);
    // }

}

pub(crate) fn update_exam_time(term_clock: &mut [[Character; 80]; 24], exam: &mut ExamStatus) {
    let exam_time_color = 32;

    add_start_end_duration(term_clock, exam, exam_time_color);
}