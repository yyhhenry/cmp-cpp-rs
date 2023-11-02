struct Data {
    pub value: i32,
}
impl Data {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

/**
 * Expected to act like `out := max { 0, a, b }` in math.
 */
fn positive_max(out: &mut Data, a: &Data, b: &Data) {
    out.value = 0;
    out.value = out.value.max(a.value);
    out.value = out.value.max(b.value);
}

// Compile Error:
// cannot borrow `a` as immutable because it is also borrowed as mutable immutable borrow occurs here
// fn case1() {
//     let mut a = Data::new(1);
//     let b = Data::new(-1);
//     positive_max(&mut a, &b, &a);
//     println!("a: {}", a.value);
// }

fn case2() {
    let mut a = Data::new(1);
    let b = Data::new(-1);
    let c = Data::new(1);
    positive_max(&mut a, &b, &c);
    println!("a: {}", a.value);

    // Output:
    // a: 1
}

fn main() {
    // case1();
    case2();
}
