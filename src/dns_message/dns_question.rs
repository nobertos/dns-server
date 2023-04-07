use crate::error::Result;
use crate::packet_buffer::PacketBuffer;

use super::QueryType;

#[derive(Clone, Debug)]
pub struct DnsQuestion {
    pub qname: String,
    pub qtype: QueryType,
    qclass: u16,
}

impl DnsQuestion {
    pub fn new(qname: String, qtype: QueryType) -> Self {
        Self {
            qname,
            qtype,
            qclass: 0,
        }
    }

    pub fn read(&mut self, buffer: &mut PacketBuffer) -> Result<()> {
        buffer.read_qname(&mut self.qname)?;
        self.qtype = buffer.read_u16()?.into();
        self.qclass = buffer.read_u16()?;
        Ok(())
    }
}
