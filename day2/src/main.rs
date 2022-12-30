fn get_loose_icon_for(opponent_icon: &str) -> i8 {
    return match opponent_icon {
        "A" => 3,
        "B" => 1,
        "C" => 2,
        _ => 0
    };
}

fn get_draw_icon_for(opponent_icon: &str) -> i8 {
    return match opponent_icon {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0
    };
}

fn get_win_icon_for(opponent_icon: &str) -> i8 {
    return match opponent_icon {
        "A" => 2,
        "B" => 3,
        "C" => 1,
        _ => 0
    };
}

fn get_icon_score(opponent: &str, outcome: &str) -> i32 {
    let my_icon = match outcome
    {
        "X" => get_loose_icon_for(opponent),
        "Y" => get_draw_icon_for(opponent),
        "Z" => get_win_icon_for(opponent),
        _ => 0
    };
    return my_icon as i32;
}


fn get_outcome_score(icon: &str) -> i32
{
    return match icon {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0
    };
}


fn get_score_from_line(line: &str) -> i32
{
    let split: Vec<&str> = line.split(" ").collect();
    return get_icon_score(split[0], split[1]) + get_outcome_score(split[1]);
}


fn main() {
    let contents = include_str!("../puzzle_input.txt");

    let sum_score: i32 = contents.lines().map(|line| get_score_from_line(line)).sum();

    println!("{}", sum_score);
}
