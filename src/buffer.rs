use std::fs;

const GAP_SIZE: usize = 256;

pub struct GapBuffer {
    l: usize,
    r: usize,
    buf: Vec<char>,
    gap_size: usize,
}

impl GapBuffer {
    pub fn from_file(file_name: &str) -> GapBuffer {
        let mut gap: Vec<char> = vec!['\0'; GAP_SIZE];
        let mut file_contents: Vec<char> = fs::read_to_string(file_name)
            .expect("Failed to read file.")
            .chars()
            .collect();
        gap.append(&mut file_contents);
        let buf = GapBuffer {
            l: 0,
            r: GAP_SIZE - 1,
            buf: gap,
            gap_size: GAP_SIZE,
        };
        return buf;
    }

    pub fn print(&self) {
        let mut cursor_found = false;
        for c in self.buf.iter() {
            if *c == '\0' && cursor_found == false {
                print!("[CURSOR]");
                cursor_found = true;
            }
            if *c != '\0' {
                print!("{}", c);
            }
        }
    }

    pub fn print_debug(&self) {
        let buf = self.get_buf();
        println!("{:?}", buf);
    }

    pub fn get_buf(&self) -> Vec<char> {
        let buf = self.buf.clone();
        return buf;
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
        }
    }

    pub fn move_relative(&mut self, distance: i32) {
        if distance.is_negative() {
            let usize_dist = (distance * -1) as usize;
            self.move_absolute(self.l - usize_dist);
        } else {
            let usize_dist = distance as usize;
            self.move_absolute(self.l + usize_dist);
        }
    }

    pub fn move_absolute(&mut self, cursor_pos: usize) {
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
    }

    fn resize(&mut self) {
        let more_gap = vec!['\0'; self.gap_size];
        self.buf.splice(self.r+1..self.r+1, more_gap);
        self.r += self.gap_size;
        self.gap_size = self.gap_size * 2;
    }

    // Inclusive on both ends
    pub fn delete(&mut self, start: usize, end: usize) {
        self.buf.splice(start..=end, vec!['\0'; end-start+1]);
        if start < self.l {
            self.l = start;
        }
        if end > self.r {
            self.r = end;
        }
    }

    pub fn backspace(&mut self) {
        self.delete(self.l-1, self.l-1);
    }

}

