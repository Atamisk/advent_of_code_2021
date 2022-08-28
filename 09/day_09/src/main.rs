use read_file;

mod full_gameboard;
use full_gameboard::*;

fn main() {
    let fname = std::env::args().nth(1).expect("No file name provided!");
    let file_lines = read_file::get_lines(&fname);
    let gboard = FullGameBoard::new(file_lines);
    let sm:usize = gboard.sum();
    println!("Part One Answer: {}", sm);
}
