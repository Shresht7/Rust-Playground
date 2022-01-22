//  Enumerations allow you to define a type by enumerating all its possible variants.
//  enumerations are declared using the enum keyword

//  The IpAddrKind enum lists the two kind of IP Addresses
enum IpAddrKind {
    V4,
    V6,
}

#[allow(unused_variables)]
fn instantiate_ip() {
    //  We can create instances of each of the two variants like so
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;
    //  Note that the variants of the enum are namespaced under its identifier.

    //  A function can take any IpAddrKind, and we can call it with any variant
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

#[allow(unused_variables)]
fn route(ip_kind: IpAddrKind) {}

//  We can also store data inside each enum variant
enum IpAddr {
    V4(String),
    V6(String),
}

#[allow(unused_variables)]
fn enums_with_data() {
    //  Here we attach data to each variant of the enum directly.
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

//  Each variant can have different types and amounts of associated data.
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//  Quit has no data associated with it at all
//  Move has named fields like a struct
//  Write includes a single String
//  ChangeColor includes three i32 values.

//You can also define methods for enums!
#[allow(dead_code)]
impl Message {
    fn call(&self) {
        println!("Hello!")
    }
}
fn main() {
    instantiate_ip();
    enums_with_data();
}
