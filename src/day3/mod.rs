use crate::utils::get_lines_from_file;

const INPUT_EXAMPLE_FILE_PATH: &str = "src/day3/inputExample.txt";
const INPUT_TEST_FILE_PATH: &str = "src/day3/input.txt";

fn part1(lines: &Vec<String>) -> i32 {
    // Two assumptions are made about lines
    // 1. lines is not empty
    // 2. all lines are the same length
    let line_length: usize = lines[0].len();

    let numbers: Vec<i32> = lines.iter()
                                 .map(|line| i32::from_str_radix(line, 2).expect("Could not convert binary line to i32"))
                                 .collect();

    let mut gamma = 0;
    let mut epislon = 0;
    
    for i in 0..line_length {
        let mut zero_count = 0;
        let mut one_count = 0;

        for num in &numbers {
            let bit = (1 << i) & num;

            match bit != 0 {
                // false then bit is 0
                false => zero_count += 1,
                // true then bit is 1
                true => one_count += 1,
            }
        }

        const BASE: i32 = 2;
        let exp: u32 = i.try_into().unwrap();
        if one_count > zero_count {
            gamma += BASE.pow(exp);
        } else {
            epislon += BASE.pow(exp);
        }
    }

    gamma * epislon
}

fn part2(lines: &Vec<String>) -> i32 {
    // Two assumptions are made about lines
    // 1. lines is not empty
    // 2. all lines are the same length
    let line_length: usize = lines[0].len();

    let numbers: Vec<i32> = lines.iter()
                                 .map(|line| i32::from_str_radix(line, 2).expect("Could not convert binary line to i32"))
                                 .collect();

    let mut oxygen_generator_rating = numbers.clone();
    let mut co2_scrubber_rating = numbers.clone();

    for i in 0..line_length {
        let mut zero_list: Vec<i32> = Vec::new();
        let mut one_list: Vec<i32> = Vec::new();

        for num in &oxygen_generator_rating {
            let bit = (1 << (line_length - i -1)) & num;

            match bit != 0 {
                // false then bit is 0
                false => zero_list.push(*num),
                // true then bit is 1
                true => one_list.push(*num),
            }
        }

        if one_list.len() >= zero_list.len() {
            oxygen_generator_rating = one_list.clone();
        } else {
            oxygen_generator_rating = zero_list.clone();
        }

        if oxygen_generator_rating.len() == 1 {break;}
    }

    for i in 0..line_length {
        let mut zero_list: Vec<i32> = Vec::new();
        let mut one_list: Vec<i32> = Vec::new();

        for num in &co2_scrubber_rating {
            let bit = (1 << (line_length - i -1)) & num;

            match bit != 0 {
                // false then bit is 0
                false => zero_list.push(*num),
                // true then bit is 1
                true => one_list.push(*num),
            }
        }

        if zero_list.len() <= one_list.len() {
            co2_scrubber_rating = zero_list.clone();
        } else {
            co2_scrubber_rating = one_list.clone();
        }

        if co2_scrubber_rating.len() == 1 {break;}
    }

    oxygen_generator_rating[0] * co2_scrubber_rating[0]
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

        assert_eq!(answer, 198);
    }

    #[test]
    fn part1_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part1(&lines);

        assert_eq!(answer, 841526);
    }

    #[test]
    fn part2_example_input() {
        let lines = get_lines_from_file(INPUT_EXAMPLE_FILE_PATH);
        let answer = part2(&lines);

        assert_eq!(answer, 230);
    }

    #[test]
    fn part2_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part2(&lines);

        assert_eq!(answer, 4790390);
    }
}