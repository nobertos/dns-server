use tokio::net::UdpSocket;

use crate::dns_message::dns_header::ResultCode;
use crate::dns_message::packet_buffer::PacketBuffer;
use crate::dns_message::DnsMessage;
use crate::dns_resolver::lookup::recursive_lookup;
use crate::errors::Result;

/// Handle a single incoming packet
pub async fn handle_query(socket: &UdpSocket) -> Result<()> {
    let mut recv_buffer = PacketBuffer::new();

    let (_, src) = socket.recv_from(&mut recv_buffer.buf).await?;

    let mut request = DnsMessage::from_buf(&mut recv_buffer)?;

    let mut message = DnsMessage::new();
    message.header.id = request.header.id;
    message.header.recursion_desired = true;
    message.header.recursion_available = true;
    message.header.response = true;

    if let Some(question) = request.questions.pop() {
        println!("Received query: {:#?}", question);
        if let Ok(result) = recursive_lookup(&question.qname, question.qtype).await {
            message.questions.push(question);
            message.header.rescode = result.header.rescode;
            for rec in result.answers {
                message.answers.push(rec);
            }
            for rec in result.authorities {
                message.authorities.push(rec);
            }
            for rec in result.resources {
                message.resources.push(rec);
            }
        } else {
            message.header.rescode = ResultCode::SERVFAIL;
        }
    } else {
        message.header.rescode = ResultCode::FORMERR;
    }

    let send_buffer = message.into_buf()?;
    println!("message {:#?}", message);
    let len = send_buffer.pos();
    println!("The data length is {}", len);
    let data = send_buffer.get_range(0, len)?;
    socket.send_to(data, src).await?;

    Ok(())
}
