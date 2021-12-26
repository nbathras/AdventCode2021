use crate::utils::get_lines_from_file;

const INPUT_EXAMPLE_FILE_PATH: &str = "src/day6/inputExample.txt";
const INPUT_TEST_FILE_PATH: &str = "src/day6/input.txt";

const GESTATION_DAYS: usize = 7;
const MATURITY_DAYS: usize = 2;

struct Population {
    entries: Vec<PopulationEntry>
}

impl Population {
    pub fn new(line: &String) -> Population {
        let mut entries: Vec<PopulationEntry> = Vec::new();
        for _ in 0..(GESTATION_DAYS + MATURITY_DAYS) {
            entries.push(PopulationEntry {
                current_population: 0,
                next_population: 0,
            })
        }

        let fishes = line.split(",");
        for fish in fishes {
            let index = fish.parse::<usize>().expect("Could not convert entry to i32");
            entries[index].increment_current_population();
        }

        Population { entries }
    }

    fn simulate_next_day(&mut self) {
        for i in 0..(GESTATION_DAYS + MATURITY_DAYS) {
            let curr_pop = self.entries[i].current_population;

            if i == 0 {
                self.entries[GESTATION_DAYS + MATURITY_DAYS - 1].add_to_next_population(curr_pop);
                self.entries[GESTATION_DAYS - 1].add_to_next_population(curr_pop);
            } else {
                self.entries[i - 1].add_to_next_population(curr_pop);
            }
        }

        for i in 0..(GESTATION_DAYS + MATURITY_DAYS) {
            self.entries[i].execute();
        }
    }

    fn simulate(&mut self, number_of_days: i64) {
        for _ in 0..number_of_days {
            self.simulate_next_day();
        }
    }

    fn get_total_population(&mut self) -> i64{
        let mut total_population = 0;

        for entry in &self.entries {
            total_population += entry.current_population;
        }

        total_population
    }
}

struct PopulationEntry {
    current_population: i64,
    next_population: i64,
}

impl PopulationEntry {
    fn increment_current_population(&mut self) {
        self.current_population += 1;
    }

    fn add_to_next_population(&mut self, pop: i64) {
        self.next_population += pop;
    }

    fn execute(&mut self) {
        self.current_population = self.next_population;
        self.next_population = 0;
    }
}

fn part1(lines: &Vec<String>) -> i64 {
    let mut population = Population::new(&lines[0]);
    population.simulate(80);

    population.get_total_population()
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut population = Population::new(&lines[0]);
    population.simulate(256);

    population.get_total_population()
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

    fn part1_explicit_days(lines: &Vec<String>, number_of_days: i64) -> i64 {
        let mut population = Population::new(&lines[0]);
        population.simulate(number_of_days);
        population.get_total_population()
    }

    #[test]
    fn part1_example_input() {
        let lines = get_lines_from_file(INPUT_EXAMPLE_FILE_PATH);
        let answer_18 = part1_explicit_days(&lines, 18);
        let answer = part1(&lines);

        assert_eq!(answer_18, 26);
        assert_eq!(answer, 5934);
    }

    #[test]
    fn part1_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part1(&lines);

        assert_eq!(answer, 346063);
    }

    #[test]
    fn part2_example_input() {
        let lines = get_lines_from_file(INPUT_EXAMPLE_FILE_PATH);
        let answer = part2(&lines);

        assert_eq!(answer, 26984457539);
    }

    #[test]
    fn part2_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part2(&lines);

        assert_eq!(answer, 1572358335990);
    }
}