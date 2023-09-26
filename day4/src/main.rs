use std::ops::Range;

struct ElvesPair {
    a_l: i32,
    a_u: i32,

    b_l: i32,
    b_u: i32,
}

fn bounds_from_string(single_elf_input: &str) -> (i32, i32)
{
    let split = &single_elf_input.split("-").collect::<Vec<_>>();
    let lower: i32 = split[0].parse().unwrap();
    let upper: i32 = 1 + split[1].parse::<i32>().unwrap();
    (lower, upper)
}

impl ElvesPair {
    fn new(input: &str) -> Self {
        let split = &input.split(",").collect::<Vec<_>>();
        let first_elf_bounds = bounds_from_string(split[0]);
        let second_elf_bounds = bounds_from_string(split[1]);

        Self {
            a_l: first_elf_bounds.0,
            a_u: first_elf_bounds.1,
            b_l: second_elf_bounds.0,
            b_u: second_elf_bounds.1,
        }
    }

    pub fn check_for_full_overlap(&self) -> bool {
        // range a < range b
        if self.a_l >= self.b_l && self.a_u <= self.b_u {
            return true;
        }
        // range b < range a
        if self.b_l >= self.a_l && self.b_u <= self.a_u {
            return true;
        }

        // range a == range b
        if self.a_l == self.b_l && self.a_u == self.b_u {
            return true;
        }

        false
    }
}


fn main() {
    let contents = include_str!("../puzzle_input.txt");
    let elves = contents.lines().map(|l| ElvesPair::new(l));
    let count = elves.filter(|ep| ep.check_for_full_overlap()).count();
    println!("elf pairs with full overlap: {}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn elves_pair_no_overlap() {
        let ep = ElvesPair::new("1-1,2-2");
        assert_eq!(ep.check_for_full_overlap(), false);
    }

    #[test]
    fn elves_pair_full_overlap_1() {
        let ep = ElvesPair::new("1-1,1-1");
        assert_eq!(ep.check_for_full_overlap(), true);
    }

    #[test]
    fn elves_pair_full_overlap_2() {
        let ep = ElvesPair::new("4-110,4-110");
        assert_eq!(ep.check_for_full_overlap(), true);
    }

    #[test]
    fn elves_pair_partial_overlap_1() {
        let ep = ElvesPair::new("1-5,4-6");
        assert_eq!(ep.check_for_full_overlap(), false);
    }

    #[test]
    fn elves_pair_completely_enclosed_overlap() {
        let ep = ElvesPair::new("1-5,3-4");
        assert_eq!(ep.check_for_full_overlap(), true);
    }
}

