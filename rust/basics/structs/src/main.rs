use crate::structs::struct_basics;
use crate::practice::practice;
use crate::methods::methods;

mod structs;
mod practice;
mod methods;

fn main() {
    // Struct Basics:
    println!("======================================== B A S I C S ============================================");
    struct_basics();


    println!("======================================= P R A C T I C E =========================================");
    // practice structs
    practice();


    
    println!("======================================== M E T H O D S =========================================");
    // practice structs
    methods();
}
