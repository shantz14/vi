use std::io;

mod buffer;

fn main() {
    run_test();
}

fn run_test() {
    let mut gb = buffer::GapBuffer::from_file("./src/test.txt");
    gb.print();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        let char_input = input.chars().next().expect("Failed to get char inputted");
        if char_input == 'l' {
            gb.move_relative(1);
            gb.print_debug();
        } else if char_input == 'h' {
            gb.move_relative(-1);
            gb.print_debug();
        } else if char_input == 'p' {
            gb.insert("All my fellas");
        } else {
            gb.insert(&char_input.to_string());
        }
        gb.print();
    }
}
