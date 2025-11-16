#[derive(Debug)]
pub enum IPAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
pub enum IPAddrKindWithIP {
    V4(String),
    V6(String)
}
impl IPAddrKindWithIP {
    pub fn route(&self) {
        println!("Routing IP:{self:?}");
    }
}

#[derive(Debug)]
pub enum Message {
    Quit,
    ChangeColor(i32,i32,i32),
    Write(String),
    Move{x:i32,y:i32}
}
impl Message {
    pub fn display(&self) {
        println!("Enum : {:#?}",self);
    }
}
