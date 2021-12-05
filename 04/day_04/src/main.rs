use read_file;

mod bingo_card;
use bingo_card::*;
fn main() {
    let path = std::env::args().nth(1).expect("No path argument given.");    
    let main_string = get_fstring(&path);
    let (instrs, cards) = parse_input(&main_string);
    let sol_1 = solve_1(instrs.clone(), cards.clone()).unwrap();
    let sol_2 = solve_2(instrs, cards).unwrap();
    println!("Part 1: {}", sol_1);
    println!("Part 2: {}", sol_2);
}

fn get_fstring(path: &str) -> String {
    read_file::read_to_string(path).expect("Error reading File")
}

fn parse_input(main_string:&str) -> (Vec<usize>, Vec<BingoCard>) {
    let mut spl = main_string.split("\n\n");

    let instructions: Vec<usize> = spl
        .next()
        .unwrap()
        .split(",")
        .map(|item| -> usize {
            item.parse().unwrap()
        })
        .collect();

    let cards:Vec<BingoCard> = spl
        .map(|card_str| {
            BingoCard::from_string(card_str)
        })
        .collect();
    (instructions, cards)
}

fn solve_1(instrs: Vec<usize>, mut cards: Vec<BingoCard>) -> Result<usize, &'static str>{
    for instr in instrs {
        for card in cards.iter_mut(){
            card.call_num(instr);
            if card.is_winner() {
                return Ok(instr * card.score())
            }
        }
    }
    Err("No Winner Found!")
}

fn solve_2(instrs: Vec<usize>, mut cards: Vec<BingoCard>) -> Result<usize, &'static str>{
    let mut last_score = 0;
    let mut winning_cards: Vec<usize> = Vec::new();
    for instr in instrs {
        for (i, card) in cards.iter_mut().enumerate(){
            card.call_num(instr);
            if card.is_winner() && !(winning_cards.iter().any(|x| *x == i)){
                last_score = instr * card.score();
                winning_cards.push(i);
            }
        }
    }
    if last_score == 0 {
        Err("No Winner Found!")
    }
    else{
        Ok(last_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sol_1() {
        let in_str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
                      \n\
                      22 13 17 11  0\n\
                       8  2 23  4 24\n\
                      21  9 14 16  7\n\
                       6 10  3 18  5\n\
                       1 12 20 15 19\n\
                      \n\
                       3 15  0  2 22\n\
                       9 18 13 17  5\n\
                      19  8  7 25 23\n\
                      20 11 10 24  4\n\
                      14 21 16 12  6\n\
                      \n\
                      14 21 17 24  4\n\
                      10 16 15  9 19\n\
                      18  8 23 26 20\n\
                      22 11 13  6  5\n\
                       2  0 12  3  7";
        let (instrs, cards) = parse_input(in_str);
        let sol_1 = solve_1(instrs, cards);
        assert_eq!(sol_1, Ok(4512));
    }
    #[test]
    fn test_sol_2() {
        let in_str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
                      \n\
                      22 13 17 11  0\n\
                       8  2 23  4 24\n\
                      21  9 14 16  7\n\
                       6 10  3 18  5\n\
                       1 12 20 15 19\n\
                      \n\
                       3 15  0  2 22\n\
                       9 18 13 17  5\n\
                      19  8  7 25 23\n\
                      20 11 10 24  4\n\
                      14 21 16 12  6\n\
                      \n\
                      14 21 17 24  4\n\
                      10 16 15  9 19\n\
                      18  8 23 26 20\n\
                      22 11 13  6  5\n\
                       2  0 12  3  7";
        let (instrs, cards) = parse_input(in_str);
        let sol_2 = solve_2(instrs, cards);
        assert_eq!(sol_2, Ok(1924));
    }
}
