use crossterm::cursor;
use crossterm::queue;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::SetBackgroundColor;
use crossterm::terminal::{Clear, ClearType};
use std::fmt::Display;
use std::io;
use std::io::Stdout;
use std::io::Write;

use crate::grid::Grid;

// Todo: These constants should be stored in one place and be synced
const BACKGROUND_COLOR: Color = Color::Black;
const CELL_COLOR: Color = Color::Yellow;
const TOP_MARGIN: u16 = 2;

// ToDo: check if it's a good idea to constrain generation to
// unsigned integers
pub fn print_generation<T: Display>(stdout: &mut Stdout, generation: T) -> Result<(), io::Error> {
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        SetBackgroundColor(BACKGROUND_COLOR),
        Print(format!("{:<25}", format!("Generation: {:<13}", generation))),
    )?;
    stdout.flush()?;
    Ok(())
}

pub fn print_population<T: Display>(stdout: &mut Stdout, population: T) -> Result<(), io::Error> {
    queue!(
        stdout,
        cursor::MoveTo(0, 1),
        SetBackgroundColor(BACKGROUND_COLOR),
        Print(format!("{:<25}", format!("Population: {:<13}", population))),
    )?;
    stdout.flush()?;
    Ok(())
}

pub fn print_speed(stdout: &mut Stdout, delay: u8) -> Result<(), io::Error> {
    queue!(
        stdout,
        cursor::MoveTo(26, 0),
        SetBackgroundColor(BACKGROUND_COLOR),
        Print(format!("Speed: {:<10}", 100 - delay)),
    )?;
    stdout.flush()?;
    Ok(())
}

pub fn print_cells(stdout: &mut Stdout, grid: &Grid) -> Result<(), io::Error> {
    for width in 0..grid.width {
        for height in 0..grid.height {
            if grid[(width, height)] {
                queue!(stdout, SetBackgroundColor(CELL_COLOR))?;
            } else {
                queue!(stdout, SetBackgroundColor(BACKGROUND_COLOR))?;
            }
            queue!(
                stdout,
                cursor::MoveTo(width, height + TOP_MARGIN),
                Print(' '),
            )?;
        }
    }

    stdout.flush()?;

    Ok(())
}

pub fn print_ribbon_bottom(stdout: &mut Stdout, terminal_height: u16) -> Result<(), io::Error> {
    queue!(
        stdout,
        cursor::MoveTo(0, terminal_height - 1),
        Clear(ClearType::CurrentLine),
        Print("q: quit    p: pause    speed: +-"),
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn print_ribbon_top(
    stdout: &mut Stdout,
    generation: usize,
    delay: u8,
    population: usize,
) -> Result<(), io::Error> {
    queue!(
        stdout,
        cursor::MoveToRow(0),
        Clear(ClearType::CurrentLine),
        cursor::MoveToRow(1),
        Clear(ClearType::CurrentLine)
    )?;
    print_generation(stdout, generation)?;
    print_speed(stdout, delay)?;
    print_population(stdout, population)?;
    Ok(())
}
