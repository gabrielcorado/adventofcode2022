use common;
use std::collections::HashSet;

fn main() {
    let lines = common::read_lines_vec("./day9/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result: {}", second_part(lines.clone()));
}

fn first_part(lines: Vec<String>) -> isize {
    let mut head = (0isize, 0isize);
    let mut tail = (0isize, 0isize);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    for line in lines {
        let (m, dist_str) = line.split_once(' ').unwrap();
        let dist = dist_str.parse::<isize>().unwrap();

        let d: (isize, isize) = match m {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };

        for _ in 0..dist {
            // Move head.
            head.0 += d.0;
            head.1 += d.1;
            // Move tail following the HEAD.
            follow(&head, &mut tail);
            visited.insert(tail);
        }
    }

    //println!("head: {:?}, tail: {:?}", head, tail);
    visited.len() as isize
}

fn second_part(lines: Vec<String>) -> isize {
    let mut rope: Vec<(isize, isize)> = (0..10).map(|_| (0, 0)).collect();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    for line in lines {
        let (m, dist_str) = line.split_once(' ').unwrap();
        let dist = dist_str.parse::<isize>().unwrap();

        let d: (isize, isize) = match m {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };

        for _ in 0..dist {
            // Moves the actual rope head.
            rope[0].0 += d.0;
            rope[0].1 += d.1;

            // Move the knots as it was the tail (using follow function). To do
            // this we use the previous knot as it was the HEAD. DO NOT VISIT
            // THE TAIL ITSELF to avoid overflowing the rope.
            for i in 0..(rope.len() - 1) {
                let fake_head = rope[i];
                follow(&fake_head, &mut rope[i + 1]);
            }

            // REMEMBER: 9 is the size and the tail of our rope.
            visited.insert(rope[9]);
        }
    }

    visited.len() as isize
}

fn follow(head: &(isize, isize), tail: &mut (isize, isize)) {
    let xdiff = head.0 - tail.0;
    let ydiff = head.1 - tail.1;
    // if both are less than one, we don't need to perform a movement.
    if xdiff.abs() <= 1 && ydiff.abs() <= 1 {
        return;
    }
    // Move the tail towards the HEAD. If the diff is 0, signum returns 0 too so
    // no movement is made.
    tail.0 += 1 * xdiff.signum();
    tail.1 += 1 * ydiff.signum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            13
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            1
        );
    }
}
