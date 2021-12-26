use crate::utils::Input;

const INPUT_EXAMPLE_FILE_PATH: &str = "src/day7/inputExample.txt";
const INPUT_TEST_FILE_PATH: &str = "src/day7/input.txt";

fn part1(file_name: &str) -> i32 {
    let mut input = Input::init(file_name);
    let positions: Vec<i32> = Input::split_and_parse(input.read_line(), ",");

    let max_position: i32 = *positions.iter().max().expect("No max value found in position");

    let mut min_fuel_cost = i32::MAX;

    for target_position in 0..max_position+1 {
        let mut fuel_cost = 0;

        for j in 0..positions.len() {
            fuel_cost += (positions[j] - target_position).abs();
        }

        if fuel_cost < min_fuel_cost {
            min_fuel_cost = fuel_cost;
        }
    }

    min_fuel_cost
}

fn part2(file_name: &str) -> i32 {
    let mut input = Input::init(file_name);
    let positions: Vec<i32> = Input::split_and_parse(input.read_line(), ",");

    let max_position: i32 = *positions.iter().max().expect("No max value found in position");
    println!("Max Target Position: {}", max_position);

    let mut min_fuel_cost = i32::MAX;

    for target_position in 0..max_position+1 {
        if target_position % 100 == 0 {
            println!("Processing Target Position {}...", target_position);
        }
        let mut fuel_cost = 0;

        for j in 0..positions.len() {
            let distance = (positions[j] - target_position).abs();
            fuel_cost += (distance * (distance + 1)) / 2;
        }

        if fuel_cost < min_fuel_cost {
            min_fuel_cost = fuel_cost;
        }
    }

    min_fuel_cost
}

pub fn compute_solution() {
    println!("Part 1:");
    println!("Example Input:");
    println!("{}", part1(INPUT_EXAMPLE_FILE_PATH));
    println!("Test Input:");
    println!("{}", part1(INPUT_TEST_FILE_PATH));

    println!("Part 2:");
    println!("Example Input:");
    println!("{}", part2(INPUT_EXAMPLE_FILE_PATH));
    println!("Test Input:");
    println!("{}", part2(INPUT_TEST_FILE_PATH));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_input() {
        let answer = part1(INPUT_EXAMPLE_FILE_PATH);

        assert_eq!(answer, 37);
    }

    #[test]
    fn part1_input() {
        let answer = part1(INPUT_TEST_FILE_PATH);

        assert_eq!(answer, 357353);
    }

    #[test]
    fn part2_example_input() {
        let answer = part2(INPUT_EXAMPLE_FILE_PATH);

        assert_eq!(answer, 168);
    }

    #[test]
    fn part2_input() {
        let answer = part2(INPUT_TEST_FILE_PATH);

        assert_eq!(answer, 104822130);
    }
}