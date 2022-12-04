use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut total: u32 = 0;
        for line in lines {
            if let Ok(value) = line {
                let length = value.len();
                let first_half = &value[0..(length / 2)];
                let second_half = &value[(length / 2)..length];
                let unique_first: Vec<char> = first_half.chars().collect();
                let mut char = unique_first[0];
                for c in second_half.chars() {
                    if unique_first.contains(&c) {
                        char = c;
                        break;
                    }
                }
                let ascii_value = char as u32;
                if ascii_value > 96 {
                    total += ascii_value - 96;
                } else {
                    total += ascii_value - 38;
                }
            }
        }
        println!("{}", total);
    }
}
