use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn main() {
    let data = read_data("./data.txt".to_owned());
    let (points, valid_strings) = count_points(data.clone());
    println!("Part 1. Syntax points: {}", points);
    let missing_points = count_missing(valid_strings);
    println!("Part 2. Auto complete points: {}", missing_points);
}

fn count_missing(valid_strings: Vec<String>) -> i64 {
    let mut missing: Vec<i64> = vec![];
    let chars = vec!['(', '{', '[', '<'];
    for s in &valid_strings {
        let mut left = vec![];
        for c in s.chars() {
            if chars.contains(&c) {
                left.push(c);
            } else {
                left.remove(left.len() - 1);
            }
        }
        let mut cur_points = 0;
        let left_rev: Vec<_> = left.clone().into_iter().rev().collect();
        for lc in left_rev {
            cur_points = cur_points * 5;
            match lc {
                '(' => cur_points += 1,
                '{' => cur_points += 3,
                '[' => cur_points += 2,
                '<' => cur_points += 4,
                _ => panic!("has no such char"),
            };
        }
        missing.push(cur_points);
    }
    missing.sort();
    missing[missing.len() / 2]
}

fn count_points(data: Vec<String>) -> (i32, Vec<String>) {
    let mut walked_chars: Vec<char> = vec![];
    let chars = vec!['(', '{', '[', '<'];
    let mut invalid_ct = vec![0; 4];
    let mut valid_strings = vec![];
    for s in data {
        let mut is_valid = true;
        for c in s.chars() {
            if chars.contains(&c) {
                walked_chars.push(c);
            } else {
                let last = walked_chars[walked_chars.len() - 1];
                let found = match c {
                    ')' => '(',
                    '}' => '{',
                    ']' => '[',
                    '>' => '<',
                    _ => panic!("has no such char"),
                };
                if last != found {
                    let c_idx = chars.iter().position(|cc| *cc == found).unwrap();
                    invalid_ct[c_idx] += 1;
                    is_valid = false;
                    break;
                } else {
                    walked_chars.remove(walked_chars.len() - 1);
                }
            }
        }
        if is_valid {
            valid_strings.push(s);
        }
    }
    let mut result = 0;
    let points = vec![3, 1197, 57, 25137];
    let mut idx = 0;
    for i in invalid_ct {
        result += i * points[idx];
        idx += 1;
    }
    (result, valid_strings)
}

fn read_data(data_file: String) -> Vec<String> {
    let mut data: Vec<String> = vec![];
    if let Ok(lines) = read_lines(data_file) {
        for line in lines {
            if let Ok(d) = line {
                let d = d.trim().to_owned();
                if d != "" {
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
