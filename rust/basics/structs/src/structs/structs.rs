// Defining a Struct:
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub email: String,
    pub age: i32,
    pub gender: char,
}

// associated funtions with struct: Person
impl Person {
    pub fn get_new_person(name: String, email: String, age: i32, gender: char) -> Self {
        Self {
            name,
            email,
            age,
            gender,
        }
    }

    pub fn get_new_person_2(name: String, email: String, age: String, gender: String) -> Self {
        let age: i32 = age
            .trim()
            .parse()
            .expect("Unable to parse user input age into i32");
        let gender: char = gender
            .trim()
            .parse()
            .expect("Unable to parse user input gender into character");
        Self {
            name,
            email,
            age,
            gender,
        }
    }

    pub fn default() -> Self {
        Self {
            name: String::default(),
            email: String::default(),
            age: 0,
            gender: 'M',
        }
    }
}
