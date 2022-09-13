use loop_unwrap::ToOption;
use std::{collections::HashMap, io, num::ParseIntError, process};

fn main() {
    let is_happy = happy_number();

    println!("{}", is_happy);
}

fn check_string_parse_to_signed_int(user_input: &String) -> Result<u128, ParseIntError> {
    match user_input.trim().parse::<u128>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e),
    }
}

fn integers_to_digits(digit: u128) -> Vec<u128> {
    let mut digit_copy: u128 = digit;
    let mut digit_hashmap: Vec<u128> = Vec::with_capacity(10);
    while digit_copy > 0 {
        let num_to_vec: u128 = integer_squared(digit_copy % 10);
        digit_copy = digit_copy / 10;
        digit_hashmap.push(num_to_vec);
    }
    digit_hashmap
}

fn integer_squared(number: u128) -> u128 {
    number.pow(2)
}

fn happy_number() -> String {
    println!(
        "Input a positive integer from 0 to 255 and I will tell you if it is a 'happy number': "
    );
    let mut user_input = String::new();
    let mut num = loop {
        io::stdin().read_line(&mut user_input).unwrap_or_else(|e| {
            println!("There was an error: {}", e);
            process::exit(1)
        });
        let num = loop_unwrap::unwrap_continue!(
            check_string_parse_to_signed_int(&user_input),
            "Not a valid input, try again: "
        );
        break num;
    };

    let mut control_hashmap = HashMap::new();

    loop {
        num = integers_to_digits(num).iter().sum();
        if num == 1 {
            return format!("{} is a happy number!", user_input.trim());
        }
        if control_hashmap.contains_key(&num) {
            return format!("{} is not a happy number..", user_input.trim());
        }
        control_hashmap.insert(num, false);
    }
}
