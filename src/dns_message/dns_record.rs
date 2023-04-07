use std::net::Ipv4Addr;

use crate::{errors::Result, packet_buffer::PacketBuffer};

use super::QueryType;

#[derive(Clone, Debug)]
pub enum DnsRecord {
    UNKNOWN {
        domain: String,
        qtype: u16,
        data_len: u16,
        class: u16,
        ttl: u32,
    },
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32,
    },
}

impl DnsRecord {
    pub fn read(buffer: &mut PacketBuffer) -> Result<DnsRecord> {
        let mut domain = String::new();
        buffer.read_qname(&mut domain)?;

        let qtype: QueryType = buffer.read_u16()?.into();
        let class = buffer.read_u16()?;
        let ttl = buffer.read_u32()?;
        let data_len = buffer.read_u16()?;
        match qtype {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                let addr = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    ((raw_addr >> 0) & 0xFF) as u8,
                );
                Ok(DnsRecord::A { domain, addr, ttl })
            }
            QueryType::UNKNOWN(_) => {
                buffer.advance(data_len as usize)?;

                Ok(DnsRecord::UNKNOWN {
                    domain,
                    qtype: qtype.into(),
                    data_len,
                    class,
                    ttl,
                })
            }
        }
    }
}
