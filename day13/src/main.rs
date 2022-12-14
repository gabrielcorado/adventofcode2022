use common;
use std::cmp::Ordering;

fn main() {
    let lines = common::read_lines_vec("./day13/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result: {}", second_part(lines.clone()));
}

fn first_part(lines: Vec<String>) -> isize {
    let mut iter = lines.iter();
    let mut total = 0;
    let mut packet = 1;

    loop {
        let left = iter.next().unwrap();
        let right = iter.next().unwrap();

        match compare(&left, &right) {
            Some(true) => total += packet,
            Some(false) => {}
            None => unreachable!(),
        }

        // Skip line break or end parsing.
        if let None = iter.next() {
            break;
        }

        packet += 1;
    }

    total
}

fn second_part(lines: Vec<String>) -> isize {
    let mut sorted: Vec<String> = Vec::new();

    fn search(l: String) -> Box<dyn Fn(&String) -> Ordering> {
        Box::new(move |r| match compare(&l, r) {
            Some(true) => Ordering::Greater,
            Some(false) => Ordering::Less,
            None => unreachable!(),
        })
    }

    for l in lines.iter() {
        if l.is_empty() {
            continue;
        }

        match sorted.binary_search_by(search(l.clone())) {
            Err(i) => sorted.insert(i, l.clone()),
            Ok(_) => unreachable!(),
        }
    }

    (sorted
        .binary_search_by(search("[[2]]".to_owned()))
        .unwrap_err() as isize
        + 1)
        * (sorted
            .binary_search_by(search("[[6]]".to_owned()))
            .unwrap_err() as isize
            + 2)
}

fn is_list(s: &String) -> bool {
    s.starts_with('[')
}

fn compare(l: &String, r: &String) -> Option<bool> {
    // When both are integers, just compare them.
    if !is_list(&l) && !is_list(&r) {
        let ln: isize = l.parse().unwrap();
        let rn: isize = r.parse().unwrap();

        if ln > rn {
            return Some(false);
        } else if ln < rn {
            return Some(true);
        } else {
            return None;
        }
    }

    // Cast them into lists and compare items.
    let ll: Vec<String> = if is_list(&l) {
        split(&strip(&l))
    } else {
        vec![l.clone()]
    };

    let lr: Vec<String> = if is_list(&r) {
        split(&strip(&r))
    } else {
        vec![r.clone()]
    };

    for i in 0..usize::max(ll.len(), lr.len()) {
        if ll.get(i).is_none() {
            return Some(true);
        } else if lr.get(i).is_none() {
            return Some(false);
        }

        match compare(&ll[i], &lr[i]) {
            None => {}
            x => return x,
        }
    }

    None
}

fn split(s: &String) -> Vec<String> {
    if s.len() == 0 {
        return Vec::new();
    }

    let mut inner = 0;
    let mut res: Vec<String> = Vec::new();
    let mut start: usize = 0;

    for (i, c) in s.chars().into_iter().enumerate() {
        match c {
            '[' => inner += 1,
            ']' => inner -= 1,
            ',' => {
                if inner == 0 {
                    res.push(s.get(start..=(i - 1)).unwrap().to_owned());
                    start = i + 1
                }
            }
            _ => {}
        }
    }

    res.push(s.get(start..).unwrap().to_owned());
    res
}

fn strip(s: &String) -> String {
    s.strip_prefix('[')
        .unwrap()
        .strip_suffix(']')
        .unwrap()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            13
        )
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            140
        )
    }
}
