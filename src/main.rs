use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn get_stripped_user_input(user_input: &mut String) {
    std::io::stdin().read_line(user_input).expect("Failed to read user input");
    user_input.retain(|c| !c.is_whitespace());
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_filename() -> String {
    let mut user_input = String::new();
    println!("Which input file should we use.  Type '1' for example input and '2' for test input: ");
    get_stripped_user_input(&mut user_input);
    return match user_input.as_str() {
        "1" => String::from("src/inputExample.txt"),
        _ => String::from("src/input.txt")
    }
}

// ---

fn main() {
    let filename = get_filename();
    let lines = get_lines_from_file(filename);

    let mut previous_depth = 0;
    let mut increase_counter = 0;

    for line in lines {
        let new_depth = line.parse::<i32>().expect("Could not convert line to i32");

        if new_depth > previous_depth {
            increase_counter += 1;
        }

        previous_depth = new_depth;
    }

    println!("{:?}", increase_counter - 1);
}