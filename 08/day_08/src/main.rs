use day_08::{part_one, part_two};
fn main() {
    let in_file_name = std::env::args().nth(1).expect("No file name provided.");
    println!("Part One Answer: {}", part_one(&in_file_name));
    println!("Part Two Answer: {}", part_two(&in_file_name));
}

