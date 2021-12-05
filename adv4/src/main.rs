use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn main() {
    let data_file = "./data.txt";
    let mut cur_line = 0;
    let mut boards: Vec<Board> = vec![];
    let mut current_board = Board::new();
    let mut number_string: String = "".to_owned();
    if let Ok(lines) = read_lines(data_file) {
        for line in lines {
            if let Ok(d) = line {
                if cur_line == 0 {
                    number_string = d;
                    cur_line += 1;
                } else if d.trim() != "".to_owned() {
                    if current_board.is_full() {
                        boards.push(current_board);
                        current_board = Board::new();
                    }
                    let d = d.trim();
                    let d = d.replace("  ", " ");
                    let row = d.split(" ");
                    for s in row {
                        current_board.fill(s.trim().parse::<u16>().unwrap());
                    }
                }
            }
        }
        boards.push(current_board);
    }

    let mut winner: Board = Board::new();
    let mut last_number = 0;
    'number_loop: for s in number_string.split(",") {
        last_number = s.parse::<u16>().unwrap();
        for b in &mut boards {
            b.mark(last_number);
            if b.won() {
                println!("\n{:?} : {}", b, last_number);
                winner = b.clone();
                break 'number_loop;
            }
        }
    }

    let sum = match u32::try_from(winner.unmarked_sum()) {
        Ok(c) => c,
        _ => u32::MAX,
    };

    let last_number = match u32::try_from(last_number) {
        Ok(c) => c,
        _ => u32::MAX,
    };

    println!(
        "board: {:?}, last_number: {}, sum: {}, result: {}",
        winner,
        last_number,
        sum,
        sum * &last_number
    );
}

#[derive(Debug, Clone)]
struct Board {
    current_row: usize,
    current_col: usize,
    data: Vec<Vec<u16>>,
    filler: Vec<Vec<u16>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            current_col: 0,
            current_row: 0,
            data: vec![vec![0; 5]; 5],
            filler: vec![vec![0; 5]; 5],
        }
    }
    pub fn fill(&mut self, num: u16) {
        if self.current_col < 5 && self.current_row < 5 {
            self.data[self.current_row][self.current_col] = num;
            self.current_col += 1;
        }

        if self.current_col == 5 {
            self.current_col = 0;
            self.current_row += 1;
        }
    }

    pub fn mark(&mut self, val: u16) {
        for i in 0..5 {
            for j in 0..5 {
                if self.data[i][j] == val {
                    self.filler[i][j] = 1;
                }
            }
        }
    }

    pub fn unmarked_sum(&self) -> u16 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.filler[i][j] == 0 {
                    sum += self.data[i][j];
                }
            }
        }

        sum
    }

    pub fn is_full(&self) -> bool {
        self.current_col == 0 && self.current_row == 5
    }

    pub fn won(&self) -> bool {
        let mut found_row = false;
        let mut found_col = false;
        'rows: for i in 0..5 {
            found_row = true;
            found_col = true;
            for j in 0..5 {
                if self.filler[i][j] == 0 {
                    found_row = false;
                }
                if self.filler[j][i] == 0 {
                    found_col = false;
                }
            }

            if found_col || found_row {
                break 'rows;
            }
        }

        found_row || found_col
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
