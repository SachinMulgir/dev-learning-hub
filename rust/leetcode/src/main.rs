mod questions;
use questions::is_vec_sorted::*;

fn main() {
    //let nums: Vec<i32> = vec![3,4,5,1,2]; //true
    //let nums: Vec<i32> = vec![2,1,3,4]; //false
    let nums: Vec<i32> = vec![1,2,3]; //true
    //let nums: Vec<i32> = vec![6, 10, 6]; //true
    //let nums: Vec<i32> = vec![10,1,1,10]; //true
    //let nums: Vec<i32> = vec![5,1,5,1]; //false

    // 1. to check whether a vector is sorted or not: if it is rotated after sorting or not.
    println!("Vector Sorted: {}", check_2(nums));

    // 2.
}
