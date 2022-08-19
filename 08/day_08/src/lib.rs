use read_file;
use std::collections::HashMap;

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
    let (lhs, rhs) = extract_halves(&line);
    let segment_freq = count_segment_freq(lhs);
    println!("{:?}", segment_freq);
    let first_segments = get_unique_segments(segment_freq);
    println!("{:?}", first_segments);
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

fn get_unique_segments(map: HashMap<char, usize>) -> (char, char, char) {
    let mut b = 'h';
    let mut e = 'h';
    let mut f = 'h';
    for (k, v) in map.iter(){
        match v {
            6 => {b = *k},
            4 => {e = *k},
            9 => {f = *k},
            _ => (),
        }
    }
    (b,e,f)
}

fn count_segment_freq(digits: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for x in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
        map.insert(x,0);
    }
    for chr in digits.chars() {
        match map.get_mut(&chr) {
            Some(e) => {*e += 1},
            None => (),
        }
    }
    map
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

fn digit_str_to_digits (tgt: &str) -> Vec<&str> {
    tgt.trim().split_whitespace().collect()
}
