use read_file;

pub fn part_one(fname: &String) -> usize {
    read_file::get_lines(fname)
        .map(|line| process_line_p1(&line.unwrap()))
        .sum()
}

pub fn part_two(fname: &String) -> usize {
    42
}

fn process_line_p1(line: &String) -> usize {
    line_to_digits(line).1
        .into_iter()
        .map(|x| -> usize {
            let length = x.chars().count();
            is_unique_length(length).into()
        })
        .sum()
}

fn is_unique_length(length: usize) -> bool {
    let uniques = vec![2, 3, 4, 7];
    uniques.into_iter().any(|x| x == length)
}

fn line_to_digits(line: &String) -> (Vec<&str>, Vec<&str>) {
    let mut halves = line.split('|');
    let lhs = halves
        .next()
        .expect("Empty line!?");
    let rhs = halves
        .next()
        .expect("No \"|\" character in line. Malformed input.");
    (digit_str_to_digits(lhs), digit_str_to_digits(rhs)) 
}

fn digit_str_to_digits (tgt: &str) -> Vec<&str> {
    tgt.trim().split_whitespace().collect()
}
