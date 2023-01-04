fn main() {
    let four = IpAddrKind::V4;
    route(four);
    let six = IpAddrKind::V6;
    route(six);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind)
}
