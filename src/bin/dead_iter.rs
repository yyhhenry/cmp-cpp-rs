// Compile Error:
// cannot borrow `arr` as mutable because it is also borrowed as immutable mutable borrow occurs here
// fn case1() {
//     let mut arr = vec![1, 2, 3];
//     let cap = arr.capacity();
//     let mut iter = arr.iter();
//     for _ in arr.len()..=cap {
//         arr.push(0);
//     }
//     println!("first: {}", iter.next().unwrap());
// }
fn case2() {
    let mut arr = vec![1, 2, 3];
    let cap = arr.capacity();
    for _ in arr.len()..=cap {
        arr.push(0);
    }
    let mut iter = arr.iter();
    println!("first: {}", iter.next().unwrap());

    // Output:
    // first: 1
}
fn main() {
    // case1();
    case2();
}
