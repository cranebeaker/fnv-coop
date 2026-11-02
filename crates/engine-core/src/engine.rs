use crate::abi::{AbiFn, Arg, Ret, RetVal};
use crate::mem::Mem;
use crate::ptr::{Field, Layout, Ptr, Scalar};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::path::Path;
use std::rc::Rc;

extern "Rust" {
    /// The registrations of the engine crates above this one, defined once
    /// per program by [`crate::assemble!`] (docs/ENGINE_CRATE.md, "The engine
    /// crates"). A program that links this crate without it does not link.
    fn nv_engine_assembled_funcs(out: &mut Vec<(u32, AbiFn)>);
}

/// Defines which part crates' functions [`Engine::new`] registers besides
/// this crate's own, as a list of `funcs(out)` functions (each part crate's
/// `units::funcs`). Exactly one crate of a program invokes it: the `engine`
/// crate, with every part crate (so the full engine has every translated
/// function), or a part crate under `cfg(test)` for its own unit tests, with
/// itself and the crates below it (generated into its `units/mod.rs` by
/// `ledger units`).
#[macro_export]
macro_rules! assemble {
    ($($funcs:path),* $(,)?) => {
        #[doc(hidden)]
        #[no_mangle]
        pub fn nv_engine_assembled_funcs(out: &mut Vec<(u32, $crate::AbiFn)>) {
            let _ = &out;
            $( $funcs(out); )*
        }
    };
}

/// The C runtime's `__sse2_available` and `__locale_changed` words ([`Engine::new`]).
const CRT_SSE2_AVAILABLE: u32 = 0x0127_0a6c;
const CRT_LOCALE_CHANGED: u32 = 0x0126_fdf4;
/// The C runtime's math-error word (`011c2b60`, [`Engine::new`]) and its image value.
const CRT_MATHERR_SKIP: u32 = 0x011c_2b60;
const CRT_MATHERR_SKIP_IMAGE: u32 = 0x0000_2694;

/// A test double that may keep state (see [`Engine::register_double`]).
pub type Double = Rc<RefCell<dyn FnMut(&mut Engine, &[u32]) -> Ret>>;

/// What runs when a call reaches an address nobody has translated
/// ([`Engine::on_open`]): given the address and the argument words, it may
/// record the miss; `Some(ret)` makes the call return `ret` (a diagnostic
/// run that goes on past the gap), `None` stops as without a handler.
pub type OpenHandler = Rc<RefCell<dyn FnMut(&mut Engine, u32, &[u32]) -> Option<Ret>>>;

#[derive(Clone)]
enum Callable {
    Abi(AbiFn),
    Double(Double),
}

/// The hasher of the function table and the call profile: their keys are exe addresses (and
/// caller/address pairs), so a multiply whose high half is folded back into the low one
/// spreads them well, at a fraction of SipHash's cost ([`Engine::call`] runs millions of
/// times a second).
#[derive(Default, Clone, Copy)]
pub struct AddrHasher(u64);

impl Hasher for AddrHasher {
    fn finish(&self) -> u64 {
        let p = self.0.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        p ^ (p >> 32)
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(8) ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
    }
    fn write_u32(&mut self, n: u32) {
        self.0 = self.0.rotate_left(32) ^ u64::from(n);
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = self.0.rotate_left(32) ^ n;
    }
    fn write_usize(&mut self, n: usize) {
        self.write_u64(n as u64);
    }
}

/// A map keyed by exe address (or a caller/address pair) with [`AddrHasher`].
pub type AddrMap<K, V> = HashMap<K, V, BuildHasherDefault<AddrHasher>>;

/// A set of exe addresses with [`AddrHasher`].
pub type AddrSet<K> = std::collections::HashSet<K, BuildHasherDefault<AddrHasher>>;

/// How many entries [`Engine::call`]'s direct-mapped cache of plain functions has (a power of
/// two). 65536 (1 MB): a frame of the game calls thousands of functions, which collided in
/// 16384 slots (a29's `spread_dispatch_speed`: 9.2 to 8.4 ns a call over 8000 functions, 11.8
/// to 9.6 over 20000).
const CALL_CACHE_LEN: usize = 1 << 16;

/// The cache slot of `addr` (Fibonacci hashing: the top bits of the product).
#[inline(always)]
const fn cache_slot(addr: u32) -> usize {
    (addr.wrapping_mul(0x9e37_79b1) >> (32 - CALL_CACHE_LEN.trailing_zeros())) as usize
}

/// One entry of [`Engine::call`]'s cache: a plain function and its address.
#[derive(Clone, Copy)]
struct Cached {
    addr: u32,
    f: AbiFn,
}

// The empty entries' addresses ([`Cached::empty`]) map to other slots: 0 to slot 0, 1 elsewhere.
const _: () = assert!(cache_slot(0) == 0 && cache_slot(1) != 0);

impl Cached {
    /// The empty entry of `slot`: an address that maps to another slot, so no call's address
    /// matches it (the call's own address maps to `slot`), and a function that never runs. The
    /// lookup is then one compare, without a separate test for an empty entry.
    #[inline]
    fn empty(slot: usize) -> Cached {
        Cached {
            addr: u32::from(slot == 0),
            f: Cached::never_called,
        }
    }

    fn never_called(_: &mut Engine, _: &[u32]) -> Ret {
        unreachable!("an empty entry of the call cache was called")
    }
}

/// The game's memory plus every translated function, by exe address.
pub struct Engine {
    pub mem: Mem,
    funcs: AddrMap<u32, Callable>,
    /// A direct-mapped cache in front of `funcs` for plain functions (doubles never enter it):
    /// (address, function) at [`cache_slot`] of the address, [`Cached::empty`] when empty.
    /// [`Engine::register`] and [`Engine::register_double`] empty the slot of the address they
    /// change, so it always answers as `funcs` does. A fixed-size array: the slot needs no
    /// bounds check.
    call_cache: Box<[Cached; CALL_CACHE_LEN]>,
    /// When `Some`, every call made through [`Engine::call`] is recorded
    /// as (address, argument words). Tests use it to check what a
    /// translation called.
    pub call_log: Option<Vec<(u32, Vec<u32>)>>,
    tls: u32,
    /// Milliseconds (since the machine started once the real clock is on): the one clock `GetTickCount`, `timeGetTime` and
    /// `QueryPerformanceCounter` report ([`Engine::clock_now_ms`]). `Sleep(n)` moves it on by
    /// `n`; with the real clock on ([`Engine::start_real_clock`], the running game) the time
    /// that really passed is added at each reading. Tests set it directly.
    pub clock_ms: u32,
    /// The microseconds past `clock_ms` (the counter's finer part).
    pub clock_us: u32,
    /// When the real clock is on: the instant the clock was last brought up to date.
    pub real_clock: Option<std::time::Instant>,
    /// The Windows kernel objects the platform imports hand out (events,
    /// semaphores, threads, `VirtualAlloc` regions; `units/platform.rs`).
    pub kernel: crate::units::platform::Kernel,
    /// The functions entered through [`Engine::call`] that haven't returned,
    /// innermost last: the nearest caller entered by address of whatever
    /// runs now (calls within one unit file are direct and not on it). A
    /// stop (a panic) leaves it as it was; whoever catches the stop
    /// truncates it.
    pub call_stack: Vec<u32>,
    /// Runs when a call reaches an untranslated address, before the stop
    /// ([`OpenHandler`]). The host uses it to record which addresses a run
    /// of the game's code needs, and from where.
    pub on_open: Option<OpenHandler>,
    /// When `Some`, every call that returns through [`Engine::call`] is
    /// recorded as (address, argument words, `EAX`), in the order they
    /// return, keeping the last [`TRACE_KEEP`]: a diagnostic of where a run
    /// went wrong before it stopped (the host's `NV_MOVE_TRACE`).
    pub trace: Option<std::collections::VecDeque<(u32, Vec<u32>, u32)>>,
    /// How many returned calls the trace keeps ([`TRACE_KEEP`] unless the host says otherwise).
    pub trace_keep: usize,
    /// When set, how many more calls through [`Engine::call`] may run before the run stops with
    /// "call budget spent" (a diagnostic for a run that loops for ever; the host's
    /// `NV_MOVE_CALL_BUDGET`).
    pub call_budget: Option<u64>,
    /// When `Some`, how many times each (caller, address) pair was called through
    /// [`Engine::call`], keyed `caller << 32 | address` (the caller is the innermost function on
    /// the call stack, 0 at a stage's top): where a long run spends its calls (the host's
    /// `NV_MOVE_CALL_PROFILE`).
    pub call_counts: Option<AddrMap<u64, u64>>,
    /// The thread host's time slice: calls left in the running thread's turn, the length a
    /// turn starts with, and what runs at each call once a turn is used up, given the called
    /// address (`thread_host::preempt`: game threads run beside each other on Windows, so a
    /// thread that never waits must not keep the others from running). The hook starts the
    /// next turn itself (sets `slice_left` again) when it switches; until then it is asked
    /// again at every call. `None`: no slicing (tests, start-up without the host).
    pub slice_left: u64,
    pub slice_len: u64,
    pub slice_hook: Option<fn(&mut Engine, u32)>,
    /// Diagnostic only (not exe behaviour): runs at every call through [`Engine::call`] that
    /// reaches a translated function, given the called address, once as it is entered (`false`,
    /// before it goes on the call stack) and once as it returns (`true`, after it came off).
    /// Lets a check look at memory between any two calls (`NV_HKMEM_CHECK=calls`, engine-havok's
    /// `hkthreadmemory.rs`). `None` in normal runs.
    pub call_hook: Option<fn(&mut Engine, u32, bool)>,
    /// Diagnostic only (not exe behaviour): runs as a heap block is freed through the game's
    /// `MemoryManager::Deallocate` (`00aa4060`) or the CRT's `free`, given the block (not 0),
    /// before it is freed. Lets a check see who frees what (`NV_SPEECH_CHECK`, engine-fallout's
    /// `bsfacegenkeyframe.rs`). `None` in normal runs.
    pub free_hook: Option<fn(&mut Engine, u32)>,
    /// The import slots [`Engine::register_imports`] gave a platform function: the calls at
    /// which the thread host's default scheme may end a used-up turn
    /// (`thread_host::SwitchAt::Platform`, `thread_host::request_wake`).
    pub platform_calls: AddrSet<u32>,
    /// The nv-trace record of the listed functions' calls (`NV_TRACE_OUT`, `NV_TRACE_LIST`;
    /// [`crate::call_trace`]); `None` in normal runs.
    pub call_trace: Option<Box<crate::call_trace::CallTrace>>,
    /// The live `with_stack` blocks, innermost last, kept while [`Engine::call_trace`] is on (its
    /// stack records say which words point into them).
    pub stack_frames: Vec<(u32, u32)>,
    /// Each game thread's stack for its `with_stack` locals, by its index in the thread host's
    /// list: the region's lowest address and the current top (frames go down from the region's
    /// end); made at the thread's first local ([`Engine::with_stack`]).
    guest_stacks: Vec<Option<(u32, u32)>>,
}

/// The bytes of each game thread's stack for `with_stack` locals.
pub const GUEST_STACK_SIZE: u32 = 1 << 20;
/// The bytes kept free at the top of each game thread's stack, below its end: they stand for the
/// frames the exe has above a thread's first translated local (its start routine's, the C
/// runtime's), so a store past the outermost local stays on the stack instead of reaching the
/// heap block allocated after it.
pub const STACK_TOP_RESERVE: u32 = 0x1000;
/// The bytes left free above every `with_stack` local on a game thread's stack, so a store a little
/// past a local (a local translated smaller than the exe's, which in the exe lands among the
/// frame's other locals) stays on the stack instead of reaching the caller's locals.
pub const FRAME_GAP: u32 = 0x40;

/// How many returned calls [`Engine::trace`] keeps.
pub const TRACE_KEEP: usize = 4000;

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    /// Empty memory, every translated unit registered: this crate's
    /// (`units::crt`, `units::platform`) and those of the engine crates the
    /// program assembles ([`crate::assemble!`]): in the `engine` crate every
    /// part crate's, so every translated function; in a part crate's own
    /// unit tests that crate's and those of the crates below it.
    pub fn new() -> Self {
        let mut list = Vec::new();
        crate::units::funcs(&mut list);
        // SAFETY: `nv_engine_assembled_funcs` is the plain Rust function that
        // `assemble!` defines in the crate assembling the engine; it only
        // appends to `list`.
        unsafe { nv_engine_assembled_funcs(&mut list) };
        let mut funcs = AddrMap::with_capacity_and_hasher(list.len(), Default::default());
        for (addr, f) in list {
            if funcs.insert(addr, Callable::Abi(f)).is_some() {
                panic!("{addr:08x} is registered twice");
            }
        }
        // A new main thread: its block's control word reads as the default
        // until set, and the FPU register the x87 helpers read starts there
        // too (a test that ran earlier on this host thread may have left
        // another word in it).
        crate::x87::set_control_word(crate::x87::DEFAULT_CONTROL_WORD);
        // `NV_PROFILE` (a diagnostic): read once, before any call.
        crate::profile::init_from_env();
        let mut mem = Mem::new();
        // `NV_BLOCK_CHECK=free|store[,freed][,log]` (a diagnostic): guards after every heap block
        // and stack local, checked when the block is freed or at every store
        // ([`Mem::enable_block_check`]); `freed` also holds freed blocks back and reports stores
        // into them ([`Mem::enable_freed_check`]). A finding stops the run unless `,log`.
        if let Some((store, stop, freed)) = block_check_from_env() {
            mem.enable_block_check(store, stop);
            if freed {
                mem.enable_freed_check(stop);
            }
        }
        // The C runtime words its string and memory routines read first, as the image
        // holds them before the CRT start-up: `__sse2_available` (`01270a6c`) and
        // `__locale_changed` (`0126fdf4`) lie in `.data` past its file contents, so they
        // start as 0. In a real run the start-up's CPU check sets `__sse2_available` from
        // the machine's CPUID answers; a fresh engine (the tests) therefore takes the
        // exe's non-SSE2 paths and the C locale.
        mem.map(CRT_SSE2_AVAILABLE, 4);
        mem.map(CRT_LOCALE_CHANGED, 4);
        // The word `__handle_qnan1`, `__except1`, `__87except` and `__cintrindisp1/2` read
        // before reporting a math error through `_umatherr` (`011c2b60`, `.data` inside its
        // file contents): the image holds 0x2694 and nothing in the exe writes it, so it
        // keeps that value for the whole run (nonzero: no report).
        mem.map(CRT_MATHERR_SKIP, 4);
        mem.write(CRT_MATHERR_SKIP, &CRT_MATHERR_SKIP_IMAGE.to_le_bytes());
        let mut engine = Engine {
            mem,
            funcs,
            call_cache: (0..CALL_CACHE_LEN)
                .map(Cached::empty)
                .collect::<Box<[Cached]>>()
                .try_into()
                .unwrap_or_else(|_| unreachable!("the call cache has CALL_CACHE_LEN entries")),
            call_log: None,
            tls: 0,
            clock_ms: 0,
            clock_us: 0,
            real_clock: None,
            kernel: Default::default(),
            call_stack: Vec::new(),
            on_open: None,
            trace: None,
            trace_keep: TRACE_KEEP,
            call_budget: None,
            call_counts: None,
            slice_left: 0,
            slice_len: 0,
            slice_hook: None,
            call_hook: None,
            free_hook: None,
            platform_calls: AddrSet::default(),
            call_trace: crate::call_trace::for_new_engine(),
            stack_frames: Vec::new(),
            guest_stacks: Vec::new(),
        };
        // `NV_WATCH` (a diagnostic, not exe behaviour): reports the writers of a range of memory.
        crate::watch_check::install(&mut engine);
        engine
    }

    /// [`Engine::new`] with the data sections of the player's FalloutNV.exe
    /// mapped ([`crate::exe::map_data_sections`]).
    pub fn with_exe(exe: &Path) -> Result<Self, String> {
        let mut e = Self::new();
        crate::exe::map_data_sections(&mut e.mem, exe)?;
        let file = std::fs::read(exe).map_err(|err| format!("{}: {err}", exe.display()))?;
        e.register_imports(&crate::exe::imports(&file)?);
        // The process starts in the executable's folder (where Steam starts
        // the game): `GetCurrentDirectoryA` reports it.
        if let Some(folder) = exe.parent() {
            let folder = folder.to_string_lossy().replace('/', "\\");
            e.kernel.current_directory = folder.into_bytes();
        }
        Ok(e)
    }

    /// Registers the platform versions of imported functions
    /// (`units::platform::imports`) at their import-slot addresses, so a
    /// translation calls `e.call(slot, ..)` as the game calls `[slot]`.
    /// As the Windows loader fills the import table, each slot's word (when
    /// mapped) becomes the address of the function behind it: here the slot's
    /// own address, so code that reads a slot and calls the word later (the
    /// CRT's `__mtinit` `00ed1327` keeps `TlsGetValue`, `TlsSetValue` and
    /// `TlsFree` that way) reaches the same stand-in, or stops at the slot when
    /// none is provided. Returns how many were registered.
    pub fn register_imports(&mut self, imports: &[crate::exe::Import]) -> usize {
        let table = crate::units::platform::imports();
        let mut n = 0;
        for i in imports {
            if self.mem.is_mapped(i.slot) {
                self.mem.set_u32(i.slot, i.slot);
            }
            if let Some((_, _, f)) = table
                .iter()
                .find(|(d, name, _)| d.eq_ignore_ascii_case(&i.dll) && *name == i.name)
            {
                self.register(i.slot, *f);
                self.platform_calls.insert(i.slot);
                crate::profile::name_import(i.slot, &i.dll, &i.name);
                n += 1;
            }
        }
        n
    }

    /// Address of the running thread's TLS block: what the game reaches as
    /// `*(*(FS:[0x2c]) + _tls_index * 4)`, so a field the game reads at
    /// `+0x2b4` of it is at `e.tls() + 0x2b4`. Each game thread has its own
    /// (the thread host, `crate::thread_host`, swaps them when it switches
    /// threads). Allocated on first use ([`Engine::new_tls_block`]).
    pub fn tls(&mut self) -> u32 {
        if self.tls == 0 {
            self.tls = self.new_tls_block();
        }
        self.tls
    }

    /// A new TLS block: 0x1000 bytes, zeroed, then the exe's `.tls` template
    /// when it is mapped, as Windows makes one for each thread.
    pub fn new_tls_block(&mut self) -> u32 {
        let block = self.mem.alloc(0x1000);
        if self.mem.is_mapped(0x0127_2000) {
            let template = self.mem.bytes(0x0127_2000, 0x2c5);
            self.mem.write(block, &template);
        }
        block
    }

    /// The running thread's TLS block as it is (0 before first use), and
    /// the replacement of it: the thread host's switch.
    pub(crate) fn swap_tls(&mut self, block: u32) -> u32 {
        std::mem::replace(&mut self.tls, block)
    }

    /// Turns the real clock on: from now each reading of the clock adds the time that really
    /// passed since the last one (the running game; tests leave it off). On Windows the clock
    /// starts at the host's own `GetTickCount` (milliseconds since the machine started), as the
    /// game sees it: code that takes a zero start stamp as long ago depends on it (the loading
    /// menu's `Update` 00789a90 checks the intro movie only within 3 s of `+0x5b4`, 0 at first).
    pub fn start_real_clock(&mut self) {
        #[cfg(windows)]
        {
            extern "system" {
                fn GetTickCount() -> u32;
            }
            // SAFETY: GetTickCount takes nothing and only reads the system's tick count.
            self.clock_ms = unsafe { GetTickCount() };
            self.clock_us = 0;
        }
        self.real_clock = Some(std::time::Instant::now());
    }

    /// Brings the clock up to date with the real time that passed (real clock on only). Only
    /// whole microseconds are counted, and the real-time mark moves on by exactly what was
    /// counted, so the part of a microsecond left over is counted at a later reading: moving the
    /// mark to `now` instead lost about half a microsecond at every reading, and with thousands
    /// of readings a second the game's clock fell behind the real time by a few percent (the
    /// sound, which follows that clock, then got gaps of silence to wait for it).
    fn catch_up_clock(&mut self) {
        if let Some(last) = self.real_clock {
            let now = std::time::Instant::now();
            let passed = now.duration_since(last).as_micros() as u64;
            self.real_clock = Some(last + std::time::Duration::from_micros(passed));
            self.advance_clock_us(passed);
        }
    }

    /// Moves the clock on by `us` microseconds (milliseconds wrap as a `DWORD` does).
    pub fn advance_clock_us(&mut self, us: u64) {
        let total = u64::from(self.clock_us) + us;
        self.clock_ms = self.clock_ms.wrapping_add((total / 1000) as u32);
        self.clock_us = (total % 1000) as u32;
    }

    /// Waits until the clock reads `until` (microseconds): with the real clock on, the host
    /// really sleeps until then, so the clock never runs ahead of the real time; with it off
    /// (the tests) the clock moves on to `until` at once. A time already passed returns at once.
    pub fn wait_clock_until_us(&mut self, until: u64) {
        let now = self.clock_now_us();
        if until <= now {
            return;
        }
        if self.real_clock.is_some() {
            let phase = crate::profile::set_phase(crate::profile::Phase::Sleep);
            std::thread::sleep(std::time::Duration::from_micros(until - now));
            crate::profile::set_phase(phase);
            self.catch_up_clock();
        } else {
            self.advance_clock_us(until - now);
        }
    }

    /// The clock in milliseconds now (what `GetTickCount` and `timeGetTime` answer).
    pub fn clock_now_ms(&mut self) -> u32 {
        self.catch_up_clock();
        self.clock_ms
    }

    /// The clock in microseconds now (what `QueryPerformanceCounter` answers at 1 MHz).
    pub fn clock_now_us(&mut self) -> u64 {
        self.catch_up_clock();
        u64::from(self.clock_ms) * 1000 + u64::from(self.clock_us)
    }

    /// Number of registered functions.
    pub fn translated_count(&self) -> usize {
        self.funcs.len()
    }

    /// Whether a call at `addr` has something to run: a registered function,
    /// or a row of the atexit-destructor table ([`crate::static_exit`]) whose
    /// destructor is translated.
    pub fn is_translated(&self, addr: u32) -> bool {
        self.funcs.contains_key(&addr) || crate::static_exit::runs(self, addr)
    }

    /// Registers (or replaces) the function at `addr`. Tests use this for
    /// doubles of callees that are not under test.
    /// Whether something (a translation, a platform function or a double) is registered at
    /// `addr`.
    pub fn is_registered(&self, addr: u32) -> bool {
        self.funcs.contains_key(&addr)
    }

    pub fn register(&mut self, addr: u32, f: AbiFn) {
        self.funcs.insert(addr, Callable::Abi(f));
        self.call_cache[cache_slot(addr)] = Cached::empty(cache_slot(addr));
    }

    /// Registers a test double that keeps state (counts calls, returns a
    /// sequence, records arguments). A double that calls itself again
    /// while running panics.
    pub fn register_double(
        &mut self,
        addr: u32,
        f: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        self.funcs
            .insert(addr, Callable::Double(Rc::new(RefCell::new(f))));
        self.call_cache[cache_slot(addr)] = Cached::empty(cache_slot(addr));
    }

    /// Calls the function at exe address `addr` with argument words in
    /// the uniform form ([`crate::abi`]). Panics if nobody has translated
    /// it yet: the translation that called it can only be checked once
    /// that callee exists (or a test supplies a double). An address with no
    /// registered function runs its row of the atexit-destructor table
    /// ([`crate::static_exit`]) if it has one.
    pub fn call(&mut self, addr: u32, args: &[u32]) -> Ret {
        self.dispatch(addr, args)
    }

    /// [`Engine::call`]'s body, also inlined into [`Engine::vcall`].
    ///
    /// The fast path: a plain function in the cache, no diagnostic on, the time slice not used
    /// up and room on the call stack. It counts the slice down, pushes the call stack, calls
    /// and pops, with the one call to the function in its body. The rest is out of line and
    /// does the same in the same order: a used-up slice in [`Engine::call_at_slice_end`] (the
    /// hook, then the call), a full call stack, a diagnostic or a cache miss in
    /// [`Engine::call_full`]. So the fast path keeps few registers across the call (on Windows
    /// every register a function uses anywhere is saved on entry).
    #[inline(always)]
    fn dispatch(&mut self, addr: u32, args: &[u32]) -> Ret {
        let entry = self.call_cache[cache_slot(addr)];
        if entry.addr == addr && !self.diagnosing() {
            // With the thread host's hook the slice counts down by one (`None`: used up, 0);
            // without it the slice stays as it is.
            let left = self
                .slice_left
                .checked_sub(u64::from(self.slice_hook.is_some()));
            match left {
                Some(left) if self.call_stack.len() < self.call_stack.capacity() => {
                    self.slice_left = left;
                    self.call_stack.push(addr);
                    let ret = (entry.f)(self, args);
                    self.call_stack.pop();
                    return ret;
                }
                None => return self.call_at_slice_end(addr, args, entry.f),
                Some(_) => {}
            }
        }
        self.call_full(addr, args)
    }

    /// [`Engine::dispatch`]'s fast path when the slice is used up: the hook (which starts the
    /// next turn, or asks again at the next call), then the call.
    #[cold]
    #[inline(never)]
    fn call_at_slice_end(&mut self, addr: u32, args: &[u32], f: AbiFn) -> Ret {
        if let Some(hook) = self.slice_hook {
            hook(self, addr);
        }
        self.call_stack.push(addr);
        let ret = f(self, args);
        self.call_stack.pop();
        ret
    }

    /// Whether any per-call diagnostic is on (the call log, budget, profile, hook, nv-trace
    /// record, return trace, block check, `NV_PROFILE`): [`Engine::call`] then takes its full
    /// path.
    #[inline(always)]
    fn diagnosing(&self) -> bool {
        // The hook and the nv-trace record are read as their pointer words, volatile: the
        // compiler otherwise tests four of these fields with a dozen vector instructions where
        // four compares do.
        // SAFETY: `Option<fn>` and `Option<Box<_>>` are one pointer word, 0 for `None` (the
        // guaranteed null-pointer optimisation); each is read in place as that word.
        let (hook, traced) = unsafe {
            (
                std::ptr::read_volatile(std::ptr::addr_of!(self.call_hook).cast::<usize>()),
                std::ptr::read_volatile(std::ptr::addr_of!(self.call_trace).cast::<usize>()),
            )
        };
        self.call_log.is_some()
            || self.call_budget.is_some()
            || self.call_counts.is_some()
            || hook != 0
            || traced != 0
            || self.trace.is_some()
            || self.mem.block_check_on()
            || crate::profile::on()
    }

    /// [`Engine::call`] with every diagnostic, the slice's hook, the call stack's growth and
    /// the function table: what the fast path ([`Engine::dispatch`]) leaves out.
    #[cold]
    #[inline(never)]
    fn call_full(&mut self, addr: u32, args: &[u32]) -> Ret {
        if let Some(log) = &mut self.call_log {
            log.push((addr, args.to_vec()));
        }
        let slot = cache_slot(addr);
        let entry = self.call_cache[slot];
        if entry.addr == addr {
            return self.run(addr, args, Callable::Abi(entry.f));
        }
        if crate::profile::on() {
            crate::profile::cache_miss();
        }
        let Some(f) = self.funcs.get(&addr).cloned() else {
            if let Some(row) = crate::static_exit::row(addr) {
                self.call_stack.push(addr);
                let ret = crate::static_exit::run_row(self, row);
                self.call_stack.pop();
                return ret;
            }
            if let Some(handler) = self.on_open.clone() {
                if let Some(ret) = (handler.borrow_mut())(self, addr, args) {
                    return ret;
                }
            }
            panic!("open function {addr:08x}: not translated yet (FalloutNV.exe 1.4.0.525, docs/LEDGER.md)")
        };
        if let Callable::Abi(plain) = f {
            self.call_cache[slot] = Cached { addr, f: plain };
        }
        self.run(addr, args, f)
    }

    /// The rest of [`Engine::call`] once the callee is found: the budget, the profile, the
    /// call stack, the call itself and the trace.
    #[inline(always)]
    fn run(&mut self, addr: u32, args: &[u32], f: Callable) -> Ret {
        if let Some(hook) = self.slice_hook {
            if self.slice_left == 0 {
                hook(self, addr);
            } else {
                self.slice_left -= 1;
            }
        }
        if let Some(budget) = &mut self.call_budget {
            if *budget == 0 {
                panic!("call budget spent (at {addr:08x})");
            }
            *budget -= 1;
        }
        if let Some(counts) = &mut self.call_counts {
            let caller = self.call_stack.last().copied().unwrap_or(0);
            *counts
                .entry(u64::from(caller) << 32 | u64::from(addr))
                .or_default() += 1;
        }
        if let Some(hook) = self.call_hook {
            hook(self, addr, false);
        }
        let traced = match &self.call_trace {
            Some(t) if t.wants(addr) && t.admit(addr, args, &self.mem) => {
                let thread = crate::call_trace::thread_of(self);
                let seq = t.enter(thread, addr, args);
                let stack = crate::call_trace::stack_words(&self.stack_frames, args);
                if !stack.is_empty() {
                    t.stack(thread, seq, &stack);
                }
                t.dump(thread, seq, addr, args, &self.mem);
                Some(seq)
            }
            _ => None,
        };
        // The block check (`NV_BLOCK_CHECK`) names the function running at a finding.
        let checked = self
            .mem
            .block_check_on()
            .then(|| self.mem.set_check_fn(addr));
        // `NV_PROFILE` (a diagnostic): the sampler's shadow of the call stack.
        let profiled = crate::profile::on();
        if profiled {
            crate::profile::push(addr);
        }
        self.call_stack.push(addr);
        let ret = match f {
            Callable::Abi(f) => f(self, args),
            Callable::Double(d) => (d.borrow_mut())(self, args),
        };
        self.call_stack.pop();
        if profiled {
            crate::profile::pop();
        }
        if let Some(caller) = checked {
            self.mem.set_check_fn(caller);
        }
        if let (Some(seq), Some(t)) = (traced, &self.call_trace) {
            let thread = crate::call_trace::thread_of(self);
            t.leave(thread, addr, seq, &ret);
            t.rdump(thread, seq, addr, args, &self.mem);
        }
        if let Some(hook) = self.call_hook {
            hook(self, addr, true);
        }
        if let Some(trace) = &mut self.trace {
            // A full ring hands its oldest entry's word buffer to the newest: no allocation
            // per call once full.
            let mut words = if trace.len() >= self.trace_keep {
                trace.pop_front().map(|(_, w, _)| w).unwrap_or_default()
            } else {
                Vec::new()
            };
            words.clear();
            words.extend_from_slice(args);
            trace.push_back((addr, words, ret.eax));
        }
        ret
    }

    /// [`Engine::call`] with the result converted.
    pub fn call_as<R: RetVal>(&mut self, addr: u32, args: &[u32]) -> R {
        R::from_ret(self.call(addr, args))
    }

    /// Virtual call: reads the vtable pointer at `this + 0` and the
    /// function at byte offset `slot` in that table (as the game's
    /// `call dword ptr [eax + slot]`), then calls it with `this` first.
    pub fn vcall(&mut self, this: u32, slot: u32, args: &[u32]) -> Ret {
        // The vtable and its slot as `Mem::u32` reads them. A read that needs its slow path
        // (across a page, or unmapped: a stop) is made out of line, both reads again (the fast
        // reads have no effect), so this body keeps few registers across its calls.
        let target = match self.mem.u32_in_page(this) {
            Some(vtable) => self.mem.u32_in_page(vtable.wrapping_add(slot)),
            None => None,
        };
        let Some(target) = target else {
            return self.vcall_slow(this, slot, args);
        };
        self.vcall_to(target, this, args)
    }

    /// [`Engine::vcall`] when a read of the vtable or its slot needs [`Mem::u32`]'s slow path.
    #[cold]
    #[inline(never)]
    fn vcall_slow(&mut self, this: u32, slot: u32, args: &[u32]) -> Ret {
        let vtable = self.mem.u32(this);
        let target = self.mem.u32(vtable.wrapping_add(slot));
        self.vcall_to(target, this, args)
    }

    /// [`Engine::vcall`] once the target is read.
    #[inline(always)]
    fn vcall_to(&mut self, target: u32, this: u32, args: &[u32]) -> Ret {
        // `this` and the arguments on the stack (no allocation per call). The short lists, most
        // virtual calls, are a fixed-size array (a variable-length copy is a call to `memcpy`,
        // which cost more than the rest of the call: a15's benchmark) passed to the one inlined
        // [`Engine::dispatch`]; longer ones are out of line.
        let (words, n) = match *args {
            [] => ([this, 0, 0, 0, 0], 1),
            [a] => ([this, a, 0, 0, 0], 2),
            [a, b] => ([this, a, b, 0, 0], 3),
            [a, b, c] => ([this, a, b, c, 0], 4),
            [a, b, c, d] => ([this, a, b, c, d], 5),
            _ => return self.vcall_long(target, this, args),
        };
        self.dispatch(target, &words[..n])
    }

    /// [`Engine::vcall`] with more than four arguments, once the target is read.
    #[inline(never)]
    fn vcall_long(&mut self, target: u32, this: u32, args: &[u32]) -> Ret {
        const INLINE: usize = 15;
        if args.len() <= INLINE {
            let mut words = [0u32; INLINE + 1];
            words[0] = this;
            words[1..=args.len()].copy_from_slice(args);
            self.call(target, &words[..=args.len()])
        } else {
            self.vcall_longer(target, this, args)
        }
    }

    /// [`Engine::vcall`] of more than 15 words after `this` (rare), out of
    /// [`Engine::vcall_long`]'s body: on the stack too up to 63 of them (no allocation per
    /// call), else on the heap.
    #[cold]
    #[inline(never)]
    fn vcall_longer(&mut self, target: u32, this: u32, args: &[u32]) -> Ret {
        const LONG: usize = 63;
        if args.len() <= LONG {
            let mut words = [0u32; LONG + 1];
            words[0] = this;
            words[1..=args.len()].copy_from_slice(args);
            self.call(target, &words[..=args.len()])
        } else {
            let mut words = Vec::with_capacity(args.len() + 1);
            words.push(this);
            words.extend_from_slice(args);
            self.call(target, &words)
        }
    }

    /// Field read through a typed pointer.
    pub fn get<S, T: Scalar>(&self, p: Ptr<S>, f: Field<S, T>) -> T {
        T::load(&self.mem, p.0.wrapping_add(f.off))
    }

    /// Field write through a typed pointer.
    pub fn set<S, T: Scalar>(&mut self, p: Ptr<S>, f: Field<S, T>, v: T) {
        v.store(&mut self.mem, p.0.wrapping_add(f.off))
    }

    /// A global variable at its exe address (in `.data`/`.rdata`).
    pub fn global<T: Scalar>(&self, addr: u32) -> T {
        T::load(&self.mem, addr)
    }

    pub fn set_global<T: Scalar>(&mut self, addr: u32, v: T) {
        v.store(&mut self.mem, addr)
    }

    /// A zeroed heap block the size of `T` (tests and allocators).
    pub fn new_object<T: Layout>(&mut self) -> Ptr<T> {
        Ptr::new(self.mem.alloc(T::SIZE))
    }

    /// Maps zeroed memory at a fixed address (a global or a vtable in a
    /// test that runs without the executable).
    pub fn map(&mut self, addr: u32, len: u32) {
        self.mem.map(addr, len)
    }

    /// Writes a vtable (slot targets) at `addr`; for tests without the
    /// executable. With it mapped, the game's own vtables are in memory.
    pub fn put_vtable(&mut self, addr: u32, slots: &[u32]) {
        self.mem.map(addr, 4 * slots.len() as u32);
        for (i, s) in slots.iter().enumerate() {
            self.mem.set_u32(addr + 4 * i as u32, *s);
        }
    }

    /// Typed argument helper for [`Engine::call`]: `e.call(a, &words![..])`
    /// is usually written with the `args!` macro; this is the one-argument
    /// form.
    pub fn call1<A: Arg, R: RetVal>(&mut self, addr: u32, a: A) -> R {
        let mut w = crate::abi::Words::new();
        a.put_words(&mut w);
        self.call_as(addr, &w)
    }
}

impl Engine {
    /// A block of `size` zeroed bytes for the duration of `f`: a local the
    /// game keeps on its stack and passes by address (a scope guard, a
    /// `NiPointer` temporary, an out parameter). Given back when `f` returns.
    ///
    /// It is taken from the running game thread's own stack ([`GUEST_STACK_SIZE`] bytes, frames
    /// going down, [`FRAME_GAP`] bytes left free above each), as the exe keeps its locals on the
    /// thread's stack: a store a little past a local (up to [`FRAME_GAP`] bytes, or
    /// [`STACK_TOP_RESERVE`] past the outermost one) or through a pointer kept to an ended one
    /// stays on that stack, as in the exe, instead of reaching a heap object. With the block
    /// check on the local is a guarded heap block instead, so the check can name those stores.
    ///
    /// A stop (a panic) unwinding through `f` leaves the stack's top where it was; a caller that
    /// catches stops and goes on with the engine puts it back with [`Engine::guest_stack_mark`]
    /// and [`Engine::guest_stack_restore`].
    pub fn with_stack<R>(&mut self, size: u32, f: impl FnOnce(&mut Engine, Ptr) -> R) -> R {
        // It goes back the way it came, even if the check is switched on meanwhile.
        let checked = self.mem.block_check_on();
        let thread = self.kernel.threads.current;
        let (block, top) = if checked {
            (self.mem.alloc(size), 0)
        } else {
            self.push_frame(thread, size)
        };
        let traced = self.call_trace.is_some();
        if traced {
            self.stack_frames.push((block, size));
        }
        let r = f(self, Ptr::new(block));
        if traced {
            self.stack_frames.pop();
        }
        if checked {
            self.mem.free(block);
        } else if let Some(Some(stack)) = self.guest_stacks.get_mut(thread) {
            stack.1 = top;
        }
        r
    }

    /// The address the next [`Engine::with_stack`] of `size` bytes on the running thread takes
    /// (block check off), for tests that need to know a local's address beforehand.
    pub fn next_stack_local(&mut self, size: u32) -> u32 {
        let thread = self.kernel.threads.current;
        let (block, top) = self.push_frame(thread, size);
        if let Some(Some(stack)) = self.guest_stacks.get_mut(thread) {
            stack.1 = top;
        }
        block
    }

    /// The running thread's stack top, for a caller that catches stops: after a stop unwound
    /// through `with_stack` frames, [`Engine::guest_stack_restore`] with this mark gives them
    /// back (the exe's frames go with ESP however a frame ends).
    pub fn guest_stack_mark(&self) -> (usize, Option<u32>) {
        let thread = self.kernel.threads.current;
        let top = self
            .guest_stacks
            .get(thread)
            .copied()
            .flatten()
            .map(|(_, top)| top);
        (thread, top)
    }

    /// Puts a thread's stack top back where [`Engine::guest_stack_mark`] found it; a stack made
    /// since the mark is empty again.
    pub fn guest_stack_restore(&mut self, (thread, top): (usize, Option<u32>)) {
        if let Some(Some(stack)) = self.guest_stacks.get_mut(thread) {
            stack.1 = top.unwrap_or(stack.0 + GUEST_STACK_SIZE - STACK_TOP_RESERVE);
        }
    }

    /// Takes a local of `size` bytes (rounded up to 8, zeroed) from game thread `thread`'s stack,
    /// making the stack at its first local; answers the local and the top to put back.
    #[inline]
    fn push_frame(&mut self, thread: usize, size: u32) -> (u32, u32) {
        if self.guest_stacks.len() <= thread {
            self.guest_stacks.resize(thread + 1, None);
        }
        let (base, top) = match self.guest_stacks[thread] {
            Some(stack) => stack,
            None => {
                let base = self.mem.alloc(GUEST_STACK_SIZE);
                (base, base + GUEST_STACK_SIZE - STACK_TOP_RESERVE)
            }
        };
        // In 64 bits, so a size near 4 GB stops here before anything is written.
        let bytes = u64::from(size.max(1)).div_ceil(8) * 8;
        let need = bytes + u64::from(FRAME_GAP);
        assert!(
            u64::from(top - base) >= need,
            "game thread {thread}: its stack for locals ({GUEST_STACK_SIZE:#x} bytes) is full"
        );
        let block = top - need as u32;
        self.guest_stacks[thread] = Some((base, block));
        self.mem.zero(block, bytes as u32);
        (block, top)
    }
}

/// `NV_BLOCK_CHECK`, read once: `free` or `store` (whether every store is checked), with
/// `,freed` for the freed-block check and `,log` to report without stopping. `None` when unset.
fn block_check_from_env() -> Option<(bool, bool, bool)> {
    static CHECK: std::sync::OnceLock<Option<(bool, bool, bool)>> = std::sync::OnceLock::new();
    *CHECK.get_or_init(|| {
        let v = std::env::var("NV_BLOCK_CHECK").ok()?;
        let store = v.split(',').any(|p| p.trim() == "store");
        let stop = !v.split(',').any(|p| p.trim() == "log");
        let freed = v.split(',').any(|p| p.trim() == "freed");
        Some((store, stop, freed))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args;

    /// The block check names the translated function that writes past a `with_stack` local
    /// (the 00553bb0 / 004e75d0 case: a 0x20-byte local, nine words written).
    #[test]
    fn the_block_check_names_who_overruns_a_stack_local() {
        let mut e = Engine::new();
        e.mem.enable_block_check(true, false);
        e.register(0x004e_75d0, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[0] + 4 * i, i);
            }
            Ret::default()
        });
        e.register(0x0055_3bb0, |e, _| {
            e.with_stack(0x20, |e, out| e.call(0x004e_75d0, &[out.addr()]));
            Ret::default()
        });
        e.call(0x0055_3bb0, &[]);
        let r = e.mem.take_block_reports();
        assert_eq!(r.len(), 2, "{r:?}");
        assert!(r[0].starts_with("004e75d0 wrote 4 byte(s)"), "{}", r[0]);
        assert!(r[0].contains("allocated while 00553bb0 ran"), "{}", r[0]);
        assert!(
            r[1].contains("found when it was freed, while 00553bb0 ran"),
            "{}",
            r[1]
        );
    }

    /// The freed-block check names who writes through a pointer kept to an ended stack frame,
    /// which without it would land in whatever block took the frame's place.
    #[test]
    fn the_freed_check_names_a_store_through_a_stale_stack_pointer() {
        let mut e = Engine::new();
        e.mem.enable_freed_check(false);
        e.map(0x0100_0000, 0x1000);
        // 00401000 keeps the address of its 8-byte local in a global; 00402000 writes through it
        // after the frame ended.
        e.register(0x0040_1000, |e, _| {
            e.with_stack(8, |e, local| e.set_global(0x0100_0000, local.addr()));
            Ret::default()
        });
        e.register(0x0040_2000, |e, _| {
            let stale = e.global::<u32>(0x0100_0000);
            e.mem.set_u32(stale, 0);
            Ret::default()
        });
        e.call(0x0040_1000, &[]);
        // A new block of the same size is not the freed frame, and the frame reads as freed.
        let fresh = e.mem.alloc(8);
        assert_ne!(fresh, e.global::<u32>(0x0100_0000));
        let stale = e.global::<u32>(0x0100_0000);
        assert_eq!(e.mem.u32(stale), crate::mem::freed_word(stale));
        e.call(0x0040_2000, &[]);
        let r = e.mem.take_block_reports();
        assert_eq!(r.len(), 1, "{r:?}");
        assert!(r[0].starts_with("00402000 wrote 4 byte(s)"), "{}", r[0]);
        assert!(r[0].contains("allocated while 00401000 ran"), "{}", r[0]);
        assert!(
            r[0].contains("after it was freed while 00401000 ran"),
            "{}",
            r[0]
        );
    }

    /// Readings closer together than a microsecond still add up to the real time that passed
    /// (each one used to drop its part of a microsecond, so back-to-back readings stopped the
    /// clock).
    #[test]
    fn the_real_clock_keeps_up_with_back_to_back_readings() {
        use std::time::{Duration, Instant};
        let mut e = Engine::new();
        e.start_real_clock();
        // The real time between the two readings lies between what passed from just after the
        // first to just before the last and what passed from just before the first to just
        // after the last (a microsecond of rounding at each reading either way).
        let t0 = Instant::now();
        let start = e.clock_now_us();
        let t1 = Instant::now();
        while t1.elapsed() < Duration::from_millis(20) {
            e.clock_now_us();
        }
        let t2 = Instant::now();
        let end = e.clock_now_us();
        let t3 = Instant::now();
        let counted = end - start;
        let least = (t2 - t1).as_micros() as u64;
        let most = (t3 - t0).as_micros() as u64;
        assert!(
            counted + 2 >= least,
            "counted {counted} us, at least {least}"
        );
        assert!(counted <= most + 2, "counted {counted} us, at most {most}");
    }

    /// `with_stack` locals come from the running thread's own stack: nested ones go down with
    /// [`FRAME_GAP`] bytes above each, a store past the outermost local stays on the stack (the
    /// heap block allocated right after the stack is not touched), and an ended local's place is
    /// taken again, zeroed.
    #[test]
    fn locals_come_from_the_threads_stack() {
        let mut e = Engine::new();
        // The stack is made at the first local; the next heap block sits right at its end.
        let first = e.next_stack_local(0x10);
        let heap = e.mem.alloc(0x10);
        assert_eq!(heap, first + 0x10 + FRAME_GAP + STACK_TOP_RESERVE);
        e.mem.write(heap, &[0x11; 0x10]);
        let (outer, inner) = e.with_stack(0x10, |e, outer| {
            let past = 0x10 + FRAME_GAP + STACK_TOP_RESERVE;
            e.mem.write(outer.addr(), &vec![0xaa; past as usize]);
            let inner = e.with_stack(8, |_, inner| inner.addr());
            (outer.addr(), inner)
        });
        assert_eq!(outer, first);
        assert_eq!(inner, outer - 8 - FRAME_GAP);
        assert_eq!(e.mem.bytes(heap, 0x10), vec![0x11; 0x10]);
        assert_eq!(e.next_stack_local(0x10), outer);
        let again = e.with_stack(0x10, |e, local| {
            assert_eq!(e.mem.bytes(local.addr(), 0x10), vec![0; 0x10]);
            local.addr()
        });
        assert_eq!(again, outer);
    }

    /// A stop unwinding through locals leaves the stack's top lowered; the catcher's mark puts
    /// it back, so the next local is where it was before the stop.
    #[test]
    fn a_caught_stop_gives_its_locals_back_through_the_mark() {
        let mut e = Engine::new();
        let first = e.next_stack_local(0x20);
        let mark = e.guest_stack_mark();
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.with_stack(0x20, |e, _| {
                e.with_stack(0x20, |_, _| -> u32 { panic!("stop") })
            })
        }));
        assert!(stopped.is_err());
        assert_ne!(e.next_stack_local(0x20), first);
        e.guest_stack_restore(mark);
        assert_eq!(e.next_stack_local(0x20), first);
    }

    /// A pointer read from a freed object leads nowhere, and the read through it names the object.
    #[test]
    #[should_panic(expected = "a pointer read from the freed block")]
    fn the_freed_check_names_the_block_a_stale_pointer_came_from() {
        let mut e = Engine::new();
        e.mem.enable_freed_check(false);
        let object = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(object + 4, other);
        // The game's deallocator gives the calls it was reached through (outermost first).
        e.mem.note_path(&[0x0040_1000, 0x0040_2000]);
        e.mem.free(object);
        let stale = e.mem.u32(object + 4);
        let message = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.mem.u32(stale + 0x10);
        }))
        .err()
        .and_then(|p| p.downcast_ref::<String>().cloned())
        .unwrap_or_default();
        assert!(message.contains("(in 00402000 < 00401000)"), "{message}");
        panic!("{message}");
    }

    /// Not a check: the host's costs per operation, for the frame-rate work (`cargo test
    /// --release -p engine-core --lib cost_of -- --ignored --nocapture`).
    #[test]
    #[ignore]
    fn cost_of_calls_and_memory() {
        fn leaf(e: &mut Engine, a: &[u32]) -> Ret {
            Ret {
                eax: e.mem.u32(a[0]).wrapping_add(a[1]),
                ..Ret::default()
            }
        }
        let mut e = Engine::new();
        let block = e.mem.alloc(0x40_0000);
        e.register(0x0040_1000, leaf);
        // Prime the cache.
        e.call(0x0040_1000, &[block, 1]);
        let n = 20_000_000u32;
        let t = std::time::Instant::now();
        let mut sum = 0u32;
        for i in 0..n {
            sum = sum.wrapping_add(e.call(0x0040_1000, &[block + (i & 0xffc), i]).eax);
        }
        let call_ns = t.elapsed().as_nanos() as f64 / f64::from(n);
        let t = std::time::Instant::now();
        for i in 0..n {
            sum = sum.wrapping_add(leaf(&mut e, &[block + (i & 0xffc), i]).eax);
        }
        let direct_ns = t.elapsed().as_nanos() as f64 / f64::from(n);
        // Loads spread over 4 MB (a working set like a frame's), a fixed pseudo-random walk.
        let t = std::time::Instant::now();
        let mut x = 12345u32;
        for _ in 0..n {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            sum = sum.wrapping_add(e.mem.u32(block + ((x >> 8) & 0x3f_fffc)));
        }
        let load_ns = t.elapsed().as_nanos() as f64 / f64::from(n);
        let t = std::time::Instant::now();
        for i in 0..n {
            e.mem.set_u32(block + ((i * 4) & 0x3f_fffc), i);
        }
        let store_ns = t.elapsed().as_nanos() as f64 / f64::from(n);
        let t = std::time::Instant::now();
        for i in 0..n {
            e.with_stack(0x20, |e, p| e.mem.set_u32(p.addr(), i));
        }
        let frame_ns = t.elapsed().as_nanos() as f64 / f64::from(n);
        let t = std::time::Instant::now();
        let mut f = 1.0f64;
        for i in 0..n {
            f = crate::x87::x87_mul(f, 1.000_001 + f64::from(i & 1) * 1e-7);
        }
        let x87_ns = t.elapsed().as_nanos() as f64 / f64::from(n);
        eprintln!(
            "cost: call {call_ns:.1} ns (direct {direct_ns:.1}), random load {load_ns:.1} ns, \
             store {store_ns:.1} ns, with_stack {frame_ns:.1} ns, x87_mul {x87_ns:.1} ns ({sum} {f})"
        );
    }

    /// Not a check: the call path's cost as the translated crates use it (`cargo test --release
    /// -p engine-core --lib cost_of_call_shapes -- --ignored --nocapture`), ns per call, the
    /// best of five runs. [`Engine::call`] and [`Engine::vcall`] are reached through function
    /// pointers the optimiser cannot see through, as from another crate (no inlining into the
    /// loop), and the callees are `entry!` functions with the thread host's time slice on, as in
    /// the running game. `NV_BENCH_CALL_COUNTS=FILE` (lines `address<TAB>count[<TAB>..]`, as
    /// `executed-withcalls.tsv`) adds a run over those addresses, each called as often as listed.
    #[test]
    #[ignore]
    fn cost_of_call_shapes() {
        type CallFn = fn(&mut Engine, u32, &[u32]) -> Ret;
        type VcallFn = fn(&mut Engine, u32, u32, &[u32]) -> Ret;
        const GETTER: u32 = 0x0055_9450;
        const THREE: u32 = 0x0044_de40;
        const NESTED: u32 = 0x0044_ddc0;
        fn call() -> CallFn {
            std::hint::black_box(Engine::call as CallFn)
        }
        fn vcall() -> VcallFn {
            std::hint::black_box(Engine::vcall as VcallFn)
        }
        fn three(e: &mut Engine, this: Ptr, n: u32, x: f32) -> u32 {
            e.mem.u32(this.addr() + 4).wrapping_add(n) ^ x.to_bits()
        }
        fn nested(e: &mut Engine, this: Ptr) -> u32 {
            call()(e, GETTER, &args![this]).u32().wrapping_add(1)
        }
        fn slice(e: &mut Engine, _: u32) {
            e.slice_left = e.slice_len;
        }
        fn best(n: u32, mut run: impl FnMut() -> u32) -> f64 {
            let mut best = f64::MAX;
            let mut sum = 0u32;
            for _ in 0..5 {
                let start = std::time::Instant::now();
                sum = sum.wrapping_add(run());
                best = best.min(start.elapsed().as_secs_f64() * 1e9 / f64::from(n));
            }
            std::hint::black_box(sum);
            best
        }
        let mut e = Engine::new();
        e.register(
            GETTER,
            crate::entry!(0x0055_9450, field_getter(Ptr) -> u32).1,
        );
        e.register(
            THREE,
            crate::entry!(0x0044_de40, three(Ptr, u32, f32) -> u32).1,
        );
        e.register(NESTED, crate::entry!(0x0044_ddc0, nested(Ptr) -> u32).1);
        e.slice_len = 200_000;
        e.slice_left = 200_000;
        e.slice_hook = Some(slice);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 1);
        let vtable = e.mem.alloc(16);
        e.mem.set_u32(vtable + 8, GETTER);
        e.mem.set_u32(vtable + 12, THREE);
        e.mem.set_u32(this, vtable);
        let n = 20_000_000u32;
        let words = best(n, || {
            let (mut s, p) = (0u32, std::hint::black_box(this));
            for _ in 0..n {
                s = s.wrapping_add(call()(&mut e, GETTER, &[p]).eax);
            }
            s
        });
        let built = best(n, || {
            let (mut s, p) = (0u32, std::hint::black_box(this));
            for _ in 0..n {
                s = s.wrapping_add(call()(&mut e, GETTER, &args![p]).eax);
            }
            s
        });
        let three_words = best(n, || {
            let (mut s, p) = (0u32, Ptr::<()>::new(std::hint::black_box(this)));
            for i in 0..n {
                s = s.wrapping_add(call()(&mut e, THREE, &args![p, i, 1.5f32]).eax);
            }
            s
        });
        let virtual0 = best(n, || {
            let (mut s, p) = (0u32, std::hint::black_box(this));
            for _ in 0..n {
                s = s.wrapping_add(vcall()(&mut e, p, 8, &[]).eax);
            }
            s
        });
        let virtual2 = best(n, || {
            let (mut s, p) = (0u32, std::hint::black_box(this));
            for i in 0..n {
                s = s.wrapping_add(vcall()(&mut e, p, 12, &args![i, 1.5f32]).eax);
            }
            s
        });
        let two_deep = best(n, || {
            let (mut s, p) = (0u32, std::hint::black_box(this));
            for _ in 0..n {
                s = s.wrapping_add(call()(&mut e, NESTED, &args![p]).eax);
            }
            s
        });
        eprintln!(
            "call shapes: &[w] {words:.2} ns, args![w] {built:.2} ns, args![p, u, f] {three_words:.2} ns, \
             vcall() {virtual0:.2} ns, vcall(u, f) {virtual2:.2} ns, nested (2 calls) {two_deep:.2} ns"
        );
        let Ok(path) = std::env::var("NV_BENCH_CALL_COUNTS") else {
            return;
        };
        let text = std::fs::read_to_string(&path).expect("NV_BENCH_CALL_COUNTS");
        let mut funcs = Vec::new();
        let mut total = 0u64;
        for line in text.lines() {
            let mut cols = line.split('\t');
            let (Some(a), Some(c)) = (cols.next(), cols.next()) else {
                continue;
            };
            let (Ok(a), Ok(c)) = (u32::from_str_radix(a, 16), c.parse::<u64>()) else {
                continue;
            };
            if c > 0 && !e.is_registered(a) {
                e.register(a, crate::entry!(0x0055_9450, field_getter(Ptr) -> u32).1);
                total += c;
                funcs.push((a, total));
            }
        }
        let mut x = 0x2545_f491_4f6c_dd1du64;
        let order: Vec<u32> = (0..4_000_000)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                let pick = x % total;
                funcs[funcs.partition_point(|&(_, end)| end <= pick)].0
            })
            .collect();
        let spread = best(order.len() as u32, || {
            let (mut s, p) = (0u32, std::hint::black_box(this));
            for &a in &order {
                s = s.wrapping_add(call()(&mut e, a, &args![p]).eax);
            }
            s
        });
        eprintln!(
            "call shapes: {} functions by their counts: {spread:.2} ns a call",
            funcs.len()
        );
    }

    /// No address matches an empty entry of the call cache: each holds an address that maps to
    /// another slot.
    #[test]
    fn empty_cache_entries_match_no_address() {
        for slot in 0..CALL_CACHE_LEN {
            assert_ne!(cache_slot(Cached::empty(slot).addr), slot, "slot {slot}");
        }
        let e = Engine::new();
        assert!(e
            .call_cache
            .iter()
            .enumerate()
            .all(|(slot, c)| cache_slot(c.addr) != slot));
    }

    /// The fast path hands slow cases to the full path: a used-up slice (the hook runs, then the
    /// call), a full call stack (it grows), a cache miss (the table answers and fills the
    /// cache). Each call returns the same and leaves the stack as it found it.
    #[test]
    fn the_fast_path_hands_slow_cases_to_the_full_path() {
        fn hook(e: &mut Engine, addr: u32) {
            e.mem.set_u32(0x2000, addr);
            e.slice_left = e.slice_len;
        }
        let mut e = Engine::new();
        e.mem.map(0x2000, 4);
        e.register(0x0040_6000, |e, a| Ret {
            eax: a[0] + e.call_stack.len() as u32,
            ..Ret::default()
        });
        // A miss, then a hit.
        assert_eq!(e.call(0x0040_6000, &[10]).eax, 11);
        assert_eq!(e.call(0x0040_6000, &[10]).eax, 11);
        // A full stack: the call grows it.
        e.call_stack.shrink_to_fit();
        assert_eq!(e.call_stack.capacity(), 0);
        assert_eq!(e.call(0x0040_6000, &[20]).eax, 21);
        assert!(e.call_stack.is_empty());
        // A used-up slice: the hook runs first and starts the next turn.
        e.slice_hook = Some(hook);
        e.slice_len = 2;
        e.slice_left = 0;
        assert_eq!(e.call(0x0040_6000, &[30]).eax, 31);
        assert_eq!((e.mem.u32(0x2000), e.slice_left), (0x0040_6000, 2));
        assert_eq!(e.call(0x0040_6000, &[40]).eax, 41);
        assert_eq!(e.slice_left, 1);
        // A stop inside leaves the callee on the stack (the catcher truncates it), on the full
        // path (the first call, a miss) and on the fast one.
        e.register(0x0040_6010, |_, _| panic!("stop"));
        for _ in 0..2 {
            let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call(0x0040_6010, &[]);
            }));
            assert!(stopped.is_err());
            assert_eq!(e.call_stack, [0x0040_6010]);
            e.call_stack.clear();
        }
    }

    /// Not a check: the cost of virtual calls by the length of their word lists, and of a call
    /// with 13 words built by `args!`, for the frame-rate work (`cargo test --release -p
    /// engine-core --lib cost_of -- --ignored --nocapture`).
    #[test]
    #[ignore]
    fn cost_of_virtual_calls() {
        fn leaf(e: &mut Engine, a: &[u32]) -> Ret {
            Ret {
                eax: e.mem.u32(a[0]).wrapping_add(a.len() as u32),
                ..Ret::default()
            }
        }
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(0x0040_1000, leaf);
        e.put_vtable(0x0100_0000, &[0x0040_1000]);
        e.mem.set_u32(this, 0x0100_0000);
        let n = 10_000_000u32;
        let mut sum = 0u32;
        let mut line = String::from("cost: vcall");
        for words in [0usize, 2, 4, 6, 16] {
            let args: Vec<u32> = (0..words as u32).collect();
            e.vcall(this, 0, &args);
            let t = std::time::Instant::now();
            for i in 0..n {
                let a = std::hint::black_box(&args[..]);
                sum = sum.wrapping_add(e.vcall(std::hint::black_box(this), 0, a).eax ^ i);
            }
            let ns = t.elapsed().as_nanos() as f64 / f64::from(n);
            line += &format!(" {words} words {ns:.1} ns,");
        }
        let t = std::time::Instant::now();
        for i in 0..n / 10 {
            let x = std::hint::black_box(i);
            sum = sum.wrapping_add(
                e.call(
                    0x0040_1000,
                    &args![this, x, x, x, x, x, x, x, x, x, x, x, x],
                )
                .eax,
            );
        }
        let ns = t.elapsed().as_nanos() as f64 / f64::from(n / 10);
        eprintln!("{line} call with args! of 13 words {ns:.1} ns ({sum})");
    }

    /// Not a check: the host heap allocations of the call machinery, memory and locals per
    /// operation, for the frame-rate work (`cargo test --release -p engine-core --lib
    /// allocations_per -- --ignored --nocapture`).
    #[test]
    #[ignore]
    fn allocations_per_operation_of_the_core() {
        use crate::alloc_count::{per_run, row};
        fn leaf(e: &mut Engine, a: &[u32]) -> Ret {
            Ret {
                eax: e.mem.u32(a[0]).wrapping_add(a.len() as u32),
                ..Ret::default()
            }
        }
        let mut e = Engine::new();
        let block = e.mem.alloc(0x100);
        e.register(0x0040_1000, leaf);
        e.put_vtable(0x0100_0000, &[0x0040_1000]);
        e.mem.set_u32(block, 0x0100_0000);
        row(
            "Engine::call, 2 words",
            per_run(1000, || {
                e.call(0x0040_1000, &[block, 1]);
            }),
        );
        for n in [0usize, 4, 5, 15, 16, 20] {
            let args = vec![block; n];
            row(
                &format!("Engine::vcall, {n} words after this"),
                per_run(1000, || {
                    e.vcall(block, 0, &args);
                }),
            );
        }
        row(
            "Engine::call with args! of 13 words",
            per_run(1000, || {
                e.call(
                    0x0040_1000,
                    &args![
                        block, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32,
                        12u32
                    ],
                );
            }),
        );
        row(
            "Engine::call1",
            per_run(1000, || {
                e.call1::<u32, u32>(0x0040_1000, block);
            }),
        );
        row(
            "Engine::with_stack 0x20, a call inside",
            per_run(1000, || {
                e.with_stack(0x20, |e, p| {
                    e.mem.set_u32(p.addr(), block);
                    e.call(0x0040_1000, &[p.addr()]);
                });
            }),
        );
        row(
            "Mem::alloc + Mem::free, 0x40 bytes",
            per_run(1000, || {
                let a = e.mem.alloc(0x40);
                e.mem.free(a);
            }),
        );
        row(
            "Mem::alloc + Mem::free, 0x10000 bytes",
            per_run(1000, || {
                let a = e.mem.alloc(0x10000);
                e.mem.free(a);
            }),
        );
        let mut f = 1.0f64;
        row(
            "x87_mul, x87_add, x87_div, x87_sqrt, x87_round",
            per_run(1000, || {
                f = crate::x87::x87_mul(f, 1.000_001);
                f = crate::x87::x87_add(f, 0.5);
                f = crate::x87::x87_div(f, 1.5);
                f = crate::x87::x87_sqrt(f + 2.0);
                f = crate::x87::x87_round(f);
            }),
        );
        // The diagnostics cost what they cost; listed for reference.
        e.trace = Some(Default::default());
        row(
            "Engine::call with the return trace on (NV_MOVE_TRACE)",
            per_run(1000, || {
                e.call(0x0040_1000, &[block, 1]);
            }),
        );
        e.trace = None;
        std::hint::black_box(f);
    }

    #[test]
    fn doubles_keep_state() {
        let mut e = Engine::new();
        let mut n = 0u32;
        e.register_double(0x0040_1000, move |_, a| {
            n += a[0];
            Ret {
                eax: n,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x0040_1000, &args![2u32]).u32(), 2);
        assert_eq!(e.call(0x0040_1000, &args![3u32]).u32(), 5);
    }

    /// A one-line getter in the uniform form, as the game's field getters are translated.
    fn field_getter(e: &mut Engine, this: Ptr) -> u32 {
        e.mem.u32(this.addr() + 4)
    }

    /// A function registered or replaced after it was called answers from then on (the
    /// call cache follows the table).
    #[test]
    fn replacing_a_called_function_takes_effect() {
        let mut e = Engine::new();
        e.register(0x0040_2000, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0040_2000, &[]).u32(), 1);
        e.register(0x0040_2000, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0040_2000, &[]).u32(), 2);
        e.register_double(0x0040_2000, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0040_2000, &[]).u32(), 3);
        e.register(0x0040_2000, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0040_2000, &[]).u32(), 4);
    }

    /// The fast path (no diagnostic on) and the full path (a diagnostic on) count the time slice
    /// down, run the hook at its end and keep the call stack alike.
    #[test]
    fn the_fast_and_the_full_path_slice_and_stack_alike() {
        fn hook(e: &mut Engine, addr: u32) {
            let n = e.mem.u32(0x2000);
            e.mem.set_u32(0x2000 + 4 + 4 * n, addr);
            e.mem.set_u32(0x2000, n + 1);
            e.slice_left = e.slice_len;
        }
        let run = |full: bool| {
            let mut e = Engine::new();
            e.mem.map(0x2000, 0x100);
            e.register(0x0040_5000, |e, a| Ret {
                eax: e.call_stack.len() as u32 * 100 + a[0],
                ..Ret::default()
            });
            e.register(0x0040_5010, |e, a| e.call(0x0040_5000, &[a[0] + 1]));
            if full {
                e.call_budget = Some(u64::MAX);
            }
            e.slice_len = 3;
            e.slice_left = 3;
            e.slice_hook = Some(hook);
            let rets: Vec<u32> = (0..5).map(|i| e.call(0x0040_5010, &[i]).eax).collect();
            let hooks: Vec<u32> = (0..e.mem.u32(0x2000))
                .map(|i| e.mem.u32(0x2004 + 4 * i))
                .collect();
            assert!(e.call_stack.is_empty());
            (rets, hooks, e.slice_left)
        };
        let fast = run(false);
        assert_eq!(fast.0, [201, 202, 203, 204, 205]);
        assert_eq!(fast, run(true));
    }

    /// The trace keeps the last `trace_keep` returned calls with their words, oldest first.
    #[test]
    fn the_trace_keeps_the_last_calls() {
        let mut e = Engine::new();
        e.register(0x0040_3000, |_, a| Ret {
            eax: a.iter().sum(),
            ..Ret::default()
        });
        e.trace = Some(Default::default());
        e.trace_keep = 3;
        e.call(0x0040_3000, &[1, 2, 3]);
        e.call(0x0040_3000, &[4]);
        e.call(0x0040_3000, &[]);
        e.call(0x0040_3000, &[5, 6]);
        let trace: Vec<_> = e.trace.take().unwrap().into_iter().collect();
        assert_eq!(
            trace,
            vec![
                (0x0040_3000, vec![4], 4),
                (0x0040_3000, vec![], 0),
                (0x0040_3000, vec![5, 6], 11)
            ]
        );
    }

    /// Times calls of a registered one-line getter through [`Engine::call`] (the dispatch
    /// cost): `cargo test --release -p engine-core --lib -- --ignored --nocapture
    /// dispatch_speed`; `NV_BENCH_CALLS` sets how many (default 300M).
    #[test]
    #[ignore]
    fn dispatch_speed() {
        let n: u64 = std::env::var("NV_BENCH_CALLS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300_000_000);
        let mut e = Engine::new();
        let (addr, f) = crate::entry!(0x0055_9450, field_getter(Ptr) -> u32);
        e.register(addr, f);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 1);
        e.trace = Some(Default::default());
        e.call_budget = Some(u64::MAX);
        for (name, with_args) in [("&[this]", false), ("&args![this]", true)] {
            let start = std::time::Instant::now();
            let mut sum = 0u32;
            for _ in 0..n {
                let ret = if with_args {
                    e.call(addr, &args![std::hint::black_box(this)])
                } else {
                    e.call(addr, &[std::hint::black_box(this)])
                };
                sum = sum.wrapping_add(ret.eax);
            }
            let secs = start.elapsed().as_secs_f64();
            assert_eq!(sum, n as u32);
            println!(
                "{name}: {n} calls in {secs:.2} s: {:.1} M calls/s, {:.1} ns per call (trace and budget on)",
                n as f64 / secs / 1e6,
                secs * 1e9 / n as f64
            );
        }
    }

    /// Times calls spread over a working set of registered functions, as a frame of the game
    /// makes them (the cache and the table, not one hot getter), with the thread host's time slice
    /// on: `cargo test --release -p engine-core --lib -- --ignored --nocapture spread_dispatch`;
    /// `NV_BENCH_FUNCS` registered (default 48000), `NV_BENCH_HOT` called (default 8000).
    #[test]
    #[ignore]
    fn spread_dispatch_speed() {
        let env = |k: &str, d: usize| {
            std::env::var(k)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(d)
        };
        let (funcs, hot) = (env("NV_BENCH_FUNCS", 48_000), env("NV_BENCH_HOT", 8_000));
        let mut e = Engine::new();
        let (_, f) = crate::entry!(0x0055_9450, field_getter(Ptr) -> u32);
        let mut x = 0x2545_f491_4f6c_dd1du64;
        let mut next = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        let mut addrs = Vec::with_capacity(funcs);
        while addrs.len() < funcs {
            let a = (0x0040_1000 + (next() as u32) % 0x00c0_0000) & !0xf;
            if !e.is_registered(a) {
                e.register(a, f);
                addrs.push(a);
            }
        }
        let order: Vec<u32> = (0..2_000_000)
            .map(|_| addrs[(next() as usize) % hot])
            .collect();
        fn slice(e: &mut Engine, _: u32) {
            e.slice_left = e.slice_len;
        }
        e.slice_len = 200_000;
        e.slice_left = 200_000;
        e.slice_hook = Some(slice);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 1);
        let mut best = f64::MAX;
        for _ in 0..5 {
            let start = std::time::Instant::now();
            let mut sum = 0u32;
            for &a in &order {
                sum = sum.wrapping_add(e.call(a, &[std::hint::black_box(this)]).eax);
            }
            assert_eq!(sum, order.len() as u32);
            best = best.min(start.elapsed().as_secs_f64() * 1e9 / order.len() as f64);
        }
        println!("{funcs} functions, {hot} called: {best:.2} ns a call");
    }

    /// A virtual call passes `this` and then the arguments, for every list length (the short
    /// ones are built as fixed arrays, the longer ones on the stack or the heap).
    #[test]
    fn virtual_calls_pass_this_and_every_argument() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(0x0040_4000, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.put_vtable(0x0100_0000, &[0, 0, 0x0040_4000]);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0100_0000);
        for n in 0..20u32 {
            let args: Vec<u32> = (1..=n).collect();
            e.vcall(this, 8, &args);
            let mut want = vec![this];
            want.extend_from_slice(&args);
            assert_eq!(seen.borrow().last(), Some(&want));
        }
    }

    /// A virtual call whose vtable or slot read needs the memory's slow path (a word across a
    /// page boundary) reads and calls as [`Mem::u32`] does; one through unmapped memory stops
    /// with the read's own message.
    #[test]
    fn virtual_calls_read_across_pages_and_stop_on_unmapped_memory() {
        let mut e = Engine::new();
        e.register(0x0040_4100, |_, a| Ret {
            eax: a.iter().sum(),
            ..Ret::default()
        });
        e.map(0x0100_0000, 0x3000);
        // The slot word straddles the first page's end, `this`'s word the second's.
        let vtable = 0x0100_0ff6;
        e.mem.set_u32(vtable + 8, 0x0040_4100);
        let this = 0x0100_1ffe;
        e.mem.set_u32(this, vtable);
        let aligned = 0x0100_1100;
        e.mem.set_u32(aligned, vtable);
        for n in 0..7u32 {
            let args: Vec<u32> = (1..=n).collect();
            let sum = n * (n + 1) / 2;
            assert_eq!(e.vcall(this, 8, &args).eax, this + sum, "{n} arguments");
            assert_eq!(e.vcall(aligned, 8, &args).eax, aligned + sum);
        }
        let message = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.vcall(0x0300_0000, 0, &[]);
        }))
        .err()
        .and_then(|p| p.downcast_ref::<String>().cloned())
        .unwrap_or_default();
        assert_eq!(message, "read of unmapped memory at 03000000");
    }

    #[test]
    fn stack_blocks_are_freed() {
        let mut e = Engine::new();
        let p = e.with_stack(8, |e, p| {
            e.mem.set_u32(p.addr(), 1);
            p
        });
        assert_eq!(e.mem.block_size(p.addr()), None);
    }

    #[test]
    #[should_panic(expected = "open function 00abcdef")]
    fn untranslated_calls_name_the_address() {
        Engine::new().call(0x00ab_cdef, &[]);
    }

    /// The open-function handler sees the address, the arguments and the
    /// nearest caller entered by address; returning a value goes on, and
    /// the call stack is back to empty after the outer call.
    #[test]
    fn the_open_handler_sees_the_miss_and_its_caller() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.on_open = Some(Rc::new(RefCell::new(
            move |e: &mut Engine, addr: u32, args: &[u32]| {
                record
                    .borrow_mut()
                    .push((addr, args.to_vec(), e.call_stack.last().copied()));
                Some(Ret {
                    eax: 7,
                    ..Ret::default()
                })
            },
        )));
        e.register(0x00aa_0000, |e, a| e.call(0x00ab_cdef, &[a[0] + 1]));
        assert_eq!(e.call(0x00aa_0000, &[1]).u32(), 7);
        assert_eq!(
            *seen.borrow(),
            vec![(0x00ab_cdef, vec![2], Some(0x00aa_0000))]
        );
        assert!(e.call_stack.is_empty());
    }
}
