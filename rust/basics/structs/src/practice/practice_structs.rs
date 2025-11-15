#[derive(Debug)]
pub struct Rectangle {
    pub width : u32,
    pub height : u32
}
impl Rectangle {
    pub fn new(width:u32, height:u32) -> Self {
        Self {
            width,
            height
        }
    }
    pub fn get_area (&self) -> u32 {
        self.width * self.height
    }
}

