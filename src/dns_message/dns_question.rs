use crate::dns_message::packet_buffer::PacketBuffer;
use crate::errors::Result;

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
            qclass: 1,
        }
    }

    pub fn read(&mut self, buffer: &mut PacketBuffer) -> Result<()> {
        buffer.read_qname(&mut self.qname)?;
        self.qtype = buffer.read_u16()?.into();
        self.qclass = buffer.read_u16()?;
        Ok(())
    }

    pub fn write(&self, buffer: &mut PacketBuffer) -> Result<()> {
        buffer.write_qname(&self.qname)?;
        buffer.write_u16(self.qtype.into())?;
        buffer.write_u16(self.qclass)
    }
}
