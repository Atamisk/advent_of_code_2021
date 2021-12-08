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
    pub fn best_pos_1(&self) -> usize {
        median(self.crab_subs.clone())
    }
    pub fn best_fuel_2(&self) -> usize {
        let mut min_fuel = usize::MAX;
        for pos in 0..{
            let current_fuel = self.fuel_to_2(pos);
            if current_fuel > min_fuel {
                break;
            }
            else {
                min_fuel = current_fuel;
            }
        }
        min_fuel
    }
    pub fn fuel_to_1(&self, pos: usize) -> usize{
        self.crab_subs
            .iter()
            .fold(0, |acc, sub| -> usize {
                acc + ((*sub as isize - pos as isize).abs() as usize)
            })
    }
    pub fn fuel_to_2(&self, pos:usize) -> usize {
        self.crab_subs
            .iter()
            .fold(0, |acc, x| -> usize {
                let err = (pos as isize - *x as isize).abs() as usize;
                if err != 0 {
                    acc + (err.pow(2)-((err-1)*err/2))
                }
                else {
                    acc
                }
            })
    }
}
fn median(v: Vec<usize>) -> usize {
    let len = v.len();
    if len % 2 == 0 {
        v[(len - 1) / 2]
    }
    else {
        let i = (len - 1) / 2;
        (v[i] + v[i+1]) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fuel(){
        let cr = vec![16,1,2,0,4,2,7,1,2,14];
        let subs = Crabs::from_list(cr);
        let med = subs.best_fuel_2();
        assert_eq!(med, 168);
    }
}
