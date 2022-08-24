use read_file;

mod lp_finder;
use lp_finder::*;

fn main() {
    let fname = std::env::args().nth(1).expect("No file name provided!");
    let file_lines = read_file::get_lines(&fname);
    let gboard = LPFinder::new(file_lines);
    let sm:usize = gboard.sum();
    println!("Part One Answer: {}", sm);
}
