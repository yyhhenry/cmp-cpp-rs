#include <format>
#include <iostream>
#include <string>

int main() {
    using namespace std::string_literals;
    std::string s = "World"s;
    std::cout << std::format("Hello {}!", s) << std::endl;
    return 0;
}
