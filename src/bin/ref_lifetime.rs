struct Data {
    pub value: String,
}
impl Data {
    pub fn new(value: String) -> Self {
        println!("Creating {}", value);
        Self { value }
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        println!("Dropping {}", self.value);
    }
}

fn get_ref_return_ref<T>(x: &T) -> &T {
    x
}
// Compile Error:
// temporary value dropped while borrowed creates a temporary value which is freed while still in use
// fn case1() {
//     let data = get_ref_return_ref(&Data::new(String::from("case2")));
//     println!("data: {}", data.value);
// }
fn case2() {
    let data = Data::new(String::from("case2"));
    let data = get_ref_return_ref(&data);
    println!("data: {}", data.value);

    // Output:
    // Creating case2
    // data: case2
    // Dropping case2
}

fn main() {
    // case1();
    case2();
}
