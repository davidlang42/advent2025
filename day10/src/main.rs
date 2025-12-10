use std::fs;
use std::env;
use std::str::FromStr;
use pathfinding::prelude::bfs;

#[derive(Debug)]
struct Machine {
    goal: Vec<bool>,
    buttons: Vec<ButtonSet>,
    joltages: Vec<usize>
}

#[derive(Debug)]
struct ButtonSet(Vec<usize>);

impl FromStr for Machine {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        let sections: Vec<&str> = line.split(' ').collect();

        let mut goal = Vec::new();
        for ch in remove_first_and_last(sections[0]).chars() {
            goal.push(ch == '#');
        }

        let mut buttons = Vec::new();
        for s in &sections[1..(sections.len() - 1)] {
            buttons.push(remove_first_and_last(s).parse().unwrap());
        }

        let mut joltages = Vec::new();
        for j in remove_first_and_last(sections[sections.len() - 1]).split(',') {
            joltages.push(j.parse().unwrap())
        }

        Ok(Self {
            goal,
            buttons,
            joltages
        })
    }
}

fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next(); // Consume the first character
    chars.next_back(); // Consume the last character
    chars.as_str() // Return the remaining slice
}

impl FromStr for ButtonSet {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self(line.split(',').map(|s| s.parse().unwrap()).collect()))
    }
}

impl Machine {
    fn minimum_presses_to_light_goal(&self) -> usize {
        let mut state = Vec::new();
        for _ in 0..self.goal.len() {
            state.push(false);
        }
        let result = bfs(&state, |s| self.successors_light(s), |s| state_matches(s, &self.goal));
        result.unwrap().len() - 1
    }

    fn successors_light(&self, state: &Vec<bool>) -> Vec<Vec<bool>> {
        let mut v = Vec::new();
        for i in 0..self.buttons.len() {
            let mut new_state = state.clone();
            self.buttons[i].push(&mut new_state);
            v.push(new_state);
        }
        v
    }

    fn minimum_presses_to_joltage_goal(&self) -> usize {
        let mut state = Vec::new();
        for _ in 0..self.joltages.len() {
            state.push(0);
        }
        let result = bfs(&state, |s| self.successors_joltage(s, &self.joltages), |s| joltage_matches(s, &self.joltages));
        result.unwrap().len() - 1
    }

    fn successors_joltage(&self, state: &Vec<usize>, goal_state: &Vec<usize>) -> Vec<Vec<usize>> {
        let mut v = Vec::new();
        for i in 0..self.buttons.len() {
            let mut new_state = state.clone();
            if self.buttons[i].push_jolt(&mut new_state, goal_state) {
                v.push(new_state);
            }
        }
        v
    }
}

impl ButtonSet {
    fn push(&self, state: &mut Vec<bool>) {
        for b in &self.0 {
            state[*b] = !state[*b];
        }
    }

    fn push_jolt(&self, state: &mut Vec<usize>, goal: &Vec<usize>) -> bool {
        for b in &self.0 {
            if state[*b] == goal[*b] {
                return false; // too big to be valid
            }
            state[*b] += 1;
        }
        true
    }
}

fn state_matches(a: &Vec<bool>, b: &Vec<bool>) -> bool {
    if a.len() != b.len() {
        panic!("Length mismatch")
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn joltage_matches(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    if a.len() != b.len() {
        panic!("Length mismatch")
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let machines: Vec<Machine> = text.lines().map(|s| s.parse().unwrap()).collect();
        let mut sum = 0;
        for m in &machines {
            let min = m.minimum_presses_to_light_goal();
            println!("{}", min);
            sum += min;
        }
        println!("Part1: {}", sum);

        sum = 0;
        for m in machines {
            let min = m.minimum_presses_to_joltage_goal();
            println!("{}", min);
            sum += min;
        }
        println!("Part2: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}