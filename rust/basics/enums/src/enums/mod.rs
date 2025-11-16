mod enums;
mod structs;
use crate::enums::enums::*;
use crate::enums::structs::IP;

pub fn enums() {
    // 1. usage of enums : routing using plain definition of IP address.
    manual_defn();

    // Problem with step 1 : Here ip and kind are independent and free variables and do not show any relation.

    // 2. Using struct to define IP with related fields as (addr + kind)
    struct_defn();

    // Problem with step 2 : Here the code becomes very verbose and we have to link multiple things together (struct -> enum)

    // 3. Using enums only to store its respective IP_addr along with the kind:
    enum_defn();

    // Temp example for enums with associated values:
    enum_message();
    
}

// 1. usage of enums : routing using plain definition of IP address.
fn manual_defn() {
    let ip = "192.1.2.3";
    routing_ip(ip, IPAddrKind::V4);                    // IPAddrKind used to define the type of IP.
}

fn routing_ip(ip: &str, kind: IPAddrKind) {
    println!("Routing IP:{ip} of kind: {kind:?}");
}

// 2. Using struct to define IP with related fields as (addr + kind)
fn struct_defn() {
    let ip_addr = IP::new("1.2.3.4", IPAddrKind::V4);
    let ip_addr_2 = IP::new("9.8.7.6", IPAddrKind::V6);

    // routing the ip_addr using a method defined with IP structs:
    ip_addr.route();
    ip_addr_2.route();
}

// 3. Using enums only to store its respective IP_addr along with the kind:
fn enum_defn() {
    let ip_addr = IPAddrKindWithIP::V4("1.2.3.4".to_string());
    ip_addr.route();
}

// Temp example for practice
fn enum_message() {
    let quit_msg = Message::Quit;
    let move_msg = Message::Move { x: 3, y: 4 };
    let write_msg = Message::Write(String::from("Hello"));
    let color_change_msg = Message::ChangeColor(255,0,0);

    println!("My Quit Msg : {quit_msg:?}");
    println!("My Move Msg : {move_msg:?}");
    println!("My Write Msg : {write_msg:?}");
    println!("My Color Change Msg : {color_change_msg:?}");
}
