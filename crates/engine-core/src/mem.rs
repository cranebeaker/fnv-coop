//! The game's 32-bit address space.
//!
//! Game objects live here at their real field offsets, and pointers are the
//! 32-bit addresses the game uses. Memory is paged (4 KiB) and sparse: a
//! page exists once something maps it. Reading an unmapped address is a
//! fault that names the address, so a translation that reads data nobody
//! provided stops at once instead of computing with zeros.
//!
//! Layout: every [`Mem`] reserves one block of 4 GB of host address space
//! when it is made, so guest address `a` is host address `base + a`. The
//! block costs no memory until pages of it are used. [`Mem::map`] makes
//! whole pages of it usable (committed, reading zero where new) and
//! [`Mem::unmap`] makes them unusable again, dropping their contents: a
//! small unmap keeps the pages committed, to be zeroed in place if they are
//! mapped again, and a large one hands them back to the system
//! ([`KEEP_SPAN`]). A table of one byte per page (1 MB, in the same block
//! just below the guest space, so `base` finds it too) says which pages are
//! mapped. Every guest access tests the table first, and an unmapped page
//! is the fault above, so no guest access reaches a page that is not
//! mapped.
//!
//! A load or store of one value (at most 8 bytes) tests the bytes of its
//! page and of the next page with one 2-byte load. When both pages are
//! mapped (nearly always) the access is one host load or store at
//! `base + a`, even across the end of the page, since the guest space is
//! one block. The table's address and the data's are both worked out from
//! `base` and `a` alone, neither from the other's result, so the processor
//! starts both loads at once. Otherwise (the last mapped page of a range,
//! or an unmapped page) an out-of-line path tests the page alone and,
//! failing that, goes page by page and faults where the old page table did.
//! (Bytes rather than bits, and one test of the pair rather than tests of
//! the offset in the page and of the page: each of those costs instructions
//! on every access, and measured slower than the old page table on random
//! loads and stores.)
//!
//! The host calls are `VirtualAlloc` and `VirtualFree` on Windows (the
//! block starts with no access, and pages are committed as they are mapped
//! and decommitted as they are handed back), `mmap` on Linux and macOS (the
//! block is readable and writable from the start; see `host` below for
//! why). The block needs a 64-bit host.
//!
//! Layout of the guest space: the whole 4 GB a large-address-aware 32-bit process has on
//! 64-bit Windows (the community's 4 GB patch marks FalloutNV.exe so, and Viva New Vegas
//! needs it), from low to high:
//! - `0x00010000..0x00400000`: nothing (tests map their own memory here);
//! - `0x00400000..0x01409000`: the executable's image; its data sections at their own
//!   addresses (`.rdata` 0x00FDF000, `.data` 0x01183000, ...), mapped from the player's
//!   installed FalloutNV.exe by [`crate::exe`] or set up by a test;
//! - `0x01409000..`[`HEAP_BASE`]: free for `VirtualAlloc` reservations at an address the
//!   caller chooses (`units/platform.rs`; the game's `MemoryPool`s would search 16 MB
//!   boundaries here, but the port's memory manager does not make them);
//! - [`HEAP_BASE`]`..`[`HEAP_LOW_END`]: the heap's lower range, handed out by [`Mem::alloc`]
//!   (the translation of the game's allocator is `platform`, see `units/platform.rs`; its free
//!   space and the choice of where a block goes are in `mem/heap.rs`). Every heap block, every
//!   game thread's stack and TLS block, every `HeapAlloc`, `malloc` and address-0
//!   `VirtualAlloc` comes from here first;
//! - [`HEAP_LOW_END`]`..`[`HEAP_HIGH_BASE`]: the platform layer's own addresses
//!   ([`PLATFORM_AREA`]): the code addresses of its COM objects and procedures (DirectInput,
//!   DirectSound, DirectShow, Direct3D 9, NVAPI, `GetProcAddress` answers, from 0x7E000000),
//!   module handles (0x7E100000..0x7E130000), the Bink state page (0x7E140000, mapped) and the
//!   D3DX handles (from 0x7E300000, never mapped). The heap never comes here, and a
//!   `VirtualAlloc` at a chosen address here fails;
//! - [`HEAP_HIGH_BASE`]`..`[`HEAP_END`]: the heap's upper range, used only once the lower one
//!   cannot serve a request (`mem/heap.rs`), so a run that fits in the lower range places
//!   every block where it did before the upper range existed;
//! - [`HEAP_END`]`..0xFFFF0000`: free for `VirtualAlloc` reservations at a chosen address,
//!   and the freed-block check's words ([`freed_word`], never mapped by the heap);
//! - the last 64 KB: not user space on Windows; the port maps nothing there (tests do, to test
//!   the end of the space).
//!
//! The table of mapped pages is host memory just below the guest space (above), not a guest
//! region. Guest addresses at or above 0x80000000 are ordinary addresses: [`Mem`] takes any
//! `u32`, and the platform's answers describe the 4 GB process (`GetSystemInfo` in
//! `units/platform.rs`, `GlobalMemoryStatus(Ex)` in engine-platform's `process.rs`). Translated
//! code that compares two addresses as signed stays right because no block spans 0x80000000:
//! the platform's addresses separate the ranges.
//!
//! A stopgap, as the 4 GB patch is: the modern systems are what lift the limit for good. A
//! vanilla preset (not built yet) goes at `Heap::cross` and at the platform's two answers.

use std::collections::{BTreeMap, HashMap};

mod heap;

const PAGE_BITS: u32 = 12;
const PAGE: usize = 1 << PAGE_BITS;
const PAGES: usize = 1 << (32 - PAGE_BITS);

#[cfg(not(target_pointer_width = "64"))]
compile_error!(
    "the guest memory reserves the whole 4 GB guest space at once: it needs a 64-bit host"
);

/// Bytes of the guest space: the whole 32-bit address space. The table of mapped pages
/// ([`TABLE`] bytes) comes just before it in the same host block ([`Space`]).
#[cfg(target_pointer_width = "64")]
const SPACE: usize = 1 << 32;
/// Bytes of the table of mapped pages: one per page, one more for the page past the end of the
/// space (always 0, for [`Mem::fast_at`]'s pair test on the last page), rounded up to 64 KB so
/// the guest space after it starts on a 64 KB boundary of the block. Small enough that its
/// distance below the guest space fits in a load instruction's own offset.
const TABLE: usize = PAGES + (64 << 10);

/// A page's byte in the table of mapped pages when the page is not mapped and holds nothing: on
/// Windows not committed, elsewhere reading zero. Every page starts so.
const UNMAPPED: u8 = 0;
/// A page's byte in the table when the page is mapped: the only state guest accesses use.
const MAPPED: u8 = 1;
/// A page's byte in the table when the page is not mapped but stays committed, possibly still
/// holding the bytes it had ([`KEEP_SPAN`]). Guest accesses fault on it as on an unmapped page;
/// mapped again, it is zeroed in place.
const KEPT: u8 = 2;
/// The table's bytes for a page and the next both mapped ([`Mem::fast_at`]).
const MAPPED_PAIR: u16 = u16::from_ne_bytes([MAPPED, MAPPED]);

/// [`Mem::unmap`] keeps the pages of a call that unmaps at most this many (4 MB) committed,
/// marked [`KEPT`], instead of handing them back to the system: mapped again, they are zeroed
/// in place (a 4 KiB write each) where a page handed back is committed afresh and faults on its
/// first touch. That covers the game's own `VirtualFree`/`VirtualAlloc` churn through
/// `units/platform.rs`: a memory pool's single pages, the C runtime's 32 KB groups and the
/// scrap heaps' halves, which shrink to their floor and grow again by doubling (from 64 KB to
/// at most 8 MB). On Windows, handing back and committing again cost a system call each (the
/// hand-back also interrupts the process's other running threads to drop the page from their
/// address caches) and a fault on the page's first touch. Measured (2026-11-01,
/// `tests::bench_commit_and_decommit`): one page committed, written and decommitted took
/// 4.5 us handed back (8 us with three other threads busy) against 30 ns kept, and 60 to 90 ns
/// with the boxed pages before the flat block; a scrap heap grown from 64 KB to 1 MB and shrunk
/// back took 530 us handed back (920 us busy) against 5 us kept, and 210 us before. A larger
/// unmap (the 8 MB of heap pages under a new reservation) hands its pages back at once.
const KEEP_SPAN: usize = 1024;
/// The most pages (16 MB) kept at once. An unmap that would keep more first hands back kept
/// pages ([`Mem::hand_back_kept`]), the next ones round the space from where the last hand-back
/// stopped: at least as many as it would go over by, and, while it has made fewer than
/// [`KEEP_BATCH_RUNS`] host calls, up to [`KEEP_BATCH`]. Measured 2026-11-02 (standalone release
/// build of this module, three runs each; the round-2 review's patterns):
/// - Handing every kept page back at once (the first version): with 4096 kept pages in 4096
///   runs, one one-page unmap took 1.7 ms and 4096 host calls (3.2-3.5 ms in the test build).
/// - Handing back only what it went over by (the second): a window of 64 pages sliding forward
///   (a page unmapped and one mapped a step) made a host call at every unmap, 2.3-2.6 us a step
///   against 1.7-1.8 us for the first version, and 5.8-6.2 us against 2.0-2.4 us with three
///   other threads busy, which each call interrupts. Its look for the next kept page read the
///   1 MB table, up to all of it when the kept pages lay behind where the last hand-back
///   stopped: 90-130 us for that one unmap.
/// - Now: the window 1.6-1.7 us a step idle and 2.1-2.3 us busy (a host call per 256 unmaps;
///   the unmap that makes it, 1 MB handed back, 0.1-0.2 ms); one whose look goes round the
///   space ([`Mem::kept_groups`]) 2-4 us. With the kept pages scattered, a one-page unmap makes
///   at most [`KEEP_BATCH_RUNS`] host calls. The slowest took 4-6 us on an idle machine (9-14 us
///   with another build running), but 27-79 us with three other threads busy, which each call
///   interrupts (the second version, one call per unmap: 30-44 us busy). The idle figure is the
///   repair's; the others the third review's (600 unmaps past the limit with 4096 kept pages in
///   4096 runs, three rounds, twice).
/// - An unmap of more pages that the next [`KEEP_BATCH_RUNS`] runs of kept pages cannot make
///   room for hands its own pages back instead of keeping them ([`Mem::kept_runs_hold`]), a
///   host call per run of its range. With the kept pages scattered, a 1024-page unmap (half of
///   an 8 MB scrap heap, which the game decommits in one `VirtualFree`) handed back a kept page
///   per page it went over by: 1024 host calls, 0.9-1.2 ms idle and 3.7-3.9 ms with three other
///   threads busy (the fourth review's measurement). Now one call: 0.14-0.27 ms idle and
///   0.21-0.31 ms busy, the old page table's unmap 0.33-0.36 ms (2026-11-02, five rounds each,
///   other builds running).
const KEEP_MAX: usize = 4096;
/// The most kept pages (1 MB) a hand-back at the limit hands back when it needs fewer: a window
/// of pages sliding forward makes one host call per this many one-page unmaps at the limit, not
/// one each ([`KEEP_MAX`]). With 64, the window busy took 2.3-2.5 us a step.
const KEEP_BATCH: usize = 256;
/// The most host calls (runs of kept pages) an unmap at [`KEEP_MAX`] makes handing back kept
/// pages: a hand-back that has what it needs stops after this many short of [`KEEP_BATCH`], and
/// an unmap that this many runs cannot make room for hands its own pages back instead.
const KEEP_BATCH_RUNS: usize = 8;
/// The pages each bit of [`Mem::kept_groups`] stands for.
const KEEP_GROUP: usize = 64;
/// The words of [`Mem::kept_groups`] (2 KB).
const KEEP_GROUP_WORDS: usize = PAGES / KEEP_GROUP / 64;

/// The host's virtual memory calls: reserve a block of address space with no access, commit
/// part of it (readable and writable, zero-filled where new), hand part of it back (its contents
/// dropped; committed again, it reads zero) and release the whole block. A call the system
/// refuses panics with the system's reason: guest memory cannot go on without it.
#[cfg(windows)]
mod host {
    use std::ffi::c_void;
    use windows::Win32::System::Memory::{
        VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_DECOMMIT, MEM_RELEASE, MEM_RESERVE,
        PAGE_NOACCESS, PAGE_READWRITE,
    };

    pub fn reserve(len: usize) -> *mut u8 {
        // SAFETY: a new reservation wherever the system puts it; nothing existing is touched.
        let p = unsafe { VirtualAlloc(None, len, MEM_RESERVE, PAGE_NOACCESS) };
        assert!(
            !p.is_null(),
            "could not reserve {len:#x} bytes of address space for guest memory: {}",
            std::io::Error::last_os_error()
        );
        p.cast()
    }

    /// # Safety
    /// `[p, p + len)` lies inside a block from [`reserve`].
    pub unsafe fn commit(p: *mut u8, len: usize) {
        // Pages already committed keep their contents (the call allows them in the range).
        let r = VirtualAlloc(
            Some(p.cast_const().cast::<c_void>()),
            len,
            MEM_COMMIT,
            PAGE_READWRITE,
        );
        assert!(
            !r.is_null(),
            "could not commit {len:#x} bytes of guest memory: {}",
            std::io::Error::last_os_error()
        );
    }

    /// # Safety
    /// `[p, p + len)` lies inside a block from [`reserve`], and nothing refers into it.
    pub unsafe fn decommit(p: *mut u8, len: usize) {
        // Pages in the range that are not committed are allowed and stay so.
        if let Err(e) = VirtualFree(p.cast::<c_void>(), len, MEM_DECOMMIT) {
            panic!("could not hand back {len:#x} bytes of guest memory: {e}");
        }
    }

    /// # Safety
    /// `p` is a block from [`reserve`] of `len` bytes, and nothing uses it after this.
    pub unsafe fn release(p: *mut u8, _len: usize) {
        // Releasing a whole reservation takes size 0. A failure here (a bad block) leaves only
        // address space behind, so a drop does not panic over it.
        let _ = VirtualFree(p.cast::<c_void>(), 0, MEM_RELEASE);
    }

    /// The runs of `[p, p + len)` (part of a block from [`reserve`]) the system has committed
    /// and has not, as (offset from `p`, bytes, committed): what the tests check the table of
    /// pages against.
    #[cfg(test)]
    pub fn committed_runs(p: *mut u8, len: usize) -> Vec<(usize, usize, bool)> {
        use windows::Win32::System::Memory::{VirtualQuery, MEMORY_BASIC_INFORMATION};
        let mut runs = Vec::new();
        let mut at = 0;
        while at < len {
            let mut info = MEMORY_BASIC_INFORMATION::default();
            // SAFETY: a query of the block's own pages, into a structure of the size given.
            let n = unsafe {
                VirtualQuery(
                    Some(p.wrapping_add(at).cast_const().cast::<c_void>()),
                    &mut info,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                )
            };
            assert_ne!(n, 0, "VirtualQuery: {}", std::io::Error::last_os_error());
            let end = (info.BaseAddress as usize + info.RegionSize - p as usize).min(len);
            runs.push((at, end - at, info.State == MEM_COMMIT));
            at = end;
        }
        runs
    }
}

/// The host's virtual memory calls, as on Windows above, through the C library's `mmap` and
/// `munmap` (the standard library links it; this workspace has no `libc` dependency, so the
/// few declarations and flag values are here). The flag values are the systems' own: Linux on
/// x86-64, AArch64 and RISC-V, and macOS.
///
/// Here the whole block is readable and writable from the start. The system gives memory to a
/// page only when it is first written, and with `MAP_NORESERVE` does not count the block
/// against the memory it promises (except Linux set to strict overcommit, where a block may be
/// refused), so committing is nothing. Handing back zeroes the bytes, and
/// gives whole aligned 64 KB pieces of them back to the system by laying a fresh mapping over
/// them. (Committing and handing back each 4 KiB page with `mprotect` and a fresh mapping would
/// need the system's own pages to be 4 KiB, which they are not on macOS on Apple silicon,
/// 16 KiB, nor on some 64-bit ARM Linux systems, 16 or 64 KiB; `MAP_FIXED` needs an address on
/// a page of the system's own.)
#[cfg(unix)]
mod host {
    use std::ffi::{c_int, c_void};

    extern "C" {
        fn mmap(
            addr: *mut c_void,
            len: usize,
            prot: c_int,
            flags: c_int,
            fd: c_int,
            offset: i64,
        ) -> *mut c_void;
        fn munmap(addr: *mut c_void, len: usize) -> c_int;
    }

    /// The pieces handed back to the system whole: a multiple of every page size the systems
    /// above use.
    const GIVE_BACK: usize = 64 << 10;

    const PROT_READ: c_int = 1;
    const PROT_WRITE: c_int = 2;
    const MAP_PRIVATE: c_int = 0x02;
    const MAP_FIXED: c_int = 0x10;
    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "riscv64"
        )
    ))]
    const MAP_ANONYMOUS: c_int = 0x20;
    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "riscv64"
        )
    ))]
    const MAP_NORESERVE: c_int = 0x4000;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const MAP_ANONYMOUS: c_int = 0x1000;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const MAP_NORESERVE: c_int = 0x40;
    #[cfg(not(any(
        all(
            any(target_os = "linux", target_os = "android"),
            any(
                target_arch = "x86_64",
                target_arch = "aarch64",
                target_arch = "riscv64"
            )
        ),
        target_os = "macos",
        target_os = "ios"
    )))]
    compile_error!("guest memory: the mmap flag values for this system are not known here");

    /// What `mmap` answers when it fails (`MAP_FAILED`).
    fn failed(p: *mut c_void) -> bool {
        p as usize == usize::MAX
    }

    /// A fresh readable and writable mapping, all zero, at `addr` (`MAP_FIXED`, replacing what
    /// was there) or (null) wherever the system puts it.
    ///
    /// # Safety
    /// With an address, `[addr, addr + len)` lies inside a block from [`reserve`], on pages of
    /// the system's own, and nothing refers into it.
    unsafe fn fresh(addr: *mut c_void, len: usize) -> *mut c_void {
        let fixed = if addr.is_null() { 0 } else { MAP_FIXED };
        let p = mmap(
            addr,
            len,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE | fixed,
            -1,
            0,
        );
        assert!(
            !failed(p) && (addr.is_null() || p == addr),
            "could not map {len:#x} bytes of address space for guest memory: {}",
            std::io::Error::last_os_error()
        );
        p
    }

    pub fn reserve(len: usize) -> *mut u8 {
        // SAFETY: a new mapping wherever the system puts it; nothing existing is touched.
        unsafe { fresh(std::ptr::null_mut(), len) }.cast()
    }

    /// Nothing to do: the block is readable and writable from the start, and a page not yet
    /// written (or zeroed by [`decommit`]) reads zero.
    ///
    /// # Safety
    /// `[p, p + len)` lies inside a block from [`reserve`].
    pub unsafe fn commit(_p: *mut u8, _len: usize) {}

    /// # Safety
    /// `[p, p + len)` lies inside a block from [`reserve`], and nothing refers into it.
    pub unsafe fn decommit(p: *mut u8, len: usize) {
        // The aligned 64 KB pieces inside the range go back to the system under a fresh
        // mapping (reading zero); the bytes before and after them are zeroed in place.
        let start = p as usize;
        let lo = start.next_multiple_of(GIVE_BACK) - start;
        let hi = (start + len) / GIVE_BACK * GIVE_BACK;
        let hi = hi.saturating_sub(start);
        if lo < hi {
            std::ptr::write_bytes(p, 0, lo);
            fresh(p.add(lo).cast::<c_void>(), hi - lo);
            std::ptr::write_bytes(p.add(hi), 0, len - hi);
        } else {
            std::ptr::write_bytes(p, 0, len);
        }
    }

    /// # Safety
    /// `p` is a block from [`reserve`] of `len` bytes, and nothing uses it after this.
    pub unsafe fn release(p: *mut u8, len: usize) {
        // A failure here leaves only address space behind, so a drop does not panic over it.
        let _ = munmap(p.cast::<c_void>(), len);
    }
}

#[cfg(not(any(windows, unix)))]
compile_error!("guest memory: no virtual memory calls for this host");

/// The host block that holds the guest space, reserved by `host::reserve`: first the table of
/// mapped pages, [`TABLE`] bytes committed when the block is made (so it starts all zero:
/// [`UNMAPPED`]), page `p`'s byte at `base - TABLE + p` ([`MAPPED`], [`KEPT`] or
/// [`UNMAPPED`]); then the [`SPACE`] bytes of the guest space, guest address `a` at host address
/// `base + a`.
/// (The table below the guest space rather than above it: its distance from `base` then fits
/// in the instruction that loads from it, where 4 GB would take an instruction of its own on
/// every access.) Released when dropped.
struct Space {
    /// The host address of guest address 0.
    base: *mut u8,
}

// SAFETY: a `Space` owns its block alone, as a `Box` owns its allocation: no other value holds
// `base`, and only `Drop` releases it. [`Mem`], its only user, reads the block only through
// `&self` and writes it only through `&mut self`, so the borrow rules keep every write apart
// from any other access on any thread, exactly as they did for the boxed pages this replaced
// (which made `Mem` `Send` and `Sync` by themselves). The thread host hands the engine from one
// OS thread to the next with only one of them running it at a time (`thread_host`'s `Baton`),
// which a block of host address space serves as well as boxed pages did: the reservation is
// the process's, not a thread's.
unsafe impl Send for Space {}
// SAFETY: as for `Send` above: shared access only ever reads.
unsafe impl Sync for Space {}

impl Space {
    fn new() -> Self {
        let block = host::reserve(TABLE + SPACE);
        // SAFETY: the table lies inside the block just reserved.
        unsafe { host::commit(block, TABLE) };
        Space {
            base: block.wrapping_add(TABLE),
        }
    }

    /// Where page `p`'s byte of the table of mapped pages is (`p` at most [`PAGES`]). Worked
    /// out from `base` like the guest bytes, so one load of `base` serves both.
    #[inline(always)]
    fn flag(&self, p: usize) -> *mut u8 {
        self.base.wrapping_sub(TABLE).wrapping_add(p)
    }
}

impl Drop for Space {
    fn drop(&mut self) {
        // SAFETY: the block (which starts with the table) is this `Space`'s own, and nothing uses
        // it after this.
        unsafe { host::release(self.base.wrapping_sub(TABLE), TABLE + SPACE) }
    }
}

/// First heap address. Above the executable image (which ends below
/// 0x01410000) and clear of it, so a heap pointer never aliases image data.
pub const HEAP_BASE: u32 = 0x2000_0000;
/// End of the heap's lower range: the platform layer's addresses ([`PLATFORM_AREA`]) start
/// here. (Before the upper range the heap ran to 0x7FFF0000, through the platform's
/// addresses: a heap that grew past 0x7E000000 would have handed out the Bink state page.)
pub const HEAP_LOW_END: u32 = 0x7E00_0000;
/// Start of the heap's upper range, the first address past the 2 GB a 32-bit process has
/// without the large-address-aware flag.
pub const HEAP_HIGH_BASE: u32 = 0x8000_0000;
/// End of the heap (of its upper range). Above it up to 0xFFFF0000 stays free for reservations
/// at a chosen address and for the freed-block check's words ([`freed_word`]).
pub const HEAP_END: u32 = 0xF000_0000;
/// The platform layer's own addresses, between the heap's two ranges (module documentation).
pub const PLATFORM_AREA: std::ops::Range<u32> = HEAP_LOW_END..HEAP_HIGH_BASE;
/// The most bytes one heap block can have: the larger of the heap's two ranges (a block never
/// spans both).
pub const HEAP_LARGEST_BLOCK: u32 = {
    let low = HEAP_LOW_END - HEAP_BASE;
    let high = HEAP_END - HEAP_HIGH_BASE;
    if low > high {
        low
    } else {
        high
    }
};

/// A read or write of memory nobody mapped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fault(pub u32);

pub struct Mem {
    /// The guest space: one block of host address space, guest address `a` at host address
    /// `base + a`, and just below `base`, in the same block, the table of mapped pages ([`Space`],
    /// module documentation). Guest accesses read and write only pages the table marks
    /// [`MAPPED`]. On Windows the only pages of the guest space committed are those and the ones
    /// the table marks [`KEPT`]; elsewhere the whole block is readable and writable from the
    /// start (`host`).
    space: Space,
    /// How many pages the table marks [`KEPT`] (at most [`KEEP_MAX`]).
    kept: usize,
    /// The page the next hand-back of kept pages starts looking at ([`Mem::hand_back_kept`]).
    keep_hand: usize,
    /// One bit for each [`KEEP_GROUP`] pages (bit `g % 64` of word `g / 64` for pages
    /// `64g..64g + 64`), set exactly when the table marks one of them [`KEPT`]
    /// ([`Mem::set_state`] keeps it so): [`Mem::next_kept`] looks at these 2 KB, not at the
    /// table's 1 MB.
    kept_groups: Box<[u64; KEEP_GROUP_WORDS]>,
    /// How many times [`Mem::map`] and [`Mem::unmap`] have called the host's commit and
    /// hand-back (what the tests count).
    #[cfg(test)]
    host_calls: u32,
    /// The heap's live blocks (start -> size, rounded and with any guard; a table per region of
    /// the heap, [`heap::Regions`], so that no allocation rehashes every live block) and its free
    /// space.
    live: heap::Regions,
    heap: heap::Heap,
    /// The write watch ([`Mem::watch`]); `None` in normal runs (one test per store).
    watch: Option<Box<Watch>>,
    /// The block check ([`Mem::enable_block_check`]); `None` in normal runs.
    check: Option<Box<BlockCheck>>,
    /// The block check looks at every store ([`Mem::enable_block_check`] with `store`).
    check_store: bool,
    /// A store diagnostic is on: the write watch or the block check's store mode.
    store_diag: bool,
}

/// What a watched range saw ([`Mem::watch`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchKind {
    /// A store that changed the bytes.
    Write,
    /// [`Mem::zero`] changed the bytes.
    Zero,
    /// [`Mem::alloc`] handed (part of) the range out again (zero-filled).
    Alloc,
    /// [`Mem::free`] freed the block holding (part of) the range.
    Free,
}

/// One event in a watched range: the range's start, the address and length touched (at
/// most 8 bytes of a store are kept), the bytes before and after (little-endian).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatchHit {
    pub start: u32,
    pub addr: u32,
    pub len: u32,
    pub old: u64,
    pub new: u64,
    pub kind: WatchKind,
}

/// The ranges a diagnostic watches (start -> length and whether only frees count), a
/// count of ranges per page (the store's quick test) and the events not yet taken.
struct Watch {
    ranges: BTreeMap<u32, (u32, bool)>,
    pages: Vec<u16>,
    hits: Vec<WatchHit>,
}

/// The most events kept before [`Mem::take_watch_hits`] takes them (later ones dropped).
const WATCH_HITS_KEEP: usize = 1 << 16;

/// Bytes of guard after every block while the block check is on ([`Mem::enable_block_check`]).
pub const GUARD: u32 = 16;
/// The byte guards are filled with.
pub const GUARD_BYTE: u8 = 0xFD;

/// The block check (a diagnostic, not game behaviour): every heap block (and so every
/// `Engine::with_stack` local) gets [`GUARD`] bytes of [`GUARD_BYTE`] right after the size
/// asked for; a store into them (with `store`) or guard bytes found changed when the block is
/// freed is a write past the block's end, reported with the functions involved.
struct BlockCheck {
    /// Check every store (else only at free).
    store: bool,
    /// Stop (panic) at the first finding instead of only reporting it.
    stop: bool,
    /// Live blocks: size asked for and the function that was running when it was allocated.
    blocks: HashMap<u32, (u32, u32)>,
    /// Guard ranges: start -> (end, block).
    guards: BTreeMap<u32, (u32, u32)>,
    /// Guard ranges per page (the store's quick test).
    pages: Vec<u16>,
    /// The innermost function entered through `Engine::call` (the engine keeps it up to date).
    current_fn: u32,
    /// Findings not yet taken ([`Mem::take_block_reports`]).
    reports: Vec<String>,
    /// With [`Mem::enable_freed_check`]: freed blocks held back from reuse, oldest first
    /// (address, rounded size), each guarded whole so that a store into it is reported, until
    /// more than `quarantine_limit` bytes ([`QUARANTINE_BYTES`]) wait.
    quarantine: Option<std::collections::VecDeque<(u32, u32)>>,
    quarantine_bytes: u64,
    quarantine_limit: u64,
    /// The quarantined blocks: size asked for, the function running when it was allocated and
    /// the one running when it was freed.
    freed: HashMap<u32, (u32, u32, u32)>,
    /// The engine's calls (innermost first) the game's allocator or deallocator gave for the next
    /// block it hands out or frees ([`Mem::note_path`]), the allocation paths of live blocks, and
    /// both paths of quarantined ones.
    pending_path: Vec<u32>,
    alloc_paths: HashMap<u32, Vec<u32>>,
    freed_paths: HashMap<u32, (Vec<u32>, Vec<u32>)>,
}

/// Bytes of freed blocks the freed-block check keeps from reuse at most.
const QUARANTINE_BYTES: u64 = 64 << 20;
/// The word a freed block at `addr` is filled with while it waits: [`FREED_WORDS`] with the
/// block's address shifted down by 4, above the heap ([`HEAP_END`]), so a pointer taken from the
/// block leads to unmapped memory, and the read through it names the block
/// ([`Mem::freed_block_of`]). (It was 0xD0000000 while the heap ended below 0x80000000.)
pub fn freed_word(addr: u32) -> u32 {
    FREED_WORDS | (addr >> 4)
}

/// The base of the freed-block check's words: the heap's end, so that a heap address shifted
/// down by 4 (at most 0x0EFFFFFF) lands between it and 0xFF000000.
const FREED_WORDS: u32 = HEAP_END;
const _: () = assert!(FREED_WORDS & 0x0fff_ffff == 0 && FREED_WORDS >= HEAP_END);

impl Mem {
    /// Turns the block check on (only blocks allocated from now on are guarded). `store`:
    /// look at every store and name the function that writes into a guard; else the guard is
    /// checked when its block is freed. `stop`: panic at the first finding.
    pub fn enable_block_check(&mut self, store: bool, stop: bool) {
        self.check = Some(Box::new(BlockCheck {
            store,
            stop,
            blocks: HashMap::new(),
            guards: BTreeMap::new(),
            pages: if store { vec![0; PAGES] } else { Vec::new() },
            current_fn: 0,
            reports: Vec::new(),
            quarantine: None,
            quarantine_bytes: 0,
            quarantine_limit: QUARANTINE_BYTES,
            freed: HashMap::new(),
            pending_path: Vec::new(),
            alloc_paths: HashMap::new(),
            freed_paths: HashMap::new(),
        }));
        self.check_store = store;
        self.store_diag |= store;
    }

    /// With the block check on: the engine's calls `calls` (outermost first, as
    /// `Engine::call_stack` holds them) for the next block the game's allocator hands out or its
    /// deallocator frees, kept (the innermost 10) to name where a freed block came from.
    pub fn note_path(&mut self, calls: &[u32]) {
        if let Some(c) = self.check.as_deref_mut() {
            c.pending_path.clear();
            c.pending_path.extend(calls.iter().rev().take(10));
        }
    }

    /// Adds the freed-block check to the block check (turning that on with `store` when it is
    /// off): a freed block is not handed out again at once but waits, guarded whole, behind
    /// [`QUARANTINE_BYTES`] of later frees; a store into it meanwhile is reported with the
    /// functions that allocated, freed and wrote it (a write through a stale pointer, which a
    /// reused block would take silently; also one through a pointer kept to an ended
    /// `Engine::with_stack` frame, which in the exe would land on the dead stack instead).
    pub fn enable_freed_check(&mut self, stop: bool) {
        if !self.check.as_deref().is_some_and(|c| c.store) {
            self.enable_block_check(true, stop);
        }
        if let Some(c) = self.check.as_deref_mut() {
            c.quarantine.get_or_insert_with(Default::default);
        }
    }

    /// With the freed-block check, keeps the freed block `addr` (rounded `size`, `info` the
    /// size asked for and the allocating function) back and answers the oldest blocks that
    /// leave the quarantine for the free lists; `None` without the check.
    fn quarantine_block(
        &mut self,
        addr: u32,
        size: u32,
        info: Option<(u32, u32)>,
    ) -> Option<Vec<(u32, u32)>> {
        self.check.as_deref()?.quarantine.as_ref()?;
        // Filled with `freed_word`, so a pointer read from a freed object points nowhere and the
        // next access through it stops the run in the function that kept using the object.
        let fill: Vec<u8> = std::iter::repeat(freed_word(addr).to_le_bytes())
            .take(size as usize / 4)
            .flatten()
            .collect();
        let _ = self.try_write_raw(addr, &fill);
        let c = self.check.as_deref_mut()?;
        let queue = c.quarantine.as_mut()?;
        let (asked, owner) = info.unwrap_or((size, 0));
        queue.push_back((addr, size));
        c.quarantine_bytes += u64::from(size);
        c.freed.insert(addr, (asked, owner, c.current_fn));
        let alloc_path = c.alloc_paths.remove(&addr).unwrap_or_default();
        let free_path = std::mem::take(&mut c.pending_path);
        c.freed_paths.insert(addr, (alloc_path, free_path));
        c.guards.insert(addr, (addr + size, addr));
        for p in (addr >> PAGE_BITS)..=((addr + size - 1) >> PAGE_BITS) {
            c.pages[p as usize] = c.pages[p as usize].saturating_add(1);
        }
        let mut released = Vec::new();
        while c.quarantine_bytes > c.quarantine_limit {
            let Some((a, s)) = queue.pop_front() else {
                break;
            };
            c.quarantine_bytes -= u64::from(s);
            c.freed.remove(&a);
            c.freed_paths.remove(&a);
            c.guards.remove(&a);
            for p in (a >> PAGE_BITS)..=((a + s - 1) >> PAGE_BITS) {
                c.pages[p as usize] = c.pages[p as usize].saturating_sub(1);
            }
            released.push((a, s));
        }
        Some(released)
    }

    /// Whether the block check is on.
    #[inline(always)]
    pub fn block_check_on(&self) -> bool {
        self.check.is_some()
    }

    /// Sets the function the block check names as running; returns the one before.
    pub fn set_check_fn(&mut self, f: u32) -> u32 {
        match self.check.as_deref_mut() {
            Some(c) => std::mem::replace(&mut c.current_fn, f),
            None => 0,
        }
    }

    /// Takes the block check's findings, oldest first.
    pub fn take_block_reports(&mut self) -> Vec<String> {
        self.check
            .as_deref_mut()
            .map(|c| std::mem::take(&mut c.reports))
            .unwrap_or_default()
    }

    fn check_report(&mut self, text: String) {
        let Some(c) = self.check.as_deref_mut() else {
            return;
        };
        eprintln!("NV_BLOCK_CHECK: {text}");
        if c.stop {
            panic!("NV_BLOCK_CHECK: {text}");
        }
        c.reports.push(text);
    }

    /// After a guarded block of `asked` bytes at `addr` (`total` bytes in all) was handed out.
    fn check_guard_block(&mut self, addr: u32, asked: u32, total: u32) {
        let start = addr + asked;
        let end = addr + total;
        let fill = vec![GUARD_BYTE; (end - start) as usize];
        let _ = self.try_write_raw(start, &fill);
        let Some(c) = self.check.as_deref_mut() else {
            return;
        };
        c.blocks.insert(addr, (asked, c.current_fn));
        if !c.pending_path.is_empty() {
            let path = std::mem::take(&mut c.pending_path);
            c.alloc_paths.insert(addr, path);
        }
        c.guards.insert(start, (end, addr));
        if c.store {
            for p in (start >> PAGE_BITS)..=((end - 1) >> PAGE_BITS) {
                c.pages[p as usize] = c.pages[p as usize].saturating_add(1);
            }
        }
    }

    /// Before a guarded block is freed: its guard checked and dropped.
    fn check_free_block(&mut self, addr: u32) {
        let Some(c) = self.check.as_deref_mut() else {
            return;
        };
        let Some((asked, owner)) = c.blocks.remove(&addr) else {
            return;
        };
        let start = addr + asked;
        let Some((end, _)) = c.guards.remove(&start) else {
            return;
        };
        if c.store {
            for p in (start >> PAGE_BITS)..=((end - 1) >> PAGE_BITS) {
                c.pages[p as usize] = c.pages[p as usize].saturating_sub(1);
            }
        }
        let freer = c.current_fn;
        let mut guard = vec![0u8; (end - start) as usize];
        if self.try_read(start, &mut guard).is_err() {
            return;
        }
        if let Some(i) = guard.iter().position(|&b| b != GUARD_BYTE) {
            let last = guard.iter().rposition(|&b| b != GUARD_BYTE).unwrap_or(i);
            self.check_report(format!(
                "block {addr:08x} ({asked:#x} bytes, allocated while {owner:08x} ran) was written \
                 past its end: +{:#x}..+{:#x} changed (found when it was freed, while {freer:08x} ran)",
                asked + i as u32,
                asked + last as u32 + 1
            ));
        }
    }

    /// A store of `len` bytes at `a` on a page with guards: one that reaches a guard is
    /// reported with the running function.
    #[cold]
    #[inline(never)]
    fn check_store_at(&mut self, a: u32, len: u32) {
        let Some(c) = self.check.as_deref() else {
            return;
        };
        let last = a.wrapping_add(len.max(1) - 1);
        if c.pages[(a >> PAGE_BITS) as usize] == 0 && c.pages[(last >> PAGE_BITS) as usize] == 0 {
            return;
        }
        let end = (a as u64 + len.max(1) as u64).min(u32::MAX as u64) as u32;
        // The first guard the store reaches: one that starts at or below `a` and runs past it,
        // else the lowest one starting inside the store (a store past a live block's end that
        // also reaches a freed neighbour is named for the live block's guard first).
        let hit = c
            .guards
            .range(..=a)
            .next_back()
            .filter(|(_, &(gend, _))| gend > a)
            .or_else(|| c.guards.range(a.saturating_add(1)..end).next())
            .map(|(&gstart, &(_, block))| (gstart, block));
        let Some((gstart, block)) = hit else {
            return;
        };
        let writer = c.current_fn;
        if let Some(&(asked, owner, freer)) = c.freed.get(&block) {
            self.check_report(format!(
                "{writer:08x} wrote {len} byte(s) at {a:08x}, +{:#x} into block {block:08x} \
                 ({asked:#x} bytes, allocated while {owner:08x} ran) after it was freed while \
                 {freer:08x} ran",
                a.max(gstart) - block
            ));
            return;
        }
        let (asked, owner) = c.blocks.get(&block).copied().unwrap_or((0, 0));
        self.check_report(format!(
            "{writer:08x} wrote {len} byte(s) at {a:08x}, +{:#x} into block {block:08x} \
             ({asked:#x} bytes, allocated while {owner:08x} ran): past its end",
            (a.max(gstart)) - block
        ));
    }
}

impl Default for Mem {
    fn default() -> Self {
        Self::new()
    }
}

impl Mem {
    pub fn new() -> Self {
        Mem {
            space: Space::new(),
            kept: 0,
            keep_hand: 0,
            kept_groups: Box::new([0; KEEP_GROUP_WORDS]),
            #[cfg(test)]
            host_calls: 0,
            live: Default::default(),
            heap: heap::Heap::new(),
            watch: None,
            check: None,
            check_store: false,
            store_diag: false,
        }
    }

    /// Diagnostic, not game behaviour: records events in `[start, start + len)` (stores
    /// that change bytes, zeroing, re-allocation, frees; with `frees_only` just frees) for
    /// [`Mem::take_watch_hits`]. A range already watched at `start` is replaced.
    pub fn watch(&mut self, start: u32, len: u32, frees_only: bool) {
        if len == 0 {
            return;
        }
        self.unwatch(start);
        let w = self.watch.get_or_insert_with(|| {
            Box::new(Watch {
                ranges: BTreeMap::new(),
                pages: vec![0; PAGES],
                hits: Vec::new(),
            })
        });
        w.ranges.insert(start, (len, frees_only));
        self.store_diag = true;
        let last = (start as u64 + len as u64 - 1) >> PAGE_BITS;
        for p in (start >> PAGE_BITS) as u64..=last {
            w.pages[p as usize] = w.pages[p as usize].saturating_add(1);
        }
    }

    /// Stops watching the range that starts at `start` ([`Mem::watch`]).
    pub fn unwatch(&mut self, start: u32) {
        let Some(w) = self.watch.as_deref_mut() else {
            return;
        };
        let Some((len, _)) = w.ranges.remove(&start) else {
            return;
        };
        let last = (start as u64 + len as u64 - 1) >> PAGE_BITS;
        for p in (start >> PAGE_BITS) as u64..=last {
            w.pages[p as usize] = w.pages[p as usize].saturating_sub(1);
        }
    }

    /// Whether watched ranges have events not yet taken.
    #[inline]
    pub fn has_watch_hits(&self) -> bool {
        self.watch.as_deref().is_some_and(|w| !w.hits.is_empty())
    }

    /// Takes the watched ranges' events, oldest first.
    pub fn take_watch_hits(&mut self) -> Vec<WatchHit> {
        self.watch
            .as_deref_mut()
            .map(|w| std::mem::take(&mut w.hits))
            .unwrap_or_default()
    }

    /// Whether `[a, a + len)` touches a page with a watched range.
    #[inline(always)]
    fn watched_page(&self, a: u32, len: u32) -> bool {
        match self.watch.as_deref() {
            None => false,
            Some(w) => {
                let last = a.wrapping_add(len.max(1) - 1);
                w.pages[(a >> PAGE_BITS) as usize] != 0
                    || w.pages[(last >> PAGE_BITS) as usize] != 0
            }
        }
    }

    /// Records `kind` over `[a, a + len)` for each watched range it overlaps; `new` is the
    /// bytes a store writes (zeros otherwise). Unchanged bytes are not an event.
    #[cold]
    #[inline(never)]
    fn watch_note(&mut self, a: u32, len: u32, new: Option<&[u8]>, kind: WatchKind) {
        let end = a as u64 + len.max(1) as u64;
        let Some(w) = self.watch.as_deref() else {
            return;
        };
        let mut overlaps = Vec::new();
        for (&start, &(rlen, frees_only)) in w
            .ranges
            .range(..=((end - 1).min(u32::MAX as u64) as u32))
            .rev()
        {
            let rend = start as u64 + rlen as u64;
            if rend <= a as u64 {
                // Ranges do not overlap one another, so none further down reaches `a`.
                break;
            }
            if frees_only && kind != WatchKind::Free {
                continue;
            }
            overlaps.push((start, rend));
        }
        for (start, rend) in overlaps {
            let from = (a as u64).max(start as u64) as u32;
            let to = end.min(rend);
            let n = ((to - from as u64) as usize).min(8);
            let mut old = [0u8; 8];
            if self.try_read(from, &mut old[..n]).is_err() {
                continue;
            }
            let mut fresh = [0u8; 8];
            if let Some(bytes) = new {
                let skip = (from - a) as usize;
                let take = n.min(bytes.len().saturating_sub(skip));
                fresh[..take].copy_from_slice(&bytes[skip..skip + take]);
            }
            let (old, fresh) = (u64::from_le_bytes(old), u64::from_le_bytes(fresh));
            if matches!(kind, WatchKind::Write | WatchKind::Zero) && old == fresh {
                continue;
            }
            let w = self.watch.as_deref_mut().expect("watch");
            if w.hits.len() < WATCH_HITS_KEEP {
                w.hits.push(WatchHit {
                    start,
                    addr: from,
                    len: (to - from as u64) as u32,
                    old,
                    new: fresh,
                    kind,
                });
            }
        }
    }

    /// Makes `[addr, addr + len)` addressable: pages not mapped yet read zero (each run of them
    /// committed with one host call, or, kept by [`Mem::unmap`], zeroed in place); pages already
    /// mapped keep their bytes.
    pub fn map(&mut self, addr: u32, len: u32) {
        if len == 0 {
            return;
        }
        let first = (addr >> PAGE_BITS) as usize;
        let last = ((addr as u64 + len as u64 - 1) >> PAGE_BITS) as usize;
        let end = (last + 1).min(PAGES);
        let mut p = first;
        while p < end {
            let state = self.page_state(p);
            let run = p;
            while p < end && self.page_state(p) == state {
                p += 1;
            }
            let (at, bytes) = (self.host_page(run), (p - run) << PAGE_BITS);
            match state {
                MAPPED => continue,
                KEPT => {
                    // SAFETY: pages `run..p` lie inside the block and are committed (kept);
                    // written under `&mut self`.
                    unsafe { std::ptr::write_bytes(at, 0, bytes) };
                    self.kept -= p - run;
                }
                _ => {
                    // SAFETY: pages `run..p` lie inside the block (`p <= PAGES`).
                    unsafe { host::commit(at, bytes) };
                    self.count_host_call();
                }
            }
            self.set_state(run, p, MAPPED);
        }
        // The page table this replaced stopped here too, with the pages below the end mapped.
        assert!(
            last < PAGES,
            "map of {addr:08x}+{len:x}: runs past the end of the 4 GB space"
        );
    }

    /// Makes the pages of `[addr, addr + len)` unaddressable again, their contents dropped (a
    /// later [`Mem::map`] reads zero). A call that unmaps at most [`KEEP_SPAN`] pages keeps them
    /// committed (marked [`KEPT`]); a larger one hands them back to the system, with any kept
    /// pages in the range, one host call for each run. So does a smaller one at [`KEEP_MAX`]
    /// that the next [`KEEP_BATCH_RUNS`] runs of kept pages cannot make room for
    /// ([`Mem::kept_runs_hold`]). `addr` and `len` must be page multiples (the `VirtualFree`
    /// stand-in in `units/platform.rs` decommits this way).
    pub fn unmap(&mut self, addr: u32, len: u32) {
        assert!(
            addr % PAGE as u32 == 0 && len % PAGE as u32 == 0,
            "unmap of {addr:08x}+{len:x}: not whole pages"
        );
        let first = (addr >> PAGE_BITS) as usize;
        let end = first + (len >> PAGE_BITS) as usize;
        let stop = end.min(PAGES);
        let mapped = (first..stop)
            .filter(|&p| self.page_state(p) == MAPPED)
            .count();
        let over = (self.kept + mapped).saturating_sub(KEEP_MAX);
        if mapped > KEEP_SPAN || (mapped > 0 && over > 0 && !self.kept_runs_hold(over)) {
            // Its own pages go back: more than KEEP_SPAN, or more than the next KEEP_BATCH_RUNS
            // runs of kept pages make room for (handing back a kept page per page it went over
            // by cost a 1024-page unmap at the limit, the kept pages scattered, 1024 host calls).
            self.hand_back(first, stop);
        } else if mapped > 0 {
            if over > 0 {
                self.hand_back_kept(over);
            }
            let mut p = first;
            while p < stop {
                if self.page_state(p) != MAPPED {
                    p += 1;
                    continue;
                }
                let run = p;
                while p < stop && self.page_state(p) == MAPPED {
                    p += 1;
                }
                self.set_state(run, p, KEPT);
            }
            self.kept += mapped;
        }
        assert!(
            end <= PAGES,
            "unmap of {addr:08x}+{len:x}: runs past the end of the 4 GB space"
        );
    }

    /// Hands back at least `need` kept pages (all of them when fewer are kept), and, while it
    /// has made fewer than [`KEEP_BATCH_RUNS`] host calls, up to [`KEEP_BATCH`]: the first ones
    /// the table marks [`KEPT`] from [`Mem::keep_hand`] on, round the space, one host call per
    /// run. The next call goes on from where this one stopped, so every kept page comes round in
    /// turn. Contiguous kept pages go back [`KEEP_BATCH`] in one call, so the unmaps after it
    /// keep their pages without a host call; scattered ones cost an unmap at most
    /// [`KEEP_BATCH_RUNS`] calls ([`Mem::unmap`] calls this only when that many runs hold
    /// `need` pages, [`Mem::kept_runs_hold`]).
    fn hand_back_kept(&mut self, need: usize) {
        let most = need.max(KEEP_BATCH).min(self.kept);
        let need = need.min(most);
        let (mut done, mut runs) = (0, 0);
        let mut p = self.keep_hand;
        while done < most && (done < need || runs < KEEP_BATCH_RUNS) {
            // A run that ended at the last page of the space leaves `p` at `PAGES`: the look
            // goes on from page 0.
            p = self.next_kept(p);
            let run = p;
            while p < PAGES && done < most && self.page_state(p) == KEPT {
                p += 1;
                done += 1;
            }
            self.kept -= p - run;
            self.set_state(run, p, UNMAPPED);
            // SAFETY: pages `run..p` lie inside the block, and no reference into the guest
            // space outlives a call of this module (`&mut self` here).
            unsafe { host::decommit(self.host_page(run), (p - run) << PAGE_BITS) };
            self.count_host_call();
            runs += 1;
        }
        self.keep_hand = p % PAGES;
    }

    /// Whether the first [`KEEP_BATCH_RUNS`] runs of kept pages from [`Mem::keep_hand`] on,
    /// round the space (the runs [`Mem::hand_back_kept`] would hand back, a host call each), hold
    /// at least `need` pages. Each run holds one at least, so a `need` of at most
    /// [`KEEP_BATCH_RUNS`] always fits. `need` is below the kept count (an unmap that keeps its
    /// pages adds at most [`KEEP_SPAN`], fewer than [`KEEP_MAX`]), so the look never comes round
    /// to a page it has counted.
    fn kept_runs_hold(&self, need: usize) -> bool {
        debug_assert!(need < self.kept);
        if need <= KEEP_BATCH_RUNS {
            return true;
        }
        let (mut p, mut held) = (self.keep_hand, 0);
        for _ in 0..KEEP_BATCH_RUNS {
            p = self.next_kept(p);
            while p < PAGES && self.page_state(p) == KEPT {
                p += 1;
                held += 1;
                if held >= need {
                    return true;
                }
            }
        }
        false
    }

    /// The first page at or after `from` (round the space; `from` at most [`PAGES`], which is
    /// page 0 again) that the table marks [`KEPT`]; there must be one. Looks at the rest of
    /// `from`'s group of [`KEEP_GROUP`] pages, then at [`Mem::kept_groups`] for the next group
    /// that holds one, and at that group's pages. A hand-back's run that ends at the last page
    /// of the space leaves its next look at [`PAGES`]: read as a group, that was one past the
    /// end of [`Mem::kept_groups`], and the unmap panicked.
    fn next_kept(&self, from: usize) -> usize {
        debug_assert!(from <= PAGES);
        let from = from % PAGES;
        // SAFETY: the table holds a byte for each of the `PAGES` pages and stays committed while
        // the block lives; only `&mut self` writes it.
        let table = unsafe { std::slice::from_raw_parts(self.space.flag(0), PAGES) };
        let group = from / KEEP_GROUP;
        let end = (group + 1) * KEEP_GROUP;
        if self.group_marked(group) {
            if let Some(p) = (from..end).find(|&p| table[p] == KEPT) {
                return p;
            }
        }
        // The groups after it, round the space and back to its own word: the bits below the
        // next group in that word last (its own group's pages before `from` among them).
        let next = group + 1;
        for i in 0..=KEEP_GROUP_WORDS {
            let w = (next / 64 + i) % KEEP_GROUP_WORDS;
            let mut bits = self.kept_groups[w];
            if i == 0 {
                bits &= !0u64 << (next % 64);
            }
            if bits != 0 {
                let g = w * 64 + bits.trailing_zeros() as usize;
                return (g * KEEP_GROUP..(g + 1) * KEEP_GROUP)
                    .find(|&p| table[p] == KEPT)
                    .unwrap_or_else(|| panic!("group {g} is marked with no kept page"));
            }
        }
        panic!("no kept page anywhere ({} counted)", self.kept)
    }

    /// Whether [`Mem::kept_groups`] marks group `g` (pages `64g..64g + 64`).
    #[inline]
    fn group_marked(&self, g: usize) -> bool {
        (self.kept_groups[g / 64] >> (g % 64)) & 1 != 0
    }

    /// Whether the table marks any of group `g`'s pages [`KEPT`].
    fn group_holds_kept(&self, g: usize) -> bool {
        (g * KEEP_GROUP..(g + 1) * KEEP_GROUP).any(|p| self.page_state(p) == KEPT)
    }

    /// Hands back to the system each run of the pages `from..to` that the table marks [`KEPT`]
    /// or [`MAPPED`], one host call per run, and marks them [`UNMAPPED`].
    fn hand_back(&mut self, from: usize, to: usize) {
        let held = |s: u8| s == KEPT || s == MAPPED;
        let mut p = from;
        while p < to {
            if !held(self.page_state(p)) {
                p += 1;
                continue;
            }
            let run = p;
            while p < to && held(self.page_state(p)) {
                if self.page_state(p) == KEPT {
                    self.kept -= 1;
                }
                p += 1;
            }
            self.set_state(run, p, UNMAPPED);
            // SAFETY: pages `run..p` lie inside the block, and no reference into the guest
            // space outlives a call of this module (`&mut self` here).
            unsafe { host::decommit(self.host_page(run), (p - run) << PAGE_BITS) };
            self.count_host_call();
        }
    }

    /// Counts a host call of [`Mem::map`] or [`Mem::unmap`] for the tests; nothing otherwise.
    #[inline(always)]
    fn count_host_call(&mut self) {
        #[cfg(test)]
        {
            self.host_calls += 1;
        }
    }

    /// How many times [`Mem::map`] and [`Mem::unmap`] have called the host so far.
    #[cfg(test)]
    pub(crate) fn host_calls(&self) -> u32 {
        self.host_calls
    }

    /// How many pages the table marks [`KEPT`] (the tests' view of the count).
    #[cfg(test)]
    pub(crate) fn kept_pages(&self) -> usize {
        self.kept
    }

    /// Checks the table of pages: the count of kept pages is the number the table marks
    /// [`KEPT`] and within [`KEEP_MAX`], [`Mem::kept_groups`] marks exactly the groups that hold
    /// one, and on Windows the system has committed exactly the pages the table marks
    /// [`MAPPED`] or [`KEPT`].
    #[cfg(test)]
    pub(crate) fn check_pages(&self) {
        let kept = (0..PAGES).filter(|&p| self.page_state(p) == KEPT).count();
        assert_eq!(kept, self.kept, "the kept pages counted");
        assert!(self.kept <= KEEP_MAX);
        for g in 0..PAGES / KEEP_GROUP {
            assert_eq!(
                self.group_marked(g),
                self.group_holds_kept(g),
                "group {g} (pages {:08x}..)",
                (g * KEEP_GROUP) << PAGE_BITS
            );
        }
        #[cfg(windows)]
        for (at, len, committed) in host::committed_runs(self.space.base, SPACE) {
            for p in at >> PAGE_BITS..(at + len) >> PAGE_BITS {
                let state = self.page_state(p);
                assert_eq!(
                    committed,
                    state == MAPPED || state == KEPT,
                    "page {:08x} (table {state})",
                    p << PAGE_BITS
                );
            }
        }
    }

    pub fn is_mapped(&self, addr: u32) -> bool {
        self.mapped_at(addr)
    }

    /// Page `p`'s byte in the table of mapped pages ([`MAPPED`], [`KEPT`] or [`UNMAPPED`]; `p`
    /// an address shifted down by [`PAGE_BITS`], so below [`PAGES`]).
    #[inline(always)]
    fn page_state(&self, p: usize) -> u8 {
        debug_assert!(p < PAGES);
        // SAFETY: the table holds a byte for each of the `PAGES` pages and stays committed while
        // the block lives; only `&mut self` writes it.
        unsafe { *self.space.flag(p) }
    }

    /// Whether page `p` (below [`PAGES`]) is mapped.
    #[inline(always)]
    fn page_is_mapped(&self, p: usize) -> bool {
        self.page_state(p) == MAPPED
    }

    /// The host address of page `p`'s first byte (`p` at most [`PAGES`]).
    fn host_page(&self, p: usize) -> *mut u8 {
        self.space.base.wrapping_add(p << PAGE_BITS)
    }

    /// Sets the table's bytes of pages `from..to` (at most [`PAGES`]) to `state`, and the bits of
    /// [`Mem::kept_groups`] of the groups they lie in: set for [`KEPT`], else cleared for each
    /// group none of whose pages the table still marks so (a group only partly in the range is
    /// looked at, one wholly in it is not).
    fn set_state(&mut self, from: usize, to: usize, state: u8) {
        assert!(from <= to && to <= PAGES);
        // SAFETY: the bytes of pages `from..to` lie in the table, written under `&mut self`.
        unsafe { std::ptr::write_bytes(self.space.flag(from), state, to - from) };
        if from == to {
            return;
        }
        for g in from / KEEP_GROUP..=(to - 1) / KEEP_GROUP {
            let bit = 1u64 << (g % 64);
            if state == KEPT {
                self.kept_groups[g / 64] |= bit;
            } else if self.kept_groups[g / 64] & bit != 0 {
                let whole = g * KEEP_GROUP >= from && (g + 1) * KEEP_GROUP <= to;
                if whole || !self.group_holds_kept(g) {
                    self.kept_groups[g / 64] &= !bit;
                }
            }
        }
    }

    /// Whether the page holding `a` is mapped: the test every access makes first.
    #[inline(always)]
    fn mapped_at(&self, a: u32) -> bool {
        self.page_is_mapped((a >> PAGE_BITS) as usize)
    }

    /// The host address of guest address `a`. Worked out from `a` alone (not from the table of
    /// mapped pages), so a load through it can start before the page's byte is known; it may be
    /// used only once that byte says the page is mapped.
    #[inline(always)]
    fn host_at(&self, a: u32) -> *mut u8 {
        self.space.base.wrapping_add(a as usize)
    }

    /// Reads `buf.len()` bytes; faults on the first unmapped byte.
    pub fn try_read(&self, addr: u32, buf: &mut [u8]) -> Result<(), Fault> {
        let mut a = addr;
        let mut done = 0;
        while done < buf.len() {
            let off = (a as usize) & (PAGE - 1);
            let n = (PAGE - off).min(buf.len() - done);
            if !self.mapped_at(a) {
                return Err(Fault(a));
            }
            // SAFETY: `[a, a + n)` lies in one mapped page; `buf` is host memory of its own (no
            // reference into the guest space leaves this module), and nothing writes the guest
            // space while `&self` lives.
            unsafe {
                std::ptr::copy_nonoverlapping(self.host_at(a), buf.as_mut_ptr().add(done), n)
            };
            done += n;
            a = a.wrapping_add(n as u32);
        }
        Ok(())
    }

    pub fn try_write(&mut self, addr: u32, data: &[u8]) -> Result<(), Fault> {
        if self.check_store {
            self.check_store_at(addr, data.len() as u32);
        }
        if self.watched_page(addr, data.len() as u32) {
            self.watch_note(addr, data.len() as u32, Some(data), WatchKind::Write);
        }
        self.try_write_raw(addr, data)
    }

    /// [`Mem::try_write`] without the write watch.
    fn try_write_raw(&mut self, addr: u32, data: &[u8]) -> Result<(), Fault> {
        let mut a = addr;
        let mut done = 0;
        while done < data.len() {
            let off = (a as usize) & (PAGE - 1);
            let n = (PAGE - off).min(data.len() - done);
            if !self.mapped_at(a) {
                return Err(Fault(a));
            }
            // SAFETY: `[a, a + n)` lies in one mapped page, written under `&mut self`; `data` is
            // host memory of its own (no reference into the guest space leaves this module).
            unsafe { std::ptr::copy_nonoverlapping(data.as_ptr().add(done), self.host_at(a), n) };
            done += n;
            a = a.wrapping_add(n as u32);
        }
        Ok(())
    }

    /// Zeroes `[addr, addr + len)` page by page (what writing `len` zero bytes does, without
    /// a buffer of them). Panics as [`Mem::write`] does on the first unmapped byte, with the
    /// bytes before it zeroed.
    pub fn zero(&mut self, addr: u32, len: u32) {
        if self.check_store {
            self.check_store_at(addr, len);
        }
        if self.watched_page(addr, len) {
            self.watch_note(addr, len, None, WatchKind::Zero);
        }
        self.zero_raw(addr, len);
    }

    /// [`Mem::zero`] without the write watch.
    fn zero_raw(&mut self, addr: u32, len: u32) {
        let mut a = addr;
        let mut left = len as usize;
        while left > 0 {
            let off = (a as usize) & (PAGE - 1);
            let n = (PAGE - off).min(left);
            if !self.mapped_at(a) {
                panic!("write to unmapped memory at {a:08x}");
            }
            // SAFETY: `[a, a + n)` lies in one mapped page, written under `&mut self`.
            unsafe { std::ptr::write_bytes(self.host_at(a), 0, n) };
            left -= n;
            a = a.wrapping_add(n as u32);
        }
    }

    /// Reads bytes. Panics with the address on a fault: a translated
    /// function touched memory that was never provided.
    pub fn read(&self, addr: u32, buf: &mut [u8]) {
        if let Err(Fault(a)) = self.try_read(addr, buf) {
            self.read_fault(a)
        }
    }

    /// The panic of a read that met the unmapped address `a` ([`Mem::read`]).
    #[cold]
    #[inline(never)]
    fn read_fault(&self, a: u32) -> ! {
        match self.freed_block_of(a) {
            Some(block) => panic!("read of unmapped memory at {a:08x} ({block})"),
            None => panic!("read of unmapped memory at {a:08x}"),
        }
    }

    /// With the freed-block check: the waiting freed block a fault at `a` came from, when `a` is
    /// a [`freed_word`] plus less than 64 KB: the block and what the check knows of it.
    #[cold]
    fn freed_block_of(&self, a: u32) -> Option<String> {
        let c = self.check.as_deref()?;
        c.quarantine.as_ref()?;
        if a < FREED_WORDS {
            return None;
        }
        let word = a & 0x0fff_ffff;
        (0..0x1_0000u32.min(word + 1)).find_map(|offset| {
            let base = (word - offset) << 4;
            [base, base + 8].into_iter().find_map(|block| {
                c.freed.get(&block).map(|&(asked, owner, freer)| {
                    let path = |p: &[u32]| {
                        p.iter()
                            .map(|f| format!("{f:08x}"))
                            .collect::<Vec<_>>()
                            .join(" < ")
                    };
                    let (alloc, free) = c.freed_paths.get(&block).cloned().unwrap_or_default();
                    format!(
                        "a pointer read from the freed block {block:08x}, plus {offset:#x}: \
                         {asked:#x} bytes, allocated while {owner:08x} ran (in {}), freed while \
                         {freer:08x} ran (in {})",
                        path(&alloc),
                        path(&free)
                    )
                })
            })
        })
    }

    pub fn write(&mut self, addr: u32, data: &[u8]) {
        if let Err(Fault(a)) = self.try_write(addr, data) {
            panic!("write to unmapped memory at {a:08x}");
        }
    }

    pub fn bytes(&self, addr: u32, len: u32) -> Vec<u8> {
        let mut v = vec![0; len as usize];
        self.read(addr, &mut v);
        v
    }

    /// Whether a load or store of up to 8 bytes at `a` takes the fast path: the page holding `a`
    /// and the page after it are both mapped, so the bytes are mapped wherever in the page they
    /// start, even across its end (the guest space is one host block). One 2-byte load of the
    /// table of mapped pages, whose bytes are exactly 1 for a mapped page; this one test costs
    /// as many instructions as the old page table's null test, where testing the offset in the
    /// page as well (to keep the bytes inside one page) measured slower on random accesses.
    #[inline(always)]
    fn fast_at(&self, a: u32) -> bool {
        // SAFETY: the table has a byte for page `PAGES` too (always 0: the space ends there), so
        // both bytes of the pair lie in it; only `&mut self` writes it.
        let pair = unsafe {
            std::ptr::read_unaligned(self.space.flag((a >> PAGE_BITS) as usize).cast::<u16>())
        };
        pair == MAPPED_PAIR
    }

    /// Whether the `n` bytes at `a` lie in one mapped page: the fast path's second chance, for
    /// the last mapped page of a range (whose next page is not mapped).
    #[inline(always)]
    fn in_one_mapped_page(&self, a: u32, n: usize) -> bool {
        (a as usize & (PAGE - 1)) <= PAGE - n && self.mapped_at(a)
    }

    /// The `N` bytes at `a` (`N` at most 8): one fixed-size host load at `base + a` when
    /// [`Mem::fast_at`] allows it (no call to `memcpy`, which a variable-length copy costs; the
    /// translated code reads memory hundreds of millions of times a second), else
    /// [`Mem::load_slow`]. Faults (panics) exactly as [`Mem::read`] does.
    #[inline(always)]
    fn load<const N: usize>(&self, a: u32) -> [u8; N] {
        debug_assert!(N <= 8);
        let p = self.host_at(a);
        if self.fast_at(a) {
            // SAFETY: the `N` bytes start in a mapped page and end in it or in the next one,
            // also mapped; nothing writes the guest space while `&self` lives.
            return unsafe { std::ptr::read_unaligned(p.cast::<[u8; N]>()) };
        }
        self.load_slow(a)
    }

    /// The word at `a` when [`Mem::u32`] reads it without [`Mem::read`] (its page and the next
    /// are mapped, or it lies inside one mapped page), else `None`: for a caller that keeps the
    /// slow path out of its own body (`Engine::vcall`). Reads as [`Mem::load`] does, with no
    /// other effect.
    #[inline(always)]
    pub(crate) fn u32_in_page(&self, a: u32) -> Option<u32> {
        let p = self.host_at(a);
        if self.fast_at(a) || self.in_one_mapped_page(a, 4) {
            // SAFETY: as in `Mem::load` and `Mem::load_slow`: four mapped bytes.
            return Some(u32::from_le_bytes(unsafe {
                std::ptr::read_unaligned(p.cast::<[u8; 4]>())
            }));
        }
        None
    }

    /// [`Mem::load`] when the page after `a`'s is not mapped (or `a`'s is not): still one host
    /// load when the bytes lie inside one mapped page, else [`Mem::read`] across the page
    /// boundary or into the unmapped page.
    #[cold]
    #[inline(never)]
    fn load_slow<const N: usize>(&self, a: u32) -> [u8; N] {
        if self.in_one_mapped_page(a, N) {
            // SAFETY: the `N` bytes lie in one mapped page; nothing writes the guest space while
            // `&self` lives.
            return unsafe { std::ptr::read_unaligned(self.host_at(a).cast::<[u8; N]>()) };
        }
        let mut b = [0; N];
        self.read(a, &mut b);
        b
    }

    /// The store diagnostics of [`Mem::store`], out of its hot path.
    #[cold]
    #[inline(never)]
    fn store_diagnostics(&mut self, a: u32, b: &[u8]) {
        if self.check_store {
            self.check_store_at(a, b.len() as u32);
        }
        if self.watched_page(a, b.len() as u32) {
            self.watch_note(a, b.len() as u32, Some(b), WatchKind::Write);
        }
    }

    /// Writes the `N` bytes `b` at `a`, as [`Mem::load`] reads them.
    #[inline(always)]
    fn store<const N: usize>(&mut self, a: u32, b: [u8; N]) {
        // One test for both diagnostics (the write watch, the block check's store mode), so a
        // normal run pays a single predictable branch per store.
        if self.store_diag {
            self.store_diagnostics(a, &b);
        }
        debug_assert!(N <= 8);
        let p = self.host_at(a);
        if self.fast_at(a) {
            // SAFETY: the `N` bytes start in a mapped page and end in it or in the next one,
            // also mapped; written under `&mut self`.
            unsafe { std::ptr::write_unaligned(p.cast::<[u8; N]>(), b) };
            return;
        }
        self.store_slow(a, b)
    }

    /// [`Mem::store`] off its fast path, as [`Mem::load_slow`] for loads.
    #[cold]
    #[inline(never)]
    fn store_slow<const N: usize>(&mut self, a: u32, b: [u8; N]) {
        if self.in_one_mapped_page(a, N) {
            // SAFETY: the `N` bytes lie in one mapped page, written under `&mut self`.
            unsafe { std::ptr::write_unaligned(self.host_at(a).cast::<[u8; N]>(), b) };
            return;
        }
        if let Err(Fault(at)) = self.try_write_raw(a, &b) {
            panic!("write to unmapped memory at {at:08x}");
        }
    }

    #[inline]
    pub fn u8(&self, a: u32) -> u8 {
        self.load::<1>(a)[0]
    }
    #[inline]
    pub fn u16(&self, a: u32) -> u16 {
        u16::from_le_bytes(self.load(a))
    }
    #[inline]
    pub fn u32(&self, a: u32) -> u32 {
        u32::from_le_bytes(self.load(a))
    }
    #[inline]
    pub fn u64(&self, a: u32) -> u64 {
        u64::from_le_bytes(self.load(a))
    }
    #[inline]
    pub fn i8(&self, a: u32) -> i8 {
        self.u8(a) as i8
    }
    #[inline]
    pub fn i16(&self, a: u32) -> i16 {
        self.u16(a) as i16
    }
    #[inline]
    pub fn i32(&self, a: u32) -> i32 {
        self.u32(a) as i32
    }
    #[inline]
    pub fn f32(&self, a: u32) -> f32 {
        f32::from_bits(self.u32(a))
    }
    #[inline]
    pub fn f64(&self, a: u32) -> f64 {
        f64::from_bits(self.u64(a))
    }

    #[inline]
    pub fn set_u8(&mut self, a: u32, v: u8) {
        self.store(a, [v]);
    }
    #[inline]
    pub fn set_u16(&mut self, a: u32, v: u16) {
        self.store(a, v.to_le_bytes());
    }
    #[inline]
    pub fn set_u32(&mut self, a: u32, v: u32) {
        self.store(a, v.to_le_bytes());
    }
    #[inline]
    pub fn set_u64(&mut self, a: u32, v: u64) {
        self.store(a, v.to_le_bytes());
    }
    #[inline]
    pub fn set_i8(&mut self, a: u32, v: i8) {
        self.set_u8(a, v as u8);
    }
    #[inline]
    pub fn set_i16(&mut self, a: u32, v: i16) {
        self.set_u16(a, v as u16);
    }
    #[inline]
    pub fn set_i32(&mut self, a: u32, v: i32) {
        self.set_u32(a, v as u32);
    }
    #[inline]
    pub fn set_f32(&mut self, a: u32, v: f32) {
        self.set_u32(a, v.to_bits());
    }
    #[inline]
    pub fn set_f64(&mut self, a: u32, v: f64) {
        self.set_u64(a, v.to_bits());
    }

    /// NUL-terminated byte string at `a` (without the NUL). Searched a page at a time; a string
    /// that runs into an unmapped page faults at that page's first byte, as reading it byte by
    /// byte does.
    pub fn cstr(&self, a: u32) -> Vec<u8> {
        let mut out = Vec::new();
        let mut p = a;
        loop {
            if !self.mapped_at(p) {
                self.read_fault(p);
            }
            let n = PAGE - (p as usize & (PAGE - 1));
            // SAFETY: `[p, p + n)` is the rest of one mapped page, and nothing writes the guest
            // space while `&self` lives; the slice ends with this pass of the loop.
            let bytes = unsafe { std::slice::from_raw_parts(self.host_at(p), n) };
            match bytes.iter().position(|&c| c == 0) {
                Some(len) => {
                    out.extend_from_slice(&bytes[..len]);
                    return out;
                }
                None => {
                    out.extend_from_slice(bytes);
                    p = p.wrapping_add(n as u32);
                }
            }
        }
    }

    /// Writes `s` and a NUL at `a`.
    pub fn set_cstr(&mut self, a: u32, s: &[u8]) {
        self.write(a, s);
        self.set_u8(a.wrapping_add(s.len() as u32), 0);
    }

    /// The highest address the heap has reached: no block was ever handed out at or above it.
    /// Tools that map their own memory inside the heap range above it (the oracle check's
    /// buffer arena at 0x30000000) check that the heap has not reached them. The heap's next
    /// fresh address can be lower: it comes down again when the blocks at the top are freed
    /// (`mem/heap.rs`), and this stays at the highest it has been.
    pub fn heap_top(&self) -> u32 {
        self.heap.peak()
    }

    /// The bytes the heap can still hand out, in both its ranges (`GlobalMemoryStatus`'s
    /// available virtual size).
    pub fn heap_available(&self) -> u64 {
        self.heap.available()
    }

    /// Allocates `size` bytes (8-byte aligned, zero-filled) on the heap. A freed block of the
    /// same rounded size is reused first, newest first: from its quick stack (up to 4 KB), or
    /// among the first blocks of its bin; else part of a bigger free block, else fresh memory. A
    /// fitting block deeper in its bin is used only when the top has no room (`mem/heap.rs` has
    /// the policy).
    pub fn alloc(&mut self, size: u32) -> u32 {
        let asked = size;
        let guard = if self.check.is_some() { GUARD } else { 0 };
        let size = size.max(1).div_ceil(8) * 8 + guard;
        let addr = match self.heap.take(size) {
            Some(a) => a,
            None => self.heap_exhausted(size),
        };
        self.map(addr, size);
        if self.watched_page(addr, size) {
            self.watch_note(addr, size, None, WatchKind::Alloc);
        }
        self.zero_raw(addr, size);
        self.live.insert(addr, size);
        if guard != 0 {
            self.check_guard_block(addr, asked, size);
        }
        addr
    }

    /// Frees a block from [`Mem::alloc`]. Freeing 0 does nothing (as the
    /// game's `Deallocate`); freeing anything else that is not a live block
    /// panics.
    pub fn free(&mut self, addr: u32) {
        if addr == 0 {
            return;
        }
        let size = self
            .live
            .remove(&addr)
            .unwrap_or_else(|| panic!("free of {addr:08x}, which is not a live heap block"));
        // The watch sees the bytes the block held, before the freed check fills it.
        if self.watched_page(addr, size) {
            self.watch_note(addr, size, None, WatchKind::Free);
        }
        let mut released = None;
        if let Some(c) = self.check.as_deref() {
            let info = c.blocks.get(&addr).copied();
            self.check_free_block(addr);
            released = self.quarantine_block(addr, size, info);
            if released.is_none() {
                if let Some(c) = self.check.as_deref_mut() {
                    c.alloc_paths.remove(&addr);
                    c.pending_path.clear();
                }
            }
        }
        match released {
            // Blocks leaving the freed-block check's quarantine go straight to the bins, where
            // they merge with their free neighbours.
            Some(blocks) => {
                for (a, s) in blocks {
                    self.heap.release(a, s);
                }
            }
            None => self.heap.give(addr, size),
        }
    }

    /// Stops with the heap's state: live blocks and bytes, free bytes and blocks with the
    /// largest free block, what waited in the quick stacks when the request came (the request
    /// moved it all to the bins before giving up, so it is counted in the free bytes too), the
    /// bytes in the block check's quarantine, and the sizes holding the most live and the most
    /// free bytes.
    #[cold]
    #[inline(never)]
    fn heap_exhausted(&self, size: u32) -> ! {
        fn most(by_size: HashMap<u32, (u64, u64)>) -> String {
            let mut top: Vec<_> = by_size.into_iter().collect();
            top.sort_by(|a, b| b.1 .1.cmp(&a.1 .1).then(a.0.cmp(&b.0)));
            top.iter()
                .take(8)
                .map(|(s, (n, b))| format!("{n}x{s}={}MB", b >> 20))
                .collect::<Vec<_>>()
                .join(" ")
        }
        let tally = |sizes: &mut dyn Iterator<Item = u32>| {
            let mut by_size: HashMap<u32, (u64, u64)> = HashMap::new();
            for s in sizes {
                let e = by_size.entry(s).or_default();
                e.0 += 1;
                e.1 += u64::from(s);
            }
            by_size
        };
        let live: u64 = self.live.values().map(|&s| u64::from(s)).sum();
        let quarantine = self.check.as_deref().map_or(0, |c| c.quarantine_bytes);
        let h = &self.heap;
        let waited = h.failed_flush();
        panic!(
            "heap exhausted allocating {size} bytes (live {} blocks {}MB, free {}MB in {} blocks, \
             the largest {} bytes, quick stacks {}KB in {} blocks (moved to the bins first), \
             quarantine {}MB, top {:08x}; most live {}; most free {})",
            self.live.len(),
            live >> 20,
            h.free_bytes() >> 20,
            h.free_blocks(),
            h.largest_free(),
            waited.0 >> 10,
            waited.1,
            quarantine >> 20,
            h.top(),
            most(tally(&mut self.live.values().copied())),
            most(tally(&mut h.free_sizes()))
        )
    }

    /// Size of a live heap block.
    pub fn block_size(&self, addr: u32) -> Option<u32> {
        if let Some(&(asked, _)) = self.check.as_deref().and_then(|c| c.blocks.get(&addr)) {
            return Some(asked.max(1).div_ceil(8) * 8);
        }
        self.live.get(&addr).copied()
    }

    /// Checks the heap's bookkeeping against the live blocks and those the freed-block check
    /// holds back (see `heap::Heap::check`).
    #[cfg(test)]
    pub(crate) fn check_heap(&self) {
        let held = self
            .check
            .as_deref()
            .and_then(|c| c.quarantine.as_ref())
            .into_iter()
            .flatten()
            .copied();
        self.heap
            .check(self.live.iter().map(|(&a, &s)| (a, s)).chain(held));
    }

    /// Takes the rest of the heap's lower range as one live block without mapping its pages:
    /// the next allocation that does not fit in a free block goes to the upper range (the
    /// tests of a heap that has used its lower range). Answers the block, if any was left.
    #[cfg(test)]
    pub(crate) fn fill_lower_range(&mut self) -> Option<(u32, u32)> {
        let taken = self.heap.take_lower_rest();
        if let Some((a, s)) = taken {
            self.live.insert(a, s);
        }
        taken
    }

    /// Takes the rest of both heap ranges as live blocks (one per range) without mapping their
    /// pages: the tests of a full heap.
    #[cfg(test)]
    pub(crate) fn fill_heap(&mut self) {
        for (a, s) in self.heap.take_rest() {
            self.live.insert(a, s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A word written just past a guarded block is found when the block is freed, with the
    /// functions that allocated and freed it; writes inside it are not.
    #[test]
    fn the_block_check_finds_a_write_past_the_end_at_free() {
        let mut m = Mem::new();
        m.enable_block_check(false, false);
        m.set_check_fn(0x0055_3bb0);
        let a = m.alloc(0x20);
        assert_eq!(m.block_size(a), Some(0x20));
        m.set_u32(a + 0x1c, 7);
        m.free(a);
        assert!(m.take_block_reports().is_empty());
        let b = m.alloc(0x20);
        m.set_u32(b + 0x20, 0);
        m.set_check_fn(0x0040_1000);
        m.free(b);
        let r = m.take_block_reports();
        assert_eq!(r.len(), 1, "{r:?}");
        assert!(r[0].contains("+0x20..+0x24 changed"), "{}", r[0]);
        assert!(r[0].contains("allocated while 00553bb0 ran"), "{}", r[0]);
        assert!(r[0].contains("while 00401000 ran"), "{}", r[0]);
    }

    /// With `store`, the store into the guard is reported at once, naming the writer; the
    /// slack between the size asked and the 8-byte rounding is guarded too.
    #[test]
    fn the_block_check_names_the_writer_with_store() {
        let mut m = Mem::new();
        m.enable_block_check(true, false);
        let a = m.alloc(0x1c);
        m.set_check_fn(0x004e_75d0);
        m.set_u32(a + 0x18, 1);
        assert!(m.take_block_reports().is_empty());
        m.set_u32(a + 0x1c, 2);
        let r = m.take_block_reports();
        assert_eq!(r.len(), 1, "{r:?}");
        assert!(r[0].starts_with("004e75d0 wrote 4 byte(s)"), "{}", r[0]);
        assert!(r[0].contains("+0x1c into block"), "{}", r[0]);
        // A write well outside any block is not a finding.
        let c = m.alloc(0x100);
        m.write(c, &[1; 0x100]);
        assert!(m.take_block_reports().is_empty());
    }

    /// With the check on, `stop` panics at the finding.
    #[test]
    #[should_panic(expected = "NV_BLOCK_CHECK")]
    fn the_block_check_stops_when_asked() {
        let mut m = Mem::new();
        m.enable_block_check(true, true);
        let a = m.alloc(8);
        m.set_u8(a + 8, 1);
    }

    /// With the freed check, a store that runs past a live block's end and on into a freed
    /// neighbour is named for the live block's own guard, which it reaches first.
    #[test]
    fn an_overrun_into_a_freed_neighbour_names_the_overrun_block() {
        let mut m = Mem::new();
        m.enable_freed_check(false);
        let live = m.alloc(0x20);
        let neighbour = m.alloc(0x20);
        assert_eq!(neighbour, live + 0x20 + GUARD);
        m.free(neighbour);
        m.write(live, &[1; 0x34]);
        let reports = m.take_block_reports();
        assert_eq!(reports.len(), 1, "{reports:?}");
        assert!(
            reports[0].contains(&format!("+0x20 into block {live:08x}"))
                && reports[0].contains("past its end"),
            "{}",
            reports[0]
        );
    }

    /// The write watch records changing stores, frees and re-allocation of a watched range
    /// and nothing outside it; a frees-only range sees only its free.
    #[test]
    fn watch_records_changes_frees_and_reuse() {
        let mut m = Mem::new();
        let a = m.alloc(0x18);
        let b = m.alloc(0x10);
        assert!(!m.has_watch_hits());
        m.watch(a, 0x14, false);
        m.watch(b, 0x10, true);
        m.set_u32(a + 4, 0x1234);
        m.set_u32(a + 4, 0x1234); // unchanged: no event
        m.set_u32(a + 0x14, 9); // past the range
        m.set_u32(b, 7); // frees only
        let hits = m.take_watch_hits();
        assert_eq!(hits.len(), 1);
        assert_eq!(
            (
                hits[0].start,
                hits[0].addr,
                hits[0].len,
                hits[0].old,
                hits[0].new,
                hits[0].kind
            ),
            (a, a + 4, 4, 0, 0x1234, WatchKind::Write)
        );
        m.write(a, &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(m.take_watch_hits()[0].new, 0x0807_0605_0403_0201);
        m.free(b);
        m.free(a);
        let kinds: Vec<(u32, WatchKind)> = m
            .take_watch_hits()
            .iter()
            .map(|h| (h.start, h.kind))
            .collect();
        assert_eq!(kinds, vec![(b, WatchKind::Free), (a, WatchKind::Free)]);
        assert_eq!(m.alloc(0x18), a);
        let hits = m.take_watch_hits();
        assert_eq!(
            (hits[0].kind, hits[0].old),
            (WatchKind::Alloc, 0x0807_0605_0403_0201)
        );
        m.unwatch(a);
        m.unwatch(b);
        m.set_u32(a, 5);
        assert!(!m.has_watch_hits());
    }

    #[test]
    fn unmapped_reads_fault() {
        let m = Mem::new();
        let mut b = [0; 4];
        assert_eq!(m.try_read(0x0118_3000, &mut b), Err(Fault(0x0118_3000)));
    }

    #[test]
    fn values_cross_pages() {
        let mut m = Mem::new();
        m.map(0x1000, 0x2000);
        m.set_u32(0x1ffe, 0xdead_beef);
        assert_eq!(m.u32(0x1ffe), 0xdead_beef);
        m.set_f64(0x1ffc, 1.25);
        assert_eq!(m.f64(0x1ffc), 1.25);
    }

    /// The fixed-size accessors read and write every width at every offset of a page, and
    /// across the page's end into the next mapped page, as bytes would.
    #[test]
    fn scalar_accessors_match_byte_reads_at_every_offset() {
        let mut m = Mem::new();
        m.map(0x1000, 0x2000);
        for (i, a) in (0x1ff0..0x2010u32).enumerate() {
            m.set_u8(a, i as u8 ^ 0x5a);
        }
        for a in 0x1ff0..0x2008u32 {
            let b = m.bytes(a, 8);
            assert_eq!(m.u8(a), b[0]);
            assert_eq!(m.u16(a), u16::from_le_bytes(b[..2].try_into().unwrap()));
            assert_eq!(m.u32(a), u32::from_le_bytes(b[..4].try_into().unwrap()));
            assert_eq!(m.u64(a), u64::from_le_bytes(b.try_into().unwrap()));
        }
        for a in 0x1ff8..0x2001u32 {
            m.set_u64(a, 0x0102_0304_0506_0708);
            assert_eq!(m.bytes(a, 8), [8, 7, 6, 5, 4, 3, 2, 1]);
            m.set_u16(a, 0xa1b2);
            assert_eq!(m.bytes(a, 2), [0xb2, 0xa1]);
        }
    }

    /// A read of an unmapped page faults with the address, from the fast path and from a
    /// read that crosses into the unmapped page (with the first unmapped byte's address).
    #[test]
    #[should_panic(expected = "read of unmapped memory at 00005000")]
    fn unmapped_scalar_reads_name_the_address() {
        Mem::new().u32(0x5000);
    }

    #[test]
    #[should_panic(expected = "read of unmapped memory at 00002000")]
    fn reads_into_an_unmapped_page_name_its_first_byte() {
        let mut m = Mem::new();
        m.map(0x1000, 0x1000);
        m.u32(0x1ffe);
    }

    /// A write that crosses into an unmapped page writes the mapped part, then faults there.
    #[test]
    fn writes_into_an_unmapped_page_fault_after_the_mapped_part() {
        let mut m = Mem::new();
        m.map(0x1000, 0x1000);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            m.set_u32(0x1ffe, 0xaabb_ccdd)
        }));
        assert!(r.is_err());
        assert_eq!(m.bytes(0x1ffe, 2), [0xdd, 0xcc]);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut m = Mem::new();
            m.set_u8(0x7000, 1)
        }));
        let why = r.unwrap_err();
        assert_eq!(
            why.downcast_ref::<String>().map(String::as_str),
            Some("write to unmapped memory at 00007000")
        );
    }

    /// `zero` clears a range across pages.
    #[test]
    fn zero_clears_across_pages() {
        let mut m = Mem::new();
        m.map(0x1000, 0x2000);
        m.write(0x1ff0, &[0xff; 0x20]);
        m.zero(0x1ff4, 0x18);
        let b = m.bytes(0x1ff0, 0x20);
        assert_eq!(&b[..4], &[0xff; 4]);
        assert!(b[4..0x1c].iter().all(|&x| x == 0));
        assert_eq!(&b[0x1c..], &[0xff; 4]);
    }

    #[test]
    fn heap_reuses_freed_blocks() {
        let mut m = Mem::new();
        let a = m.alloc(12);
        let b = m.alloc(12);
        assert_eq!(b, a + 16);
        m.set_u32(a, 7);
        m.free(a);
        let c = m.alloc(16);
        assert_eq!(c, a);
        assert_eq!(m.u32(c), 0, "reused blocks are zeroed");
        m.free(0);
    }

    #[test]
    fn strings() {
        let mut m = Mem::new();
        let a = m.alloc(16);
        m.set_cstr(a, b"Goodsprings");
        assert_eq!(m.cstr(a), b"Goodsprings");
    }

    /// Freeing 0 does nothing: no block, no change to the heap.
    #[test]
    fn freeing_null_does_nothing() {
        let mut m = Mem::new();
        let a = m.alloc(0x40);
        m.free(0);
        assert_eq!(m.live.len(), 1);
        assert_eq!(m.heap.top(), a + 0x40);
        m.check_heap();
    }

    /// Several freed blocks of one size come back newest first, as before the bins, for sizes
    /// the quick stacks hold and bigger ones (the bins) alike.
    #[test]
    fn same_size_blocks_come_back_newest_first() {
        for size in [0x18, 0x1f8, 0x1000, 0x2000] {
            let mut m = Mem::new();
            let blocks: Vec<u32> = (0..5).map(|_| m.alloc(size)).collect();
            m.free(blocks[1]);
            m.free(blocks[3]);
            assert_eq!(m.alloc(size), blocks[3]);
            assert_eq!(m.alloc(size), blocks[1]);
            assert_eq!(m.alloc(size), blocks[4] + size);
            m.check_heap();
        }
    }

    /// A big freed block serves smaller requests from its start, zero-filled, and the rest of it
    /// serves the next.
    #[test]
    fn a_big_freed_block_serves_smaller_requests() {
        let mut m = Mem::new();
        let big = m.alloc(4 << 20);
        let after = m.alloc(0x40);
        m.write(big, &[0xab; 0x100]);
        m.free(big);
        let a = m.alloc(1 << 20);
        assert_eq!(a, big);
        assert_eq!(m.bytes(a, 0x100), vec![0; 0x100]);
        assert_eq!(m.alloc(0x300), big + (1 << 20));
        assert_eq!(m.alloc((3 << 20) - 0x300), big + (1 << 20) + 0x300);
        assert_eq!(m.heap_top(), after + 0x40);
        m.check_heap();
    }

    /// Freed blocks at the top lower the heap's next fresh address (merging on the way down);
    /// `heap_top` keeps the highest it reached.
    #[test]
    fn freed_blocks_at_the_top_lower_it() {
        let mut m = Mem::new();
        // Bigger than the quick stacks take, so a freed block goes straight to the bins.
        let a = m.alloc(0x2000);
        let b = m.alloc(0x2000);
        let c = m.alloc(0x2000);
        let peak = m.heap_top();
        m.free(a);
        m.free(c);
        assert_eq!(m.heap.top(), c);
        m.free(b);
        assert_eq!(m.heap.top(), HEAP_BASE);
        assert_eq!(m.heap_top(), peak);
        m.check_heap();
        assert_eq!(m.alloc(0x1000), a);
    }

    /// Blocks leaving the freed-block check's quarantine merge with free neighbours in the bins,
    /// even small ones.
    #[test]
    fn blocks_leaving_the_quarantine_merge() {
        let mut m = Mem::new();
        m.enable_freed_check(false);
        m.check.as_deref_mut().unwrap().quarantine_limit = 0x90;
        let blocks: Vec<u32> = (0..3).map(|_| m.alloc(0x40)).collect();
        let keep = m.alloc(8);
        assert_eq!(blocks[1], blocks[0] + 0x40 + GUARD);
        m.free(blocks[0]);
        assert_eq!(m.heap.free_blocks(), 0, "held back");
        m.free(blocks[1]);
        assert_eq!(m.heap.free_block_at(blocks[0]), Some((blocks[0], 0x50)));
        m.free(blocks[2]);
        assert_eq!(m.heap.free_block_at(blocks[0]), Some((blocks[0], 0xa0)));
        assert_eq!(m.heap.quick_blocks(), 0);
        m.check_heap();
        assert_eq!(m.alloc(0x90), blocks[0]);
        m.free(keep);
        m.check_heap();
        assert!(m.take_block_reports().is_empty());
    }

    /// A full heap stops with its state: live and free bytes, the largest free block, the sizes
    /// holding the most.
    #[test]
    fn heap_exhausted_reports_the_heap() {
        let mut m = Mem::new();
        let _a = m.alloc(0x100);
        let b = m.alloc(1 << 20);
        let _c = m.alloc(0x100);
        m.free(b);
        // The rest of the heap as one block per range, taken without mapping its pages.
        m.fill_heap();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.alloc(2 << 20)));
        let why = r.unwrap_err();
        let text = why.downcast_ref::<String>().cloned().unwrap_or_default();
        assert!(
            text.starts_with("heap exhausted allocating 2097152 bytes (live 4 blocks "),
            "{text}"
        );
        assert!(
            text.contains("free 1MB in 1 blocks, the largest 1048576 bytes"),
            "{text}"
        );
        let high = HEAP_END - HEAP_HIGH_BASE;
        assert!(text.contains(&format!("most live 1x{high}=")), "{text}");
        assert!(text.contains("2x256=0MB"), "{text}");
        assert!(
            text.contains("quick stacks 0KB in 0 blocks (moved to the bins first)"),
            "{text}"
        );
        assert!(text.contains("most free 1x1048576=1MB)"), "{text}");
    }

    /// The stop names what waited in the quick stacks when the request came: the request moved
    /// it all to the bins before giving up, so the stacks are empty by the time of the report.
    /// Here 64 freed 0x40-byte blocks, each with a live block after it, cannot make 0x100 bytes.
    #[test]
    fn heap_exhausted_names_what_waited_in_the_quick_stacks() {
        let mut m = Mem::new();
        let blocks: Vec<u32> = (0..64)
            .map(|_| {
                let a = m.alloc(0x40);
                m.alloc(8);
                a
            })
            .collect();
        m.fill_heap();
        for &a in &blocks {
            m.free(a);
        }
        assert_eq!((m.heap.quick_blocks(), m.heap.quick_bytes()), (64, 0x1000));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.alloc(0x100)));
        let text = panic_text(r);
        assert!(
            text.contains(
                "free 0MB in 64 blocks, the largest 64 bytes, quick stacks 4KB in 64 blocks"
            ),
            "{text}"
        );
        assert_eq!(m.heap.quick_blocks(), 0);
    }

    /// A nearly full, fragmented heap serves a request from a free block that fits even when
    /// eight newer, smaller blocks of the same bin sit in front of it (0x2000, 0x2080 and 0x20f8
    /// share a bin).
    #[test]
    fn a_full_heap_serves_a_request_from_deep_in_its_bin() {
        let mut m = Mem::new();
        let fitting = m.alloc(0x20f8);
        m.alloc(8);
        let smaller: Vec<u32> = (0..8)
            .map(|_| {
                let a = m.alloc(0x2000);
                m.alloc(8);
                a
            })
            .collect();
        m.fill_heap();
        m.free(fitting);
        for a in smaller {
            m.free(a);
        }
        m.check_heap();
        assert_eq!(m.alloc(0x2080), fitting);
        assert_eq!(
            m.heap.free_block_at(fitting + 0x2080),
            Some((fitting + 0x2080, 0x78))
        );
        m.check_heap();
    }

    /// Once the lower range is used up, the heap goes on in the upper range, from 0x80000000: a
    /// block there reads and writes like any other (across its pages too) and frees; the lower
    /// range's free blocks still serve first; when the upper range empties, the top goes back
    /// to the lower range.
    #[test]
    fn the_heap_goes_on_above_2_gb_once_the_lower_range_is_used() {
        let mut m = Mem::new();
        let low = m.alloc(0x100);
        assert!(low < HEAP_LOW_END);
        let (filler, _) = m.fill_lower_range().unwrap();
        let a = m.alloc(0x3000);
        assert_eq!(a, HEAP_HIGH_BASE);
        assert!(m.bytes(a, 0x3000).iter().all(|&b| b == 0));
        m.set_u32(a, 0x1122_3344);
        m.set_u64(a + 0xffc, 0x5566_7788_99aa_bbcc);
        m.set_u32(a + 0x2ffc, 0xdead_beef);
        assert_eq!(m.u32(a), 0x1122_3344);
        assert_eq!(m.u64(a + 0xffc), 0x5566_7788_99aa_bbcc);
        assert_eq!(m.u32(a + 0x1000), 0x5566_7788);
        assert_eq!(m.u32(a + 0x2ffc), 0xdead_beef);
        assert_eq!(m.block_size(a), Some(0x3000));
        let b = m.alloc(0x2000);
        assert_eq!(b, a + 0x3000);
        assert_eq!(m.heap_top(), b + 0x2000);
        m.check_heap();
        // A freed block of the lower range serves the next request it fits.
        m.free(low);
        assert_eq!(m.alloc(0x100), low);
        // A freed block up there is reused, zero-filled, and counts as available again.
        let room = m.heap_available();
        m.free(a);
        assert_eq!(m.heap_available(), room + 0x3000);
        assert_eq!(m.alloc(0x3000), a);
        assert_eq!(m.u32(a), 0, "zero-filled");
        assert_eq!(m.heap_available(), room);
        // Freeing the upper blocks merges them and brings the top back to the lower range.
        m.free(a);
        assert_eq!(m.heap.free_block_at(a), Some((a, 0x3000)));
        m.free(b);
        assert_eq!(m.heap.top(), HEAP_LOW_END);
        assert_eq!(m.heap.free_blocks(), 0);
        assert_eq!(m.heap_top(), b + 0x2000, "the peak stays");
        m.check_heap();
        m.free(filler);
        assert_eq!(m.heap.top(), filler);
        m.check_heap();
        assert_eq!(m.alloc(0x3000), filler);
    }

    /// The rest of the lower range above the top, when it is too small for a request, becomes
    /// a free block that later requests take; a block never spans the platform's addresses.
    #[test]
    fn the_lower_range_rest_stays_usable_after_the_move_up() {
        let mut m = Mem::new();
        let (filler, size) = m.fill_lower_range().unwrap();
        m.free(filler);
        assert_eq!(m.heap.top(), HEAP_BASE);
        // Leave 0x2000 bytes at the end of the lower range.
        let big = m.heap.take(size - 0x2000).unwrap();
        m.live.insert(big, size - 0x2000);
        let a = m.alloc(0x3000);
        assert_eq!(a, HEAP_HIGH_BASE);
        assert_eq!(
            m.heap.free_block_at(HEAP_LOW_END - 0x2000),
            Some((HEAP_LOW_END - 0x2000, 0x2000))
        );
        let b = m.alloc(0x1800);
        assert_eq!(b, HEAP_LOW_END - 0x2000);
        m.set_u32(b + 0x17fc, 0x0bad_f00d);
        assert_eq!(m.u32(b + 0x17fc), 0x0bad_f00d);
        assert!(
            !m.is_mapped(HEAP_LOW_END),
            "nothing of the heap is in the platform's area"
        );
        m.check_heap();
        // Too big for the upper range: the heap is exhausted, not wrapped.
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            m.alloc(HEAP_END - HEAP_HIGH_BASE)
        }));
        assert!(panic_text(r).starts_with("heap exhausted"));
    }

    /// The freed-block check's words lie above the heap's upper range, so a pointer read from a
    /// freed block there still faults and names the block.
    #[test]
    fn freed_words_lie_above_the_heap() {
        for a in [HEAP_BASE, HEAP_LOW_END - 8, HEAP_HIGH_BASE, HEAP_END - 8] {
            let w = freed_word(a);
            assert!((HEAP_END..0xff00_0000).contains(&w), "{a:08x}: {w:08x}");
        }
        assert!(PLATFORM_AREA.contains(&HEAP_LOW_END) && !PLATFORM_AREA.contains(&HEAP_HIGH_BASE));
        let mut m = Mem::new();
        m.enable_freed_check(false);
        m.fill_lower_range();
        let a = m.alloc(0x40);
        assert!(a >= HEAP_HIGH_BASE);
        m.free(a);
        assert_eq!(m.u32(a + 4), freed_word(a));
        let named = m.freed_block_of(freed_word(a) + 4).unwrap();
        assert!(named.contains(&format!("freed block {a:08x}")), "{named}");
        assert!(!m.is_mapped(freed_word(a)));
    }

    /// A small pseudo-random generator (xorshift64) for the tests below.
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u32 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 >> 32) as u32
        }
        fn below(&mut self, n: u32) -> u32 {
            self.next() % n
        }
    }

    /// Random allocations and frees of mixed sizes, checked against a shadow list of the live
    /// blocks: every block is 8-byte aligned, inside `[HEAP_BASE, top)`, disjoint from every
    /// other live block and zero-filled when handed out, and keeps what was written into it;
    /// the heap's own bookkeeping is checked as it goes. With `freed_check`, the freed-block
    /// check holds blocks back and lets them go into the bins.
    fn stress(freed_check: bool, seed: u64) {
        let mut m = Mem::new();
        if freed_check {
            m.enable_freed_check(false);
            m.check.as_deref_mut().unwrap().quarantine_limit = 0x4_0000;
        }
        let big = if freed_check { 0x1_0000 } else { 0x10_0000 };
        let mut rng = Rng(seed);
        let mut shadow: BTreeMap<u32, (u32, u32)> = BTreeMap::new();
        let mut order: Vec<u32> = Vec::new();
        for step in 0..20_000u32 {
            let grow = order.len() < 50 || (order.len() < 3000 && rng.below(100) < 55);
            if grow {
                let asked = match rng.below(100) {
                    0..=4 => 0,
                    5..=74 => rng.below(300),
                    75..=97 => 300 + rng.below(6000),
                    _ => 0x1_0000 + rng.below(big),
                };
                let a = m.alloc(asked);
                let size = *m.live.get(&a).expect("a live block");
                assert_eq!(a % 8, 0);
                assert!(
                    a >= HEAP_BASE && a + size <= m.heap.top(),
                    "{a:08x}+{size:x}"
                );
                if let Some((&p, &(ps, _))) = shadow.range(..a).next_back() {
                    assert!(p + ps <= a, "{a:08x} overlaps {p:08x}+{ps:x}");
                }
                if let Some((&n, _)) = shadow.range(a..).next() {
                    assert!(a + size <= n, "{a:08x}+{size:x} overlaps {n:08x}");
                }
                assert!(m.bytes(a, asked.min(64)).iter().all(|&x| x == 0));
                let words = asked / 4;
                if words > 0 {
                    m.set_u32(a, step);
                    m.set_u32(a + (words - 1) * 4, step);
                }
                shadow.insert(a, (size, words));
                order.push(a);
            } else {
                let a = order.swap_remove(rng.below(order.len() as u32) as usize);
                let (_, words) = shadow.remove(&a).unwrap();
                if words > 0 {
                    let tag = m.u32(a);
                    assert_eq!(
                        m.u32(a + (words - 1) * 4),
                        tag,
                        "block {a:08x} was overwritten"
                    );
                }
                m.free(a);
            }
            if step % 1000 == 0 {
                m.check_heap();
            }
        }
        m.check_heap();
        for a in order {
            m.free(a);
        }
        m.check_heap();
        assert!(m.take_block_reports().is_empty());
    }

    #[test]
    fn random_allocs_and_frees_never_overlap() {
        stress(false, 0x2545_f491_4f6c_dd1d);
    }

    #[test]
    fn random_allocs_and_frees_never_overlap_with_the_freed_check() {
        stress(true, 0x9e37_79b9_7f4a_7c15);
    }

    /// The allocator before the bins, kept for the comparisons below: fresh blocks from a bump
    /// pointer, and a freed block reused only by a later request of exactly its size, newest
    /// first (indexed by size / 8 up to 4096 bytes, a map above).
    struct OldHeap {
        brk: u32,
        small_free: Vec<Vec<u32>>,
        free: crate::engine::AddrMap<u32, Vec<u32>>,
    }

    impl OldHeap {
        fn new() -> Self {
            OldHeap {
                brk: HEAP_BASE,
                small_free: vec![Vec::new(); 4096 / 8 + 1],
                free: Default::default(),
            }
        }
        #[inline]
        fn list(&mut self, size: u32) -> &mut Vec<u32> {
            if size <= 4096 {
                &mut self.small_free[(size / 8) as usize]
            } else {
                self.free.entry(size).or_default()
            }
        }
        #[inline]
        fn take(&mut self, size: u32) -> u32 {
            if let Some(a) = self.list(size).pop() {
                return a;
            }
            let a = self.brk;
            assert!(
                (a as u64 + size as u64) <= HEAP_END as u64,
                "old heap exhausted"
            );
            self.brk += size;
            a
        }
    }

    /// One step of a replayed pattern: allocate object `id` with `size` bytes, or free it.
    enum Op {
        Alloc(usize, u32),
        Free(usize),
    }

    /// Cells loaded and unloaded as the open world does: each round allocates a cell of 300
    /// objects of varied sizes (70% up to 256 bytes, 25% 1-16 KB, 5% 16-256 KB, any multiple of
    /// 8), unloads the oldest when more than four are loaded, and replaces 200 random objects of
    /// the loaded cells. Answers the steps, each object's size, the most bytes live at once and
    /// the step where the second half of the rounds starts.
    fn cell_churn(rounds: usize) -> (Vec<Op>, Vec<u32>, u64, usize) {
        fn size(rng: &mut Rng) -> u32 {
            match rng.below(100) {
                0..=69 => 8 + rng.below(32) * 8,
                70..=94 => 1024 + rng.below(1920) * 8,
                _ => 16384 + rng.below(30720) * 8,
            }
        }
        let mut rng = Rng(0x0123_4567_89ab_cdef);
        let mut ops = Vec::new();
        let mut sizes: Vec<u32> = Vec::new();
        let mut cells: std::collections::VecDeque<Vec<usize>> = Default::default();
        let (mut live, mut most) = (0u64, 0u64);
        let mut half = 0;
        for round in 0..rounds {
            if round == rounds / 2 {
                half = ops.len();
            }
            let mut cell = Vec::new();
            for _ in 0..300 {
                let s = size(&mut rng);
                ops.push(Op::Alloc(sizes.len(), s));
                cell.push(sizes.len());
                sizes.push(s);
                live += u64::from(s);
                most = most.max(live);
            }
            cells.push_back(cell);
            if cells.len() > 4 {
                for id in cells.pop_front().unwrap() {
                    ops.push(Op::Free(id));
                    live -= u64::from(sizes[id]);
                }
            }
            for _ in 0..200 {
                let c = rng.below(cells.len() as u32) as usize;
                let k = rng.below(cells[c].len() as u32) as usize;
                let old = cells[c][k];
                ops.push(Op::Free(old));
                live -= u64::from(sizes[old]);
                let s = size(&mut rng);
                ops.push(Op::Alloc(sizes.len(), s));
                cells[c][k] = sizes.len();
                sizes.push(s);
                live += u64::from(s);
                most = most.max(live);
            }
        }
        (ops, sizes, most, half)
    }

    /// The fragmentation the open world showed ("heap exhausted" with 901 MB on the free lists):
    /// with sizes that seldom repeat exactly, the old allocator's top keeps climbing with every
    /// cell loaded, while the heap's stays within a small multiple of the bytes live at once and
    /// stops climbing.
    #[test]
    fn churn_of_varied_sizes_keeps_the_heap_bounded() {
        let (ops, sizes, most, half) = cell_churn(60);
        let mut m = Mem::new();
        let mut addrs = vec![0u32; sizes.len()];
        let mut at_half = 0;
        for (i, op) in ops.iter().enumerate() {
            if i == half {
                at_half = m.heap_top() - HEAP_BASE;
            }
            match *op {
                Op::Alloc(id, s) => addrs[id] = m.alloc(s),
                Op::Free(id) => m.free(addrs[id]),
            }
        }
        m.check_heap();
        let new = m.heap_top() - HEAP_BASE;
        let mut old = OldHeap::new();
        let mut old_half = 0;
        for (i, op) in ops.iter().enumerate() {
            if i == half {
                old_half = old.brk - HEAP_BASE;
            }
            match *op {
                Op::Alloc(id, s) => addrs[id] = old.take(s),
                Op::Free(id) => old.list(sizes[id]).push(addrs[id]),
            }
        }
        let old_top = old.brk - HEAP_BASE;
        let mb = |b: u32| f64::from(b) / f64::from(1 << 20);
        eprintln!(
            "most live {:.1} MB; heap top {:.1} MB (half way {:.1}); old allocator {:.1} MB (half \
             way {:.1})",
            most as f64 / f64::from(1 << 20),
            mb(new),
            mb(at_half),
            mb(old_top),
            mb(old_half)
        );
        assert!(
            u64::from(new) <= most * 3 / 2 + heap::QUICK_BYTES,
            "the heap reached {new:#x} bytes for {most:#x} live"
        );
        // Both tops hold whatever waits in the quick stacks, so the climb is measured without
        // an allowance for them.
        assert!(
            u64::from(new) <= u64::from(at_half) * 5 / 4,
            "the heap kept climbing: {at_half:#x} half way, {new:#x} at the end"
        );
        // The pattern is one the old allocator could not hold: it climbs without bound.
        assert!(u64::from(old_top) > most * 5, "{old_top:#x}");
        assert!(old_top > old_half * 3 / 2, "{old_half:#x} -> {old_top:#x}");
    }

    /// What the speed comparison runs on.
    trait Allocator {
        fn alloc(&mut self, size: u32) -> u32;
        fn free(&mut self, addr: u32);
        fn top(&self) -> u32;
    }

    impl Allocator for Mem {
        #[inline]
        fn alloc(&mut self, size: u32) -> u32 {
            Mem::alloc(self, size)
        }
        #[inline]
        fn free(&mut self, addr: u32) {
            Mem::free(self, addr)
        }
        fn top(&self) -> u32 {
            self.heap_top()
        }
    }

    /// [`Mem::alloc`] and [`Mem::free`] as they were with the old allocator (the same steps
    /// around it: rounding, mapping, the watch test, zero-fill, the live map), on a `Mem` used
    /// for its pages.
    struct OldMem {
        mem: Mem,
        heap: OldHeap,
        live: crate::engine::AddrMap<u32, u32>,
    }

    impl Allocator for OldMem {
        #[inline]
        fn alloc(&mut self, size: u32) -> u32 {
            let guard = if self.mem.check.is_some() { GUARD } else { 0 };
            let size = size.max(1).div_ceil(8) * 8 + guard;
            let addr = self.heap.take(size);
            self.mem.map(addr, size);
            if self.mem.watched_page(addr, size) {
                self.mem.watch_note(addr, size, None, WatchKind::Alloc);
            }
            self.mem.zero_raw(addr, size);
            self.live.insert(addr, size);
            addr
        }
        #[inline]
        fn free(&mut self, addr: u32) {
            if addr == 0 {
                return;
            }
            let size = self.live.remove(&addr).expect("live block");
            if self.mem.watched_page(addr, size) {
                self.mem.watch_note(addr, size, None, WatchKind::Free);
            }
            assert!(self.mem.check.is_none());
            self.heap.list(size).push(addr);
        }
        fn top(&self) -> u32 {
            self.heap.brk
        }
    }

    /// `steps` frees of a random one of `slots` live blocks, each followed by an allocation of a
    /// size from `size`, after filling the slots; answers ns per free and allocation.
    fn churn_ns(
        h: &mut impl Allocator,
        slots: usize,
        steps: u32,
        size: fn(&mut Rng) -> u32,
    ) -> f64 {
        let mut rng = Rng(0x5851_f42d_4c95_7f2d);
        let mut live: Vec<u32> = (0..slots).map(|_| h.alloc(size(&mut rng))).collect();
        let t = std::time::Instant::now();
        for _ in 0..steps {
            let i = rng.below(slots as u32) as usize;
            h.free(live[i]);
            live[i] = h.alloc(size(&mut rng));
        }
        let ns = t.elapsed().as_nanos() as f64 / f64::from(steps);
        for a in live {
            h.free(a);
        }
        ns
    }

    /// Not a check: ns per free and allocation of the heap against the allocator before the
    /// bins, the best of three runs each, on small blocks alone, on blocks of 264 bytes to 4 KB
    /// alone (every size, and five sizes) and on a game-like mix (`cargo test --release -p
    /// engine-core --lib heap_speed -- --ignored --nocapture`).
    #[test]
    #[ignore]
    fn heap_speed() {
        fn small(rng: &mut Rng) -> u32 {
            8 * (1 + rng.below(32).min(rng.below(32)))
        }
        // Every multiple of 8 from 264 to 4096.
        fn mid(rng: &mut Rng) -> u32 {
            264 + rng.below(480) * 8
        }
        fn five(rng: &mut Rng) -> u32 {
            [264, 512, 1024, 2048, 4096][rng.below(5) as usize]
        }
        // 89% up to 256 bytes, 10% 264 bytes to 4 KB, 1% 1-64 KB (64 sizes of each), 0.005%
        // multi-MB (8454160 is one of the open world's).
        fn game(rng: &mut Rng) -> u32 {
            match rng.below(100_000) {
                0..=88_999 => small(rng),
                89_000..=98_999 => 264 + rng.below(64) * 60,
                99_000..=99_994 => 1024 + rng.below(64) * 1008,
                _ => [1 << 20, (2 << 20) + 16, 4 << 20, 8_454_160][rng.below(4) as usize],
            }
        }
        let mixes: [(&str, fn(&mut Rng) -> u32); 4] = [
            ("small 8-256", small),
            ("mid 264-4096", mid),
            ("five sizes 264-4096", five),
            ("game mix", game),
        ];
        for (name, size) in mixes {
            let (mut old, mut new, mut bins) = (f64::MAX, f64::MAX, f64::MAX);
            let (mut old_top, mut new_top) = (0, 0);
            for _ in 0..3 {
                let mut o = OldMem {
                    mem: Mem::new(),
                    heap: OldHeap::new(),
                    live: Default::default(),
                };
                old = old.min(churn_ns(&mut o, 50_000, 5_000_000, size));
                old_top = o.top() - HEAP_BASE;
                drop(o);
                let mut m = Mem::new();
                new = new.min(churn_ns(&mut m, 50_000, 5_000_000, size));
                new_top = m.top() - HEAP_BASE;
                drop(m);
                let mut m = Mem::new();
                m.heap = heap::Heap::new().without_quick();
                bins = bins.min(churn_ns(&mut m, 50_000, 5_000_000, size));
            }
            eprintln!(
                "heap speed, {name}: old {old:.1} ns, new {new:.1} ns ({:+.0}%), bins alone \
                 {bins:.1} ns ({:+.0}%) per free and allocation; top old {:.1} MB, new {:.1} MB",
                (new / old - 1.0) * 100.0,
                (bins / old - 1.0) * 100.0,
                f64::from(old_top) / f64::from(1 << 20),
                f64::from(new_top) / f64::from(1 << 20)
            );
        }
    }

    /// The text of a caught panic.
    fn panic_text<T>(r: std::thread::Result<T>) -> String {
        let why = r.err().expect("a panic");
        why.downcast_ref::<String>()
            .cloned()
            .or_else(|| why.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default()
    }

    /// Mapping a range again keeps the bytes of its pages already mapped; its new pages read
    /// zero, and the pages around it stay unmapped.
    #[test]
    fn map_keeps_the_bytes_of_pages_already_mapped() {
        let mut m = Mem::new();
        m.map(0x3000, 0x1000);
        m.write(0x3ff0, &[0xab; 0x10]);
        // Pages 0x2000, 0x3000 and 0x4000.
        m.map(0x2800, 0x2000);
        assert!(m.is_mapped(0x2000) && m.is_mapped(0x4fff));
        assert!(!m.is_mapped(0x1fff) && !m.is_mapped(0x5000));
        assert_eq!(m.bytes(0x3ff0, 0x10), [0xab; 0x10]);
        assert_eq!(m.u32(0x3ffc), 0xabab_abab);
        assert!(m.bytes(0x2000, 0x1000).iter().all(|&b| b == 0));
        assert!(m.bytes(0x4000, 0x1000).iter().all(|&b| b == 0));
        assert!(m.bytes(0x3000, 0xff0).iter().all(|&b| b == 0));
    }

    /// Unmapped pages lose their bytes: reads of them fault, and mapped again they read zero;
    /// a range with unmapped pages inside it unmaps as a whole.
    #[test]
    fn unmap_then_map_reads_zero() {
        let mut m = Mem::new();
        m.map(0x1_0000, 0x3000);
        m.write(0x1_0000, &[0x5a; 0x3000]);
        m.unmap(0x1_1000, 0x1000);
        assert!(m.is_mapped(0x1_0fff) && !m.is_mapped(0x1_1000) && m.is_mapped(0x1_2000));
        let mut b = [0; 8];
        assert_eq!(m.try_read(0x1_0ffc, &mut b), Err(Fault(0x1_1000)));
        assert_eq!(m.u32_in_page(0x1_1000), None);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.u32(0x1_1abc)));
        assert_eq!(panic_text(r), "read of unmapped memory at 00011abc");
        m.map(0x1_1000, 0x1000);
        assert!(m.bytes(0x1_1000, 0x1000).iter().all(|&b| b == 0));
        assert_eq!(m.u32(0x1_0ffc), 0x5a5a_5a5a);
        assert_eq!(m.u32(0x1_2000), 0x5a5a_5a5a);
        m.write(0x1_1000, &[0x77; 0x1000]);
        m.unmap(0x1_0000, 0x8000);
        assert!(!(0x1_0000..0x1_8000u32)
            .step_by(PAGE)
            .any(|a| m.is_mapped(a)));
        m.map(0x1_0000, 0x3000);
        assert!(m.bytes(0x1_0000, 0x3000).iter().all(|&b| b == 0));
    }

    /// A small unmap keeps its pages committed without a host call: they fault like any
    /// unmapped page (on the fast path, the slow path and bulk reads), and mapped again they are
    /// zeroed in place, again without a host call.
    #[test]
    fn small_unmaps_keep_their_pages_and_map_zeroes_them() {
        let mut m = Mem::new();
        m.map(0x10_0000, 0x1_0000);
        assert_eq!(m.host_calls(), 1, "sixteen new pages, one run");
        m.write(0x10_0000, &[0x5a; 0x1_0000]);
        m.unmap(0x10_4000, 0x4000);
        assert_eq!(m.host_calls(), 1, "kept, not handed back");
        assert_eq!(m.kept, 4);
        assert!(m.is_mapped(0x10_3fff) && m.is_mapped(0x10_8000));
        assert!(!m.is_mapped(0x10_4000) && !m.is_mapped(0x10_7fff));
        let mut b = [0; 8];
        assert_eq!(m.try_read(0x10_3ffc, &mut b), Err(Fault(0x10_4000)));
        assert_eq!(m.u32_in_page(0x10_3ffe), None);
        assert_eq!(m.u32_in_page(0x10_4000), None);
        assert_eq!(m.u32(0x10_3ffc), 0x5a5a_5a5a);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.u32(0x10_3ffe)));
        assert_eq!(panic_text(r), "read of unmapped memory at 00104000");
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.u64(0x10_5000)));
        assert_eq!(panic_text(r), "read of unmapped memory at 00105000");
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.set_u32(0x10_7ffe, 1)));
        assert_eq!(panic_text(r), "write to unmapped memory at 00107ffe");
        // Unmapping kept pages again changes nothing.
        m.unmap(0x10_4000, 0x4000);
        assert_eq!((m.kept, m.host_calls()), (4, 1));
        m.map(0x10_4000, 0x4000);
        assert_eq!(m.host_calls(), 1, "zeroed in place");
        assert_eq!(m.kept, 0);
        assert!(m.bytes(0x10_4000, 0x4000).iter().all(|&b| b == 0));
        assert_eq!(m.u32(0x10_3ffe), 0x5a5a);
        assert_eq!(m.bytes(0x10_8000, 0x8000), vec![0x5a; 0x8000]);
    }

    /// One `map` across mapped, kept and unmapped pages keeps the mapped pages' bytes, zeroes
    /// the kept ones in place and commits each run of unmapped ones with one host call.
    #[test]
    fn map_across_mapped_kept_and_unmapped_pages() {
        let mut m = Mem::new();
        // Pages 0x20..0x28: mapped, kept, mapped, unmapped, kept, unmapped, unmapped, mapped.
        m.map(0x2_0000, 0x3000);
        m.map(0x2_4000, 0x1000);
        m.map(0x2_7000, 0x1000);
        m.write(0x2_0000, &[0x11; 0x3000]);
        m.write(0x2_4000, &[0x22; 0x1000]);
        m.write(0x2_7000, &[0x33; 0x1000]);
        m.unmap(0x2_1000, 0x1000);
        m.unmap(0x2_4000, 0x1000);
        assert_eq!((m.kept, m.host_calls()), (2, 3));
        m.map(0x2_0000, 0x8000);
        assert_eq!(
            m.host_calls(),
            5,
            "pages 0x23 and 0x25 to 0x26, one call each"
        );
        assert_eq!(m.kept, 0);
        let b = m.bytes(0x2_0000, 0x8000);
        let page = |i: usize| &b[i * PAGE..(i + 1) * PAGE];
        assert!(page(0).iter().all(|&x| x == 0x11));
        assert!(page(2).iter().all(|&x| x == 0x11));
        assert!(page(7).iter().all(|&x| x == 0x33));
        for i in [1, 3, 4, 5, 6] {
            assert!(page(i).iter().all(|&x| x == 0), "page {i} reads zero");
        }
    }

    /// A small unmap across mapped, kept and unmapped pages keeps the mapped ones (the kept count
    /// grows by those alone) and leaves the others as they were; mapped again, the range reads
    /// zero, with only its unmapped page committed by a host call.
    #[test]
    fn a_small_unmap_across_mapped_kept_and_unmapped_pages() {
        let mut m = Mem::new();
        // Pages 0x30..0x36: mapped, kept, unmapped, mapped, mapped, kept.
        m.map(0x3_0000, 0x2000);
        m.map(0x3_3000, 0x3000);
        m.write(0x3_0000, &[0x44; 0x2000]);
        m.write(0x3_3000, &[0x55; 0x3000]);
        m.unmap(0x3_1000, 0x1000);
        m.unmap(0x3_5000, 0x1000);
        assert_eq!((m.kept, m.host_calls()), (2, 2));
        m.unmap(0x3_0000, 0x6000);
        assert_eq!((m.kept, m.host_calls()), (5, 2));
        assert!(!(0x30..0x36u32).any(|p| m.is_mapped(p << PAGE_BITS)));
        m.map(0x3_0000, 0x6000);
        assert_eq!(
            (m.kept, m.host_calls()),
            (0, 3),
            "page 0x32 alone committed"
        );
        assert!(m.bytes(0x3_0000, 0x6000).iter().all(|&b| b == 0));
    }

    /// An unmap of more than [`KEEP_SPAN`] pages hands them back to the system, with the kept
    /// pages among them, one host call for each run; mapped again they read zero.
    #[test]
    fn large_unmaps_hand_their_pages_back() {
        let mut m = Mem::new();
        let len = (KEEP_SPAN as u32 + 2) << PAGE_BITS;
        m.map(0x100_0000, len);
        m.set_u32(0x100_0000, 7);
        m.set_u32(0x100_3000, 8);
        m.set_u32(0x100_0000 + len - 4, 9);
        m.unmap(0x100_3000, 0x1000);
        assert_eq!((m.kept, m.host_calls()), (1, 1));
        // KEEP_SPAN + 1 mapped pages, and the kept one between them: one run.
        m.unmap(0x100_0000, len);
        assert_eq!((m.kept, m.host_calls()), (0, 2));
        assert!(!m.is_mapped(0x100_0000) && !m.is_mapped(0x100_0000 + len - 1));
        m.map(0x100_0000, len);
        assert_eq!(m.host_calls(), 3);
        assert_eq!(
            (
                m.u32(0x100_0000),
                m.u32(0x100_3000),
                m.u32(0x100_0000 + len - 4)
            ),
            (0, 0, 0)
        );
    }

    /// Kept pages never pass [`KEEP_MAX`]: the unmap that would keep more first hands back the
    /// oldest kept pages (here [`KEEP_BATCH`] of one run, one host call), then keeps its own.
    #[test]
    fn kept_pages_stay_under_the_limit() {
        let mut m = Mem::new();
        const AT: u32 = 0x1000_0000;
        let pages = KEEP_MAX as u32 + 2;
        m.map(AT, pages << PAGE_BITS);
        for i in 0..pages {
            m.set_u32(AT + (i << PAGE_BITS), i + 1);
        }
        for i in 0..KEEP_MAX as u32 {
            m.unmap(AT + (i << PAGE_BITS), 0x1000);
        }
        assert_eq!((m.kept, m.host_calls()), (KEEP_MAX, 1));
        m.unmap(AT + ((KEEP_MAX as u32) << PAGE_BITS), 0x1000);
        assert_eq!((m.kept, m.host_calls()), (KEEP_MAX - KEEP_BATCH + 1, 2));
        let first = (AT >> PAGE_BITS) as usize;
        assert_eq!(m.page_state(first + KEEP_BATCH - 1), UNMAPPED);
        assert_eq!(m.page_state(first + KEEP_BATCH), KEPT);
        assert!(m.is_mapped(AT + ((pages - 1) << PAGE_BITS)));
        m.check_pages();
        // The pages handed back are committed again in one call, the kept ones zeroed in place.
        m.map(AT, pages << PAGE_BITS);
        assert_eq!((m.kept, m.host_calls()), (0, 3));
        let batch = KEEP_BATCH as u32;
        for i in [0, batch - 1, batch, KEEP_MAX as u32 - 1, KEEP_MAX as u32] {
            assert_eq!(m.u32(AT + (i << PAGE_BITS)), 0, "page {i} reads zero");
        }
        assert_eq!(m.u32(AT + ((pages - 1) << PAGE_BITS)), pages);
        m.check_pages();
    }

    /// One-page unmaps past [`KEEP_MAX`] with the kept pages in one run (a window of pages
    /// sliding forward, as a pool's or a scrap heap's pages can) hand the oldest back
    /// [`KEEP_BATCH`] at a time: one host call per [`KEEP_BATCH`] unmaps, not one each (each
    /// call also interrupts the process's other running threads).
    #[test]
    fn a_window_sliding_past_the_limit_hands_back_kept_pages_in_batches() {
        let mut m = Mem::new();
        const AT: u32 = 0x3000_0000;
        const PAST: u32 = 4 * KEEP_BATCH as u32;
        let pages = KEEP_MAX as u32 + PAST;
        m.map(AT, pages << PAGE_BITS);
        for i in 0..KEEP_MAX as u32 {
            m.unmap(AT + (i << PAGE_BITS), 0x1000);
        }
        assert_eq!((m.kept, m.host_calls()), (KEEP_MAX, 1));
        // (unmaps past the limit before it, host calls it made) for each unmap that made any.
        let mut calls = Vec::new();
        for i in 0..PAST {
            let before = m.host_calls();
            m.unmap(AT + ((KEEP_MAX as u32 + i) << PAGE_BITS), 0x1000);
            if m.host_calls() != before {
                calls.push((i, m.host_calls() - before));
            }
            assert!(m.kept <= KEEP_MAX);
        }
        let first_calls = &calls[..calls.len().min(8)];
        assert_eq!(
            calls.len(),
            4,
            "unmaps that made host calls: {first_calls:?}..."
        );
        let batches: Vec<(u32, u32)> = (0..4).map(|k| (k * KEEP_BATCH as u32, 1)).collect();
        assert_eq!(calls, batches);
        // The oldest pages went back, the newest KEEP_MAX are kept.
        let first = (AT >> PAGE_BITS) as usize;
        assert_eq!(m.page_state(first + PAST as usize - 1), UNMAPPED);
        assert_eq!(m.page_state(first + PAST as usize), KEPT);
        assert_eq!(m.kept, KEEP_MAX);
        m.check_pages();
    }

    /// With the kept pages scattered (every other page kept: 4096 runs), the unmap past
    /// [`KEEP_MAX`] hands back [`KEEP_BATCH_RUNS`] runs, a host call each (handing every kept
    /// page back made 4096 and took about 3 ms); each hand-back goes on from where the last
    /// stopped, the next kept pages round the space. One that goes over by more than those runs
    /// hold hands its own pages back instead, a host call per run of its range (it handed back
    /// a kept page a call, one per page it went over by).
    #[test]
    fn scattered_kept_pages_cost_an_unmap_a_bounded_hand_back() {
        let mut m = Mem::new();
        const AT: u32 = 0x2000_0000;
        let pages = 2 * KEEP_MAX as u32 + 2;
        m.map(AT, pages << PAGE_BITS);
        for i in 0..KEEP_MAX as u32 {
            m.unmap(AT + ((2 * i) << PAGE_BITS), 0x1000);
        }
        assert_eq!((m.kept, m.host_calls()), (KEEP_MAX, 1));
        let page = |i: usize| (AT >> PAGE_BITS) as usize + i;
        let runs = KEEP_BATCH_RUNS;
        let calls = 1 + runs as u32;
        // One page over: the first KEEP_BATCH_RUNS kept pages go, pages 0 to 14.
        m.unmap(AT + ((pages - 1) << PAGE_BITS), 0x1000);
        assert_eq!((m.kept, m.host_calls()), (KEEP_MAX - runs + 1, calls));
        for p in (0..2 * runs).step_by(2) {
            assert_eq!(m.page_state(page(p)), UNMAPPED, "page {p}");
        }
        assert_eq!(m.page_state(page(2 * runs)), KEPT);
        // The next seven one-page unmaps (pages 1 to 13) keep their pages without a host call.
        for p in 0..runs as u32 - 1 {
            m.unmap(AT + ((2 * p + 1) << PAGE_BITS), 0x1000);
        }
        assert_eq!((m.kept, m.host_calls()), (KEEP_MAX, calls));
        // Pages 15 to 38: twelve mapped (the odd ones), so twelve over, more than the next
        // KEEP_BATCH_RUNS runs hold (the kept pages 16, 18, ... 30, one each). The range goes
        // back instead, its kept pages (the even ones) with it: one run, one call.
        let from = 2 * runs - 1;
        let hand = m.keep_hand;
        m.unmap(AT + ((from as u32) << PAGE_BITS), 24 << PAGE_BITS);
        assert_eq!((m.kept, m.host_calls()), (KEEP_MAX - 12, calls + 1));
        for p in from..from + 24 {
            assert_eq!(m.page_state(page(p)), UNMAPPED, "page {p}");
        }
        assert_eq!(m.page_state(page(from + 25)), KEPT);
        assert_eq!(
            m.keep_hand, hand,
            "no kept page outside the range handed back"
        );
        m.check_pages();
    }

    /// An unmap of up to [`KEEP_SPAN`] pages at [`KEEP_MAX`] makes at most [`KEEP_BATCH_RUNS`]
    /// host calls for the kept pages, however far it goes over. With the kept pages scattered
    /// (4096 runs), a 1024-page unmap (half of an 8 MB scrap heap, which the game decommits in
    /// one `VirtualFree`) made 1024 calls, one per page it went over by, and took 1-4 ms; now its
    /// own pages go back in one call. Where the next runs hold enough (here one run of kept pages
    /// that the hand reaches before the scattered ones), it hands those back and keeps its own.
    #[test]
    fn an_unmap_far_past_the_limit_makes_a_bounded_number_of_host_calls() {
        let mut m = Mem::new();
        const AT: u32 = 0x2000_0000;
        const HALF: u32 = 0x4000_0000;
        let half = KEEP_SPAN as u32;
        m.map(AT, (2 * KEEP_MAX as u32) << PAGE_BITS);
        for i in 0..KEEP_MAX as u32 {
            m.unmap(AT + ((2 * i) << PAGE_BITS), 0x1000);
        }
        assert_eq!((m.kept, m.host_calls()), (KEEP_MAX, 1));
        m.map(HALF, half << PAGE_BITS);
        for i in 0..half {
            m.set_u32(HALF + (i << PAGE_BITS), i + 1);
        }
        let before = m.host_calls();
        m.unmap(HALF, half << PAGE_BITS);
        assert_eq!((m.kept, m.host_calls() - before), (KEEP_MAX, 1));
        assert!(!m.is_mapped(HALF) && m.page_state((HALF >> PAGE_BITS) as usize) == UNMAPPED);
        m.check_pages();
        // Mapped again, its pages read zero (committed afresh, one call).
        m.map(HALF, half << PAGE_BITS);
        assert_eq!(m.host_calls() - before, 2);
        assert!((0..half).all(|i| m.u32(HALF + (i << PAGE_BITS)) == 0));
        // The first 1024 scattered kept pages mapped again, the same number kept in one run
        // below them, where the hand (still at page 0) comes first: the next unmap of the half
        // hands that run back in one call and keeps its own pages.
        let first = (AT >> PAGE_BITS) as usize;
        let run = 0x1000_0000;
        m.map(AT, (2 * half) << PAGE_BITS);
        m.map(run, half << PAGE_BITS);
        m.unmap(run, half << PAGE_BITS);
        assert_eq!((m.kept, m.keep_hand), (KEEP_MAX, 0));
        let before = m.host_calls();
        m.unmap(HALF, half << PAGE_BITS);
        assert_eq!((m.kept, m.host_calls() - before), (KEEP_MAX, 1));
        assert_eq!(m.page_state((run >> PAGE_BITS) as usize), UNMAPPED);
        assert_eq!(m.page_state((HALF >> PAGE_BITS) as usize), KEPT);
        assert_eq!(m.page_state(first + 2 * half as usize), KEPT);
        m.check_pages();
    }

    /// A hand-back whose run of kept pages ends at the last page of the space, short of
    /// [`KEEP_BATCH`] and with host calls to spare ([`KEEP_BATCH_RUNS`]), goes on from page 0.
    /// Its next look started at [`PAGES`], one group past the end of [`Mem::kept_groups`], and
    /// the unmap panicked (index out of bounds). The game can reach it: `units/platform.rs`
    /// takes a fixed reservation that ends at 4 GB, and a memory pool tries 16 MB slots up to
    /// 0xFF000000 and decommits single pages.
    #[test]
    fn a_hand_back_that_reaches_the_end_of_the_space_goes_on_from_its_start() {
        let mut m = Mem::new();
        const LOW: u32 = 0x1000_0000;
        let page = |a: u32| (a >> PAGE_BITS) as usize;
        // The last page of the space, then 4095 pages in one run at LOW: 4096 kept.
        m.map(0xffff_f000, 0x1000);
        m.unmap(0xffff_f000, 0x1000);
        let low_pages = KEEP_MAX as u32 - 1;
        m.map(LOW, low_pages << PAGE_BITS);
        for at in (0..low_pages).step_by(KEEP_SPAN) {
            let len = (low_pages - at).min(KEEP_SPAN as u32);
            m.unmap(LOW + (at << PAGE_BITS), len << PAGE_BITS);
        }
        assert_eq!(m.kept, KEEP_MAX);
        // Unmaps of 1000, 1000, 1000 and 1024 pages below LOW (behind the hand) each hand back
        // as many kept pages at LOW, one run a call, and keep their own: still 4096 kept.
        let behind = [
            (0x0100_0000, 1000),
            (0x0200_0000, 1000),
            (0x0300_0000, 1000),
            (0x0400_0000, 1024),
        ];
        for (at, pages) in behind {
            m.map(at, pages << PAGE_BITS);
            m.unmap(at, pages << PAGE_BITS);
        }
        assert_eq!(m.kept, KEEP_MAX);
        assert_eq!(m.keep_hand, page(LOW) + 4024);
        // 71 kept pages are left at LOW, then the last page of the space. A one-page unmap
        // hands back those 72 (two runs, the second ending at the end of the space), then goes
        // on round the space: the first 184 of the 1000 pages kept at 0x01000000.
        const SINGLE: u32 = 0x0500_0000;
        m.map(SINGLE, 0x1000);
        let before = m.host_calls();
        m.unmap(SINGLE, 0x1000);
        assert_eq!(m.host_calls() - before, 3);
        assert_eq!(m.kept, KEEP_MAX - KEEP_BATCH + 1);
        assert_eq!(m.page_state(page(LOW) + low_pages as usize - 1), UNMAPPED);
        assert_eq!(m.page_state(PAGES - 1), UNMAPPED);
        let wrapped = page(behind[0].0);
        assert_eq!(m.page_state(wrapped + 183), UNMAPPED);
        assert_eq!(m.page_state(wrapped + 184), KEPT);
        assert_eq!(m.keep_hand, wrapped + 184);
        m.check_pages();
    }

    /// The look for the next kept page ([`Mem::next_kept`]) goes round the space through the
    /// groups' bits: from any page it finds the first kept page at or after it, in its own group
    /// or a later one, and round from the start to the pages of its own group before it; a
    /// group whose kept pages are all mapped again loses its bit.
    #[test]
    fn the_look_for_the_next_kept_page_goes_round_the_space() {
        let mut m = Mem::new();
        let last = PAGES - 1;
        let kept = [5, 200, 240, 0x5_0000, last];
        for &p in &kept {
            m.map((p << PAGE_BITS) as u32, 0x1000);
            m.unmap((p << PAGE_BITS) as u32, 0x1000);
        }
        m.check_pages();
        let first_kept_from = |m: &Mem, from: usize| {
            (from..PAGES)
                .chain(0..from)
                .find(|&p| m.page_state(p) == KEPT)
                .expect("a kept page")
        };
        let froms = [
            0,
            5,
            6,
            63,
            64,
            199,
            201,
            241,
            0x4_ffff,
            0x5_0001,
            last - 64,
            last,
        ];
        for from in froms {
            assert_eq!(
                m.next_kept(from),
                first_kept_from(&m, from),
                "from {from:#x}"
            );
        }
        // From the end of the space (where a hand-back's run that ends at the last page leaves
        // its next look): round to the start.
        assert_eq!(m.next_kept(PAGES), first_kept_from(&m, 0));
        // Only pages 5 and 200 kept: from past them the look goes round the space.
        for &p in &kept[2..] {
            m.map((p << PAGE_BITS) as u32, 0x1000);
        }
        m.check_pages();
        assert_eq!(m.kept, 2);
        for (from, want) in [(201, 5), (6, 200), (0x8_0000, 5), (last, 5), (5, 5)] {
            assert_eq!(m.next_kept(from), want, "from {from:#x}");
        }
        // Page 200 alone: from just past it, round the space to it.
        m.map(5 << PAGE_BITS, 0x1000);
        assert_eq!(m.next_kept(201), 200);
        assert_eq!(m.next_kept(0), 200);
        assert_eq!(m.next_kept(PAGES), 200);
        m.check_pages();
    }

    /// The last page of a mapped range (its next page unmapped, so off the pair test) reads and
    /// writes every width at every offset that stays inside it, without touching the next page;
    /// a value that runs one byte further faults at the next page's first byte, a store after
    /// writing the bytes before it.
    #[test]
    fn the_last_page_of_a_range_reaches_its_last_word() {
        let mut m = Mem::new();
        m.map(0x7000, 0x1000);
        m.set_u64(0x7ff8, 0x0102_0304_0506_0708);
        m.set_u32(0x7ffc, 0xcafe_f00d);
        assert_eq!(m.u32(0x7ffc), 0xcafe_f00d);
        assert_eq!(m.u64(0x7ff8), 0xcafe_f00d_0506_0708);
        assert_eq!(m.u16(0x7ffe), 0xcafe);
        assert_eq!(m.u8(0x7fff), 0xca);
        assert_eq!(m.u32_in_page(0x7ffc), Some(0xcafe_f00d));
        assert_eq!(m.u32_in_page(0x7ffd), None);
        for a in 0x7ff0..0x8000u32 {
            m.set_u8(a, a as u8);
        }
        for a in 0x7ff0..0x8000u32 {
            let left = (0x8000 - a) as usize;
            let b = m.bytes(a, left as u32);
            assert_eq!(m.u8(a), b[0]);
            if left >= 2 {
                assert_eq!(m.u16(a), u16::from_le_bytes(b[..2].try_into().unwrap()));
            }
            if left >= 4 {
                assert_eq!(m.u32(a), u32::from_le_bytes(b[..4].try_into().unwrap()));
            }
            if left >= 8 {
                assert_eq!(m.u64(a), u64::from_le_bytes(b[..8].try_into().unwrap()));
            }
        }
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.u32(0x7ffd)));
        assert_eq!(panic_text(r), "read of unmapped memory at 00008000");
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.u64(0x7ff9)));
        assert_eq!(panic_text(r), "read of unmapped memory at 00008000");
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            m.set_u32(0x7ffe, 0x5566_7788)
        }));
        assert_eq!(panic_text(r), "write to unmapped memory at 00008000");
        assert_eq!(m.u16(0x7ffe), 0x7788);
    }

    /// The last page of the 4 GB space is an ordinary page: the fast path reads its last word,
    /// and a value that runs past its end wraps to address 0, as with the page table before.
    #[test]
    fn the_last_page_of_the_space_wraps_to_address_zero() {
        let mut m = Mem::new();
        m.map(0xffff_f000, 0x1000);
        m.set_u32(0xffff_fffc, 0x1122_3344);
        assert_eq!(m.u32(0xffff_fffc), 0x1122_3344);
        assert_eq!(m.u32_in_page(0xffff_fffc), Some(0x1122_3344));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.u32(0xffff_fffe)));
        assert_eq!(panic_text(r), "read of unmapped memory at 00000000");
        m.map(0, 0x1000);
        m.set_u32(0xffff_fffe, 0xaabb_ccdd);
        assert_eq!(m.bytes(0xffff_fffe, 4), [0xdd, 0xcc, 0xbb, 0xaa]);
        assert_eq!(m.u16(0), 0xaabb);
        assert_eq!(m.u32(0xffff_fffe), 0xaabb_ccdd);
        m.unmap(0xffff_f000, 0x1000);
        assert!(!m.is_mapped(0xffff_ffff) && m.is_mapped(0));
    }

    /// A value across two pages mapped by separate calls reads and writes as its bytes do (the
    /// fast path takes it whole, the guest space being one block); `u32_in_page` reads it too.
    #[test]
    fn a_value_across_pages_mapped_apart() {
        let mut m = Mem::new();
        m.map(0x2_1000, 0x1000);
        m.map(0x2_0000, 0x1000);
        m.set_u64(0x2_0ffd, 0x8877_6655_4433_2211);
        assert_eq!(
            m.bytes(0x2_0ffd, 8),
            [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]
        );
        assert_eq!(m.u64(0x2_0ffd), 0x8877_6655_4433_2211);
        assert_eq!(m.u32(0x2_0ffe), 0x5544_3322);
        assert_eq!(m.u32_in_page(0x2_0ffe), Some(0x5544_3322));
        assert_eq!(m.u32_in_page(0x2_1000), Some(0x7766_5544));
        // Unmapping the second page leaves the word across the boundary to the slow path.
        m.unmap(0x2_1000, 0x1000);
        assert_eq!(m.u32_in_page(0x2_0ffe), None);
        assert_eq!(m.u32_in_page(0x2_0ffc), Some(0x3322_1100));
        m.map(0x2_1000, 0x1000);
        assert_eq!(m.u32(0x2_0ffe), 0x3322);
        m.write(0x2_0ffe, b"abc\0");
        assert_eq!(m.cstr(0x2_0ffe), b"abc");
    }

    /// Bulk accesses fault where they did before: a read or write at the first byte of the
    /// first unmapped page (a write after storing the bytes before it), a string that runs
    /// into an unmapped page at that page's first byte, a zeroing likewise after zeroing what
    /// lies before it.
    #[test]
    fn bulk_accesses_fault_at_the_first_unmapped_byte() {
        let mut m = Mem::new();
        m.map(0x4000, 0x2000);
        let mut buf = vec![0; 0x3000];
        assert_eq!(m.try_read(0x4800, &mut buf), Err(Fault(0x6000)));
        assert_eq!(m.try_read(0x3fff, &mut buf[..2]), Err(Fault(0x3fff)));
        assert_eq!(m.try_write(0x5ffe, &[1, 2, 3, 4]), Err(Fault(0x6000)));
        assert_eq!(m.bytes(0x5ffe, 2), [1, 2]);
        m.write(0x5ff0, &[b'x'; 0x10]);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.cstr(0x5ff8)));
        assert_eq!(panic_text(r), "read of unmapped memory at 00006000");
        m.set_u8(0x5ff4, 0);
        assert_eq!(m.cstr(0x5ff0), b"xxxx");
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.zero(0x5ff0, 0x20)));
        assert_eq!(panic_text(r), "write to unmapped memory at 00006000");
        assert_eq!(m.bytes(0x5ff0, 0x10), [0; 0x10]);
    }

    /// A dropped `Mem` gives its 4 GB block of address space back: more `Mem`s are made and
    /// dropped here, one after another, than a Windows process's 128 TB could hold at once
    /// (32,768 blocks), each with a page in use.
    #[test]
    fn dropped_mems_give_their_address_space_back() {
        for i in 0..40_000u32 {
            let mut m = Mem::new();
            m.map(0x1000, 4);
            m.set_u32(0x1000, i);
            assert_eq!(m.u32(0x1000), i);
        }
    }

    /// `Mem` stays `Send` and `Sync`, as it was with boxed pages.
    #[test]
    fn mem_is_send_and_sync() {
        fn send_sync<T: Send + Sync>() {}
        send_sync::<Mem>();
    }

    /// The page table this module used before the flat block (one boxed page per slot), kept
    /// only to compare against in [`bench_random_word_access`].
    struct PagedMem {
        pages: Box<[Option<Box<[u8; PAGE]>>; PAGES]>,
        store_diag: bool,
    }

    #[cold]
    #[inline(never)]
    fn paged_slow(a: u32) -> u32 {
        panic!("the benchmark left its pages at {a:08x}")
    }

    impl PagedMem {
        fn new() -> Self {
            let mut pages = Vec::with_capacity(PAGES);
            pages.resize_with(PAGES, || None);
            PagedMem {
                pages: pages.into_boxed_slice().try_into().expect("PAGES slots"),
                // Not known to the compiler, as `Mem`'s is not: each store tests it.
                store_diag: std::hint::black_box(false),
            }
        }

        fn map(&mut self, addr: u32, len: u32) {
            for p in (addr >> PAGE_BITS)..=((addr + len - 1) >> PAGE_BITS) {
                self.pages[p as usize].get_or_insert_with(|| Box::new([0; PAGE]));
            }
        }

        fn unmap(&mut self, addr: u32, len: u32) {
            for p in (addr >> PAGE_BITS)..((addr + len) >> PAGE_BITS) {
                self.pages[p as usize] = None;
            }
        }

        /// What `units/platform.rs` did for each page it committed before the flat block: map
        /// the page, then write 4 KiB of zeros over it.
        fn commit_page(&mut self, page: u32) {
            self.map(page, PAGE as u32);
            let p = self.pages[(page >> PAGE_BITS) as usize].as_deref_mut();
            p.expect("mapped")
                .copy_from_slice(std::hint::black_box(&[0; PAGE]));
        }

        #[inline(always)]
        fn u32(&self, a: u32) -> u32 {
            let off = (a as usize) & (PAGE - 1);
            if off <= PAGE - 4 {
                if let Some(p) = self.pages[(a >> PAGE_BITS) as usize].as_deref() {
                    let mut b = [0; 4];
                    b.copy_from_slice(&p[off..off + 4]);
                    return u32::from_le_bytes(b);
                }
            }
            paged_slow(a)
        }

        #[inline(always)]
        fn set_u32(&mut self, a: u32, v: u32) {
            if self.store_diag {
                paged_slow(a);
            }
            let off = (a as usize) & (PAGE - 1);
            if off <= PAGE - 4 {
                if let Some(p) = self.pages[(a >> PAGE_BITS) as usize].as_deref_mut() {
                    p[off..off + 4].copy_from_slice(&v.to_le_bytes());
                    return;
                }
            }
            paged_slow(a);
        }
    }

    /// Not a check: what the game's `VirtualAlloc(MEM_COMMIT)` and `VirtualFree(MEM_DECOMMIT)`
    /// cost through `units/platform.rs`, in three versions: the page table before the flat block
    /// (`paged`: a boxed page each, mapped and zeroed page by page), the flat block handing every
    /// page back as it is unmapped and committing it afresh, page by page (`hand back`: the
    /// first version of the flat block), and the flat block as it is now (`now`: one call per
    /// run, small unmaps kept). Two patterns: one page committed, written and decommitted, over
    /// 64 pages in turn (a memory pool's pages), and a scrap heap growing from 64 KB to 1 MB by
    /// doubling, one word written in each page, and shrinking back by halving. Each with the
    /// process otherwise idle and with three other threads busy (as the viewer has: a decommit
    /// then has to interrupt them). Run with `cargo test --release -p engine-core --lib
    /// mem::tests::bench_commit_and_decommit -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn bench_commit_and_decommit() {
        use std::hint::black_box;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::time::Instant;
        const BASE: u32 = 0x0400_0000;
        const P: u32 = PAGE as u32;
        // The scrap heap's sizes: grown 64 KB -> 1 MB by doubling, then shrunk back by halving.
        const SIZES: [u32; 5] = [0x1_0000, 0x2_0000, 0x4_0000, 0x8_0000, 0x10_0000];

        /// Best of five, in nanoseconds per round of `f`.
        fn best(rounds: u32, f: &mut dyn FnMut()) -> f64 {
            (0..5)
                .map(|_| {
                    let t = Instant::now();
                    for _ in 0..rounds {
                        f();
                    }
                    t.elapsed().as_secs_f64() * 1e9 / rounds as f64
                })
                .fold(f64::INFINITY, f64::min)
        }

        let pool_paged = |m: &mut PagedMem| {
            for i in 0..64 {
                let page = BASE + i * P;
                m.commit_page(page);
                m.set_u32(page + 8, i);
                m.unmap(page, P);
            }
        };
        let pool_hand_back = |m: &mut Mem| {
            for i in 0..64 {
                let page = BASE + i * P;
                m.map(page, P);
                m.zero(page, P);
                m.set_u32(page + 8, i);
                let p = (page >> PAGE_BITS) as usize;
                m.hand_back(p, p + 1);
            }
        };
        let pool_now = |m: &mut Mem| {
            for i in 0..64 {
                let page = BASE + i * P;
                m.map(page, P);
                m.set_u32(page + 8, i);
                m.unmap(page, P);
            }
        };
        // The scrap heap's first 64 KB stay committed (mapped below, before the rounds).
        const HEAP: u32 = 0x0800_0000;
        let scrap_paged = |m: &mut PagedMem| {
            for w in SIZES.windows(2) {
                for page in (HEAP + w[0]..HEAP + w[1]).step_by(PAGE) {
                    m.commit_page(page);
                    m.set_u32(page, page);
                }
            }
            for w in SIZES.windows(2).rev() {
                m.unmap(HEAP + w[0], w[1] - w[0]);
            }
        };
        let scrap_hand_back = |m: &mut Mem| {
            for w in SIZES.windows(2) {
                for page in (HEAP + w[0]..HEAP + w[1]).step_by(PAGE) {
                    m.map(page, P);
                    m.zero(page, P);
                    m.set_u32(page, page);
                }
            }
            for w in SIZES.windows(2).rev() {
                for page in (HEAP + w[0]..HEAP + w[1]).step_by(PAGE) {
                    let p = (page >> PAGE_BITS) as usize;
                    m.hand_back(p, p + 1);
                }
            }
        };
        let scrap_now = |m: &mut Mem| {
            for w in SIZES.windows(2) {
                m.map(HEAP + w[0], w[1] - w[0]);
                for page in (HEAP + w[0]..HEAP + w[1]).step_by(PAGE) {
                    m.set_u32(page, page);
                }
            }
            for w in SIZES.windows(2).rev() {
                m.unmap(HEAP + w[0], w[1] - w[0]);
            }
        };

        let stop = AtomicBool::new(false);
        for busy in [0, 3] {
            std::thread::scope(|s| {
                for _ in 0..busy {
                    s.spawn(|| {
                        let mut n = 0u64;
                        while !stop.load(Ordering::Relaxed) {
                            n = black_box(n.wrapping_add(1));
                        }
                    });
                }
                for round in 0..3 {
                    let mut paged = PagedMem::new();
                    let mut flat = Mem::new();
                    paged.map(HEAP, SIZES[0]);
                    flat.map(HEAP, SIZES[0]);
                    let a = best(200, &mut || pool_paged(black_box(&mut paged)));
                    let b = best(200, &mut || pool_hand_back(black_box(&mut flat)));
                    let c = best(200, &mut || pool_now(black_box(&mut flat)));
                    println!(
                        "{busy} busy threads, round {round}: one page committed, written and \
                         decommitted, ns per page: paged {:.0}, hand back {:.0}, now {:.0}",
                        a / 64.0,
                        b / 64.0,
                        c / 64.0
                    );
                    let a = best(50, &mut || scrap_paged(black_box(&mut paged)));
                    let b = best(50, &mut || scrap_hand_back(black_box(&mut flat)));
                    let c = best(50, &mut || scrap_now(black_box(&mut flat)));
                    println!(
                        "{busy} busy threads, round {round}: scrap heap 64 KB -> 1 MB -> 64 KB, \
                         us per cycle: paged {:.0}, hand back {:.0}, now {:.0}",
                        a / 1e3,
                        b / 1e3,
                        c / 1e3
                    );
                }
                stop.store(true, Ordering::Relaxed);
            });
            stop.store(false, Ordering::Relaxed);
        }
    }

    /// Not a check: nanoseconds per access of random 4-byte loads, random 4-byte stores and a
    /// chain of dependent loads (each address read from the word before), on the flat block and
    /// on the page table it replaced, over working sets of 1 MB and 64 MB. Run with
    /// `cargo test --release -p engine-core --lib mem::tests::bench_random_word_access --
    /// --ignored --nocapture`.
    #[test]
    #[ignore]
    fn bench_random_word_access() {
        use std::hint::black_box;
        use std::time::Instant;
        const BASE: u32 = 0x2000_0000;
        const ACCESSES: usize = 1 << 23;
        let mut x = 0x9e37_79b9u32;
        let mut next = move || {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            x
        };
        for set in [1u32 << 20, 64 << 20] {
            let mut flat = Mem::new();
            let mut paged = PagedMem::new();
            flat.map(BASE, set);
            paged.map(BASE, set);
            // Both where the compiler cannot follow them (as `flat` is, handed to functions it
            // does not inline), so the loops below reload their fields alike.
            black_box(&mut flat);
            black_box(&mut paged);
            let words = (set / 4) as usize;
            let addrs: Vec<u32> = (0..ACCESSES)
                .map(|_| BASE + ((next() % set) & !3))
                .collect();
            // One cycle through every word of the set in a random order, for the chain.
            let mut order: Vec<u32> = (0..words as u32).map(|w| BASE + w * 4).collect();
            for i in (1..words).rev() {
                order.swap(i, next() as usize % (i + 1));
            }
            for i in 0..words {
                let to = order[(i + 1) % words];
                flat.set_u32(order[i], to);
                paged.set_u32(order[i], to);
            }
            let best = |f: &mut dyn FnMut() -> u32| {
                (0..5)
                    .map(|_| {
                        let t = Instant::now();
                        black_box(f());
                        t.elapsed().as_secs_f64() * 1e9 / ACCESSES as f64
                    })
                    .fold(f64::INFINITY, f64::min)
            };
            // Three rounds, each variant right after the other, so a move of the thread to
            // another kind of core (this machine has two) shows as a round apart.
            for round in 0..3 {
                let flat_load = best(&mut || addrs.iter().fold(0u32, |s, &a| s ^ flat.u32(a)));
                let paged_load = best(&mut || addrs.iter().fold(0u32, |s, &a| s ^ paged.u32(a)));
                let flat_chain = best(&mut || (0..ACCESSES).fold(BASE, |a, _| flat.u32(a)));
                let paged_chain = best(&mut || (0..ACCESSES).fold(BASE, |a, _| paged.u32(a)));
                // Stores write the address back, so the chain stays intact for the next set.
                let flat_store = best(&mut || {
                    for &a in &addrs {
                        flat.set_u32(a, black_box(flat.u32(a)));
                    }
                    0
                });
                let paged_store = best(&mut || {
                    for &a in &addrs {
                        paged.set_u32(a, black_box(paged.u32(a)));
                    }
                    0
                });
                println!(
                    "{} MB set, round {round}, ns per access: random load {flat_load:.2} flat, \
                     {paged_load:.2} paged; dependent load {flat_chain:.2} flat, {paged_chain:.2} \
                     paged; random load and store {flat_store:.2} flat, {paged_store:.2} paged",
                    set >> 20
                );
            }
        }
    }
}
