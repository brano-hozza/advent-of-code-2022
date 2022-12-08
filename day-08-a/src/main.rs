use std::borrow::BorrowMut;
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
struct Tree {
    height: i8,
    visible: bool,
}

impl Tree {
    fn set_visible(&mut self) {
        self.visible = true;
    }
}
fn main() {
    // File hosts must exist in current path before this produces output
    let mut row: Vec<Vec<Tree>> = Vec::new();
    let mut first_row = true;
    let mut visible_count = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(value) = line {
                let mut top_height_in_row: i8 = -1;
                for (index, c) in value.chars().enumerate() {
                    let new_vec = Vec::new();
                    if first_row {
                        row.push(new_vec);
                    }

                    let height = c.to_digit(10).unwrap() as i8;
                    let mut visible = false;
                    if top_height_in_row < height {
                        top_height_in_row = height;
                        visible = true;
                    }
                    let tree = Tree { height, visible };
                    if tree.visible {
                        visible_count += 1;
                        print!("{} ", tree.height);
                    }
                    row.get_mut(index).unwrap().push(tree);
                }
            }
            first_row = false;
        }
    }
    println!();
    // first let read rows backwards
    let row_count = row.get(0).unwrap().len();
    for r_index in 0..row_count {
        let mut max_height = -1;
        for column in row.iter_mut().rev() {
            let tree_ref = column.get_mut(r_index).unwrap();
            let tree = tree_ref.borrow_mut();

            let mut visible = false;
            if max_height < tree.height {
                visible = true;
                max_height = tree.height;
            }
            if !tree.visible && visible {
                tree.set_visible();
                visible_count += 1;
                print!("{} ", tree.height);
            }
        }
    }
    println!();

    // now let read columns up -> down
    for column in row.iter_mut() {
        let mut max_height = -1;
        for tree in column.iter_mut() {
            let mut visible = false;
            if max_height < tree.height {
                visible = true;
                max_height = tree.height;
            }
            if !tree.visible && visible {
                tree.set_visible();
                visible_count += 1;
                print!("{} ", tree.height);
            }
        }
    }
    println!();

    // now let read columns down -> up
    for column in row.iter_mut() {
        let mut max_height = -1;
        for tree in column.iter_mut().rev() {
            let mut visible = false;
            if max_height < tree.height {
                visible = true;
                max_height = tree.height;
            }
            if !tree.visible && visible {
                tree.set_visible();
                visible_count += 1;
                print!("{} ", tree.height);
            }
        }
    }

    println!("Visible trees: {}", visible_count);
}
