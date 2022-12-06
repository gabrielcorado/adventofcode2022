// TODO: very simple but not efficient solution.
use std::collections::HashSet;

fn main() {
    let lines = common::read_lines_vec("./day6/src/input.txt");
    println!(
        "First part result: {}",
        first_part(lines.get(0).unwrap().clone())
    );
    println!(
        "Second part result: {}",
        second_part(lines.get(0).unwrap().clone())
    );
}

fn first_part(line: String) -> usize {
    let chars: Vec<char> = line.chars().map(|c| c.clone()).collect();
    for (i, c) in chars.windows(4).enumerate() {
        if c.iter().collect::<HashSet<&char>>().len() == 4 {
            return i + 4;
        }
    }

    unreachable!()
}

fn second_part(line: String) -> usize {
    let chars: Vec<char> = line.chars().map(|c| c.clone()).collect();
    for (i, c) in chars.windows(14).enumerate() {
        if c.iter().collect::<HashSet<&char>>().len() == 14 {
            return i + 14;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        static EXAMPLES: &'static [(&str, usize)] = &[
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for e in EXAMPLES {
            assert_eq!(first_part(String::from(e.0)), e.1);
        }
    }

    #[test]
    fn test_second_part() {
        static EXAMPLES: &'static [(&str, usize)] = &[
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for e in EXAMPLES {
            assert_eq!(second_part(String::from(e.0)), e.1);
        }
    }
}
