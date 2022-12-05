// TODO: not satisfied with this solution, it seems over complicated.
use std::collections::VecDeque;

fn main() {
    let lines = common::read_lines_vec("./day5/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result: {}", second_part(lines.clone()));
}

fn first_part(lines: Vec<String>) -> String {
    // Initialize the stacks.
    let mut lines_iter = lines.iter();
    let mut stacks = init_stacks(&mut lines_iter);

    // Advance one break line.
    lines_iter.next();

    // Iterate over the remaining lines, which are the instructions.
    for line in lines_iter {
        // Take only numbers instructions.
        let numbers: Vec<usize> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        for _ in 0..numbers[0] {
            let elem = stacks.get_mut(numbers[1] - 1).unwrap().pop_front().unwrap();
            stacks.get_mut(numbers[2] - 1).unwrap().push_front(elem);
        }
    }

    // Build final string.
    let mut res: Vec<u8> = Vec::new();
    for stack in stacks {
        res.push(*stack.front().unwrap())
    }

    String::from_utf8(res).unwrap()
}

fn second_part(lines: Vec<String>) -> String {
    // Initialize the stacks.
    let mut lines_iter = lines.iter();
    let mut stacks = init_stacks(&mut lines_iter);

    // Advance one break line.
    lines_iter.next();

    // Iterate over the remaining lines, which are the instructions.
    for line in lines_iter {
        // Take only numbers instructions.
        let numbers: Vec<usize> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // pop front elements.
        let mut elems: Vec<u8> = Vec::new();
        for _ in 0..numbers[0] {
            elems.push(stacks.get_mut(numbers[1] - 1).unwrap().pop_front().unwrap());
        }

        // insert in front in reverse order.
        for elem in elems.iter().rev() {
            stacks.get_mut(numbers[2] - 1).unwrap().push_front(*elem);
        }
    }

    // Build final string.
    let mut res: Vec<u8> = Vec::new();
    for stack in stacks {
        res.push(*stack.front().unwrap())
    }

    String::from_utf8(res).unwrap()
}

fn init_stacks(lines_iter: &mut dyn std::iter::Iterator<Item = &String>) -> Vec<VecDeque<u8>> {
    let mut stacks = Vec::<VecDeque<u8>>::new();

    loop {
        if let Some(line) = lines_iter.next() {
            // Check if it is the stack definition ending.
            if line.starts_with(" 1") {
                break;
            }

            // Parse the line every 4 chars: "[X] ".
            let mut stack_num = 0;
            for c in line.as_bytes().chunks(4) {
                // Init the stack if it wasn't yet.
                if stacks.get(stack_num).is_none() {
                    stacks.insert(stack_num, VecDeque::<u8>::new());
                }

                // Only parse crates.
                if *c.first().unwrap() == '[' as u8 {
                    stacks
                        .get_mut(stack_num)
                        .unwrap()
                        .push_back(c.get(1).unwrap().clone());
                }

                stack_num += 1;
            }
        }
    }

    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            "CMZ",
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            "MCD",
        );
    }
}
