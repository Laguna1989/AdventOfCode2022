fn main() {
    let contents = include_str!("../puzzle_input.txt");
    let forest = Forest::new(contents);
    let visible_trees = forest.get_visible_trees();

    println!("visible_trees: {}", visible_trees);

    let highest_scenic_score = forest.calculate_scenic_score_max();

    println!("highest scenic score: {}", highest_scenic_score);
}

struct Forest {
    trees: Vec<i32>,
    size_x: i32,
    size_y: i32,
}

enum Direction {
    N,
    S,
    E,
    W,
}


impl Forest {
    pub fn new(contents: &str) -> Self {
        let lines = contents.lines().collect::<Vec<_>>();
        let x = if lines.len() == 0 { 0 } else { lines[0].len() } as i32;
        let tree_heights: Vec<i32> = contents.replace("\n", "").chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();
        Self { trees: tree_heights, size_x: x, size_y: lines.len() as i32 }
    }

    pub fn get_visible_trees(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.size_x {
            for j in 0..self.size_y {
                let dirs = vec![Direction::N, Direction::S, Direction::E, Direction::W];
                if dirs.iter().any(|d| self.check_tree_visible_in_direction(i, j, d)) {
                    sum += 1;
                }
            }
        }

        sum
    }

    fn pos_to_idx(&self, x: i32, y: i32) -> usize {
        if x < 0 || x >= self.size_x {
            panic!("x index out of bounds");
        }
        if y < 0 || x >= self.size_y {
            panic!("y index out of bounds");
        }
        (x + y * self.size_x) as usize
    }

    pub fn get_height_at(&self, x: i32, y: i32) -> i32 {
        self.trees[self.pos_to_idx(x, y)]
    }

    pub fn direction_to_vec(&self, dir: &Direction) -> (i32, i32) {
        match dir {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
        }
    }

    pub fn check_tree_visible_in_direction(&self, x: i32, y: i32, dir: &Direction) -> bool
    {
        if x == 0 || y == 0 || x == self.size_x - 1 || y == self.size_y - 1 {
            return true;
        }
        let (dir_x, dir_y) = self.direction_to_vec(dir);
        let (mut i, mut j) = (x, y);
        let original_height = self.get_height_at(x, y);
        let mut found_higher_tree = false;
        loop {
            i += dir_x;
            j += dir_y;

            if i < 0 || i >= self.size_x {
                break;
            }
            if j < 0 || j >= self.size_y {
                break;
            }
            let other_height = self.get_height_at(i, j);
            found_higher_tree |= other_height >= original_height;
            if found_higher_tree {
                break;
            }
        }

        !found_higher_tree
    }

    pub fn get_view_distance(&self, x: i32, y: i32, dir: &Direction) -> i32
    {
        let (dir_x, dir_y) = self.direction_to_vec(dir);

        let (mut i, mut j) = (x, y);
        let original_height = self.get_height_at(x, y);
        let mut view_distance = 0;
        loop {
            view_distance += 1;
            i += dir_x;
            j += dir_y;

            if i < 0 {
                view_distance -= 1;
                break;
            }
            if i >= self.size_x {
                view_distance -= 1;
                break;
            }
            if j < 0 {
                view_distance -= 1;
                break;
            }
            if j >= self.size_y {
                view_distance -= 1;
                break;
            }

            let other_height = self.get_height_at(i, j);
            if other_height >= original_height {
                break;
            }
        }
        return view_distance;
    }

    pub fn calculate_scenic_score_at(&self, x: i32, y: i32) -> i32 {
        vec![Direction::N, Direction::S, Direction::E, Direction::W].
            iter().
            map(
                |d| self.get_view_distance(x, y, d)
            ).product()
    }

    pub fn calculate_scenic_score_max(&self) -> i32 {
        let mut max = 0;
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                let current_score = self.calculate_scenic_score_at(x, y);
                if current_score > max {
                    max = current_score;
                }
            }
        }
        return max;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn end_to_end_example() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        let visible_trees = forest.get_visible_trees();

        assert_eq!(visible_trees, 21);
    }

    #[test]
    fn end_to_end_one_single_tree() {
        let contents = "1";
        let forest = Forest::new(contents);
        let visible_trees = forest.get_visible_trees();

        assert_eq!(visible_trees, 1);
    }

    #[test]
    fn forest_constructor_parses_size_y_for_1x1_forest() {
        let contents = "1";
        let forest = Forest::new(contents);
        assert_eq!(forest.size_y, 1);
    }

    #[test]
    fn forest_constructor_parses_size_y_for_2x2_forest() {
        let contents = "12\n34";
        let forest = Forest::new(contents);
        assert_eq!(forest.size_y, 2);
    }

    #[test]
    fn forest_constructor_parses_size_y_for_3x1_forest() {
        let contents = "123";
        let forest = Forest::new(contents);
        assert_eq!(forest.size_y, 1);
    }

    #[test]
    fn forest_constructor_parses_size_y_for_example_forest() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.size_y, 5);
    }

    #[test]
    fn forest_constructor_parses_size_x_for_1x1_forest() {
        let contents = "1";
        let forest = Forest::new(contents);
        assert_eq!(forest.size_x, 1);
    }

    #[test]
    fn forest_constructor_parses_size_x_for_2x2_forest() {
        let contents = "12\n34";
        let forest = Forest::new(contents);
        assert_eq!(forest.size_x, 2);
    }

    #[test]
    fn forest_constructor_parses_size_x_for_3x1_forest() {
        let contents = "123";
        let forest = Forest::new(contents);
        assert_eq!(forest.size_x, 3);
    }

    #[test]
    fn forest_constructor_parses_size_x_for_example_forest() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.size_x, 5);
    }


    #[test]
    fn forest_get_height_at() {
        let contents = "12\n34";
        let forest = Forest::new(contents);
        assert_eq!(forest.get_height_at(0, 0), 1);
        assert_eq!(forest.get_height_at(1, 0), 2);
        assert_eq!(forest.get_height_at(0, 1), 3);
        assert_eq!(forest.get_height_at(1, 1), 4);
    }

    #[test]
    fn forest_check_tree_visible_in_dir_for_3x3_asc() {
        let contents = "123\n456\n768";
        let forest = Forest::new(contents);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::N), true);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::W), true);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::S), false);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::E), false);
    }

    #[test]
    fn forest_check_tree_visible_in_dir_for_3x3_center_high() {
        let contents = "111\n191\n111";
        let forest = Forest::new(contents);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::N), true);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::W), true);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::S), true);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::E), true);
    }

    #[test]
    fn forest_check_tree_visible_in_dir_for_3x3_center_low() {
        let contents = "999\n919\n999";
        let forest = Forest::new(contents);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::N), false);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::W), false);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::S), false);
        assert_eq!(forest.check_tree_visible_in_direction(1, 1, &Direction::E), false);
    }

    #[test]
    fn forest_check_tree_visible_in_dir_for_1x1() {
        let contents = "9";
        let forest = Forest::new(contents);
        assert_eq!(forest.check_tree_visible_in_direction(0, 0, &Direction::N), true);
        assert_eq!(forest.check_tree_visible_in_direction(0, 0, &Direction::W), true);
        assert_eq!(forest.check_tree_visible_in_direction(0, 0, &Direction::S), true);
        assert_eq!(forest.check_tree_visible_in_direction(0, 0, &Direction::E), true);
    }

    #[test]
    fn forest_check_tree_visible_in_dir_for_3x3_on_edge() {
        let contents = "999\n999\n999";
        let forest = Forest::new(contents);
        for i in 0..3
        {
            for d in &vec![Direction::N, Direction::S, Direction::E, Direction::W]
            {
                assert_eq!(forest.check_tree_visible_in_direction(i, 0, d), true);
                assert_eq!(forest.check_tree_visible_in_direction(0, i, d), true);
                assert_eq!(forest.check_tree_visible_in_direction(i, 2, d), true);
                assert_eq!(forest.check_tree_visible_in_direction(2, i, d), true);
            }
        }
    }

    #[test]
    fn forest_check_tree_visible_edge_case() {
        let contents = "1111\n1111\n1219\n1111";
        let forest = Forest::new(contents);

        assert_eq!(forest.check_tree_visible_in_direction(1, 2, &Direction::E), false);
    }

    #[test]
    fn forest_check_view_distance_example_1_n() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.get_view_distance(2, 1, &Direction::N), 1);
    }

    #[test]
    fn forest_check_view_distance_example_1_w() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.get_view_distance(2, 1, &Direction::W), 1);
    }

    #[test]
    fn forest_check_view_distance_example_1_e() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.get_view_distance(2, 1, &Direction::E), 2);
    }

    #[test]
    fn forest_check_view_distance_example_1_s() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.get_view_distance(2, 1, &Direction::S), 2);
    }


    #[test]
    fn forest_check_view_distance_example_2_n() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.get_view_distance(2, 3, &Direction::N), 2);
    }

    #[test]
    fn forest_check_view_distance_example_2_w() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.get_view_distance(2, 3, &Direction::W), 2);
    }

    #[test]
    fn forest_check_view_distance_example_2_e() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.get_view_distance(2, 3, &Direction::E), 2);
    }

    #[test]
    fn forest_check_view_distance_example_2_s() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.get_view_distance(2, 3, &Direction::S), 1);
    }

    #[test]
    fn calculate_scenic_score_exaple_1() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.calculate_scenic_score_at(2, 1), 4);
    }

    #[test]
    fn calculate_scenic_score_exaple_2() {
        let contents = include_str!("../example.txt");
        let forest = Forest::new(contents);
        assert_eq!(forest.calculate_scenic_score_at(2, 3), 8);
    }
}