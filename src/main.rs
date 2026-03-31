use std::{env, io};
use std::io::{stdout};

use crossterm::style::Print;
use crossterm::terminal;
use crossterm::{event::{read, Event}, cursor, execute, terminal::{ClearType, Clear, enable_raw_mode, disable_raw_mode}};
use crossterm::event::{self, KeyCode, KeyEvent};

mod gap_buffer;
use crate::gap_buffer::GapBuffer;

fn main() -> io::Result<()> {
    init_terminal()?;

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        open_homepage();
    } else if args.len() == 2 {
        load_buf_from_filename(&args[1])?;
    } else {
        println!("Error: Only 1 argument(file name) is supported");
        close()?;
        return Ok(());
    }

    let mut open_buffer = GapBuffer::from_text("Hello World.\r\nThis is some test text...\r\nOKOKOk");
    execute!(stdout(), Print(open_buffer.get_text()))?;

    loop {
        match read()? {
            Event::Key(event) => {
                let input = handle_key_event(event);
                open_buffer.input(&input);
            },
            _ => {}
        }

        let (row, col) = open_buffer.get_cursor_pos();
        execute!(stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            Print(open_buffer.get_text()),
            cursor::MoveTo(col.try_into().unwrap(), row.try_into().unwrap())
        )?;
    }
}

fn handle_key_event(e: KeyEvent) -> String {
    match e.code {
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Esc => "Escape".to_string(),
        _ => "Other".to_string()
    }
}

fn init_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(
        io::stdout(),
        terminal::EnterAlternateScreen,
        event::EnableBracketedPaste,
        event::EnableFocusChange,
        event::EnableMouseCapture,
        cursor::Show,
        cursor::MoveTo(0, 0),
    )?;

    Ok(())
}

fn close() -> io::Result<()> {
    execute!(
        io::stdout(),
        event::DisableBracketedPaste,
        event::DisableFocusChange,
        event::DisableMouseCapture,
        terminal::LeaveAlternateScreen
    )?;
    disable_raw_mode()
}

fn load_buf_from_filename(filename: &str) -> io::Result<()> {
    // TODO
    Ok(())
}

fn open_homepage() {

}

