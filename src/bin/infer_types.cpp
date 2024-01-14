#include <iostream>

void case1() {
    std::cout << "Case 1:" << std::endl;
    auto a = 1; // int
    long long b = 2;
    std::cout << "Is `inferred` the same type as `typed`? " << (typeid(a) == typeid(b)) << std::endl;
    auto _ = a + b; // long long
}
void case2() {
    std::cout << "Case 2:" << std::endl;
    auto a = 1; // int
    long long b = 2;
    std::cout << "Is `inferred` the same type as `typed`? " << (typeid(a) == typeid(b)) << std::endl;
}

int main() {
    case1();
    case2();
    return 0;
}
