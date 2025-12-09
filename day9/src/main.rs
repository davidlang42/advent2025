use std::fs;
use std::env;
use std::str::FromStr;

struct Pos {
    x: usize,
    y: usize
}

struct Map {
    red_tiles: Vec<Pos>
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
        Ok(Self {
            red_tiles
        })
    }
}

impl Map {
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
    } else {
        println!("Please provide 1 argument: Filename");
    }
}