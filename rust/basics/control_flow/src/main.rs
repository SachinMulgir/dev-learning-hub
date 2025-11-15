fn main() {
    // Conditionals

    // if else
    let x = 5;

    if x < 10 {
        println!("True case");
    } else {
        println!("False case");
    }

    println!("\n");

    // Loops:
    // 1. loop
    // 2. while loop
    // 3. for loop

    // 1. loop (infinite loop):
    let mut count = 5;
    loop {
        println!("count : {count}");
        count -= 1;
        if count == 0 {
            break;
        }
    }
    println!("\n");


    // nested loop:
    let mut outer_count = 5;
    'outer_loop: loop {
        let mut inner_count = 5;

        println!("outer print : outer_count : {outer_count} && inner_count:{inner_count}");

        loop {
            println!("inner print : outer_count : {outer_count} && inner_count:{inner_count}");
            inner_count -= 1;

            if outer_count == 4 {
                break 'outer_loop;
            }

            if inner_count == 2 {
                break;
            }
        }
        outer_count -= 1;
        println!("\n");
    }

    // for loop:
    let arr = [1,2,3,4,5];
    println!();
    for val in arr {
        print!("{val} -> ");
    }

    println!();
    for i in 0..arr.len() {
        print!("{} -> ",arr[i]);
    }


    println!("\n");

    let str = "Sachin";
    let char1 = str.chars().nth(0).expect("Unable to get char at index");
    println!("{}",char1);








    // ASSIGNMENTS:

    // 1. convert celcius to farenheit
    // 2. generate nth fibonacci number
    // 3. Print lyrics of christmas songs: 'The Twelve Days of Christmas'.



    // Celcius to farenheit: (c * 9/5) + 32;
    println!("\n");
    celcius_to_farenheit(37);

    // generate nth fibonacci number:
    println!("\n");
    nth_fibonacci(6);

    // print lyrics of the christmas song:
    println!("\n");
    christmas_song();

}

fn celcius_to_farenheit (c: i32) {
    let farenheit = (c * 9/5) + 32;
    println!("Celcius: {c} -> Farenheit: {farenheit}");
}

fn nth_fibonacci(n: i32) {
    let mut tmp1;
    let mut tmp2 = 1;

    let mut ans = 0;
    print!("{ans} -> ");
    for _i in 0..n-1 {
        tmp1 = tmp2;
        tmp2 = ans;
        ans = tmp1 + tmp2;
        print!("{ans} -> ");
    }
    println!("\n{n}th fibonacci number : {ans}");
}

fn christmas_song() {
    let prefix_str = "The {day} day of Christmas,\nMy true love sent to me\n";
    let mut suffix_str = String::new();

    for i in 1..=12 {
        let temp_prefix ;
        let temp_suffix;
        match i {
            1 => {
                temp_prefix = prefix_str.replace("{day}", "first");
                temp_suffix = "A partridge in a pear tree".to_string();
            },
            2 => {
                temp_prefix = prefix_str.replace("{day}", "second");
                temp_suffix = "Two turtle doves, and".to_string();
            },
            3 => {
                temp_prefix = prefix_str.replace("{day}", "third");
                temp_suffix = "Three French hens,".to_string();
            },
            4 => {
                temp_prefix = prefix_str.replace("{day}", "fourth");
                temp_suffix = "Four colly birds,".to_string();
            },
            5 => {
                temp_prefix = prefix_str.replace("{day}", "fifth");
                temp_suffix = "Five gold rings,".to_string();
            },
            6 => {
                temp_prefix = prefix_str.replace("{day}", "sixth");
                temp_suffix = "Six geese a-laying,".to_string();
            },
            7 => {
                temp_prefix = prefix_str.replace("{day}", "seventh");
                temp_suffix = "Seven swans a-swimming,".to_string();
            },
            8 => {
                temp_prefix = prefix_str.replace("{day}", "eighth");
                temp_suffix = "Eight maids a-milking,".to_string();
            },
            9 => {
                temp_prefix = prefix_str.replace("{day}", "ninth");
                temp_suffix = "Nine drummers drumming,".to_string();
            },
            10 => {
                temp_prefix = prefix_str.replace("{day}", "tenth");
                temp_suffix = "Ten pipers piping,".to_string();
            },
            11 => {
                temp_prefix = prefix_str.replace("{day}", "eleventh");
                temp_suffix = "Eleven ladies dancing,".to_string();
            },
            12 => {
                temp_prefix = prefix_str.replace("{day}", "twelfth");
                temp_suffix = "Twelve fiddlers fiddling,".to_string();
            },
            _ => {
                break;
            }
        }
        suffix_str = temp_suffix + "\n" + &suffix_str;
        println!("{}{}\n",temp_prefix,suffix_str);
    }

    
}
