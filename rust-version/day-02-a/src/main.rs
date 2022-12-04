use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum Oponent {
    A = 1,
    B = 2,
    C = 3,
}

impl FromStr for Oponent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Oponent::A),
            "B" => Ok(Oponent::B),
            "C" => Ok(Oponent::C),
            _ => Err(()),
        }
    }
}

enum Your {
    X = 1,
    Y = 2,
    Z = 3,
}

impl FromStr for Your {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Your::X),
            "Y" => Ok(Your::Y),
            "Z" => Ok(Your::Z),
            _ => Err(()),
        }
    }
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

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut points: u32 = 0;
        for line in lines {
            if let Ok(value) = line {
                let mut split = value.split(" ");
                let x_val = Oponent::from_str(split.next().unwrap()).unwrap();
                let y_val = Your::from_str(split.next().unwrap()).unwrap();
                points += evaluate(x_val as i8, y_val as i8);
                points += y_val as u32;
            }
        }
        println!("{}", points);
    }
}
