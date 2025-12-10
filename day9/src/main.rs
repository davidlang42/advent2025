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

    fn all_rects(&self) -> Vec<Rect> {
        let mut v = Vec::new();
        for i in 0..self.red_tiles.len() {
            for j in (i+1)..self.red_tiles.len() {
                let size = self.red_tiles[i].rect(&self.red_tiles[j]);
                v.push(Rect {
                    size,
                    corners: [self.red_tiles[i], self.red_tiles[j]]
                });
            }
        }
        v.sort_by(|a, b| b.size.cmp(&a.size));
        v
    }

    fn largest_valid_rect(&self, max_size_to_check: Option<usize>) -> usize {
        let rects = self.all_rects();
        let mut i = 0;
        for r in &rects {
            if max_size_to_check.is_some() && r.size > max_size_to_check.unwrap() {
                if i % 1000 == 0 {
                    println!("Skipped {}/{}, answer is less than {}", i, rects.len(), r.size);
                }
                i += 1;
                continue;
            }
            if self.valid_rect(&r.corners[0], &r.corners[1]) {
                return r.size;
            }
            i += 1;
            if i % 1000 == 0 {
                println!("Checked {}/{}, answer is less than {}", i, rects.len(), r.size);
            }
        }
        0
    }

    fn valid_rect(&self, a: &Pos, b: &Pos) -> bool {
        if (a.x as isize - b.x as isize).abs() > (a.y as isize - b.y as isize).abs() {
            // check by col
            let y_range = if a.y < b.y {
                (a.y + 1)..b.y
            } else {
                (b.y + 1)..a.y
            };
            let (x_from, x_to) = if b.x > a.x {
                (a.x + 1, b.x - 1)
            } else {
                (b.x + 1, a.x - 1)
            };
            for y in y_range {
                if self.does_col_cross_edge(x_from, x_to, y) {
                    return false;
                }
            }
        } else {
            // check by row
            let x_range = if a.x < b.x {
                (a.x + 1)..b.x
            } else {
                (b.x + 1)..a.x
            };
            let (y_from, y_to) = if b.y > a.y {
                (a.y + 1, b.y - 1)
            } else {
                (b.y + 1, a.y - 1)
            };
            for x in x_range {
                if self.does_row_cross_edge(x, y_from, y_to) {
                    return false;
                }
            }
        }
        self.is_inside_tile_shape(&Pos {
            x: (a.x + b.x) / 2,
            y: (a.y + b.y) / 2
        })
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

    fn does_row_cross_edge(&self, x: usize, y_from: usize, y_to: usize) -> bool {
        for y in y_from..(y_to + 1) {
            let p = Pos { x, y };
            if self.tiles.contains(&p) {
                return true;
            }
        }
        false
    }

    fn does_col_cross_edge(&self, x_from: usize, x_to: usize, y: usize) -> bool {
        for x in x_from..(x_to + 1) {
            let p = Pos { x, y };
            if self.tiles.contains(&p) {
                return true;
            }
        }
        false
    }

    fn is_inside_tile_shape(&self, p: &Pos) -> bool {
        if self.tiles.contains(p) {
            return true;
        }
        let mut crossings = 0;
        for x in 0..p.x {
            if self.tiles.contains(&Pos { x, y: p.y }) {
                crossings += 1;
            }
        }
        crossings % 2 == 1 // odd crossings means it was inside the shape
    }
}

impl Pos {
    fn rect(&self, other: &Pos) -> usize {
        ((self.x as isize - other.x as isize + 1).abs() * (self.y as isize - other.y as isize + 1).abs()).try_into().unwrap()
    }
}

struct Rect {
    size: usize,
    corners: [Pos; 2]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 || args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let max_size_to_check = if let Some(max) = args.get(2) {
            Some(max.parse().unwrap())
        } else {
            None
        };
        let map: Map = text.parse().unwrap();
        println!("Largest: {}", map.all_rects().iter().next().unwrap().size);
        println!("Valid: {}", map.largest_valid_rect(max_size_to_check));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}