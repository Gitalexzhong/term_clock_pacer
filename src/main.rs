use std::{ thread::sleep, time::Duration, io::Stdout, env };
use chrono::{ Local, Duration as Dur };
// use chrono::{DateTime, Local, Duration};
use color_char::Character;

use std::io::{ stdout, Write };
use crossterm::{ cursor::{ self, Hide }, style::Print, Result, queue };
use crossterm::execute;
use crossterm::event::{read, Event, KeyCode};

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
    options: &DisplayOptions
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

    loop {
        match read().unwrap() {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => println!("char typed: {}", c),
                KeyCode::Enter => println!("enter key pressed"),
                KeyCode::Esc => println!("esc key pressed"),
                _ => {}
            },
            Event::Mouse(event) => println!("{:?}", event),
            _ => {print!(".");}
        }
    }

    return Ok(()); 

    let args: Vec<String> = env::args().collect();
    let display_options;
    let mut exam;
    // let mut exam = ExamStatus { duration_hour: 0, duration_min: 1, start: Local::now()};

    if args.len() == 1 {
        display_options = DisplayOptions { timer: false };
        exam = ExamStatus { duration_hour: 0, duration_min: 0, start: Local::now()};
    } else if args.len() == 2 {
        // Option to have current time as basis of exam start time.
        display_options = DisplayOptions { timer: true };
        let mins_str = args.get(1).unwrap().to_owned();
        let mins = mins_str.parse::<i64>();

        match mins {
            Ok(time) => {
                exam = ExamStatus { duration_hour: time/60, duration_min: time%60, start: Local::now() + Dur::minutes(1)};
            }
            Err(e) => {
                return Ok(());
            }
        }
    } else {
        // Required error checking
        return Ok(());
    }

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
