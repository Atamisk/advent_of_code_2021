use std::fs::File;
use std::io::{BufReader, Lines};

use crate::lp_finder::Handle;

pub struct FullGameBoard {
    board: Vec<Vec<usize>>,
    handle_pos: usize,
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
            handle_pos: 0,
        }
    }
}

impl Handle for FullGameBoard {
    fn pull_handle(&mut self) -> Option<Vec<usize>> {
        let ret = match self.board.get(self.handle_pos){
            Some(line) => Some(line.clone()),
            None => None
        };
        self.handle_pos += 1;
        ret
    }
}
