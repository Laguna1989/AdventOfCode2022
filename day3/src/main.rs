fn main() {
    let contents = include_str!("../puzzle_input.txt");
    let sum: u32 = contents
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| calculate_value_from_group(x)).sum();

    println!("calculated value: {}", sum);
}

fn calculate_value_from_group(group: &[&str]) -> u32 {
    let common_char = get_common_char(group);
    return char_to_value(common_char);
}

fn char_to_value(c: char) -> u32
{
    if c.is_uppercase() {
        return c as u32 - 38;
    }
    return c as u32 - 96;
}

fn get_common_char(group: &[&str]) -> char {
    for c in group[0].chars() {
        let contained_in_second = group[1].contains(c);
        let contained_in_third = group[3].contains(c);
        if contained_in_second && contained_in_third {
            return c;
        }
    }

    panic!("no common char found")
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lower_case_char_to_value_test() {
        assert_eq!(char_to_value('a'), 1);
        assert_eq!(char_to_value('b'), 2);
        assert_eq!(char_to_value('p'), 16);
        assert_eq!(char_to_value('v'), 22);
        assert_eq!(char_to_value('t'), 20);
        assert_eq!(char_to_value('s'), 19);
    }

    #[test]
    fn upper_case_char_to_value_test() {
        assert_eq!(char_to_value('L'), 38);
        assert_eq!(char_to_value('A'), 27);
        assert_eq!(char_to_value('B'), 28);
        assert_eq!(char_to_value('P'), 42);
    }
}