use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::path::Path;

// Define a trait for objects in the hierarchy
trait HierarchyObject {
    fn is_folder(&self) -> bool;
}

struct Folder {
    name: String,
    children: HashMap<String, Box<dyn HierarchyObject>>,
}

// Implement the HierarchyObject trait for the Folder struct
impl HierarchyObject for Folder {
    fn is_folder(&self) -> bool {
        true
    }
}

// Define a struct for files in the hierarchy
struct File {
    name: String,
    size: u32,
}

// Implement the HierarchyObject trait for the File struct
impl HierarchyObject for File {
    fn is_folder(&self) -> bool {
        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // let root = Folder {
    //     name: "/".to_string(),
    //     children: Vec::new(),
    // };

    // Parse the hierarchy and store the folders in a Vec<Box>
    // let mut system = Vec::new();

    let root_folder = Box::new(Folder {
        name: "/".to_string(),
        children: HashMap::new(),
    });
    let mut current_path: Vec<Box<Folder>> = vec![root_folder];
    let mut file_system: Vec<Box<dyn HierarchyObject>> = vec![root_folder];

    for shell_log in input.split("$") {
        if shell_log == "" {
            continue;
        };
        let mut lines = shell_log.lines().peekable();
        let cmd = lines.next().unwrap();

        let current_folder = &current_path[current_path.len() - 1];

        match lines.peek() {
            Some(_) => {
                // println!("ls {:?}", current_path);
                // current_path[current_path.len() - 1].children = Vec::new();
                for entry in lines {
                    let [size_or_dir, name]: [&str; 2] = entry
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .try_into()
                        .ok() // I trust my inputs <3
                        .unwrap();

                    println!("file {:?} {:?}", size_or_dir, name);
                }
            }
            None => {
                let dest_dir = cmd.split_whitespace().rev().next().unwrap();
                println!("cd {:?}", dest_dir);

                match dest_dir {
                    ".." => {
                        current_path.pop();
                    }
                    f if f != "/" => {
                        // let target = *current_folder.children.iter().find(|c| {
                        //     match c {
                        //         Some(f) if f.is_folder() => true
                        //         _ => false
                        //     }
                        // });
                        // current_path.push(*current_folder.ch);
                    }
                };
            }
        }
    }
    // parse_hierarchy(Path::new(&root.name), &root, &mut folders);

    // // Print the names of the folders in the hierarchy
    // for folder in folders {
    //     println!("{}", folder.name);
    // }
    None
}

// fn parse_hierarchy(path: &Path, folder: &Folder, folders: &mut Vec<Box<Folder>>) {
//     // Add the current folder to the Vec<Box>
//     folders.push(Box::new(folder.clone()));

//     // Iterate over the children of the current folder
//     for entry in fs::read_dir(path).expect("Failed to read directory") {
//         let entry = entry.expect("Failed to read entry");
//         let path = entry.path();

//         // Check if the entry is a folder or a file
//         if path.is_dir() {
//             // If it is a folder, create a new Folder object and recursively parse its children
//             let name = path.file_name().unwrap().to_str().unwrap();
//             let mut children = Vec::new();
//             let subfolder = Folder {
//                 name: name.to_string(),
//                 children: children,
//             };
//             parse_hierarchy(&path, &subfolder, folders);
//         } else {
//             // If it is a file, create a new File object and add it to the current folder's children
//             let name = path.file_name().unwrap().to_str().unwrap();
//             let file = File {
//                 name: name.to_string(),
//             };
//             folder.children.push(Box::new(file));
//         }
//     }
// }

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc2022::read_file("inputs", 7);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
