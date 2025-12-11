use std::fmt::Display;
use std::str::FromStr;
use crate::pos::Pos;
use crate::rect::Rect;
use crate::line::Line;

pub struct Shape {
    corners: Vec<Pos>,
    edges: Vec<Line>,
    bounds: Rect
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let corners = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self::new(corners))
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.bounds.x_range() {
            for y in self.bounds.y_range() {
                let p = Pos {x: y, y: x};//TODO flip?
                if self.corners.contains(&p) {
                    write!(f, "{}", "#")?;
                } else if self.edges.iter().any(|l| l.contains(&p)) {
                    write!(f, "{}", "X")?;
                } else {
                    write!(f, "{}", ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Shape {
    pub fn new(corners: Vec<Pos>) -> Self {
        let bounds = Rect::containing(&corners);
        let mut edges = Vec::new();
        for i in 0..(corners.len() - 1) {
            edges.push(Line::between(corners[i], corners[i+1]));
        }
        edges.push(Line::between(corners[corners.len() - 1], corners[0]));
        Self {
            corners,
            edges,
            bounds
        }
    }

    pub fn all_rects(&self) -> Vec<Rect> {
        let mut v = Vec::new();
        for i in 0..self.corners.len() {
            for j in (i+1)..self.corners.len() {
                v.push(Rect::from_corners(self.corners[i],self.corners[j]));
            }
        }
        v
    }

    //TODO speed up below here

    pub fn contains(&self, p: &Pos) -> bool {
        if !self.bounds.contains(p) { //TODO checking this will rarely help
            return false;
        }
        if self.corners.contains(p) { //TODO edges will include corners anyway
            return true;
        }
        let line_out_of_shape = Line::same_x(p.x, p.y, self.bounds.max_y() + 1);
        let mut crossings = 0;
        for edge in &self.edges {
            if edge.contains(p) { //TODO would probably work without, but this shortcuts
                return true;
            }
            if line_out_of_shape.crosses(edge) {
                crossings += 1;
            }
        }
        crossings % 2 == 1 // odd crossings means point was inside the shape
    }

    pub fn encapsulates(&self, r: &Rect) -> bool {
        //print!("Checking {}: ", r);
        for x in r.x_range() {
            // let low_end = Pos { x, y: r.low.y };
            // if !self.is_inside_tile_shape(&low_end) {
            //     //println!("INVALID at {}", p);
            //     return false;
            // }
            // let high_end = Pos { x, y: r.high.y };
            // let y_from = r.low.y + 1;
            // let y_to = r.high.y - 1;
            // if self.count_edge_crossings(x, self.tiles.contains(&low_end), y_from, y_to, self.tiles.contains(&high_end)) > 0 {
            //     //println!("INVALID at {}", p);
            //     return false;
            // }
            for y in r.y_range() {
                if !self.contains(&Pos { x, y }) {
                    return false;
                }
            }
        }
        //println!("VALID");
        true
    }
}
