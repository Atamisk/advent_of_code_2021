use read_file;

mod translation;
use translation::*;

/// Runs part one functionality
pub fn part_one(fname: &String) -> usize {
    read_file::get_lines(fname)
        .map(|line| process_line_p1(&line.unwrap()))
        .sum()
}

/// Handles part two functionality
pub fn part_two(fname: &String) -> usize {
    let mut lines = read_file::get_lines(fname);
    //testing code
    let line = lines.next().unwrap().unwrap();
    process_line_p2(&line);
    //end of testing code
    42
}

fn process_line_p1(line: &String) -> usize {
    digit_str_to_digits(extract_halves(line).1)
        .into_iter()
        .map(|x| -> usize {
            let length = x.chars().count();
            is_unique_length(length).into()
        })
        .sum()
}

fn process_line_p2(line: &String) -> usize {
    let (lhs, rhs) = extract_halves(&line);
    let translator = get_translator(lhs);
    42
}



fn extract_halves(line: &String) -> (&str, &str) {
    let mut halves = line.split('|');
    let lhs = halves
        .next()
        .expect("Empty line!?");
    let rhs = halves
        .next()
        .expect("No \"|\" character in line. Malformed input.");
    (lhs, rhs)
}

fn is_unique_length(length: usize) -> bool {
    let uniques = vec![2, 3, 4, 7];
    uniques.into_iter().any(|x| x == length)
}


