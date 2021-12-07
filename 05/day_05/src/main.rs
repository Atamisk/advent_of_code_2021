use read_file::*;
use std::cmp;

mod vent;
use vent::*;

mod grid;
use grid::*;

fn main() {
    let path = std::env::args().nth(1).expect("No path argument given");
    let input: Vec<String> = read_file::get_lines(&path)
        .map(|x| x.unwrap())
        .collect();
    let vents = parse_input(input);
    let sol_1 = solve_1(vents.clone());
    let sol_2 = solve_2(vents);
    println!("Part 1: {}", sol_1);
    println!("Part 2: {}", sol_2);
    
}


fn parse_input(input: Vec<String>) -> Vec<Vent>{
    input.into_iter().map(|item| -> Vent {
        let mut tuples = item.split(" -> ");
        let start: Vec<_> = tuples
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let start = (start[0], start[1]);
        let end: Vec<_> = tuples
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let end = (end[0], end[1]);
        Vent::new(start, end)
    })
    .collect()
}

fn solve_1(vents: Vec<Vent>) -> usize{
    let grid_dims = vents
        .iter()
        .fold((0,0), |acc, x| -> (usize, usize) {
            if !x.is_ortho() {
                acc
            }
            else{
                let max_vals = x.max_values();
                (cmp::max(acc.0, max_vals.0), cmp::max(acc.1, max_vals.1))
            }
        });
    let mut grid = Grid::new(grid_dims.0+1, grid_dims.1+1);
    vents
        .into_iter()
        .filter(|x| x.is_ortho())
        .for_each(|x| {
            grid.increment_vent(x)
        });
    grid.count_intersections()
}

fn solve_2(vents: Vec<Vent>) -> usize{
    let grid_dims = vents
        .iter()
        .fold((0,0), |acc, x| -> (usize, usize) {
            let max_vals = x.max_values();
            (cmp::max(acc.0, max_vals.0), cmp::max(acc.1, max_vals.1))
        });
    let mut grid = Grid::new(grid_dims.0+1, grid_dims.1+1);
    vents
        .into_iter()
        .for_each(|x| {
            grid.increment_vent(x)
        });
    grid.count_intersections()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sol_1(){
        let test_str = vec![ "0,9 -> 5,9".to_string(),
                             "8,0 -> 0,8".to_string(),
                             "9,4 -> 3,4".to_string(),
                             "2,2 -> 2,1".to_string(),
                             "7,0 -> 7,4".to_string(),
                             "6,4 -> 2,0".to_string(),
                             "0,9 -> 2,9".to_string(),
                             "3,4 -> 1,4".to_string(),
                             "0,0 -> 8,8".to_string(),
                             "5,5 -> 8,2".to_string()];
        let vents = parse_input(test_str);
        let sol_1 = solve_1(vents);
        assert_eq!(sol_1, 5);
    }
    #[test]
    fn test_sol_2(){
        let test_str = vec![ "0,9 -> 5,9".to_string(),
                             "8,0 -> 0,8".to_string(),
                             "9,4 -> 3,4".to_string(),
                             "2,2 -> 2,1".to_string(),
                             "7,0 -> 7,4".to_string(),
                             "6,4 -> 2,0".to_string(),
                             "0,9 -> 2,9".to_string(),
                             "3,4 -> 1,4".to_string(),
                             "0,0 -> 8,8".to_string(),
                             "5,5 -> 8,2".to_string()];
        let vents = parse_input(test_str);
        let sol_2 = solve_2(vents);
        assert_eq!(sol_2, 12);
    }
}
