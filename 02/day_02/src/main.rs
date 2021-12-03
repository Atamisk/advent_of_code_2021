use read_file;
fn main() {
    let path = std::env::args().nth(1).expect("No path argument provided");
    let items = parse_file(&path);
    let instructions = parse_instructions(items);
    let soln_1 = solve_1(instructions.clone());
    let soln_2 = solve_2(instructions);
    println!("Part 1: {}", soln_1);
    println!("Part 2: {}", soln_2);

}

fn parse_file(path: &str) -> Vec<String> {
    read_file::get_lines(path)
        .map(|x| { x.unwrap() })
        .collect()
}

fn parse_instructions(items: Vec<String>) -> Vec<MoveInstruction> {
    items
        .into_iter()
        .map(|line: String| -> MoveInstruction {
            let mut kvp = line.split(" ");
            let key = kvp.next().unwrap();
            let k = key.chars().next().unwrap();
            let value: usize = kvp
                .next()
                .unwrap()
                .parse()
                .unwrap();
            match k {
                'f' => MoveInstruction::Fw(value),
                'd' => MoveInstruction::Dn(value),
                'u' => MoveInstruction::Up(value),
                a   => panic!("Unknown Command: {}", a)
            }
        })
        .collect()
}

fn solve_1(instructions: Vec<MoveInstruction>) -> usize{
    let mut sub = Submarine::new();
    
    for instr in instructions{
        sub.move_directly(instr);
    }

    sub.answer()
}

fn solve_2(instructions: Vec<MoveInstruction>) -> usize{
    let mut sub = Submarine::new();

    for instr in instructions{
        sub.move_aim(instr)
    }

    sub.answer()
}

#[derive(Debug, Clone, Copy)]
enum MoveInstruction {
    Up(usize),
    Dn(usize),
    Fw(usize)
}

struct Submarine {
    aim: usize,
    depth: usize,
    hor_pos: usize,
}

impl Submarine {
    fn new() -> Self {
        Self{
            aim: 0,
            depth:   0,
            hor_pos: 0
        }
    }
    fn move_directly(&mut self, dir: MoveInstruction){
        match dir {
            MoveInstruction::Up(a) => self.depth -= a,
            MoveInstruction::Dn(a) => self.depth += a,
            MoveInstruction::Fw(a) => self.hor_pos += a,
        }
    }
    fn move_aim(&mut self, dir: MoveInstruction){
        match dir {
            MoveInstruction::Up(a) => self.aim -= a,
            MoveInstruction::Dn(a) => self.aim += a,
            MoveInstruction::Fw(a) => {
                self.hor_pos += a;
                self.depth   += a * self.aim;
            }
        }
    }
    fn answer(&self) -> usize {
        self.depth * self.hor_pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_1(){
        let items: Vec<String> = vec![  "forward 5".to_string()
                                      , "down 5".to_string()
                                      , "forward 8".to_string()
                                      , "up 3".to_string()
                                      , "down 8".to_string()
                                      , "forward 2".to_string()];
        let instructions = parse_instructions(items);
        assert_eq!(solve_1(instructions), 150);
    }
    #[test]
    fn test_solve_2(){
        let items: Vec<String> = vec![  "forward 5".to_string()
                                      , "down 5".to_string()
                                      , "forward 8".to_string()
                                      , "up 3".to_string()
                                      , "down 8".to_string()
                                      , "forward 2".to_string()];
        let instructions = parse_instructions(items);
        assert_eq!(solve_2(instructions), 900);
    }
}
