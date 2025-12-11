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

    pub fn successors(&self, all_buttons: &Vec<Button>, goal: &Self) -> Vec<(Self, u32)> {
        let (_index, remaining, buttons) = self.find_fewest_available_buttons(all_buttons, goal);
        let mut v = Vec::new();
        for option in Self::combinations_of_button_presses(self, &buttons, remaining, goal) {
            v.push((option, remaining as u32));
        }
        v
    }

    fn combinations_of_button_presses(initial_state: &Self, remaining_buttons: &[&Button], remaining_presses: usize, goal: &JoltageState) -> Vec<Self> {
        let mut v = Vec::new();
        if remaining_buttons.len() == 1 {
            let mut new_state = initial_state.clone();
            remaining_buttons[0].push(&mut new_state, remaining_presses); // only 1 button left, press it the remaining times
            if new_state.is_valid(goal) {
                v.push(new_state);
            }
        } else {
            // try the first button any number of times
            for times in 0..(remaining_presses + 1) {
                let mut new_state = initial_state.clone();
                remaining_buttons[0].push(&mut new_state, times);
                if !new_state.is_valid(goal) {
                    continue;
                }
                // then generate combinations for remaining buttons & presses
                for option in Self::combinations_of_button_presses(&new_state, &remaining_buttons[1..], remaining_presses - times, goal) {
                    v.push(option);
                }
            }
        }
        v
    }

    fn find_fewest_available_buttons<'a>(&self, buttons: &'a Vec<Button>, goal: &Self) -> (usize, usize, Vec<&'a Button>) {
        let mut min: Option<(usize, usize, Vec<&Button>)>  = None;
        for i in 0..self.0.len() {
            if self.0[i] == goal.0[i] {
                continue; // already finished
            }
            let remaining = goal.0[i] - self.0[i];
            let available_buttons: Vec<_> = buttons.iter().filter(|b| b.affects(i)).collect();
            if let Some((_min_i, _min_r, min_b)) = &min {
                if available_buttons.len() < min_b.len() {
                    min = Some((i, remaining, available_buttons));
                }
            } else {
                min = Some((i, remaining, available_buttons));
            }
        }
        min.unwrap()
    }
}