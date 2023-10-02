use std::collections::HashSet;

fn main() {
    let contents = include_str!("../puzzle_input.txt");
    let commands = parse_commands(contents);

    println!("covered tail fields: {}", calculate_fields_covered_by_tail(&commands));
}

fn parse_commands(contents: &str) -> Vec<(i32, i32)>
{
    let mut commands = vec![];
    let lines = contents.lines().collect::<Vec<_>>();
    for l in lines {
        let (dir, count) = l.split_once(" ").unwrap();
        let (x, y) = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => (0, 0)
        };
        for _ in 0..count.parse::<i32>().unwrap() {
            commands.push((x, y));
        }
    }
    commands
}

struct Board {
    head: (i32, i32),
    tail: (i32, i32),
    tail_set: HashSet<(i32, i32)>,
}

fn update_tail(head: (i32, i32), old_tail: (i32, i32)) -> (i32, i32) {
    let dist_x = head.0 - old_tail.0;
    let dist_y = head.1 - old_tail.1;
    let mut ofs_x = 0;
    let mut ofs_y = 0;
    if dist_y <= -2 {
        ofs_y = -1;
        if dist_x != 0 {
            ofs_x = dist_x;
        }
    } else if dist_y >= 2 {
        ofs_y = 1;
        if dist_x != 0 {
            ofs_x = dist_x;
        }
    }

    if dist_x <= -2 {
        ofs_x = -1;
        if dist_y != 0 {
            ofs_y = dist_y;
        }
    }
    if dist_x >= 2 {
        ofs_x = 1;
        if dist_y != 0 {
            ofs_y = dist_y;
        }
    }


    let new_tail = (old_tail.0 + ofs_x, old_tail.1 + ofs_y);
    new_tail
}

fn update_board_single_step(board: &Board, command: (i32, i32)) -> Board {
    let new_head = (board.head.0 + command.0, board.head.1 + command.1);
    let new_tail = update_tail(new_head, board.tail);
    let mut new_tail_set = board.tail_set.clone();
    new_tail_set.insert(new_tail);
    Board { head: new_head, tail: new_tail, tail_set: new_tail_set }
}

fn update_board(board: &Board, commands: &Vec<(i32, i32)>) -> Board {
    let mut initial_tail_set = board.tail_set.clone();
    initial_tail_set.insert(board.tail);
    let mut updated_board: Board = Board {
        head: board.head,
        tail: board.tail,
        tail_set: initial_tail_set,
    };
    for c in commands {
        updated_board = update_board_single_step(&updated_board, c.clone());
    }
    updated_board
}

fn calculate_fields_covered_by_tail(commands: &Vec<(i32, i32)>) -> i32
{
    let board = Board {
        head: (0, 0),
        tail: (0, 0),
        tail_set: Default::default(),
    };
    let board_end = update_board(&board, &commands);
    board_end.tail_set.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn end_to_end_test_example() {
        let contents = include_str!("../example.txt");
        let commands = parse_commands(contents);
        let fields_covered_by_tail = calculate_fields_covered_by_tail(&commands);
        assert_eq!(fields_covered_by_tail, 13);
    }

    #[test]
    fn parse_single_command_r1() {
        let contents = "R 1";
        let commands = parse_commands(contents);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].0, 1);
        assert_eq!(commands[0].1, 0);
    }

    #[test]
    fn parse_single_command_l1() {
        let contents = "L 1";
        let commands = parse_commands(contents);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].0, -1);
        assert_eq!(commands[0].1, 0);
    }

    #[test]
    fn parse_single_command_u1() {
        let contents = "U 1";
        let commands = parse_commands(contents);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].0, 0);
        assert_eq!(commands[0].1, -1);
    }

    #[test]
    fn parse_single_command_d1() {
        let contents = "D 1";
        let commands = parse_commands(contents);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].0, 0);
        assert_eq!(commands[0].1, 1);
    }

    #[test]
    fn parse_single_command_d2() {
        let contents = "D 2";
        let commands = parse_commands(contents);
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].0, 0);
        assert_eq!(commands[0].1, 1);
        assert_eq!(commands[1].0, 0);
        assert_eq!(commands[1].1, 1);
    }


    #[test]
    fn update_board_single_step_moves_head_north() {
        let b = Board { head: (0, 0), tail: (0, 0), tail_set: HashSet::new() };
        let command = (0, -1);
        let b2 = update_board_single_step(&b, command);
        assert_eq!(b2.head.0, 0);
        assert_eq!(b2.head.1, -1);
    }

    #[test]
    fn update_board_single_step_moves_head_south() {
        let b = Board { head: (0, 0), tail: (0, 0), tail_set: HashSet::new() };
        let command = (0, 1);
        let b2 = update_board_single_step(&b, command);
        assert_eq!(b2.head.0, 0);
        assert_eq!(b2.head.1, 1);
    }

    #[test]
    fn update_board_single_step_moves_head_east() {
        let b = Board { head: (0, 0), tail: (0, 0), tail_set: HashSet::new() };
        let command = (1, 0);
        let b2 = update_board_single_step(&b, command);
        assert_eq!(b2.head.0, 1);
        assert_eq!(b2.head.1, 0);
    }

    #[test]
    fn update_board_single_step_moves_head_west() {
        let b = Board { head: (0, 0), tail: (0, 0), tail_set: HashSet::new() };
        let command = (-1, 0);
        let b2 = update_board_single_step(&b, command);
        assert_eq!(b2.head.0, -1);
        assert_eq!(b2.head.1, 0);
    }

    #[test]
    fn update_board_example_moves_head_correctly() {
        let contents = include_str!("../example.txt");
        let commands = parse_commands(contents);

        let b = Board { head: (0, 0), tail: (0, 0), tail_set: HashSet::new() };
        let b_end = update_board(&b, &commands);

        assert_eq!(b_end.head.0, 2);
        assert_eq!(b_end.head.1, -2);
    }

    #[test]
    fn update_tail_on_same_pos() {
        let head = (0, 0);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_north_does_not_move_tail() {
        let head = (0, -1);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_northeast_does_not_move_tail() {
        let head = (1, -1);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_east_does_not_move_tail() {
        let head = (1, 0);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_southeast_does_not_move_tail() {
        let head = (1, 1);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_south_does_not_move_tail() {
        let head = (0, 1);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_southwest_does_not_move_tail() {
        let head = (-1, 1);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_west_does_not_move_tail() {
        let head = (-1, 0);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_northwest_does_not_move_tail() {
        let head = (-1, -1);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_two_north_does_move_tail() {
        let head = (0, -2);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, -1);
    }

    #[test]
    fn update_tail_two_south_does_move_tail() {
        let head = (0, 2);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 1);
    }

    #[test]
    fn update_tail_two_east_does_move_tail() {
        let head = (2, 0);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 1);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_two_west_does_move_tail() {
        let head = (-2, 0);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, -1);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_two_north_one_east() {
        let head = (1, -2);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 1);
        assert_eq!(new_tail.1, -1);
    }

    #[test]
    fn update_tail_two_south_one_east() {
        let head = (1, 2);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 1);
        assert_eq!(new_tail.1, 1);
    }

    #[test]
    fn update_tail_two_north_one_west() {
        let head = (-1, -2);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, -1);
        assert_eq!(new_tail.1, -1);
    }

    #[test]
    fn update_tail_one_south_two_east() {
        let head = (2, 1);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, 1);
        assert_eq!(new_tail.1, 1);
    }

    #[test]
    fn update_tail_one_north_two_west() {
        let head = (-2, -1);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, -1);
        assert_eq!(new_tail.1, -1);
    }

    #[test]
    fn update_tail_two_south_one_west() {
        let head = (-1, 2);
        let tail = (0, 0);
        let new_tail = update_tail(head, tail);

        assert_eq!(new_tail.0, -1);
        assert_eq!(new_tail.1, 1);
    }
}