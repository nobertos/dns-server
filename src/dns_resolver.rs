use std::net::UdpSocket;

use crate::dns_message::dns_question::DnsQuestion;
use crate::dns_message::{DnsMessage, QueryType};
use crate::errors::Result;
use crate::packet_buffer::PacketBuffer;

pub fn resolve(qname: &str, qtype: QueryType) -> Result<()> {
    Ok(())
}
