use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
    z: usize
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = line.split(',').collect();
        Ok(Self {
            x: sections[0].parse().unwrap(),
            y: sections[1].parse().unwrap(),
            z: sections[2].parse().unwrap()
        })
    }
}

impl Pos {
    fn distance(&self, other: &Pos) -> f64 {
        let sum = Self::square(self.x, other.x) + Self::square(self.y, other.y) + Self::square(self.z, other.z);
        sum.sqrt()
    }

    fn square(a: usize, b: usize) -> f64 {
        let diff = a as isize - b as isize;
        (diff * diff) as f64
    }
}

struct Lights {
    circuits: Vec<Circuit>
}

impl Lights {
    fn connect_closest_pair(&mut self) {
        if self.circuits.len() == 1 {
            panic!("Only 1 circuit");
        }
        let mut min = None;
        for i in 0..self.circuits.len() {
            for j in (i+1)..self.circuits.len() {
                let d = self.circuits[i].minimum_distance(&self.circuits[j]);
                if let Some((min_d, min_i, min_j)) = min {
                    if d < min_d {
                        min = Some((d, i, j));
                    }
                } else {
                    min = Some((d, i, j));
                }
            }
        }
        let (_, i, j) = min.unwrap();
        self.connect_circuits(i, j);
    }

    fn connect_circuits(&mut self, i: usize, j: usize) {
        if j < i {
            panic!("Connect circuits in increasing order")
        }
        let removed = self.circuits.remove(j);
        for junction in removed.junctions.into_iter() {
            self.circuits[i].junctions.insert(junction);
        }
    }

    fn multiply_three_largest_circuits(&self) -> usize {
        let mut sizes: Vec<usize> = self.circuits.iter().map(|c| c.junctions.len()).collect();
        sizes.sort();
        sizes.reverse();
        sizes[0] * sizes[1] * sizes[2]
    }
}

impl FromStr for Lights {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let junctions: Vec<Pos> = text.lines().map(|s| s.parse().unwrap()).collect();
        let circuits: Vec<Circuit> = junctions.into_iter().map(Circuit::single).collect();
        Ok(Self {
            circuits
        })
    }
}

struct Circuit {
    junctions: HashSet<Pos>
}

impl Circuit {
    fn minimum_distance(&self, other: &Circuit) -> f64 {
        let mut min = None;
        for i in self.junctions.iter() {
            for j in other.junctions.iter() {
                let d = i.distance(j);
                if let Some(min_d) = min {
                    if d < min_d {
                        min = Some(d)
                    }
                } else {
                    min = Some(d);
                }
            }
        }
        min.unwrap()
    }
}

impl Circuit {
    fn single(junction: Pos) -> Self {
        let mut junctions = HashSet::new();
        junctions.insert(junction);
        Self {
            junctions
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut lights: Lights = text.parse().unwrap();
        for i in 0..9 {
            lights.connect_closest_pair();
        }
        

        println!("Answer: {}", lights.multiply_three_largest_circuits());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}