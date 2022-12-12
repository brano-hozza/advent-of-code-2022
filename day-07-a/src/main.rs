use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{self, BufRead};
use std::mem;
use std::path::Path;
use std::ptr;
/**
 * I am not going to do this because of skill issues
 */

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// enum CommandType {
//     List,
//     Directory,
// }
// struct L_File {
//     name: String,
//     size: u32,
// }

// struct Rawlink<T> {
//     p: *mut T,
// }

// impl<T> Copy for Rawlink<T> {}

// impl<T> Clone for Rawlink<T> {
//     fn clone(&self) -> Self {
//         Rawlink { p: self.p }
//     }
// }

// impl<T> Rawlink<T> {
//     /// Like Option::None for Rawlink
//     fn none() -> Rawlink<T> {
//         Rawlink { p: ptr::null_mut() }
//     }

//     /// Like Option::Some for Rawlink
//     fn some(n: &mut T) -> Rawlink<T> {
//         Rawlink { p: n as *mut T }
//     }

//     /// Convert the `Rawlink` into an Option value
//     fn resolve_immut<'a>(&self) -> Option<&'a T> {
//         unsafe { self.p.as_ref() }
//     }

//     /// Convert the `Rawlink` into an Option value
//     fn resolve<'a>(&mut self) -> Option<&'a mut T> {
//         unsafe { self.p.as_mut() }
//     }

//     /// Return the `Rawlink` and replace with `Rawlink::none()`
//     fn take(&mut self) -> Rawlink<T> {
//         mem::replace(self, Rawlink::none())
//     }
// }

// pub struct Node<T> {
//     next: Option<Box<Node<T>>>,
//     prev: Rawlink<Node<T>>,
//     value: T,
// }

// struct L_Directory {
//     name: String,
//     files: Vec<L_File>,
//     directories: Vec<L_Directory>,
//     parent: Rawlink<L_Directory>,
// }

// impl<'a> L_Directory {
//     pub fn new(name: String) -> L_Directory {
//         L_Directory {
//             name,
//             files: Vec::new(),
//             directories: Vec::new(),
//             parent: Rawlink::none(),
//         }
//     }

//     pub fn add_file(&mut self, file: L_File) {
//         self.files.push(file);
//     }

//     pub fn create_dir(&mut self, name: String) -> &mut L_Directory {
//         let mut dir = L_Directory::new(name);
//         dir.parent = Some(Box::new(self));
//         self.directories.push(dir);
//         self.directories.last_mut().unwrap()
//     }
// }

fn main() {
    //
    // TODO: Complete this????
    //
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // let mut command_type: CommandType = CommandType::List;
        // let mut root_directory = L_Directory::new(String::from("root"));
        // let mut current_directory = root_directory.borrow_mut();
        for line in lines {
            if let Ok(value) = line {
                // let parts: Vec<String> = value.split_whitespace().map(|f| f.to_string()).collect();
                // if parts[0].starts_with("$") {
                //     if parts[1] == "ls" {
                //         command_type = CommandType::List;
                //     } else if parts[1] == "cd" {
                //         command_type = CommandType::Directory;
                //         if parts[2] == "/" {
                //             current_directory = root_directory.borrow_mut();
                //             continue;
                //         } else if parts[2] == ".." {
                //             if current_directory.parent.is_some() {
                //                 current_directory = current_directory.parent.unwrap().as_mut();
                //                 continue;
                //             }
                //         } else {
                //             if current_directory
                //                 .directories
                //                 .iter()
                //                 .any(|dir| dir.name == parts[2])
                //             {
                //                 current_directory = current_directory
                //                     .directories
                //                     .iter()
                //                     .find(|dir| dir.name == parts[2])
                //                     .unwrap()
                //                     .borrow_mut();
                //                 continue;
                //             } else {
                //                 current_directory.create_dir(parts[2].clone());
                //             }
                //         }
                //     }
                // }
            }
        }
    }
}
