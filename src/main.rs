use std::{ thread::sleep, time::Duration, io::Stdout };
use chrono::{ Local };
use color_char::Character;

mod clock;
use clock::update_time;

mod exam_timer;
use exam_timer::{ ExamStatus, update_exam_time };

use std::io::{ stdout, Write };
use crossterm::{ cursor, terminal, Result, ExecutableCommand };

fn display(term_clock: &[[Character; 80]; 24], stdout: &mut Stdout) -> Result<()> {
    let mut stdout_all = "".to_string();
    // for i in 0..24 {
    for i in 0..12 {
        for j in 0..40 {
        // for j in 0..80 {
            // write!(stdout, "\x1b[0;{}m{}\x1b[0m", term_clock[i][j].get_color(), term_clock[i][j])?;
            stdout_all += &format!("\x1b[0;{}m{}\x1b[0m", term_clock[i][j].get_color(), term_clock[i][j]);
        }
        // write!(stdout, "\n")?;
        stdout_all += "\n";
    }

    // TODO SOLUTION IS TO TRY CROSS TERM INSTEAD TO SOLVE FLICKERING ISSUE 

    // stdout.execute(cursor::MoveUp(24))?;
    stdout.execute(cursor::MoveUp(13))?;
    stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
    writeln!(stdout, "{}", stdout_all)?;

    Ok(())
}

fn main() -> Result<()> {
    // Initial end and start times
    let mut exam = ExamStatus { duration_hour: 2, duration_min: 30, start: Local::now() };
    let mut stdout = stdout(); // lock stdout and use the same locked instance throughout

    // Initial display
    let mut term_clock = [[Character::default(); 80]; 24];
    update_time(&mut term_clock);
    update_exam_time(&mut term_clock, &mut exam);
    display(&term_clock, &mut stdout)?;

    // Check time endlessly
    loop {
        // Update time every second
        sleep(Duration::from_millis(1000));

        // change and update display
        term_clock = [[Character::default(); 80]; 24];
        update_time(&mut term_clock);
        update_exam_time(&mut term_clock, &mut exam);

        display(&term_clock, &mut stdout)?;
    }
}
