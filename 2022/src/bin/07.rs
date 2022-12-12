// Stolen from amos
//   https://fasterthanli.me/series/advent-of-code-2022/part-7#using-a-stack
#[derive(Debug)]
struct FsEntry {
    path: String,
    size: u64,
    children: Vec<FsEntry>,
}

impl FsEntry {
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }

    // Q: why does this return a Box? Learn more about `dyn`. And what's with the `+ '_`?
    fn all_dirs(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        Box::new(
            // Q: std::iter::once ? Read about chain
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            ),
        )
    }
}

fn parse_filesystem(input: &str) -> FsEntry {
    let mut stack = vec![FsEntry {
        path: "/".into(),
        size: 0,
        children: vec![],
    }];

    for shell_log in input.split("$") {
        if shell_log == "" {
            continue;
        };
        let mut lines = shell_log.lines().peekable();
        let cmd = lines.next().unwrap();

        match lines.peek() {
            Some(_) => {
                // ls
                for entry in lines {
                    let [size_or_dir, name]: [&str; 2] = entry
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .try_into()
                        .ok() // I trust my inputs <3
                        .unwrap();

                    match size_or_dir {
                        "dir" => (), // Skip: use `cd` to populate dirs
                        _ => {
                            let size = size_or_dir.parse::<u64>().unwrap();
                            let node = FsEntry {
                                size,
                                path: name.to_string(),
                                children: vec![],
                            };
                            stack.last_mut().unwrap().children.push(node);
                        }
                    }
                }
            }
            None => {
                // cd
                let dest_dir = cmd.split_whitespace().rev().next().unwrap();
                match dest_dir {
                    ".." => {
                        let child = stack.pop();
                        stack.last_mut().unwrap().children.push(child.unwrap());
                    }
                    f if f != "/" => {
                        let node = FsEntry {
                            path: f.to_string(),
                            size: 0,
                            children: vec![],
                        };
                        stack.push(node);
                    }
                    _ => (),
                };
            }
        }
    }

    let mut root = stack.pop().unwrap();
    while let Some(mut next) = stack.pop() {
        // Q: Why do we still need to mutate children here while popping off?
        next.children.push(root);
        root = next;
    }

    root
}

pub fn part_one(input: &str) -> Option<u64> {
    let root = parse_filesystem(input);

    Some(
        root.all_dirs()
            .map(|d| d.total_size())
            .filter(|s| s < &100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let root = parse_filesystem(input);

    let total_size = root.total_size();
    let min_deletion_size = total_size - (70000000 - 30000000);

    let mut sizes = root.all_dirs().map(|d| d.total_size()).collect::<Vec<_>>();
    sizes.sort();

    Some(*sizes.iter().find(|s| *s > &min_deletion_size).unwrap())
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
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
