use std::fs;
use std::env;
use crate::machine::Machine;

mod machine;
mod button;
mod states;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let machines: Vec<Machine> = text.lines().map(|s| s.parse().unwrap()).collect();
        let mut part1 = 0;
        for m in &machines {
            let min = m.minimum_presses_to_lights();
            println!("{}", min);
            part1 += min;
        }
        println!("Part1: {}", part1);

        let mut part2 = 0;
        for m in machines {
            let min = m.minimum_presses_to_joltages();
            println!("{}", min);
            part2 += min;
        }
        println!("Part2: {}", part2);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}