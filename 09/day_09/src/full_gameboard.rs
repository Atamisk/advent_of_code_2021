use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Debug)]
pub struct FullGameBoard {
    board: Vec<Vec<usize>>,
}

impl FullGameBoard {
    pub fn new(board_file: Lines<BufReader<File>>) -> Self {
        let mut board: Vec<Vec<usize>> = Vec::new();
        for line in board_file{
            board.push(line
                .expect("IO Error!")
                .chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect());
        }
        Self{
            board,
        }
    }
    pub fn get(&self, idx: usize) -> Option<&Vec<usize>> {
        self.board.get(idx)
    }
}

