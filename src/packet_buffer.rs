use crate::error::{index_out_of_bound, jumps_limit, Result};

const BUF_SIZE: usize = 512;

pub struct PacketBuffer {
    pub buf: [u8; BUF_SIZE],
    pub pos: usize,
}

impl PacketBuffer {
    pub fn new() -> Self {
        Self {
            buf: [0; BUF_SIZE],
            pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn advance(&mut self, steps: usize) -> Result<()> {
        self.pos += steps;
        Ok(())
    }

    pub fn seek(&mut self, pos: usize) -> Result<()> {
        self.pos = pos;
        Ok(())
    }

    pub fn read(&mut self) -> Result<u8> {
        if self.pos >= BUF_SIZE {
            return Err(index_out_of_bound());
        }
        let res = self.buf[self.pos];
        self.pos += 1;

        Ok(res)
    }

    pub fn get(&self, pos: usize) -> Result<u8> {
        if pos >= BUF_SIZE {
            return Err(index_out_of_bound());
        }
        Ok(self.buf[pos])
    }

    pub fn get_range(&self, start: usize, len: usize) -> Result<&[u8]> {
        if (start + len) >= BUF_SIZE {
            return Err(index_out_of_bound());
        }
        Ok(&self.buf[start..start + len])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);
        Ok(res)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let res = (self.read()? as u32) << 24
            | (self.read()? as u32) << 16
            | (self.read()? as u32) << 8
            | (self.read()? as u32);
        Ok(res)
    }

    fn qname_jump(&mut self, pos: &mut usize, len: u8, num_jumps: &mut u8) -> Result<bool> {
        let byte = self.get(*pos + 1)? as u16;
        let offset = (((len as u16) ^ 0xC0) << 8) | byte;
        *pos = offset as usize;
        *num_jumps += 1;
        Ok(true)
    }
    fn qname_push(
        &mut self,
        outstr: &mut String,
        delim: &str,
        pos: &mut usize,
        len: u8,
    ) -> Result<()> {
        outstr.push_str(delim);
        let str_buf = self.get_range(*pos, len as usize)?;
        outstr.push_str(&String::from_utf8_lossy(str_buf).to_lowercase());

        *pos += len as usize;
        Ok(())
    }
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
        let mut pos = self.pos();
        let mut jumped = false;
        let mut num_jumps: u8 = 0;
        let max_jumps: u8 = 5;
        let mut delim = "";

        loop {
            if num_jumps > max_jumps {
                return Err(jumps_limit(max_jumps));
            }
            let len = self.get(pos)?;
            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }
                jumped = self.qname_jump(&mut pos, len, &mut num_jumps)?;
                continue;
            }

            pos += 1;
            if len == 0 {
                break;
            }

            self.qname_push(outstr, delim, &mut pos, len)?;
            delim = ".";
        }

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }
}
