use std::io::{self, BufRead};
use std::vec;
use std::{fs::File, path::Path};

fn main() {
    let basic_map = vec![
        vec![0, 1, 2, 4, 5, 6],    // 0
        vec![2, 5],                // 1 unique
        vec![0, 2, 3, 4, 6],       // 2
        vec![0, 2, 3, 5, 6],       // 3
        vec![1, 2, 3, 5],          // 4 unique
        vec![0, 1, 3, 5, 6],       // 5
        vec![0, 1, 3, 4, 5, 6],    // 6
        vec![0, 2, 5],             // 7 unique
        vec![0, 1, 2, 3, 4, 5, 6], // 8 unique
        vec![0, 1, 2, 3, 5, 6],    // 9
    ];

    let data = read_data("./data.txt".to_owned());

    let (d, res) = get_data(data);
    println!("Part 1. Ct: {}", find_unique(res.clone()));
    solve(d, res, basic_map);
    // 2 - one
    // 3 - seven
    // 4 - four
    // 7 - eight
}

fn solve(data: Vec<Vec<String>>, res: Vec<Vec<String>>, basics: Vec<Vec<i32>>) {
    let mut sum = 0;
    let mut mapped = vec!["".to_owned(); 10];
    let mut idx = 0;
    for mut s_collection in data {
        let mut positions = vec![' '; 7];
        s_collection.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut two_or_five: Vec<String> = vec![];
        for s in &s_collection {
            let c = match s.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 => {
                    // 2, 3 or five
                    let res;
                    // 3 contains both 1 and 7
                    let mut is_three = true;
                    for i in mapped[7].chars() {
                        if !s.contains(i) {
                            is_three = false;
                        }
                    }
                    if is_three {
                        res = 3;
                    } else {
                        //wait for other iteration
                        res = 99;
                    }
                    res
                }
                7 => 8,
                _ => 100,
            };
            if c == 99 {
                // 2 or 5
                two_or_five.push(s.clone());
            } else if c != 100 {
                mapped[c] = s.clone();
            }
        }
        // compare 7 to 1
        // 7 has top separated
        for c in mapped[7].chars() {
            if !mapped[1].contains(c) {
                positions[0] = c;
                break;
            }
        }

        // compare 4 to 1 and get mid and top left
        let mut mid_and_left_top = "".to_owned();
        for c in mapped[4].chars() {
            if !mapped[1].contains(c) {
                mid_and_left_top.push(c);
            }
        }

        // compare mid and top left to 3 - will get mid
        for c in mid_and_left_top.chars() {
            if mapped[3].contains(c) {
                positions[3] = c;
            } else {
                positions[1] = c;
            }
        }

        // find 5 or 2
        for tf_s in two_or_five {
            if tf_s.contains(positions[0])
                && tf_s.contains(positions[1])
                && tf_s.contains(positions[3])
            {
                // five
                mapped[5] = tf_s;
            } else {
                //two
                mapped[2] = tf_s;
            }
        }

        // get bottom and right lower
        let hed_of_five = format!("{}{}{}", positions[0], positions[1], positions[3]);
        for c in mapped[5].chars() {
            if !hed_of_five.contains(c) {
                if mapped[1].contains(c) {
                    // right lower
                    positions[5] = c;
                } else {
                    // bottom
                    positions[6] = c;
                }
            }
        }

        // get top right
        for c in mapped[1].chars() {
            if c != positions[5] {
                positions[2] = c;
            }
        }

        // get last one
        for c in "abcdefg".chars() {
            if !positions.contains(&c) {
                positions[4] = c;
            }
        }

        let mut result_string = "".to_owned();
        for rez_s in &res[idx] {
            result_string.push(get_number(rez_s.clone(), positions.clone(), basics.clone()));
        }

        sum += result_string.parse::<i32>().unwrap();

        idx += 1;
    }
    println!("Part 2. Sum: {}", sum);
}

fn get_number(rez_s: String, positions: Vec<char>, basics: Vec<Vec<i32>>) -> char {
    match rez_s.len() {
        2 => '1',
        3 => '7',
        4 => '4',
        5 => {
            let mut res = number_or_empty(basics[2].clone(), rez_s.clone(), positions.clone(), '2'); // 2
            if res == ' ' {
                res = number_or_empty(basics[3].clone(), rez_s.clone(), positions.clone(), '3');
                // 3
            }

            if res == ' ' {
                res = number_or_empty(basics[5].clone(), rez_s.clone(), positions.clone(), '5');
                // 5
            }
            res
        }
        7 => '8',
        _ => {
            //069
            let mut res = number_or_empty(basics[6].clone(), rez_s.clone(), positions.clone(), '6'); // 6
            if res == ' ' {
                res = number_or_empty(basics[9].clone(), rez_s.clone(), positions.clone(), '9');
                // 9
            }
            if res == ' ' {
                res = number_or_empty(basics[0].clone(), rez_s.clone(), positions.clone(), '0');
                // 0
            }
            res
        }
    }
}

fn number_or_empty(map: Vec<i32>, st: String, positions: Vec<char>, val: char) -> char {
    let mut res = ' ';
    for i in map {
        if !st.contains(positions[usize::try_from(i).unwrap()]) {
            res = ' ';
            break;
        } else {
            res = val;
        }
    }
    res
}

fn find_unique(res: Vec<Vec<String>>) -> usize {
    let mut c = 0;
    for s_collection in res {
        for s in s_collection {
            c += match s.len() {
                a if [2, 3, 4, 7].contains(&a) => 1,
                _ => 0,
            }
        }
    }
    c
}

fn get_data(data: Vec<String>) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut init_data: Vec<Vec<String>> = vec![];
    let mut result_data: Vec<Vec<String>> = vec![];
    for s in data {
        let s1: Vec<&str> = s.split("|").map(|s| s.trim()).collect();
        init_data.push(s1[0].split(" ").map(|s| s.trim().to_owned()).collect());
        result_data.push(s1[1].split(" ").map(|s| s.trim().to_owned()).collect());
    }

    (init_data, result_data)
}

fn read_data(data_file: String) -> Vec<String> {
    let mut data: Vec<String> = vec![];
    if let Ok(lines) = read_lines(data_file) {
        for line in lines {
            if let Ok(d) = line {
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
