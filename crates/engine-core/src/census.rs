//! `NV_CENSUS=<folder>,<frame>,<frame>..` (a diagnostic of the typed migration, ADR-0014 step T3;
//! not game behaviour): at the end of each listed frame of main's, writes what the census tools
//! hold into the folder:
//! - `heap-<frame>.tsv`, always: the live heap blocks grouped by allocating function, its caller
//!   and class (bytes and blocks per owner). The allocating function is the first one of the
//!   block's allocation path past the allocator's own ([`crate::mem::ALLOC_WRAPPERS`]); a block
//!   with no path (a `with_stack` local, the C runtime's `malloc`, the platform's own blocks) is
//!   put on the function that was running when it was made. The class is the one whose vtable the
//!   block's first word is, named from the exe's RTTI (its mangled type name), else empty;
//! - `touch-<frame>.tsv` with `NV_TOUCH` on: the accesses to tagged objects so far
//!   ([`crate::mem::TouchHit`]);
//! - `threads-<frame>.tsv` with `NV_PROFILE=1` and `NV_PROFILE_THREAD_CALLS` on: the calls to each
//!   target so far by thread ([`crate::profile::thread_calls_tsv`]).
//!
//! The heap census needs the block check's allocation paths, so `NV_CENSUS` turns the block check
//! on (at frees only, reporting without stopping) when `NV_BLOCK_CHECK` doesn't. Off (unset), each
//! of main's frames pays one look at a value read once.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::mem::{allocating_function, CensusBlock, Mem, TouchSpec};
use crate::Engine;

/// Where and when the census writes (`NV_CENSUS`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CensusAt {
    pub folder: PathBuf,
    pub frames: Vec<u64>,
}

impl CensusAt {
    /// Reads `<folder>,<frame>,<frame>..`; `None` when it isn't that.
    pub fn parse(value: &str) -> Option<CensusAt> {
        let mut parts = value.split(',');
        let folder = PathBuf::from(parts.next()?.trim());
        if folder.as_os_str().is_empty() {
            return None;
        }
        let frames = parts
            .map(|p| p.trim().parse::<u64>().ok())
            .collect::<Option<Vec<_>>>()?;
        (!frames.is_empty()).then_some(CensusAt { folder, frames })
    }
}

/// `NV_CENSUS`, read once.
pub fn settings() -> Option<&'static CensusAt> {
    static AT: OnceLock<Option<CensusAt>> = OnceLock::new();
    AT.get_or_init(|| {
        let v = std::env::var("NV_CENSUS").ok()?;
        let at = CensusAt::parse(&v);
        if at.is_none() {
            eprintln!("NV_CENSUS={v:?}: expected <folder>,<frame>[,<frame>..]; no census");
        }
        at
    })
    .as_ref()
}

/// Main's frame `frame` ended: writes the census files when it is one of `NV_CENSUS`'s frames.
pub fn at_frame(e: &mut Engine, frame: u64) {
    let Some(at) = settings() else {
        return;
    };
    if !at.frames.contains(&frame) {
        return;
    }
    if let Err(err) = write_all(e, at, frame) {
        eprintln!("NV_CENSUS: frame {frame}: {err}");
    }
}

fn write_all(e: &Engine, at: &CensusAt, frame: u64) -> std::io::Result<()> {
    std::fs::create_dir_all(&at.folder)?;
    let heap = heap_tsv(&e.mem, frame);
    std::fs::write(at.folder.join(format!("heap-{frame}.tsv")), heap)?;
    if e.mem.touch_on() {
        std::fs::write(
            at.folder.join(format!("touch-{frame}.tsv")),
            touch_tsv(&e.mem, frame),
        )?;
    }
    if crate::profile::counting_threads() {
        std::fs::write(
            at.folder.join(format!("threads-{frame}.tsv")),
            crate::profile::thread_calls_tsv(),
        )?;
    }
    println!(
        "NV_CENSUS: frame {frame} written to {}",
        at.folder.display()
    );
    Ok(())
}

/// One owner in the heap census: allocating function, the function that called it, the class.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeapRow {
    pub function: u32,
    pub caller: u32,
    pub vtable: u32,
    pub class: String,
    pub blocks: u64,
    /// The bytes asked for, and the bytes the heap holds for them (rounded, without the check's
    /// guard).
    pub bytes: u64,
    pub held: u64,
}

/// The live heap blocks grouped by owner ([module documentation](self)), most bytes first.
pub fn heap_rows(mem: &Mem) -> Vec<HeapRow> {
    let mut groups: HashMap<(u32, u32, u32), HeapRow> = HashMap::new();
    let mut names: HashMap<u32, Option<String>> = HashMap::new();
    mem.for_each_census_block(
        |CensusBlock {
             addr,
             held,
             asked,
             running,
             path,
         }| {
            let (function, caller) = if path.is_empty() {
                (running, 0)
            } else {
                let f = allocating_function(path);
                let at = path.iter().position(|&p| p == f).unwrap_or(0);
                (f, path.get(at + 1).copied().unwrap_or(0))
            };
            let word = if asked >= 4 { first_word(mem, addr) } else { 0 };
            let class = names
                .entry(word)
                .or_insert_with(|| rtti_name(mem, word))
                .clone();
            let vtable = if class.is_some() { word } else { 0 };
            let row = groups
                .entry((function, caller, vtable))
                .or_insert_with(|| HeapRow {
                    function,
                    caller,
                    vtable,
                    class: class.unwrap_or_default(),
                    ..Default::default()
                });
            row.blocks += 1;
            row.bytes += u64::from(asked);
            row.held += u64::from(held);
        },
    );
    let mut rows: Vec<HeapRow> = groups.into_values().collect();
    rows.sort_by(|a, b| {
        b.bytes
            .cmp(&a.bytes)
            .then((a.function, a.caller, a.vtable).cmp(&(b.function, b.caller, b.vtable)))
    });
    rows
}

/// The heap census as a table: a heading line with the totals, then one row an owner.
pub fn heap_tsv(mem: &Mem, frame: u64) -> String {
    let rows = heap_rows(mem);
    let (blocks, bytes, held) = rows.iter().fold((0, 0, 0), |(n, b, h), r| {
        (n + r.blocks, b + r.bytes, h + r.held)
    });
    let mut out = format!(
        "# heap census at main's frame {frame}: {blocks} live blocks, {bytes} bytes asked, {held} held\n\
         function\tcaller\tvtable\tclass\tblocks\tbytes\theld\n"
    );
    for r in rows {
        let _ = writeln!(
            out,
            "{:08x}\t{:08x}\t{:08x}\t{}\t{}\t{}\t{}",
            r.function, r.caller, r.vtable, r.class, r.blocks, r.bytes, r.held
        );
    }
    out
}

/// The touch census so far as a table: the tags with the objects each has had, then one row per
/// kind of access with its count.
pub fn touch_tsv(mem: &Mem, frame: u64) -> String {
    let names = mem.touch_names();
    let tagged = mem.touch_tagged();
    let mut out = format!("# touch census at main's frame {frame}\n");
    for (i, n) in names.iter().enumerate() {
        let _ = writeln!(
            out,
            "# tag {n}: {} objects tagged",
            tagged.get(i).copied().unwrap_or(0)
        );
    }
    out.push_str("function\tcaller\ttag\tclass\toffset\tlen\taccess\tcount\n");
    for (h, n) in mem.touch_hits() {
        let _ = writeln!(
            out,
            "{:08x}\t{:08x}\t{}\t{:08x}\t{:x}\t{}\t{}\t{n}",
            h.function,
            h.caller,
            names.get(h.tag as usize).map_or("?", String::as_str),
            h.class,
            h.offset,
            h.len,
            if h.write { "w" } else { "r" },
        );
    }
    out
}

/// The word at `addr`, or 0 when it can't be read.
fn first_word(mem: &Mem, addr: u32) -> u32 {
    let mut b = [0u8; 4];
    match mem.peek(addr, &mut b) {
        Ok(()) => u32::from_le_bytes(b),
        Err(_) => 0,
    }
}

/// The exe's image, where vtables and their RTTI lie.
const IMAGE: std::ops::Range<u32> = crate::exe::IMAGE_BASE..0x0140_9000;

/// The mangled type name (`.?AVNiNode@@`) of the class whose vtable is `vtable`, from the exe's RTTI
/// (the complete object locator in the word before the vtable, its type descriptor, the name in
/// that); `None` when `vtable` isn't one.
pub fn rtti_name(mem: &Mem, vtable: u32) -> Option<String> {
    if !IMAGE.contains(&vtable) || vtable < 4 {
        return None;
    }
    let col = first_word(mem, vtable - 4);
    if !IMAGE.contains(&col) || first_word(mem, col) != 0 {
        return None;
    }
    let td = first_word(mem, col.wrapping_add(12));
    if !IMAGE.contains(&td) {
        return None;
    }
    let mut name = Vec::new();
    for i in 0..256 {
        let mut b = [0u8; 1];
        mem.peek(td.wrapping_add(8 + i), &mut b).ok()?;
        if b[0] == 0 {
            break;
        }
        name.push(b[0]);
    }
    let name = String::from_utf8(name).ok()?;
    name.starts_with(".?A").then_some(name)
}

/// `NV_TOUCH=<spec file>`, read once at `Engine::new` ([`crate::mem::TouchSpec`]): `None` when unset,
/// and a stop when the file can't be read or isn't a spec (a census that silently tags nothing would
/// read as "nothing touches the family").
pub fn touch_spec() -> Option<TouchSpec> {
    let path = std::env::var("NV_TOUCH").ok()?;
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("NV_TOUCH={path}: can't read the spec: {e}"));
    let spec = TouchSpec::parse(&text).unwrap_or_else(|e| panic!("NV_TOUCH={path}: {e}"));
    println!(
        "NV_TOUCH: {} vtables, {} allocating functions, tags {}",
        spec.vtables.len(),
        spec.allocs.len(),
        spec.names.join(", ")
    );
    Some(spec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::Ret;

    /// A class's vtable at `vtable` with the exe's RTTI behind it: the complete object locator in
    /// the word before, its type descriptor, the mangled name in that.
    fn fake_class(mem: &mut Mem, vtable: u32, name: &str) {
        let col = vtable + 0x100;
        let td = vtable + 0x200;
        mem.map(vtable & !0xfff, 0x1000);
        mem.set_u32(vtable - 4, col);
        mem.set_u32(col, 0);
        mem.set_u32(col + 12, td);
        mem.set_cstr(td + 8, name.as_bytes());
    }

    #[test]
    fn the_census_points_read_a_folder_and_frames() {
        assert_eq!(
            CensusAt::parse(r"C:\out,9700, 12000"),
            Some(CensusAt {
                folder: PathBuf::from(r"C:\out"),
                frames: vec![9700, 12000]
            })
        );
        assert_eq!(CensusAt::parse(r"C:\out"), None);
        assert_eq!(CensusAt::parse(r"C:\out,soon"), None);
        assert_eq!(CensusAt::parse(",5"), None);
    }

    #[test]
    fn rtti_names_a_vtable_and_nothing_else() {
        let mut e = Engine::new();
        fake_class(&mut e.mem, 0x0110_0400, ".?AVThing@@");
        assert_eq!(
            rtti_name(&e.mem, 0x0110_0400).as_deref(),
            Some(".?AVThing@@")
        );
        assert_eq!(rtti_name(&e.mem, 0x0110_0404), None);
        assert_eq!(rtti_name(&e.mem, 0x2000_0000), None);
        assert_eq!(rtti_name(&e.mem, 0), None);
    }

    /// Blocks are put on the first function of their allocation path past the allocator's own and
    /// its caller, split by class; a block with no allocator path on the function running when it was
    /// made and its caller.
    #[test]
    fn the_heap_census_groups_blocks_by_allocating_function_and_class() {
        let mut e = Engine::new();
        e.mem.enable_block_check(false, false);
        fake_class(&mut e.mem, 0x0110_0400, ".?AVThing@@");
        let path = [0x0060_0000, 0x0050_0000, 0x0040_1000, 0x00aa_3e40];
        for _ in 0..2 {
            e.mem.note_path(&path);
            let b = e.mem.alloc(0x40);
            e.mem.set_u32(b, 0x0110_0400);
        }
        e.mem.note_path(&path);
        let plain = e.mem.alloc(0x13);
        e.mem.set_u32(plain, 0x0110_0404);
        e.mem.set_check_frame(0x0070_0000, 0x0071_0000);
        e.mem.alloc(0x100);
        let rows = heap_rows(&e.mem);
        let find = |f: u32, v: u32| {
            rows.iter()
                .find(|r| r.function == f && r.vtable == v)
                .cloned()
                .unwrap_or_else(|| panic!("no row {f:08x} {v:08x}: {rows:?}"))
        };
        let things = find(0x0050_0000, 0x0110_0400);
        assert_eq!(
            (
                things.caller,
                things.class.as_str(),
                things.blocks,
                things.bytes,
                things.held
            ),
            (0x0060_0000, ".?AVThing@@", 2, 0x80, 0x80)
        );
        let plain = find(0x0050_0000, 0);
        assert_eq!(
            (plain.class.as_str(), plain.blocks, plain.bytes, plain.held),
            ("", 1, 0x13, 0x18)
        );
        let running = find(0x0070_0000, 0);
        assert_eq!((running.caller, running.bytes), (0x0071_0000, 0x100));
        let text = heap_tsv(&e.mem, 42);
        assert!(
            text.starts_with("# heap census at main's frame 42:"),
            "{text}"
        );
        assert!(
            text.contains("00500000\t00600000\t01100400\t.?AVThing@@\t2\t128\t128\n"),
            "{text}"
        );
    }

    /// Through the engine: the touch census names the function that read a tagged object and its
    /// caller, also for a virtual call whose vtable read is the access.
    #[test]
    fn touch_hits_name_the_function_and_its_caller() {
        let mut e = Engine::new();
        e.mem
            .enable_touch(TouchSpec::parse("vtable 01100400 20 thing\n").expect("spec"));
        fake_class(&mut e.mem, 0x0110_0400, ".?AVThing@@");
        e.mem.set_u32(0x0110_0400, 0x0040_5000);
        let obj = e.mem.alloc(0x20);
        e.register(0x0040_5000, |_, _| Ret::default());
        e.register(0x0040_1100, |e, a| {
            e.mem.set_u32(a[0], 0x0110_0400);
            Ret::default()
        });
        e.register(0x0040_2000, |e, a| {
            let v = e.mem.u32(a[0] + 0x1c);
            e.vcall(a[0], 0, &[]);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(0x0040_3000, |e, a| e.call(0x0040_2000, a));
        e.call(0x0040_1100, &[obj]);
        e.call(0x0040_3000, &[obj]);
        let hits: Vec<_> = e
            .mem
            .touch_hits()
            .into_iter()
            .map(|(h, n)| (h.function, h.caller, h.offset, h.len, h.write, n))
            .collect();
        assert_eq!(
            hits,
            [
                (0x0040_1100, 0, 0, 4, true, 1),
                (0x0040_2000, 0x0040_3000, 0, 4, false, 1),
                (0x0040_2000, 0x0040_3000, 0x1c, 4, false, 1)
            ]
        );
        let text = touch_tsv(&e.mem, 7);
        assert!(text.contains("# tag thing: 1 objects tagged\n"), "{text}");
        assert!(
            text.contains("00402000\t00403000\tthing\t01100400\t1c\t4\tr\t1\n"),
            "{text}"
        );
    }
}
