use read_file;

mod game_board;
use game_board::*;

fn main() {
    let fname = std::env::args().nth(1).expect("No file name provided!");
    let file_lines = read_file::get_lines(&fname);
    let gboard = GameBoard::new(file_lines);
    for x in gboard {
        println!("{}\n\n", x);
    }
}
