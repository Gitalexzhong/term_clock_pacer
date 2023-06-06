use std::{ thread::sleep, time::Duration, io::Stdout, env };
use chrono::{ Local };
use color_char::Character;

use std::io::{ stdout, Write };
use crossterm::{ cursor::{ self, Hide }, style::Print, Result, queue };
use crossterm::execute;

mod clock;
use clock::update_time;

mod exam_timer;
use exam_timer::{ ExamStatus, update_exam_time };

mod type_defines;
use type_defines::StdTerm;

struct DisplayOptions {
    timer: bool,

}

fn clear_display() {
    print!("{}[2J", 27 as char);
}

fn display(prev_term_clock: &mut StdTerm, term_clock: &StdTerm, stdout: &mut Stdout) -> Result<()> {
    for i in 0..24 {
        for j in 0..80 {
            if term_clock[i][j] != prev_term_clock[i][j] {
                queue!(
                    stdout,
                    cursor::MoveTo(j as u16, i as u16),
                    Print(
                        format!(
                            "\x1b[0;{}m{}\x1b[0m",
                            term_clock[i][j].get_color(),
                            term_clock[i][j]
                        )
                    )
                )?;
            }
        }
    }

    prev_term_clock.clone_from(term_clock);
    stdout.flush()?;
    execute!(stdout, Hide)?;

    Ok(())
}

fn init_display(
    prev_term_clock: &mut StdTerm,
    term_clock: &mut StdTerm,
    stdout: &mut Stdout,
    exam: &mut ExamStatus,
    options: &DisplayOptions,
) -> Result<()> {
    clear_display();

    update_time(term_clock);
    if options.timer {
        update_exam_time(term_clock, exam);
    }

    display(prev_term_clock, &term_clock, stdout)?;

    Ok(())
}

fn main() -> Result<()> {
    // Initial end and start times
    // let mut exam = ExamStatus { duration_hour: 2, duration_min: 30, start: Local::now() };

    let args: Vec<String> = env::args().collect();
    
    let mut exam = ExamStatus { duration_hour: 0, duration_min: 1, start: Local::now() };

    let display_options = DisplayOptions { timer: true };

    let mut stdout = stdout(); // lock stdout and use the same locked instance throughout

    // Initial display
    let mut term_clock = [[Character::default(); 80]; 24];
    let mut prev_term_clock = [[Character::default(); 80]; 24];
    init_display(&mut prev_term_clock, &mut term_clock, &mut stdout, &mut exam, &display_options)?;

    // Check time endlessly
    loop {
        // Update time every second
        sleep(Duration::from_millis(1000));

        // change and update display
        term_clock = [[Character::default(); 80]; 24];
        update_time(&mut term_clock);
        if display_options.timer {
            update_exam_time(&mut term_clock, &mut exam);
        }

        display(&mut prev_term_clock, &term_clock, &mut stdout)?;
    }
}
