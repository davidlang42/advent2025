use crate::pos::Pos;

pub struct Line {
    same: Same, // one coordinate is always the same
    from: usize, // the other goes from this minimum value
    to: usize, // to this maximum (inclusive)
}

#[derive(Copy, Clone)]
enum Same {
    X(usize),
    Y(usize)
}

impl Line {
    pub fn between(a: Pos, b: Pos) -> Self {
        if a.x == b.x {
            let (from, to) = if b.y > a.y {
                (a.y, b.y)
            } else {
                (b.y, a.y)
            };
            Self { same: Same::X(a.x), from, to }
        } else if a.y == b.y {
            let (from, to) = if b.x > a.x {
                (a.x, b.x)
            } else {
                (b.x, a.x)
            };
            Self { same:Same::Y(a.y), from, to }
        } else {
            panic!()
        }
    }

    pub fn same_x(x: usize, y_from: usize, y_to: usize) -> Self {
        Self { same:Same::X(x), from: y_from, to: y_to }
    }

    pub fn same_y(y: usize, x_from: usize, x_to: usize) -> Self {
        Self { same:Same::Y(y), from: x_from, to: x_to }
    }

    pub fn contains(&self, p: &Pos) -> bool {
        match self.same {
            Same::X(x) => p.x == x && in_range(p.y, self),
            Same::Y(y) => p.y == y && in_range(p.x, self),
        }
    }

    pub fn crosses(&self, other: &Line) -> bool {
        match (self.same, other.same) {
            // crossing lines
            (Same::X(self_x), Same::Y(other_y)) => in_range(self_x, other) && in_range(other_y, self),
            (Same::Y(self_y), Same::X(other_x)) => in_range(self_y, other) && in_range(other_x, self),
            // overlapping lines
            (Same::X(self_x), Same::X(other_x)) => self_x == other_x && range_overlap(self, other),
            (Same::Y(self_y), Same::Y(other_y)) => self_y == other_y && range_overlap(self, other),
        }
    }
}

fn in_range(value: usize, line: &Line) -> bool {
    value >= line.from && value <= line.to
}

fn range_overlap(a: &Line, b: &Line) -> bool {
    in_range(a.from, b) || in_range(a.to, b) || in_range(b.from, a) || in_range(b.to, a)
}