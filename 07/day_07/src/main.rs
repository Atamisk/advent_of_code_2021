mod crabs; 
use crabs::*;


fn main() {
    let path = std::env::args().nth(1).expect("No path argument provided.");
    let input: String = read_file::read_to_string(&path).unwrap().trim().to_string();
    let crab_list = parse_input(&input);
    let sol_1 = solve_1(crab_list);
    println!("Part 1: {}", sol_1);
}

fn parse_input(input: &str) -> Vec<usize>{
    input
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve_1(cr: Vec<usize>) -> usize {
    let crabs = Crabs::from_list(cr);
    let best_pos = crabs.median();
    crabs.fuel_to(best_pos)
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
}
