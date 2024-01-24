struct IP(u32);
impl IP {
    fn to_string(self) -> String {
        let ip = self.0;
        format!(
            "{}.{}.{}.{}",
            (ip >> 24) as u8,
            (ip >> 16) as u8,
            (ip >> 8) as u8,
            ip as u8
        )
    }
}
impl From<(u8, u8, u8, u8)> for IP {
    fn from(ip: (u8, u8, u8, u8)) -> IP {
        IP((ip.0 as u32) << 24 | (ip.1 as u32) << 16 | (ip.2 as u32) << 8 | ip.3 as u32)
    }
}
impl From<&str> for IP {
    fn from(ip: &str) -> IP {
        let ip: Vec<u8> = ip
            .split('.')
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        (ip[0], ip[1], ip[2], ip[3]).into()
    }
}
#[derive(Debug)]
enum Protocol {
    TCP,
    UDP,
}
struct FiveTuple {
    source_ip: IP,
    destination_ip: IP,
    source_port: u16,
    destination_port: u16,
    protocol: Protocol,
}
enum SourceData {
    IP(IP),
    FiveTuple(FiveTuple),
}
fn print_data(source_data: SourceData) {
    match source_data {
        SourceData::IP(ip) => println!("IP: {}", ip.to_string()),
        SourceData::FiveTuple(five_tuple) => println!(
            "FiveTuple: {:?}",
            (
                five_tuple.source_ip.to_string(),
                five_tuple.destination_ip.to_string(),
                five_tuple.source_port,
                five_tuple.destination_port,
                five_tuple.protocol
            )
        ),
    }
}
fn main() {
    print_data(SourceData::IP("192.168.0.1".into()));
    print_data(SourceData::FiveTuple(FiveTuple {
        source_ip: "192.168.0.1".into(),
        destination_ip: "192.168.0.2".into(),
        source_port: 55555,
        destination_port: 53,
        protocol: Protocol::UDP,
    }));
    print_data(SourceData::FiveTuple(FiveTuple {
        source_ip: "127.0.0.1".into(),
        destination_ip: "127.0.0.1".into(),
        source_port: 54321,
        destination_port: 8080,
        protocol: Protocol::TCP,
    }));
}
