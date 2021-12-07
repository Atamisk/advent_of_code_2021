mod vent;
use vent::*;

mod grid;
use grid::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sol_1(){
        let test_str = "0,9 -> 5,9\n\
                        8,0 -> 0,8\n\
                        9,4 -> 3,4\n\
                        2,2 -> 2,1\n\
                        7,0 -> 7,4\n\
                        6,4 -> 2,0\n\
                        0,9 -> 2,9\n\
                        3,4 -> 1,4\n\
                        0,0 -> 8,8\n\
                        5,5 -> 8,2";
    }
}
