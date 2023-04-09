use cdn_dns::load_balancer::connection::ConnectionList;
#[test]
fn iter_servers_test() {
    let connections = ConnectionList::read_connections("connections.json");
    for server in connections.iter_servers("192.168.10.2") {
        println!("Server ip address: {}", server);
    }
}
