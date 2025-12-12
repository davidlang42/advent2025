use std::str::FromStr;
use crate::region::Region;
use crate::shape::ShapeSet;

pub struct Puzzle {
    shapes: [ShapeSet; 6],
    regions: Vec<Region>
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        let mut shapes = Vec::new();
        for i in 0..6 {
            shapes.push(ShapeSet::new(sections[i].parse().unwrap()));
        }
        let regions = sections[shapes.len()].lines().map(|line| line.parse().unwrap()).collect();
        Ok(Self {
            shapes: shapes.try_into().unwrap(),
            regions
        })
    }
}

impl Puzzle {
    pub fn count_successful_regions(&self) -> usize {
        let mut count = 0;
        for r in &self.regions {
            if r.can_fit(&self.shapes) {
                count += 1;
            }
        }
        count
    }
}