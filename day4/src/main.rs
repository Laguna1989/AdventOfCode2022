struct ElvesPair {
    a_l: i32,
    a_u: i32,

    b_l: i32,
    b_u: i32,
}

impl ElvesPair {
    fn new(input: &str) -> Self {
        let (first, second) = &input.split_once(",").unwrap();
        let ((a_l, a_u), (b_l, b_u)) = (first.split_once('-').unwrap(), second.split_once('-').unwrap());

        Self {
            a_l: a_l.parse::<i32>().unwrap(),
            a_u: a_u.parse::<i32>().unwrap(),
            b_l: b_l.parse::<i32>().unwrap(),
            b_u: b_u.parse::<i32>().unwrap(),
        }
    }

    pub fn check_for_any_overlap(&self) -> bool {
        return self.a_l <= self.b_u && self.b_l <= self.a_u;
    }
}


fn main() {
    let contents = include_str!("../puzzle_input.txt");
    let elves = contents.lines().map(|l| ElvesPair::new(l));
    let count = elves.filter(|ep| ep.check_for_any_overlap()).count();
    println!("elf pairs with full overlap: {}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn elves_pair_no_overlap() {
        let ep = ElvesPair::new("1-1,2-2");
        assert_eq!(ep.check_for_any_overlap(), false);
    }

    #[test]
    fn elves_pair_full_overlap_1() {
        let ep = ElvesPair::new("1-1,1-1");
        assert_eq!(ep.check_for_any_overlap(), true);
    }

    #[test]
    fn elves_pair_full_overlap_2() {
        let ep = ElvesPair::new("4-110,4-110");
        assert_eq!(ep.check_for_any_overlap(), true);
    }

    #[test]
    fn elves_pair_partial_overlap_1() {
        let ep = ElvesPair::new("1-5,4-6");
        assert_eq!(ep.check_for_any_overlap(), true);
    }

    #[test]
    fn elves_pair_completely_enclosed_overlap() {
        let ep = ElvesPair::new("1-5,3-4");
        assert_eq!(ep.check_for_any_overlap(), true);
    }

    #[test]
    fn example_overlap_1() {
        let ep = ElvesPair::new("5-7,7-9");
        assert_eq!(ep.check_for_any_overlap(), true);
    }

    #[test]
    fn example_overlap_2() {
        let ep = ElvesPair::new("2-8,3-7");
        assert_eq!(ep.check_for_any_overlap(), true);
    }

    #[test]
    fn example_overlap_3() {
        let ep = ElvesPair::new("6-6,4-6");
        assert_eq!(ep.check_for_any_overlap(), true);
    }

    #[test]
    fn example_overlap_4() {
        let ep = ElvesPair::new("2-6,4-8");
        assert_eq!(ep.check_for_any_overlap(), true);
    }

    #[test]
    fn example_overlap_5() {
        let ep = ElvesPair::new("2-4,6-8");
        assert_eq!(ep.check_for_any_overlap(), false);
    }

    #[test]
    fn example_overlap_6() {
        let ep = ElvesPair::new("2-3,4-5");
        assert_eq!(ep.check_for_any_overlap(), false);
    }
}

