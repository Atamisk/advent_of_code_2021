use read_file;

mod full_gameboard;
use full_gameboard::*;

fn main() {
    let fname = std::env::args().nth(1).expect("No file name provided!");
    let file_lines = read_file::get_lines(&fname);
    let gboard = FullGameBoard::new(file_lines);
    let sm = gboard.find_all_low_points();
    let mut basins = sm.basins();
    basins.sort_unstable();
    basins.reverse();
    let basin_prod = basins.into_iter().take(3).reduce(|acc, x| acc * x).unwrap();
    println!("Part One Answer: {}", sm.risk());
    println!("Part Two Answer: {}", basin_prod);
}
