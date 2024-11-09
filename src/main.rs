use std::io;

fn sum(first_variable_plus_second_variable: f32) { [
    first_variable, action, second_variable
]
}

fn defference(first_variable_minus_second_variable: f32) { [
    first_variable, action, second_variable
]
}

fn product(first_variable_division_second_variable: f32) { [
    first_variable, action, second_variable
]
}

fn quotient(first_variable_multiplication_second_variable: f32) { [
    first_variable, action, second_variable
]
}

fn remainder(first_variable_remainder_second_variable: f32) { [
    first_variable, action, second_variable
]
}

fn action(self: first_variable, second_variable: f32, choise: u32) -> Option<f32>{
    match choise {
        1 => Some(sum(result_of_action)),
        2 => Some(defference(result_of_action)),
        3 => Some(product(result_of_action)),
        4 => Some(quotient(result_of_action)),
        5 => Some(remainder(result_of_action)),
        _ => None,
    }
}

fn main(){
    println!("\n Wellcome to calculator on Rust!");
    println!("Choose your action: \n (1) sum \n (2) defference \n (3) product  \n (4) quotient \n (5) remainder");

    let mut user_choise = String::new();

    io::stdin().read_line(&mut user_choise).unwrap();

    let n_coise = user_choise
        .trim()
        .parse::<u32>()
        .expect("Please type a number.");
    
    let mut action = String::new();
        _trim()
        .parse::<u32>()
        .expect("Your choise: ");

    let mut first_variable = String::new();
        _trim()
        .parse::<f32>()
        .expect("Write a first variable.");
    
    let mut second_variable = String::new();
        _trim()
        .parse::<f32>()
        .expect("Write a second variable.");

    io::stdin().read_line(&mut action).unwrap();

    io::stdin().read_line(&mut first_variable).unwrap();

    io::stdin().read_line(&mut second_variable).unwrap();

    let mut result_of_action = String::new();

    let result_of_action = result_of_action
        _trim()
        .parse::<f32>()
        .expect("Please type a number of action.")

    match convert(result_of_action, n_coise){
        Some(result_of_action) => println!("The result of action: {result_of_action}")
        Nano => println("Please wtite correct nomber of choise.")
    };
}