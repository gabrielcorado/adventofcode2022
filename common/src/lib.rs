use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Alias to file lines.
pub type Lines = io::Lines<io::BufReader<File>>;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
//
// Taken from: https://stackoverflow.com/questions/70206716/how-to-read-a-text-file-in-rust-and-read-mutliple-values-per-line
pub fn read_lines<P>(filename: P) -> Lines
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
