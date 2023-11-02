#include <algorithm>
#include <iostream>

struct Data {
    int value;
    Data(int v) : value(v) {}
};

/**
 * @brief Expected to act like `out := max { 0, a, b }` in math.
 *
 */
void positive_max(Data &out, const Data &a, const Data &b) {
    out.value = 0;
    out.value = std::max(out.value, a.value);
    out.value = std::max(out.value, b.value);
}

void case1() {
    auto a = Data(1);
    const auto b = Data(-1);
    positive_max(a, b, a);
    std::cout << "a: " << a.value << std::endl;

    // Does not act like expected.

    // Output:
    // a: 0
}

void case2() {
    auto a = Data(1);
    auto b = Data(-1);
    auto c = Data(1);
    positive_max(a, b, c);
    std::cout << "a: " << a.value << std::endl;

    // Output:
    // a: 1
}

int main() {
    case1();
    case2();
    return 0;
}
