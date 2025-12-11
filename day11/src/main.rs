use std::fs;
use std::env;
use std::str::FromStr;
use pathfinding::prelude::count_paths;

#[derive(Debug)]
struct Device {
    name: String,
    outputs: Vec<String>
}

impl FromStr for Device {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let sections: Vec<_> = line.split(' ').collect();
        let name = sections[0].split(':').next().unwrap().to_string();
        let outputs = sections[1..].iter().map(|s| s.to_string()).collect();
        Ok(Self {
            name,
            outputs
        })
    }
}

struct Map {
    devices: Vec<Device>
}

impl FromStr for Map {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let devices: Vec<Device> = text.lines().map(|s| s.parse().unwrap()).collect();
        Ok(Self { devices })
    }
}

impl Map {
    fn paths(&self, start: &str, end: &str) -> Vec<Path> {
        let mut v = Vec::new();
        for d in &self.devices {
            if d.name == start {
                for o in &d.outputs {
                    if o == end {
                        v.push(Path::at(o));
                    } else {
                        for sub in self.paths(o, end) {
                            v.push(sub.prepend(o));
                        }
                    }
                }
            }
        }
        v
    }
}

struct Path(Vec<String>);

impl Path {
    fn at(s: &str) -> Self {
        Self(vec![s.to_string()])
    }

    fn prepend(self, s: &str) -> Self {
        let mut v = self.0;
        v.insert(0, s.to_string());
        Self(v)
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    at: String,
    fft: bool,
    dac: bool
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let map: Map = text.parse().unwrap();
        let paths = map.paths("you", "out");
        println!("Part1: {}", paths.len());

        let start = State { at: "svr".to_string(), dac: false, fft: false };
        let end = State { at: "out".to_string(), dac: true, fft: true };
        let count = count_paths(start,
            |s| sucessors(s, &map),
            |s| *s == end);
        println!("Part2: {}", count);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn sucessors(state: &State, map: &Map) -> Vec<State> {
    let mut v = Vec::new();
    for d in &map.devices {
        if d.name == state.at {
            for o in &d.outputs {
                let mut new_state = state.clone();
                new_state.at = o.to_string();
                if o == "dac" {
                    new_state.dac = true;
                } else if o == "fft" {
                    new_state.fft = true;
                }
                v.push(new_state);
            }
        }
    }
    v
}