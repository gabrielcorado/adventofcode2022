use common;
use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};

fn main() {
    let lines = common::read_lines_vec("./day11/src/input.txt");
    println!("First part result: {}", first_part(lines.clone()));
    println!("Second part result: {}", second_part(lines.clone()));
}

struct Monkey {
    items: VecDeque<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    divisible_by: usize,
    true_throw: usize,
    false_throw: usize,
    inspections: usize,
}

fn first_part(lines: Vec<String>) -> usize {
    let mut monkeys = parse_monkeys(lines);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspections += 1;
                let val = (monkeys[i].op)(item) / 3;

                let throw_to = if val % monkeys[i].divisible_by == 0 {
                    monkeys[i].true_throw
                } else {
                    monkeys[i].false_throw
                };
                monkeys[throw_to].items.push_back(val);
            }
        }
    }

    // sort monkeys by inspections.
    monkeys.sort_by_key(|m| m.inspections);
    monkeys.reverse();

    monkeys[0].inspections * monkeys[1].inspections
}

fn second_part(lines: Vec<String>) -> usize {
    let mut monkeys = parse_monkeys(lines);
    let product: usize = monkeys.iter().map(|m| m.divisible_by).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspections += 1;
                let val = (monkeys[i].op)(item) % product;

                let throw_to = if val % monkeys[i].divisible_by == 0 {
                    monkeys[i].true_throw
                } else {
                    monkeys[i].false_throw
                };
                monkeys[throw_to].items.push_back(val);
            }
        }
    }

    // sort monkeys by inspections.
    monkeys.sort_by_key(|m| m.inspections);
    monkeys.reverse();

    monkeys[0].inspections * monkeys[1].inspections
}

fn parse_monkeys(lines: Vec<String>) -> Vec<Monkey> {
    let mut iter = lines.iter();
    let mut monkeys: Vec<Monkey> = Vec::new();

    // parse monkeys.
    loop {
        // skip first line.
        iter.next().unwrap();

        let starting_items: VecDeque<usize> = iter
            .next()
            .unwrap()
            .split_once(':')
            .map(|(_, nums)| nums.split(',').map(|n| n.trim().parse().unwrap()))
            .unwrap()
            .collect();

        let op = iter
            .next()
            .unwrap()
            .split_once('=')
            .map(|(_, e)| {
                let raw: Vec<&str> = e.trim().splitn(3, ' ').collect();
                expr(
                    raw[1],
                    raw[0].parse::<usize>().ok(),
                    raw[2].parse::<usize>().ok(),
                )
            })
            .unwrap();

        // seems that only "divisible" is present here, so lets hard code a bit.
        let divisible_by: usize = iter
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let true_throw: usize = iter
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let false_throw: usize = iter
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        monkeys.push(Monkey {
            items: starting_items,
            op,
            divisible_by,
            true_throw,
            false_throw,
            inspections: 0,
        });

        // break line
        if iter.next().is_none() {
            break;
        }
    }

    monkeys
}

fn expr(op_c: &str, a: Option<usize>, b: Option<usize>) -> Box<dyn Fn(usize) -> usize> {
    let op = match op_c {
        "+" => Add::add,
        "-" => Sub::sub,
        "*" => Mul::mul,
        "/" => Div::div,
        _ => unreachable!(),
    };

    Box::new(move |x| op(a.unwrap_or(x), b.unwrap_or(x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            first_part(common::read_lines_vec("./src/test-input.txt")),
            10605
        )
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            second_part(common::read_lines_vec("./src/test-input.txt")),
            2713310158
        )
    }
}
