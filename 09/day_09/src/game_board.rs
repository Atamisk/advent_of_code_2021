use std::fs::File;
use std::io::{BufReader, Lines};
use std::mem::swap;

#[derive(Debug)]
pub struct GameBoard {
    handle: Lines<BufReader<File>>,
    last_line: Option<Vec<usize>>,
    this_line: Vec<usize>,
    next_line: Option<Vec<usize>>,
    this_line_id: usize,
}

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

impl GameBoard {
    pub fn new(mut handle: Lines<BufReader<File>>) -> Self{
        let this_line = handle.pull_handle().unwrap();
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
        let mut tmp_unwrapped = tmp_line.unwrap();
        swap(&mut self.this_line, &mut tmp_unwrapped);
        self.last_line = Some(tmp_unwrapped);
        self.this_line_id += 1;
    }
    pub fn id(&self) -> usize{
        self.this_line_id
    }
    pub fn last_line(&self) -> Option<Vec<usize>> {
        self.last_line.clone()
    }
    pub fn this_line(&self) -> Vec<usize> {
        self.this_line.clone()
    }
    pub fn next_line(&self) -> Option<Vec<usize>> {
        self.next_line.clone()
    }
}
