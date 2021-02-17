enum IpAddressKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;
    route(four);
    route(six);
}

fn route(ip_kind: IpAddressKind) {}
