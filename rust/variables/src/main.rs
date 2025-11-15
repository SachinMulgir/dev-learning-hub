fn main() {
    // variable (immutable)
    let age = 25;
    println!("value of age : {age}");


    // mutable variable
    let mut mut_age = 25;
    println!("value of mutable age : {mut_age}");
    mut_age = 26;
    println!("value of mutable age after change : {mut_age}");

    // const variable:
    // By default: It should be in upper-case
    // We cannot use 'mut' keyword with const
    // We must give annotation for the data-type of the constant.
    // Can be defined in any scope.
    // Can be used with fixed value, should not be defined at runtime.
    const PI: f64 = 3.14;
    println!("value of const PI: {PI}");

    // Shadowing:
    let x = 3;
    let x = x + 2;   //Shadowing
    println!("Shadowing variable x : {x}");
}
