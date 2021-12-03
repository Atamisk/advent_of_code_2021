use read_file;

fn main() {
    let path = std::env::args().nth(1).expect("No path argument provided.");
    let items = parse_file(&path);
    let sol_1 = solve_1(items);
    println!("Part 1: {}", sol_1);
}

fn parse_file(path: &str) -> Vec<String>{
    let lines = read_file::get_lines(path);
    lines
        .map(|line| -> String {
            line
                .unwrap()
        })
        .collect()
}

fn solve_1(items: Vec<String>) -> usize{
    let item_width: usize = items[0].chars().count();
    
    let gamma_rate_str = get_gamma_rate_str(items);
    let gamma_rate = usize::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = invert_bin(gamma_rate, item_width);
    gamma_rate * epsilon_rate
}

fn get_gamma_rate_str(items: Vec<String>) -> String {
    let item_count: usize = items.len();
    let item_width: usize = items[0].chars().count();
    let mut totals: Vec<usize> = vec![0; item_width];

    for item in items{
        for (i, c) in item.chars().enumerate(){
            let value = match c {
                '0' => 0,
                '1' => 1,
                 a  => panic!("Invalid Char: {}", a)
            };
            totals[i] += value;
        }
    }

    //build total string
    totals
        .iter()
        .map(|total| {
            if total * 2 > item_count {
                '1'
            }
            else {
                '0'
            }
        })
        .collect()
}

fn invert_bin(num: usize, width: usize) -> usize{
    num ^ ((1 << width) - 1)
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_bin_convert(){
        assert_eq!(usize::from_str_radix("11110",2).unwrap(), 30); 
    }
    #[test]
    fn test_bin_invert(){
        assert_eq!(invert_bin(22, 5), 9);
    }
    #[test]
    fn test_solve_1(){
        let items: Vec<String> = vec!["00100".to_string(),
                                  "11110".to_string(),
                                  "10110".to_string(),
                                  "10111".to_string(),
                                  "10101".to_string(),
                                  "01111".to_string(),
                                  "00111".to_string(),
                                  "11100".to_string(),
                                  "10000".to_string(),
                                  "11001".to_string(),
                                  "00010".to_string(),
                                  "01010".to_string()];
        let sol_1 = solve_1(items);
        assert_eq!(sol_1, 198);
    }
}
