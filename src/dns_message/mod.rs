pub mod dns_header;
pub mod dns_question;
pub mod dns_record;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType {
    UNKNOWN(u16),
    A,     // 1
    NS,    // 2
    CNAME, // 5
    MX,    // 15
    AAAA,  // 28
}

impl From<u16> for QueryType {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            5 => Self::CNAME,
            15 => Self::MX,
            28 => Self::AAAA,
            _ => Self::UNKNOWN(value),
        }
    }
}
impl Into<u16> for QueryType {
    fn into(self) -> u16 {
        match self {
            Self::A => 1,
            Self::NS => 2,
            Self::CNAME => 5,
            Self::MX => 15,
            Self::AAAA => 28,
            Self::UNKNOWN(value) => value,
        }
    }
}

use crate::errors::Result;
use crate::packet_buffer::PacketBuffer;

use self::dns_header::DnsHeader;
use self::dns_question::DnsQuestion;
use self::dns_record::DnsRecord;

#[derive(Clone, Debug)]
pub struct DnsMessage {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsMessage {
    pub fn new() -> Self {
        Self {
            header: DnsHeader::new(),
            questions: Vec::new(),
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
            result.questions.push(question);
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

    pub fn into_buf(&mut self) -> Result<PacketBuffer> {
        self.header.questions = self.questions.len() as u16;
        self.header.answers = self.answers.len() as u16;
        self.header.authoritative_entries = self.authorities.len() as u16;
        self.header.resource_entries = self.resources.len() as u16;

        let mut buffer = PacketBuffer::new();
        self.header.write(&mut buffer)?;

        for qst in &self.questions {
            qst.write(&mut buffer)?;
        }

        for rec in &self.answers {
            rec.write(&mut buffer)?;
        }

        for rec in &self.authorities {
            rec.write(&mut buffer)?;
        }

        for rec in &self.resources {
            rec.write(&mut buffer)?;
        }

        Ok(buffer)
    }
}
