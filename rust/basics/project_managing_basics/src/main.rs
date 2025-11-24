mod garden;                                 // used to define external modules: searches in ./garden.rs or ./garden/mod.rs
use crate::garden::garden;                  // used to import the item from the mentioned path
                                            // 'use' keyword is used to give shortcuts for the usage of items present in different modules;

mod internal_module {
    pub fn say_hi() {println!("Hi");}      // pub func() -> can be accessed from anywhere
    fn _say_bye() {println!("Bye");}         // private func() -> cannot be accessed from outside
}

fn main() {
    // accessing internal module
    internal_module::say_hi();
    //internal_module::say_bye();             // this will throw error saying say_bye() is private

    // accessing external module
    garden();                                 // accessing function present in different module (its a public func())
}
