use std::fmt::Display;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<&str> = line.split(',').collect();
        Ok(Self {
            x: numbers[0].parse().unwrap(),
            y: numbers[1].parse().unwrap()
        })
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}