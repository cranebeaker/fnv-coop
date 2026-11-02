//! `NiSystem/run_00aa1de0` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::NiBinaryStream;

/// The word of the thread's TLS block (`*(*(FS:[0x2c]) + _tls_index * 4)`) at `+0x2b4`
/// that `fn_00aa20b0` sets to 0xf while it runs.
const TLS_MODE: u32 = 0x2b4;
/// The holder object pointer (`011f6118`) the allocator set-up `00aa2020` resolves.
const HOLDER: u32 = 0x011f_6118;
/// Byte `011f610d`: 1 when `fn_00aa20b0` built the holder itself.
const HOLDER_CREATED: u32 = 0x011f_610d;
/// Byte `011f610c`: set to 1 when `fn_00aa20b0` has run to its end.
const STARTUP_FINISHED: u32 = 0x011f_610c;
/// The function stored by `fn_00aa2060` (`011f6110`), run once by `fn_00aa20b0`.
const START_HOOK: u32 = 0x011f_6110;
/// The second word stored by `fn_00aa2060` (`011f6114`).
const START_HOOK_DATA: u32 = 0x011f_6114;
/// Number of entries registered with `fn_00aa2080` (`011f6108`).
const STARTUP_COUNT: u32 = 0x011f_6108;
/// The registered functions (`011f60c8`).
const STARTUP_FUNCTIONS: u32 = 0x011f_60c8;
/// The registered second words (`011f6088`).
const STARTUP_SECOND_WORDS: u32 = 0x011f_6088;
/// Byte `011f6229`: set once `fn_00aa3050` has begun.
const START_UP_DONE: u32 = 0x011f_6229;
/// The memory manager singleton (`011f6238`), the `this` of `00aa3e40`.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const ALLOCATE: u32 = 0x00aa_3e40;
/// `00aa2200` (`thiscall`, no stack word): constructs the 8-byte holder in the block.
const CONSTRUCT_HOLDER: u32 = 0x00aa_2200;
/// `00aa2020`: the allocator initialization.
const ALLOCATOR_INITIALIZE: u32 = 0x00aa_2020;
/// `00aa1920`: allocates 8 bytes and stores the result at `011f6078`.
const TIMER_INITIALIZE: u32 = 0x00aa_1920;
/// `00aa7eb0` (Ghidra name `OEI::IO::initialize`).
const OEI_IO_INITIALIZE: u32 = 0x00aa_7eb0;

/// Calls the stream's function pointer at `member` (`m_pfnRead` +8 or
/// `m_pfnWrite` +0xc) once as the game does:
/// `function(stream, buffer, 1, &count, 1)` with `count` a local set to 1 first.
/// Adds the byte count the function returns to the stream's position and
/// returns it.
fn transfer_one_byte(
    e: &mut Engine,
    stream: Ptr<NiBinaryStream>,
    member: u32,
    buffer: u32,
    count: Ptr,
) -> u32 {
    e.mem.set_u32(count.addr(), 1);
    let function = e.mem.u32(stream.addr().wrapping_add(member));
    let done = e
        .call(function, &args![stream, buffer, 1u32, count, 1u32])
        .u32();
    let position = e.get(stream, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        stream,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
    done
}

// Translated from 00aa1f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reads one line from a binary stream into `buffer`
/// (`thiscall`, `RET 8`: `stream`, `buffer`, `size`). Reads byte by byte
/// through the stream's read function; stops at a line feed (not stored),
/// skips carriage returns, stops when `size - 1` characters are stored or when
/// a read does not give exactly one byte, and terminates the text with 0.
/// Returns the number of bytes read from the stream (the characters, the
/// skipped carriage returns and the line feed). With a `size` below 2 it stores
/// only the terminator and reads nothing.
pub fn fn_00aa1f10(e: &mut Engine, stream: Ptr<NiBinaryStream>, buffer: Ptr, size: u32) -> i32 {
    if size <= 1 {
        e.mem.set_u8(buffer.addr(), 0);
        return 0;
    }
    e.with_stack(8, |e, locals| {
        let count = locals.byte_add(4);
        let mut total = 0u32;
        let mut stored = 0u32;
        loop {
            let done = transfer_one_byte(e, stream, 0x08, locals.addr(), count);
            total = total.wrapping_add(done);
            if done != 1 {
                break;
            }
            let c = e.mem.u8(locals.addr());
            if c == 0x0a {
                break;
            }
            if c != 0x0d {
                e.mem.set_u8(buffer.addr().wrapping_add(stored), c);
                stored += 1;
            }
            if stored + 1 >= size {
                break;
            }
        }
        e.mem.set_u8(buffer.addr().wrapping_add(stored), 0);
        total as i32
    })
}

// Translated from 00aa1fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: writes the characters of the C string `text` to a binary
/// stream, without the terminator (`thiscall`, `RET 4`). Writes byte by byte
/// through the stream's write function and stops at the first write that does not
/// give exactly one byte. Returns the number of bytes written.
pub fn fn_00aa1fd0(e: &mut Engine, stream: Ptr<NiBinaryStream>, text: Ptr) -> i32 {
    e.with_stack(4, |e, count| {
        let mut written = 0i32;
        let mut at = text.addr();
        while e.mem.u8(at) != 0 {
            let done = transfer_one_byte(e, stream, 0x0c, at, count);
            at = at.wrapping_add(1);
            if done != 1 {
                break;
            }
            written += 1;
        }
        written
    })
}

// Translated from 00aa2060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): stores `function` at `011f6110` and
/// `data` at `011f6114`. `fn_00aa20b0` calls the function stored at `011f6110`, without
/// arguments, right after the allocator set-up; `NiMainSDM`'s constructor `00a8a5f0`
/// stores its initializer `00a89cc0` here. Returns `function` (the exe leaves it in
/// `EAX`).
pub fn fn_00aa2060(e: &mut Engine, function: u32, data: u32) -> u32 {
    e.mem.set_u32(START_HOOK, function);
    e.mem.set_u32(START_HOOK_DATA, data);
    function
}

// Translated from 00aa2080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): adds `function` and `second` to the
/// two parallel tables at `011f60c8` and `011f6088` (entry number `011f6108`, which it
/// increments). When `fn_00aa20b0` has already finished (`011f610c` set) the exe jumps to
/// `function`, which then runs with this function's two argument words and its own
/// return address, so it is called with `function` and `second` and its result is
/// returned; otherwise it returns the new entry count (the exe's `EAX`).
pub fn fn_00aa2080(e: &mut Engine, function: u32, second: u32) -> u32 {
    let count = e.mem.u32(STARTUP_COUNT);
    e.mem.set_u32(
        STARTUP_FUNCTIONS.wrapping_add(count.wrapping_mul(4)),
        function,
    );
    e.mem.set_u32(
        STARTUP_SECOND_WORDS.wrapping_add(count.wrapping_mul(4)),
        second,
    );
    let count = count.wrapping_add(1);
    let finished = e.mem.u8(STARTUP_FINISHED);
    e.mem.set_u32(STARTUP_COUNT, count);
    if finished != 0 {
        return e.call(function, &args![function, second]).u32();
    }
    count
}

// Translated from 00aa3050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments): once only (`011f6229` set to 1
/// first), runs `00aa2020` (allocator initialization), `00aa1920` (builds an 8-byte
/// object and stores its address at `011f6078`), then jumps to `00aa7eb0` (Ghidra name
/// `OEI::IO::initialize`). Later calls only return.
pub fn fn_00aa3050(e: &mut Engine) {
    if e.mem.u8(START_UP_DONE) != 0 {
        return;
    }
    e.mem.set_u8(START_UP_DONE, 1);
    e.call(ALLOCATOR_INITIALIZE, &args![]);
    e.call(TIMER_INITIALIZE, &args![]);
    e.call(OEI_IO_INITIALIZE, &args![]);
}

// Translated from 00aa20b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the game's `main` runs it at `0086a9f8`; called `NiInit`
/// by the task, not confirmed by a symbol; `cdecl`, one word): the Gamebryo start-up.
/// Saves the word at `+0x2b4` of the thread's TLS block and sets it to 0xf for the
/// duration. Chooses the holder at `011f6118`: with `holder` null and no holder yet, it
/// allocates 8 bytes from the memory manager (`011f6238`, `00aa3e40`) and constructs
/// them with `00aa2200` (a failed allocation leaves null), marking `011f610d` as 1; when
/// the global already holds one it is kept, else `holder` is used, and `011f610d` is 0.
/// Then runs `fn_00aa3050`, the hook stored at `011f6110` (if any, no arguments), and every
/// registered function of `011f60c8` in order (the count at `011f6108` is read again after
/// each), restores the TLS word, and sets `011f610c` to 1.
pub fn fn_00aa20b0(e: &mut Engine, holder: u32) {
    let mode = e.tls().wrapping_add(TLS_MODE);
    let saved = e.mem.u32(mode);
    let existing = e.mem.u32(HOLDER);
    e.mem.set_u32(mode, 0xf);
    let chosen = if holder == 0 && existing == 0 {
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
        let built = if block == 0 {
            0
        } else {
            e.call(CONSTRUCT_HOLDER, &args![block]).u32()
        };
        e.mem.set_u8(HOLDER_CREATED, 1);
        built
    } else {
        e.mem.set_u8(HOLDER_CREATED, 0);
        if existing != 0 {
            existing
        } else {
            holder
        }
    };
    e.mem.set_u32(HOLDER, chosen);
    fn_00aa3050(e);
    let hook = e.mem.u32(START_HOOK);
    if hook != 0 {
        e.call(hook, &args![]);
    }
    let mut index = 0u32;
    if e.mem.u32(STARTUP_COUNT) != 0 {
        loop {
            let function = e
                .mem
                .u32(STARTUP_FUNCTIONS.wrapping_add(index.wrapping_mul(4)));
            e.call(function, &args![]);
            index = index.wrapping_add(1);
            if index >= e.mem.u32(STARTUP_COUNT) {
                break;
            }
        }
    }
    e.mem.set_u32(mode, saved);
    e.mem.set_u8(STARTUP_FINISHED, 1);
}

// Translated from 00aa2200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word; run by `fn_00aa20b0` on the 8-byte
/// block it allocates for the holder): allocates 4 bytes from the memory manager
/// (`00aa3e40`); when that succeeds it stores the vtable `010a24fc` in them and their
/// address at `this`, otherwise null at `this`; the byte at `this + 4` is set to 1 either
/// way. Returns `this`.
pub fn fn_00aa2200(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    if block != 0 {
        e.mem.set_u32(block, 0x010a_24fc);
        e.mem.set_u32(this.addr(), block);
    } else {
        e.mem.set_u32(this.addr(), 0);
    }
    e.mem.set_u8(this.addr() + 4, 1);
    this
}

/// `00aa4060` (`thiscall` on the memory manager, one stack word): deallocate.
const DEALLOCATE: u32 = 0x00aa_4060;
/// `00aa3070` (no arguments): the shutdown step `00aa2170` makes after the hooks.
const SHUTDOWN_STEP: u32 = 0x00aa_3070;
/// `00aa21e0` (`thiscall`, no stack word): run on the holder before it is freed.
const HOLDER_TEAR_DOWN: u32 = 0x00aa_21e0;

// Translated from 00aa2170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the counterpart of the start-up. Calls each
/// registered hook's second word (`011f6088`, as many as `011f6108` counts, read again after
/// each call), then the word at `011f6114` when it is set, then `00aa3070`; when the holder
/// was created (`011f610d`) and `011f6118` holds it, runs `00aa21e0` on it and gives it back
/// to the memory manager (the pointer is not cleared). Last, clears the byte `011f610c`.
pub fn fn_00aa2170(e: &mut Engine) {
    let mut index = 0u32;
    if e.mem.u32(STARTUP_COUNT) > index {
        loop {
            let function = e.mem.u32(STARTUP_SECOND_WORDS + 4 * index);
            e.call(function, &args![]);
            index = index.wrapping_add(1);
            if index >= e.mem.u32(STARTUP_COUNT) {
                break;
            }
        }
    }
    let last_hook = e.mem.u32(START_HOOK_DATA);
    if last_hook != 0 {
        e.call(last_hook, &args![]);
    }
    e.call(SHUTDOWN_STEP, &args![]);
    if e.mem.u8(HOLDER_CREATED) != 0 {
        let holder = e.mem.u32(HOLDER);
        if holder != 0 {
            e.call(HOLDER_TEAR_DOWN, &args![holder]);
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, holder]);
        }
    }
    e.mem.set_u8(STARTUP_FINISHED, 0);
}

// Translated from 00aa21e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): the holder's tear-down, the
/// counterpart of [`fn_00aa2200`]. When the byte at `this + 4` is set and the object at
/// `this` is not null, calls the object's vtable slot 0 (its deleting destructor) with 1.
pub fn fn_00aa21e0(e: &mut Engine, this: Ptr) {
    if e.mem.u8(this.addr() + 4) == 0 {
        return;
    }
    let object = e.mem.u32(this.addr());
    if object == 0 {
        return;
    }
    e.vcall(object, 0, &args![1u32]);
}

/// `00aa1d60` (`cdecl`: the address and a count of 8-byte values): reverses
/// the bytes of each value.
const SWAP_EIGHT_BYTES: u32 = 0x00aa_1d60;

// Translated from 00aa1de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the buffer, its byte count, the
/// component sizes and their count): swaps the byte order of the buffer's
/// components in place. While fewer than `count` bytes have been passed, it
/// goes through the sizes in order (a pass over all of them each round):
/// a component of 2 bytes swaps its two bytes, of 4 bytes reverses them, of
/// 8 bytes goes to `00aa1d60` with a count of 1, of 16 or 64 bytes reverses
/// each of its 4-byte groups; any other size is left as it is. Every
/// component moves on by its size. (The compiler's byte map and jump table
/// for the size are the `match`; with no sizes and a non-zero count the exe
/// never leaves the loop, and neither does this.)
pub fn fn_00aa1de0(e: &mut Engine, buffer: Ptr, count: u32, sizes: Ptr, size_count: u32) {
    if count == 0 {
        return;
    }
    let mut at = buffer.addr();
    let mut passed = 0u32;
    loop {
        let mut i = 0u32;
        while i < size_count {
            let size = e.mem.u32(sizes.addr().wrapping_add(i.wrapping_mul(4)));
            match size {
                2 => {
                    let low = e.mem.u8(at.wrapping_add(1));
                    let high = e.mem.u8(at);
                    e.mem.set_u8(at, low);
                    e.mem.set_u8(at.wrapping_add(1), high);
                }
                4 => swap_four(e, at),
                8 => {
                    e.call(SWAP_EIGHT_BYTES, &args![at, 1u32]);
                }
                16 | 64 => {
                    let mut groups = size >> 2;
                    let mut group = at;
                    while groups != 0 {
                        swap_four(e, group);
                        group = group.wrapping_add(4);
                        groups -= 1;
                    }
                }
                _ => {}
            }
            passed = passed.wrapping_add(size);
            i = i.wrapping_add(1);
            at = at.wrapping_add(size);
        }
        if passed >= count {
            return;
        }
    }
}

/// Reverses the four bytes at `at` (byte 0 with 3, then 1 with 2).
fn swap_four(e: &mut Engine, at: u32) {
    let first = e.mem.u8(at);
    let last = e.mem.u8(at.wrapping_add(3));
    e.mem.set_u8(at, last);
    e.mem.set_u8(at.wrapping_add(3), first);
    let third = e.mem.u8(at.wrapping_add(2));
    let second = e.mem.u8(at.wrapping_add(1));
    e.mem.set_u8(at.wrapping_add(1), third);
    e.mem.set_u8(at.wrapping_add(2), second);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_1de0, fn_00aa1de0(Ptr, u32, Ptr, u32)),
        entry!(0x00aa_21e0, fn_00aa21e0(Ptr)),
        entry!(
            0x00aa_1f10,
            fn_00aa1f10(Ptr<NiBinaryStream>, Ptr, u32) -> i32
        ),
        entry!(0x00aa_1fd0, fn_00aa1fd0(Ptr<NiBinaryStream>, Ptr) -> i32),
        entry!(0x00aa_2060, fn_00aa2060(u32, u32) -> u32),
        entry!(0x00aa_2080, fn_00aa2080(u32, u32) -> u32),
        entry!(0x00aa_20b0, fn_00aa20b0(u32)),
        entry!(0x00aa_3050, fn_00aa3050()),
        entry!(0x00aa_2200, fn_00aa2200(Ptr) -> Ptr),
        entry!(0x00aa_2170, fn_00aa2170()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    const READ: u32 = 0x00aa_2e60;
    const WRITE: u32 = 0x00aa_2ea0;

    type Input = Rc<RefCell<VecDeque<u8>>>;
    type Output = Rc<RefCell<Vec<u8>>>;
    type Fixture = (Engine, Ptr<NiBinaryStream>, Input, Output);

    /// A stream whose read function hands out `input` one byte at a time
    /// (0 bytes read when it is empty) and whose write function collects the
    /// bytes, accepting at most `accept` of them.
    fn fixture(input: &[u8], accept: usize) -> Fixture {
        let mut e = Engine::new();
        let stream = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 8, READ);
        e.mem.set_u32(stream.addr() + 0xc, WRITE);
        let queue: Input = Rc::new(RefCell::new(input.iter().copied().collect()));
        let sink: Output = Rc::default();
        let source = queue.clone();
        e.register_double(READ, move |e, a| {
            // (stream, buffer, 1, &count, 1)
            assert_eq!(a[2], 1);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), 1);
            match source.borrow_mut().pop_front() {
                Some(byte) => {
                    e.mem.set_u8(a[1], byte);
                    1u32.into_ret()
                }
                None => 0u32.into_ret(),
            }
        });
        let collected = sink.clone();
        e.register_double(WRITE, move |e, a| {
            assert_eq!(a[2], 1);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), 1);
            if collected.borrow().len() >= accept {
                return 0u32.into_ret();
            }
            collected.borrow_mut().push(e.mem.u8(a[1]));
            1u32.into_ret()
        });
        (e, stream, queue, sink)
    }

    fn read_line(input: &[u8], size: u32) -> (i32, Vec<u8>, u32, Vec<u8>) {
        let (mut e, stream, queue, _) = fixture(input, 0);
        let buffer = e.mem.alloc(0x40);
        e.mem.write(buffer, &[0x55; 0x40]);
        let read = e.call(0x00aa_1f10, &args![stream, buffer, size]).i32();
        let left: Vec<u8> = queue.borrow().iter().copied().collect();
        let position = e.mem.u32(stream.addr() + 4);
        (read, e.mem.cstr(buffer), position, left)
    }

    #[test]
    fn read_line_stops_at_the_line_feed_and_skips_carriage_returns() {
        let (read, text, position, left) = read_line(b"ab\r\ncd", 0x20);
        assert_eq!(read, 4);
        assert_eq!(text, b"ab");
        assert_eq!(position, 4);
        assert_eq!(left, b"cd");
    }

    #[test]
    fn read_line_stops_when_the_buffer_is_full() {
        let (read, text, _, left) = read_line(b"abcdef", 4);
        assert_eq!(read, 3);
        assert_eq!(text, b"abc");
        assert_eq!(left, b"def");
    }

    #[test]
    fn read_line_ends_at_the_end_of_the_input() {
        let (read, text, position, _) = read_line(b"xy", 0x20);
        assert_eq!(read, 2);
        assert_eq!(text, b"xy");
        assert_eq!(position, 2);
    }

    #[test]
    fn read_line_with_a_tiny_buffer_only_terminates() {
        let (read, text, _, left) = read_line(b"abc", 1);
        assert_eq!(read, 0);
        assert!(text.is_empty());
        assert_eq!(left, b"abc");
    }

    #[test]
    fn write_string_writes_the_characters_without_the_terminator() {
        let (mut e, stream, _, sink) = fixture(b"", usize::MAX);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"NIF");
        assert_eq!(e.call(0x00aa_1fd0, &args![stream, text]).i32(), 3);
        assert_eq!(*sink.borrow(), b"NIF");
        assert_eq!(e.mem.u32(stream.addr() + 4), 3);
    }

    #[test]
    fn write_string_stops_at_the_first_short_write() {
        let (mut e, stream, _, sink) = fixture(b"", 2);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"NIF");
        assert_eq!(e.call(0x00aa_1fd0, &args![stream, text]).i32(), 2);
        assert_eq!(*sink.borrow(), b"NI");
        assert_eq!(e.mem.u32(stream.addr() + 4), 2);
    }

    #[test]
    fn write_string_of_the_empty_string_writes_nothing() {
        let (mut e, stream, _, sink) = fixture(b"", usize::MAX);
        let text = e.mem.alloc(8);
        assert_eq!(e.call(0x00aa_1fd0, &args![stream, text]).i32(), 0);
        assert!(sink.borrow().is_empty());
    }
}

#[cfg(test)]
mod startup_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Order = Rc<RefCell<Vec<u32>>>;

    fn tls_mode(e: &mut Engine) -> u32 {
        let at = e.tls() + TLS_MODE;
        e.mem.u32(at)
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e
    }

    /// Doubles for the callees of `fn_00aa3050` that record the order they run in.
    fn double_startup_callees(e: &mut Engine) -> Order {
        let order: Order = Rc::default();
        for address in [ALLOCATOR_INITIALIZE, TIMER_INITIALIZE, OEI_IO_INITIALIZE] {
            let log = order.clone();
            e.register_double(address, move |_, _| {
                log.borrow_mut().push(address);
                Ret::default()
            });
        }
        order
    }

    #[test]
    fn hook_registration_stores_both_words_and_returns_the_function() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00aa_2060, &args![0xa89cc0u32, 0xa8a280u32]).u32(),
            0xa89cc0
        );
        assert_eq!(e.mem.u32(START_HOOK), 0xa89cc0);
        assert_eq!(e.mem.u32(START_HOOK_DATA), 0xa8a280);
    }

    #[test]
    fn registration_before_the_end_of_start_up_only_adds_the_entry() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_2080, &args![0x1111u32, 0x2222u32]).u32(), 1);
        assert_eq!(e.call(0x00aa_2080, &args![0x3333u32, 0x4444u32]).u32(), 2);
        assert_eq!(e.mem.u32(STARTUP_COUNT), 2);
        assert_eq!(e.mem.u32(STARTUP_FUNCTIONS), 0x1111);
        assert_eq!(e.mem.u32(STARTUP_FUNCTIONS + 4), 0x3333);
        assert_eq!(e.mem.u32(STARTUP_SECOND_WORDS), 0x2222);
        assert_eq!(e.mem.u32(STARTUP_SECOND_WORDS + 4), 0x4444);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(2));
    }

    #[test]
    fn registration_after_the_end_of_start_up_runs_the_function_at_once() {
        let mut e = engine();
        e.mem.set_u8(STARTUP_FINISHED, 1);
        e.register(0x1111, |_, a| (a[0] + a[1]).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_2080, &args![0x1111u32, 0x2222u32]).u32(),
            0x3333
        );
        assert_eq!(e.mem.u32(STARTUP_COUNT), 1);
        let calls: Vec<_> = e
            .call_log
            .clone()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x1111)
            .collect();
        assert_eq!(calls, vec![(0x1111, vec![0x1111, 0x2222])]);
    }

    #[test]
    fn start_up_helper_runs_its_three_steps_once() {
        let mut e = engine();
        let order = double_startup_callees(&mut e);
        e.call(0x00aa_3050, &args![]);
        assert_eq!(e.mem.u8(START_UP_DONE), 1);
        assert_eq!(
            *order.borrow(),
            vec![ALLOCATOR_INITIALIZE, TIMER_INITIALIZE, OEI_IO_INITIALIZE]
        );
        e.call(0x00aa_3050, &args![]);
        assert_eq!(order.borrow().len(), 3);
    }

    /// `fn_00aa20b0` with the allocation callees doubled, plus a hook and two registered
    /// functions that record the TLS word they run under.
    fn init_fixture(allocation: u32) -> (Engine, Order) {
        let mut e = engine();
        let order = double_startup_callees(&mut e);
        e.register_double(ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER, 8]);
            allocation.into_ret()
        });
        e.register(CONSTRUCT_HOLDER, |_, a| a[0].into_ret());
        let at = e.tls() + TLS_MODE;
        e.mem.set_u32(at, 0x55);
        for address in [0x2000u32, 0x2100, 0x2200] {
            let log = order.clone();
            e.register_double(address, move |e, a| {
                assert!(a.is_empty());
                assert_eq!(tls_mode(e), 0xf);
                log.borrow_mut().push(address);
                Ret::default()
            });
        }
        (e, order)
    }

    #[test]
    fn init_builds_the_holder_when_none_is_given_or_held() {
        let (mut e, order) = init_fixture(0x7000);
        e.mem.set_u32(START_HOOK, 0x2000);
        e.call(0x00aa_2080, &args![0x2100u32, 0]);
        e.call(0x00aa_2080, &args![0x2200u32, 0]);
        e.call(0x00aa_20b0, &args![0u32]);
        assert_eq!(e.mem.u32(HOLDER), 0x7000);
        assert_eq!(e.mem.u8(HOLDER_CREATED), 1);
        assert_eq!(e.mem.u8(STARTUP_FINISHED), 1);
        assert_eq!(tls_mode(&mut e), 0x55);
        assert_eq!(
            *order.borrow(),
            vec![
                ALLOCATOR_INITIALIZE,
                TIMER_INITIALIZE,
                OEI_IO_INITIALIZE,
                0x2000,
                0x2100,
                0x2200
            ]
        );
    }

    #[test]
    fn init_leaves_a_null_holder_when_the_allocation_fails() {
        let (mut e, _) = init_fixture(0);
        e.mem.set_u32(HOLDER, 0);
        e.call(0x00aa_20b0, &args![0u32]);
        assert_eq!(e.mem.u32(HOLDER), 0);
        assert_eq!(e.mem.u8(HOLDER_CREATED), 1);
    }

    #[test]
    fn init_uses_the_given_holder_unless_one_is_already_held() {
        let (mut e, _) = init_fixture(0x7000);
        e.mem.set_u8(HOLDER_CREATED, 1);
        e.call(0x00aa_20b0, &args![0x8000u32]);
        assert_eq!(e.mem.u32(HOLDER), 0x8000);
        assert_eq!(e.mem.u8(HOLDER_CREATED), 0);
        e.call(0x00aa_20b0, &args![0x9000u32]);
        assert_eq!(e.mem.u32(HOLDER), 0x8000);
        e.mem.set_u8(HOLDER_CREATED, 1);
        e.call(0x00aa_20b0, &args![0u32]);
        assert_eq!(e.mem.u32(HOLDER), 0x8000);
        assert_eq!(e.mem.u8(HOLDER_CREATED), 0);
    }

    #[test]
    fn init_runs_functions_registered_while_it_runs() {
        let (mut e, order) = init_fixture(0x7000);
        e.mem.set_u32(HOLDER, 0x1234);
        e.call(0x00aa_2080, &args![0x2000u32, 0]);
        let log = order.clone();
        e.register_double(0x2000, move |e, _| {
            log.borrow_mut().push(0x2000);
            e.call(0x00aa_2080, &args![0x2100u32, 0]);
            Ret::default()
        });
        e.call(0x00aa_20b0, &args![0u32]);
        assert_eq!(e.mem.u32(STARTUP_COUNT), 2);
        let ran: Vec<u32> = order
            .borrow()
            .iter()
            .copied()
            .filter(|a| (0x2000..0x3000).contains(a))
            .collect();
        assert_eq!(ran, vec![0x2000, 0x2100]);
    }
}

#[cfg(test)]
mod holder_tests {
    use super::*;

    #[test]
    fn holder_takes_a_four_byte_block_with_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        let block = e.mem.alloc(4);
        e.register_double(ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER, 4]);
            block.into_ret()
        });
        assert_eq!(e.call(0x00aa_2200, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), block);
        assert_eq!(e.mem.u8(this + 4), 1);
        assert_eq!(e.mem.u32(block), 0x010a_24fc);
    }

    #[test]
    fn holder_keeps_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x1234);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00aa_2200, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u8(this + 4), 1);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 != 0x00aa_2170)
            .collect()
    }

    #[test]
    fn fn_00aa2170_runs_the_second_words_then_frees_the_holder() {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        for function in [0x0150_0000u32, 0x0150_0010, 0x0150_0020, SHUTDOWN_STEP] {
            e.register(function, |_, _| Ret::default());
        }
        e.register(HOLDER_TEAR_DOWN, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.mem.set_u32(STARTUP_COUNT, 2);
        e.mem.set_u32(STARTUP_SECOND_WORDS, 0x0150_0000);
        e.mem.set_u32(STARTUP_SECOND_WORDS + 4, 0x0150_0010);
        e.mem.set_u32(START_HOOK_DATA, 0x0150_0020);
        e.mem.set_u8(HOLDER_CREATED, 1);
        e.mem.set_u32(HOLDER, 0x7000);
        e.mem.set_u8(STARTUP_FINISHED, 1);
        e.call_log = Some(vec![]);
        e.call(0x00aa_2170, &args![]);
        assert_eq!(
            calls(&mut e),
            vec![
                (0x0150_0000, vec![]),
                (0x0150_0010, vec![]),
                (0x0150_0020, vec![]),
                (SHUTDOWN_STEP, vec![]),
                (HOLDER_TEAR_DOWN, vec![0x7000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, 0x7000]),
            ]
        );
        assert_eq!(e.mem.u8(STARTUP_FINISHED), 0);
        assert_eq!(e.mem.u32(HOLDER), 0x7000);

        // Nothing registered, no last hook, holder not created.
        e.mem.set_u32(STARTUP_COUNT, 0);
        e.mem.set_u32(START_HOOK_DATA, 0);
        e.mem.set_u8(HOLDER_CREATED, 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_2170, &args![]);
        assert_eq!(calls(&mut e), vec![(SHUTDOWN_STEP, vec![])]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00aa21e0_deletes_the_owned_object() {
        let mut e = Engine::new();
        e.put_vtable(0x0090_1000, &[0x0090_0000]);
        e.register(0x0090_0000, |_, _| Ret::default());
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0090_1000);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, object);
        e.mem.set_u8(holder + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00aa_21e0, &args![holder]);
        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0090_0000).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![object, 1]);
        // Not owned, or no object: nothing is called.
        e.mem.set_u8(holder + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_21e0, &args![holder]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u8(holder + 4, 1);
        e.mem.set_u32(holder, 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_21e0, &args![holder]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod tests_n084 {
    use super::*;

    #[test]
    fn fn_00aa1de0_swaps_each_component_size() {
        let mut e = Engine::new();
        e.register(SWAP_EIGHT_BYTES, |_, _| Ret::default());
        // Sizes 2, 4, 8, 16, 3: 33 bytes a round, two rounds for 40 bytes.
        let sizes = e.mem.alloc(20);
        for (i, s) in [2u32, 4, 8, 16, 3].iter().enumerate() {
            e.mem.set_u32(sizes + 4 * i as u32, *s);
        }
        let buffer = e.mem.alloc(66);
        let bytes: Vec<u8> = (0..66).collect();
        e.mem.write(buffer, &bytes);
        e.call_log = Some(vec![]);
        e.call(0x00aa_1de0, &args![buffer, 40u32, sizes, 5u32]);
        let mut out = vec![0u8; 66];
        for (i, b) in out.iter_mut().enumerate() {
            *b = e.mem.u8(buffer + i as u32);
        }
        let mut expected = Vec::new();
        for round in 0..2u8 {
            let b = round * 33;
            expected.extend([b + 1, b]);
            expected.extend([b + 5, b + 4, b + 3, b + 2]);
            expected.extend(b + 6..b + 14);
            for g in 0..4u8 {
                let s = b + 14 + 4 * g;
                expected.extend([s + 3, s + 2, s + 1, s]);
            }
            expected.extend(b + 30..b + 33);
        }
        assert_eq!(out, expected);
        let eights: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == SWAP_EIGHT_BYTES)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(eights, vec![vec![buffer + 6, 1], vec![buffer + 39, 1]]);
    }

    #[test]
    fn fn_00aa1de0_does_nothing_for_no_bytes() {
        let mut e = Engine::new();
        e.call(0x00aa_1de0, &args![0u32, 0u32, 0u32, 0u32]);
    }
}
