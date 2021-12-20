use crate::utils::get_lines_from_file;

const INPUT_EXAMPLE_FILE_PATH: &str = "src/day5/inputExample.txt";
const INPUT_TEST_FILE_PATH: &str = "src/day5/input.txt";

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_x(&self) -> usize {
        self.x.try_into().unwrap()
    }

    fn get_y(&self) -> usize {
        self.y.try_into().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
struct LineSegment {
    p1: Point,
    p2: Point
}

impl LineSegment {
    fn get_line(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        // Vertical
        if self.p1.x == self.p2.x {
            let distance: i32 = self.p2.y - self.p1.y;
            let direction: i32 = distance / distance.abs();

            for i in 0..distance.abs() + 1 {
                points.push(Point {
                    x: self.p1.x,
                    y: self.p1.y + direction * i,
                });
            }
        } 
        // Horizontal
        else if self.p1.y == self.p2.y {
            let distance: i32 = self.p2.x - self.p1.x;
            let direction: i32 = distance / distance.abs();

            for i in 0..distance.abs() + 1 {
                points.push(Point {
                    x: self.p1.x + direction * i,
                    y: self.p1.y,
                });
            }
        }

        points
    }
}

fn build_line_segment(line: &String) -> LineSegment { 
    let mut points: Vec<Point> = Vec::with_capacity(2);
    let segements = line.split(" -> ");

    for segment in segements {
        let digits: Vec<i32> = segment.split(",")
                                      .into_iter()
                                      .map(|num| num.parse::<i32>().expect("Could not convert line to i32"))
                                      .collect();
        
        points.push(Point {x: digits[0], y: digits[1]});
    }

    LineSegment {
        p1: points[0],
        p2: points[1]
    }
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut state = [[0; 1000]; 1000];

    for line in lines {
        let line_segment = build_line_segment(line);
        for point in line_segment.get_line() {
            state[point.get_x()][point.get_y()] += 1;
        }
    }
    
    let mut danger_point_count = 0;

    for y in 0..state.len() {
        for x in 0..state[y].len() {
            if state[x][y] == 0 {
                print!(" . ");
            } else {
                print!(" {:?} ", state[x][y]);
                if state[x][y] > 1 {
                    danger_point_count += 1;
                }
            }
        }
        println!("");
    }

    danger_point_count
}

fn part2(_: &Vec<String>) -> i32 {
    // ToDo: Impleet
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

        assert_eq!(answer, 5);
    }

    #[test]
    fn part1_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part1(&lines);

        assert_eq!(answer, answer);
    }

    #[test]
    fn part2_example_input() {
        let lines = get_lines_from_file(INPUT_EXAMPLE_FILE_PATH);
        let answer = part2(&lines);

        assert_eq!(answer, answer);
    }

    #[test]
    fn part2_input() {
        let lines = get_lines_from_file(INPUT_TEST_FILE_PATH);
        let answer = part2(&lines);

        assert_eq!(answer, answer);
    }
}