use std::fs;

fn main() {
    let file_path = "./example.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut max_calories_per_elf : u32 = 0;

    let split_content = contents.split("\n\n");
    for calories_per_elf in split_content {
        let mut total_calories_per_elf : u32 = 0;
        
        for calory_str in calories_per_elf.trim().split("\n") {
            let calory = calory_str.trim().parse::<u32>();
            total_calories_per_elf += calory.unwrap();
        }

        if total_calories_per_elf >= max_calories_per_elf {
            max_calories_per_elf = total_calories_per_elf;
        }
    }

    println!("Max calories per Elf:\n{max_calories_per_elf}");
}
