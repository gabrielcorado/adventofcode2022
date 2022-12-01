use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("First part result: {}", first());
    println!("Second part result: {}", second());
}

fn first() -> isize {
    let mut largest: isize = -1;
    let mut curr: isize = 0;

    if let Ok(lines) = read_lines("./day1/src/input.txt") {
        for line in lines {
            let line_val = line.unwrap();
            // If the line is empty there is a new elf.
            if line_val.is_empty() {
                if curr > largest {
                    largest = curr;
                }

                curr = 0;
                continue;
            }

            let val = line_val.as_str().parse::<isize>().unwrap();
            curr += val;
        }
    }

    if curr > largest {
        return curr;
    }

    largest
}

fn second() -> isize {
    let mut heap = BinaryHeap::new();

    // Current elf calories.
    let mut curr: isize = 0;

    if let Ok(lines) = read_lines("./day1/src/input.txt") {
        for line in lines {
            let line_val = line.unwrap();
            // If the line is empty there is a new elf.
            if line_val.is_empty() {
                heap.push(curr);
                curr = 0;
                continue;
            }

            let val = line_val.as_str().parse::<isize>().unwrap();
            curr += val;
        }
    }

    // Take top 3 and sum.
    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
//
// Taken from: https://stackoverflow.com/questions/70206716/how-to-read-a-text-file-in-rust-and-read-mutliple-values-per-line
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
