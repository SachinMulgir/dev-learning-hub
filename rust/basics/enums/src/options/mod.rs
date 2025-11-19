pub fn options() {
    println!("\n O P T I O N S \n");

    // Option <T> is a prelude in rust and can be used exactly same as a enum:

    let op_val = Some(1);                     // here the compiler auto infers the value to be a i32 as we are passing numeric value inside some()
    let op_val_2 = Some('s');                // here the compilet auto infers the value to be a char as we are passing a character inside some()


    // null value:
    let no_value : Option<i32> = None;                    // Here we have to explicitly annotate the value as the rust compiler wont be able to understand the datatype we are expecting here if any value to be given to the variable.
    // let no_value = None;                               // Uncomment this to see the error. It will ask to explicitly annotate the type.


    // Rust do not allow standard operations on Option type:

    let x = 5;
    let y = Some(5);

    // the below operation is not permitted because rust does not allow Option<T> type, as it is not necessary to be a valid value and need to explicitly handle it before using:
    //let sum = x + y;

    // therefore, explicitly change Option<T> -> T
    let y_final = match y {
        Some(v) => v,
        None => panic!("Value in variable 'y' is not valid.")
    };

    let sum = x + y_final;                          // This is valid as we have made sure explicitly that y_final is a valid i32 value and there is no possibility of it being in null.

}
