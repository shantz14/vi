#![allow(non_snake_case)]

mod gap_buffer;
use std::{fs::{self, File}, io::{Error, ErrorKind}};

use crate::{buffer::gap_buffer::GapBuffer, log_debug, log_error};

pub enum Mode { N = 0, I, V, C }

// Higher level buffer struct that is not concerned with the workings of
// the GapBuffer data structure
pub struct Buffer {
    pub name: String,
    pub gb: GapBuffer,
    pub mode: Mode,
    pub cmd: String,
}

impl Buffer {
    pub fn load_buf_from_filename(filename: &str) -> Result<Buffer, Error> {
        match File::open(filename) {
            Ok(_file) => {
                // New buffer from the found file
                return Ok(Buffer {
                    name: filename.to_string(),
                    // Problem i open the file twice maybe idk maybe pass the
                    // file ptr
                    gb: GapBuffer::from_file(filename),
                    mode: Mode::N,
                    cmd: "".to_string(),
                });
            },
            Err(error) => {
                if error.kind() == ErrorKind::NotFound {
                    // New buffer with name of filename
                    return Ok(Buffer {
                        name: filename.to_string(),
                        gb: GapBuffer::from_text(""),
                        mode: Mode::N,
                        cmd: "".to_string(),
                    });
                } else {
                    // Error opening file that does exist? Throw?
                    return Err(error);
                }
            }
        }
    }

    pub fn input(&mut self, c: &str) {
        match self.mode {
            Mode::N => {
                self.input_n(c);
            }
            Mode::I => {
                self.input_i(c);
            }
            Mode::V => {
                self.input_v(c);
            }
            Mode::C => {
                self.input_c(c);
            }
        }

        log_debug!("Row: {} Col: {}", self.gb.get_row(), self.gb.get_col());
    }

    fn input_n(&mut self, c: &str) {
        match c {
            "h" => self.gb.n_h(),
            "l" => self.gb.n_l(),
            "j" => self.gb.n_j(),
            "k" => self.gb.n_k(),
            "i" => self.mode = Mode::I,
            "a" => {
                self.gb.n_a();
                self.mode = Mode::I;
            },
            "o" => {
                self.gb.n_o();
                self.mode = Mode::I;
            },
            "O" => {
                self.gb.n_O();
                self.mode = Mode::I;
            }
            "$" => self.gb.n_dolla(),
            "0" => self.gb.n_0(),
            "w" => self.gb.n_w(),
            "b" => self.gb.n_b(),
            "W" => self.gb.n_W(),
            "B" => self.gb.n_B(),
            ":" => self.mode = Mode::C,
            _ => {
                //mmmm
            }
        }
    }

    fn input_i(&mut self, c: &str) {
        match c {
            "Escape" => self.mode = Mode::N,
            "Backspace" => self.gb.backspace(),
            "Tab" => self.gb.tab(),
            "Enter" => self.gb.enter(),
            "Shift" => {},
            _ => {
                self.gb.insert(&c.to_string());
            }
        }
    }

    fn input_v(&mut self, _c: &str) {

    }

    fn input_c(&mut self, c: &str) {
        match c {
            "Enter" => {
                self.submit_command();
            },
            _ => {
                if c.len() == 1 {
                    self.cmd.push_str(c);
                }
            }
        }
    }

    fn submit_command(&mut self) {
        // Super basic rn
        let cmd = &self.cmd;

        let args: Vec<&str> = cmd.split(" ").collect();
        let command = args[0];
        let mut arg1: Option<&str> = None;
        if args.len() == 1 {
            arg1 = None;
        } else if args.len() == 2 {
            arg1 = Some(args[1]);
        }

        match command {
            "w" | "write" => {
                self.write(arg1);
                self.mode = Mode::N;
            },
            "q" | "quit" => {
                let _ = crate::close();
                std::process::exit(0);
            },
            "Escape" => {
                self.mode = Mode::N;
            },
            _ => {
                //nothin
            }
        }
        self.cmd.clear();
    }

    fn write(&self, filename: Option<&str>) {
        match filename {
            None => {
                if let Err(e) = fs::write(&self.name, self.gb.get_text()) {
                    log_error!("failed to write buffer to file: {}", e);
                }
            },
            Some(filename) => {
                if let Err(e) = fs::write(filename, self.gb.get_text()) {
                    log_error!("failed to write buffer to file: {}", e);
                }
            }
        }
    }

}
