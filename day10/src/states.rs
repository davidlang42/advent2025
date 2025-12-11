use std::str::FromStr;

use crate::button::Button;

pub trait State : Eq {
    fn is_valid(&self, goal: &Self) -> bool;
    fn poke(&mut self, index: usize);
    fn new(goal: &Self) -> Self;
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct LightState(Vec<bool>);

impl FromStr for LightState {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self(line.chars().map(|c| c == '#').collect()))
    }
}

impl State for LightState {
    fn new(goal: &Self) -> Self {
        let mut v = Vec::new();
        for _ in 0..goal.0.len() {
            v.push(false);
        }
        Self(v)
    }

    fn is_valid(&self, _goal: &Self) -> bool {
        true
    }

    fn poke(&mut self, index: usize) {
        self.0[index] = !self.0[index];
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct JoltageState(Vec<usize>);

impl FromStr for JoltageState {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self(line.split(',').map(|s| s.parse().unwrap()).collect()))
    }
}

impl State for JoltageState {
    fn new(goal: &Self) -> Self {
        let mut v = Vec::new();
        for _ in 0..goal.0.len() {
            v.push(0);
        }
        Self(v)
    }

    fn is_valid(&self, goal: &Self) -> bool {
        for i in 0..self.0.len() {
            if self.0[i] > goal.0[i] {
                return false;
            }
        }
        true
    }

    fn poke(&mut self, index: usize) {
        self.0[index] += 1;
    }
}

impl LightState {
    pub fn successors(&self, buttons: &Vec<Button>) -> Vec<Self> {
        let mut v = Vec::new();
        for i in 0..buttons.len() {
            let mut new_state = self.clone();
            buttons[i].push(&mut new_state, 1);
            v.push(new_state);
        }
        v
    }
}

impl JoltageState {
    pub fn min_cost_to_goal(&self, goal: &Self) -> u32 {
        self.remaining_to_goal(goal).into_iter().max().unwrap() as u32
    }

    fn remaining_to_goal(&self, goal: &Self) -> Vec<usize> {
        let mut v = Vec::new();
        for i in 0..self.0.len() {
            v.push(self.0[i].abs_diff(goal.0[i]));
        }
        v
    }

    pub fn successors(&self, all_buttons: &Vec<Button>, goal: &Self) -> Vec<(Self, u32)> {
        // calculate remaining diff
        let remaining = self.remaining_to_goal(goal);

        // find any indices which are already finished
        let mut finished_indices = Vec::new();
        let mut unfinished_indices = Vec::new();
        for i in 0..remaining.len() {
            if remaining[i] == 0 {
                finished_indices.push(i);
            } else {
                unfinished_indices.push(i);
            }
        }
        
        // find the unfinished index with the least available buttons to press
        let mut min: Option<(usize, Vec<&Button>)> = None;
        for i in unfinished_indices {
            let available_buttons: Vec<_> = all_buttons.iter().filter(|b| b.indices.contains(&i) && !finished_indices.iter().any(|f| b.indices.contains(f))).collect();
            if let Some((_min_r, min_b)) = &min {
                if available_buttons.len() < min_b.len() {
                    min = Some((i, available_buttons));
                }
            } else {
                min = Some((i, available_buttons));
            }
        }
        let (_index, available_buttons) = min.unwrap();
        
        // enumerate options for pressing the available buttons
        let mut v = Vec::new();
        for button in available_buttons {
            let max_presses = button.indices.iter().map(|i| remaining[*i]).min().unwrap();
            for times in 0..(max_presses + 1) {
                let mut new_state = self.clone();
                button.push(&mut new_state, times);
                if new_state.is_valid(goal) {
                    v.push((new_state, times as u32));
                }
            }
        }
        v
    }
}