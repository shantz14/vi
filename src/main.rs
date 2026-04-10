use std::{env, fs};
use std::io::{self, stdout};
use std::sync::OnceLock;

use crossterm::style::Print;
use crossterm::{event::{self, read, Event, KeyCode, KeyEvent}, cursor, execute, terminal::{self, ClearType, Clear, enable_raw_mode, disable_raw_mode}};

mod buffer;
use crate::buffer::Buffer;
mod logger;
use crate::logger::{LogLevel, Logger};

static LOGGER: OnceLock<Logger> = OnceLock::new();

fn main() -> io::Result<()> {
    init_terminal()?;

    let mut open_buf: Buffer;
    let mut verbosity = 0;

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        open_buf = open_homepage();
    } else {
        // handle filename
        if args[1].chars().nth(0).unwrap() != '-' {
            open_buf = Buffer::load_buf_from_filename(&args[1])?;
        } else {
            open_buf = open_homepage();
        }
        // handle flags
        args[1..].iter().for_each(|arg| {
            if arg.chars().nth(0).unwrap() == '-' {
                match arg.as_str() {
                    "-V1" => {
                        verbosity = 1;
                    },
                    "-V2" => {
                        verbosity = 2;
                    },
                    _ => {
                        let _ = close();
                        panic!("unknown flag");
                    }
                }
            }
        });
    }

    // Init logger
    let _ = match verbosity {
        0 => LOGGER.set(Logger {level: LogLevel::WARN}),
        1 => LOGGER.set(Logger {level: LogLevel::INFO}),
        2 => LOGGER.set(Logger {level: LogLevel::DEBUG}),
        _ => {
            close()?;
            panic!("invalid verbosity");
        }
    };

    execute!(stdout(), Print(open_buf.gb.get_text()))?;

    loop {
        match read()? {
            Event::Key(event) => {
                let input = handle_key_event(event);
                open_buf.input(&input);
            },
            _ => {}
        }

        let (row, col) = open_buf.gb.get_cursor_pos();
        execute!(stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            Print(open_buf.gb.get_text()),
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

fn open_homepage() -> Buffer {
    // return Buffer {
    //     name: "homepage".to_string(),
    //     gb: GapBuffer::from_text("This is the homepage"),
    //     mode: buffer::Mode::N,
    //     cmd: "".to_string(),
    // }
    !unimplemented!();
}

