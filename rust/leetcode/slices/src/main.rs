fn main() {
    let str = String::from("Hello World"); 
    let res = get_first_space(&str);
    let first_word = get_first_word(&str);

    println!("First space in '{str}' occurs at index : {res}");
    println!("First word in '{str}' is : '{first_word}'");
}

fn get_first_space(s:&String) -> usize {
    let chars = s.as_bytes();

    for (i, &char) in chars.iter().enumerate() {
        if char == b' ' {
            return i;
        }
    }
    
    s.len()
}

fn get_first_word(s:&String) -> &str {
    let chars = s.as_bytes();
    for (i, &char) in chars.iter().enumerate() {
        if char == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
