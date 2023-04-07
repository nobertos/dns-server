pub mod dns_header;
pub mod dns_question;
pub mod dns_record;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType {
    UNKNOWN(u16),
    A,
}

impl From<u16> for QueryType {
    fn from(value: u16) -> Self {
        match value {
            1 => QueryType::A,
            _ => QueryType::UNKNOWN(value),
        }
    }
}
impl Into<u16> for QueryType {
    fn into(self) -> u16 {
        match self {
            Self::A => 1,
            Self::UNKNOWN(value) => value,
        }
    }
}

use crate::error::Result;
use crate::packet_buffer::PacketBuffer;

use self::dns_header::DnsHeader;
use self::dns_question::DnsQuestion;
use self::dns_record::DnsRecord;

#[derive(Clone, Debug)]
pub struct DnsMessage {
    pub header: DnsHeader,
    pub questons: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsMessage {
    pub fn new() -> Self {
        Self {
            header: DnsHeader::new(),
            questons: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new(),
        }
    }

    pub fn from_buf(buffer: &mut PacketBuffer) -> Result<DnsMessage> {
        let mut result = DnsMessage::new();
        result.header.read(buffer)?;
        let header = &result.header;

        for _ in 0..header.questions {
            let mut question = DnsQuestion::new("".into(), QueryType::UNKNOWN(0));
            question.read(buffer)?;
            result.questons.push(question);
        }

        for _ in 0..header.answers {
            let rec = DnsRecord::read(buffer)?;
            result.resources.push(rec);
        }

        for _ in 0..header.authoritative_entries {
            let rec = DnsRecord::read(buffer)?;
            result.authorities.push(rec);
        }

        for _ in 0..header.resource_entries {
            let rec = DnsRecord::read(buffer)?;
            result.resources.push(rec);
        }

        Ok(result)
    }
}
