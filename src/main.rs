mod printer;
use printer::{print_generation, print_population, print_speed};

use std::io::stdout;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;
use std::ops::Index;
use std::ops::IndexMut;
use crossterm::event::poll;
use crossterm::event::read;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crossterm::event::MouseEventKind;
use crossterm::event::MouseButton;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::style::SetBackgroundColor;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::queue;
use crossterm::execute;
use crossterm::cursor;

// Consider creating an Index struct with width and height fields

// store all the rows in one vec contiguously
//
// ------------------------> +width
// |
// |
// |
// |
// |
// |
// +
// heigth
#[derive(Clone)]
struct Grid {
    width: u16,
    height: u16,
    cells: Vec<bool>,
    population: usize,
    generation: usize
}

impl Grid {
    fn new(width: u16, height: u16) -> Self {
        Grid {
            width,
            height,
            cells: vec![false; (width * height) as usize],
            // Todo: Check if population or generation aren't going out of bound
            population: 0,
            generation: 0,
        }
    }

    fn next_generation(&mut self) {
        let mut should_die_indices = Vec::<(u16, u16)>::new();
        let mut should_birth_indices = Vec::<(u16, u16)>::new();

        for width in 0..self.width {
            for height in 0..self.height {
                let mut alive_neighbors_count = 0_u8;
                let neighbor_offsets: [(isize, isize); 8] =
                [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];
                for neighbor_offset in neighbor_offsets {
                    let neighbor_width = width as isize + neighbor_offset.0;
                    let neighbor_height = height as isize + neighbor_offset.1;
                    if neighbor_width < 0 || neighbor_height < 0 {
                        continue;
                    }
                    // Todo: Check for the numeric type and make sure no overflow happens
                    if let Some(is_alive) = self.get(neighbor_width as u16, neighbor_height as u16) {
                        if *is_alive {
                            alive_neighbors_count = alive_neighbors_count + 1;
                        }
                    }
                }
                if self[(width, height)] {
                    if alive_neighbors_count < 2 || alive_neighbors_count > 3 {
                        should_die_indices.push((width, height));
                    }
                } else {
                    if alive_neighbors_count == 3 {
                        should_birth_indices.push((width, height));
                    }
                }
            }
        }

        // fn count_alive_neighbors(&self, width, height) -> u8 {
        //     
        // }

        self.population = self.population + should_birth_indices.len() - should_die_indices.len();

        for index in should_die_indices {
            self[index] = false;
        }

        for index in should_birth_indices {
            self[index] = true;
        }

        self.generation = self.generation + 1;
    }

    fn get(&self, width: u16, height: u16) -> Option<&bool> {
        // Return None if the index is out of bound
        if width >= self.width || height >= self.height {
            return None;
        }

        let width = width as usize;
        let height = height as usize;
        let grid_width = self.width as usize;
        // Todo: Check if the number doesn't go out of bound of usize
        self.cells.get(height * grid_width + width)
    }
}

impl Index<(u16, u16)> for Grid {
    type Output = bool;

    fn index(self: &Self, index: (u16, u16)) -> &Self::Output {
        let width = index.0 as usize;
        let height = index.1 as usize;
        let grid_width = self.width as usize;
        &self.cells[height * grid_width + width]
    }
}

impl IndexMut<(u16, u16)> for Grid {
    fn index_mut(self: &mut Self, index: (u16, u16)) -> &mut Self::Output {
        let width = index.0 as usize;
        let height = index.1 as usize;
        let grid_width = self.width as usize;
        &mut self.cells[height * grid_width + width]
    }
}


fn main() {
    // ToDo : overwrite the size() function to always return usize
    // so you don't have to convert it everywhere you need it

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
    enable_raw_mode();
    queue!(
        stdout,
        EnterAlternateScreen,
        SetBackgroundColor(BACKGROUND_COLOR),
        Clear(ClearType::All),
        EnableMouseCapture,
        cursor::Hide
    );
    stdout.flush();

    // Print help ribbon at bottom of pane
    queue!(
        stdout,
        cursor::MoveTo(0, terminal_height - 1),
        Print("q: quit    p: pause    speed: +-"),
    );
    stdout.flush();

    // Print top ribbon
    print_generation(&mut stdout, grid.generation);
    print_population(&mut stdout, grid.population);
    print_speed(&mut stdout, delay);

    // ToDo: Comment
    let mut start = Instant::now();
    loop {
        // Read an event
        if poll(Duration::from_millis(5)).unwrap() {
            match read().unwrap() {
                Event::Key(key_event) => {
                    match (key_event.code, key_event.modifiers) {
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) |
                        (KeyCode::Char('q'), KeyModifiers::NONE) => {
                            break;
                        },
                        (KeyCode::Char('p'), KeyModifiers::NONE) => {
                            if game_is_paused {
                                execute!(stdout, DisableMouseCapture);
                            } else {
                                execute!(stdout, EnableMouseCapture);
                            }
                            game_is_paused = !game_is_paused;
                        },
                        (KeyCode::Char('+'), KeyModifiers::NONE) => {
                            if delay > 0 {
                                delay -= 1;
                                print_speed(&mut stdout, delay);
                            }
                        },
                        (KeyCode::Char('-'), KeyModifiers::NONE) => {
                            if delay < 99 {
                                delay += 1;
                                print_speed(&mut stdout, delay);
                            }
                        },
                        _ => {},
                    }
                },
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            let height = mouse_event.row;
                            if !(height < TOP_MARGIN || height >= terminal_height - BOTTOM_MARGIN) {
                                let width = mouse_event.column;
                                // ToDo : create a macro to toggle a bools value
                                // ToDo : do something for this repeatative use of into()
                                if grid[(width, height - TOP_MARGIN)] {
                                    grid[(width, height - TOP_MARGIN)] = false;
                                    grid.population -= 1;
                                    queue!(stdout,SetBackgroundColor(BACKGROUND_COLOR));
                                } else {
                                    grid[(width, height - TOP_MARGIN)] = true;
                                    grid.population += 1;
                                    queue!(stdout,SetBackgroundColor(CELL_COLOR));
                                }
                                queue!(
                                    stdout,
                                    cursor::MoveTo(width, height),
                                    Print(' '),
                                );
                                stdout.flush();
                                print_population(&mut stdout, grid.population);
                            }
                        },
                        _ => {}
                    }
                },
                // Event::Resize(columns, rows) => {
                //     for row in cells
                // },
                _ => {},
            }
        }

        // Check if game is paused
        if game_is_paused || start.elapsed() < Duration::from_millis(8 * (delay as u64) + 250) {
            continue;
        }

        // Generate next generation grid
        grid.next_generation();

        // Print cells
        for width in 0..grid.width {
            for height in 0..grid.height {
                if grid[(width, height)] {
                    queue!(stdout, SetBackgroundColor(CELL_COLOR));
                } else {
                    queue!(stdout, SetBackgroundColor(BACKGROUND_COLOR));
                }
                queue!(
                    stdout,
                    cursor::MoveTo(width, height + TOP_MARGIN),
                    Print(' '),
                );
            }
        }

        // Print top ribbon
        print_generation(&mut stdout, grid.generation);
        print_population(&mut stdout, grid.population);

        // Reset the instant
        start = Instant::now();
    }

    // Restore terminal settings to default
    disable_raw_mode();
    queue!(stdout, DisableMouseCapture, cursor::Show, LeaveAlternateScreen);
    stdout.flush();
}
