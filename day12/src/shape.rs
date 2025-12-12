use std::str::FromStr;

#[derive(Debug)]
pub struct Shape {
    rows: [[bool; 3]; 3]
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = text.lines().skip(1).collect();
        let mut rows = Vec::new();
        for r in 0..sections.len() {
            let chars: Vec<char> = sections[r].chars().collect();
            let mut row = [false; 3];
            for c in 0..row.len() {
                row[c] = chars[c] == '#';
            }
            rows.push(row);
        }
        Ok(Self { rows: rows.try_into().unwrap() })
    }
}