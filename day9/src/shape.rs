use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;
use crate::pos::Pos;
use crate::rect::Rect;
use crate::line::Line;

pub struct Shape {
    corners: Vec<Pos>,
    edges: Vec<Line>,
    bounds: Rect,
    unique_x: Vec<usize>,
    unique_y: Vec<usize>
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
        // find bounds
        let bounds = Rect::containing(&corners);
        // make edges
        let mut edges = Vec::new();
        for i in 0..(corners.len() - 1) {
            edges.push(Line::between(corners[i], corners[i+1]));
        }
        edges.push(Line::between(corners[corners.len() - 1], corners[0]));
        // find uniques
        let mut x_set = HashSet::new();
        let mut y_set = HashSet::new();
        for p in &corners {
            x_set.insert(p.x);
            y_set.insert(p.y);
        }
        let mut unique_x: Vec<usize> = x_set.into_iter().collect();
        unique_x.sort();
        let mut unique_y: Vec<usize> = y_set.into_iter().collect();
        unique_y.sort();
        // return
        Self {
            corners,
            edges,
            bounds,
            unique_x,
            unique_y
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
        let x_values = scanline_range(&self.unique_x, r.min.x, r.max.x);
        let y_values = scanline_range(&self.unique_y, r.min.y, r.max.y);
        for x in 1..x_values.len() {
            for y in 1..y_values.len() {
                let r = Rect::new(Pos {
                    x: x_values[x-1],
                    y: y_values[y-1]
                }, Pos {
                    x: x_values[x],
                    y: y_values[y]
                });
                if !self.contains(&r.centre()) {
                    return false;
                }
            }
        }
        true
    }
}

fn scanline_range<'a>(unique: &'a Vec<usize>, from: usize, to: usize) -> &'a [usize] {
    let f = unique.iter().position(|u| *u == from).unwrap();
    let t = unique.iter().position(|u| *u == to).unwrap();
    &unique[f..(t+1)]
}