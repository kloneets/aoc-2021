use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn main() {
    first_task();
    second_task();
}

fn second_task() {
    if let Ok(lines) = read_lines("./data.csv") {
        let mut depth = 0;
        let mut forth = 0;
        let mut aim = 0;
        for line in lines {
            if let Ok(d) = line {
                let sp: Vec<&str> = d.split(" ").collect();
                let to = sp[1].parse::<u32>().unwrap();
                match sp[0].as_ref() {
                    "forward" => {
                        forth += to;
                        depth += to * aim;
                    }
                    "up" => aim -= to,
                    "down" => aim += to,
                    _ => panic!("{}", sp[0]),
                }

            }
        }
        println!(
            "Second task: Depth: {}, Forward: {}, Multi: {}",
            depth,
            forth,
            depth * forth
        )
    }
}

fn first_task() {
    if let Ok(lines) = read_lines("./data.csv") {
        let mut depth = 0;
        let mut forth = 0;
        for line in lines {
            if let Ok(d) = line {
                let sp: Vec<&str> = d.split(" ").collect();
                let to = sp[1].parse::<u32>().unwrap();
                match sp[0].as_ref() {
                    "forward" => forth += to,
                    "up" => depth -= to,
                    "down" => depth += to,
                    _ => panic!("{}", sp[0]),
                }

                // println!("{} c {}", sp[0], &to);
            }
        }
        println!(
            "First task: Depth: {}, Forward: {}, Multi: {}",
            depth,
            forth,
            depth * forth
        )
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
