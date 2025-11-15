mod practice_structs;
use crate::practice::practice_structs::Rectangle;

pub fn practice() {
    // we will be seeing that when can we club the properties to make a struct:
    
    // Calculate area of a rectangle:
    // 1. using width and height as independent params:
    let width = 10;
    let height = 20;

    let area = calculate_area(width, height);
    println!("\n\nArea calculation using manually defining width and height : {area}");


    // 2. using width and height as members of tuple

    let dimensions = (10,20);
    let area = calculate_area_2(dimensions);

    println!("\n\nArea calculation using tupes for defining width and height : {area}");


    // 3. using struct to define width and height of a rectangle
    
    let rectangle = Rectangle::new(10,20);
    let area = calculate_area(rectangle.width, rectangle.height);
    let _area_2 = Rectangle::get_area(&rectangle);

    println!("\n\nArea calculation using struct for defining width and height : {area}");


}

fn calculate_area (width: u32, height: u32) -> i128 {
    width as i128 * height as i128
}

fn calculate_area_2 (dimensions : (i32,i32)) -> i128 {
    let width = dimensions.0;
    let height = dimensions.1;
    width as i128 * height as i128
}
