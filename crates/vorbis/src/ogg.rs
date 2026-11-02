// Follows libogg 1.1.3 (src/framing.c), Copyright (c) 2002 Xiph.org Foundation, BSD licence:
// see ../LICENSE-libvorbis.
//! Ogg pages and the logical stream that turns them into packets, as libogg does it: a page is
//! found by its capture pattern and kept only when its checksum agrees (otherwise the search
//! goes on from the next `O`), and the stream's segment table joins packets across pages, marks
//! a lost page as a hole, and gives each packet its number and (on the last packet a page
//! completes) the page's granule position.

const CRC: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut r = (i as u32) << 24;
        let mut j = 0;
        while j < 8 {
            r = if r & 0x8000_0000 != 0 {
                (r << 1) ^ 0x04c1_1db7
            } else {
                r << 1
            };
            j += 1;
        }
        t[i] = r;
        i += 1;
    }
    t
};

/// A page inside the file's bytes.
#[derive(Debug, Clone)]
pub struct Page {
    pub header: Vec<u8>,
    pub body: Vec<u8>,
}

impl Page {
    pub fn version(&self) -> u8 {
        self.header[4]
    }
    pub fn continued(&self) -> bool {
        self.header[5] & 1 != 0
    }
    pub fn bos(&self) -> bool {
        self.header[5] & 2 != 0
    }
    pub fn eos(&self) -> bool {
        self.header[5] & 4 != 0
    }
    pub fn granulepos(&self) -> i64 {
        i64::from_le_bytes(self.header[6..14].try_into().expect("8 bytes"))
    }
    pub fn serialno(&self) -> i32 {
        i32::from_le_bytes(self.header[14..18].try_into().expect("4 bytes"))
    }
    pub fn pageno(&self) -> i32 {
        i32::from_le_bytes(self.header[18..22].try_into().expect("4 bytes"))
    }
    pub fn len(&self) -> usize {
        self.header.len() + self.body.len()
    }
}

/// `ogg_sync_pageseek` over the whole file: from `at`, the next page whose checksum agrees and
/// where it starts, or `None` at the end of the data.
pub fn next_page(data: &[u8], mut at: usize) -> Option<(usize, Page)> {
    loop {
        let page = data.get(at..)?;
        if page.len() < 27 {
            return None;
        }
        let ok = page.starts_with(b"OggS") && {
            let header_len = 27 + usize::from(page[26]);
            header_len <= page.len() && {
                let body_len: usize = page[27..header_len].iter().map(|&b| usize::from(b)).sum();
                if header_len + body_len > page.len() {
                    // Not enough data for the whole page: libogg waits for more, and there is
                    // no more.
                    return None;
                }
                let mut crc = 0u32;
                for (i, &b) in page[..header_len + body_len].iter().enumerate() {
                    let b = if (22..26).contains(&i) { 0 } else { b };
                    crc = (crc << 8) ^ CRC[((crc >> 24) as u8 ^ b) as usize];
                }
                if crc == u32::from_le_bytes(page[22..26].try_into().expect("4 bytes")) {
                    return Some((
                        at,
                        Page {
                            header: page[..header_len].to_vec(),
                            body: page[header_len..header_len + body_len].to_vec(),
                        },
                    ));
                }
                false
            }
        };
        debug_assert!(!ok);
        // Lose sync: on to the next possible capture.
        at += match page[1..].iter().position(|&b| b == b'O') {
            Some(p) => p + 1,
            None => page.len(),
        };
    }
}

/// A packet out of the stream.
#[derive(Debug, Clone)]
pub struct Packet {
    pub data: Vec<u8>,
    pub bos: bool,
    pub eos: bool,
    pub granulepos: i64,
    pub packetno: i64,
}

/// `ogg_stream_state` (decode side).
#[derive(Debug, Clone)]
pub struct Stream {
    serialno: i32,
    body: Vec<u8>,
    body_returned: usize,
    lacing: Vec<u32>,
    granule: Vec<i64>,
    lacing_packet: usize,
    lacing_returned: usize,
    pageno: i64,
    packetno: i64,
}

/// What `packetout` found.
pub enum Out {
    None,
    Hole,
    Packet(Packet),
}

impl Stream {
    pub fn new(serialno: i32) -> Self {
        Stream {
            serialno,
            body: Vec::new(),
            body_returned: 0,
            lacing: Vec::new(),
            granule: Vec::new(),
            lacing_packet: 0,
            lacing_returned: 0,
            pageno: -1,
            packetno: 0,
        }
    }

    /// `ogg_stream_reset_serialno`.
    pub fn reset(&mut self, serialno: i32) {
        *self = Stream::new(serialno);
    }

    /// `ogg_stream_pagein`: false when the page is not this stream's (or a later version).
    pub fn pagein(&mut self, og: &Page) -> bool {
        // Clean up returned data.
        if self.body_returned > 0 {
            self.body.drain(..self.body_returned);
            self.body_returned = 0;
        }
        if self.lacing_returned > 0 {
            let lr = self.lacing_returned;
            self.lacing.drain(..lr);
            self.granule.drain(..lr);
            self.lacing_packet -= lr;
            self.lacing_returned = 0;
        }
        if og.serialno() != self.serialno || og.version() > 0 {
            return false;
        }
        let pageno = i64::from(og.pageno());
        let segments = usize::from(og.header[26]);
        let mut bos = og.bos();
        let mut body = &og.body[..];
        let mut segptr = 0;
        if pageno != self.pageno {
            // Unroll a partial packet; mark the lost data.
            for i in self.lacing_packet..self.lacing.len() {
                let n = (self.lacing[i] & 0xff) as usize;
                self.body.truncate(self.body.len() - n);
            }
            self.lacing.truncate(self.lacing_packet);
            self.granule.truncate(self.lacing_packet);
            if self.pageno != -1 {
                self.lacing.push(0x400);
                self.granule.push(-1);
                self.lacing_packet += 1;
            }
        }
        if og.continued() && (self.lacing.is_empty() || *self.lacing.last().expect("one") == 0x400)
        {
            bos = false;
            while segptr < segments {
                let val = usize::from(og.header[27 + segptr]);
                body = &body[val..];
                segptr += 1;
                if val < 255 {
                    break;
                }
            }
        }
        self.body.extend_from_slice(body);
        let mut saved = None;
        while segptr < segments {
            let val = u32::from(og.header[27 + segptr]);
            let mut v = val;
            if bos {
                v |= 0x100;
                bos = false;
            }
            self.lacing.push(v);
            self.granule.push(-1);
            if val < 255 {
                saved = Some(self.lacing.len() - 1);
                self.lacing_packet = self.lacing.len();
            }
            segptr += 1;
        }
        if let Some(s) = saved {
            self.granule[s] = og.granulepos();
        }
        if og.eos() {
            if let Some(last) = self.lacing.last_mut() {
                *last |= 0x200;
            }
        }
        self.pageno = pageno + 1;
        true
    }

    /// `ogg_stream_packetout` (`take`) or dropping the packet (`ogg_stream_packetout(os, NULL)`).
    pub fn packetout(&mut self) -> Out {
        let mut ptr = self.lacing_returned;
        if self.lacing_packet <= ptr {
            return Out::None;
        }
        if self.lacing[ptr] & 0x400 != 0 {
            self.lacing_returned += 1;
            self.packetno += 1;
            return Out::Hole;
        }
        let mut size = (self.lacing[ptr] & 0xff) as usize;
        let mut bytes = size;
        let mut eos = self.lacing[ptr] & 0x200 != 0;
        let bos = self.lacing[ptr] & 0x100 != 0;
        while size == 255 {
            ptr += 1;
            let val = self.lacing[ptr];
            size = (val & 0xff) as usize;
            if val & 0x200 != 0 {
                eos = true;
            }
            bytes += size;
        }
        let start = self.body_returned;
        let packet = Packet {
            data: self.body[start..start + bytes].to_vec(),
            bos,
            eos,
            granulepos: self.granule[ptr],
            packetno: self.packetno,
        };
        self.body_returned += bytes;
        self.lacing_returned = ptr + 1;
        self.packetno += 1;
        Out::Packet(packet)
    }

    pub fn serialno(&self) -> i32 {
        self.serialno
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A one-segment page: the stream finds it, keeps its packet and granule position, and a
    /// page whose checksum no longer agrees is skipped.
    #[test]
    fn pages_are_found_by_checksum_and_give_their_packets() {
        let mut page = b"OggS\0\x02".to_vec();
        page.extend_from_slice(&1234i64.to_le_bytes());
        page.extend_from_slice(&7i32.to_le_bytes());
        page.extend_from_slice(&0i32.to_le_bytes());
        page.extend_from_slice(&[0; 4]);
        page.push(1);
        page.push(3);
        page.extend_from_slice(b"abc");
        let mut crc = 0u32;
        for &b in &page {
            crc = (crc << 8) ^ CRC[((crc >> 24) as u8 ^ b) as usize];
        }
        page[22..26].copy_from_slice(&crc.to_le_bytes());
        let mut data = b"junk".to_vec();
        data.extend_from_slice(&page);
        let (at, og) = next_page(&data, 0).expect("the page");
        assert_eq!(
            (at, og.serialno(), og.granulepos(), og.bos()),
            (4, 7, 1234, true)
        );
        let mut s = Stream::new(7);
        assert!(s.pagein(&og));
        match s.packetout() {
            Out::Packet(p) => {
                assert_eq!(
                    (p.data.as_slice(), p.bos, p.granulepos, p.packetno),
                    (&b"abc"[..], true, 1234, 0)
                );
            }
            _ => panic!("a packet"),
        }
        assert!(matches!(s.packetout(), Out::None));
        data[30] ^= 1;
        assert!(next_page(&data, 0).is_none());
    }
}
