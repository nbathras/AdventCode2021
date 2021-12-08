use core::num;

use crate::utils::get_lines_from_file;

const INPUT_EXAMPLE_FILE_PATH: &str = "src/day4/inputExample.txt";
const INPUT_TEST_FILE_PATH: &str = "src/day4/input.txt";

fn part1(lines: &Vec<String>) -> i32 {
    // ToDo:
    // 1. Move constants to file scope
    // 2. Create a file scope method to get the nums (also come up with a better var name then nums)
    // 3. Create a file scope method to get the games
    // 4. Iterate through the list of nums
    //     4.1. When we find one of the nums in the game make the number negative
    //     4.2. Create a file scope method to check if the game board has a negatives in all rows or all columns
    // 5. Combine result and return

    const NUMBER_LIST_INDEX: usize = 0;
    const FIRST_BOARD_INDEX: usize = 2;
    const BOARD_SIZE: usize = 5;

    let nums: Vec<i32> = lines[NUMBER_LIST_INDEX].split(",")
                                                 .into_iter()
                                                 .map(|num| num.parse::<i32>().expect("Could not convert line to i32"))
                                                 .collect();

    let mut games: Vec<Vec<Vec<i32>>> = Vec::new();

    let num_of_games = (lines.len() - 1) / (BOARD_SIZE + 1);

    for g in 0..num_of_games {
        games.push(Vec::new());

        for r in 0..BOARD_SIZE {
            let i = FIRST_BOARD_INDEX + ((BOARD_SIZE + 1) * g) + r;

            let row: Vec<i32> = lines[i].split_whitespace()
                                        .into_iter()
                                        .map(|num| num.parse::<i32>().expect("Could not convert line to i32"))
                                        .collect();

            games[g].push(row);
        }
    }

    println!("test1");

    0
}

fn part2(lines: &Vec<String>) -> i32 {
    // ToDo: Implement
    0
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

        assert_eq!(answer, answer);
    }

    // #[test]
    // fn part1_input() {
    //     let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
    //     let answer = part1(&lines);

    //     assert_eq!(answer, 841526);
    // }

    // #[test]
    // fn part2_example_input() {
    //     let lines = get_lines_from_file(INPUT_EXAMPLE_FILE_PATH);
    //     let answer = part2(&lines);

    //     assert_eq!(answer, 230);
    // }

    // #[test]
    // fn part2_input() {
    //     let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
    //     let answer = part2(&lines);

    //     assert_eq!(answer, 4790390);
    // }
}