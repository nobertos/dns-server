use crate::dns_message::packet_buffer::PacketBuffer;
use crate::errors::Result;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResultCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl From<u8> for ResultCode {
    fn from(value: u8) -> Self {
        match value {
            1 => ResultCode::FORMERR,
            2 => ResultCode::SERVFAIL,
            3 => ResultCode::NXDOMAIN,
            4 => ResultCode::NOTIMP,
            5 => ResultCode::REFUSED,
            0 | _ => ResultCode::NOERROR,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16,

    pub recursion_desired: bool,
    pub truncated_message: bool,
    pub authoritative_answer: bool,
    pub opcode: u8,
    pub response: bool,

    pub rescode: ResultCode,
    pub checking_disabled: bool,
    pub authed_data: bool,
    pub z: bool,
    pub recursion_available: bool,

    pub questions: u16,
    pub answers: u16,
    pub authoritative_entries: u16,
    pub resource_entries: u16,
}

impl DnsHeader {
    pub fn new() -> DnsHeader {
        DnsHeader {
            id: 0,

            recursion_desired: false,
            truncated_message: false,
            authoritative_answer: false,
            opcode: 0,
            response: false,

            rescode: ResultCode::NOERROR,
            checking_disabled: false,
            authed_data: false,
            z: false,
            recursion_available: false,

            questions: 0,
            answers: 0,
            authoritative_entries: 0,
            resource_entries: 0,
        }
    }

    pub fn read(&mut self, buffer: &mut PacketBuffer) -> Result<()> {
        self.id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;
        // get the left side
        let tmp_a = (flags >> 8) as u8;
        // get the right side:
        //    - flags & 0xFF == flags & 0x00FF
        let tmp_b = (flags & 0xFF) as u8;

        self.recursion_desired = (tmp_a & (1 << 0)) > 0;
        self.truncated_message = (tmp_a & (1 << 1)) > 0;
        self.authoritative_answer = (tmp_a & (1 << 2)) > 0;
        self.opcode = (tmp_a >> 3) & 0x0F;
        self.response = (tmp_a & (1 << 7)) > 0;

        self.rescode = (tmp_b & 0x0F).into();
        self.checking_disabled = (tmp_b & (1 << 4)) > 0;
        self.authed_data = (tmp_b & (1 << 5)) > 0;
        self.z = (tmp_b & (1 << 6)) > 0;
        self.recursion_available = (tmp_b & (1 << 7)) > 0;

        self.questions = buffer.read_u16()?;
        self.answers = buffer.read_u16()?;
        self.authoritative_entries = buffer.read_u16()?;
        self.resource_entries = buffer.read_u16()?;

        Ok(())
    }

    pub fn write(&self, buffer: &mut PacketBuffer) -> Result<()> {
        buffer.write_u16(self.id)?;
        buffer.write(
            ((self.recursion_desired as u8) << 0)
                | ((self.truncated_message as u8) << 1)
                | ((self.authoritative_answer as u8) << 2)
                | ((self.opcode) << 3)
                | ((self.response as u8) << 7) as u8,
        )?;

        buffer.write(
            (self.rescode as u8)
                | ((self.checking_disabled as u8) << 4)
                | ((self.authed_data as u8) << 5)
                | ((self.z as u8) << 6)
                | ((self.recursion_available as u8) << 7) as u8,
        )?;

        buffer.write_u16(self.questions)?;
        buffer.write_u16(self.answers)?;
        buffer.write_u16(self.authoritative_entries)?;
        buffer.write_u16(self.resource_entries)
    }
}
