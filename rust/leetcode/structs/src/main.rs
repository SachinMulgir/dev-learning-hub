mod structures;

use crate::structures::structs::*;

fn main() {

    // two ways to instantiate a struct:

    //1. Create a impl function with the struct:
    let person1 = Person::new("Sachin", 28, "smulgir636@gmail.com", 'M');
    
    //2. Explicitly create a instance of the struct.
    let person2 = Person {
        name: "Sachin2".to_string(),
        age: 34,
        email: "sachin2@gmail.com".to_string(),
        gender: 'M'
    };

    println!("Person1 : {:?}\nPerson2 : {:?}", person1, person2);

    // Accessing particular values: using dot(.) operator
    let person1_name = person1.name;
    let person2_name = person2.name;

    println!("Person1 Name : {person1_name} && Person2 Name : {person2_name}");

    let person3 = Person {
        name : String::from("Person3"),
        ..person2
    };


    // Structs without data or field names:
    let black = Color(0,0,0);
    let origin = Origin(0,0,0);

    


    
}
