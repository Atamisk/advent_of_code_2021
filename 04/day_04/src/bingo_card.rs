use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct BingoCard{
    board: Vec<Vec<usize>>,
    called: HashSet<BoardIdx>,
    score: usize
}

impl BingoCard {
    pub fn from_string(s: &str) -> Self {
        let board: Vec<Vec<usize>> = s
            .split("\n")
            .map( |line: &str| -> Vec<usize> {
                line
                    .trim()
                    .split_whitespace()
                    .map(|num_str: &str| -> usize {
                        match num_str.parse() {
                            Ok(a) => a,
                            Err(_) => panic!("Line with Error: \n\
                                              {:?}\n\
                                              Offending entry: \n\
                                              {:?}", line, num_str) 
                        }

                    })
                    .collect()
            })
            .collect();
        let score = board
            .iter()
            .map(|row| -> usize {
                row.iter().sum()
            })
            .sum();
        Self{
            board, 
            called: HashSet::new(),
            score
        }

    }

    #[cfg(test)]
    fn check_board_dim(&self){
        let dim = self.board.len();
        for x in self.board.iter() {
            assert_eq!(x.len(), dim);
        }
    }

    #[cfg(test)]
    fn get(&self, idx: BoardIdx) -> Option<&usize>{
        let row = self.board.get(idx.0)?;
        let out = row.get(idx.1)?;
        Some(out)
    }

    pub fn is_winner(&self) -> bool {
        let board_dim = self.board.len();
        let mut row_totals: Vec<usize> = vec![0; board_dim];
        let mut col_totals: Vec<usize> = vec![0; board_dim];
        for item in self.called.iter() {
            row_totals[item.0] += 1;
            col_totals[item.1] += 1;
        }
        if row_totals.iter().any(|x| *x == 5) ||
            col_totals.iter().any(|x| *x == 5)
        {
            true
        }
        else {
            false
        }
    }
    pub fn call_num(&mut self, num: usize){
        for (i, row) in self.board.iter().enumerate(){
            for (j, entry) in row.iter().enumerate(){
                if *entry == num{
                    if self.called.insert(BoardIdx(i,j)){
                        self.score -= num;
                    }
                    break;
                }
            }
        }
    }
    pub fn score(&self) -> usize {
        self.score
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct BoardIdx(usize, usize);

#[cfg(test)]
mod tests {
    use super::*;
    fn get_test_card() -> BingoCard {
        let card_string = "14 21 17 24  4\n\
                           10 16 15  9 19\n\
                           18  8 23 26 20\n\
                           22 11 13  6  5\n\
                            2  0 12  3  7";
        BingoCard::from_string(card_string)
    }
    #[test]
    fn test_bingo() {
        let card = get_test_card();
        assert_eq!(card.get(BoardIdx(4,4)), Some(&7));
        card.check_board_dim();
    }
    #[test]
    fn test_row_winner(){
        let mut card = get_test_card();
        card.call_num(14);
        card.call_num(21);
        card.call_num(17);
        card.call_num(24);
        card.call_num(4);
        assert!(card.is_winner());
        assert_eq!(card.score(), 245);
    }
    #[test]
    fn test_col_winner(){
        let mut card = get_test_card();
        card.call_num(14);
        card.call_num(10);
        card.call_num(2);
        card.call_num(22);
        card.call_num(18);
        card.call_num(18);
        card.call_num(9);
        assert!(card.is_winner());
        assert_eq!(card.score(), 250);
    }
}
