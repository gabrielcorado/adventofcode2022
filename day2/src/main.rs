use common;

fn main() {
    // TODO: Couldn't make it work with the same "common::Lines" due to move errors.
    println!(
        "First part result is: {}",
        first_part(common::read_lines("./day2/src/input.txt")),
    );
    println!(
        "Second part result is: {}",
        second_part(common::read_lines("./day2/src/input.txt")),
    );
}

fn first_part(lines: common::Lines) -> isize {
    // Player1: A: Rock, B: Paper, C: Scissors.
    // Player2: X: Rock, Y: Paper, Z: Scissors.

    fn equal(a: &str, b: &str) -> bool {
        match b {
            "X" => a == "A",
            "Y" => a == "B",
            "Z" => a == "C",
            _ => unreachable!(),
        }
    }

    fn winner(a: &str, b: &str) -> bool {
        match b {
            "X" => a == "C",
            "Y" => a == "A",
            "Z" => a == "B",
            _ => unreachable!(),
        }
    }

    let mut total = 0;

    for line in lines {
        if let Ok(data) = line {
            let values: Vec<&str> = data.split(' ').collect();
            // let match values[0] {
            //     "A" => 0,
            // }
            let round = if equal(values[0], values[1]) {
                3
            } else if winner(values[0], values[1]) {
                6
            } else {
                0
            };
            let play_value = match values[1] {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => unreachable!(),
            };

            total += round + play_value;
        }
    }

    total
}

fn second_part(lines: common::Lines) -> isize {
    // Player: A: Rock, B: Paper, C: Scissors.
    fn score(a: &str) -> isize {
        match a {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => unreachable!(),
        }
    }

    fn winner(a: &str) -> &str {
        match a {
            "A" => "B",
            "B" => "C",
            "C" => "A",
            _ => unreachable!(),
        }
    }

    fn loser(a: &str) -> &str {
        match a {
            "A" => "C",
            "B" => "A",
            "C" => "B",
            _ => unreachable!(),
        }
    }

    let mut total = 0;
    for line in lines {
        if let Ok(data) = line {
            let values: Vec<&str> = data.split(' ').collect();
            match values[1] {
                "X" => total += score(loser(values[0])),
                "Y" => total += 3 + score(values[0]),
                "Z" => total += 6 + score(winner(values[0])),
                _ => unreachable!(),
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(common::read_lines("./src/test-input.txt")), 15)
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(common::read_lines("./src/test-input.txt")), 12)
    }
}
