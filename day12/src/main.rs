use std::fs;
use std::env;
use crate::puzzle::Puzzle;

mod pos;
mod region;
mod shape;
mod puzzle;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut puzzle: Puzzle = text.parse().unwrap();
        println!("Answer: {}", puzzle.count_successful_regions());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}