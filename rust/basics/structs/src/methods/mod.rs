mod method_structs;

use crate::methods::method_structs::Student;

pub fn methods() {
    // NOTE: ALL METHODS ARE ASSOCIATED FUNCTIONS BUT ALL ASSOCIATED FUNCTIONS ARE NOT METHODS

    let name = "Sachin".to_string();
    let class = "Surya Fintech".to_string();
    let marks = 90;

    // below function get_student() is a associated funtion => called on the type Student. => using Type::func()
    let student_1 = Student::get_student(name, class, marks);


    // below funtion get_student_details() is a method => called on the instance of the type Student. => using instance.func()
    let (name, class, marks) = student_1.get_student_details();

    println!("name:{name},\nclass:{class}\nmarks:{marks}");

}