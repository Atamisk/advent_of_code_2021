#[derive(Clone, Copy)]
pub struct Fish{
    timer: u8
}

impl Fish{
    pub fn new(timer: u8) -> Self{
        Self{
            timer
        }
    }
    pub fn new_child() -> Self{
        Self{
            timer: 8
        }
    }
    pub fn tick(&mut self) -> TimerState{
        if self.timer == 0 {
            self.timer = 6;
            TimerState::Reset
        }
        else {
            self.timer -= 1;
            TimerState::Continue
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TimerState{
    Continue,
    Reset
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_timer() {
       let mut fish = Fish::new_child();
       assert_eq!(fish.tick(), TimerState::Continue);
       assert_eq!(fish.tick(), TimerState::Continue);
       assert_eq!(fish.tick(), TimerState::Continue);
       assert_eq!(fish.tick(), TimerState::Continue);
       assert_eq!(fish.tick(), TimerState::Continue);
       assert_eq!(fish.tick(), TimerState::Continue);
       assert_eq!(fish.tick(), TimerState::Continue);
       assert_eq!(fish.tick(), TimerState::Continue);
       assert_eq!(fish.tick(), TimerState::Reset);
    }
}

