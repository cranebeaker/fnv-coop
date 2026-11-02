//! The trace file. Both recorders write it: the DLL inside the real FalloutNV.exe
//! (`nv-trace-rec`) and the port's engine (`crates/engine-core/src/call_trace.rs`, which keeps
//! its own copy of the writer because the engine crates take no research crates).
//!
//! ```text
//! "NVTRACE1"  u32 header length  header text (key=value lines; fn= lines repeat the list)
//! then blocks:  u32 thread id  u32 length  records (a thread's byte stream, in order)
//! ```
//!
//! A thread's records may be split across blocks at any byte; a reader joins each thread's
//! blocks in file order before parsing. Records, little-endian:
//!
//! ```text
//! enter: u8 1, u8 n, u16 0, u32 address, u32 seq, n x u32 words (ECX first for thiscall,
//!        ECX and EDX first for fastcall, then the stack words)
//! leave: u8 2, u8 flags (bit 0: ST0 valid), u16 0, u32 address, u32 seq of its enter,
//!        u32 EAX, u32 EDX, u64 ST0 as the bits of a double
//! thread: u8 3, u8 0, u16 0, u32 start address (0: the main thread), u32 start parameter;
//!        once per thread, before its other records (optional)
//! stack:  u8 4, u8 n, u16 0, u32 seq of its enter, n x (u8 word index, u8 frame depth,
//!        u16 0, i32 offset): which words are stack addresses (optional; see StackWord)
//! dump:   u8 5, u8 n, u16 0, u32 seq of its enter, n x (u32 address, u16 length, u16 1 if
//!        read else 0, the bytes padded to 4): the list's dump= at entry (optional)
//! rdump:  u8 7, the same as dump: the list's rdump= and rhash= at return (optional)
//! ```
//!
//! `seq` numbers the enter records of the whole process in the order they were taken (one
//! counter for all threads), so it orders calls across threads too.

use std::collections::BTreeMap;

pub const MAGIC: &[u8; 8] = b"NVTRACE1";
pub const ENTER: u8 = 1;
pub const LEAVE: u8 = 2;
pub const ENTER_LEN: usize = 12;
pub const LEAVE_LEN: usize = 28;
pub const THREAD: u8 = 3;
pub const THREAD_LEN: usize = 12;
pub const STACK: u8 = 4;
pub const DUMP: u8 = 5;
pub const RDUMP: u8 = 7;

/// One record as written.
#[derive(Clone, Debug, PartialEq)]
pub enum Record {
    Enter {
        addr: u32,
        seq: u32,
        words: Vec<u32>,
    },
    Leave {
        addr: u32,
        seq: u32,
        eax: u32,
        edx: u32,
        st0: Option<u64>,
    },
    /// Which words of the enter record `seq` are stack addresses, with where they point.
    Stack { seq: u32, words: Vec<StackWord> },
    /// The memory dumped at the entry of the call `seq` (the list's `dump=`), in order.
    Dump {
        seq: u32,
        dumps: Vec<crate::dump::Dumped>,
    },
    /// The memory recorded when the call `seq` returned (the list's `rdump=`, `rhash=`).
    RDump {
        seq: u32,
        dumps: Vec<crate::dump::Dumped>,
    },
}

/// A word that is a stack address: its index in the call's words, the depth of the frame
/// it points into (0: the caller's; [`DEPTH_UNKNOWN`] when the recorder can't tell) and its
/// offset from that frame's base (the caller's stack pointer after its prologue).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StackWord {
    pub index: u8,
    pub depth: u8,
    pub offset: i32,
}

pub const DEPTH_UNKNOWN: u8 = 0xff;

impl Record {
    pub fn encode(&self, out: &mut Vec<u8>) {
        match self {
            Record::Enter { addr, seq, words } => {
                let n = words.len().min(255);
                out.extend_from_slice(&[ENTER, n as u8, 0, 0]);
                out.extend_from_slice(&addr.to_le_bytes());
                out.extend_from_slice(&seq.to_le_bytes());
                for w in &words[..n] {
                    out.extend_from_slice(&w.to_le_bytes());
                }
            }
            Record::Dump { seq, dumps } | Record::RDump { seq, dumps } => {
                let n = dumps.len().min(255);
                let kind = if matches!(self, Record::Dump { .. }) {
                    DUMP
                } else {
                    RDUMP
                };
                out.extend_from_slice(&[kind, n as u8, 0, 0]);
                out.extend_from_slice(&seq.to_le_bytes());
                for d in &dumps[..n] {
                    let bytes = d.bytes.as_deref().unwrap_or(&[]);
                    out.extend_from_slice(&d.addr.to_le_bytes());
                    out.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                    out.extend_from_slice(&u16::from(d.bytes.is_some()).to_le_bytes());
                    out.extend_from_slice(bytes);
                    out.resize(out.len() + (4 - bytes.len() % 4) % 4, 0);
                }
            }
            Record::Stack { seq, words } => {
                let n = words.len().min(255);
                out.extend_from_slice(&[STACK, n as u8, 0, 0]);
                out.extend_from_slice(&seq.to_le_bytes());
                for w in &words[..n] {
                    out.extend_from_slice(&[w.index, w.depth, 0, 0]);
                    out.extend_from_slice(&w.offset.to_le_bytes());
                }
            }
            Record::Leave {
                addr,
                seq,
                eax,
                edx,
                st0,
            } => {
                out.extend_from_slice(&[LEAVE, u8::from(st0.is_some()), 0, 0]);
                out.extend_from_slice(&addr.to_le_bytes());
                out.extend_from_slice(&seq.to_le_bytes());
                out.extend_from_slice(&eax.to_le_bytes());
                out.extend_from_slice(&edx.to_le_bytes());
                out.extend_from_slice(&st0.unwrap_or(0).to_le_bytes());
            }
        }
    }
}

/// The start of a file: magic, length, header text.
pub fn encode_header(text: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(12 + text.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(text.len() as u32).to_le_bytes());
    out.extend_from_slice(text.as_bytes());
    out
}

/// One block: thread id, length, bytes.
pub fn encode_block(tid: u32, bytes: &[u8], out: &mut Vec<u8>) {
    out.extend_from_slice(&tid.to_le_bytes());
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(bytes);
}

fn u32_at(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}

/// A whole trace, read back.
#[derive(Clone, Debug, Default)]
pub struct Trace {
    pub header: String,
    /// Each thread's records in order, by thread id.
    pub threads: BTreeMap<u32, Vec<Record>>,
    /// Bytes at the end of a thread's stream that don't make a whole record (a run that was
    /// killed while writing), by thread.
    pub torn: BTreeMap<u32, usize>,
    /// Each thread's start address (0 for the main thread), from its thread record.
    pub starts: BTreeMap<u32, u32>,
}

impl Trace {
    /// Header values for `key` (in order).
    pub fn header_values<'a>(&'a self, key: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        self.header.lines().filter_map(move |l| {
            let (k, v) = l.split_once('=')?;
            (k.trim() == key).then_some(v.trim())
        })
    }

    pub fn parse(bytes: &[u8]) -> Result<Trace, String> {
        if bytes.len() < 12 || &bytes[..8] != MAGIC {
            return Err("not an nv-trace file (no NVTRACE1 magic)".into());
        }
        let hlen = u32_at(bytes, 8) as usize;
        let hend = 12usize
            .checked_add(hlen)
            .filter(|&e| e <= bytes.len())
            .ok_or("header runs past the end of the file")?;
        let header = String::from_utf8_lossy(&bytes[12..hend]).into_owned();
        let mut streams: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
        let mut at = hend;
        while at + 8 <= bytes.len() {
            let tid = u32_at(bytes, at);
            let len = u32_at(bytes, at + 4) as usize;
            let end = (at + 8 + len).min(bytes.len());
            streams
                .entry(tid)
                .or_default()
                .extend_from_slice(&bytes[at + 8..end]);
            at = end;
        }
        let mut trace = Trace {
            header,
            ..Trace::default()
        };
        for (tid, s) in streams {
            let mut recs = Vec::new();
            let mut i = 0;
            loop {
                if i + 4 > s.len() {
                    break;
                }
                match s[i] {
                    ENTER => {
                        let n = s[i + 1] as usize;
                        let len = ENTER_LEN + 4 * n;
                        if i + len > s.len() {
                            break;
                        }
                        let words = (0..n).map(|k| u32_at(&s, i + 12 + 4 * k)).collect();
                        recs.push(Record::Enter {
                            addr: u32_at(&s, i + 4),
                            seq: u32_at(&s, i + 8),
                            words,
                        });
                        i += len;
                    }
                    LEAVE => {
                        if i + LEAVE_LEN > s.len() {
                            break;
                        }
                        let st0 =
                            u64::from(u32_at(&s, i + 20)) | u64::from(u32_at(&s, i + 24)) << 32;
                        recs.push(Record::Leave {
                            addr: u32_at(&s, i + 4),
                            seq: u32_at(&s, i + 8),
                            eax: u32_at(&s, i + 12),
                            edx: u32_at(&s, i + 16),
                            st0: (s[i + 1] & 1 != 0).then_some(st0),
                        });
                        i += LEAVE_LEN;
                    }
                    DUMP | RDUMP => {
                        let returned = s[i] == RDUMP;
                        let n = s[i + 1] as usize;
                        let seq = if i + 8 <= s.len() {
                            u32_at(&s, i + 4)
                        } else {
                            break;
                        };
                        let mut at = i + 8;
                        let mut dumps = Vec::new();
                        let mut whole = true;
                        for _ in 0..n {
                            if at + 8 > s.len() {
                                whole = false;
                                break;
                            }
                            let addr = u32_at(&s, at);
                            let len = u16::from_le_bytes([s[at + 4], s[at + 5]]) as usize;
                            let ok = s[at + 6] != 0;
                            let padded = len + (4 - len % 4) % 4;
                            if at + 8 + padded > s.len() {
                                whole = false;
                                break;
                            }
                            dumps.push(crate::dump::Dumped {
                                addr,
                                bytes: ok.then(|| s[at + 8..at + 8 + len].to_vec()),
                            });
                            at += 8 + padded;
                        }
                        if !whole {
                            break;
                        }
                        recs.push(if returned {
                            Record::RDump { seq, dumps }
                        } else {
                            Record::Dump { seq, dumps }
                        });
                        i = at;
                    }
                    STACK => {
                        let n = s[i + 1] as usize;
                        let len = 8 + 8 * n;
                        if i + len > s.len() {
                            break;
                        }
                        let words = (0..n)
                            .map(|k| {
                                let at = i + 8 + 8 * k;
                                StackWord {
                                    index: s[at],
                                    depth: s[at + 1],
                                    offset: u32_at(&s, at + 4) as i32,
                                }
                            })
                            .collect();
                        recs.push(Record::Stack {
                            seq: u32_at(&s, i + 4),
                            words,
                        });
                        i += len;
                    }
                    THREAD => {
                        if i + THREAD_LEN > s.len() {
                            break;
                        }
                        trace.starts.insert(tid, u32_at(&s, i + 4));
                        i += THREAD_LEN;
                    }
                    k => return Err(format!("thread {tid}: unknown record kind {k} at byte {i}")),
                }
            }
            if i < s.len() {
                trace.torn.insert(tid, s.len() - i);
            }
            trace.threads.insert(tid, recs);
        }
        Ok(trace)
    }

    pub fn read(path: &std::path::Path) -> Result<Trace, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Trace::parse(&bytes).map_err(|e| format!("{}: {e}", path.display()))
    }
}

/// What a call returned.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Returned {
    pub eax: u32,
    pub edx: u32,
    pub st0: Option<u64>,
}

/// One call: its enter record and, when it returned while recorded, its leave record.
#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub tid: u32,
    pub addr: u32,
    pub seq: u32,
    pub words: Vec<u32>,
    pub ret: Option<Returned>,
    /// How many recorded calls were open on the thread when this one was entered.
    pub depth: u32,
    /// Which words are stack addresses (when the recorder said).
    pub stack: Vec<StackWord>,
    /// The memory dumped at entry (the list's `dump=`).
    pub dumps: Vec<crate::dump::Dumped>,
    /// The memory recorded at return (`rdump=`, `rhash=`).
    pub rdumps: Vec<crate::dump::Dumped>,
}

/// Each thread's calls in the order they were entered, enter and leave records paired. A
/// leave record closes the open call with its `seq`; calls above it on the thread's stack
/// never returned (left by an exception) and keep `ret: None`.
pub fn calls(trace: &Trace) -> BTreeMap<u32, Vec<Call>> {
    let mut out = BTreeMap::new();
    for (&tid, recs) in &trace.threads {
        let mut list: Vec<Call> = Vec::new();
        let mut open: Vec<usize> = Vec::new();
        for r in recs {
            match r {
                Record::Enter { addr, seq, words } => {
                    open.push(list.len());
                    list.push(Call {
                        tid,
                        addr: *addr,
                        seq: *seq,
                        words: words.clone(),
                        ret: None,
                        depth: open.len() as u32 - 1,
                        stack: Vec::new(),
                        dumps: Vec::new(),
                        rdumps: Vec::new(),
                    });
                }
                Record::Stack { seq, words } => {
                    if let Some(c) = list.iter_mut().rev().find(|c| c.seq == *seq) {
                        c.stack = words.clone();
                    }
                }
                Record::Dump { seq, dumps } => {
                    if let Some(c) = list.iter_mut().rev().find(|c| c.seq == *seq) {
                        c.dumps = dumps.clone();
                    }
                }
                Record::RDump { seq, dumps } => {
                    if let Some(c) = list.iter_mut().rev().find(|c| c.seq == *seq) {
                        c.rdumps = dumps.clone();
                    }
                }
                Record::Leave {
                    seq, eax, edx, st0, ..
                } => {
                    if let Some(pos) = open.iter().rposition(|&i| list[i].seq == *seq) {
                        let i = open[pos];
                        open.truncate(pos);
                        list[i].ret = Some(Returned {
                            eax: *eax,
                            edx: *edx,
                            st0: *st0,
                        });
                    }
                }
            }
        }
        out.insert(tid, list);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_round_trip_across_split_blocks() {
        let mut file = encode_header("source=test\n");
        let mut a = Vec::new();
        Record::Enter {
            addr: 0x0064_aa00,
            seq: 0,
            words: vec![1, 2, 3],
        }
        .encode(&mut a);
        Record::Leave {
            addr: 0x0064_aa00,
            seq: 0,
            eax: 7,
            edx: 8,
            st0: Some(1.5f64.to_bits()),
        }
        .encode(&mut a);
        // Thread 5's stream split in the middle of a record, thread 9 between them.
        encode_block(5, &a[..10], &mut file);
        let mut b = Vec::new();
        Record::Enter {
            addr: 0x10,
            seq: 1,
            words: vec![],
        }
        .encode(&mut b);
        encode_block(9, &b, &mut file);
        encode_block(5, &a[10..], &mut file);
        let t = Trace::parse(&file).unwrap();
        assert_eq!(t.header_values("source").collect::<Vec<_>>(), ["test"]);
        assert_eq!(t.threads[&5].len(), 2);
        assert_eq!(
            t.threads[&5][1],
            Record::Leave {
                addr: 0x0064_aa00,
                seq: 0,
                eax: 7,
                edx: 8,
                st0: Some(1.5f64.to_bits())
            }
        );
        assert_eq!(t.threads[&9].len(), 1);
        assert!(t.torn.is_empty());
    }

    #[test]
    fn dumps_round_trip_and_attach_to_their_call() {
        use crate::dump::Dumped;
        let mut file = encode_header("");
        let mut a = Vec::new();
        Record::Enter {
            addr: 7,
            seq: 3,
            words: vec![1],
        }
        .encode(&mut a);
        let d = Record::Dump {
            seq: 3,
            dumps: vec![
                Dumped {
                    addr: 0x1020,
                    bytes: Some(vec![1, 2, 3, 4, 5]),
                },
                Dumped {
                    addr: 0,
                    bytes: None,
                },
            ],
        };
        d.encode(&mut a);
        encode_block(9, &a, &mut file);
        let t = Trace::parse(&file).unwrap();
        assert_eq!(t.threads[&9][1], d);
        let c = &calls(&t)[&9][0];
        assert_eq!(c.dumps[0].bytes.as_deref(), Some(&[1u8, 2, 3, 4, 5][..]));
        assert_eq!(c.dumps[1].bytes, None);
    }

    #[test]
    fn a_torn_tail_is_counted_not_fatal() {
        let mut file = encode_header("");
        let mut a = Vec::new();
        Record::Enter {
            addr: 1,
            seq: 0,
            words: vec![4],
        }
        .encode(&mut a);
        encode_block(1, &a[..14], &mut file);
        let t = Trace::parse(&file).unwrap();
        assert!(t.threads[&1].is_empty());
        assert_eq!(t.torn[&1], 14);
    }

    #[test]
    fn calls_pair_leaves_and_skip_unreturned() {
        let mut t = Trace::default();
        t.threads.insert(
            3,
            vec![
                Record::Enter {
                    addr: 1,
                    seq: 0,
                    words: vec![],
                },
                Record::Enter {
                    addr: 2,
                    seq: 1,
                    words: vec![],
                },
                // 2 never returns (exception); 1 returns.
                Record::Leave {
                    addr: 1,
                    seq: 0,
                    eax: 9,
                    edx: 0,
                    st0: None,
                },
                Record::Enter {
                    addr: 3,
                    seq: 2,
                    words: vec![],
                },
                Record::Leave {
                    addr: 3,
                    seq: 2,
                    eax: 4,
                    edx: 0,
                    st0: None,
                },
            ],
        );
        let c = &calls(&t)[&3];
        assert_eq!(c.len(), 3);
        assert_eq!(c[0].ret.unwrap().eax, 9);
        assert_eq!(c[1].ret, None);
        assert_eq!(c[1].depth, 1);
        assert_eq!((c[2].depth, c[2].ret.unwrap().eax), (0, 4));
    }
}
