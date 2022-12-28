fn parse_and_unwrap(calory_str: &str) -> u32 {
    calory_str.parse::<u32>().unwrap()
}

fn main() {
    let contents = include_str!("../example.txt");

    let mut vec_max_calories_per_elf: Vec<u32> = contents.split("\n\n")
        .map(|calories_per_elf| calories_per_elf
            .lines()
            .map(|calory_str| parse_and_unwrap(calory_str))
            .sum())
        .collect();
    vec_max_calories_per_elf.sort_unstable();

    println!("Top 3 calories:\n{}", vec_max_calories_per_elf.iter().rev().take(3).sum::<u32>());
}
