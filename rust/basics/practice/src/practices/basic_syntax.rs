pub fn basic_syntax() {
    // 1. func() to take 2 nums and give sum:
    sum(1,2);

    // 2. Printing hello world on console:
    say_hello();

    // 3. loop 1 to 10:
    loop_till_10();

}


fn sum(a:i32, b:i32) {
    println!("sum = {}", a+b);
}

fn say_hello() {
    println!("Hello World.");
}

fn loop_till_10() {
    // we have 3 types of loops:
    let mut num = 1;
    println!("Print 1 to 10:");
    println!("loop");
    loop {
        print!("{num} ");
        if num == 10 {break;}
        num+=1;
    }

    num = 1;
    println!("\n\nWhile loop:");
    while num <= 10 {
        print!("{num} ");
        num+=1;
    }

    println!("\n\nFor loop:");
    for num in 1..=10 {
        print!("{num} ")
    }
    println!();
}

