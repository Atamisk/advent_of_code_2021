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
    pub fn find_all_low_points(&self) -> LPInfo {
        let max_idx = self.board.len();
        let risk: usize = (0..max_idx).map(|x| self.find_line_low_points(x).unwrap()).sum();
        LPInfo{
            part_one:risk,
            part_two:2,
        }
    }
    fn find_line_low_points(&self, idx: usize) -> Option<usize> {
        if let Some(line) = self.board.get(idx) {
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
                        match idx.checked_sub(1) {
                            Some(last_line) => self.board.get(last_line).unwrap().get(i).unwrap(),
                            None => &10
                        }
                    };
                    let next_line_val = match self.board.get(idx + 1) {
                        Some(ref vec) => { vec.get(i).unwrap() },
                        None => &10,
                    };
                    if frame < last_line_val && frame < next_line_val{
                        risk += frame + 1;
                        println!("Possible low point: ({}, {})", idx, i);
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

pub struct LPInfo {
    part_one: usize,
    part_two: usize
}

impl LPInfo {
    pub fn part_one(&self) -> usize {
        self.part_one
    }
}
