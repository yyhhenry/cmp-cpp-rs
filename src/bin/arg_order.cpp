#include <iostream>
#include <string>

std::string new_arg(const std::string &title) {
    std::cout << "new_arg: " << title << std::endl;
    return title;
}

void consume_args(const std::string &v1, const std::string &v2) {
    std::cout << "consume_args: " << v1 << ", " << v2 << std::endl;
}

int main() {
    auto v1 = new_arg("v1");
    auto v2 = new_arg("v2");
    consume_args(v1, v2);
    // new_arg: v1
    // new_arg: v2
    // consume_args: v1, v2

    consume_args(new_arg("v1"), new_arg("v2"));
    // new_arg: v2
    // new_arg: v1
    // consume_args: v1, v2
    return 0;
}
