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

fn solve_x(key: &str) -> Result<u32, u32> {
    match key {
        "A" => Ok(3),
        "B" => Ok(1),
        "C" => Ok(2),
        "value" => Ok(0),
        _ => return Err(0),
    }
}

fn solve_y(key: &str) -> Result<u32, u32> {
    match key {
        "A" => Ok(1),
        "B" => Ok(2),
        "C" => Ok(3),
        "value" => Ok(3),
        _ => return Err(0),
    }
}

fn solve_z(key: &str) -> Result<u32, u32> {
    match key {
        "A" => Ok(2),
        "B" => Ok(3),
        "C" => Ok(1),
        "value" => Ok(6),
        _ => return Err(0),
    }
}

fn evaluate(key: &str) -> Result<fn(&str) -> Result<u32, u32>, u32> {
    match key {
        "X" => return Ok(solve_x),
        "Y" => return Ok(solve_y),
        "Z" => return Ok(solve_z),
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
                let x_key = split.next().unwrap();
                let result = evaluate(split.next().unwrap()).unwrap();
                points += result(x_key).unwrap() + result("value").unwrap();
            }
        }
        println!("{}", points);
    }
}
