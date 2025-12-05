mod printer;
use printer::{print_generation, print_population, print_speed};

mod grid;
use grid::Grid;

use crossterm::cursor;
use crossterm::event::poll;
use crossterm::event::read;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crossterm::event::MouseButton;
use crossterm::event::MouseEventKind;
use crossterm::execute;
use crossterm::queue;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::SetBackgroundColor;
use crossterm::terminal;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;

use crate::printer::print_cells;
use crate::printer::print_ribbon_bottom;
use crate::printer::print_ribbon_top;

fn main() {
    match run() {
        Ok(()) => (),
        Err(error) => {
            eprintln!("{error}");
        }
    }
}

fn run() -> Result<(), std::io::Error> {
    // Constants
    const CELL_COLOR: Color = Color::Yellow;
    const BACKGROUND_COLOR: Color = Color::Black;
    const TOP_MARGIN: u16 = 2;
    const BOTTOM_MARGIN: u16 = 1;
    const VERTICAL_MARGIN: u16 = TOP_MARGIN + BOTTOM_MARGIN;

    // Global variables
    let mut terminal_width: u16 = terminal::size().unwrap().0;
    let mut terminal_height: u16 = terminal::size().unwrap().1;
    let mut game_is_paused = true;
    let mut delay: u8 = 50;

    // Create a grid to represent the terminal sheet
    let mut grid = Grid::new(terminal_width, terminal_height - VERTICAL_MARGIN);

    // ToDo
    let mut stdout = stdout();

    // Configure terminal settings for optimal display and usage
    enable_raw_mode()?;
    queue!(
        stdout,
        EnterAlternateScreen,
        SetBackgroundColor(BACKGROUND_COLOR),
        Clear(ClearType::All),
        EnableMouseCapture,
        cursor::Hide
    )?;
    stdout.flush()?;

    // Print help ribbon at bottom of pane
    print_ribbon_bottom(&mut stdout, terminal_height)?;

    // Print top ribbon
    print_ribbon_top(&mut stdout, grid.generation, delay, grid.population)?;

    // TODO: Comment
    let mut start = Instant::now();
    loop {
        // Read an event
        if poll(Duration::from_millis(5)).unwrap() {
            match read().unwrap() {
                Event::Key(key_event) => match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL)
                    | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                        break;
                    }
                    (KeyCode::Char('p'), KeyModifiers::NONE) => {
                        if game_is_paused {
                            execute!(stdout, DisableMouseCapture)?;
                        } else {
                            execute!(stdout, EnableMouseCapture)?;
                        }
                        game_is_paused = !game_is_paused;
                    }
                    (KeyCode::Char('+'), KeyModifiers::NONE) => {
                        if delay > 0 {
                            delay -= 1;
                            print_speed(&mut stdout, delay)?;
                        }
                    }
                    (KeyCode::Char('-'), KeyModifiers::NONE) => {
                        if delay < 99 {
                            delay += 1;
                            print_speed(&mut stdout, delay)?;
                        }
                    }
                    _ => {}
                },
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        let height = mouse_event.row;
                        if !(height < TOP_MARGIN || height >= terminal_height - BOTTOM_MARGIN) {
                            let width = mouse_event.column;

                            if grid[(width, height - TOP_MARGIN)] {
                                grid.toggle_cell((width, height - TOP_MARGIN));
                                queue!(stdout, SetBackgroundColor(BACKGROUND_COLOR))?;
                            } else {
                                grid.toggle_cell((width, height - TOP_MARGIN));
                                queue!(stdout, SetBackgroundColor(CELL_COLOR))?;
                            }
                            queue!(stdout, cursor::MoveTo(width, height), Print(' '),)?;
                            stdout.flush()?;
                            print_population(&mut stdout, grid.population)?;
                        }
                    }
                    _ => {}
                },
                Event::Resize(columns, rows) => {
                    terminal_width = terminal::size().unwrap().0;
                    terminal_height = terminal::size().unwrap().1;
                    grid.resize(columns, rows - VERTICAL_MARGIN);
                    if game_is_paused {
                        print_cells(&mut stdout, &grid)?;
                        print_ribbon_top(&mut stdout, grid.generation, delay, grid.population)?;
                        print_ribbon_bottom(&mut stdout, terminal_height)?;
                    }
                }
                _ => {}
            }
        }

        // Check if game is paused
        if game_is_paused || start.elapsed() < Duration::from_millis(8 * (delay as u64) + 250) {
            continue;
        }

        // Generate next generation grid
        grid.next_generation();

        // Print cells
        print_cells(&mut stdout, &grid)?;

        // Print top ribbon
        print_generation(&mut stdout, grid.generation)?;
        print_population(&mut stdout, grid.population)?;

        // Reset the instant
        start = Instant::now();
    }
    quit(stdout)?;
    Ok(())
}

fn quit(mut stdout: io::Stdout) -> Result<(), io::Error> {
    // Restore terminal settings to default
    disable_raw_mode()?;
    queue!(
        stdout,
        DisableMouseCapture,
        cursor::Show,
        LeaveAlternateScreen
    )?;
    stdout.flush()?;
    Ok(())
}
