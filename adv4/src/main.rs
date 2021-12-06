use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn main() {
    let data_file = "./data.txt";
    let mut first_line = true;
    let mut boards: Vec<Board> = vec![];
    let mut current_board = Board::new();
    let mut number_string: String = "".to_owned();
    if let Ok(lines) = read_lines(data_file) {
        for line in lines {
            if let Ok(d) = line {
                if first_line {
                    number_string = d;
                    first_line = false;
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

    resolve(boards.clone(), number_string.clone(), 1);
    resolve(boards.clone(), number_string.clone(), 2);
}

fn resolve(mut boards: Vec<Board>, number_string: String, part: u8) {
    let mut winner: Board = Board::new();
    let mut exclude: Vec<usize> = vec![];
    let mut winner_idx = 0;
    let mut last_win_number = 0;
    'number_loop: for s in number_string.split(",") {
        let last_number = s.parse::<u16>().unwrap();
        for idx in 0..boards.len() {
            boards[idx].mark(last_number);
            if boards[idx].won() && !exclude.contains(&idx) {
                winner = boards[idx].clone();
                last_win_number = last_number;
                winner_idx = idx;
                if part == 1 {
                    break 'number_loop;
                } else {
                    exclude.push(idx);
                }
            }
        }
    }

    let sum = match u32::try_from(winner.unmarked_sum()) {
        Ok(c) => c,
        _ => u32::MAX,
    };

    let last_number = match u32::try_from(last_win_number) {
        Ok(c) => c,
        _ => u32::MAX,
    };

    println!(
        "\nPart: {}, board: {:?}, last_/win/_number: {}/{}, sum: {}, result: {}, Winner idx: {}",
        part,
        winner,
        last_win_number,
        last_number,
        sum,
        sum * &last_number,
        winner_idx
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
        for i in 0..5 {
            found_row = true;
            found_col = true;
            for j in 0..5 {
                if self.filler[i][j] == 0 {
                    found_row = false;
                }
                if self.filler[j][i] == 0 {
                    found_col = false;
                }
                if !found_col && !found_row {
                    break;
                }
            }

            if found_col || found_row {
                break;
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
