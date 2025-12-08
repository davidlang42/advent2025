use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize
}

struct Manifold {
    splitters: HashSet<Pos>,
    size: Pos,
    start: Pos
}

impl FromStr for Manifold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut y = 0;
        let mut x = 0;
        let mut splitters = HashSet::new();
        let mut start = None;
        for line in s.lines() {
            x = 0;
            for ch in line.chars() {
                if ch == '^' {
                    splitters.insert(Pos { x, y });
                } else if ch == 'S' {
                    start = Some(Pos { x, y });
                }
                x += 1;
            }
            y += 1;
        }
        Ok(Self {
            splitters,
            size: Pos { x, y },
            start: start.unwrap()
        })
    }
}

impl Manifold {
    fn simulate(&self) -> usize {
        let mut splits = 0;
        let mut beams: HashSet<Pos> = HashSet::new();
        beams.insert(self.start);
        while beams.len() > 0 {
            let mut new_beams = HashSet::new();
            for b in beams {
                if b.y == self.size.y {
                    break;
                }
                let new_pos = Pos { x: b.x, y: b.y + 1 };
                if self.splitters.contains(&new_pos) {
                    new_beams.insert(Pos { x: b.x - 1, y: b.y + 1 });
                    new_beams.insert(Pos { x: b.x + 1, y: b.y + 1 });
                    splits += 1;
                } else {
                    new_beams.insert(new_pos);
                }
            }
            beams = new_beams;
        }
        splits
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let manifold: Manifold = text.parse().unwrap();
        println!("Splits: {}", manifold.simulate());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}