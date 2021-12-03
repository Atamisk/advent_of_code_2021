use read_file;

fn main() {
    let path = std::env::args().nth(1).expect("Supply a file path!");
    let values = parse_input(&path);
    let count_1 = solve_part_1(values.clone());
    let count_2 = solve_part_2(values);
    println!("Part 1: {}", count_1);
    println!("Part 2: {}", count_2);
}

fn parse_input(path: &str) -> Vec<usize>{
    let lines = read_file::get_lines(&path);
    lines.map(|line| {
        line
            .unwrap()
            .parse()
            .unwrap()
    })
    .collect()
}

fn solve_part_1(values: Vec<usize>) -> usize{
    let mut values = values.into_iter();
    let mut previous: usize = values
        .next()
        .unwrap();
    let mut count = 0;
    for value in values{
        if value > previous {
            count += 1;
        }
        previous = value;
    }
    count
}

fn solve_part_2(values: Vec<usize>) -> usize{
    let values = &values[..];
    let mut windows = values.windows(3).into_iter();
    let mut previous: usize = windows
        .next()
        .unwrap()
        .iter()
        .sum::<usize>();
    let mut count = 0;
    for window in windows{
        let s: usize = window
            .iter()
            .sum::<usize>();
        if s > previous {
            count += 1;
        }
        previous = s;
    }
    count
}

#[cfg(test)]
mod tests{
    use super::*;
    const VALUES: [usize; 10]  = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_solve_1(){
        let count = solve_part_1(Vec::from(VALUES));
        assert_eq!(count, 7);
    }
    #[test]
    fn test_solve_2(){
        let count = solve_part_2(Vec::from(VALUES));
        assert_eq!(count, 5);
    }
}
