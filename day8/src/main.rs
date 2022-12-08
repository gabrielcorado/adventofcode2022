use common;

fn main() {
    let lines = common::read_lines_vec("./day8/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result: {}", second_part(lines.clone()));
}

fn first_part(lines: Vec<String>) -> isize {
    // Map trees.
    let trees: Vec<Vec<isize>> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    // Initialize a map with visible trees.
    let mut visible: Vec<Vec<isize>> = (0..trees.len())
        .map(|_| (0..trees[0].len()).map(|_| 4).collect())
        .collect();

    // Iterate in lines.
    for x in 0..trees.len() {
        let mut highest = -1;

        // Left -> Right.
        for y in 0..trees[x].len() - 1 {
            if trees[x][y] <= highest {
                visible[x][y] -= 1;
            } else {
                highest = trees[x][y];
            }
        }

        // Reset highest.
        highest = -1;

        // Right -> Left.
        for y in (0..trees[x].len()).rev() {
            if trees[x][y] <= highest {
                visible[x][y] -= 1;
            } else {
                highest = trees[x][y];
            }
        }
    }

    // Iterate in columns.
    for y in 0..trees[0].len() {
        let mut highest = -1;

        // Top -> Bottom.
        for x in 0..trees.len() {
            if trees[x][y] <= highest {
                visible[x][y] -= 1;
            } else {
                highest = trees[x][y];
            }
        }

        // Reset highest.
        highest = -1;

        // Bottom -> Top.
        for x in (0..trees.len()).rev() {
            if trees[x][y] <= highest {
                visible[x][y] -= 1;
            } else {
                highest = trees[x][y];
            }
        }
    }

    visible.iter().flatten().filter(|v| **v > 0).count() as isize
}

fn second_part(lines: Vec<String>) -> isize {
    // Map trees.
    let trees: Vec<Vec<isize>> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    let mut max = -1;
    for x in 0..trees.len() {
        for y in 0..trees[x].len() {
            // Curr height.
            let curr = trees[x][y];

            // Looking up.
            let mut up = 0;
            for m in (0..x).rev() {
                up += 1;
                if trees[m][y] >= curr {
                    break;
                }
            }

            // Looking down.
            let mut down = 0;
            for m in x + 1..trees.len() {
                down += 1;
                if trees[m][y] >= curr {
                    break;
                }
            }

            // Looking left.
            let mut left = 0;
            for m in (0..y).rev() {
                left += 1;
                if trees[x][m] >= curr {
                    break;
                }
            }

            // Looking right.
            let mut right = 0;
            for m in y + 1..trees.len() {
                right += 1;
                if trees[x][m] >= curr {
                    break;
                }
            }

            let total = up * down * left * right;
            if total > max {
                max = total;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            21
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            8
        );
    }
}
