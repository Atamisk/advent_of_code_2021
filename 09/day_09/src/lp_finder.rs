
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
    pub fn increment(&mut self) {
        self.this_line += 1;
    }
    fn find_low_points(&self) -> Option<usize> {
        if let Some(line) = self.board.get(self.this_line) {
            let mut frames = line.iter().enumerate();
            let mut current_frame = frames.next();
            let mut next_frame = frames.next();
            let mut last_frame: Option<(usize, &usize)> = None;
            let mut risk = 0;
            while let Some((i, frame)) = current_frame {
                let next_frame_val = next_frame.unwrap_or((0, &10)).1;
                let last_frame_val = last_frame.unwrap_or((0, &10)).1;
                if frame < next_frame_val && frame < last_frame_val {
                    let last_line_val = {
                        match self.this_line.checked_sub(1) {
                            Some(last_line) => self.board.get(last_line).unwrap().get(i).unwrap(),
                            None => &10
                        }
                    };
                    let next_line_val = match self.board.get(self.this_line + 1) {
                        Some(ref vec) => { vec.get(i).unwrap() },
                        None => &10,
                    };
                    if frame < last_line_val && frame < next_line_val{
                        risk += frame + 1;
                        println!("Possible low point: ({}, {})", self.this_line, i);
                    }
                }

                last_frame = current_frame;
                current_frame = next_frame;
                next_frame = frames.next();
            }
            Some(risk)
        } else {
            None
        }
    }
}

impl<'a> Iterator for LPFinder<'a>{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item>{
        let out = self.find_low_points();
        //increment board. 
        (*self).increment();
        out
    }
}
