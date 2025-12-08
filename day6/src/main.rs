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

    fn calculate_right_to_left(&self, inputs: &Vec<Vec<String>>, i: usize) -> usize {
        let data: Vec<&String> = inputs.iter().map(|v| &v[i]).collect();
        let mut numbers: Vec<usize> = Vec::new();
        for i in 0..data[0].len() {
            let mut s = String::new();
            for d in &data {
                let ch = d.chars().skip(i).next().unwrap();
                if ch != ' ' {
                    s.push(ch);
                }
            }
            if s.len() != 0 {
                numbers.push(s.parse().unwrap());
            }
        }

        let mut n = match self {
            Self::Add => 0,
            Self::Multiply => 1
        };
        for number in numbers {
            n = match self {
                Self::Add => n + number,
                Self::Multiply => n * number
            }
        }
        n
    }
}

fn parse_operators(line: &str) -> Vec<(Operator, usize)> {
    let mut v = Vec::new();
    let mut spaces = 0;
    let mut last_operator = None;
    for ch in line.chars() {
        if ch == '+' {
            if let Some(op) = last_operator {
                v.push((op, spaces));
            }
            last_operator = Some(Operator::Add);
            spaces = 0;
        } else if ch == '*' {
            if let Some(op) = last_operator {
                v.push((op, spaces));
            }
            last_operator = Some(Operator::Multiply);
            spaces = 0;
        } else {
            spaces += 1;
        }
    }
    if let Some(op) = last_operator {
        v.push((op, spaces + 1));
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

fn parse_strings(line: &str, lengths: &Vec<usize>) -> Vec<String> {
    let mut v = Vec::new();
    let mut sum = 0;
    for l in lengths {
        v.push(line.chars().skip(sum).take(*l).collect::<String>());
        sum += *l + 1;
    }
    v
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        // part1
        let count = text.lines().count();
        let numbers: Vec<Vec<usize>> = text.lines().take(count - 1).map(parse_numbers).collect();
        let operators: Vec<Operator> = parse_operators(text.lines().skip(count - 1).next().unwrap()).into_iter().map(|(o, _)| o).collect();
        let mut sum = 0;
        for i in 0..operators.len() {
            sum += operators[i].calculate(&numbers, i);
        }
        println!("Part1: {}", sum);
        // part2
        let lengths: Vec<usize> = parse_operators(text.lines().skip(count - 1).next().unwrap()).into_iter().map(|(_, l)| l).collect();
        let strings: Vec<Vec<String>> = text.lines().take(count - 1).map(|line| parse_strings(line, &lengths)).collect();
        sum = 0;
        for i in 0..operators.len() {
            sum += operators[i].calculate_right_to_left(&strings, i);
        }
        println!("Part2: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}