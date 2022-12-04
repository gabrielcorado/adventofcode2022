use common;

fn main() {
    let lines = common::read_lines_vec("./day4/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result: {}", second_part(lines.clone()));
}

fn first_part(lines: Vec<String>) -> isize {
    let mut total = 0;
    for line in lines {
        let numbers: Vec<isize> = line
            .split([',', '-'])
            .map(|n| n.parse::<isize>().unwrap())
            .collect();
        assert_eq!(numbers.len(), 4);

        let (a, b, c, d) = (numbers[0], numbers[1], numbers[2], numbers[3]);
        if a >= c && b <= d {
            total += 1;
        } else if c >= a && d <= b {
            total += 1;
        }
    }

    total
}

fn second_part(lines: Vec<String>) -> isize {
    let mut total = 0;
    for line in lines {
        let numbers: Vec<isize> = line
            .split([',', '-'])
            .map(|n| n.parse::<isize>().unwrap())
            .collect();
        assert_eq!(numbers.len(), 4);

        let (a, b, c, d) = (numbers[0], numbers[1], numbers[2], numbers[3]);
        if a <= d && c <= b {
            total += 1;
        }
    }

    total
    // A-B
    // C-D
    //
    // A >= C && B <= D
    //
    //     |----|
    //   |----|
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            2
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            4
        );
    }
}
