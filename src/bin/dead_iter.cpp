#include <iostream>
#include <vector>

void case1() {
    auto arr = std::vector{1, 2, 3};
    auto iter = arr.begin();
    auto cap = arr.capacity();
    for (auto i = arr.size(); i <= cap; ++i) {
        arr.push_back(0);
    }
    // PANIC!!!
    // This may be not 1 (some times still 1, since the memory is not reused),
    std::cout << "first: " << *iter << std::endl;

    // and this is always not 0, because the vector has been reallocated.
    std::cout << "(arr.begin() - iter): " << (arr.begin() - iter) << std::endl;
}

void case2() {
    auto arr = std::vector{1, 2, 3};
    auto cap = arr.capacity();
    for (auto i = arr.size(); i <= cap; ++i) {
        arr.push_back(0);
    }
    auto iter = arr.begin();
    std::cout << "first: " << *iter << std::endl;

    // Output:
    // iter: 1
}

int main() {
    case1();
    case2();
    return 0;
}
