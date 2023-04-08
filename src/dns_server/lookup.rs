use crate::dns_message::dns_header::ResultCode;
use crate::dns_message::dns_question::DnsQuestion;
use crate::dns_message::{DnsMessage, QueryType};
use crate::errors::Result;
use crate::packet_buffer::PacketBuffer;
use std::future::Future;
use std::net::Ipv4Addr;
use std::pin::Pin;
use tokio::net::UdpSocket;

pub async fn lookup(qname: &str, qtype: QueryType, server: (Ipv4Addr, u16)) -> Result<DnsMessage> {
    let socket = UdpSocket::bind(("0.0.0.0", 0)).await?;

    let mut message = DnsMessage::new();

    message.header.id = 6666;
    message.header.questions = 1;
    message.header.recursion_desired = true;
    message
        .questions
        .push(DnsQuestion::new(qname.into(), qtype));

    let send_buffer = message.into_buf()?;
    socket
        .send_to(&send_buffer.buf[0..send_buffer.pos()], server)
        .await?;

    let mut recv_buffer = PacketBuffer::new();
    socket.recv(&mut recv_buffer.buf).await?;

    DnsMessage::from_buf(&mut recv_buffer)
}

pub fn recursive_lookup<'a>(
    qname: &'a str,
    qtype: QueryType,
) -> Pin<Box<dyn Future<Output = std::result::Result<DnsMessage, ()>> + Send + 'a>> {
    Box::pin(async move {
        let mut ns = "198.41.0.4".parse::<Ipv4Addr>().unwrap();

        loop {
            println!("attempting lookup of {:?} {} with ns {}", qtype, qname, ns);

            let server = (ns, 53);
            let response = lookup(qname, qtype, server).await.unwrap();

            if !response.answers.is_empty() && response.header.rescode == ResultCode::NOERROR {
                return Ok(response);
            }

            if response.header.rescode == ResultCode::NXDOMAIN {
                return Ok(response);
            }

            if let Some(new_ns) = response.get_resolved_ns(qname) {
                ns = new_ns;
                continue;
            }

            let new_ns_name = match response.get_unresolved_ns(qname) {
                Some(name) => name,
                None => return Ok(response),
            };

            let recursive_response = recursive_lookup(&new_ns_name, QueryType::A).await.unwrap();

            if let Some(new_ns) = recursive_response.random_ipv4() {
                ns = new_ns;
            } else {
                return Ok(response);
            }
        }
    })
}
