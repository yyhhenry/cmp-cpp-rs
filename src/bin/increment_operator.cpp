#include <iostream>

void case1() {
    int x = 1;
    int y = ++x + x++; // Of course, this is UB.
    std::cout << "x: " << x << std::endl;
    std::cout << "y: " << y << std::endl;
    // Output (Not deterministic, depends on compiler):
    // x: 3
    // y: 5
}

void case2() {
    int x = 1;
    int tmp1 = ++x;
    int tmp2 = x++;
    int y = tmp1 + tmp2;
    std::cout << "x: " << x << std::endl;
    std::cout << "y: " << y << std::endl;

    // Output:
    // x: 3
    // y: 4
}

int main() {
    case1();
    case2();
    return 0;
}
