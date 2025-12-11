use std::str::FromStr;
use pathfinding::prelude::bfs;
use pathfinding::prelude::astar;
use crate::button::Button;
use crate::states::JoltageState;
use crate::states::LightState;
use crate::states::State;

#[derive(Debug)]
pub struct Machine {
    lights: LightState,
    buttons: Vec<Button>,
    joltages: JoltageState
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        let sections: Vec<&str> = line.split(' ').collect();
        let mut buttons = Vec::new();
        for s in &sections[1..(sections.len() - 1)] {
            buttons.push(remove_first_and_last(s).parse().unwrap());
        }
        Ok(Self {
            lights: remove_first_and_last(sections[0]).parse().unwrap(),
            buttons,
            joltages: remove_first_and_last(sections[sections.len() - 1]).parse().unwrap()
        })
    }
}

fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next(); // Consume the first character
    chars.next_back(); // Consume the last character
    chars.as_str() // Return the remaining slice
}

impl Machine {
    pub fn minimum_presses_to_lights(&self) -> usize {
        let start = LightState::new(&self.lights);
        let result = bfs(&start,
            |state| state.successors(&self.buttons),
            |state| *state == self.lights);
        result.unwrap().len() - 1
    }

    pub fn minimum_presses_to_joltages(&self) -> u32 {
        let start = JoltageState::new(&self.joltages);
        let result = astar(&start,
            |state| state.successors(&self.buttons, &self.joltages),
            |state| state.min_cost_to_goal(&self.joltages),
            |state| *state == self.joltages);
        result.unwrap().1
    }
}
