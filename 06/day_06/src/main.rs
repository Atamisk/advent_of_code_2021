mod fish;
use fish::*;

use read_file;

fn main() {
    let path = std::env::args().nth(1).expect("No path argument supplied.");
    let input = read_file::read_to_string(&path).unwrap();
    let fish = parse_input(&input);
    let sol_1 = solve_1(fish.clone(), 80);
    let sol_2 = solve_1(fish, 256);
    println!("Part 1: {}", sol_1);
    println!("Part 2: {}", sol_2);
}

fn parse_input(input: &str) -> Vec<Fish> {
    input
        .trim()
        .split(",")
        .map(|x| Fish::new(x.parse().unwrap()))
        .collect()
}

fn solve_1(mut fish: Vec<Fish>, gens: usize) -> usize {
    for _ in 0..gens {
        let mut new_kids = fish
            .iter_mut()
            .fold(Vec::new(), |mut acc: Vec<Fish>, fishlet: &mut Fish| -> Vec<Fish> {
                match fishlet.tick(){
                    TimerState::Continue => acc,
                    TimerState::Reset    => {
                        acc.push(Fish::new_child());
                        acc
                    }
                }
            });
        fish.append(&mut new_kids);
    }
    fish.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sol_1(){
        let input = "3,4,3,1,2";
        let fish = parse_input(input);
        let sol_1_a = solve_1(fish.clone(), 18);
        let sol_1_b = solve_1(fish, 80);
        assert_eq!(sol_1_a, 26);
        assert_eq!(sol_1_b, 5934);
    }
}
