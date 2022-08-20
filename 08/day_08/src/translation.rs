use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

pub fn translate(translator: HashMap<char, char>, rhs: &str) -> usize{
    let digits = digit_str_to_digits(rhs);
    digits
        .iter()
        .enumerate()
        .map(|(i, digit)| translate_digit(&translator, digit) * 10_usize.pow((3-i).try_into().unwrap()))
        .sum()
}

pub fn get_translator(lhs: &str) -> HashMap<char, char>{
    let mut translator: HashMap<char, char> = HashMap::new();

    //determine first three uniquely identifiable translations. 
    let segment_freq = count_segment_freq(lhs);
    let (tru_b, tru_e, tru_f) = get_unique_segments(&segment_freq);
    translator.insert(tru_b, 'b');
    translator.insert(tru_e, 'e');
    translator.insert(tru_f, 'f');
    
    //determine letters based on unique numbers
    let (one, four) = get_unique_digits(lhs);
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

pub fn digit_str_to_digits (tgt: &str) -> Vec<&str> {
    tgt.trim().split_whitespace().collect()
}

fn translate_digit(translator: &HashMap<char, char>, digit: &str) -> usize{
    let right_digits: HashMap<&str, usize> = [
        ("abcefg" ,  0), 
        ("cf"     ,  1), 
        ("acdeg"  ,  2),
        ("acdfg"  ,  3),
        ("bcdf"   ,  4),
        ("abdfg"  ,  5),
        ("abdefg" ,  6),
        ("acf"    ,  7),
        ("abcdefg",  8),
        ("abcdfg" ,  9) 
    ].into();
    let mut translated_digit: Vec<char> = digit
        .chars()
        .map(|x| *translator.get(&x).unwrap())
        .collect();
    translated_digit.sort();
    let trans_digit_str = translated_digit.iter().collect::<String>();
    let digit_out = *right_digits.get(&trans_digit_str[..]).unwrap();
    digit_out
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
