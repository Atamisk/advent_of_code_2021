use read_file::get_lines;
use std::collections::HashMap;


fn main() {
   let fname = std::env::args().nth(1).expect("No file name provided");
   let lines = get_lines(&fname);
   let point_map = make_point_map();
   let mate_map = make_mate_map();

   let mut total: usize = 0;
   for line in lines {
       let line = line.unwrap();
       if let Some(points) =  get_score(&mate_map, &point_map, &line) {
           total += points;
       }
   }
   println!("{}", total);
}

fn make_point_map() -> HashMap<char, usize> {
    HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)])
}
fn make_mate_map() -> HashMap<char, char> {
    HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')])
}

fn get_score(mate_map: &HashMap<char, char>, point_map: &HashMap<char, usize>, line: &str) -> Option<usize> {
    let mut stk: Vec<char> = Vec::new();
    for chr in line.chars() {
        if let Some(_) = mate_map.get(&chr){
            stk.push(chr);
        } else if let Some(points) = point_map.get(&chr) {
            let mate = stk.pop()?;
            let correct_chr = *mate_map.get(&mate)?;
            if chr != correct_chr {
                return Some(*points);
            }
        }
    }
    None
}
