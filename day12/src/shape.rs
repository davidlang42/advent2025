use std::str::FromStr;

#[derive(Debug)]
pub struct Shape {
    pub rows: [[bool; 3]; 3]
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

impl Shape {
    fn rotate_90(&self) -> Self {
        let x = self.rows;
        Self {
            rows: [
                [x[2][0], x[1][0], x[0][0]],
                [x[2][1], x[1][1], x[0][1]],
                [x[2][2], x[1][2], x[0][2]],
            ]
        }
    }

    fn rotate_180(&self) -> Self {
        self.rotate_90().rotate_90()
    }

    fn rotate_270(&self) -> Self {
        self.rotate_90().rotate_90().rotate_90()
    }

    fn flip_horizontal(&self) -> Self {
        let x = self.rows;
        Self {
            rows: [
                x[2],
                x[1],
                x[0],
            ]
        }
    }

    fn flip_vertical(&self) -> Self {
        let x = self.rows;
        Self {
            rows: [
                [x[0][2], x[1][1], x[0][0]],
                [x[1][2], x[1][1], x[1][0]],
                [x[2][2], x[2][1], x[2][0]],
            ]
        }
    }

    fn area(&self) -> usize {
        let mut area = 0;
        for r in &self.rows {
            for c in r {
                if *c {
                    area += 1;
                }
            }
        }
        area
    }

    fn linear_row_len(&self) -> usize {
        let mut max = 0;
        for row in self.rows {
            let len = Self::row_len(row);
            if len > max {
                max = len;
            }
        }
        max
    }

    fn row_len(row: [bool; 3]) -> usize {
        match row {
            [true, true, true] => 3,
            [true, true, false] => 2,
            [false, true, true] => 2,
            _ => 1 // we don't have shapes with empty rows
        }
    }
}

#[derive(Debug)]
pub struct ShapeSet {
    pub shapes: [Shape; 6]
}

impl ShapeSet {
    pub fn new(shape: Shape) -> Self {
        Self {
            shapes: [
                shape.rotate_90(),
                shape.rotate_180(),
                shape.rotate_270(),
                shape.flip_horizontal(),
                shape.flip_vertical(),
                shape
            ]
        }
    }

    pub fn area(&self) -> usize {
        self.shapes[0].area()
    }

    pub fn linear_len(&self) -> usize {
        let mut max = 0;
        for s in &self.shapes {
            let len = s.linear_row_len();
            if len > max {
                max = len;
            }
        }
        max
    }
}