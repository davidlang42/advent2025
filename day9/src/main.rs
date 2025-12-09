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

    fn largest_valid_rect(&self) -> usize {
        for r in self.all_rects() {
            if self.valid_rect(&r.corners[0], &r.corners[1]) {
                return r.size;
            }
        }
        0
    }

    fn valid_rect(&self, a: &Pos, b: &Pos) -> bool {
        let x_range = if a.x < b.x {
            a.x..(b.x + 1)
        } else {
            b.x..(a.x + 1)
        };
        let y_range = if a.y < b.y {
            a.y..(b.y + 1)
        } else {
            b.y..(a.y + 1)
        };
        for x in x_range {
            for y in y_range.clone() {
                if !self.is_inside_tile_shape(&Pos { x, y }) {
                    return false;
                }
            }
        }
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
        let a = &self.red_tiles[i];
        let b = &self.red_tiles[j];
        if a.x == b.x {
            if a.y < b.y {
                for y in a.y..(b.y + 1) {
                    self.tiles.insert(Pos { x: a.x, y });
                }
            } else {
                for y in b.y..(a.y + 1) {
                    self.tiles.insert(Pos { x: a.x, y });
                }
            }
        } else {
            if a.x < b.x {
                for x in a.x..(b.x + 1) {
                    self.tiles.insert(Pos { y: a.y, x });
                }
            } else {
                for x in b.x..(a.x + 1) {
                    self.tiles.insert(Pos { y: a.y, x });
                }
            }
        }
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
        crossings % 2 == 0 // even crossings means it was inside the shape
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
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let map: Map = text.parse().unwrap();
        println!("Largest: {}", map.all_rects().iter().next().unwrap().size);
        println!("Valid: {}", map.largest_valid_rect());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}