use std::str::FromStr;
use crate::states::State;

#[derive(Debug)]
pub struct Button {
    indices: Vec<usize>
}

impl FromStr for Button {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            indices: line.split(',').map(|s| s.parse().unwrap()).collect()
        })
    }
}

impl Button {
    pub fn push<S: State>(&self, state: &mut S, times: usize) {
        for index in &self.indices {
            for _ in 0..times {
                state.poke(*index);
            }
        }
    }

    pub fn affects(&self, index: usize) -> bool {
        self.indices.contains(&index)
    }
}