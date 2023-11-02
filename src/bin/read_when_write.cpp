#include <iostream>

struct Data {
    int value;
    Data(int v) : value(v) {}
};

const Data &max_data(const Data &a, const Data &b) {
    return a.value > b.value ? a : b;
}

/**
 * @brief Expected to act like `a.value = b.value;`
 */
void unusual_assignment(Data &a, const Data &b) {
    a.value = 0;
    a.value = b.value;
}

void case1() {
    auto a = Data(2);
    auto b = Data(1);
    unusual_assignment(a, max_data(a, b));
    std::cout << "a: " << a.value << std::endl;

    // Does not act like expected.

    // Output:
    // a: 0
}

void case2() {
    auto a = Data(1);
    auto b = Data(2);
    unusual_assignment(a, b);
    std::cout << "a: " << a.value << std::endl;

    // Output:
    // a: 2
}

int main() {
    case1();
    case2();
    return 0;
}
