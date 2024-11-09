use std::io;

fn sum(first_variable: f32, second_variable: f32) -> f32 {
    first_variable + second_variable
}

fn difference(first_variable: f32, second_variable: f32) -> f32 {
    first_variable - second_variable
}

fn product(first_variable: f32, second_variable: f32) -> f32 {
    first_variable * second_variable
}

fn quotient(first_variable: f32, second_variable: f32) -> f32 {
    first_variable / second_variable
}

fn remainder(first_variable: f32, second_variable: f32) -> f32 {
    first_variable % second_variable
}

fn action(first_variable: f32, second_variable: f32, choice: u32) -> Option<f32> {
    match choice {
        1 => Some(sum(first_variable, second_variable)),
        2 => Some(difference(first_variable, second_variable)),
        3 => Some(product(first_variable, second_variable)),
        4 => Some(quotient(first_variable, second_variable)),
        5 => Some(remainder(first_variable, second_variable)),
        _ => None,
    }
}

fn main() {
    println!("\nWelcome to the Rust calculator!");
    println!("Choose your action: \n (1) sum \n (2) difference \n (3) product  \n (4) quotient \n (5) remainder");

    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).expect("Failed to read line");
    let nomber_of_choice: u32 = user_choice.trim().parse().expect("Please enter a number between 1 and 5.");

    println!("Write the first variable:");
    let mut first_variable = String::new();
    io::stdin().read_line(&mut first_variable).expect("Failed to read line");
    let first_variable: f32 = first_variable.trim().parse().expect("Please enter a valid number.");

    println!("Write the second variable:");
    let mut second_variable = String::new();
    io::stdin().read_line(&mut second_variable).expect("Failed to read line");
    let second_variable: f32 = second_variable.trim().parse().expect("Please enter a valid number.");

    match action(first_variable, second_variable, nomber_of_choice) {
        Some(result) => println!("The result of the action is: {result}"),
        None => println!("Please enter a valid choice."),
    };
}