use read_file::get_lines;
use std::collections::HashMap;

fn main() {
    let fname = std::env::args().nth(1).expect("No file name provided");
    let lines = get_lines(&fname);
    let maps = Maps::new();
    let get_score_pre = |x: &str| -> LineIssue { get_score(&maps, x) };

    let mut incorrect_total: usize = 0;
    let mut incompletes: Vec<usize> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        match get_score_pre(&line) {
            LineIssue::Incorrect(points) => {
                incorrect_total += points;
            }
            LineIssue::Incomplete(points) => {
                incompletes.push(points);
            }
        }
    }
    println!("{}", incorrect_total);

    incompletes.sort();
    println!("{}", incompletes[incompletes.len() / 2]);
}

enum LineIssue {
    Incorrect(usize),
    Incomplete(usize),
}

struct Maps {
    incorrect: HashMap<char, usize>,
    incomplete: HashMap<char, usize>,
    mate: HashMap<char, char>,
}

impl Maps {
    fn new() -> Self {
        Self {
            incorrect: HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]),
            incomplete: HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]),
            mate: HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]),
        }
    }
}

fn get_score(maps: &Maps, line: &str) -> LineIssue {
    let mut stk: Vec<char> = Vec::new();
    for chr in line.chars() {
        if let Some(_) = maps.mate.get(&chr) {
            stk.push(chr);
        } else if let Some(points) = maps.incorrect.get(&chr) {
            let mate = stk.pop().unwrap();
            let correct_chr = maps.mate.get(&mate).unwrap();
            if chr != *correct_chr {
                return LineIssue::Incorrect(*points);
            }
        }
    }
    //If we get here, the line is incomplete.
    let mut incomplete_total = 0;
    while let Some(chr) = stk.pop() {
        incomplete_total *= 5;
        incomplete_total += maps
            .incomplete
            .get(&chr)
            .expect(&format!("Unexpected char {} in incomplete!", chr));
    }
    LineIssue::Incomplete(incomplete_total)
}
