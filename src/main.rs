use std::io::stdin;

fn get_stripped_user_input(user_input: &mut String) {
    stdin().read_line(user_input).expect("Failed to read user input");
    user_input.retain(|c| !c.is_whitespace());
}

fn main() {
    let mut user_input = String::new();
    println!("Enter the number of the day you would like to run");
    get_stripped_user_input(&mut user_input);
    match user_input.as_str() {
        "1" => lib::day1::compute_solution(),
        "2" => lib::day2::compute_solution(),
        "3" => lib::day3::compute_solution(),
        "4" => lib::day4::compute_solution(),
        "5" => lib::day5::compute_solution(),
        "6" => lib::day6::compute_solution(),
        _ => println!("User entered invalid day"),
    }
}
