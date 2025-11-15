use std::io;

fn main() {
    // This will cover all the basic programs to be done in rust:

    //1. Check if the number is palindrome or not.
    //2. Print number from 1 to n.
    //3. Check if the given number is Armstrong or not.
    //4. Check if the given number is a prime number or not.

    // Solutions:
    // Make proper functions for the same.
    // Call different files for the solutions.

    // Solutions:
    let mut input = String::new();
    let mut choice = String::new();

    println!("Enter the program to use:");
    println!("1. Palindrome Number.");
    println!("2. Armstrong Number.");
    println!("3. Prime Number.");
    println!("4. Odd number from 1 to n.");
    println!("5. Even number from 1 to n.");

    println!("Enter Choice:");
    io::stdin()
        .read_line(&mut choice)
        .expect("Unable to get user input for option!!");

    println!("\nEnter n : ");
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to get user input!!");

    let input_num: u32 = input
        .trim()
        .parse()
        .expect("Unable to parse input number into integer");
    let choice_num: u32 = choice
        .trim()
        .parse()
        .expect("Unable to parse choice number into integer");

    match choice_num {
        1 => palindrome_number(&input_num),
        2 => armstrong_number(&input_num),
        3 => prime_number(&input_num),
        4 => odd_number_till_n(&input_num),
        5 => even_number_till_n(&input_num),
        _ => {
            println!("Invalid choice !!");
        }
    }
}

fn palindrome_number(num: &u32) {
    let mut temp = *num;
    let mut new_num = 0;
    while temp > 0 {
        let remain = temp % 10;
        new_num = new_num * 10 + remain;
        temp = temp / 10;
    }

    if new_num == *num {
        println!("Number is Palindrome");
    } else {
        println!("Number is not Palindrome");
    }
}

fn armstrong_number(num: &u32) {
    let mut temp = *num;
    let mut new_num = 0;
    while temp > 0 {
        let remain = temp % 10;
        new_num = new_num + (remain * remain * remain);
        temp = temp / 10;
    }

    if new_num == *num {
        println!("Number is Armstrong");
    } else {
        println!("Number is not Armstrong");
    }
}

fn prime_number(num: &u32) {
    let temp = *num;
    let mut count = 0;

    for i in 2..temp {
        if temp % i == 0 {
            count += 1;
        }
    }

    if count > 0 {
        println!("Number is not a Prime number.")
    } else {
        println!("Number is a prime number.");
    }
}

fn odd_number_till_n(num: &u32) {
    let temp = *num;
    for i in 1..=temp {
        if i % 2 != 0 {
            print!("{i}, ");
        }
    }
}

fn even_number_till_n(num: &u32) {
    let temp = *num;
    for i in 1..=temp {
        if i % 2 == 0 {
            print!("{i}, ");
        }
    }
}
