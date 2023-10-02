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
    knots: Vec<(i32, i32)>,
    tail_set: HashSet<(i32, i32)>,
}

fn update_single_tail(head: (i32, i32), old_tail: (i32, i32)) -> (i32, i32) {
    let dist_x = head.0 - old_tail.0;
    let dist_y = head.1 - old_tail.1;
    let mut ofs_x = 0;
    let mut ofs_y = 0;

    let dist = ((dist_x as f32) * (dist_x as f32) + (dist_y as f32) * (dist_y as f32)).sqrt();
    let touching = dist < 1.5;

    if dist_x != 0 && dist_y != 0 && (!touching) {
        let ofs_x = if dist_x > 0 { 1 } else { -1 };
        let ofs_y = if dist_y > 0 { 1 } else { -1 };
        return (old_tail.0 + ofs_x, old_tail.1 + ofs_y);
    }

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

fn apply_command_to_board(board: &Board, command: (i32, i32)) -> Board {
    let mut new_knots = board.knots.clone();
    new_knots[0].0 += command.0;
    new_knots[0].1 += command.1;
    Board { knots: new_knots, tail_set: board.tail_set.clone() }
}

fn update_all_tails(board: &Board) -> Board {
    let mut new_knots = board.knots.clone();

    for i in 1..board.knots.len()
    {
        new_knots[i] = update_single_tail(new_knots[i - 1], new_knots[i]);
    }

    let mut new_tail_set = board.tail_set.clone();
    new_tail_set.insert(new_knots.last().unwrap().clone());
    Board { knots: new_knots, tail_set: new_tail_set }
}

fn update_board(board: &Board, commands: &Vec<(i32, i32)>) -> Board {
    let mut initial_tail_set = board.tail_set.clone();
    initial_tail_set.insert(board.knots.last().unwrap().clone());
    let mut updated_board: Board =
        Board {
            knots: board.knots.clone(),
            tail_set: initial_tail_set.clone(),
        };
    for c in commands {
        updated_board = apply_command_to_board(&updated_board, c.clone());
        updated_board = update_all_tails(&updated_board)
    }
    Board {
        knots: updated_board.knots,
        tail_set: updated_board.tail_set.clone(),
    }
}

fn calculate_fields_covered_by_tail(commands: &Vec<(i32, i32)>) -> i32
{
    let board = Board {
        knots: vec![
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ],
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
        assert_eq!(fields_covered_by_tail, 1);
    }

    #[test]
    fn end_to_end_test_example2_full() {
        let contents = include_str!("../example2.txt");
        let commands = parse_commands(contents);
        let fields_covered_by_tail = calculate_fields_covered_by_tail(&commands);
        assert_eq!(fields_covered_by_tail, 36);
    }

    #[test]
    fn end_to_end_test_example2_part1() {
        let contents = "R 5";
        let commands = parse_commands(contents);
        let fields_covered_by_tail = calculate_fields_covered_by_tail(&commands);
        assert_eq!(fields_covered_by_tail, 1);
    }

    #[test]
    fn end_to_end_test_example2_part2_fields_covered() {
        let contents = "R 5\nU 8";
        let commands = parse_commands(contents);
        let fields_covered_by_tail = calculate_fields_covered_by_tail(&commands);
        assert_eq!(fields_covered_by_tail, 1);
    }

    #[test]
    fn end_to_end_test_example2_part1_positions() {
        let contents = "R 5";
        let commands = parse_commands(contents);

        let board = Board {
            knots: vec![
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
            ],
            tail_set: Default::default(),
        };
        let board_end = update_board(&board, &commands);
        assert_eq!(board_end.knots[0].0, 5);
        assert_eq!(board_end.knots[0].1, 0);

        assert_eq!(board_end.knots[1].0, 4);
        assert_eq!(board_end.knots[2].0, 3);
        assert_eq!(board_end.knots[3].0, 2);
        assert_eq!(board_end.knots[4].0, 1);
        assert_eq!(board_end.knots[5].0, 0);
        assert_eq!(board_end.knots[6].0, 0);
        assert_eq!(board_end.knots[7].0, 0);
        assert_eq!(board_end.knots[8].0, 0);
        assert_eq!(board_end.knots[9].0, 0);
    }

    #[test]
    fn end_to_end_test_example2_part2_positions() {
        let contents = "R 5\nU 8";
        let commands = parse_commands(contents);

        let board = Board {
            knots: vec![
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
            ],
            tail_set: Default::default(),
        };
        let board_end = update_board(&board, &commands);
        assert_eq!(board_end.knots[0].0, 5);
        assert_eq!(board_end.knots[0].1, -8);

        assert_eq!(board_end.knots[1].0, 5);
        assert_eq!(board_end.knots[1].1, -7);

        assert_eq!(board_end.knots[2].0, 5);
        assert_eq!(board_end.knots[2].1, -6);

        assert_eq!(board_end.knots[3].0, 5);
        assert_eq!(board_end.knots[3].1, -5);

        assert_eq!(board_end.knots[4].0, 5);
        assert_eq!(board_end.knots[4].1, -4);

        assert_eq!(board_end.knots[5].0, 4);
        assert_eq!(board_end.knots[5].1, -4);

        assert_eq!(board_end.knots[6].0, 3);
        assert_eq!(board_end.knots[6].1, -3);

        assert_eq!(board_end.knots[7].0, 2);
        assert_eq!(board_end.knots[7].1, -2);

        assert_eq!(board_end.knots[8].0, 1);
        assert_eq!(board_end.knots[8].1, -1);

        assert_eq!(board_end.knots[9].0, 0);
        assert_eq!(board_end.knots[9].1, 0);
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
        let b = Board { knots: vec![(0, 0)], tail_set: HashSet::new() };
        let command = (0, -1);
        let b2 = apply_command_to_board(&b, command);
        assert_eq!(b2.knots[0].0, 0);
        assert_eq!(b2.knots[0].1, -1);
    }

    #[test]
    fn update_board_single_step_moves_head_south() {
        let b = Board { knots: vec![(0, 0)], tail_set: HashSet::new() };
        let command = (0, 1);
        let b2 = apply_command_to_board(&b, command);
        assert_eq!(b2.knots[0].0, 0);
        assert_eq!(b2.knots[0].1, 1);
    }

    #[test]
    fn update_board_single_step_moves_head_east() {
        let b = Board { knots: vec![(0, 0)], tail_set: HashSet::new() };
        let command = (1, 0);
        let b2 = apply_command_to_board(&b, command);
        assert_eq!(b2.knots[0].0, 1);
        assert_eq!(b2.knots[0].1, 0);
    }

    #[test]
    fn update_board_single_step_moves_head_west() {
        let b = Board { knots: vec![(0, 0)], tail_set: HashSet::new() };
        let command = (-1, 0);
        let b2 = apply_command_to_board(&b, command);
        assert_eq!(b2.knots[0].0, -1);
        assert_eq!(b2.knots[0].1, 0);
    }

    #[test]
    fn update_board_example_moves_head_correctly() {
        let contents = include_str!("../example.txt");
        let commands = parse_commands(contents);

        let b = Board { knots: vec![(0, 0)], tail_set: HashSet::new() };
        let b_end = update_board(&b, &commands);

        assert_eq!(b_end.knots[0].0, 2);
        assert_eq!(b_end.knots[0].1, -2);
    }

    #[test]
    fn update_board_example_moves_tail_correctly() {
        let contents = include_str!("../example.txt");
        let commands = parse_commands(contents);

        let b = Board { knots: vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)], tail_set: HashSet::new() };
        let b_end = update_board(&b, &commands);

        assert_eq!(b_end.knots.last().unwrap().0, 0);
        assert_eq!(b_end.knots.last().unwrap().1, 0);
    }

    #[test]
    fn update_tail_on_same_pos() {
        let head = (0, 0);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_north_does_not_move_tail() {
        let head = (0, -1);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_northeast_does_not_move_tail() {
        let head = (1, -1);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_east_does_not_move_tail() {
        let head = (1, 0);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_southeast_does_not_move_tail() {
        let head = (1, 1);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_south_does_not_move_tail() {
        let head = (0, 1);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_southwest_does_not_move_tail() {
        let head = (-1, 1);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_west_does_not_move_tail() {
        let head = (-1, 0);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_one_northwest_does_not_move_tail() {
        let head = (-1, -1);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_two_north_does_move_tail() {
        let head = (0, -2);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, -1);
    }

    #[test]
    fn update_tail_two_south_does_move_tail() {
        let head = (0, 2);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 0);
        assert_eq!(new_tail.1, 1);
    }

    #[test]
    fn update_tail_two_east_does_move_tail() {
        let head = (2, 0);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 1);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_two_west_does_move_tail() {
        let head = (-2, 0);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, -1);
        assert_eq!(new_tail.1, 0);
    }

    #[test]
    fn update_tail_two_north_one_east() {
        let head = (1, -2);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 1);
        assert_eq!(new_tail.1, -1);
    }

    #[test]
    fn update_tail_two_south_one_east() {
        let head = (1, 2);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 1);
        assert_eq!(new_tail.1, 1);
    }

    #[test]
    fn update_tail_two_north_one_west() {
        let head = (-1, -2);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, -1);
        assert_eq!(new_tail.1, -1);
    }

    #[test]
    fn update_tail_one_south_two_east() {
        let head = (2, 1);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, 1);
        assert_eq!(new_tail.1, 1);
    }

    #[test]
    fn update_tail_one_north_two_west() {
        let head = (-2, -1);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, -1);
        assert_eq!(new_tail.1, -1);
    }

    #[test]
    fn update_tail_two_south_one_west() {
        let head = (-1, 2);
        let tail = (0, 0);
        let new_tail = update_single_tail(head, tail);

        assert_eq!(new_tail.0, -1);
        assert_eq!(new_tail.1, 1);
    }
}