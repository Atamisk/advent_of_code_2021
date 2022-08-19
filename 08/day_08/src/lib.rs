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
    let mut translator: HashMap<char, char> = HashMap::new();
    let (lhs, rhs) = extract_halves(&line);

    //determine first three uniquely identifiable translations. 
    let segment_freq = count_segment_freq(lhs);
    let (tru_b, tru_e, tru_f) = get_unique_segments(segment_freq);
    translator.insert('b', tru_b);
    translator.insert('e', tru_e);
    translator.insert('f', tru_f);
    
    //determine letters based on unique numbers
    let (one, four) = get_unique_digits(rhs);
    println!("({}, {})", one, four);
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

fn get_unique_digits(rhs: &str) -> (&str, &str) {
    let digits = digit_str_to_digits(rhs);
    let mut one = "";
    let mut four = "";
    for digit in digits{
        let length = digit.chars().count();
        if length == 2 {
            one = digit;
        } else if length == 4 {
            four = digit
        }
    }
    (one, four)
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
