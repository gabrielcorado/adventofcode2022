use common;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::once;

fn main() {
    let lines = common::read_lines_vec("./day12/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result: {}", second_part(lines.clone()));
}

fn first_part(lines: Vec<String>) -> isize {
    let (start, end, map): ((usize, usize), (usize, usize), Vec<Vec<usize>>) = lines
        .iter()
        .enumerate()
        .fold(((0, 0), (0, 0), Vec::new()), |mut acc, (i, l)| {
            let mut line = Vec::new();
            let mut starting_point = acc.0;
            let mut end_point = acc.1;
            for (j, c) in l.chars().enumerate() {
                let height = char_to_usize(c);
                if c == 'S' {
                    starting_point = (i, j);
                } else if c == 'E' {
                    end_point = (i, j)
                }
                line.push(height);
            }

            acc.2.push(line);
            (starting_point, end_point, acc.2)
        });

    let rows = map.len();
    let cols = map[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = once(start).collect();
    let mut dist: HashMap<(usize, usize), usize> = once((start, 0)).collect();

    while let Some(curr @ (x, y)) = queue.pop_front() {
        if !visited.insert(curr) {
            continue;
        }

        // Found it?
        if curr == end {
            break;
        }

        for neighbour in get_neighbours(x, y, rows, cols) {
            let height = map[neighbour.0][neighbour.1];
            if height <= map[x][y] + 1 {
                let curr_dist = dist[&curr];
                dist.insert(neighbour, curr_dist + 1);
                queue.push_back(neighbour);
            }
        }
    }

    dist[&end] as isize
}

fn second_part(lines: Vec<String>) -> isize {
    let (end, map): ((usize, usize), Vec<Vec<usize>>) =
        lines
            .iter()
            .enumerate()
            .fold(((0, 0), Vec::new()), |mut acc, (i, l)| {
                let mut line = Vec::new();
                let mut end_point = acc.0;
                for (j, c) in l.chars().enumerate() {
                    let height = char_to_usize(c);
                    if c == 'E' {
                        end_point = (i, j)
                    }
                    line.push(height);
                }

                acc.1.push(line);
                (end_point, acc.1)
            });

    let rows = map.len();
    let cols = map[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = once(end).collect();
    let mut dist: HashMap<(usize, usize), usize> = once((end, 0)).collect();

    while let Some(curr @ (x, y)) = queue.pop_front() {
        if !visited.insert(curr) {
            continue;
        }

        // Find any 'a' or 'S'.
        if map[x][y] == 0 {
            return dist[&curr] as isize;
        }

        for neighbour in get_neighbours(x, y, rows, cols) {
            let height = map[neighbour.0][neighbour.1];
            if height >= map[x][y] - 1 {
                let curr_dist = dist[&curr];
                dist.insert(neighbour, curr_dist + 1);
                queue.push_back(neighbour);
            }
        }
    }

    -1
}

fn get_neighbours(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    // Up
    if x > 0 {
        res.push((x - 1, y));
    }

    // Down
    if x < max_x - 1 {
        res.push((x + 1, y));
    }

    // Left
    if y > 0 {
        res.push((x, y - 1));
    }

    // Right
    if y < max_y - 1 {
        res.push((x, y + 1));
    }

    res
}

fn char_to_usize(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize,
        'S' => 0,
        'E' => 25,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            31
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            29
        );
    }
}
