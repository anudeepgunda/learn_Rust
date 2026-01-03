fn main() {
    let endpoint: EndPoint = EndPoint {
        ip_address: String::from("127.0.0.1"),
        port: 8443,
        address_type: IpAddrkind::V4,
        is_active: true,
    };
    println!("IpAddress is: {:?}", endpoint);
}
#[derive(Debug)]
enum IpAddrkind {
    V4,
    V6,
}
#[derive(Debug)]
struct EndPoint {
    ip_address: String,
    port: u32,
    address_type: IpAddrkind,
    is_active: bool,
}
