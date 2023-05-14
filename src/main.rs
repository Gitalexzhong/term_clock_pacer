use grux::GridWriter;

fn main() {

    let mut term_clock = String::new();
    term_clock.set((1, 2), '█');

    display(&term_clock);
    display(&term_clock);
}

fn display(term_clock: &String) {
    print!("{}[2J", 27 as char);
    println!("{}", term_clock);
    println!("");
}