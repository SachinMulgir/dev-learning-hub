fn main() {
    // case1:
    let x = 10;
    let y = x;

    println!("x = {x} && y = {y}");

    // case2:
    let str1 = String::from("Sachin");
    let str2 = &str1;

    println!("str1 = {str1} && str2 = {str2}");


    // Move and Borrow
    // move : When we move the ownership of an object to some other variable.
    // borrow : When we pass the reference of the object to some other variable. Ownership in this case does not get moved, only the new variable starts pointing to the owner of the object.


    // MOVE
    let str1 = String::from("Sachin");
    let len1 = calculate_length_with_move(str1);
    // here we have passed the complete 'str1' variable as the argument
    // this will move the value to the function scope and the ownership of the String object is moved inside the function.
    // the string object will get dropped as soon as the function ends, as the owner of the object will be dropped.
    // println!("str1 : {str1}");      -- uncomment this to check the error.

    println!("length post move example: {len1}");

    // BORROW:
    let str2 = String::from("Borrow");
    let len2 = calculate_length_with_borrow(&str2);
    // here we have passed the reference of the 'str2' instead of the complete object as an argument to the function.
    // this will not move the ownership of the object from 'str2' to the function, it will only pass the reference to the 'str2' variable, which will point to the string object in the heap
    // the string object will not get dropped once the function is over, since we never gave the ownership of the object to the function.
    // we'll still be able to use the object
    // println!("str2: {str2}");
    println!("length of str2 : {str2} -> post borrow example: {len2}");

    // MUTABLE REFERENCING:
    let mut str3 = String::from("Mutable borrow");
    let len3 = calculate_length_with_mutable_borrow(&mut str3);
    // here also we have passed the reference, instead of the object
    // we have passed it as mutable, which means we will be able to expand or shrink  the string as per our need in the function.
    // here we are restoring the ownership to the initial variable.

    println!("length of string: {str3} -> post mutable borrow example is : {len3}");




    // Major Rules of mutability:
    let str_val = String::from("Sachin");

    let ref1 = &str_val;
    let ref2 = &str_val;

    println!("two refs are -> ref1 : {ref1} & ref2 : {ref2}");

    // Here we are able to create two immutable references to a value.
    // B U T
    // -----------   IF YOU HAVE A MUTABLE REFERENCE TO A VALUE -> YOU CAN HAVE NO OTHER REFERENCES TO THAT VALUE ------------------
    // Why two mutable references are not allowed?
    // To prevent DATA_RACE condition which occuers due to 3 behaviours:
    // 1. Two or more pointers access the same data at the same time.
    // 2. At least one of the pointer is being used to write the data
    // 3. There's no mechanism being used to synchronize access to the data
    //
    // NOTE: MEMORY SAFETY
    //

    // below code is a failure:
    //
    // let mut str_val = String::from("Sachin");

    // let ref1 = &mut str_val;
    // let ref2 = &mut str_val;

}

fn calculate_length_with_move(s:String) -> usize {
    s.len()
} 

fn calculate_length_with_borrow(s: &String) -> usize {
    s.len()
}

fn calculate_length_with_mutable_borrow(s : &mut String) -> usize {
    s.push_str(", Added String");
    s.len()
}
