use common;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let lines = common::read_lines_vec("./day7/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result: {}", second_part(lines.clone()));
}

fn first_part(lines: Vec<String>) -> isize {
    let fs = build_fs(&lines);
    let mut total = 0;
    for d in fs.values() {
        if *d < 100000 {
            total += d;
        }
    }

    total
}

fn second_part(lines: Vec<String>) -> isize {
    let fs = build_fs(&lines);
    let space = 70000000 - fs.get("/").unwrap();

    let mut sizes: Vec<&isize> = fs.values().collect();
    sizes.sort();

    for s in sizes {
        if *s + space >= 30000000 {
            return *s;
        }
    }

    unreachable!();
}

fn build_fs(lines: &Vec<String>) -> HashMap<String, isize> {
    // Build filesytem.
    let mut fs: HashMap<String, isize> = HashMap::new();
    let mut iter = lines.iter().peekable();

    // Manually do the cd /
    assert_eq!(iter.next().unwrap(), "$ cd /");
    let mut curr_dir = String::from("/");
    fs.insert(curr_dir.clone(), 0);

    while iter.peek().is_some() {
        // Assume we'll get inside this loop only when command is next.
        let command = iter.next().unwrap();

        // "ls" command?
        if command == "$ ls" {
            // Read all contents until next command or EOF.
            loop {
                if let Some(n) = iter.next_if(|&l| !l.starts_with("$")) {
                    match *n.as_bytes().get(0).unwrap() as char {
                        'a'..='z' => {}
                        '0'..='9' => {
                            // read file size.
                            let file_size = n
                                .split_whitespace()
                                .next()
                                .unwrap()
                                .parse::<isize>()
                                .unwrap();

                            // Add size to current.
                            if let Some(size) = fs.get_mut(curr_dir.as_str()) {
                                *size += file_size;
                            }

                            // Add size to parent directories.
                            let path = Path::new(curr_dir.as_str());
                            if let Some(parent) = path.parent() {
                                for p in parent.ancestors() {
                                    if let Some(size) =
                                        fs.get_mut(&String::from(p.to_str().unwrap()))
                                    {
                                        *size += file_size;
                                    }
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    break;
                }
            }
        } else {
            // Accessing new directory. Creates a new directory.
            match command.strip_prefix("$ cd ") {
                Some("..") => {
                    curr_dir = String::from(
                        Path::new(curr_dir.as_str())
                            .parent()
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    )
                }
                Some(dir) => {
                    curr_dir =
                        String::from(Path::new(curr_dir.as_str()).join(dir).to_str().unwrap());
                    fs.entry(curr_dir.clone()).or_insert(0);
                }
                _ => unreachable!(),
            }
        }
    }

    fs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            95437
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            24933642
        );
    }
}
