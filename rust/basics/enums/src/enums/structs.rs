use crate::enums::enums::IPAddrKind;

#[derive(Debug)]
pub struct IP {
    pub ip : String,
    pub kind : IPAddrKind
}
impl IP {
    pub fn new(ip: &str, kind:IPAddrKind) -> Self {
        Self {
            ip : ip.to_string(),
            kind
        }
    }
    pub fn route(&self) {
        println!("Routing ip : {} of kind: {:?}", self.ip, self.kind)
    }
}
