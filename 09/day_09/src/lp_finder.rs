
use crate::full_gameboard::*;

#[derive(Debug)]
pub struct LPFinder<'a> {
    board: &'a FullGameBoard,
    this_line: usize,
}

impl<'a> LPFinder<'a> {
    pub fn new(board: &'a FullGameBoard) -> Self{
        Self{
            board,
            this_line: 0,
        }
    }
}

