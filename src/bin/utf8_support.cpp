
#if defined(_WIN32) || defined(_WIN64)
#include <Windows.h>
#endif

#include <filesystem>
#include <iostream>

int main() {
#if defined(_WIN32) || defined(_WIN64)
    SetConsoleOutputCP(CP_UTF8);
#endif
    // This command is required to print UTF-8 characters to the console.
    // But in many other languages, utf8 is the default encoding.

    auto current_dir = std::filesystem::current_path() / "data";
    for (auto &entry : std::filesystem::directory_iterator(current_dir)) {
        auto path = entry.path().filename().string();
        std::cout << "Filename: " << path << std::endl;
    }
}
