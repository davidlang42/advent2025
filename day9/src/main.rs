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
        for i in 0..self.red_tiles.len() {
            for j in (i+1)..self.red_tiles.len() {
                let size = self.red_tiles[i].rect(&self.red_tiles[j]);
                if let Some(existing) = max {
                    if existing < size {
                        max = Some(size);
                    }
                } else {
                    max = Some(size)
                }
            }
        }
        max.unwrap()
    }

    fn largest_valid_rect(&self) -> usize {
        let mut max = None;
        for i in 0..self.red_tiles.len() {
            for j in (i+1)..self.red_tiles.len() {
                if !self.valid_rect(&self.red_tiles[i], &self.red_tiles[j]) {
                    continue;
                }
                let size = self.red_tiles[i].rect(&self.red_tiles[j]);
                if let Some(existing) = max {
                    if existing < size {
                        max = Some(size);
                    }
                } else {
                    max = Some(size)
                }
            }
        }
        max.unwrap()
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
                if !self.tiles.contains(&Pos { x, y }) {
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
        //map.print(9, 14);
        for i in 0..(map.red_tiles.len() - 1) {
            map.add_line(i, i+1);
        }
        map.add_line(map.red_tiles.len() - 1, 0);
        //map.print(9, 14);
        map.fill_inside();
        //map.print(9, 14);
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

    fn fill_inside(&mut self) {
        if let Some(set) = self.try_fill(-1, -1) {
            for s in set  {
                self.tiles.insert(s);
            }
        } else if let Some(set) = self.try_fill(-1, 1) {
            for s in set  {
                self.tiles.insert(s);
            }
        } else if let Some(set) = self.try_fill(1, -1) {
            for s in set  {
                self.tiles.insert(s);
            }
        } else if let Some(set) = self.try_fill(1, 1) {
            for s in set  {
                self.tiles.insert(s);
            }
        } else {
            panic!("Could not fill")
        }
    }

    fn try_fill(&self, delta_x: isize, delta_y: isize) -> Option<HashSet<Pos>> {
        // find a seed which isn't in the tiles set
        let mut seed = self.red_tiles[0];
        while self.tiles.contains(&seed) {
            let x = seed.x as isize + delta_x;
            let y = seed.y as isize + delta_y;
            if x < 0 || y < 0 {
                return None;
            }
            seed = Pos { x: x.try_into().unwrap(), y: y.try_into().unwrap() };
        }
        // fill from that seed
        let mut set = HashSet::new();
        if self.try_fill_inner(&mut set, seed) {
            Some(set)
        } else {
            None
        }
    }

    fn try_fill_inner(&self, set: &mut HashSet<Pos>, current: Pos) -> bool {
        let next = self.find_next(set, &current);
        set.insert(current);
        match next {
            Next::FillComplete => return true,
            Next::ReachedEdge => return false,
            Next::Pos(v) => {
                for p in v {
                    if !self.try_fill_inner(set, p) {
                        return false;
                    }
                }
                return true;
            }
        };
    }

    fn find_next(&self, set: &HashSet<Pos>, current: &Pos) -> Next {
        let try_deltas = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1)
        ];
        let mut v = Vec::new();
        for (dx, dy) in try_deltas {
            let x = current.x as isize + dx;
            let y = current.y as isize + dy;
            if x < 0 || y < 0 {
                return Next::ReachedEdge;
            }
            let p = Pos { x: x.try_into().unwrap(), y: y.try_into().unwrap() };
            if !self.tiles.contains(&p) && !set.contains(&p) {
                v.push(p);
            }
        }
        if v.len() == 0 {
            return Next::FillComplete
        }
        Next::Pos(v)
    }
}

enum Next {
    FillComplete,
    ReachedEdge,
    Pos(Vec<Pos>)
}

impl Pos {
    fn rect(&self, other: &Pos) -> usize {
        ((self.x as isize - other.x as isize + 1).abs() * (self.y as isize - other.y as isize + 1).abs()).try_into().unwrap()
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