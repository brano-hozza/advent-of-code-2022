use std::collections::HashSet;
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
        for line in lines {
            if let Ok(value) = line {
                let chars = value.chars();
                let mut stack_14: Vec<char> = Vec::new();
                let mut index = 0;
                for char in chars {
                    index += 1;
                    stack_14.push(char);
                    if stack_14.len() == 14 {
                        let uniques: HashSet<char> = HashSet::from_iter(stack_14.iter().cloned());
                        if uniques.len() == 14 {
                            println!("{}", index);
                            return;
                        }
                        stack_14.remove(0);
                    }
                }
            }
        }
    }
}
