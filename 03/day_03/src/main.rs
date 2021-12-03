use read_file;

fn main() {
    let path = std::env::args().nth(1).expect("No path argument provided.");
    let items = parse_file(&path);
    let sol_1 = solve_1(items.clone());
    let sol_2 = solve_2(items);
    println!("Part 1: {}", sol_1);
    println!("Part 2: {}", sol_2);
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
    
    let gamma_rate = get_gamma_rate(items);
    let epsilon_rate = invert_bin(gamma_rate, item_width);
    gamma_rate * epsilon_rate
}

fn solve_2(items: Vec<String>) -> usize {
    let item_width: usize = items[0].chars().count();

    let item_ints: Vec<usize> = items
        .iter()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect();
    
    let first_gamma = single_column_gamma(&item_ints, item_width - 1);
    let (mut o2, mut co2) = get_life_rates(item_ints, first_gamma, item_width - 1);

    for x in (0..(item_width - 1)).rev() {
        if o2.len() > 1{
            let gamma = single_column_gamma(&o2, x);
            o2 = get_life_rates(o2, gamma, x).0;
        }
        if co2.len() > 1 {
            let gamma = single_column_gamma(&co2, x);
            co2 = get_life_rates(co2, gamma, x).1;
        }
    }
    o2[0] * co2[0]
}

fn get_life_rates(
    items: Vec<usize>, 
    gamma_value: bool, 
    bit_loc: usize,
) -> (Vec<usize>, Vec<usize>) {
    let pred = |item: &usize| -> bool {
        let mask = 1 << bit_loc;
        let item_value = (item & mask) == mask;
        item_value == gamma_value
    };
    items.into_iter().partition(pred)
}

fn get_gamma_rate(items: Vec<String>) -> usize {
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
    let gamma_str: String = totals
        .iter()
        .map(|total| {
            if total * 2 >= item_count {
                '1'
            }
            else {
                '0'
            }
        })
        .collect();

    usize::from_str_radix(&gamma_str, 2).unwrap()
}

fn single_column_gamma(items: &Vec<usize>, column: usize) -> bool{
    let item_count = items.len();
    let high_bits_count = items
        .iter()
        .filter(|item: &&usize| -> bool{
            let mask = 1 << column;
            (**item & mask) == mask
        })
        .count();
    high_bits_count * 2 >= item_count
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
    #[test]
    fn test_life_rates(){
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
        let item_ints: Vec<usize> = items
            .iter()
            .map(|x| usize::from_str_radix(x, 2).unwrap())
            .collect();
        let gamma = single_column_gamma(&item_ints,4);

        let (o2, _) = get_life_rates(item_ints, gamma, 4);
        println!("{:?}", o2);
        assert_eq!(o2.iter().sum::<usize>(), 165);
    }
    #[test]
    fn test_solve_2(){
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
        let sol_2 = solve_2(items);
        assert_eq!(sol_2, 230);
    }
}
