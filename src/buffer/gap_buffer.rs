use std::fs::{self};

const GAP_SIZE: usize = 256;

enum CharClass {
    Alpha,
    Whitespace,
    Symbol,
}

pub struct GapBuffer {
    l: usize,
    r: usize,
    buf: Vec<char>,
    gap_size: usize,
    row: usize,
    col: usize,
    abs_col: usize,
    abs_col_flag: bool,
}

impl GapBuffer {
    pub fn from_text(text: &str) -> GapBuffer {
        let mut gap: Vec<char> = vec!['\0'; GAP_SIZE];
        gap.append(&mut text.chars().collect());
        GapBuffer {
            l: 0,
            r: GAP_SIZE - 1,
            buf: gap,
            gap_size: GAP_SIZE,
            row: 0,
            col: 0,
            abs_col: 0,
            abs_col_flag: true,
        }
    }

    pub fn get_text(&self) -> String {
        let mut out = String::new();
        self.buf.iter().for_each(|c| {
            if *c == '\n' {
                out.push_str("\r\n");
            } else {
                out.push(*c);
            }
        });

        out
    }

    pub fn get_cursor_pos(&self) -> (usize, usize) {
        (self.get_row(), self.get_col())
    }
}

impl GapBuffer {
    fn down(&mut self) {
        if self.on_last_line() {
            return;
        }
        if self.abs_col_flag {
            self.abs_col = self.col;
        }
        // Move to start of next line
        self.move_relative(1);
        while self.buf[self.l-1] != '\n' {
            self.move_relative(1);
        }
        // Right until end of line or abs_col or end of buffer
        while self.r != self.buf.len()-1 && self.buf[self.r+1] != '\n' && self.col != self.abs_col {
            self.move_relative(1);
        }
        self.abs_col_flag = self.col == self.abs_col;
    }

    fn up(&mut self) {
        if self.on_first_line() {
            return;
        }
        if self.abs_col_flag {
            self.abs_col = self.col;
        }
        // Move to end of previous line
        self.move_relative(-1);
        while self.buf[self.r+1] != '\n' {
            self.move_relative(-1);
        }
        // Dont move if col is less then abs_col
        // Otherwise left until col == abs_col
        while self.col > self.abs_col && self.l > 0 {
            self.move_relative(-1);
        }
        self.abs_col_flag = self.col == self.abs_col;
    }

    pub fn move_relative(&mut self, distance: i32) {
        if distance.is_negative() {
            let usize_dist = (-distance) as usize;
            if usize_dist > self.l {
                return;
            }
            self.move_absolute(self.l - usize_dist);
        } else {
            let usize_dist = distance as usize;
            self.move_absolute(self.l + usize_dist);
        }
    }

    fn move_absolute(&mut self, cursor_pos: usize) {
        if cursor_pos < self.l {
            self.left(cursor_pos);
        } else if cursor_pos > self.l {
            self.right(cursor_pos);
        }
    }

    pub fn insert(&mut self, s: &str) {
        if s.len() >= self.gap_size {
            self.resize();
        }
        self.buf.splice(self.l..self.l+s.len(), s.chars());
        self.l += s.len();
        self.gap_size -= s.len();
        self.col = self.find_col();
        // TODO this is slow
        self.row += s.matches('\n').count();
    }

    fn end_of_line(&mut self) {
        while self.r < self.buf.len()-1 && self.buf[self.r+1] != '\n' {
            self.move_relative(1);
        }
    }

    fn start_of_line(&mut self) {
        while self.l > 0 && self.buf[self.l-1] != '\n' {
            self.move_relative(-1);
        }
    }

    pub fn backspace(&mut self) {
        self.delete(self.l-1, self.l-1);
        self.col -= 1;
    }

    pub fn tab(&mut self) {
        self.insert("    ");
    }

    pub fn from_filename(file_name: &str) -> GapBuffer {
        let mut gap: Vec<char> = vec!['\0'; GAP_SIZE];
        let mut file_contents: Vec<char> = fs::read_to_string(file_name)
            .expect("Failed to read file.")
            .chars()
            .collect();
        gap.append(&mut file_contents);
        GapBuffer {
            l: 0,
            r: GAP_SIZE - 1,
            buf: gap,
            gap_size: GAP_SIZE,
            row: 0,
            col: 0,
            abs_col: 0,
            abs_col_flag: true,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut cursor_found = false;
        for c in self.buf.iter() {
            if *c == '\0' && !cursor_found {
                print!("[CURSOR]");
                cursor_found = true;
            }
            if *c != '\0' {
                print!("{}", c);
            }
        }
    }

    #[allow(dead_code)]
    fn print_debug(&self) {
        let buf = self.get_buf();
        println!("{:?}", buf);
        //println!("{}", self.col);
    }

    fn get_buf(&self) -> Vec<char> {
        let buf = self.buf.clone();
        buf
    }

    fn right(&mut self, cursor_pos: usize) {
        if self.r >= self.buf.len() - 1 {
            return;
        }
        while cursor_pos > self.l {
            self.buf[self.l] = self.buf[self.r + 1];
            self.buf[self.r + 1] = '\0';
            self.l += 1;
            self.r += 1;

            if self.buf[self.l-1] == '\n' {
                self.row += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
    }

    fn left(&mut self, cursor_pos: usize) {
        if self.l <= 0 {
            return;
        }
        while cursor_pos < self.l {
            self.buf[self.r] = self.buf[self.l - 1];
            self.buf[self.l - 1] = '\0';
            self.l -= 1;
            self.r -= 1;

            if self.buf[self.r+1] == '\n' {
                self.row -= 1;
                self.col = self.find_col();
            } else {
                self.col -= 1;
            }
        }
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_col(&self) -> usize {
        self.col
    }

    // TODO: FIX
    fn find_col(&self) -> usize {
        assert!(self.l <= self.buf.len(), "self.l={} buf.len()={}", self.l, self.buf.len());
        let mut i = self.l;
        let mut col: usize = 0;
        while i > 0 && self.buf[i-1] != '\n' {
            i -= 1;
            col += 1;
        }

        col
    }

    fn on_first_line(&mut self) -> bool {
        let mut i = self.l;
        while i != 0 {
            i -= 1;
            if self.buf[i] == '\n' {
                return false;
            }
        } 
        return true;
    }

    fn on_last_line(&mut self) -> bool {
        let mut i = self.r;
        while i != self.buf.len()-1 {
            if self.buf[i] == '\n' {
                return false;
            }
            i += 1;
        }
        return true;
    }

    fn resize(&mut self) {
        let more_gap = vec!['\0'; GAP_SIZE];
        self.buf.splice(self.r..self.r, more_gap);
        self.r += GAP_SIZE;
        self.gap_size += GAP_SIZE;
    }

    // Inclusive on both ends
    fn delete(&mut self, start: usize, end: usize) {
        self.buf.splice(start..=end, vec!['\0'; end-start+1]);
        if start < self.l {
            self.l = start;
        }
        if end > self.r {
            self.r = end;
        }
    }

    pub fn enter(&mut self) {
        self.insert("\n");
    }

    pub fn n_o(&mut self) {
        self.end_of_line();
        self.insert("\n");
    }

    pub fn n_O(&mut self) {
        if self.on_first_line() {
            self.start_of_line();
            self.insert("\n");
            self.up();
        } else {
            self.up();
            self.end_of_line();
            self.insert("\n");
        }
    }

    pub fn n_w(&mut self) {
        
    }

    pub fn n_b(&mut self) {

    }

    pub fn n_W(&mut self) {
        while self.r != self.buf.len()-2 && self.buf[self.r+1] != ' ' {
            self.move_relative(1);
        }
        while self.r != self.buf.len()-2 && self.buf[self.r+1] == ' ' {
            self.move_relative(1);
        }
    }

    pub fn n_B(&mut self) {
        while self.l != 0 && self.buf[self.l-1] != ' ' {
            self.move_relative(-1);
        }
        while self.l != 0 && self.buf[self.l-1] == ' ' {
            self.move_relative(-1);
        }
        while self.l != 0 && self.buf[self.l-1] != ' ' {
            self.move_relative(-1);
        }
    }

    pub fn n_h(&mut self) {
        self.move_relative(-1);
    }

    pub fn n_l(&mut self) {
        self.move_relative(1);
    }

    pub fn n_j(&mut self) {
        self.down();
    }

    pub fn n_k(&mut self) {
        self.up();
    }

    pub fn n_a(&mut self) {
        self.move_relative(1);
    }

    pub fn n_dolla(&mut self) {
        self.end_of_line();
    }

    pub fn n_0(&mut self) {
        self.start_of_line();
    }

    fn get_char_class(c: char) -> CharClass {
        if c.is_alphanumeric() || c == '_' {
            CharClass::Alpha
        } else if c.is_whitespace() {
            CharClass::Whitespace
        } else {
            CharClass::Symbol
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_updates_text() {
        let mut gb = GapBuffer::from_text("");
        gb.insert("hello");
        assert_eq!(gb.get_text().trim_matches('\0'), "hello");
    }

    #[test]
    fn cursor_position_after_newline() {
        let mut gb = GapBuffer::from_text("");
        gb.insert("foo\n");
        gb.insert("bar");
        assert_eq!(gb.get_row(), 1);
        assert_eq!(gb.get_col(), 3);
    }
}

