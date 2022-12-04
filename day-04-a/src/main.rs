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
        let mut pairs: Vec<[[u32; 2]; 2]> = Vec::new();
        let mut total: u32 = 0;
        for line in lines {
            if let Ok(value) = line {
                let pair_str: Vec<&str> = value.split(",").collect();
                let first_block: Vec<&str> = pair_str[0].split("-").collect();
                let second_block: Vec<&str> = pair_str[1].split("-").collect();
                let first_pair: [u32; 2] = [
                    first_block[0].parse::<u32>().unwrap(),
                    first_block[1].parse::<u32>().unwrap(),
                ];
                let second_pair: [u32; 2] = [
                    second_block[0].parse::<u32>().unwrap(),
                    second_block[1].parse::<u32>().unwrap(),
                ];
                let pair: [[u32; 2]; 2] = [first_pair, second_pair];
                pairs.push(pair);
            }
        }
        // Check for inclusion in pairs
        for pair in pairs {
            let first_range = pair[0];
            let second_range = pair[1];
            if first_range[0] >= second_range[0] && first_range[1] <= second_range[1] {
                total += 1;
                continue;
            }
            if second_range[0] >= first_range[0] && second_range[1] <= first_range[1] {
                total += 1;
            }
        }

        println!("{}", total);
    }
}
