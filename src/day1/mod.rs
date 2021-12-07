use crate::utils::get_lines_from_file;

const INPUT_EXAMPLE_FILE_PATH: &str = "src/day1/inputExample.txt";
const INPUT_TEST_FILE_PATH: &str = "src/day1/input.txt";

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

pub fn compute_solution() {
    println!("Example Input:");
    let lines = get_lines_from_file(INPUT_EXAMPLE_FILE_PATH);
    compute_solutions(lines);

    println!("Test Input:");
    let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
    compute_solutions(lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        let lines = get_lines_from_file(INPUT_EXAMPLE_FILE_PATH);
        let answer = part1(&lines);

        assert_eq!(answer, 7);
    }

    #[test]
    fn part1_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part1(&lines);

        assert_eq!(answer, 1766);
    }

    #[test]
    fn part2_example_input() {
        let lines = get_lines_from_file(INPUT_EXAMPLE_FILE_PATH);
        let answer = part2(&lines);

        assert_eq!(answer, 5);
    }

    #[test]
    fn part2_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part2(&lines);

        assert_eq!(answer, 1797);
    }
}