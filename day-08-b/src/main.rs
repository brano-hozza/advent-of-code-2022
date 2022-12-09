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
    let mut row: Vec<Vec<i8>> = Vec::new();
    let mut first_row = true;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(value) = line {
                for (index, c) in value.chars().enumerate() {
                    let new_vec = Vec::new();
                    if first_row {
                        row.push(new_vec);
                    }

                    let height = c.to_digit(10).unwrap() as i8;
                    row.get_mut(index).unwrap().push(height);
                }
            }
            first_row = false;
        }
    }
    // for each tree check how many other trees it can see
    let mut max_scenic_score = -1;
    for (x, current_column) in row.iter().enumerate() {
        for (y, current_tree_height) in current_column.iter().enumerate() {
            // first check from left to right
            if x == 2 && y == 1 {
                println!("{} {}", x, y);
            }
            let mut lr_score = 0;
            if x <= row.len() {
                for _x in x + 1..row.len() {
                    let _tree_height = row.get(_x).unwrap().get(y).unwrap();
                    lr_score += 1;
                    if _tree_height >= current_tree_height {
                        break;
                    }
                }
                if lr_score == 0 {
                    continue;
                }
            }

            // now check from right to left
            let mut rl_score = 0;
            for _x in (0..x).rev() {
                let _tree_height = row.get(_x).unwrap().get(y).unwrap();
                rl_score += 1;
                if _tree_height >= current_tree_height {
                    break;
                }
            }
            if rl_score == 0 {
                continue;
            }

            // now check from top to bottom
            let mut tb_score = 0;
            if y <= current_column.len() {
                for _y in y + 1..current_column.len() {
                    let _tree_height = row.get(x).unwrap().get(_y).unwrap();
                    tb_score += 1;
                    if _tree_height >= current_tree_height {
                        break;
                    }
                }
                if tb_score == 0 {
                    continue;
                }
            }

            // now check from bottom to top
            let mut bt_score = 0;
            for _y in (0..y).rev() {
                let _tree_height = row.get(x).unwrap().get(_y).unwrap();
                bt_score += 1;
                if _tree_height >= current_tree_height {
                    break;
                }
            }

            if bt_score == 0 {
                continue;
            }
            let current_scenic_score = lr_score * rl_score * tb_score * bt_score;

            if current_scenic_score > max_scenic_score {
                max_scenic_score = current_scenic_score;
            }
        }
    }
    println!("Max Scenic Score: {}", max_scenic_score);
}
