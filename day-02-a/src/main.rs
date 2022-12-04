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

fn evaluate(x: i8, y: i8) -> u32 {
    let result: i8 = x - y;
    if result == -1 || result == 2 {
        return 6;
    }
    if result == 0 {
        return 3;
    }
    return 0;
}

fn get_val(key: &str) -> Result<u32, u32> {
    match key {
        "A" | "X" => return Ok(1),
        "B" | "Y" => return Ok(2),
        "C" | "Z" => return Ok(3),
        _ => return Err(0),
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut points: u32 = 0;
        for line in lines {
            if let Ok(value) = line {
                let mut split = value.split(" ");
                let x_val = get_val(split.next().unwrap()).unwrap();
                let y_val = get_val(split.next().unwrap()).unwrap();
                points += evaluate(x_val as i8, y_val as i8);
                points += y_val as u32;
            }
        }
        println!("{}", points);
    }
}
