pub struct Student {
    pub name: String,
    pub class: String,
    pub marks: u32
}

impl Student {
    // Associated Function:
    pub fn get_student(name: String, class: String, marks:u32) -> Self {
        Self {
            name,
            class,
            marks
        }
    }
 
    // Associated function + Method:
    pub fn get_student_details (&self) -> (&String, &String, u32) {
        (&self.name, &self.class, self.marks)
    }
}
