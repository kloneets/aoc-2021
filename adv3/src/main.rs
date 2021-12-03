use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn main() {
    first_part();
    second_part();
}

fn second_part() {
    let mut ogr_data: Vec<String> = vec![];
    let mut co2_data: Vec<String> = vec![];
    if let Ok(lines) = read_lines("./data.csv") {
        for line in lines {
            if let Ok(d) = line {
                ogr_data.push(d.clone());
                co2_data.push(d.clone());
            }
        }
    }

    let mut i = 0;
    while ogr_data.len() > 1 || i > 12 {
        let mut new_ones: Vec<String> = vec![];
        let mut new_zeros: Vec<String> = vec![];
        for st in ogr_data {
            let ch = st.chars().nth(i).unwrap();
            match ch {
                '0' => new_zeros.push(st),
                '1' => new_ones.push(st),
                _ => (),
            }
        }
        i += 1;
        if new_ones.len() >= new_zeros.len() {
            ogr_data = new_ones;
        } else {
            ogr_data = new_zeros;
        }
    }

    let mut i = 0;
    while co2_data.len() > 1 || i > 12 {
        let mut new_ones: Vec<String> = vec![];
        let mut new_zeros: Vec<String> = vec![];
        for st in co2_data {
            let ch = st.chars().nth(i).unwrap();
            match ch {
                '0' => new_zeros.push(st),
                '1' => new_ones.push(st),
                _ => (),
            }
        }
        i += 1;
        if new_ones.len() < new_zeros.len() && new_ones.len() != 0 {
            co2_data = new_ones;
        } else {
            co2_data = new_zeros;
        }
    }

    let ogr_dec = isize::from_str_radix(ogr_data[0].as_str(), 2).unwrap();
    let co2_dec = isize::from_str_radix(co2_data[0].as_str(), 2).unwrap();

    println!(
        "1. ogr bin: {}, co2 bin: {}, ogr dec: {}, co2 dec: {}, Life support rating: {}",
        ogr_data[0],
        co2_data[0],
        ogr_dec,
        co2_dec,
        ogr_dec * &co2_dec
    );
}

fn first_part() {
    if let Ok(lines) = read_lines("./data.csv") {
        let mut one_zero_count: Vec<OZC> = vec![];
        for _i in 0..12 {
            one_zero_count.push(OZC { zero: 0, one: 0 });
        }
        for line in lines {
            if let Ok(d) = line {
                for (i, c) in d.chars().enumerate() {
                    match c {
                        '0' => one_zero_count[i].zero += 1,
                        '1' => one_zero_count[i].one += 1,
                        _ => (),
                    }
                }
            }
        }

        let mut gamma: String = "".to_owned();
        let mut epsilon: String = "".to_owned();
        for d in one_zero_count {
            if d.zero > d.one {
                gamma.push('0');
                epsilon.push('1');
            } else {
                gamma.push('1');
                epsilon.push('0');
            }
        }

        let gamma_dec = isize::from_str_radix(gamma.as_str(), 2).unwrap();
        let epsilon_dec = isize::from_str_radix(epsilon.as_str(), 2).unwrap();

        println!(
            "1. Gamma bin: {}, Epsilon bin: {}, Gamma dec: {}, Epsilon dec: {}, Power consumption: {}",
            gamma,
            epsilon,
            gamma_dec,
            epsilon_dec,
            gamma_dec * &epsilon_dec
        );
    }
}

#[derive(Debug)]
struct OZC {
    pub zero: i32,
    pub one: i32,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
