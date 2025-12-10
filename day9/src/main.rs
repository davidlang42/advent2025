use std::collections::VecDeque;
use std::fmt::Display;
use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize
}

struct Map {
    red_tiles: Vec<Pos>,
    tiles: HashSet<Pos>
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red_tiles = Vec::new();
        for line in s.lines() {
            let sections: Vec<&str> = line.split(',').collect();
            red_tiles.push(Pos {
                x: sections[0].parse().unwrap(),
                y: sections[1].parse().unwrap()
            })
        }
        Ok(Self::new(red_tiles))
    }
}

impl Map {
    fn _print(&self, max_x: usize, max_y: usize) {
        for x in 0..max_x {
            for y in 0..max_y {
                let p = Pos {x: y, y: x};
                if self.red_tiles.contains(&p) {
                    print!("{}", "#");
                } else if self.tiles.contains(&p) {
                    print!("{}", "X");
                } else {
                    print!("{}", ".");
                }
            }
            println!("");
        }
    }

    fn largest_rect(&self) -> usize {
        let mut max = None;
        for r in self.all_rects() {
            if let Some(existing) = max {
                if r.size > existing {
                    max = Some(r.size);
                }
            } else {
                max = Some(r.size);
            }
        }
        max.unwrap()
    }

    fn all_rects(&self) -> Vec<Rect> {
        let mut v = Vec::new();
        for i in 0..self.red_tiles.len() {
            for j in (i+1)..self.red_tiles.len() {
                v.push(Rect::new(self.red_tiles[i],self.red_tiles[j]));
            }
        }
        v
    }

    fn largest_valid_rect(&self) -> usize {
        let mut all_rects =self.all_rects();
        all_rects.sort_by(|a,b| a.size.cmp(&b.size)); // process in increasing order so we narrow out bad ones first
        let mut rects = VecDeque::new();
        for r in all_rects {
            rects.push_back(r);
        }
        let mut max = None;
        while rects.len() > 0 {
            let rect = rects.pop_front().unwrap();
            if self.valid_rect(&rect) {
                // valid rect, find the highest
                if let Some(existing) = max {
                    if rect.size > existing {
                        max = Some(rect.size);
                        println!("Max valid rect {}", rect.size);
                    }
                } else {
                    max = Some(rect.size);
                    println!("First valid rect {}", rect.size);
                }
                // any rects smaller than this (valid) rect dont need checking because they wont be the max even if valid
                // let before = rects.len();
                // rects.retain(|r| r.size > rect.size);
                // let after = rects.len();
                // println!("Dropped {} small options, now {} remaining", before - after, after);
            } else {
                // any rects which contain this (invalid) rect must also be invalid
                let before = rects.len();
                rects.retain(|r| !r.contains_rect(&rect));
                let after = rects.len();
                if before != after {
                    println!("Dropped {} invalid options, now {} remaining", before - after, after);
                }
            }
        }
        max.unwrap()
    }

    fn valid_rect(&self, r: &Rect) -> bool {
        //print!("Checking {}: ", r);
        for x in r.low.x..(r.high.x + 1) {
            let low_end = Pos { x, y: r.low.y };
            if !self.is_inside_tile_shape(&low_end) {
                //println!("INVALID at {}", p);
                return false;
            }
            let high_end = Pos { x, y: r.high.y };
            let y_from = r.low.y + 1;
            let y_to = r.high.y - 1;
            if self.count_edge_crossings(x, self.tiles.contains(&low_end), y_from, y_to, self.tiles.contains(&high_end)) > 0 {
                //println!("INVALID at {}", p);
                return false;
            }
        }
        //println!("VALID");
        true
    }

    fn new(red_tiles: Vec<Pos>) -> Self {
        let mut map = Self {
            red_tiles,
            tiles: HashSet::new()
        };
        //map._print(9, 14);
        for i in 0..(map.red_tiles.len() - 1) {
            map.add_line(i, i+1);
        }
        map.add_line(map.red_tiles.len() - 1, 0);
        //map._print(9, 14);
        map
    }

    fn add_line(&mut self, i: usize, j: usize) {
        // line from a (including a) to b (NOT including b)
        let a = &self.red_tiles[i];
        let b = &self.red_tiles[j];
        if a.x == b.x {
            if a.y < b.y {
                for y in a.y..b.y {
                    if !self.tiles.insert(Pos { x: a.x, y }) {
                        panic!("Overlapping lines")
                    }
                }
            } else {
                for y in (b.y + 1)..(a.y + 1) {
                    if !self.tiles.insert(Pos { x: a.x, y }) {
                        panic!("Overlapping lines")
                    }
                }
            }
        } else {
            if a.x < b.x {
                for x in a.x..b.x {
                    if !self.tiles.insert(Pos { y: a.y, x }) {
                        panic!("Overlapping lines")
                    }
                }
            } else {
                for x in (b.x + 1)..(a.x + 1) {
                    if !self.tiles.insert(Pos { y: a.y, x }) {
                        panic!("Overlapping lines")
                    }
                }
            }
        }
    }

    fn count_edge_crossings(&self, x: usize, edge_before: bool, y_from: usize, y_to: usize, edge_after: bool) -> usize {
        let mut crossings = 0;
        let mut last_was_edge = edge_before;
        for y in y_from..(y_to + 1) {
            if self.tiles.contains(&Pos { x, y }) {
                if !last_was_edge {
                    crossings += 1;
                    last_was_edge = true;
                }
            } else {
                last_was_edge = false;
            }
        }
        if last_was_edge != edge_after {
            crossings += 1;
        }
        crossings
    }

    fn is_inside_tile_shape(&self, p: &Pos) -> bool {
        if self.tiles.contains(p) {
            return true;
        }
        self.count_edge_crossings(p.x, false, 0, p.y, false) % 2 == 1 // odd crossings means it was inside the shape
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} [{}]", self.actual[0], self.actual[1], self.size)
    }
}

struct Rect {
    low: Pos,
    high: Pos,
    size: usize,
    actual: [Pos; 2]
}

impl Rect {
    fn contains_pos(&self, p: &Pos) -> bool {
        p.x >= self.low.x && p.x <= self.high.x && p.y >= self.low.y && p.y <= self.high.y
    }

    fn contains_rect(&self, r: &Rect) -> bool {
        self.contains_pos(&r.low) && self.contains_pos(&r.high)
    }

    fn new(a: Pos, b: Pos) -> Self {
        let (low, high) = match (a.x > b.x, a.y > b.y) {
            (true, true) => (b, a),
            (false, false) => (a, b),
            (true, false) => (Pos { x: b.x, y: a.y }, Pos { x: a.x, y: b.y }),
            (false, true) => (Pos { x: a.x, y: b.y }, Pos { x: b.x, y: a.y }),
        };
        Rect {
            low,
            high,
            size: (high.x - low.x + 1) * (high.y - low.y + 1),
            actual: [a, b]
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let map: Map = text.parse().unwrap();
        println!("Largest: {}", map.largest_rect());
        println!("Valid: {}", map.largest_valid_rect());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}