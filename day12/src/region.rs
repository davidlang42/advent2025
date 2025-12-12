use std::str::FromStr;
use crate::shape::Shape;

pub struct Region {
    rows: Vec<Vec<bool>>,
    contains_shapes: [usize; 6],
    required_shapes: [usize; 6]
}

impl FromStr for Region {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = line.split(": ").collect();
        let sizes: Vec<&str> = sections[0].split('x').collect();
        let required_shapes: Vec<usize> = sections[1].split(' ').map(|s| s.parse().unwrap()).collect();
        Ok(Self::new(sizes[0].parse().unwrap(), sizes[1].parse().unwrap(), required_shapes.try_into().unwrap()))
    }
}

impl Region {
    fn new(row_size: usize, col_size: usize, required_shapes: [usize; 6]) -> Self {
        let mut rows = Vec::new();
        for _ in 0..row_size {
            let mut row = Vec::new();
            for _ in 0..col_size {
                row.push(false);
            }
            rows.push(row);
        }
        Self {
            rows,
            required_shapes,
            contains_shapes: [0; 6]
        }
    }

    pub fn fill_shapes(&mut self, shapes: &[Shape; 6]) -> bool {
        todo!()
    }
}