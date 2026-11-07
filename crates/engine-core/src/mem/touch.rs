//! `NV_TOUCH` (a diagnostic of the typed migration, not game behaviour): which functions read and
//! write the fields of chosen objects, and at which offsets. The touch census (ADR-0014, step T3)
//! that says whether a family's objects can move out of guest memory: nothing outside the family
//! may touch them.
//!
//! Objects are tagged three ways, all read from a spec file (`NV_TOUCH=<file>`):
//! - `vtable <addr> <size> <tag>`: a store of `addr` as a word tags the `size` bytes it starts
//!   (the store every constructor makes, and a destructor again; `size` 0: the heap block it
//!   starts, else 4). So objects of the class are tagged wherever they are made: on the heap,
//!   embedded in another object, on a stack or in the image's data.
//! - `alloc <addr> <tag>`: a heap block whose allocation the function at `addr` asked for (the
//!   first function of its path past the allocator's own, [`ALLOC_WRAPPERS`]) is tagged whole: a
//!   family's buckets and nodes, which have no vtable.
//! - `made <addr> <tag>`: a heap block not from the game's allocator (a platform object's buffer, a
//!   `VirtualAlloc`) made while the function or platform entry at `addr` ran.
//!
//! The pages that hold a tagged object get the page state [`TOUCH`]: the fast path of every load
//! and store (`Mem::fast_at`) misses it as it misses an unmapped page, so the access takes the
//! slow path, which does it as for a mapped page and counts it with the function running, its
//! caller, the object's tag and class, the offset in the object, the length and whether it wrote.
//! With no tagged object no page has the state, and nothing changes for any access: the fast path
//! is the same instructions, and the slow paths test the state only where they already found a
//! page that is not plainly mapped.
//!
//! A tag ends when its heap block is freed (or a range holding it unmapped). The census needs the
//! block check (`NV_BLOCK_CHECK`, at least `free,log`), which names the running function and makes
//! every `with_stack` local a heap block, so a tagged stack object's tag ends with its frame.

use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;

use super::{Mem, MAPPED, PAGES, PAGE_BITS};

/// A page's byte in the table of mapped pages when the page is mapped and holds part of a tagged
/// object: an access to it takes the slow path, which counts it ([`Mem::touch_note`]). Its low bit
/// is [`MAPPED`]'s, so the slow paths' test of "mapped" (`state & 1`) takes it as mapped.
pub(super) const TOUCH: u8 = 3;
const _: () = assert!(TOUCH & 1 == MAPPED && TOUCH != MAPPED);

/// The game's allocator and the wrapper nearly every `new` goes through (`operator new` at
/// `00401000`, which asks the memory manager `00aa3e40`): a block's allocating function is the
/// first one of its path that is neither.
pub const ALLOC_WRAPPERS: [u32; 2] = [0x00aa_3e40, 0x0040_1000];

/// The first function of an allocation path (innermost first, as the block check keeps it) that is
/// not the allocator's own ([`ALLOC_WRAPPERS`]); 0 when there is none.
pub fn allocating_function(path: &[u32]) -> u32 {
    path.iter()
        .copied()
        .find(|f| !ALLOC_WRAPPERS.contains(f))
        .unwrap_or(0)
}

/// What `NV_TOUCH` tags ([module documentation](self)).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TouchSpec {
    /// Vtable address -> (object size, 0 for the block it starts; tag).
    pub vtables: HashMap<u32, (u32, u16)>,
    /// Allocating function -> tag.
    pub allocs: HashMap<u32, u16>,
    /// Function that ran when a block not from the game's allocator was made -> tag.
    pub made: HashMap<u32, u16>,
    /// The tags' names, by tag number.
    pub names: Vec<String>,
}

impl TouchSpec {
    /// Reads a spec: one rule a line (`vtable <hex> <hex size> <tag>`, `alloc <hex> <tag>`,
    /// `made <hex> <tag>`);
    /// blank lines and `#` comments are skipped.
    pub fn parse(text: &str) -> Result<TouchSpec, String> {
        let mut spec = TouchSpec::default();
        let hex = |s: &str, line: usize| {
            u32::from_str_radix(s.trim_start_matches("0x"), 16)
                .map_err(|_| format!("line {line}: {s:?} is not a hex number"))
        };
        for (i, raw) in text.lines().enumerate() {
            let line = i + 1;
            let words: Vec<&str> = raw
                .split('#')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .collect();
            match words.as_slice() {
                [] => {}
                ["vtable", addr, size, tag] => {
                    let t = spec.tag(tag);
                    spec.vtables.insert(hex(addr, line)?, (hex(size, line)?, t));
                }
                ["alloc", addr, tag] => {
                    let t = spec.tag(tag);
                    spec.allocs.insert(hex(addr, line)?, t);
                }
                ["made", addr, tag] => {
                    let t = spec.tag(tag);
                    spec.made.insert(hex(addr, line)?, t);
                }
                _ => return Err(format!("line {line}: {raw:?} is not a touch rule")),
            }
        }
        Ok(spec)
    }

    /// The number of the tag named `name`, made when new.
    pub fn tag(&mut self, name: &str) -> u16 {
        match self.names.iter().position(|n| n == name) {
            Some(i) => i as u16,
            None => {
                self.names.push(name.to_string());
                (self.names.len() - 1) as u16
            }
        }
    }
}

/// One kind of access to a tagged object ([`Mem::touch_hits`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TouchHit {
    /// The function running (the innermost one entered through `Engine::call`) and its caller.
    pub function: u32,
    pub caller: u32,
    /// The object's tag and class: the vtable that tagged it, or the allocating function.
    pub tag: u16,
    pub class: u32,
    /// Where in the object the access starts (0 when it starts before it), its length in the
    /// object (at most 255), and whether it wrote.
    pub offset: u32,
    pub len: u8,
    pub write: bool,
}

/// A tagged object.
#[derive(Debug, Clone, Copy)]
struct Object {
    end: u32,
    tag: u16,
    class: u32,
}

pub(super) struct Touch {
    spec: TouchSpec,
    /// Tagged objects by start (they don't overlap: a new one replaces those it overlaps).
    objects: BTreeMap<u32, Object>,
    /// Tagged objects on each page.
    pages: Vec<u16>,
    /// The accesses counted so far. Loads count through `&Mem`, so behind a lock (only the thread
    /// holding the engine takes it).
    hits: Mutex<HashMap<TouchHit, u64>>,
    /// Objects tagged so far, by tag.
    tagged: Vec<u64>,
}

impl Touch {
    pub(super) fn new(spec: TouchSpec) -> Self {
        let tags = spec.names.len();
        Touch {
            spec,
            objects: BTreeMap::new(),
            pages: vec![0; PAGES],
            hits: Mutex::new(HashMap::new()),
            tagged: vec![0; tags],
        }
    }
}

impl Mem {
    /// Turns the touch census on with `spec` ([module documentation](self)). Needs the block check
    /// for the running function and the allocation paths: turns it on (reporting, not stopping, and
    /// checking at frees only) when it is off.
    pub fn enable_touch(&mut self, spec: TouchSpec) {
        if self.check.is_none() {
            self.enable_block_check(false, false);
        }
        self.touch = Some(Box::new(Touch::new(spec)));
        self.store_diag = true;
    }

    /// Whether the touch census is on.
    #[inline(always)]
    pub fn touch_on(&self) -> bool {
        self.touch.is_some()
    }

    /// The touch census's tag names, by number (empty when it is off).
    pub fn touch_names(&self) -> Vec<String> {
        self.touch
            .as_deref()
            .map(|t| t.spec.names.clone())
            .unwrap_or_default()
    }

    /// Objects tagged so far, by tag number.
    pub fn touch_tagged(&self) -> Vec<u64> {
        self.touch
            .as_deref()
            .map(|t| t.tagged.clone())
            .unwrap_or_default()
    }

    /// The accesses to tagged objects counted so far, with their counts.
    pub fn touch_hits(&self) -> Vec<(TouchHit, u64)> {
        let Some(t) = self.touch.as_deref() else {
            return Vec::new();
        };
        let hits = t.hits.lock().unwrap_or_else(|p| p.into_inner());
        let mut v: Vec<_> = hits.iter().map(|(h, n)| (*h, *n)).collect();
        v.sort();
        v
    }

    /// Tags `[addr, addr + len)` with tag `tag` and class `class` (what the spec's rules do; for a
    /// tool or test that tags by hand). Objects it overlaps lose their tags.
    pub fn touch_tag(&mut self, addr: u32, len: u32, tag: u16, class: u32) {
        let Some(t) = self.touch.as_deref_mut() else {
            return;
        };
        let end = addr.saturating_add(len.max(1));
        if let Some((&start, o)) = t.objects.range(..=addr).next_back() {
            if start == addr && o.end == end && o.tag == tag && o.class == class {
                return;
            }
        }
        self.touch_forget(addr, end - addr);
        let t = self.touch.as_deref_mut().expect("touch is on");
        t.objects.insert(addr, Object { end, tag, class });
        if let Some(n) = t.tagged.get_mut(tag as usize) {
            *n += 1;
        }
        let pages: Vec<usize> =
            ((addr >> PAGE_BITS) as usize..=((end - 1) >> PAGE_BITS) as usize).collect();
        for &p in &pages {
            t.pages[p] = t.pages[p].saturating_add(1);
        }
        for p in pages {
            if self.page_state(p) == MAPPED {
                self.set_state(p, p + 1, TOUCH);
            }
        }
    }

    /// Drops the tags of the objects that overlap `[addr, addr + len)`; a page left with none is
    /// plainly mapped again.
    pub(super) fn touch_forget(&mut self, addr: u32, len: u32) {
        let Some(t) = self.touch.as_deref_mut() else {
            return;
        };
        let end = addr as u64 + len.max(1) as u64;
        let mut gone = Vec::new();
        if let Some((&start, o)) = t.objects.range(..addr).next_back() {
            if o.end > addr {
                gone.push(start);
            }
        }
        gone.extend(
            t.objects
                .range(addr..)
                .take_while(|(&s, _)| (s as u64) < end)
                .map(|(&s, _)| s),
        );
        let mut freed = Vec::new();
        for s in gone {
            let o = t.objects.remove(&s).expect("listed");
            for p in (s >> PAGE_BITS) as usize..=((o.end - 1) >> PAGE_BITS) as usize {
                t.pages[p] = t.pages[p].saturating_sub(1);
                if t.pages[p] == 0 {
                    freed.push(p);
                }
            }
        }
        for p in freed {
            if self.page_state(p) == TOUCH {
                self.set_state(p, p + 1, MAPPED);
            }
        }
    }

    /// A store of the word `v` at `a` with the touch census on: a vtable in the spec tags the
    /// object it starts.
    #[cold]
    pub(super) fn touch_store_word(&mut self, a: u32, v: u32) {
        let Some(t) = self.touch.as_deref() else {
            return;
        };
        let Some((size, tag)) = t.spec.vtables.get(&v).copied() else {
            return;
        };
        let size = match size {
            0 => self.check_block_asked(a).unwrap_or(4),
            n => n,
        };
        self.touch_tag(a, size, tag, v);
    }

    /// A heap block `[addr, addr + asked)` just handed out, allocated along `path` (innermost
    /// first): tagged whole when its allocating function is in the spec.
    pub(super) fn touch_alloc(&mut self, addr: u32, asked: u32, path: &[u32]) {
        let Some(t) = self.touch.as_deref() else {
            return;
        };
        // A block the game's allocator handed out goes by its allocating function (`alloc`); any other
        // (a `with_stack` local, a platform buffer, a `VirtualAlloc`: the running function and its
        // caller are its path) by the function that ran (`made`), so a family function's locals
        // aren't its body.
        let (f, tag) = if path.iter().any(|f| ALLOC_WRAPPERS.contains(f)) {
            let f = allocating_function(path);
            (f, t.spec.allocs.get(&f).copied())
        } else {
            let f = path.first().copied().unwrap_or(0);
            (f, t.spec.made.get(&f).copied())
        };
        if let Some(tag) = tag {
            self.touch_tag(addr, asked.max(1), tag, f);
        }
    }

    /// Counts an access of `n` bytes at `a` on a page marked [`TOUCH`] against each tagged object
    /// it reaches.
    #[cold]
    #[inline(never)]
    pub(super) fn touch_note(&self, a: u32, n: usize, write: bool) {
        let Some(t) = self.touch.as_deref() else {
            return;
        };
        let (function, caller) = self.check_frame();
        let end = a as u64 + n.max(1) as u64;
        let mut hits = t.hits.lock().unwrap_or_else(|p| p.into_inner());
        let mut count = |start: u32, o: &Object| {
            let from = a.max(start);
            let to = end.min(o.end as u64);
            let key = TouchHit {
                function,
                caller,
                tag: o.tag,
                class: o.class,
                offset: from - start,
                len: (to - from as u64).min(255) as u8,
                write,
            };
            *hits.entry(key).or_insert(0) += 1;
        };
        if let Some((&s, o)) = t.objects.range(..=a).next_back() {
            if o.end > a {
                count(s, o);
            }
        }
        if end > a as u64 + 1 {
            for (&s, o) in t
                .objects
                .range(a.saturating_add(1)..)
                .take_while(|(&s, _)| (s as u64) < end)
            {
                count(s, o);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::{KEPT, UNMAPPED};

    const VTABLE: u32 = 0x0101_7294;
    const ALLOCATOR: u32 = 0x0044_b750;

    fn census() -> Mem {
        let mut m = Mem::new();
        let spec = TouchSpec::parse(
            "# a map and its body\nvtable 01017294 40 lfm\nvtable 01017288 0 lfm-iterator\n\
             alloc 0044b750 lfm-body\n",
        )
        .expect("spec");
        m.enable_touch(spec);
        m
    }

    fn page(m: &Mem, a: u32) -> u8 {
        m.page_state((a >> PAGE_BITS) as usize)
    }

    /// Hits of one function, as (offset, len, write, count).
    fn hits_of(m: &Mem, function: u32) -> Vec<(u32, u8, bool, u64)> {
        m.touch_hits()
            .into_iter()
            .filter(|(h, _)| h.function == function)
            .map(|(h, n)| (h.offset, h.len, h.write, n))
            .collect()
    }

    #[test]
    fn a_spec_reads_its_rules_and_refuses_anything_else() {
        let s = TouchSpec::parse("vtable 0x01017294 40 lfm # map\n\nalloc 0044b750 lfm-body\n")
            .expect("spec");
        assert_eq!(s.vtables.get(&VTABLE), Some(&(0x40, 0)));
        assert_eq!(s.allocs.get(&ALLOCATOR), Some(&1));
        assert_eq!(s.names, ["lfm", "lfm-body"]);
        assert!(TouchSpec::parse("vtable 01017294 lfm\n").is_err());
        assert!(TouchSpec::parse("ctor 00449940 40 lfm\n").is_err());
        let made = TouchSpec::parse("made 7e02025c d3d\n").expect("spec");
        assert_eq!(made.made.get(&0x7e02_025c), Some(&0));
        assert!(TouchSpec::parse("alloc zz lfm\n").is_err());
    }

    #[test]
    fn the_allocating_function_is_the_first_past_the_allocator() {
        assert_eq!(
            allocating_function(&[0x00aa_3e40, 0x0040_1000, ALLOCATOR, 0x0044_2650]),
            ALLOCATOR
        );
        assert_eq!(allocating_function(&[0x00aa_3e40]), 0);
        assert_eq!(allocating_function(&[]), 0);
    }

    /// The census turns the block check on for the running function; a vtable store tags the object
    /// its spec size long, and every read and write of it is counted by function, caller, offset,
    /// length and kind, the constructor's store first. A neighbour on the same page is not counted.
    #[test]
    fn a_vtable_store_tags_its_object_and_its_accesses_are_counted() {
        let mut m = census();
        assert!(m.block_check_on());
        let obj = m.alloc(0x40);
        let other = m.alloc(0x10);
        assert_eq!(obj >> PAGE_BITS, other >> PAGE_BITS);
        assert_eq!(page(&m, obj), MAPPED);
        m.set_check_frame(0x0044_9940, 0x0044_2650);
        m.set_u32(obj, VTABLE);
        assert_eq!(page(&m, obj), TOUCH);
        m.set_check_frame(0x0055_0000, 0x0066_0000);
        m.set_u32(obj + 8, 7);
        assert_eq!(m.u32(obj + 8), 7);
        assert_eq!(m.u32(obj + 8), 7);
        m.set_u16(obj + 0x3e, 0xbeef);
        assert_eq!(m.u16(obj + 0x3e), 0xbeef);
        m.set_u32(other, 5);
        assert_eq!(m.u32(other), 5);
        assert_eq!(hits_of(&m, 0x0044_9940), [(0, 4, true, 1)]);
        assert_eq!(
            hits_of(&m, 0x0055_0000),
            [
                (8, 4, false, 2),
                (8, 4, true, 1),
                (0x3e, 2, false, 1),
                (0x3e, 2, true, 1)
            ]
        );
        let (h, _) = m.touch_hits()[0];
        assert_eq!((h.caller, h.tag, h.class), (0x0044_2650, 0, VTABLE));
        assert_eq!(m.touch_tagged(), [1, 0, 0]);
    }

    /// A read that starts before the object, a bulk copy over it and a string in it are counted
    /// with the part of the object they reach; a diagnostic's `peek` is not.
    #[test]
    fn reads_across_its_edges_and_bulk_reads_are_counted_with_the_part_reached() {
        let mut m = census();
        let _below = m.alloc(0x20);
        let obj = m.alloc(0x40);
        m.set_u32(obj, VTABLE);
        m.set_check_frame(0x0055_0000, 0);
        let _ = m.u64(obj - 4);
        let mut buf = [0u8; 0x60];
        m.read(obj - 0x10, &mut buf);
        m.set_cstr(obj + 0x10, b"ab");
        assert_eq!(m.cstr(obj + 0x10), b"ab");
        m.peek(obj, &mut buf).expect("mapped");
        assert_eq!(
            hits_of(&m, 0x0055_0000),
            [
                (0, 4, false, 1),
                (0, 0x40, false, 1),
                (0x10, 2, true, 1),
                (0x10, 3, false, 1),
                (0x12, 1, true, 1)
            ]
        );
    }

    /// Freeing the block ends the tag: the page is plainly mapped again and later accesses aren't
    /// counted. An iterator (size 0 in the spec) takes the block it starts.
    #[test]
    fn freeing_the_block_ends_the_tag_and_an_iterator_takes_its_block() {
        let mut m = census();
        let it = m.alloc(0x24);
        m.set_u32(it, 0x0101_7288);
        assert_eq!(page(&m, it), TOUCH);
        m.set_check_frame(0x0055_0000, 0);
        let _ = m.u32(it + 0x20);
        m.free(it);
        assert_eq!(page(&m, it), MAPPED);
        let again = m.alloc(0x24);
        let _ = m.u32(again + 0x20);
        assert_eq!(hits_of(&m, 0x0055_0000), [(0x20, 4, false, 1)]);
        assert_eq!(m.touch_tagged(), [0, 1, 0]);
        m.check_heap();
    }

    /// A block whose allocation the listed function asked for (through `operator new` and the
    /// memory manager) is tagged whole with that function as its class; one from another function
    /// is not.
    #[test]
    fn a_block_from_a_listed_allocating_function_is_tagged_whole() {
        let mut m = census();
        m.note_path(&[0x0044_2650, ALLOCATOR, 0x0040_1000, 0x00aa_3e40]);
        let body = m.alloc(0x30);
        m.note_path(&[ALLOCATOR, 0x0050_0000, 0x0040_1000, 0x00aa_3e40]);
        let foreign = m.alloc(0x30);
        m.set_check_frame(0x0055_0000, 0);
        m.set_u32(body + 0x2c, 1);
        m.set_u32(foreign, 1);
        let hits = m.touch_hits();
        assert_eq!(hits.len(), 1);
        let (h, n) = hits[0];
        assert_eq!(
            (h.tag, h.class, h.offset, h.len, h.write, n),
            (2, ALLOCATOR, 0x2c, 4, true, 1)
        );
        // A block made while the listed function runs but not through the allocator (a local): not
        // its body.
        m.set_check_frame(ALLOCATOR, 0x0044_2650);
        let local = m.alloc(0x30);
        m.set_u32(local, 1);
        assert_eq!(m.touch_hits().len(), 1);
    }

    /// A block not from the game's allocator (a platform buffer) is tagged by the function that ran.
    #[test]
    fn a_block_made_while_a_listed_function_ran_is_tagged_whole() {
        let mut m = Mem::new();
        m.enable_touch(
            TouchSpec::parse(
                "made 7e02025c d3d
",
            )
            .expect("spec"),
        );
        m.set_check_frame(0x7e02_025c, 0x00e6_8ef0);
        let buffer = m.alloc(0x100);
        m.set_check_frame(0x0044_1000, 0);
        let plain = m.alloc(0x100);
        m.set_check_frame(0x00e6_9000, 0x00e6_8ef0);
        m.set_u32(buffer + 0xf0, 1);
        m.set_u32(plain, 1);
        let hits = m.touch_hits();
        assert_eq!(hits.len(), 1);
        assert_eq!(
            (hits[0].0.class, hits[0].0.offset, hits[0].0.function),
            (0x7e02_025c, 0xf0, 0x00e6_9000)
        );
    }

    /// Unmapping a range drops the tags in it, and the pages go back as unmapped pages do.
    #[test]
    fn an_unmap_drops_the_tags_in_its_range() {
        let mut m = census();
        m.map(0x0100_0000, 0x2000);
        m.set_u32(0x0100_0ff0, VTABLE);
        assert_eq!(page(&m, 0x0100_0000), TOUCH);
        assert_eq!(page(&m, 0x0100_1000), TOUCH);
        m.unmap(0x0100_0000, 0x2000);
        assert!(matches!(page(&m, 0x0100_0000), KEPT | UNMAPPED));
        m.map(0x0100_0000, 0x1000);
        assert_eq!(page(&m, 0x0100_0000), MAPPED);
        m.check_pages();
    }

    /// Without the census no page has the state, and a run's accesses are the same as before.
    #[test]
    fn without_the_census_nothing_is_tagged() {
        let mut m = Mem::new();
        let obj = m.alloc(0x40);
        m.set_u32(obj, VTABLE);
        assert_eq!(page(&m, obj), MAPPED);
        assert!(m.touch_hits().is_empty());
        assert!(!m.touch_on());
    }
}
