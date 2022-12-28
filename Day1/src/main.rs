use std::fs;

fn parse_and_unwrap(calory_str: &str) -> u32 {
    println!("{calory_str}");
    calory_str.parse::<u32>().unwrap()
}

fn main() {
    let file_path = "./example.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut vec_max_calories_per_elf : Vec<u32> = contents.split("\n\n")
        .map(|calories_per_elf| calories_per_elf
            .trim().split("\n")
            .map(|calory_str| parse_and_unwrap(calory_str))
            .sum())
        .collect();

    vec_max_calories_per_elf.sort();
    vec_max_calories_per_elf.reverse();

    let top3_calories = vec_max_calories_per_elf[0] + vec_max_calories_per_elf[1] + vec_max_calories_per_elf[2];

    println!("Top 3 calories:\n{top3_calories}");
}
