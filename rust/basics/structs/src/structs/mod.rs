use crate::structs::structs::Person;

use std::io;
mod structs;

pub fn struct_basics() {
    //  ======================    Creating a Person struct: ========================================
    // Below are defined a multiple ways to create a struct:

    // 1. Creating a default struct using associated default() function with Person struct:
    let person_1 = Person::default();
    println!("Default Person : {:#?}\n\n", person_1);

    // 2. Define vaiables and push to create a variable from get_new_person() associated function:
    let name = String::from("Person2 Name");
    let email = String::from("person2@email.com");
    let age = 25;
    let gender = 'F';

    let person_2 = Person::get_new_person(name, email, age, gender);
    println!("Person using defined variables: {:#?}\n\n", person_2);

    // 3. Ask user to give input and create a Person using get_new_person_2() associated function:
    println!("Person creation using user input:");
    let person_3 = create_person();
    println!("Person using user input details : {:#?}\n\n", person_3);

    // ========================================   Accessing person struct details
    // We can use (.) to individually access each property of a struct.

    println!("Name of Person 2 is : {}", person_2.name);
    println!("Email of Person 2 is : {}", person_2.email);
    println!("Age of Person 2 is : {}", person_2.age);
    println!("Gender of Person 2 is : {}", person_2.gender);
}


fn create_person() -> Person {
    let mut name: String = Default::default();
    let mut email: String = Default::default();
    let mut age: String = Default::default();
    let mut gender: String = Default::default();

    println!("Enter the person details to be added:");
    println!("Enter name:");
    io::stdin()
        .read_line(&mut name)
        .expect("ERROR | Getting user input for name.");

    println!("Enter email:");
    io::stdin()
        .read_line(&mut email)
        .expect("ERROR | Getting user input for email.");

    println!("Enter age:");
    io::stdin()
        .read_line(&mut age)
        .expect("ERROR | Getting user input for age.");

    println!("Enter gender:");
    io::stdin()
        .read_line(&mut gender)
        .expect("ERROR | Getting user input for gender.");

    Person::get_new_person_2(name, email, age, gender)
}
