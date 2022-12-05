use std::collections::HashMap;
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
type Stack = Vec<char>;

type StackHolder = HashMap<u32, Stack>;
trait StackHolderExt {
    fn move_box(&mut self, amount: u32, x: u32, y: u32);
}
impl StackHolderExt for StackHolder {
    fn move_box(&mut self, amount: u32, x: u32, y: u32) {
        let mut temp = Stack::new();
        let from = self.get_mut(&x).unwrap();
        for _ in 0..amount {
            temp.push(from.pop().unwrap());
        }
        temp.reverse();
        let to = self.get_mut(&y).unwrap();
        for i in temp {
            to.push(i);
        }
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut stacks = StackHolder::new();
        let mut is_setup = true;
        let mut total_keys = 0;
        for line in lines {
            if let Ok(value) = line {
                // format:
                //     [B] [C] ...
                // [A] [D] [E] ...
                if is_setup && value.chars().nth(1).unwrap() != '1' {
                    let length = value.len();
                    let mut stack_index = 0;
                    for i in 0..length {
                        if i % 4 == 1 {
                            stack_index += 1;
                            let char = value.chars().nth(i).unwrap();
                            if char == ' ' {
                                continue;
                            }
                            if stacks.contains_key(&stack_index) {
                                let stack = stacks.get_mut(&stack_index).unwrap();
                                stack.insert(0, char);
                            } else {
                                let mut vec = Stack::new();
                                vec.push(char);
                                stacks.insert(stack_index, vec);
                            }
                        }
                    }
                    if total_keys < stack_index {
                        total_keys = stack_index;
                    }
                    continue;
                }
                if is_setup || value.len() == 0 {
                    is_setup = false;
                    continue;
                }
                let commands: Vec<&str> = value.split(" ").collect();
                // convert second value to u32
                let amount: u32 = commands[1].parse().unwrap();
                let x: u32 = commands[3].parse().unwrap();
                let y: u32 = commands[5].parse().unwrap();
                stacks.move_box(amount, x, y)
            }
        }
        let mut result = String::from("");
        for key in 1..(total_keys + 1) {
            let stack = stacks.get(&key).unwrap();
            result.push(*stack.get(stack.len() - 1).unwrap());
        }
        println!("{}", result);
    }
}
