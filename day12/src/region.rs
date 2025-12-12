use std::str::FromStr;
use crate::shape::ShapeSet;
use crate::shape::Shape;
use pathfinding::prelude::bfs;

#[derive(Clone, PartialEq, Eq, Hash)]
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

    pub fn can_fit(&self, shapes: &[ShapeSet; 6]) -> bool {
        if self.is_impossible_by_area(shapes) {
            return false;
        }
        let result = bfs(self, |r| r.successors(shapes), |r| r.is_complete());
        result.is_some()
    }

    fn is_complete(&self) -> bool {
        for i in 0..self.contains_shapes.len() {
            if self.contains_shapes[i] != self.required_shapes[i] {
                return false;
            }
        }
        true
    }

    fn is_impossible_by_area(&self, shapes: &[ShapeSet; 6]) -> bool {
        let mut requires = 0;
        for i in 0..self.contains_shapes.len() {
            requires += self.required_shapes[i] * shapes[i].area();
        }
        let available = self.rows.len() * self.rows[0].len();
        requires > available
    }

    fn successors(&self, shapes: &[ShapeSet; 6]) -> Vec<Self> {
        let mut v = Vec::new();
        // an empty row/col is the last row/col we should be willing to add a shape to,
        // because leaving a full empty row/col is automatically a waste of space
        let mut last_row = self.first_empty_row().unwrap_or(self.rows.len());
        let mut last_col = self.first_empty_col().unwrap_or(self.rows[0].len());
        if last_row > self.rows.len() - 3 {
            last_row = self.rows.len() - 3;
        }
        if last_col > self.rows[0].len() - 3 {
            last_col = self.rows[0].len() - 3;
        }
        // need to try all the shapes, because order of adding them could matter
        for s in 0..shapes.len() {
            if self.contains_shapes[s] == self.required_shapes[s] {
                continue; // already have enough of this shape
            }
            for r in 0..(last_row + 1) {
                let row = &self.rows[r];
                for c in 0..(last_col + 1) {
                    if row[c] {
                        continue; // dont try to start a shape on an existing one
                    }
                    // try this shape in any orientation
                    for orientation in &shapes[s].shapes {
                        if let Some(mut new_region) = self.apply(orientation, r, c) {
                            new_region.contains_shapes[s] += 1;
                            v.push(new_region)
                        }
                    }
                }
            }
        }
        v
    }

    fn first_empty_row(&self) -> Option<usize> {
        for r in 0..self.rows.len() {
            if self.rows[r].iter().all(|v| !v) {
                return Some(r);
            }
        }
        None
    }

    fn first_empty_col(&self) -> Option<usize> {
        for c in 0..self.rows[0].len() {
            if (0..self.rows.len()).all(|r| !self.rows[r][c]) {
                return Some(c);
            }
        }
        None
    }

    fn apply(&self, shape: &Shape, row: usize, col: usize) -> Option<Self> {
        for r in 0..3 {
            for c in 0..3 {
                if shape.rows[r][c] {
                    if row + r >= self.rows.len() || col + c >= self.rows[row + r].len() {
                        return None; // new shape would be out of bounds
                    }
                    if self.rows[row + r][col + c] {
                        return None; // new shape would overlap existing
                    }
                }
            }
        }
        let mut new_region = self.clone();
        for r in 0..3 {
            for c in 0..3 {
                if shape.rows[r][c] {
                    new_region.rows[row + r][col + c] = true;
                }
            }
        }
        Some(new_region)
    }
}