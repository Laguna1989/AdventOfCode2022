fn transform_opponent(icon: &str) -> i8
{
    return match icon {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0
    };
}

fn transform_counter(icon: &str) -> i8
{
    return match icon {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    };
}

fn get_win_score(opponent: &str, counter: &str) -> i32
{
    if transform_opponent(opponent) == transform_counter(counter)
    {
        return 3;
    }
    let expected_counter = match opponent {
        "A" => "Y",
        "B" => "Z",
        "C" => "X",
        _ => ""
    };
    if expected_counter.eq(counter)
    {
        return 6;
    }
    return 0;
}

fn icon_score(icon: &str) -> i32
{
    return match icon {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    };
}

fn get_score_from_line(line: &str) -> i32
{
    let split: Vec<&str> = line.split(" ").collect();
    return get_win_score(split[0], split[1]) + icon_score(split[1]);
}


fn main() {
    let contents = include_str!("../puzzle_input.txt");

    let sum_score: i32 = contents.lines().map(|line| get_score_from_line(line)).sum();

    println!("{}", sum_score);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_win() {
        assert_eq!(get_win_score("A", "Y"), true);
        assert_eq!(get_win_score("A", "X"), false);
        assert_eq!(get_win_score("A", "Z"), false);

        assert_eq!(get_win_score("B", "Y"), false);
        assert_eq!(get_win_score("B", "X"), false);
        assert_eq!(get_win_score("B", "Z"), true);

        assert_eq!(get_win_score("C", "Y"), false);
        assert_eq!(get_win_score("C", "X"), true);
        assert_eq!(get_win_score("C", "Z"), false);
    }
}
