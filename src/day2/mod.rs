use crate::utils::get_lines_from_file;

const INPUT_EXAMPLE_FILE_PATH: &str = "src/day2/inputExample.txt";
const INPUT_TEST_FILE_PATH: &str = "src/day2/input.txt";

fn part1(lines: &Vec<String>) -> i32 {
    let mut depth = 0;
    let mut position = 0;

    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let direction = String::from(split[0]);
        let distance = split[1].parse::<i32>().expect("Could not convert split[1] to i32");

        match direction.as_str() {
            "forward" => position += distance,
            "up" => depth -= distance,
            "down" => depth += distance,
            _ => println!("Direction does not match known directions"),
        }
    }

    depth * position
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut depth = 0;
    let mut aim = 0;
    let mut position = 0;

    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let direction = String::from(split[0]);
        let distance = split[1].parse::<i32>().expect("Could not convert split[1] to i32");

        match direction.as_str() {
            "forward" => {
                position += distance;
                depth += aim * distance;
            },
            "up" => aim -= distance,
            "down" => aim += distance,
            _ => println!("Direction does not match known directions"),
        }
    }

    depth * position
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

        assert_eq!(answer, 150);
    }

    #[test]
    fn part1_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part1(&lines);

        assert_eq!(answer, 1660158);
    }

    #[test]
    fn part2_example_input() {
        let lines = get_lines_from_file(INPUT_EXAMPLE_FILE_PATH);
        let answer = part2(&lines);

        assert_eq!(answer, 900);
    }

    #[test]
    fn part2_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part2(&lines);

        assert_eq!(answer, 1604592846);
    }
}