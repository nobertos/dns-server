use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{errors::Result, packet_buffer::PacketBuffer};

use super::QueryType;

#[derive(Clone, Debug)]
enum RecordData {
    UNKNOWN,
    A { addr: Ipv4Addr },
    NS { host: String },
    CNAME { host: String },
    MX { priority: u16, host: String },
    AAAA { addr: Ipv6Addr },
}
impl RecordData {
    fn new() -> Self {
        Self::UNKNOWN
    }
    fn read_ipv4(raw_addr: u32) -> RecordData {
        RecordData::A {
            addr: Ipv4Addr::from(raw_addr),
        }
    }
    fn read_ipv6(raw_addr: [u16; 8]) -> RecordData {
        RecordData::AAAA {
            addr: Ipv6Addr::from(raw_addr),
        }
    }
    fn read_ns(host: String) -> RecordData {
        RecordData::NS { host }
    }
    fn read_cname(host: String) -> RecordData {
        RecordData::CNAME { host }
    }

    fn read_mx(priority: u16, host: String) -> RecordData {
        RecordData::MX { priority, host }
    }
}

#[derive(Clone, Debug)]
pub struct DnsRecord {
    domain: String,
    qtype: QueryType,
    class: u16,
    ttl: u32,
    data_len: u16,
    data: RecordData,
}
impl DnsRecord {
    pub fn new() -> Self {
        Self {
            domain: String::new(),
            qtype: 0.into(),
            class: 1,
            ttl: 0,
            data_len: 0,
            data: RecordData::new(),
        }
    }

    fn read_data(record: &mut Self, buffer: &mut PacketBuffer) -> Result<()> {
        match record.qtype {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                record.data = RecordData::read_ipv4(raw_addr);
            }

            QueryType::NS => {
                let mut host = String::new();
                buffer.read_qname(&mut host)?;

                record.data = RecordData::read_ns(host)
            }
            QueryType::CNAME => {
                let mut host = String::new();
                buffer.read_qname(&mut host)?;

                record.data = RecordData::read_cname(host)
            }
            QueryType::MX => {
                let priority = buffer.read_u16()?;
                let mut host = String::new();
                buffer.read_qname(&mut host)?;

                record.data = RecordData::read_mx(priority, host)
            }
            QueryType::AAAA => {
                let mut raw_addr: [u16; 8] = [0; 8];
                for hextet in raw_addr.iter_mut() {
                    *hextet = buffer.read_u16()?;
                }
                record.data = RecordData::read_ipv6(raw_addr)
            }
            QueryType::UNKNOWN(_) => {
                buffer.advance(record.data_len as usize)?;
            }
        }
        Ok(())
    }

    pub fn read(buffer: &mut PacketBuffer) -> Result<Self> {
        let mut result = Self::new();
        buffer.read_qname(&mut result.domain)?;

        result.qtype = buffer.read_u16()?.into();
        result.class = buffer.read_u16()?;
        result.ttl = buffer.read_u32()?;
        result.data_len = buffer.read_u16()?;
        Self::read_data(&mut result, buffer)?;
        Ok(result)
    }

    fn write_data(&self, buffer: &mut PacketBuffer) -> Result<()> {
        buffer.write_u16(self.data_len)?;
        match self.data {
            RecordData::A { addr } => buffer.write_u32(addr.into())?,
            RecordData::NS { ref host } => buffer.write_qname(host)?,
            RecordData::CNAME { ref host } => buffer.write_qname(host)?,
            RecordData::MX { priority, ref host } => {
                buffer.write_u16(priority)?;
                buffer.write_qname(host)?;
            }
            RecordData::AAAA { ref addr } => {
                for hextet in addr.segments() {
                    buffer.write_u16(hextet)?;
                }
            }
            RecordData::UNKNOWN => {}
        }
        Ok(())
    }
    pub fn write(&self, buffer: &mut PacketBuffer) -> Result<usize> {
        let start_pos = buffer.pos();
        buffer.write_qname(&self.domain)?;
        buffer.write_u16(self.qtype.into())?;
        buffer.write_u16(self.class)?;
        buffer.write_u32(self.ttl)?;
        self.write_data(buffer)?;
        Ok(buffer.pos() - start_pos)
    }
}
