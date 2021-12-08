mod crabs; 
use crabs::*;


fn main() {
    let path = std::env::args().nth(1).expect("No path argument provided.");
    let input: String = read_file::read_to_string(&path).unwrap().trim().to_string();
    let crab_list = parse_input(&input);
    let sol_1 = solve_1(crab_list.clone());
    let sol_2 = solve_2(crab_list);
    println!("Part 1: {}", sol_1);
    println!("Part 2: {}", sol_2);
}

fn parse_input(input: &str) -> Vec<usize>{
    input
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve_1(cr: Vec<usize>) -> usize {
    let crabs = Crabs::from_list(cr);
    let best_pos = crabs.best_pos_1();
    crabs.fuel_to_1(best_pos)
}
fn solve_2(cr: Vec<usize>) -> usize {
    let crabs = Crabs::from_list(cr);
    crabs.best_fuel_2()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sol_1() {
        let in_str = "16,1,2,0,4,2,7,1,2,14";
        let cr = parse_input(in_str);
        let sol_1 = solve_1(cr);
        assert_eq!(sol_1,37);
    }
    #[test]
    fn sol_2() {
        let in_str = "16,1,2,0,4,2,7,1,2,14";
        let cr = parse_input(in_str);
        let sol_2 = solve_2(cr);
        assert_eq!(sol_2,168);
    }
}
