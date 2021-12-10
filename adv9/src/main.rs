use std::io::{self, BufRead};
use std::{fs::File, path::Path};
fn main() {
    let data = read_data("./data.txt".to_owned());
    let (lows, low_coord) = get_adjacent_lows(data.clone());
    let basins = get_largest_basin_multiply(data.clone(), low_coord);
    println!("Part 1. Sum: {}", lows.iter().sum::<i32>());
    println!("Part 2. Basins: {}", basins);
}

fn get_largest_basin_multiply(data: Vec<Vec<i32>>, low_coords: Vec<(usize, usize)>) -> i32 {
    let mut bc: Vec<i32> = vec![];
    for (i, j) in low_coords {
        let (r, _) = find_around_count(data.clone(), i, j, vec![]);
        bc.push(r);
    }
    bc.sort_by(|a, b| b.cmp(a));
    // count largest
    let mut res = bc[0];
    for i in 1..3 {
        res *= bc[i];
    }
    res
}

fn find_around_count(
    data: Vec<Vec<i32>>,
    i: usize,
    j: usize,
    mut exceptions: Vec<(usize, usize)>,
) -> (i32, Vec<(usize, usize)>) {
    let mut ct = 0;
    let mut next: Vec<(usize, usize)> = vec![];
    let cur_el = data[i][j];
    //top
    if i > 0 {
        if cur_el != data[i - 1][j] && data[i - 1][j] != 9 && !exceptions.contains(&(i - 1, j)) {
            ct += 1;
            exceptions.push((i - 1, j));
            next.push((i - 1, j));
        }
    }
    // bottom
    if i + 1 < data.len() {
        if cur_el != data[i + 1][j] && data[i + 1][j] != 9 && !exceptions.contains(&(i + 1, j)) {
            ct += 1;
            exceptions.push((i + 1, j));
            next.push((i + 1, j));
        }
    }
    // left
    if j > 0 {
        if cur_el != data[i][j - 1] && data[i][j - 1] != 9 && !exceptions.contains(&(i, j - 1)) {
            ct += 1;
            exceptions.push((i, j - 1));
            next.push((i, j - 1));
        }
    }

    // right
    if j + 1 < data[0].len() {
        if cur_el != data[i][j + 1] && data[i][j + 1] != 9 && !exceptions.contains(&(i, j + 1)) {
            ct += 1;
            exceptions.push((i, j + 1));
            next.push((i, j + 1));
        }
    }
    if next.len() > 0 {
        for (x, y) in next {
            let (c, ex) = find_around_count(data.clone(), x, y, exceptions.clone());
            exceptions = ex;
            ct += c;
        }
    }

    (ct, exceptions)
}

fn get_adjacent_lows(data: Vec<Vec<i32>>) -> (Vec<i32>, Vec<(usize, usize)>) {
    let mut lows: Vec<i32> = vec![];
    let mut low_coordinates: Vec<(usize, usize)> = vec![];
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            let cur = data[i][j];
            //top
            if i > 0 {
                if cur >= data[i - 1][j] {
                    continue;
                }
            }
            // bottom
            if i + 1 < data.len() {
                if cur >= data[i + 1][j] {
                    continue;
                }
            }

            // left
            if j > 0 {
                if cur >= data[i][j - 1] {
                    continue;
                }
            }

            // right
            if j + 1 < data[0].len() {
                if cur >= data[i][j + 1] {
                    continue;
                }
            }

            lows.push(cur + 1);
            low_coordinates.push((i, j));
        }
    }
    (lows, low_coordinates)
}

fn read_data(data_file: String) -> Vec<Vec<i32>> {
    let mut data: Vec<Vec<i32>> = vec![];
    if let Ok(lines) = read_lines(data_file) {
        for line in lines {
            if let Ok(d) = line {
                if d != "" {
                    let row: Vec<i32> = d
                        .chars()
                        .map(|c| {
                            let mut s = "".to_owned();
                            s.push(c);
                            s.parse::<i32>().unwrap()
                        })
                        .collect();
                    data.push(row);
                }
            }
        }
    }
    data
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
