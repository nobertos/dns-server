use tokio::net::UdpSocket;

use crate::config::CdnSettings;
use crate::dns_message::dns_header::ResultCode;
use crate::dns_message::dns_record::DnsRecord;
use crate::dns_message::DnsMessage;
use crate::errors::{failed_cdn_down, Result};
use crate::packet_buffer::PacketBuffer;

use super::connection::ConnectionList;

pub async fn handle_query(socket: &UdpSocket, config: CdnSettings) -> Result<()> {
    let mut recv_buffer = PacketBuffer::new();

    let (_, src) = socket.recv_from(&mut recv_buffer.buf).await?;
    let src = src.to_string();

    let mut request = DnsMessage::from_buf(&mut recv_buffer)?;

    let mut message = DnsMessage::new();
    message.header.id = request.header.id;
    message.header.recursion_desired = request.header.recursion_desired;
    message.header.response = true;

    if let Some(question) = request.questions.pop() {
        println!("Received query: {:#?}", question);
        if question.qname == config.hostname {
            message.questions.push(question);
            message.header.rescode = ResultCode::NOERROR;

            construct_record(&src, &mut message, &config);
            // message.answers.push(DnsRecord::new_a(addr, hostname))
        } else {
            message.header.rescode = ResultCode::SERVFAIL;
        }
    } else {
        message.header.rescode = ResultCode::FORMERR;
    }

    let send_buffer = message.into_buf()?;

    let len = send_buffer.pos();
    let data = send_buffer.get_range(0, len)?;
    socket.send_to(data, src).await?;

    Ok(())
}

fn construct_record(src: &str, message: &mut DnsMessage, config: &CdnSettings) {
    let up_servers = &config.up_servers;
    let hostname = &config.hostname;
    let connections = ConnectionList::read_connections(&config.connections_path);
    let addr = match connections
        .iter_servers(src.split(":").next().unwrap())
        .next()
    {
        Some(addr) => addr,
        None => up_servers.first().expect(failed_cdn_down()),
    };
    let record = DnsRecord::new_a(addr, hostname);
    message.answers.push(record)
}
