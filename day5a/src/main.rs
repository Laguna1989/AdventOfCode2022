use std::collections::{HashMap, VecDeque};

fn main() {
    let contents = include_str!("../puzzle_input.txt");
    let mut stack = Stacks::new(contents);
    stack.apply_moves();
    println!("{}", stack.get_top_crates());
}

struct Move {
    src: usize,
    dest: usize,
    count: i32,
}

struct Stacks {
    crates: HashMap<i32, VecDeque<char>>,
    moves: Vec<Move>,
}

fn line_to_stack(stacks: &mut HashMap<i32, VecDeque<char>>, line: &str) {
    if !line.contains("[") {
        return;
    }
    let chars = line.replace("    ", "[_] ").replace(['[', ' ', ']'], "");
    for (pos, c) in chars.chars().enumerate() {
        if !stacks.contains_key(&(pos as i32))
        {
            // fill empty VecDeque if key not present in hashmap
            stacks.insert(pos as i32, VecDeque::from([]));
        }
        if c == '_' {
            continue;
        }
        stacks.entry(pos as i32).and_modify(|s| s.push_front(c));
    }
}

fn line_to_move(line: &str) -> Move {
    let numbers_as_str: Vec<i32> = line.replace("move ", "").replace("from ", "").replace("to ", "").split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
    Move { src: numbers_as_str[1] as usize, dest: numbers_as_str[2] as usize, count: numbers_as_str[0] }
}

impl Stacks {
    fn new(input: &str) -> Self {
        let mut stacks = HashMap::new();
        let (crates_input, moves_input) = input.split_once("\n\n").unwrap();
        let lines = crates_input.lines().collect::<Vec<_>>();

        lines.iter().for_each(|l|
            line_to_stack(&mut stacks, l)
        );

        let lines = moves_input.lines().collect::<Vec<_>>();
        let mut moves = vec![];
        lines.iter().for_each(|l|
            moves.push(line_to_move(l)));
        Self { crates: stacks, moves: moves }
    }

    fn apply_moves(&mut self) -> () {
        for m in &self.moves {
            for _ in 0..m.count {
                let mut c = ' ';
                self.crates.entry(m.src as i32 - 1).and_modify(|stack| c = stack.pop_back().unwrap());
                self.crates.entry(m.dest as i32 - 1).and_modify(|stack| stack.push_back(c));
            }
        }
    }

    fn get_top_crates(&self) -> String {
        let mut a_str = String::from("");

        for i in 0..self.crates.len() {
            a_str.push(*self.crates.get(&(i as i32)).unwrap().back().unwrap());
        }

        a_str
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string_leads_to_empty_stacks() {
        let contents = "\n\n";

        let stack = Stacks::new(contents);

        assert_eq!(stack.crates.len(), 0)
    }

    #[test]
    fn parse_one_stack_with_one_crate() {
        let contents = "[A]\n\n";

        let stack = Stacks::new(contents);

        assert_eq!(stack.crates.len(), 1);
        assert_eq!(stack.crates.get(&0).unwrap().len(), 1);
        assert_eq!(stack.crates.get(&0).unwrap()[0], 'A');
    }

    #[test]
    fn parse_two_stack_with_one_crate_each() {
        let contents = "[A] [B]\n\n";

        let stack = Stacks::new(contents);

        assert_eq!(stack.crates.len(), 2);
        assert_eq!(stack.crates.get(&0).unwrap().len(), 1);
        assert_eq!(stack.crates.get(&0).unwrap()[0], 'A');
        assert_eq!(stack.crates.get(&1).unwrap().len(), 1);
        assert_eq!(stack.crates.get(&1).unwrap()[0], 'B');
    }

    #[test]
    fn parse_two_stack_with_one_crate_in_second_stack() {
        let contents = "    [B]\n\n";

        let stack = Stacks::new(contents);

        assert_eq!(stack.crates.len(), 2);
        assert_eq!(stack.crates.get(&0).unwrap().len(), 0);
        assert_eq!(stack.crates.get(&1).unwrap().len(), 1);
        assert_eq!(stack.crates.get(&1).unwrap()[0], 'B');
    }

    #[test]
    fn parse_two_stack_with_two_crates_in_first_stack() {
        let contents = "[B]    \n[A]    \n\n";

        let stack = Stacks::new(contents);

        assert_eq!(stack.crates.len(), 2);
        assert_eq!(stack.crates.get(&0).unwrap().len(), 2);
        assert_eq!(stack.crates.get(&1).unwrap().len(), 0);
        assert_eq!(stack.crates.get(&0).unwrap()[1], 'B');
        assert_eq!(stack.crates.get(&0).unwrap()[0], 'A');
    }

    #[test]
    fn parse_two_stack_with_two_crates_in_both_stacks() {
        let contents = "[A] [B]\n[C] [D]\n\n";

        let stack = Stacks::new(contents);

        assert_eq!(stack.crates.len(), 2);
        assert_eq!(stack.crates.get(&0).unwrap().len(), 2);
        assert_eq!(stack.crates.get(&1).unwrap().len(), 2);

        assert_eq!(stack.crates.get(&0).unwrap()[1], 'A');
        assert_eq!(stack.crates.get(&1).unwrap()[1], 'B');
        assert_eq!(stack.crates.get(&0).unwrap()[0], 'C');
        assert_eq!(stack.crates.get(&1).unwrap()[0], 'D');
    }

    #[test]
    fn parse_two_stack_with_two_crates_in_second_stack() {
        let contents = "    [B]\n    [A]\n\n";

        let stack = Stacks::new(contents);

        assert_eq!(stack.crates.len(), 2);
        assert_eq!(stack.crates.get(&0).unwrap().len(), 0);
        assert_eq!(stack.crates.get(&1).unwrap().len(), 2);
        assert_eq!(stack.crates.get(&1).unwrap()[1], 'B');
        assert_eq!(stack.crates.get(&1).unwrap()[0], 'A');
    }

    #[test]
    fn apply_simple_move() {
        let contents = "[A] [B]\n\nmove 1 from 1 to 2";

        let mut stack = Stacks::new(contents);

        stack.apply_moves();

        assert_eq!(stack.crates.len(), 2);
        assert_eq!(stack.crates.get(&0).unwrap().len(), 0);
        assert_eq!(stack.crates.get(&1).unwrap().len(), 2);

        assert_eq!(stack.crates.get(&1).unwrap()[0], 'B');
        assert_eq!(stack.crates.get(&1).unwrap()[1], 'A');
    }

    #[test]
    fn apply_move_of_two_entries() {
        let contents = "[A]\n[A] [B]\n\nmove 2 from 1 to 2";

        let mut stack = Stacks::new(contents);

        stack.apply_moves();

        assert_eq!(stack.crates.len(), 2);
        assert_eq!(stack.crates.get(&0).unwrap().len(), 0);
        assert_eq!(stack.crates.get(&1).unwrap().len(), 3);

        assert_eq!(stack.crates.get(&1).unwrap()[0], 'B');
        assert_eq!(stack.crates.get(&1).unwrap()[1], 'A');
        assert_eq!(stack.crates.get(&1).unwrap()[2], 'A');
    }

    #[test]
    fn parse_crates_from_example_input() {
        let contents = include_str!("../example.txt");

        let mut stack = Stacks::new(contents);
        assert_eq!(stack.crates.len(), 3);
        assert_eq!(stack.crates.get(&0).unwrap().len(), 2);
        assert_eq!(stack.crates.get(&0).unwrap()[0], 'Z');
        assert_eq!(stack.crates.get(&0).unwrap()[1], 'N');

        assert_eq!(stack.crates.get(&1).unwrap().len(), 3);
        assert_eq!(stack.crates.get(&1).unwrap()[0], 'M');
        assert_eq!(stack.crates.get(&1).unwrap()[1], 'C');
        assert_eq!(stack.crates.get(&1).unwrap()[2], 'D');

        assert_eq!(stack.crates.get(&2).unwrap().len(), 1);
        assert_eq!(stack.crates.get(&2).unwrap()[0], 'P');


        assert_eq!(stack.moves.len(), 4);
        assert_eq!(stack.moves[0].src, 2);
        assert_eq!(stack.moves[0].dest, 1);
        assert_eq!(stack.moves[0].count, 1);

        assert_eq!(stack.moves[1].src, 1);
        assert_eq!(stack.moves[1].dest, 3);
        assert_eq!(stack.moves[1].count, 3);

        assert_eq!(stack.moves[2].src, 2);
        assert_eq!(stack.moves[2].dest, 1);
        assert_eq!(stack.moves[2].count, 2);

        assert_eq!(stack.moves[3].src, 1);
        assert_eq!(stack.moves[3].dest, 2);
        assert_eq!(stack.moves[3].count, 1);

        stack.apply_moves();

        assert_eq!(stack.crates.len(), 3);
        assert_eq!(stack.crates.get(&0).unwrap().len(), 1);
        assert_eq!(stack.crates.get(&1).unwrap().len(), 1);
        assert_eq!(stack.crates.get(&2).unwrap().len(), 4);
    }

    #[test]
    fn parse_crates_from_puzzle_input() {
        let contents = include_str!("../puzzle_input.txt");

        let stack = Stacks::new(contents);
        assert_eq!(stack.crates.len(), 9);
    }

    #[test]
    fn parse_crates_long_line() {
        let contents = "            [J]             [B] [W]\n\n";
        let stack = Stacks::new(contents);
        assert_eq!(stack.crates.len(), 9);
    }
}