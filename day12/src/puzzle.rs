use std::str::FromStr;
use crate::region::Region;
use crate::shape::Shape;

pub struct Puzzle {
    shapes: [Shape; 6],
    regions: Vec<Region>
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        let mut shapes = Vec::new();
        for i in 0..6 {
            shapes.push(sections[i].parse().unwrap());
        }
        let regions = sections[shapes.len()].lines().map(|line| line.parse().unwrap()).collect();
        Ok(Self {
            shapes: shapes.try_into().unwrap(),
            regions
        })
    }
}

impl Puzzle {
    pub fn count_successful_regions(&mut self) -> usize {
        let mut count = 0;
        for r in &mut self.regions {
            if r.fill_shapes(&self.shapes) {
                count += 1;
            }
        }
        count
    }
}