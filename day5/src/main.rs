use std::fs;
use std::env;
use std::str::FromStr;

struct Range {
    start: usize,
    end: usize
}

impl FromStr for Range {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = line.split("-").collect();
        Ok(Self {
            start: sections[0].parse().unwrap(),
            end: sections[1].parse().unwrap()
        })
    }
}

impl Range {
    fn contains(&self, n: usize) -> bool {
        n >= self.start && n <= self.end
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        if sections.len() != 2 {
            panic!()
        }
        let ranges: Vec<Range> = sections[0].lines().map(|s| s.parse().unwrap()).collect();
        let ingredients: Vec<usize> = sections[1].lines().map(|s| s.parse().unwrap()).collect();
        let mut count = 0;
        for i in ingredients {
            for r in &ranges {
                if r.contains(i) {
                    count += 1;
                    break;
                }
            }
        }
        println!("Part1: {}", count);
        count = 0;
        let max = ranges.iter().map(|r| r.end).max().unwrap();
        println!("Max: {}", max);
        for i in 0..(max+1) {
            for r in &ranges {
                if r.contains(i) {
                    count += 1;
                    break;
                }
            }
            if i % 1000000000 == 0 {
                println!("{}", i);
            }
        }
        println!("Part2: {}", count);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}