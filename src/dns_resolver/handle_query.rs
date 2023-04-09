use tokio::net::UdpSocket;

use crate::dns_message::dns_header::ResultCode;
use crate::dns_message::DnsMessage;
use crate::dns_resolver::lookup::recursive_lookup;
use crate::errors::Result;
use crate::packet_buffer::PacketBuffer;

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
            message.answers = result.answers;
            message.answers = result.authorities;
            message.resources = result.resources;
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
