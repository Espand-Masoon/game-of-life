use std::io::stdout;
use std::io::Write;
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

    // Global variables
    let mut terminal_width = size().unwrap().0;
    let mut terminal_height = size().unwrap().1;

    // Create a matrix to represent the terminal sheet
    let mut cells: Vec<Vec<bool>> = vec![
        vec![false; (terminal_height).into()];
        (terminal_width).into()
    ];

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
        Print("q: quit"),
    );
    stdout.flush();


    // ToDo: Comment
    loop {
        let event = read().unwrap();
        match event {
            Event::Key(key_event) => {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) |
                    (KeyCode::Char('q'), KeyModifiers::NONE) => {
                        break;
                    },
                    _ => {},
                }
            },
            Event::Mouse(mouse_event) => {
                match mouse_event.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        let row = mouse_event.row;
                        if row == 0 || row == terminal_height - 1 {}
                        else {
                            let column = mouse_event.column;
                            // ToDo : create a macro to toggle a bools value
                            // ToDo : do something for this repeatative use of into()
                            if cells[column as usize][row as usize] {
                                cells[column as usize][row as usize] = false;
                                queue!(stdout,SetBackgroundColor(BACKGROUND_COLOR));
                            } else {
                                cells[column as usize][row as usize] = true;
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
        // println!("{:?}", event);
        
    }


    // Restore terminal settings to default
    disable_raw_mode();
    queue!(stdout, DisableMouseCapture, cursor::Show, LeaveAlternateScreen);
    stdout.flush();
}

