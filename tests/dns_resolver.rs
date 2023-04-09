use cdn_dns::dns_resolver::handle_query::handle_query;
use tokio::net::UdpSocket;

#[tokio::test]
async fn dns_resolver_test() {
    let socket = UdpSocket::bind(("0.0.0.0", 1053))
        .await
        .expect("Failed to bind UdpSocket");

    handle_query(&socket).await.unwrap();
}
