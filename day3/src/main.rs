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
    fn max_jolt(&self, digits: usize) -> usize {
        // let first = &self.batteries[0..(self.batteries.len()-1)];
        // let f_max = first.iter().max().unwrap();
        // let mut max_s = 0;
        // for (i, f) in first.iter().enumerate().filter(|(_, f)| *f == f_max) {
        //     let second = &self.batteries[(i+1)..self.batteries.len()];
        //     let s = second.iter().max().unwrap();
        //     if *s > max_s {
        //         max_s = *s;
        //     }
        // }
        // f_max * 10 + max_s
        Self::jolt(&self.batteries, digits).unwrap()
    }

    fn jolt(options: &[usize], digits: usize) -> Option<usize> {
        if options.len() < digits {
            None
        } else if digits == 1 {
            Some(*options.iter().max().unwrap())
        } else {
            let mut max = *options.iter().max().unwrap();
            let mut max_inner = None;
            while max > 0 {
                for (i, _) in options.iter().enumerate().filter(|(_, v)| **v == max) {
                    if let Some(inner) = Self::jolt(&options[(i+1)..], digits - 1) {
                        if max_inner.is_none() || inner > max_inner.unwrap() {
                            max_inner = Some(inner);
                        }
                    }
                }
                if let Some(valid_max_inner) = max_inner {
                    return Some(max * 10_usize.pow((digits - 1).try_into().unwrap()) + valid_max_inner);
                } else {
                    max -= 1;
                }
            }
            None
        }    
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let banks: Vec<Bank> = text.lines().map(|s| s.parse().unwrap()).collect();
        let mut s1 = 0;
        let mut s2 = 0;
        for b in banks {
            s1 += b.max_jolt(2);
            s2 += b.max_jolt(12);
        }
        println!("Part1: {}", s1);
        println!("Part2: {}", s2);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}