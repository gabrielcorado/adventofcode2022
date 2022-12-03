use common;
use std::collections::HashSet;
use std::convert::TryInto;

fn main() {
    let lines = common::read_lines_vec("./day3/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result: {}", second_part(lines.clone()));
}

fn first_part(lines: Vec<String>) -> isize {
    let mut total = 0;
    for line in lines.iter() {
        let compartment_size = line.len() / 2;
        let first_compartment: HashSet<_> = line.get(..compartment_size).unwrap().chars().collect();
        let second_compartment: HashSet<_> =
            line.get(compartment_size..).unwrap().chars().collect();

        for c in first_compartment.intersection(&second_compartment) {
            total += if c.is_uppercase() {
                27 + *c as u32 - 'A' as u32
            } else {
                1 + *c as u32 - 'a' as u32
            }
        }
    }

    total.try_into().unwrap()
}

fn second_part(lines: Vec<String>) -> isize {
    let mut total = 0;
    for group in lines.chunks(3) {
        // Create a hash set for each elf.
        let mut sets = group.iter().map(|elf| elf.chars().collect::<HashSet<_>>());
        // Use first elf as reference for the intersection.
        let mut inter = sets.next().unwrap();
        // Iter over remaing group to do the intersection.
        for s in sets {
            inter = inter.intersection(&s).copied().collect();
        }

        for c in inter {
            total += if c.is_uppercase() {
                27 + c as u32 - 'A' as u32
            } else {
                1 + c as u32 - 'a' as u32
            }
        }
    }

    total.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            157
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            70
        );
    }
}
