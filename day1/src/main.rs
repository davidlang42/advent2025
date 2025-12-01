use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let rotations: Vec<isize> = text.lines().map(|s| if s.chars().next().unwrap() == 'R' {
            s[1..].parse::<isize>().unwrap()
        } else {
            -1 * s[1..].parse::<isize>().unwrap()
        }).collect();
        let mut dial = 50;
        let mut count = 0;
        for r in rotations {
            if r > 0 {
                for _ in 0..r.abs() {
                    dial += 1;
                    if dial > 99 {
                        dial -= 100;
                    }
                    if dial == 0 {
                        count += 1;
                    }
                }
            }
            if r < 0 {
                for _ in 0..r.abs() {
                    dial -= 1;
                    if dial < 0 {
                        dial += 100;
                    }
                    if dial == 0 {
                        count += 1;
                    }
                }
            }
        }
        println!("Part2: {}", count);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}