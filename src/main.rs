use std::io::stdout;
use std::io::Write;
use crossterm::event::read;
use crossterm::event::EnableMouseCapture;
use crossterm::event::KeyEvent;
use crossterm::event::Event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::size;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::style::Color;
use crossterm::queue;
use crossterm::cursor;

fn main() {
    // Constants
    const CELL_COLOR: Color = Color::Yellow;

    // Ignore first row and column for easier 1-based indexing
    let mut cells: Vec<Vec<bool>> = vec![
        vec![false; (size().unwrap().1 + 1).into()];
        (size().unwrap().0 + 1).into()
    ];

    // ToDo
    let mut stdout = stdout();

    // Configure terminal settings for optimal display and usage
    enable_raw_mode();
    queue!(stdout, EnterAlternateScreen, EnableMouseCapture, cursor::Hide);
    stdout.flush();

    // ToDo
    loop {
        let event = read().unwrap();
        match event {
            Event::Key(key_event) => {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        break;
                    },
                    _ => {},
                }
            },
            _ => {},
        }
        println!("{:?}", event);
        
    }


    // Restore terminal settings to default
    disable_raw_mode();
    queue!(stdout, DisableMouseCapture, cursor::Show, LeaveAlternateScreen);
    stdout.flush();
}

