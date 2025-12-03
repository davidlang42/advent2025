use std::fs;
use std::env;
use std::str::FromStr;

struct Bank {
    batteries: Vec<usize>
}

impl FromStr for Bank {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let batteries: Vec<usize> = line.chars().map(|c| c.to_string().parse().unwrap()).collect();
        Ok(Self {
            batteries
        })
    }
}

impl Bank {
    fn max_jolt(&self) -> usize {
        let first = &self.batteries[0..(self.batteries.len()-1)];
        let f_max = first.iter().max().unwrap();
        let mut max_s = 0;
        for (i, f) in first.iter().enumerate().filter(|(_, f)| *f == f_max) {
            let second = &self.batteries[(i+1)..self.batteries.len()];
            let s = second.iter().max().unwrap();
            if *s > max_s {
                max_s = *s;
            }
        }
        f_max * 10 + max_s
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let banks: Vec<Bank> = text.lines().map(|s| s.parse().unwrap()).collect();
        let mut sum = 0;
        for b in banks {
            sum += b.max_jolt();
        }
        println!("Answer: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}