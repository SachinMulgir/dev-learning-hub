fn main() {
    // Functions follow few basic rules:
    //1. In Rust, functions and variables are defined using snake casing separated by underscore '_'.
    //2. Functions are not executed automatically, we need to make explicit calls for a function to perform.

    println!("Hi, This is the main() function");

    //calling my_function();
    my_function();

    // function to add 2 numbers:
    add(1,2);

    // funtion to return product of two numbers:
    let product = product(2,3);
    println!("{} * {} = {}", 2,3,product);

    // nested function call:
    nested_function1();

    // function to check odd or even
    let num = 6;
    is_even(num);
}

fn my_function() {
    println!("This is my_function().");
}

fn add(a:i32, b:i32) {
    println!("{} + {} = {}",a,b,a+b);
}

fn product(a:i32, b:i32) -> i32 {
    return a*b
}

fn nested_function1() {
    nested_function2();
}

fn nested_function2() {
    println!("this is nested function 2 being called from nested functon1");
}

fn is_even(num : i32) {
    if num%2==0 {
        println!("{num} is even.");
    } else {
        println!("{num} is odd.");
    }
}
