use read_file;

pub fn part_one(fname: &String) -> usize {
    read_file::get_lines(fname)
        .map(|line| process_line(&line.unwrap()))
        .sum()
}

pub fn part_two(fname: &String) -> usize {
    42
}

fn process_line(line: &String) -> usize {
    line_to_digits(line)
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

fn line_to_digits(line: &String) -> Vec<&str> {
    let rhs = line
        .split('|')
        .nth(1)
        .expect("No \"|\" character in line. Malformed input.");
    rhs.trim().split_whitespace().collect()
}
