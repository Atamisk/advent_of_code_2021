use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashSet;
use std::convert::{TryInto, TryFrom};

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
            risk,
            biggest_basins:2,
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
    pub fn get_basin_size(&self, coord: (usize, usize)) -> usize {
        let mut visited: HashSet<(usize,usize)> = HashSet::from([coord]);
        let mut stk: Vec<(usize, usize)> = vec![coord];
        while let Some(frame) = stk.pop() {
            let frame_val = self.get_val_coord(frame).unwrap();
            let top = coord_offset(frame, (-1, 0));
            let bot = coord_offset(frame, ( 1, 0));
            let lft = coord_offset(frame, ( 0,-1));
            let rgt = coord_offset(frame, ( 0, 1));
            let neighbors: Vec<(usize, usize)> = [top, bot, lft, rgt].into_iter().filter_map(|x| *x).collect();
            for neighbor in neighbors {
                let neighbor_val = self.get_val_coord(neighbor);
                if let Some(nval) = neighbor_val {
                    if nval > frame_val && nval < 9 && visited.insert(neighbor) {
                        stk.push(neighbor);
                    }
                }
            }
        }
        visited.len()
    }
    fn get_val_coord(&self, coord: (usize, usize)) -> Option<usize> {
        match self.board.get(coord.0) {
            Some(vec) => vec.get(coord.1).cloned(),
            None => None
        }
    }
}

pub struct LPInfo {
    risk: usize,
    biggest_basins: usize
}

impl LPInfo {
    pub fn part_one(&self) -> usize {
        self.risk
    }
}

fn coord_offset(coord: (usize, usize), offset: (isize, isize)) -> Option<(usize, usize)> {
    let new_x = if offset.0 < 0 {
        let offset_x:usize = offset.0.abs().try_into().unwrap();
        coord.0.checked_sub(offset_x)
    } else {
        Some(coord.0 + usize::try_from(offset.0).unwrap())
    };
    let new_y = if offset.1 < 0 {
        let offset_y:usize = offset.1.abs().try_into().unwrap();
        coord.1.checked_sub(offset_y)
    } else {
        Some(coord.1 + usize::try_from(offset.1).unwrap())
    };
    match new_x {
        Some(x) => {
            match new_y {
                Some(y) => Some((x,y)),
                None => None
            }
        },
        None => None,
    }
}
