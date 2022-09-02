use read_file::get_lines;
use std::collections::HashMap;


fn main() {
   let fname = std::env::args().nth(1).expect("No file name provided");
   let lines = get_lines(&fname);
   let map = make_point_map();
   for line in lines {
       println!("Hello");
   }
}

fn make_point_map() -> HashMap<char, usize> {
    HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)])
}
fn make_mate_map() -> HashMap<char, char> {
    HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')])
}

fn get_score(mate_map: HashMap<char, char>, point_map: HashMap<char, usize>, line: &str) -> usize {
    let mut stk: Vec<char> = Vec::new();
    for chr in line.chars() {
        if let Some(_) = mate_map.get(&chr){
            stk.push(chr);
        } else if let Some(points) = point_map.get(&chr) {
            println!("Closing bracket detected.");
        }
    }
    42
}
