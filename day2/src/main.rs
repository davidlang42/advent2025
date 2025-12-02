use std::fs;
use std::env;
use std::str::FromStr;
use prime_factorization::Factorization;

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
    fn find_invalid_ids(&self) -> Vec<usize> {
        let mut v = vec![];
        for i in self.start..(self.end+1) {
            let s = i.to_string();
            if s.len() < 2 {
                continue;
            }
            if Self::all_same_char(&s) || Self::repeated_patterns(&s) {
                v.push(i);
            }
        }
        v
    }

    fn all_same_char(s: &str) -> bool {
        let c0 = s.chars().next().unwrap();
        for c in s.chars() {
            if c != c0 {
                return false;
            }
        }
        true
    }

    fn repeated_patterns(s: &str) -> bool {
        let factorization = Factorization::run(s.len() as u128);
        for f in factorization.factors {
            if Self::inner(s, f as usize) {
                return true;
            }
        }
        false
    }

    fn inner(s: &str, f: usize) -> bool {
        let l = s.len() / f;
        let first = &s[0..l];
        for i in 1..f {
            if &s[(i*l)..((i+1) * l)] != first {
                return false;
            }
        }
        true
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let ranges: Vec<Range> = text.split(",").map(|s| s.parse().unwrap()).collect();
        let mut sum = 0;
        for r in ranges {
            for id in r.find_invalid_ids() {
                sum += id;
            }
        }
        println!("Answer: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}