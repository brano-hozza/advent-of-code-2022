use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut total = 0;
        let mut elves: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(value) = line {
                if value == "" {
                    elves.push(total);
                    total = 0;
                    continue;
                }
                if let Ok(num) = value.parse::<i32>() {
                    total += num;
                }
            }
        }
        elves.sort();
        let last_3_sum = elves.as_slice()[elves.len() - 3..]
            .to_vec()
            .iter()
            .sum::<i32>();

        println!("Max: {}", last_3_sum);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
