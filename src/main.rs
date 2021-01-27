use std::io;

fn main() {
    let mut user_input: String = String::new();
    let mut integer_one: i32; 
    let mut integer_two: i32;

    loop {
        println!("Please enter value 1: ");
        io::stdin().read_line(&mut user_input).unwrap();
        integer_one = user_input.trim().parse::<i32>().unwrap();
        user_input.clear();

        println!("Please enter value 2: ");
        io::stdin().read_line(&mut user_input).unwrap();
        integer_two = user_input.trim().parse::<i32>().unwrap();
        user_input.clear();

        println!("Operation (*,-,/,+,exit): ");
        io::stdin().read_line(&mut user_input).unwrap();
        
        if user_input.trim() == "*" {
            println!("Product of two: {}", product_of_numbers(integer_one, integer_two));
        } else if user_input.trim() == "-" {
            println!("{} - {}: {}", integer_one, integer_two, substract_from_number(integer_one, integer_two));
        } else if user_input.trim() == "+" {
            println!("Sum: {}", add_two_numbers(integer_one, integer_two));
        } else if user_input.trim() == "/" {
            println!("{} / {}: {}", integer_one, integer_two, divide_num_by_num(integer_one, integer_two));
        } else if user_input.trim() == "exit" {
            break;
        }

        user_input.clear();
    }
}

fn add_two_numbers(number_one: i32, number_two: i32) -> i32 {
    number_one + number_two
}

fn substract_from_number(number_from: i32, number_to: i32) -> i32 {
    number_from - number_to
}

fn product_of_numbers(number_one: i32, number_two: i32) -> i32 {
    number_one * number_two
}

fn divide_num_by_num(number_to: i32, number_from: i32) -> i32 {
    number_to / number_from
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}