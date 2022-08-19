use read_file;
use std::collections::{HashMap, HashSet};

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
    let translator = get_translator(lhs, rhs);
    42
}

fn get_translator(lhs: &str, rhs: &str) -> HashMap<char, char>{
    let mut translator: HashMap<char, char> = HashMap::new();

    //determine first three uniquely identifiable translations. 
    let segment_freq = count_segment_freq(lhs);
    let (tru_b, tru_e, tru_f) = get_unique_segments(&segment_freq);
    translator.insert(tru_b, 'b');
    translator.insert(tru_e, 'e');
    translator.insert(tru_f, 'f');
    
    //determine letters based on unique numbers
    let (one, four) = get_unique_digits(rhs);
    let tru_c = deduce_c(one, &tru_f);
    let tru_a = deduce_a(&segment_freq, &tru_c);
    let tru_d = deduce_d(four, &tru_b, &tru_c, &tru_f);
    translator.insert(tru_c, 'c');
    translator.insert(tru_a, 'a');
    translator.insert(tru_d, 'd');

    //determine final unknown letter. 
    let tru_g = deduce_g(&translator);
    translator.insert(tru_g, 'g');
    translator
}

fn get_unique_segments(map: &HashMap<char, usize>) -> (char, char, char) {
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

fn deduce_c(one: &str, tru_f: &char) -> char {
    one.chars().find(|chr| chr != tru_f).unwrap()
}

fn deduce_a(segs: &HashMap<char, usize>, tru_c: &char) -> char{
    let mut sc = segs.iter().filter(|&(k,&v)| v == 8 && k != tru_c);
    *sc.next().unwrap().0
}

fn deduce_d(four: &str, tru_b: &char, tru_c: &char, tru_f: &char) -> char {
    let mut set: HashSet<char> = four.chars().collect();
    set.remove(tru_b);
    set.remove(tru_c);
    set.remove(tru_f);
    set.into_iter().next().unwrap()
}

fn deduce_g(translator: &HashMap<char, char>) -> char {
    let fullset: HashSet<char> = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into();
    let known: HashSet<char> = translator.keys().map(|x| *x).collect();
    *fullset.difference(&known).next().unwrap()
}
