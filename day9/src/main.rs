use std::fs;
use std::env;
use std::collections::VecDeque;
use crate::shape::Shape;

mod pos;
mod shape;
mod rect;
mod line;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let shape: Shape = text.parse().unwrap();
        //println!("{}", shape);
        println!("");
        println!("Largest: {}", largest_rect(&shape));
        println!("Valid: {}", largest_valid_rect(&shape));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn largest_rect(shape: &Shape) -> usize {
    shape.all_rects().iter().map(|r| r.size()).max().unwrap()
}

//TODO check logic below here

fn largest_valid_rect(shape: &Shape) -> usize {
    let mut all_rects = shape.all_rects();
    all_rects.sort_by(|a,b| a.size().cmp(&b.size())); // process in increasing order so we narrow out bad ones first
    let mut rects = VecDeque::new();
    for r in all_rects {
        rects.push_back(r);
    }
    let mut max = None;
    while rects.len() > 0 {
        let rect = rects.pop_front().unwrap();
        if shape.encapsulates(&rect) {
            // valid rect, find the highest
            if let Some(existing) = max {
                if rect.size() > existing {
                    max = Some(rect.size());
                    println!("Max valid rect {}", rect.size());
                }
            } else {
                max = Some(rect.size());
                println!("First valid rect {}", rect.size());
            }
            // any rects smaller than this (valid) rect dont need checking because they wont be the max even if valid
            // let before = rects.len();
            // rects.retain(|r| r.size > rect.size);
            // let after = rects.len();
            // println!("Dropped {} small options, now {} remaining", before - after, after);
        } else {
            // any rects which contain this (invalid) rect must also be invalid
            let before = rects.len();
            rects.retain(|r| !r.encapsulates(&rect));
            let after = rects.len();
            if before != after {
                println!("Dropped {} invalid options, now {} remaining", before - after, after);
            }
        }
    }
    max.unwrap()
}
