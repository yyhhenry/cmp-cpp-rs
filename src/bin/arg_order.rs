use std::fmt::Display;

fn new_arg(title: impl AsRef<str>) -> String {
    let result = title.as_ref().to_string();
    println!("new_arg: {}", result);
    result
}
fn consume_value<T: Display>(v1: T, v2: T) {
    println!("consume_value: {} and {}", v1, v2);
}
fn main() {
    let v1 = new_arg("v1");
    let v2 = new_arg("v2");
    consume_value(v1, v2);
    // Arg: v1
    // Arg: v2
    // Arg order: v1 and v2

    consume_value(new_arg("v1"), new_arg("v2"));
    // Arg: v1
    // Arg: v2
    // Arg order: v1 and v2
}
