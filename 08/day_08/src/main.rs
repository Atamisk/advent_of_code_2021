use read_file;
fn main() {
    let in_file_name = std::env::args().nth(1).expect("No file name provided.");
    let total_unique_digits: usize = read_file::get_lines(&in_file_name)
        .map(|line| process_line(&line.unwrap()))
        .sum();
    println!("{}", total_unique_digits);
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
