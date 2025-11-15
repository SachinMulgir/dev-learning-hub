fn main() {
    // Integer
    let a = 2;
    
    // Float
    let b = 2_f32 + 2.2;


    // bool
    let v1 = true;
    let f1:bool = false;

    // char
    let c1 = 'c';
    let c2: char = '5';

    println!("char1 : {c1} -> char2 : {c2}");

    println!("tuple: {:?} -> v1:{v1} -> v2:{v2} -> v3:{v3}",a);

    // array
    let arr = [3;5];
    let val1 = arr[0];
    println!("arr: {:?} -> val1 : {val1}",arr);


    let mut arr = [3;5];
    arr = [4;5];
}
