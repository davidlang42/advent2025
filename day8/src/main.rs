use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
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
    junctions: Vec<Pos>,
    circuits: Vec<HashSet<Pos>>
}

impl FromStr for Lights {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let junctions: Vec<Pos> = text.lines().map(|s| s.parse().unwrap()).collect();
        Ok(Self {
            junctions,
            circuits: Vec::new()
        })
    }
}

impl Lights {
    fn connect_closest_pair(&mut self, more_than_distance: Option<f64>) -> f64 {
        let mut min = None;
        for i in 0..self.junctions.len() {
            for j in (i+1)..self.junctions.len() {
                let d = self.junctions[i].distance(&self.junctions[j]);
                if more_than_distance.is_some() && d <= more_than_distance.unwrap() {
                    continue;
                }
                if let Some((min_d, _, _)) = min {
                    if d < min_d {
                        min = Some((d, i, j));
                    }
                } else {
                    min = Some((d, i, j));
                }
            }
        }
        let (d, i, j) = min.unwrap();
        self.connect_junctions(self.junctions[i], self.junctions[j]);
        d
    }

    fn connect_junctions(&mut self, a: Pos, b: Pos) {
        println!("Connecting junctions: {:?}, {:?}", a, b);
        if let Some(existing_a) = self.circuits.iter().position(|c| c.contains(&a)) {
            if let Some(existing_b) = self.circuits.iter().position(|c| c.contains(&b)) {
                self.combine_circuits(existing_a, existing_b);
            } else {
                self.circuits[existing_a].insert(b);
            }
        } else {
            if let Some(existing_b) = self.circuits.iter().position(|c| c.contains(&b)) {
                self.circuits[existing_b].insert(a);
            } else {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(a);
                new_circuit.insert(b);
                self.circuits.push(new_circuit);
            }
        }
    }

    fn combine_circuits(&mut self, i: usize, j: usize) {
        if i == j {
            return; // already done
        }
        if i > j {
            let removed = self.circuits.remove(i);
            for junction in removed {
                self.circuits[j].insert(junction);
            }
        } else {
            let removed = self.circuits.remove(j);
            for junction in removed {
                self.circuits[i].insert(junction);
            }
        }
    }

    fn circuit_sizes(&self) -> Vec<usize> {
        let mut sizes: Vec<usize> = self.circuits.iter().map(|c| c.len()).collect();
        sizes.sort();
        sizes.reverse();
        sizes
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let connections: usize = args[2].parse().unwrap();
        let mut lights: Lights = text.parse().unwrap();
        let mut last_distance = None;
        for _ in 0..connections {
            last_distance = Some(lights.connect_closest_pair(last_distance));
        }
        let sizes = lights.circuit_sizes();
        println!("Circuits: {:?}", sizes);
        println!("Result: {}", sizes[0]*sizes[1]*sizes[2]);
    } else {
        println!("Please provide 2 arguments: Filename, Connections");
    }
}