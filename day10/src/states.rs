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
        let mut max_diff = 0;
        for i in 0..self.0.len() {
            let diff = self.0[i].abs_diff(goal.0[i]);
            if diff > max_diff {
                max_diff = diff;
            }
        }
        max_diff as u32
    }

    pub fn successors(&self, buttons: &Vec<Button>, goal: &Self) -> Vec<(Self, u32)> {
        let mut v = Vec::new();
        for i in 0..buttons.len() {
            let mut new_state = self.clone();
            buttons[i].push(&mut new_state, 1);
            if new_state.is_valid(goal) {
                v.push((new_state, 1));
            }
        }
        v
    }
}