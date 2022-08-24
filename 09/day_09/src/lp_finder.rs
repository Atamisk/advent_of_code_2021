use std::fs::File;
use std::io::{BufReader, Lines};
use std::mem::swap;

pub trait Handle{
    fn pull_handle(&mut self) -> Option<Vec<usize>>;
}
impl Handle for Lines<BufReader<File>> {
    fn pull_handle(&mut self) -> Option<Vec<usize>> {
        match self.next() {
            Some(res) => Some(res.unwrap().chars().map(|x| x.to_digit(10).unwrap() as usize).collect()),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct LPFinder<T:Handle> {
    handle: T,
    last_line: Option<Vec<usize>>,
    this_line: Option<Vec<usize>>,
    next_line: Option<Vec<usize>>,
    this_line_id: usize,
}

impl<T: Handle> LPFinder<T> {
    pub fn new(mut handle: T) -> Self{
        let this_line = handle.pull_handle();
        let next_line = handle.pull_handle();
        Self{
            handle,
            last_line: None,
            this_line,
            next_line,
            this_line_id: 0,
        }
    }
    pub fn increment(&mut self) {
        let mut tmp_line = self.handle.pull_handle();
        swap(&mut self.next_line, &mut tmp_line);
        swap(&mut self.this_line, &mut tmp_line);
        self.last_line = tmp_line;
        self.this_line_id += 1;
    }
    fn find_low_points(&self) -> usize {
        let line = self.this_line.as_ref().unwrap();
        let mut frames = line.iter().enumerate();
        let mut current_frame = frames.next();
        let mut next_frame = frames.next();
        let mut last_frame: Option<(usize, &usize)> = None;
        let mut risk = 0;
        while let Some((i, frame)) = current_frame {
            let next_frame_val = next_frame.unwrap_or((0, &10)).1;
            let last_frame_val = last_frame.unwrap_or((0, &10)).1;
            if frame < next_frame_val && frame < last_frame_val {
                let last_line_val = match self.last_line {
                    Some(ref vec) => { vec.get(i).unwrap() },
                    None => &10,
                };
                let next_line_val = match self.next_line {
                    Some(ref vec) => { vec.get(i).unwrap() },
                    None => &10,
                };
                if frame < last_line_val && frame < next_line_val{
                    risk += frame + 1;
                    println!("Possible low point: ({}, {})", self.this_line_id, i);
                }
            }

            last_frame = current_frame;
            current_frame = next_frame;
            next_frame = frames.next();
        }
        risk
    }
    pub fn id(&self) -> usize{
        self.this_line_id
    }
    pub fn last_line(&self) -> Option<Vec<usize>> {
        self.last_line.clone()
    }
    pub fn this_line(&self) -> Option<Vec<usize>> {
        self.this_line.clone()
    }
    pub fn next_line(&self) -> Option<Vec<usize>> {
        self.next_line.clone()
    }
}

impl<T: Handle> Iterator for LPFinder<T>{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item>{
        let out = match &self.this_line {
            //find low point
            Some(_res) => Some(self.find_low_points()),
            None => None,
        };
        println!("Line ID: {:?}\n\tLast Line: {:?}\n\tThis Line {:?}\n\tNext Line {:?}", self.id(), self.last_line(), self.this_line(), self.next_line());
        //increment board. 
        self.increment();
        out
    }
}
