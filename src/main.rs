use grux::GridWriter;

fn add_big_number(display: &mut String, start: u32, digit: u32) {
    match digit {
        1 => {
            // for row in start+4..start+5 {
            //     for col in 1..5 {
            //         display.set((row as usize, col), '█');
            //     }
            // }
            // Error seams to do with digit being replaced (cross talk)
            display.set((5,1), '█');
            display.set((5,1), '█');
            display.set((5,2), '█');


        },
        _ => todo!()
    }

}

fn display(term_clock: &String) {
    print!("{}[2J", 27 as char);
    // print!("{esc}[2J{esc}[0;0H", esc = 27 as char);
    // print!("\x1B[2J");

    println!("{}", term_clock);
    println!("");
}

fn main() {
    let mut term_clock = String::new();
    // term_clock.set((81,25), ' ');

    add_big_number(&mut term_clock, 1, 1);

    display(&term_clock);
    // add_big_number(&mut term_clock, 8, 1);

    // display(&term_clock);
}