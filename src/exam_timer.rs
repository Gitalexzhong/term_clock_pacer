use color_char::Character;
use chrono::{ Local, Timelike, DateTime, Duration };

pub(crate) struct ExamStatus {
    pub(crate) start: DateTime<Local>,
    pub(crate) duration_hour: i64,
    pub(crate) duration_min: i64,
}


pub(crate) fn update_exam_time(term_clock: &mut [[Character; 80]; 24], exam: &mut ExamStatus) {
    let exam_time_color = 32;

    // Display start time
    let mut start_time = exam.start.hour().to_string() + ":" + &exam.start.minute().to_string();
    for i in 0..start_time.len() {
        term_clock[9][i + 1] = Character::new(start_time.remove(0), exam_time_color);
    }

    // Display end time
    let end_time = exam.start + Duration::hours(exam.duration_hour) + Duration::minutes(exam.duration_min);
    let mut end_time = end_time.hour().to_string() + ":" + &end_time.minute().to_string() + ":" + &end_time.second().to_string();
    for i in 0..end_time.len() {
        term_clock[9][32 - i] = Character::new(end_time.pop().unwrap(), exam_time_color);
    }
}
