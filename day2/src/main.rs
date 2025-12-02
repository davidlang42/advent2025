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
    fn find_invalid_ids(&self) -> Vec<usize> {
        let mut v = vec![];
        for i in self.start..(self.end+1) {
            let s = i.to_string();
            let l = s.len() / 2;
            if s.len() == 2 * l && &s[0..l] == &s[l..] {
                v.push(i);
            }
        }
        v
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