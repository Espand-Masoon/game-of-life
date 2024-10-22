use std::io::Stdout;
use std::io::Write;
use std::fmt::Display;
use crossterm::queue;
use crossterm::cursor;
use crossterm::style::Print;
use crossterm::style::Color;
use crossterm::style::SetBackgroundColor;

const BACKGROUND_COLOR: Color = Color::Black;

// ToDo: check if it's a good idea to constrain generation to
// unsigned integers
pub fn print_generation<T: Display>(stdout: &mut Stdout, generation: T) {
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        SetBackgroundColor(BACKGROUND_COLOR),
        Print(format!("{:<25}", format!("Generation: {:<13}", generation))),
    );
    stdout.flush();
}

pub fn print_population<T: Display>(stdout: &mut Stdout, population: T) {
    queue!(
        stdout,
        cursor::MoveTo(0, 1),
        SetBackgroundColor(BACKGROUND_COLOR),
        Print(format!("{:<25}", format!("Population: {:<13}", population))),
    );
    stdout.flush();
}

pub fn print_speed(stdout: &mut Stdout, delay: u8) {
    queue!(
        stdout,
        cursor::MoveTo(26, 0),
        SetBackgroundColor(BACKGROUND_COLOR),
        Print(format!("Speed: {:<10}", 100 - delay)),
    );
    stdout.flush();
}
