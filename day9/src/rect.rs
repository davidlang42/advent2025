use std::{fmt::Display, ops::Range};

use crate::pos::Pos;

pub struct Rect {
    pub min: Pos,
    pub max: Pos,
    size: usize
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} [{}]", self.min, self.max, self.size)
    }
}

impl Rect {
    pub fn from_corners(a: Pos, b: Pos) -> Self {
        let (min, max) = match (a.x > b.x, a.y > b.y) {
            (true, true) => (b, a),
            (false, false) => (a, b),
            (true, false) => (Pos { x: b.x, y: a.y }, Pos { x: a.x, y: b.y }),
            (false, true) => (Pos { x: a.x, y: b.y }, Pos { x: b.x, y: a.y }),
        };
        Rect::new(min, max)
    }

    pub fn containing(v: &Vec<Pos>) -> Self {
        if v.len() == 0 {
            panic!()
        }
        let mut min = v[0];
        let mut max = v[1];
        for i in 1..v.len() {
            let p = v[i];
            if p.x < min.x {
                min.x = p.x;
            } else if p.x > max.x {
                max.x = p.x;
            }
            if p.y < min.y {
                min.y = p.y;
            } else if p.y > max.y {
                max.y = p.y;
            }
        }
        Rect::new(min, max)
    }

    pub fn new(min: Pos, max: Pos) -> Self {
        Self {
            min,
            max,
            size: (max.x - min.x + 1) * (max.y - min.y + 1)
        }
    }

    pub fn x_range(&self) -> Range<usize> {
        self.min.x..(self.max.x + 1)
    }

    pub fn y_range(&self) -> Range<usize> {
        self.min.y..(self.max.y + 1)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn max_y(&self) -> usize {
        self.max.y
    }

    pub fn centre(&self) -> Pos {
        Pos {
            x: (self.min.x + self.max.x) / 2,
            y: (self.min.y + self.max.y) / 2
        }
    }

    pub fn contains(&self, p: &Pos) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    pub fn encapsulates(&self, r: &Rect) -> bool {
        self.contains(&r.min) && self.contains(&r.max)
    }
}