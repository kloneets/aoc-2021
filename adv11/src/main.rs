use std::convert::TryFrom;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn main() {
    let data = read_data("./data.txt".to_owned());
    let flash_count = flashes(data.clone(), 100);
    println!("Part 1. Flash count: {}", flash_count);
    let all_flash_time = all_done(data);
    println!("Part 2. All flash done at {} step.", all_flash_time);
}

fn all_done(mut data: Vec<Vec<i16>>) -> u16 {
    let mut final_step = 0;
    while !all_tens(data.clone()) {
        for x in 0..data.len() {
            for y in 0..data[0].len() {
                data[x][y] += 1;
            }
        }
        let (d, _c) = flash(data);
        data = d;
        final_step += 1;
    }
    final_step
}

fn all_tens(data: Vec<Vec<i16>>) -> bool {
    let mut done = true;
    'x_loop: for x in 0..data.len() {
        for y in 0..data[0].len() {
            if data[x][y] != 0 {
                done = false;
                break 'x_loop;
            }
        }
    }
    done
}

fn flashes(mut data: Vec<Vec<i16>>, steps: u16) -> i32 {
    let mut flash_count = 0;
    for _step in 0..steps {
        for x in 0..data.len() {
            for y in 0..data[0].len() {
                data[x][y] += 1;
            }
        }
        let (d, c) = flash(data);
        data = d;
        flash_count += c;
    }
    flash_count
}

fn flash(mut data: Vec<Vec<i16>>) -> (Vec<Vec<i16>>, i32) {
    let mut flash_count = 0;
    let mut has_flashes = false;
    for x in 0..data.len() {
        for y in 0..data[0].len() {
            if data[x][y] == 10 {
                flash_count += 1;
                data[x][y] = 11;
                for i in 0..3 {
                    for j in 0..3 {
                        if (i != 1 || j != 1) // self
                        && x + i > 0  // x not -
                        && y + j > 0  // y not -
                        && x + i <= data.len() // x is not out of order 
                        && y + j <= data[0].len()
                        // y is not out of order
                        {
                            let xx = x + i - 1;
                            let yy = y + j - 1;
                            if data[xx][yy] < 10 {
                                data[xx][yy] += 1;
                                if data[xx][yy] == 10 {
                                    has_flashes = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if has_flashes {
        let (d, fc) = flash(data);
        data = d;
        flash_count += fc;
    }

    for x in 0..data.len() {
        for y in 0..data[0].len() {
            if data[x][y] > 9 {
                data[x][y] = 0;
            }
        }
    }
    (data, flash_count)
}

fn read_data(data_file: String) -> Vec<Vec<i16>> {
    let mut data: Vec<Vec<i16>> = vec![];
    if let Ok(lines) = read_lines(data_file) {
        for line in lines {
            if let Ok(d) = line {
                let d = d.trim().to_owned();
                if d != "" {
                    let d: Vec<_> = d
                        .chars()
                        .map(|c| i16::try_from(c.to_digit(10).unwrap()).unwrap())
                        .collect();
                    data.push(d);
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
