use read_file;

mod game_board;
use game_board::*;

fn main() {
    let fname = std::env::args().nth(1).expect("No file name provided!");
    let file_lines = read_file::get_lines(&fname);
    let mut gboard = GameBoard::new(file_lines);
    loop {
        println!("Line ID: {:?}\n\tLast Line: {:?}\n\tThis Line {:?}\n\tNext Line {:?}\n\n", gboard.id(), gboard.last_line(), gboard.this_line(), gboard.next_line());
        gboard.increment();
    }
}
