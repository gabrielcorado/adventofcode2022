use common;
use std::collections::HashSet;

fn main() {
    let lines = common::read_lines_vec("./day14/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result: {}", second_part(lines.clone()));
}

fn first_part(lines: Vec<String>) -> isize {
    let mut count = 0;
    let (mut map, deepest_rock, _) = make_map(lines);

    // Filling with sand!
    'outer: loop {
        // Sand position.
        let mut x = 500;
        let mut y = 0;

        // Moving particle.
        loop {
            // If y is larger than the deepest, then we're done.
            if y >= deepest_rock {
                break 'outer;
            }
            // Can we move below?
            match map.get(&(x, y + 1)) {
                None => y += 1,
                _ => {
                    // Move left?
                    if map.get(&(x - 1, y + 1)).is_none() {
                        x -= 1;
                        y += 1;
                    } else if map.get(&(x + 1, y + 1)).is_none() {
                        x += 1;
                        y += 1;
                    } else {
                        map.insert((x, y));
                        break;
                    }
                }
            }
        }

        count += 1;
    }

    count
}

fn second_part(lines: Vec<String>) -> isize {
    let mut count = 0;
    let (mut map, deepest_rock, max) = make_map(lines);

    // Add "bedrock"
    for x in 0..=max + 500 {
        map.insert((x, deepest_rock + 2));
    }

    // Filling with sand!
    'outer: loop {
        // Sand position.
        let mut x = 500;
        let mut y = 0;

        // Moving particle.
        loop {
            // Can we move below?
            match map.get(&(x, y + 1)) {
                None => y += 1,
                _ => {
                    // Move left?
                    if map.get(&(x - 1, y + 1)).is_none() {
                        x -= 1;
                        y += 1;
                    } else if map.get(&(x + 1, y + 1)).is_none() {
                        x += 1;
                        y += 1;
                    } else {
                        if y == 0 && x == 500 {
                            break 'outer;
                        }
                        map.insert((x, y));
                        break;
                    }
                }
            }
        }

        count += 1;
    }

    count + 1
}

fn make_map(lines: Vec<String>) -> (HashSet<(usize, usize)>, usize, usize) {
    let mut map: HashSet<(usize, usize)> = HashSet::new();
    let mut deepest_rock = 0;
    let mut maxx = 500;

    fn parse_coordinates(s: &str) -> (usize, usize) {
        let (x, y) = s.split_once(',').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }

    for l in lines.iter() {
        let directions: Vec<&str> = l.split(" -> ").collect();
        let mut start_point = parse_coordinates(directions[0]);
        for i in 1..directions.len() {
            let end @ (dx, dy) = parse_coordinates(directions[i]);
            map.insert(start_point);
            if dy > deepest_rock {
                deepest_rock = dy;
            }

            if dx > maxx {
                maxx = dx;
            }

            // Vertical line.
            if dx == start_point.0 {
                for y in usize::min(dy, start_point.1)..=usize::max(dy, start_point.1) {
                    map.insert((dx, y));

                    if y > deepest_rock {
                        deepest_rock = y;
                    }
                }
            } else {
                // Horiontal line.
                for x in usize::min(dx, start_point.0)..=usize::max(dx, start_point.0) {
                    map.insert((x, dy));

                    if x > maxx {
                        maxx = x;
                    }
                }
            }

            start_point = end;
        }
    }

    (map, deepest_rock, maxx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            24
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            93
        );
    }
}
