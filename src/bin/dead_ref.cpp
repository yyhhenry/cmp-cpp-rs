#include <iostream>
#include <vector>

void case1() {
    auto arr = std::vector{1, 2, 3};
    auto &ref = arr[0];
    auto cap = arr.capacity();
    for (auto i = arr.size(); i <= cap; ++i) {
        arr.push_back(i);
    }
    std::cout << "ref: " << ref << std::endl;

    // PANIC!!!

    // Output is not deterministic.
}

void case2() {
    auto arr = std::vector{1, 2, 3};
    auto cap = arr.capacity();
    for (auto i = arr.size(); i <= cap; ++i) {
        arr.push_back(i);
    }
    auto &ref = arr[0];
    std::cout << "ref: " << ref << std::endl;

    // Output:
    // ref: 1
}

int main() {
    case1();
    case2();
    return 0;
}
