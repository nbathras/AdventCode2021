use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

fn part1(lines: &Vec<String>) -> i32 {
    let mut previous_depth = 0;
    let mut increase_counter = 0;

    for line in lines {
        let new_depth = line.parse::<i32>().expect("Could not convert line to i32");

        if new_depth > previous_depth {
            increase_counter += 1;
        }

        previous_depth = new_depth;
    }

    increase_counter - 1
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut previous_depth = 0;
    let mut increase_counter = 0;
    
    for i in 0..lines.len() {
        if i + 2 >= lines.len() { break; }

        let new_depth1 = lines[i].parse::<i32>().expect("Could not convert line to i32");
        let new_depth2 = lines[i+1].parse::<i32>().expect("Could not convert line to i32");
        let new_depth3 = lines[i+2].parse::<i32>().expect("Could not convert line to i32");
        let new_depth = new_depth1 + new_depth2 + new_depth3;

        if new_depth > previous_depth {
            increase_counter += 1;
        }

        previous_depth = new_depth;
    }

    increase_counter - 1
}

fn compute_solutions(lines: Vec<String>) {
    println!("Part1: {:?}", part1(&lines));
    println!("Part2: {:?}\n", part2(&lines));
}

fn compute_solution() {
    println!("Example Input:");
    let lines = get_lines_from_file("src/inputExample.txt");
    compute_solutions(lines);

    println!("Test Input:");
    let lines = get_lines_from_file("src/input.txt");
    compute_solutions(lines);
}

fn main() {
    compute_solution();
}