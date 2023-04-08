use cdn_dns::load_balancer::{connection::Connection, LoadBalancer};

#[test]
fn read_connections_test() {
    let connections = Connection::read_connections("connections.json");
    println!("{:#?}", connections)
}

#[test]
fn iter_servers_test() {
    let connections = Connection::read_connections("connections.json");
    let lb = LoadBalancer::from(connections);
    for server in lb.iter_servers("192.168.10.2") {
        println!("Server ip address: {}", server);
    }
}
