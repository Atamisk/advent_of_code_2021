pub struct Crabs{
    crab_subs: Vec<usize>
}

impl Crabs {
    pub fn from_list(mut lst: Vec<usize>) -> Self {
        lst.sort_unstable();
        Self{
            crab_subs: lst
        }
    }
    pub fn median(&self) -> usize {
        let len = self.crab_subs.len();
        if len % 2 == 0 {
            self.crab_subs[(len - 1) / 2]
        }
        else {
            let i = (len - 1) / 2;
            (self.crab_subs[i] + self.crab_subs[i+1]) / 2
        }
    }
    pub fn fuel_to(&self, pos: usize) -> usize{
        self.crab_subs
            .iter()
            .fold(0, |acc, sub| -> usize {
                acc + ((*sub as isize - pos as isize).abs() as usize)
            })
    }
}
