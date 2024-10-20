use std::io::stdout;
use std::io::Write;
use std::time::Duration;
use crossterm::event::poll;
use crossterm::event::read;
use crossterm::event::EnableMouseCapture;
use crossterm::event::KeyEvent;
use crossterm::event::Event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crossterm::event::MouseEventKind;
use crossterm::event::MouseButton;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::size;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::style::SetBackgroundColor;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::queue;
use crossterm::cursor;

fn main() {
    // ToDo : overwrite the size() function to always return usize
    // so you don't have to convert it everywhere you need it

    // Constants
    const CELL_COLOR: Color = Color::Yellow;
    const BACKGROUND_COLOR: Color = Color::Black;
    const TOP_MARGIN: u16 = 2;
    const BOTTOM_MARGIN: u16 = 1;

    // Global variables
    // ToDo: Create two variables with different types to avoid coversion
    let mut terminal_width = size().unwrap().0;
    let mut terminal_height = size().unwrap().1;
    let mut game_is_paused = true;

    // Create a matrix to represent the terminal sheet
    // rows of cells
    let mut cells: Vec<Vec<bool>> = vec![
        vec![false; terminal_width.into()];
        terminal_height.into()
    ];
    let mut next_gen_cells = cells.clone();

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
        Print("q: quit    p: pause"),
    );
    stdout.flush();


    // ToDo: Comment
    loop {
        // Wait for an event
        if poll(Duration::from_millis(1000)).unwrap() {
            match read().unwrap() {
                Event::Key(key_event) => {
                    match (key_event.code, key_event.modifiers) {
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) |
                        (KeyCode::Char('q'), KeyModifiers::NONE) => {
                            break;
                        },
                        (KeyCode::Char('p'), KeyModifiers::NONE) => {
                            game_is_paused = !game_is_paused;
                        },
                        _ => {},
                    }
                },
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            let row = mouse_event.row;
                            if row < TOP_MARGIN || row >= terminal_height - BOTTOM_MARGIN {}
                            else {
                                let column = mouse_event.column;
                                // ToDo : create a macro to toggle a bools value
                                // ToDo : do something for this repeatative use of into()
                                if cells[row as usize][column as usize] {
                                    cells[row as usize][column as usize] = false;
                                    queue!(stdout,SetBackgroundColor(BACKGROUND_COLOR));
                                } else {
                                    cells[row as usize][column as usize] = true;
                                    queue!(stdout,SetBackgroundColor(CELL_COLOR));
                                }
                                queue!(
                                    stdout,
                                    cursor::MoveTo(column, row),
                                    Print(' '),
                                    cursor::MoveTo(0, 0),
                                );
                                stdout.flush();
                            }
                        },
                        _ => {}
                    }
                }
                _ => {},
            }
        }

        // Check if game is paused
        if game_is_paused {
            continue;
        }

        // Generate next generation cells
        for row_index in (TOP_MARGIN as usize)..(terminal_height - BOTTOM_MARGIN).into() {
            for column_index in 0_usize..(terminal_width - 1).into() {
                let mut true_neighbors: u8 = 0;
                let neighbors_differences: [(isize, isize); 8] = 
                    [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];
                for &(x, y) in neighbors_differences.iter() {
                    if row_index as isize + y >= 0 || column_index as isize + x >= 0 {
                        match cells.get((row_index as isize + y) as usize) {
                            Some(row) => {
                                match row.get((column_index as isize + x) as usize) {
                                    Some(&cell) => {
                                        if cell {true_neighbors += 1;}
                                    },
                                    None => {},
                                }
                            },
                            None => {},
                        }
                    }
                }
                match true_neighbors {
                    0..=1 => {
                        next_gen_cells[row_index][column_index] = false;
                    },
                    2 => {},
                    3 => {
                        next_gen_cells[row_index][column_index] = true;
                    },
                    _ => {
                        next_gen_cells[row_index][column_index] = true;
                    },
                }
            }
        }
        cells = next_gen_cells.clone();
        
        // Print cells
        for row_index in (TOP_MARGIN as usize)..(terminal_height - BOTTOM_MARGIN).into() {
            for column_index in 0_usize..(terminal_width - 1).into() {
                if cells[row_index][column_index] {
                    queue!(stdout, SetBackgroundColor(CELL_COLOR));
                } else {
                    queue!(stdout, SetBackgroundColor(BACKGROUND_COLOR));
                }
                queue!(
                    stdout,
                    cursor::MoveTo(column_index as u16, row_index as u16),
                    Print(' '),
                );
            }
        }
    }

    // Restore terminal settings to default
    disable_raw_mode();
    queue!(stdout, DisableMouseCapture, cursor::Show, LeaveAlternateScreen);
    stdout.flush();
}
