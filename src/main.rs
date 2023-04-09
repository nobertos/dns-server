use cdn_dns::config::get_config;
use cdn_dns::errors::{failed_config_read, failed_socket_bind};

use cdn_dns::dns_resolver::handle_query::handle_query as resolver;
use cdn_dns::load_balancer::handle_query::handle_query as load_balancer;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let config = get_config().expect(failed_config_read());

    let socket_addr = format!("{}:{}", config.application.host, config.application.port);
    let socket = UdpSocket::bind(socket_addr)
        .await
        .expect(failed_socket_bind());
    if config.application.is_load_balancer {
        load_balancer(&socket, config.cdn).await.unwrap();
    } else {
        resolver(&socket).await.unwrap();
    }
}
