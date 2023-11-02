struct Data {
    value: i32,
}
impl Data {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

fn max_data<'a>(a: &'a Data, b: &'a Data) -> &'a Data {
    if a.value > b.value {
        a
    } else {
        b
    }
}

/**
 * Expected to act like `a.value = b.value`.
 */
fn unusual_assignment(a: &mut Data, b: &Data) {
    a.value = 0;
    a.value = b.value;
}

// Compile Error:
// cannot borrow `a` as immutable because it is also borrowed as mutable immutable borrow occurs here
// fn case1() {
//     let mut a = Data::new(2);
//     let b = Data::new(1);
//     unusual_assignment(&mut a, max_data(&a, &b));
//     println!("a: {}", a.value);
// }

fn case2() {
    let mut a = Data::new(1);
    let b = Data::new(2);
    let c = Data::new(3);
    unusual_assignment(&mut a, max_data(&b, &c));
    println!("a: {}", a.value);

    // Output:
    // a: 3
}

fn main() {
    // case1();
    case2();
}
