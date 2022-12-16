use common;
use std::collections::HashSet;

fn main() {
    let lines = common::read_lines_vec("./day15/src/input.txt");
    println!("First part result: {}", first_part(lines.clone(), 2000000));
}

fn first_part(lines: Vec<String>, interest_point: isize) -> usize {
    type Point = (isize, isize);

    fn parse_coordinates(s: &str) -> Point {
        let (raw_x, raw_y) = s.split_once(", ").unwrap();
        (
            raw_x.get(2..).unwrap().parse().unwrap(),
            raw_y.get(2..).unwrap().parse().unwrap(),
        )
    }

    let mut reads: Vec<(Point, Point, isize)> = Vec::new();
    let mut interest_line: HashSet<(isize, isize)> = HashSet::new();

    for l in lines.iter() {
        let sensor = parse_coordinates(l.get(10..l.find(':').unwrap()).unwrap());
        let beacon = parse_coordinates(l.get(l.rfind('x').unwrap()..).unwrap());
        let dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        // If the beacon is present in the line of interest, add it.
        if beacon.1 == interest_point {
            interest_line.insert(beacon);
        }

        // Only consider if they'll "hit" the interest point.
        if (interest_point <= sensor.1 && interest_point >= sensor.1 - dist)
            || (interest_point >= sensor.1 && interest_point <= sensor.1 + dist)
        {
            reads.push((sensor, beacon, dist));
        }
    }

    for r in reads.iter() {
        let (sensor, _, dist) = r;
        let line_dist = dist - (interest_point - sensor.1).abs();
        for x in (sensor.0 - line_dist + 1)..=(sensor.0 + line_dist) {
            interest_line.insert((x, interest_point));
        }
    }

    interest_line.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt"), 10),
            26
        )
    }
}
