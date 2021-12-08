use std::collections::VecDeque;

#[derive(Clone)]
pub struct Fish {
    ring: VecDeque<u128>    
}

impl Fish{
    pub fn new() -> Self {
        Self{
            ring: VecDeque::from(vec![0; 9])
        }
    }
    fn increment(&mut self, i: u8) {
        self.ring[i as usize] += 1;
    }
    pub fn from_ints(ints: Vec<u8>) -> Self{
        let mut out = Self::new();
        for i in ints {
            out.increment(i);
        }
        out
    }
    pub fn total(&self) -> u128{
        self.ring.iter().sum()
    }
    pub fn tick(&mut self){
        self.ring.rotate_left(1);
        self.ring[6] += self.ring[8];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_timer() {
    }
}

