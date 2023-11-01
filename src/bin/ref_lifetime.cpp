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
