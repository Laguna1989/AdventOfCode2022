use std::fs;

fn parse_and_unwrap(calory_str: &str) -> u32 {
    calory_str.parse::<u32>().unwrap()
}

fn main() {
    let file_path = "./example.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let max_calories_per_elf: u32 = contents.split("\n\n")
        .map(|calories_per_elf| calories_per_elf
            .trim().split("\n")
            .map(|calory_str| parse_and_unwrap(calory_str))
            .sum())
        .max()
        .unwrap();

    println!("Max calories per Elf:\n{max_calories_per_elf}");
}


