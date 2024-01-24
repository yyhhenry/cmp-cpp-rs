#include <inttypes.h>
#include <iostream>
#include <tuple>
#include <variant>

enum Protocol { UDP,
                TCP };
struct IP {
    uint32_t address;
    static IP from_u32(uint32_t address) {
        IP ip;
        ip.address = address;
        return ip;
    }
    uint32_t to_u32() { return this->address; }
    static IP from_u8(uint8_t a, uint8_t b, uint8_t c, uint8_t d) {
        IP ip;
        ip.address = (a << 24) | (b << 16) | (c << 8) | d;
        return ip;
    }
    std::tuple<uint8_t, uint8_t, uint8_t, uint8_t> to_u8() {
        uint8_t a = (this->address >> 24) & 0xFF;
        uint8_t b = (this->address >> 16) & 0xFF;
        uint8_t c = (this->address >> 8) & 0xFF;
        uint8_t d = this->address & 0xFF;
        return std::make_tuple(a, b, c, d);
    }
    std::string to_string() {
        auto [a, b, c, d] = this->to_u8();
        return std::to_string(a) + "." + std::to_string(b) + "." + std::to_string(c) + "." + std::to_string(d);
    }
};
struct FiveTuple {
    IP source_ip;
    IP destination_ip;
    uint16_t source_port;
    uint16_t destination_port;
    Protocol protocol;
};
using SourceData = std::variant<IP, FiveTuple>;
void print_data(SourceData data) {
    if (std::holds_alternative<IP>(data)) {
        IP ip = std::get<IP>(data);
        std::cout << "IP: " << ip.to_string() << std::endl;
    } else if (std::holds_alternative<FiveTuple>(data)) {
        FiveTuple ft = std::get<FiveTuple>(data);
        std::cout << "FiveTuple: (" << ft.source_ip.to_string() << ", " << ft.destination_ip.to_string() << ", " << ft.source_port << ", " << ft.destination_port << ", " << ft.protocol << ")" << std::endl;
    }
}
int main() {
    print_data(IP::from_u8(192, 168, 0, 1));
    print_data(FiveTuple{
        source_ip : IP::from_u8(192, 168, 0, 1),
        destination_ip : IP::from_u8(192, 168, 0, 2),
        source_port : 55555,
        destination_port : 53,
        protocol : Protocol::UDP,
    });
    print_data(FiveTuple{
        source_ip : IP::from_u8(127, 0, 0, 1),
        destination_ip : IP::from_u8(127, 0, 0, 1),
        source_port : 54321,
        destination_port : 8080,
        protocol : Protocol::TCP,
    });
    return 0;
}
