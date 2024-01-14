use std::any::Any;

fn case1() {
    println!("Case 1:");
    let inferred = 0;
    let typed: i64 = 0;
    println!(
        "Is `inferred` the same type as `typed`? {}",
        inferred.type_id() == typed.type_id()
    );
    let _ = inferred + typed;
}
fn case2() {
    println!("Case 2:");
    let inferred = 0;
    let typed: i64 = 0;
    println!(
        "Is `inferred` the same type as `typed`? {}",
        inferred.type_id() == typed.type_id()
    );
}

fn main() {
    case1();
    case2();
}
