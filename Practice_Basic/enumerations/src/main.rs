#[derive(Debug)]

enum IpAddrKind {
    V4(String),
    V6
}

struct IpAddr {
    Ip : IpAddrKind,
    address : String
}

fn main() {
    let four : IpAddrKind = IpAddrKind::V4(String::from("test"));
    let five : IpAddrKind = IpAddrKind::V6;

    let one_ip  : IpAddr = IpAddr{
        Ip: IpAddrKind::V4(String::from("test")),
        address : String::from("127.0.0.1"),
    };

    println!("{:#?}", one_ip.address);
    println!("{:#?}", one_ip.Ip);

}
