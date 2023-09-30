fn main() {
    let disk_size = 70000000;

    let contents = include_str!("../puzzle_input.txt");
    let root_dir = get_directories(contents);

    let used_space = root_dir.get_directory_size();
    let needed_space = 30000000;
    let unused_soace = disk_size - used_space;
    let required_space = needed_space - unused_soace;

    let returned_size = add_up_file_size_if_larger_than(&root_dir);
    println!("summed size of directories smaller than: {}", returned_size);


    println!("{}", size_of_smallest_dir_above(required_space, &root_dir));
}

fn add_dir_to_vec_recursive(dirs: &mut Vec<i32>, dir: &Directory)
{
    dirs.push(dir.get_directory_size());
    for d in &dir.subdirs {
        add_dir_to_vec_recursive(dirs, d);
    }
}

fn flatten_structure(root_dir: &Directory) -> Vec<i32> {
    let mut dirs: Vec<i32> = vec![];
    add_dir_to_vec_recursive(&mut dirs, root_dir);
    return dirs;
}

fn size_of_smallest_dir_above(min_space: i32, root_dir: &Directory) -> i32
{
    let mut dirs: Vec<i32> = flatten_structure(&root_dir).iter().map(|x| *x).filter(|x| *x >= min_space).collect();
    dirs.sort();
    return dirs.first().unwrap().clone();
}

struct Directory {
    name: String,
    subdirs: Vec<Directory>,
    direct_files_size: i32,
    parent: Option<*mut Directory>,
}

impl Directory {
    fn add_file(&mut self, size: i32) {
        self.direct_files_size += size;
    }

    fn add_directory(&mut self, name: String, parent: *mut Directory) {
        let new_dir = Directory { name: name, subdirs: vec![], direct_files_size: 0, parent: Some(parent) };
        self.subdirs.push(new_dir);
    }

    fn get_directory_size(&self) -> i32 {
        return self.direct_files_size + self.subdirs.iter().map(|d| d.get_directory_size()).sum::<i32>();
    }
}

struct Command {
    cmd: String,
    output: Vec<String>,
}


fn content_to_commands(content: &str) -> Vec<Command>
{
    let mut result = vec![];

    let lines = content.lines().collect::<Vec<_>>();
    let mut current_command: Option<Command> = None;
    for line in lines {
        if line.starts_with("$")
        {
            // store the old current command
            if current_command.is_some() {
                result.push(current_command.unwrap());
            }

            let cmd_str = &line[2..];
            current_command = Some(Command { cmd: String::from(cmd_str), output: vec![] });
        } else {
            if current_command.is_none() {
                continue;
            }

            // can only unwrap once
            let old_command = current_command.unwrap();
            let old_cmd = old_command.cmd.clone();
            let mut old_output = old_command.output.clone();
            old_output.push(line.to_string());
            current_command = Some(Command { cmd: old_cmd, output: old_output });
        }
    }
    if current_command.is_some() {
        // push last command if existing
        result.push(current_command.unwrap());
    }

    return result;
}

fn apply_commands(root_dir: &mut Directory, commands: Vec<Command>)
{
    unsafe {
        let mut current_dir: *mut Directory = root_dir;
        for c in commands {
            if c.cmd == "ls" {
                for result in c.output {
                    if result.starts_with("dir") {
                        // ignore
                        continue;
                    }
                    let (size, _) = result.split_once(" ").unwrap();
                    current_dir.as_mut().unwrap().add_file(size.parse::<i32>().unwrap());
                }
            } else if c.cmd.starts_with("cd") {
                let (_, dir_name) = c.cmd.split_once(" ").unwrap();
                if dir_name == ".." {
                    current_dir = current_dir.as_mut().unwrap().parent.unwrap();
                    continue;
                } else if dir_name == "/" {
                    current_dir = root_dir;
                    continue;
                }
                current_dir.as_mut().unwrap().add_directory(String::from(dir_name), current_dir);
                current_dir = current_dir.as_mut().unwrap().subdirs.last_mut().unwrap();
            }
        }
    }
}

fn get_directories(content: &str) -> Directory {
    let mut root_dir = Directory { name: String::from("/"), subdirs: vec![], direct_files_size: 0, parent: None };
    let commands = content_to_commands(content);
    apply_commands(&mut root_dir, commands);
    return root_dir;
}

fn get_filesize_if_larger_than(dir: &Directory) -> i32
{
    let dir_size = dir.get_directory_size();
    if dir_size < 100000 {
        return dir_size;
    }
    return 0;
}

fn add_up_file_size_if_larger_than(dir: &Directory) -> i32
{
    let mut size = 0;
    size += get_filesize_if_larger_than(dir);

    for d in &dir.subdirs {
        size += add_up_file_size_if_larger_than(&d);
    }

    return size;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1_full() {
        let contents = include_str!("../example.txt");
        let dirs = get_directories(contents);
        let returned_size = add_up_file_size_if_larger_than(&dirs);
        assert_eq!(returned_size, 95437);
    }

    #[test]
    fn example_1_parse_commands() {
        let contents = include_str!("../example.txt");
        let commands = content_to_commands(contents);

        assert_eq!(commands.len(), 10);
        assert_eq!(commands[0].cmd, "cd /");
        assert_eq!(commands[1].cmd, "ls");
        let expected_output = vec![
            String::from("dir a"),
            String::from("14848514 b.txt"),
            String::from("8504156 c.dat"),
            String::from("dir d"),
        ];
        assert_eq!(commands[1].output, expected_output);
        assert_eq!(commands[2].cmd, "cd a");
        assert_eq!(commands[3].cmd, "ls");
        assert_eq!(commands[4].cmd, "cd e");
        assert_eq!(commands[5].cmd, "ls");
        assert_eq!(commands[6].cmd, "cd ..");
        assert_eq!(commands[7].cmd, "cd ..");
        assert_eq!(commands[8].cmd, "cd d");
        assert_eq!(commands[9].cmd, "ls");
    }

    #[test]
    fn empty_input_results_in_empty_root_dir() {
        let contents = "";
        let root_dir = get_directories(contents);
        assert_eq!(root_dir.name, "/");
        assert_eq!(root_dir.subdirs.len(), 0);
        assert_eq!(root_dir.direct_files_size, 0);
    }

    #[test]
    fn parse_ls_without_result_command() {
        let contents = "$ ls";
        let commands = content_to_commands(contents);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].cmd, "ls");
        let expected_output: Vec<String> = vec![];
        assert_eq!(commands[0].output, expected_output);
    }

    #[test]
    fn parse_ls_with_one_line_result() {
        let contents = "$ ls\n123 a.txt";
        let commands = content_to_commands(contents);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].cmd, "ls");
        let expected_output: Vec<String> = vec![String::from("123 a.txt")];
        assert_eq!(commands[0].output, expected_output);
    }

    #[test]
    fn parse_ls_with_two_line_result() {
        let contents = "$ ls\n123 a.txt\n456 b.txt";
        let commands = content_to_commands(contents);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].cmd, "ls");
        let expected_output: Vec<String> = vec![String::from("123 a.txt"), String::from("456 b.txt")];
        assert_eq!(commands[0].output, expected_output);
    }

    #[test]
    fn parse_cd_and_ls_command() {
        let contents = "$ cd /\n$ ls";
        let commands = content_to_commands(contents);
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].cmd, "cd /");
        assert_eq!(commands[1].cmd, "ls");
    }


    #[test]
    fn parse_root_dir_with_one_file() {
        let contents = "$ cd /\n$ ls\n123 a.txt";
        let root_dir = get_directories(contents);
        assert_eq!(root_dir.name, "/");
        assert_eq!(root_dir.subdirs.len(), 0);
        assert_eq!(root_dir.direct_files_size, 123);
    }

    #[test]
    fn parse_root_dir_with_two_files() {
        let contents = "$ cd /\n$ ls\n123 a.txt\n100 b.txt";
        let root_dir = get_directories(contents);
        assert_eq!(root_dir.name, "/");
        assert_eq!(root_dir.subdirs.len(), 0);
        assert_eq!(root_dir.direct_files_size, 223);
    }

    #[test]
    fn parse_dir_with_two_files_in_sub_dir() {
        let contents = "$ cd /\n$ cd a\n$ ls\n123 a.txt\n100 b.txt";
        let root_dir = get_directories(contents);
        assert_eq!(root_dir.name, "/");
        assert_eq!(root_dir.subdirs.len(), 1);
        assert_eq!(root_dir.direct_files_size, 0);
        assert_eq!(root_dir.subdirs[0].direct_files_size, 223);
    }

    #[test]
    fn parse_dir_with_one_file_in_root_dir_and_one_files_in_sub_dir() {
        let contents = "$ cd /\n$ cd a\n$ ls\n100 b.txt\n$ cd /\n$ ls\n123 a.txt";
        let root_dir = get_directories(contents);
        assert_eq!(root_dir.name, "/");
        assert_eq!(root_dir.subdirs.len(), 1);
        assert_eq!(root_dir.direct_files_size, 123);
        assert_eq!(root_dir.subdirs[0].direct_files_size, 100);
    }

    #[test]
    fn parse_dir_with_going_up_in_folders() {
        let contents = "$ cd /\n$ cd a\n$ cd b\n$ ls\n100 a.txt\n$ cd ..\n$ ls\n200 b.txt";
        let root_dir = get_directories(contents);
        assert_eq!(root_dir.name, "/");
        assert_eq!(root_dir.subdirs.len(), 1);
        assert_eq!(root_dir.subdirs[0].direct_files_size, 200);
        assert_eq!(root_dir.subdirs[0].subdirs.len(), 1);
        assert_eq!(root_dir.subdirs[0].subdirs[0].subdirs.len(), 0);
        assert_eq!(root_dir.subdirs[0].subdirs[0].direct_files_size, 100);
    }

    #[test]
    fn get_directory_size() {
        let contents = "$ cd /\n$ cd a\n$ cd b\n$ ls\n100 a.txt\n$ cd ..\n$ ls\n200 b.txt";
        let root_dir = get_directories(contents);
        let result_size = root_dir.get_directory_size();
        assert_eq!(result_size, 300);
    }
}
