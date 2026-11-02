//! `(unplaced)/run_0071dd50` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two groups of functions without Xbox PDB names:
//!
//! - `0071dd50` and `0071dd90` work on an object (class unconfirmed) that
//!   keeps a 0x104-byte file name at +0x810 and appends lines of text to
//!   that file through a `BSFile` on the stack.
//! - `0071dec0` to `0071e270` are the methods of a doubly linked list of
//!   8-byte strings (`BSStringT`, constructed by `004037b0`, destroyed by
//!   `004037d0`): the list header is head, tail and count, the shape of
//!   [`NiTPointerList`]; each node is 0x10 bytes, next and previous (as
//!   [`NiTListItem`]) followed by the string inline at +8. The list's
//!   allocator object is the count word at +8, which the node allocator
//!   (`0071e120`) receives as `this` and does not read.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

/// Offset of the 0x104-byte file name in the object `0071dd50` and
/// `0071dd90` work on.
const FILE_NAME: u32 = 0x810;
/// Size of that buffer (`MAX_PATH`).
const FILE_NAME_SIZE: u32 = 0x104;
/// Bounded string copy (`00406d30`, cdecl: destination, size, source).
const STRING_COPY: u32 = 0x0040_6d30;
/// `_strlen` (`00ec6130`, cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// `BSFile::BSFile` (Xbox PDB, `00b00260`): this, name, then three words.
const BS_FILE_CONSTRUCT: u32 = 0x00b0_0260;
/// `BSFile::Open` (Xbox PDB, `00aff300`): this, two words; `bool` result.
const BS_FILE_OPEN: u32 = 0x00af_f300;
/// `00b00590` (`BSSystem/bsfile_xenon.cpp`): this, two words.
const BS_FILE_SEEK_LIKE: u32 = 0x00b0_0590;
/// `00aff950` (`BSSystem/bsfile.cpp`): this, buffer, byte count.
const BS_FILE_WRITE_LIKE: u32 = 0x00af_f950;
/// `BSFile::Close` (Xbox PDB, `00affd10`).
const BS_FILE_CLOSE: u32 = 0x00af_fd10;
/// `BSFile::~BSFile` (Xbox PDB, `00aff240`).
const BS_FILE_DESTRUCT: u32 = 0x00af_f240;
/// The word `0071dd90` passes as the second argument of `00b00590`.
const SEEK_ORIGIN_010A2488: u32 = 0x010a_2488;
/// The two bytes written after each line (`"\r\n"` in the exe).
const LINE_END: u32 = 0x0101_f520;
/// Size of the `BSFile` local (`EBP-0x16C` up to the cookie at `EBP-0x14`).
const BS_FILE_LOCAL_SIZE: u32 = 0x158;

/// Size of one list node: next, previous, then the 8-byte string.
const NODE_SIZE: u32 = 0x10;
/// Offset of the string inside a node.
const NODE_STRING: u32 = 0x08;
/// Allocator (`00401000`, cdecl: size).
const ALLOCATE: u32 = 0x0040_1000;
/// Deallocator (`00401030`, cdecl: block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `BSStringT` default constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `BSStringT` destructor (`004037d0`).
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `BSStringT` copy (`004047f0`): this, source string.
const STRING_COPY_FROM: u32 = 0x0040_47f0;
/// `BSStringT` assignment from a C string (`00501c10`): this, text.
const STRING_SET: u32 = 0x0050_1c10;

// Translated from 0071dd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the file name at +0x810: copies `name` there with `00406d30`
/// (bounded by 0x104 bytes), or empties it when `name` is null.
pub fn fn_0071dd50(e: &mut Engine, this: Ptr, name: Ptr) {
    if name.is_null() {
        e.mem.set_u8(this.addr() + FILE_NAME, 0);
    } else {
        e.call(
            STRING_COPY,
            &args![this.addr() + FILE_NAME, FILE_NAME_SIZE, name],
        );
    }
}

// Translated from 0071dd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends `text` and the two bytes at `0101f520` to the file named at
/// +0x810, when that name is not empty and `text` is a non-empty string:
/// constructs a `BSFile` on the stack (`name, 1, 0x2800, 1`), opens it
/// (`0, 1`), on success calls `00b00590(0, [010a2488])` and writes the text
/// and the line end with `00aff950`, then closes and destroys the file.
// Translation incomplete: 0071dd90 - the exception frame (FS:[0]) that destroys the BSFile during unwinding and the /GS cookie check are not translated
pub fn fn_0071dd90(e: &mut Engine, this: Ptr, text: Ptr) {
    if e.mem.u8(this.addr() + FILE_NAME) as i8 == 0 {
        return;
    }
    let length = if text.is_null() {
        0
    } else {
        e.call(STRLEN, &args![text]).u32()
    };
    if length == 0 {
        return;
    }
    e.with_stack(BS_FILE_LOCAL_SIZE, |e, file| {
        e.call(
            BS_FILE_CONSTRUCT,
            &args![file, this.addr() + FILE_NAME, 1u32, 0x2800u32, 1u32],
        );
        if e.call(BS_FILE_OPEN, &args![file, 0u32, 1u32]).bool() {
            let origin: u32 = e.global(SEEK_ORIGIN_010A2488);
            e.call(BS_FILE_SEEK_LIKE, &args![file, 0u32, origin]);
            e.call(BS_FILE_WRITE_LIKE, &args![file, text, length]);
            e.call(BS_FILE_WRITE_LIKE, &args![file, LINE_END, 2u32]);
        }
        e.call(BS_FILE_CLOSE, &args![file]);
        e.call(BS_FILE_DESTRUCT, &args![file]);
    });
}

// Translated from 0071dec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The string list's destructor: frees every node (`0071e070`), then runs
/// `00483710` on the list.
// Translation incomplete: 0071dec0 - the exception frame (FS:[0]) that runs 00483710 during unwinding is not translated
pub fn fn_0071dec0(e: &mut Engine, this: Ptr<NiTPointerList>) {
    fn_0071e070(e, this);
    e.call(0x0048_3710, &args![this]);
}

// Translated from 0071df20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes a new node (`0071e0d0`), sets its string to `text` (`00501c10`)
/// and hands the node to `0049fa20` (this, node).
pub fn fn_0071df20(e: &mut Engine, this: Ptr<NiTPointerList>, text: Ptr) {
    let node = fn_0071e0d0(e, this);
    e.call(STRING_SET, &args![node.addr() + NODE_STRING, text]);
    e.call(0x0049_fa20, &args![this, node]);
}

// Translated from 0071df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes a new node (`0071e0d0`), sets its string to `text` (`00501c10`)
/// and hands the node to `00559a70` (this, node).
pub fn fn_0071df60(e: &mut Engine, this: Ptr<NiTPointerList>, text: Ptr) {
    let node = fn_0071e0d0(e, this);
    e.call(STRING_SET, &args![node.addr() + NODE_STRING, text]);
    e.call(0x0055_9a70, &args![this, node]);
}

// Translated from 0071dfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the head node and returns its string by value in `result`: the
/// next node becomes the head (its previous link cleared, or the tail
/// cleared when the list is now empty), the string is copied into a stack
/// temporary (`004047f0`), the node is freed (`0071e0f0`), the count drops
/// by one, the temporary is copied into `result` and destroyed. The head
/// is not checked for null.
// Translation incomplete: 0071dfa0 - the exception frame (FS:[0]) that destroys the temporary string during unwinding is not translated
pub fn fn_0071dfa0(e: &mut Engine, this: Ptr<NiTPointerList>, result: Ptr) -> Ptr {
    let head = e.get(this, NiTPointerList::m_pkHead);
    let next = e.mem.u32(head);
    e.set(this, NiTPointerList::m_pkHead, next);
    if next == 0 {
        e.set(this, NiTPointerList::m_pkTail, 0);
    } else {
        e.set(Ptr::<NiTListItem>::new(next), NiTListItem::m_pkPrev, 0);
    }
    e.with_stack(8, |e, temporary| {
        e.call(STRING_COPY_FROM, &args![temporary, head + NODE_STRING]);
        fn_0071e0f0(e, this, Ptr::new(head));
        let count = e.get(this, NiTPointerList::m_uiCount);
        e.set(this, NiTPointerList::m_uiCount, count.wrapping_sub(1));
        e.call(STRING_COPY_FROM, &args![result, temporary]);
        e.call(STRING_DESTRUCT, &args![temporary]);
    });
    result
}

// Translated from 0071e070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every node from the head on (`0071e0f0`), then clears the count,
/// head and tail.
pub fn fn_0071e070(e: &mut Engine, this: Ptr<NiTPointerList>) {
    let mut node = e.get(this, NiTPointerList::m_pkHead);
    while node != 0 {
        let current = node;
        node = e.mem.u32(node);
        fn_0071e0f0(e, this, Ptr::new(current));
    }
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
}

// Translated from 0071e0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A new node from the list's allocator (`0071e120` on the count word at
/// +8).
pub fn fn_0071e0d0(e: &mut Engine, this: Ptr<NiTPointerList>) -> Ptr {
    fn_0071e120(e, Ptr::new(this.addr() + 8))
}

// Translated from 0071e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees `node`: calls `00438390(0)` on its string, then deletes the node
/// through the allocator (`0071e1a0` on the count word at +8).
pub fn fn_0071e0f0(e: &mut Engine, this: Ptr<NiTPointerList>, node: Ptr) {
    e.call(0x0043_8390, &args![node.addr() + NODE_STRING, 0u32]);
    fn_0071e1a0(e, Ptr::new(this.addr() + 8), node);
}

// Translated from 0071e120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node allocator: allocates 0x10 bytes (`00401000`) and constructs a
/// node in them (`0071e1e0`); null when the allocation fails. `this` (the
/// allocator object) is not read.
// Translation incomplete: 0071e120 - the exception frame (FS:[0]) that frees the block if the constructor throws is not translated
pub fn fn_0071e120(e: &mut Engine, _this: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![NODE_SIZE]).ptr::<()>();
    if block.is_null() {
        Ptr::NULL
    } else {
        fn_0071e1e0(e, block)
    }
}

// Translated from 0071e1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes `node` (`0071e200` with flag 1) when it is not null. `this` (the
/// allocator object) is not read.
pub fn fn_0071e1a0(e: &mut Engine, _this: Ptr, node: Ptr) {
    if !node.is_null() {
        fn_0071e200(e, node, 1);
    }
}

// Translated from 0071e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Node constructor: constructs the node's string (`0071e230`) and returns
/// the node. The links are left as they are.
pub fn fn_0071e1e0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0071e230(e, this);
    this
}

// Translated from 0071e200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Node deleting destructor: destroys the node (`0071e250`) and frees it
/// (`00401030`) when bit 0 of `flags` is set; returns the node.
pub fn fn_0071e200(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0071e250(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 0071e230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the string at +8 of a node (`004037b0`) and returns the node.
pub fn fn_0071e230(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(STRING_CONSTRUCT, &args![this.addr() + NODE_STRING]);
    this
}

// Translated from 0071e250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Node destructor: `0071e270`.
pub fn fn_0071e250(e: &mut Engine, this: Ptr) {
    fn_0071e270(e, this);
}

// Translated from 0071e270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the string at +8 of a node (`004037d0`).
pub fn fn_0071e270(e: &mut Engine, this: Ptr) {
    e.call(STRING_DESTRUCT, &args![this.addr() + NODE_STRING]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0071dd50, fn_0071dd50(Ptr, Ptr)),
        entry!(0x0071dd90, fn_0071dd90(Ptr, Ptr)),
        entry!(0x0071dec0, fn_0071dec0(Ptr<NiTPointerList>)),
        entry!(0x0071df20, fn_0071df20(Ptr<NiTPointerList>, Ptr)),
        entry!(0x0071df60, fn_0071df60(Ptr<NiTPointerList>, Ptr)),
        entry!(0x0071dfa0, fn_0071dfa0(Ptr<NiTPointerList>, Ptr) -> Ptr),
        entry!(0x0071e070, fn_0071e070(Ptr<NiTPointerList>)),
        entry!(0x0071e0d0, fn_0071e0d0(Ptr<NiTPointerList>) -> Ptr),
        entry!(0x0071e0f0, fn_0071e0f0(Ptr<NiTPointerList>, Ptr)),
        entry!(0x0071e120, fn_0071e120(Ptr) -> Ptr),
        entry!(0x0071e1a0, fn_0071e1a0(Ptr, Ptr)),
        entry!(0x0071e1e0, fn_0071e1e0(Ptr) -> Ptr),
        entry!(0x0071e200, fn_0071e200(Ptr, u32) -> Ptr),
        entry!(0x0071e230, fn_0071e230(Ptr) -> Ptr),
        entry!(0x0071e250, fn_0071e250(Ptr)),
        entry!(0x0071e270, fn_0071e270(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine with doubles for every callee of the list functions; the
    /// allocator hands out real blocks and the deallocator frees them.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        for addr in [
            DEALLOCATE,
            STRING_CONSTRUCT,
            STRING_DESTRUCT,
            STRING_COPY_FROM,
            STRING_SET,
            0x0043_8390,
            0x0048_3710,
            0x0049_fa20,
            0x0055_9a70,
        ] {
            e.register(addr, nothing);
        }
        e
    }

    /// A list of `n` nodes linked both ways; returns the list and nodes.
    fn list_of(e: &mut Engine, n: usize) -> (Ptr<NiTPointerList>, Vec<u32>) {
        let list: Ptr<NiTPointerList> = e.new_object();
        let nodes: Vec<u32> = (0..n).map(|_| e.mem.alloc(NODE_SIZE)).collect();
        for (i, node) in nodes.iter().enumerate() {
            let next = nodes.get(i + 1).copied().unwrap_or(0);
            let prev = if i == 0 { 0 } else { nodes[i - 1] };
            e.mem.set_u32(*node, next);
            e.mem.set_u32(node + 4, prev);
        }
        e.set(
            list,
            NiTPointerList::m_pkHead,
            nodes.first().copied().unwrap_or(0),
        );
        e.set(
            list,
            NiTPointerList::m_pkTail,
            nodes.last().copied().unwrap_or(0),
        );
        e.set(list, NiTPointerList::m_uiCount, n as u32);
        (list, nodes)
    }

    #[test]
    fn set_file_name_copies_or_empties() {
        let mut e = Engine::new();
        e.register(STRING_COPY, nothing);
        let object: Ptr = Ptr::new(e.mem.alloc(0x920));
        e.mem.set_u8(object.addr() + FILE_NAME, b'x');
        e.call_log = Some(vec![]);
        e.call(0x0071_dd50, &args![object, 0x1234u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, STRING_COPY),
            vec![vec![object.addr() + FILE_NAME, 0x104, 0x1234]]
        );
        assert_eq!(e.mem.u8(object.addr() + FILE_NAME), b'x');

        e.call_log = Some(vec![]);
        e.call(0x0071_dd50, &args![object, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, STRING_COPY).is_empty());
        assert_eq!(e.mem.u8(object.addr() + FILE_NAME), 0);
    }

    fn file_engine(open: bool) -> Engine {
        let mut e = Engine::new();
        e.register(STRLEN, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            Ret {
                eax: n,
                ..Ret::default()
            }
        });
        for addr in [
            BS_FILE_CONSTRUCT,
            BS_FILE_SEEK_LIKE,
            BS_FILE_WRITE_LIKE,
            BS_FILE_CLOSE,
            BS_FILE_DESTRUCT,
        ] {
            e.register(addr, nothing);
        }
        if open {
            e.register(BS_FILE_OPEN, |_, _| Ret {
                eax: 0xffff_ff01,
                ..Ret::default()
            });
        } else {
            e.register(BS_FILE_OPEN, |_, _| Ret {
                eax: 0xffff_ff00,
                ..Ret::default()
            });
        }
        e.map(0x010a_2000, 0x1000);
        e.set_global(SEEK_ORIGIN_010A2488, 2u32);
        e
    }

    fn c_string(e: &mut Engine, s: &[u8]) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        for (i, b) in s.iter().enumerate() {
            e.mem.set_u8(p + i as u32, *b);
        }
        p
    }

    #[test]
    fn append_line_writes_text_and_line_end() {
        let mut e = file_engine(true);
        let object: Ptr = Ptr::new(e.mem.alloc(0x920));
        e.mem.set_u8(object.addr() + FILE_NAME, b'a');
        let text = c_string(&mut e, b"hello");
        e.call_log = Some(vec![]);
        e.call(0x0071_dd90, &args![object, text]);
        let log = e.call_log.take().unwrap();
        let construct = calls_to(&log, BS_FILE_CONSTRUCT);
        assert_eq!(construct.len(), 1);
        let file = construct[0][0];
        assert_eq!(
            &construct[0][1..],
            &[object.addr() + FILE_NAME, 1, 0x2800, 1]
        );
        assert_eq!(calls_to(&log, BS_FILE_OPEN), vec![vec![file, 0, 1]]);
        assert_eq!(calls_to(&log, BS_FILE_SEEK_LIKE), vec![vec![file, 0, 2]]);
        assert_eq!(
            calls_to(&log, BS_FILE_WRITE_LIKE),
            vec![vec![file, text, 5], vec![file, LINE_END, 2]]
        );
        assert_eq!(calls_to(&log, BS_FILE_CLOSE), vec![vec![file]]);
        assert_eq!(calls_to(&log, BS_FILE_DESTRUCT), vec![vec![file]]);
    }

    #[test]
    fn append_line_closes_without_writing_when_open_fails() {
        let mut e = file_engine(false);
        let object: Ptr = Ptr::new(e.mem.alloc(0x920));
        e.mem.set_u8(object.addr() + FILE_NAME, b'a');
        let text = c_string(&mut e, b"x");
        e.call_log = Some(vec![]);
        e.call(0x0071_dd90, &args![object, text]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, BS_FILE_WRITE_LIKE).is_empty());
        assert!(calls_to(&log, BS_FILE_SEEK_LIKE).is_empty());
        assert_eq!(calls_to(&log, BS_FILE_CLOSE).len(), 1);
        assert_eq!(calls_to(&log, BS_FILE_DESTRUCT).len(), 1);
    }

    #[test]
    fn append_line_skips_empty_name_null_or_empty_text() {
        let mut e = file_engine(true);
        let object: Ptr = Ptr::new(e.mem.alloc(0x920));
        let text = c_string(&mut e, b"x");
        let empty = c_string(&mut e, b"");
        e.call_log = Some(vec![]);
        // Empty file name.
        e.call(0x0071_dd90, &args![object, text]);
        e.mem.set_u8(object.addr() + FILE_NAME, b'a');
        // Null and empty text.
        e.call(0x0071_dd90, &args![object, 0u32]);
        e.call(0x0071_dd90, &args![object, empty]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, BS_FILE_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&log, STRLEN), vec![vec![empty]]);
    }

    #[test]
    fn destructor_frees_all_nodes_then_calls_base() {
        let mut e = list_engine();
        let (list, nodes) = list_of(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x0071_dec0, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, DEALLOCATE),
            vec![vec![nodes[0]], vec![nodes[1]]]
        );
        assert_eq!(calls_to(&log, 0x0048_3710), vec![vec![list.addr()]]);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), 0);
        // The base runs after the nodes are gone.
        let last = log.last().unwrap();
        assert_eq!(last.0, 0x0048_3710);
    }

    #[test]
    fn add_variants_make_a_node_set_it_and_pass_it_on() {
        for (entry, target) in [(0x0071_df20, 0x0049_fa20), (0x0071_df60, 0x0055_9a70)] {
            let mut e = list_engine();
            let list: Ptr<NiTPointerList> = e.new_object();
            e.call_log = Some(vec![]);
            e.call(entry, &args![list, 0x5555u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, ALLOCATE), vec![vec![NODE_SIZE]]);
            let passed = calls_to(&log, target);
            assert_eq!(passed.len(), 1);
            let node = passed[0][1];
            assert_eq!(passed[0][0], list.addr());
            assert_eq!(calls_to(&log, STRING_CONSTRUCT), vec![vec![node + 8]]);
            assert_eq!(calls_to(&log, STRING_SET), vec![vec![node + 8, 0x5555]]);
        }
    }

    #[test]
    fn remove_head_unlinks_and_copies_the_string() {
        let mut e = list_engine();
        let (list, nodes) = list_of(&mut e, 2);
        let result = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let back = e.call(0x0071_dfa0, &args![list, result]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, result);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), nodes[1]);
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), nodes[1]);
        assert_eq!(e.mem.u32(nodes[1] + 4), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 1);
        let copies = calls_to(&log, STRING_COPY_FROM);
        assert_eq!(copies.len(), 2);
        let temporary = copies[0][0];
        assert_eq!(copies[0][1], nodes[0] + 8);
        assert_eq!(copies[1], vec![result, temporary]);
        assert_eq!(calls_to(&log, 0x0043_8390), vec![vec![nodes[0] + 8, 0]]);
        assert_eq!(calls_to(&log, DEALLOCATE), vec![vec![nodes[0]]]);
        // The node's string, then the temporary.
        assert_eq!(
            calls_to(&log, STRING_DESTRUCT),
            vec![vec![nodes[0] + 8], vec![temporary]]
        );
    }

    #[test]
    fn remove_last_clears_the_tail() {
        let mut e = list_engine();
        let (list, _) = list_of(&mut e, 1);
        let result = e.mem.alloc(8);
        e.call(0x0071_dfa0, &args![list, result]);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), 0);
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 0);
    }

    #[test]
    fn clear_frees_nodes_and_resets_the_header() {
        let mut e = list_engine();
        let (list, nodes) = list_of(&mut e, 3);
        e.call_log = Some(vec![]);
        e.call(0x0071_e070, &args![list]);
        let log = e.call_log.take().unwrap();
        let freed: Vec<u32> = calls_to(&log, DEALLOCATE).iter().map(|w| w[0]).collect();
        assert_eq!(freed, nodes);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), 0);
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 0);
    }

    #[test]
    fn new_node_constructs_the_string() {
        let mut e = list_engine();
        let list: Ptr<NiTPointerList> = e.new_object();
        e.call_log = Some(vec![]);
        let node = e.call(0x0071_e0d0, &args![list]).u32();
        let log = e.call_log.take().unwrap();
        assert_ne!(node, 0);
        assert_eq!(calls_to(&log, STRING_CONSTRUCT), vec![vec![node + 8]]);
    }

    #[test]
    fn free_node_clears_the_string_then_deletes() {
        let mut e = list_engine();
        let list: Ptr<NiTPointerList> = e.new_object();
        let node = e.mem.alloc(NODE_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x0071_e0f0, &args![list, node]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).skip(1).collect();
        assert_eq!(order, vec![0x0043_8390, STRING_DESTRUCT, DEALLOCATE]);
    }

    #[test]
    fn node_allocator_returns_null_when_allocation_fails() {
        let mut e = list_engine();
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0071_e120, &args![0x10u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, STRING_CONSTRUCT).is_empty());
    }

    #[test]
    fn node_delete_skips_null() {
        let mut e = list_engine();
        e.call_log = Some(vec![]);
        e.call(0x0071_e1a0, &args![0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn node_constructor_returns_the_node() {
        let mut e = list_engine();
        let node = e.mem.alloc(NODE_SIZE);
        assert_eq!(e.call(0x0071_e1e0, &args![node]).u32(), node);
        assert_eq!(e.call(0x0071_e230, &args![node]).u32(), node);
    }

    #[test]
    fn node_destructor_frees_only_with_flag() {
        let mut e = list_engine();
        let node = e.mem.alloc(NODE_SIZE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0071_e200, &args![node, 0u32]).u32(), node);
        assert_eq!(e.call(0x0071_e200, &args![node, 3u32]).u32(), node);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_DESTRUCT).len(), 2);
        assert_eq!(calls_to(&log, DEALLOCATE), vec![vec![node]]);
    }

    #[test]
    fn node_destructor_parts_destroy_the_string() {
        let mut e = list_engine();
        let node = e.mem.alloc(NODE_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x0071_e250, &args![node]);
        e.call(0x0071_e270, &args![node]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, STRING_DESTRUCT),
            vec![vec![node + 8], vec![node + 8]]
        );
    }
}
