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
    rolls: HashSet<Pos>,
    size: Pos
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut y = 0;
        let mut x = 0;
        let mut rolls = HashSet::new();
        for line in s.lines() {
            x = 0;
            for ch in line.chars() {
                if ch == '@' {
                    rolls.insert(Pos { x, y });
                }
                x += 1;
            }
            y += 1;
        }
        Ok(Self {
            rolls,
            size: Pos { x, y }
        })
    }
}

impl Pos {
    fn adjacents(&self) -> Vec<Pos> {
        let mut v = vec![
            Pos { x: self.x + 1, y: self.y },
            Pos { x: self.x , y: self.y + 1 },
            Pos { x: self.x + 1, y: self.y + 1 },
        ];
        if self.x > 0 {
            v.push(Pos { x: self.x - 1, y: self.y });
            v.push(Pos { x: self.x - 1, y: self.y + 1 });
            if self.y > 0 {
                v.push(Pos { x: self.x - 1, y: self.y - 1 });
            }
        }
        if self.y > 0 {
            v.push(Pos { x: self.x , y: self.y - 1 });
            v.push(Pos { x: self.x + 1, y: self.y - 1 });
        }
        v
    }
}

impl Map {
    fn moveable_rolls(&self) -> Vec<Pos> {
        let mut v = Vec::new();
        for pos in &self.rolls {
            if pos.adjacents().into_iter().filter(|p| self.rolls.contains(p)).count() < 4 {
                v.push(*pos);
            }
        }
        v
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut map: Map = text.parse().unwrap();
        let mut removed = 0;
        while true {
            let moveable = map.moveable_rolls();
            if moveable.len() == 0 {
                break;
            }
            println!("Removing {} rolls", moveable.len());
            removed += moveable.len();
            for roll in moveable {
                map.rolls.remove(&roll);                
            }
        }
        println!("Removed: {}", removed);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}