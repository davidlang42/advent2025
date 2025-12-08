use std::fs;
use std::env;

enum Operator {
    Add,
    Multiply
}

impl Operator {
    fn calculate(&self, inputs: &Vec<Vec<usize>>, i: usize) -> usize {
        let mut n = match self {
            Self::Add => 0,
            Self::Multiply => 1
        };
        for input in inputs {
            n = match self {
                Self::Add => n + input[i],
                Self::Multiply => n * input[i]
            }
        }
        n
    }
}

fn parse_operators(line: &str) -> Vec<Operator> {
    let mut v = Vec::new();
    for ch in line.chars() {
        if ch == '+' {
            v.push(Operator::Add);
        } else if ch == '*' {
            v.push(Operator::Multiply);
        }
    }
    v
}

fn parse_numbers(line: &str) -> Vec<usize> {
    let mut v = Vec::new();
    for mut s in line.split(' ') {
        s = s.trim();
        if s.len() != 0 {
            v.push(s.parse().unwrap())
        }
    }
    v
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let count = text.lines().count();
        let numbers: Vec<Vec<usize>> = text.lines().take(count - 1).map(parse_numbers).collect();
        let operators: Vec<Operator> = parse_operators(text.lines().skip(count - 1).next().unwrap());
        let mut sum = 0;
        for i in 0..operators.len() {
            sum += operators[i].calculate(&numbers, i);
        }
        println!("Answer: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}