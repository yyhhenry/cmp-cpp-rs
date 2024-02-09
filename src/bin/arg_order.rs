use std::fmt::Display;

fn new_arg(title: impl AsRef<str>) -> String {
    let result = title.as_ref().to_string();
    println!("new_arg: {}", result);
    result
}
fn consume_args<T: Display>(v1: T, v2: T) {
    println!("consume_args: {} and {}", v1, v2);
}
fn main() {
    let v1 = new_arg("v1");
    let v2 = new_arg("v2");
    consume_args(v1, v2);
    // new_arg: v1
    // new_arg: v2
    // consume_args: v1 and v2

    consume_args(new_arg("v1"), new_arg("v2"));
    // new_arg: v1
    // new_arg: v2
    // consume_args: v1 and v2
}
