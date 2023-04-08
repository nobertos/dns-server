use std::net::UdpSocket;

use cdn_dns::dns_server::handle_query::handle_query;

#[test]
fn dns_server_test() {
    let socket = UdpSocket::bind(("0.0.0.0", 1053)).expect("Failed to bind UdpSocket");

    handle_query(&socket).unwrap();
}
