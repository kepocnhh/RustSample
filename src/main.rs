fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    route(home);
    let loopback = IpAddr::V6(String::from("::1"));
    route(loopback);
    route(IpAddr::V4(128, 0, 0, 1));
    route(IpAddr::V6(String::from("bar")));
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(item: IpAddr) {
    println!("{:?}", item)
}
