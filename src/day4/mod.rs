use crate::utils::get_lines_from_file;

const INPUT_EXAMPLE_FILE_PATH: &str = "src/day4/inputExample.txt";
const INPUT_TEST_FILE_PATH: &str = "src/day4/input.txt";

const NUMBER_LIST_INDEX: usize = 0;
const FIRST_BOARD_INDEX: usize = 2;
const BOARD_SIZE: usize = 5;

struct Game {
    board: Vec<Cell>,
    is_completed: bool,
}

struct Cell {
    value: i32,
    is_marked: bool
}

fn get_number_sequence(lines: &Vec<String>) -> Vec<i32> {
    lines[NUMBER_LIST_INDEX].split(",")
                            .into_iter()
                            .map(|num| num.parse::<i32>().expect("Could not convert line to i32"))
                            .collect()
}

fn get_games(lines: &Vec<String>) -> Vec<Game> { 
    let number_of_games = (lines.len() - 1) / (BOARD_SIZE + 1);

    let mut games: Vec<Game> = Vec::with_capacity(number_of_games);

    for game_number in 0..number_of_games {
        let mut board: Vec<Cell> = Vec::with_capacity(BOARD_SIZE * BOARD_SIZE);

        for row in 0..BOARD_SIZE {
            let line_number = FIRST_BOARD_INDEX + ((BOARD_SIZE + 1) * game_number) + row;

            for char in lines[line_number].split_whitespace() {
                let value = char.parse::<i32>().expect("get_games: Could not convert char to i32");
                let cell = Cell {
                    value,
                    is_marked: false,
                };
                board.push(cell);
            }
        }

        let game = Game { 
            board,
            is_completed: false,
        };

        games.push(game);
    }

    games
}

fn record_number(number: i32, games: &mut Vec<Game>) {
    for game in games {
        for i in 0..game.board.len() {
            if number == game.board[i].value {
                game.board[i].is_marked = true;
            }
        }
    }
}

fn get_newly_completed_game(games: &mut Vec<Game>) -> Option<usize> {
    let mut g = 0;

    for game in games {
        if game.is_completed { 
            g += 1;
            continue; 
        }

        for i in 0..BOARD_SIZE {
            let mut horizontal_solution_complete = true;
            let mut vertical_solution_complete = true;
            for j in 0..BOARD_SIZE {
                if !game.board[i * BOARD_SIZE + j].is_marked {
                    horizontal_solution_complete = false;
                }
                if !game.board[j * BOARD_SIZE + i].is_marked {
                    vertical_solution_complete = false;
                }
            }

            if horizontal_solution_complete || vertical_solution_complete {
                game.is_completed = true;
                return Some(g);
            }
        }
        
        g += 1;
    }

    None
}

fn get_score(number: i32, game: &Game) -> i32 {
    let mut unmarked_sum = 0;

    for cell in &game.board {
        if !cell.is_marked {
            unmarked_sum += cell.value;
        }
    }

    number * unmarked_sum
}

fn part1(lines: &Vec<String>) -> i32 {
    let number_sequence: Vec<i32> = get_number_sequence(lines);
    let mut games: Vec<Game> = get_games(lines);

    for number in number_sequence {
        record_number(number, &mut games);

        match get_newly_completed_game(&mut games) {
            Some(g) => return get_score(number, &games[g]),
            None => continue,
        }
    }

    -1 // Should not get here
}

fn part2(lines: &Vec<String>) -> i32 {
    let number_sequence: Vec<i32> = get_number_sequence(lines);
    let mut games: Vec<Game> = get_games(lines);

    let mut games_won = 0;

    // let mut last_g = 0;
    // let mut last_num = 0;

    for number in number_sequence {
        record_number(number, &mut games);

        match get_newly_completed_game(&mut games) {
            Some(g) => {
                games_won += 1;

                // last_g = g;
                // last_num = number;

                if games_won == games.len() {
                    return get_score(number, &games[g]);
                }
            },
            None => continue,
        }
    }

    println!("Number of Games: {:?}", games.len());
    println!("Games Won: {:?}", games_won);

    // (last_g as i32) * last_num // Should not get here
    -1 
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

        assert_eq!(answer, 4512);
    }

    #[test]
    fn part1_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part1(&lines);

        assert_eq!(answer, 49686);
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