trait IncrementOperator {
    fn pre_increment(&mut self) -> Self;
    fn post_increment(&mut self) -> Self;
}
impl IncrementOperator for i32 {
    fn pre_increment(&mut self) -> Self {
        println!("pre_increment x: {}", self);
        *self += 1;
        *self
    }
    fn post_increment(&mut self) -> Self {
        println!("post_increment x: {}", self);
        let y = *self;
        *self += 1;
        y
    }
}

fn case1() {
    let mut x = 1;
    let y = x.pre_increment() + x.post_increment();
    // Actually, this is not UB.
    // But this is still not recommended.

    println!("x: {}", x);
    println!("y: {}", y);
}

fn case2() {
    let mut x = 1;
    let tmp1 = x.pre_increment();
    let tmp2 = x.post_increment();
    let y = tmp1 + tmp2;
    println!("x: {}", x);
    println!("y: {}", y);
}

fn main() {
    case1();
    case2();
}
