// Compile Error:
// cannot borrow `arr` as mutable because it is also borrowed as immutable mutable borrow occurs here
// fn case1() {
//     let mut arr = vec![1, 2, 3];
//     let cap = arr.capacity();
//     let first_ref = &arr[0];
//     for _ in arr.len()..=cap {
//         arr.push(0);
//     }
//     println!("first_ref: {}", first_ref);
//     println!("arr: {:?}", arr);
// }
fn case2() {
    let mut arr = vec![1, 2, 3];
    let cap = arr.capacity();
    for _ in arr.len()..=cap {
        arr.push(0);
    }
    let first_ref = &arr[0];
    println!("first_ref: {}", first_ref);

    // Output:
    // first_ref: 1
}
fn main() {
    // case1();
    case2();
}
