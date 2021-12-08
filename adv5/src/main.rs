use std::cmp::{max, min};
use std::convert::TryFrom;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn main() {
    let (max_x, max_y, data) = get_data();
    let mut map = Map::new(vec![vec![0; max_x]; max_y], data);
    map.walk();
    println!("Result 1: {}", map.count_points());
    map.walk_diagonal();
    println!("Result 2: {}", map.count_points());
}

#[derive(Debug, Clone)]
struct Data {
    pub x1: usize,
    pub x2: usize,
    pub y1: usize,
    pub y2: usize,
}

#[derive(Debug, Clone)]
struct Map {
    data: Vec<Vec<usize>>,
    raw_data: Vec<Data>,
}

impl Map {
    pub fn new(data: Vec<Vec<usize>>, raw_data: Vec<Data>) -> Map {
        Map { data, raw_data }
    }

    pub fn walk_diagonal(&mut self) {
        for p in &self.raw_data {
            let v1_diff = i32::abs(usize_to_i32(p.x1) - usize_to_i32(p.x2));
            let v2_diff = i32::abs(usize_to_i32(p.y1) - usize_to_i32(p.y2));
            if p.x1 != p.x2 && p.y1 != p.y2 && v1_diff == v2_diff {
                if p.x1 < p.x2 {
                    if p.y1 < p.y2 {
                        // all grows
                        for i in 0..p.x2 - p.x1 + 1 {
                            self.data[p.y1 + i][p.x1 + i] += 1;
                        }
                    } else {
                        // x grows, y shrinks
                        for i in 0..p.x2 - p.x1 + 1 {
                            self.data[p.y1 - i][p.x1 + i] += 1;
                        }
                    }
                } else {
                    if p.y1 < p.y2 {
                        // x shrinks, y grows
                        for i in 0..p.x1 - p.x2 + 1 {
                            self.data[p.y1 + i][p.x1 - i] += 1;
                        }
                    } else {
                        // all shrinks
                        for i in 0..p.x1 - p.x2 + 1 {
                            self.data[p.y1 - i][p.x1 - i] += 1;
                        }
                    }
                }
            }
        }
    }

    pub fn walk(&mut self) {
        for p in &self.raw_data {
            if p.x1 == p.x2 {
                let y_high = max(p.y1, p.y2) + 1;
                let y_low = min(p.y1, p.y2);
                for idx in y_low..y_high {
                    self.data[idx][p.x1] += 1;
                }
            }
            if p.y1 == p.y2 {
                let y_high = max(p.x1, p.x2) + 1;
                let y_low = min(p.x1, p.x2);
                for idx in y_low..y_high {
                    self.data[p.y1][idx] += 1;
                }
            }
        }
    }

    pub fn count_points(&self) -> i32 {
        let mut final_count = 0;
        for i in 0..self.data[0].len() {
            for j in 0..self.data.len() {
                if self.data[i][j] > 1 {
                    final_count += 1;
                }
            }
        }
        final_count
    }
}

fn get_data() -> (usize, usize, Vec<Data>) {
    let data_file = "./data.txt";
    let mut data_values: Vec<Data> = vec![];
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    if let Ok(lines) = read_lines(data_file) {
        for line in lines {
            if let Ok(d) = line {
                if d.trim() != "".to_owned() {
                    let coordinates: Vec<String> = d.split("->").map(str::to_string).collect();
                    let xy1: Vec<String> = coordinates[0]
                        .trim()
                        .split(",")
                        .map(str::to_string)
                        .collect();
                    let xy2: Vec<String> = coordinates[1]
                        .trim()
                        .split(",")
                        .map(str::to_string)
                        .collect();
                    let (x1, y1) = pair_format(xy1);
                    let (x2, y2) = pair_format(xy2);
                    if max(x1, x2) > max_x {
                        max_x = max(x1, x2);
                    }
                    if max(y1, y2) > max_y {
                        max_y = max(y1, y2);
                    }
                    data_values.push(Data { x1, x2, y1, y2 });
                }
            }
        }
    }

    (max_x + 1, max_y + 1, data_values)
}

fn usize_to_i32(v: usize) -> i32 {
    match i32::try_from(v) {
        Ok(c) => c,
        _ => i32::MAX,
    }
}

fn pair_format(pair: Vec<String>) -> (usize, usize) {
    (
        pair[0].parse::<usize>().unwrap(),
        pair[1].parse::<usize>().unwrap(),
    )
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
