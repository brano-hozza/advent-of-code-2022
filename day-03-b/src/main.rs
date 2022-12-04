use std::collections::{HashMap, HashSet};
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
        let groups_of_3_lines: Vec<Vec<String>> = lines
            .collect::<Result<Vec<String>, _>>()
            .unwrap()
            .chunks(3)
            .map(|chunk| chunk.to_vec())
            .collect();
        for group in groups_of_3_lines {
            let mut values: HashMap<char, u32> = HashMap::new();
            for line in group {
                // get uniques from first half
                let unique_values: Vec<char> = line
                    .chars()
                    .collect::<HashSet<char>>()
                    .into_iter()
                    .collect();

                for char in unique_values {
                    if values.contains_key(&char) {
                        let count = values.get(&char).unwrap();

                        if count + 1 == 3 {
                            let ascii_value = char as u32;
                            if ascii_value > 96 {
                                total += ascii_value - 96;
                            } else {
                                total += ascii_value - 38;
                            }
                            break;
                        }
                        values.insert(char, count + 1);
                    } else {
                        values.insert(char, 1);
                    }
                }
            }
        }

        println!("{}", total);
    }
}
