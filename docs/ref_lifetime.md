# ref_lifetime

## Rust

The `case1()` part will cause a compile error, because the temporary value will be dropped after the function call, but the reference is still in use.

Rust will not allow this to happen.

```rust
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
```

## C++

The `case1()` part will not cause a compile error, but it will cause some unexpected behavior.

This will be hard to debug, when the code becomes more complex.

```cpp
#include <iostream>
using std::string_literals::operator""s;

struct Data {
    std::string value;
    Data(std::string value) : value(value) {
        std::cout << "Creating " << value << std::endl;
    }
    ~Data() {
        std::cout << "Dropping " << value << std::endl;
    }
};

template <typename T>
T const &get_ref_return_ref(T const &x) {
    return x;
}
void case1() {
    auto &data = get_ref_return_ref(Data("case1"s));
    std::cout << "data: " << data.value << std::endl;

    // PANIC!!!

    // Output:
    // Creating case1
    // Dropping case1
    // data: case1
}
void case2() {
    auto origin_data = Data("case2"s);
    auto &data = get_ref_return_ref(origin_data);
    std::cout << "data: " << data.value << std::endl;

    // Output:
    // Creating case2
    // data: case2
    // Dropping case2
}

int main() {
    case1();
    case2();
    return 0;
}
```
