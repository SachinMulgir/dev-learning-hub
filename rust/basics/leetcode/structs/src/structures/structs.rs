#[warn(unused)]
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub gender: char,
}

impl Person {
    pub fn new(name: &str, age: u32, email: &str, gender: char) -> Self {
        Self {
            name: name.to_string(),
            age: age,
            email: email.to_string(),
            gender: gender,
        }
    }
}

pub struct Color (
    pub i32,
    pub i32,
    pub i32
);

pub struct Origin (
    pub i32,
    pub i32,
    pub i32
);
