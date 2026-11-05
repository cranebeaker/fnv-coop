//! C++ exceptions by behaviour (ADR-0013): what a throw in FalloutNV.exe 1.4.0.525 ends up doing,
//! without the exe's machinery (no frames in guest memory, no `FS:[0]` chain, no unwind funclets,
//! no cookies).
//!
//! A throw reaches `_CxxThrowException` (`00ec7894`), whose `RaiseException` comes here
//! ([`throw`]). When a try region of the running OS thread is open, the throw is a Rust unwind
//! with the thrown object's addresses as its typed payload, caught by that region's
//! [`try_catch`]. The 38 functions of the exe with a handler that goes on are the only ones with
//! a region, each around its exe try states. Every one of them ends in `catch(...)`, so any open
//! region takes any C++ exception: "a region is open" is the exe's search for a handler.
//!
//! With no region open nothing catches it, which in the exe is `terminate`: a stop naming the
//! exception, at the throw, as any other stop. The exe's `catch(...)`-and-rethrow frames and
//! destructor-only frames are not built; their guest objects leak on a throw (ADR-0013 lists the
//! throw paths and what else a skipped cleanup could do).
//!
//! The thrown object stays where the thrower built it, as in the exe: the guest stack's top is
//! left at the throw while the catch block runs on top of it (the exe's catch funclet also runs
//! above the thrower's frames), and put back when the block ends.
//!
//! The regions are counted per OS thread because a Rust unwind stays on its OS thread, and each
//! game thread is one OS thread (`thread_host`): a region open on another game thread catches
//! nothing here.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

use crate::Engine;

/// The code `_CxxThrowException` raises (`0xE06D7363`, the record template at `010f1f80`).
pub const CXX_EXCEPTION_CODE: u32 = 0xe06d_7363;
/// The record's first parameter when it is a throw of this runtime: the template's value and the
/// two the handler of the exe also accepts (`___InternalCxxFrameHandler`), and the value
/// `_CxxThrowException` stores for a pure throw.
const THROW_MAGIC_NUMBERS: [u32; 4] = [0x1993_0520, 0x1993_0521, 0x1993_0522, 0x0199_4000];
/// The name `catch(std::exception&)` matches on (its `TypeDescriptor`'s `+8`).
pub const STD_EXCEPTION_NAME: &[u8] = b".?AVexception@std@@";

/// How many throws have been caught in this process. A caught throw is the exe's behaviour, but a
/// port bug that makes a throw also ends here (a short read becomes a missing tree), so every
/// catch is counted and the first [`CAUGHT_LINES`] are printed ([`note_caught`]).
static CAUGHT_THROWS: AtomicU64 = AtomicU64::new(0);
const CAUGHT_LINES: u64 = 64;

/// The throws caught so far in this process (the host's summary line).
pub fn caught_throws() -> u64 {
    CAUGHT_THROWS.load(Relaxed)
}

/// What a C++ throw carries: the thrown object (where the thrower built it) and its `ThrowInfo`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Thrown {
    pub object: u32,
    pub throw_info: u32,
}

thread_local! {
    /// How many [`try_catch`] bodies are running on this OS thread.
    static OPEN_REGIONS: Cell<u32> = const { Cell::new(0) };
    /// The exceptions whose catch blocks are running on this OS thread, innermost last: what
    /// `throw;` rethrows (`_getptd()->_curexception` in the exe).
    static HANDLING: RefCell<Vec<Thrown>> = const { RefCell::new(Vec::new()) };
}

/// What a catch puts back from when the region opened: what a stop's catcher restores.
#[derive(Clone, Copy)]
struct Mark {
    guest_stack: (usize, Option<u32>),
    call_stack: usize,
    stack_frames: usize,
}

/// `try { body } catch (...) { handler }` for one of the exe's try regions: `body`'s result, or,
/// when a C++ exception is thrown under it, `handler`'s, with the engine's stacks back where they
/// were when the region opened. A stop (any other panic) passes through untouched: `/EHs` means
/// the exe's `catch(...)` takes no structured exception either.
///
/// `handler` is the catch block and what the function's continuation needs from it; it tells
/// `catch(T&)` from `catch(...)` with [`caught_as`]. A `throw;` in it goes to the next region
/// out, or stops. A value `body` changed before the throw keeps its change, as a frame local of
/// the exe does, so results the exe keeps in frame locals stay outside the closures.
#[inline]
pub fn try_catch<R>(
    e: &mut Engine,
    body: impl FnOnce(&mut Engine) -> R,
    handler: impl FnOnce(&mut Engine, Thrown) -> R,
) -> R {
    let mark = Mark {
        guest_stack: e.guest_stack_mark(),
        call_stack: e.call_stack.len(),
        stack_frames: e.stack_frames.len(),
    };
    OPEN_REGIONS.with(|open| open.set(open.get() + 1));
    let outcome = catch_unwind(AssertUnwindSafe(|| body(e)));
    OPEN_REGIONS.with(|open| open.set(open.get() - 1));
    match outcome {
        Ok(value) => value,
        Err(payload) => caught(e, mark, payload, handler),
    }
}

/// The catch: only a guest throw stops here. Nothing here runs without a throw.
#[cold]
#[inline(never)]
fn caught<R>(
    e: &mut Engine,
    mark: Mark,
    payload: Box<dyn Any + Send>,
    handler: impl FnOnce(&mut Engine, Thrown) -> R,
) -> R {
    let thrown = match payload.downcast::<Thrown>() {
        Ok(thrown) => *thrown,
        Err(stop) => resume_unwind(stop),
    };
    note_caught(e, mark.call_stack, thrown);
    // The frames between the throw and here are gone. The guest stack's top stays: the object is
    // up there, and the catch block runs below it as the exe's does.
    unwound_to(e, mark);
    if e.mem.block_check_on() {
        let running = e.call_stack.last().copied().unwrap_or(0);
        e.mem.set_check_fn(running);
    }
    let outcome = {
        let _handling = Handling::begin(thrown);
        catch_unwind(AssertUnwindSafe(|| handler(e, thrown)))
    };
    // The catch block ended: its object is destroyed (`__DestructExceptionObject`, the `__finally`
    // at `00ece76b`), unless an outer catch block is still handling the same object
    // (`_IsExceptionObjectToBeDestroyed`).
    let still_used = HANDLING.with(|h| h.borrow().iter().any(|t| t.object == thrown.object));
    match outcome {
        Ok(result) => {
            if !still_used {
                destroy_object(e, thrown);
            }
            e.guest_stack_restore(mark.guest_stack);
            result
        }
        Err(leaving) => {
            // A new throw leaving the block destroys the object too; `throw;` passes it on (the
            // exe's filter `00ece4ab` tells them apart). A stop leaves the guest as it is.
            let new_throw = leaving
                .downcast_ref::<Thrown>()
                .is_some_and(|new| new.object != thrown.object);
            if new_throw && !still_used {
                unwound_to(e, mark);
                destroy_object(e, thrown);
            }
            resume_unwind(leaving)
        }
    }
}

/// The running frames above a region are gone: the engine's copies of the call stack go back.
fn unwound_to(e: &mut Engine, mark: Mark) {
    e.call_stack.truncate(mark.call_stack);
    e.stack_frames.truncate(mark.stack_frames);
    crate::profile::resync(&e.call_stack);
}

#[cfg(test)]
thread_local! {
    /// This thread's catches: tests run side by side, and the process-wide count is everyone's.
    static CAUGHT_HERE: Cell<u64> = const { Cell::new(0) };
}

/// A catch, counted and for the first [`CAUGHT_LINES`] printed. It only reads guest memory, with
/// no guest call, so the guest, the call log and the trace are as they would be without it.
fn note_caught(e: &Engine, depth: usize, thrown: Thrown) {
    #[cfg(test)]
    CAUGHT_HERE.with(|here| here.set(here.get() + 1));
    if CAUGHT_THROWS.fetch_add(1, Relaxed) < CAUGHT_LINES {
        eprintln!("{}", caught_line(e, depth, thrown));
    }
}

/// `C++ exception <type chain> "<text>" caught by <catcher>, thrown in <where>`: the catcher is
/// the function the region's opener was called as (a translation that calls another of its unit
/// directly is not on the call stack), and where it was thrown the top of the call stack, innermost
/// first, as a stop's `(in ...)` gives it.
fn caught_line(e: &Engine, depth: usize, thrown: Thrown) -> String {
    let catcher = depth
        .checked_sub(1)
        .and_then(|top| e.call_stack.get(top))
        .map_or("the host".to_string(), |addr| format!("{addr:08x}"));
    let path: Vec<String> = e
        .call_stack
        .iter()
        .rev()
        .take(6)
        .map(|addr| format!("{addr:08x}"))
        .collect();
    format!(
        "C++ exception {} caught by {catcher}, thrown in {}",
        describe(e, thrown),
        path.join(" < ")
    )
}

/// A guest word, or `None` where nothing is mapped.
fn peek_word(e: &Engine, addr: u32) -> Option<u32> {
    let mut bytes = [0; 4];
    e.mem.try_read(addr, &mut bytes).ok()?;
    Some(u32::from_le_bytes(bytes))
}

/// A guest C string (the first 512 bytes at most), or `None` where nothing is mapped.
fn peek_text(e: &Engine, addr: u32) -> Option<Vec<u8>> {
    let mut text = Vec::new();
    for offset in 0..512 {
        let mut byte = [0];
        e.mem.try_read(addr.wrapping_add(offset), &mut byte).ok()?;
        if byte[0] == 0 {
            break;
        }
        text.push(byte[0]);
    }
    Some(text)
}

/// The object's message, read and not asked for, by the rule nv-trace's recorder uses
/// (`nv_trace_core::exceptions::message`): `runtime_error` and what derives from it keep a
/// `std::string` at `+0xc` (its text inline at `+0x10`, or from a capacity of 0x10 on, the pointer
/// there; the capacity at `+0x24`), `std::exception` a text pointer at `+4`.
fn object_text(e: &Engine, object: u32, chain: &[Vec<u8>]) -> Option<String> {
    let has = |name: &[u8]| chain.iter().any(|n| n == name);
    let text = if has(b".?AVruntime_error@std@@") {
        if peek_word(e, object.wrapping_add(0x24))? >= 0x10 {
            peek_word(e, object.wrapping_add(0x10))?
        } else {
            object.wrapping_add(0x10)
        }
    } else if has(STD_EXCEPTION_NAME) {
        peek_word(e, object.wrapping_add(4)).filter(|&text| text != 0)?
    } else {
        return None;
    };
    peek_text(e, text).map(|text| String::from_utf8_lossy(&text).into_owned())
}

/// A thrown object in words, from what can be read: its type, the types below it and its message.
fn describe(e: &Engine, thrown: Thrown) -> String {
    let chain = type_chain(e, thrown.throw_info);
    let mut names = chain.iter().map(|name| readable_type(name));
    let most_derived = names.next().unwrap_or_else(|| "(no type)".to_string());
    let bases: Vec<String> = names.collect();
    let bases = if bases.is_empty() {
        String::new()
    } else {
        format!(" (a {})", bases.join(", "))
    };
    let text = object_text(e, thrown.object, &chain)
        .map(|text| format!(" \"{text}\""))
        .unwrap_or_default();
    format!("{most_derived}{bases}{text}")
}

/// A catch block running: what `throw;` finds. Dropped however the block ends (a host-side list,
/// nothing in guest memory; a stop unwinding through it must leave the list right too).
struct Handling {
    depth: usize,
}

impl Handling {
    fn begin(thrown: Thrown) -> Handling {
        let depth = HANDLING.with(|h| {
            let mut h = h.borrow_mut();
            h.push(thrown);
            h.len() - 1
        });
        Handling { depth }
    }
}

impl Drop for Handling {
    fn drop(&mut self) {
        HANDLING.with(|h| h.borrow_mut().truncate(self.depth));
    }
}

/// `pmfnUnwind` of the `ThrowInfo` (`+4`), when it has one, on the object (`thiscall`).
fn destroy_object(e: &mut Engine, thrown: Thrown) {
    let destructor = e.mem.u32(thrown.throw_info.wrapping_add(4));
    if destructor != 0 {
        e.call(destructor, &[thrown.object]);
    }
}

/// The throw: `RaiseException(0xE06D7363, ...)` from `_CxxThrowException(object, throw_info)`.
/// A null `throw_info` is `throw;`, the exception being handled. It goes to the innermost open
/// region; with none open it stops, naming the exception (the exe's `terminate`, with nothing
/// destroyed).
pub fn throw(e: &mut Engine, object: u32, throw_info: u32) -> ! {
    let thrown = if throw_info == 0 {
        match HANDLING.with(|h| h.borrow().last().copied()) {
            Some(handling) => handling,
            None => panic!("C++ rethrow (`throw;`) with no exception being handled: terminate"),
        }
    } else {
        Thrown { object, throw_info }
    };
    if OPEN_REGIONS.with(Cell::get) > 0 {
        // `resume_unwind` runs no panic hook: a caught throw prints nothing.
        resume_unwind(Box::new(thrown));
    }
    panic!("{}", uncaught_message(e, thrown));
}

/// Whether `arguments` (the words after the code, `RaiseException`'s `count` of them) are those of
/// a C++ throw; answers the object and the `ThrowInfo`.
pub fn throw_record(e: &Engine, code: u32, count: u32, arguments: u32) -> Option<(u32, u32)> {
    if code != CXX_EXCEPTION_CODE || count != 3 {
        return None;
    }
    let magic = e.mem.u32(arguments);
    THROW_MAGIC_NUMBERS.contains(&magic).then(|| {
        (
            e.mem.u32(arguments.wrapping_add(4)),
            e.mem.u32(arguments.wrapping_add(8)),
        )
    })
}

/// The `TypeDescriptor` names of a `ThrowInfo`'s catchable types, most derived first
/// (`+0xc` the array: a count and the `CatchableType`s; `+4` of each the descriptor; its `+8` the
/// name).
/// What cannot be read is left out: this runs while a stop or a catch is being reported.
pub fn type_chain(e: &Engine, throw_info: u32) -> Vec<Vec<u8>> {
    let Some(array) = peek_word(e, throw_info.wrapping_add(0xc)) else {
        return Vec::new();
    };
    let count = peek_word(e, array).unwrap_or(0).min(64);
    (0..count)
        .filter_map(|index| {
            let catchable = peek_word(e, array.wrapping_add(4 + 4 * index))?;
            let descriptor = peek_word(e, catchable.wrapping_add(4))?;
            peek_text(e, descriptor.wrapping_add(8))
        })
        .collect()
}

/// `catch(T&)` for the type named `name`: the object as a `T` when the throw's chain has it
/// (`__AdjustPointer` with the catchable type's displacements), else `None`.
pub fn caught_as(e: &Engine, thrown: Thrown, name: &[u8]) -> Option<u32> {
    let array = e.mem.u32(thrown.throw_info.wrapping_add(0xc));
    let count = e.mem.u32(array);
    (0..count).find_map(|index| {
        let catchable = e.mem.u32(array.wrapping_add(4 + 4 * index));
        let descriptor = e.mem.u32(catchable.wrapping_add(4));
        (e.mem.cstr(descriptor.wrapping_add(8)) == name).then(|| {
            let member = e.mem.u32(catchable.wrapping_add(8));
            let virtual_base = e.mem.i32(catchable.wrapping_add(0xc));
            let in_table = e.mem.u32(catchable.wrapping_add(0x10));
            let mut object = thrown.object.wrapping_add(member);
            if virtual_base >= 0 {
                let table = e.mem.u32(thrown.object.wrapping_add(virtual_base as u32));
                object = object
                    .wrapping_add(virtual_base as u32)
                    .wrapping_add(e.mem.u32(table.wrapping_add(in_table)));
            }
            object
        })
    })
}

/// A decorated class name as C++ writes it: `.?AVfailure@ios_base@std@@` is
/// `std::ios_base::failure`. Anything else comes back as it is.
fn readable_type(name: &[u8]) -> String {
    let text = String::from_utf8_lossy(name);
    match text
        .strip_prefix(".?AV")
        .or_else(|| text.strip_prefix(".?AU"))
    {
        Some(rest) => rest
            .trim_end_matches('@')
            .split('@')
            .rev()
            .collect::<Vec<_>>()
            .join("::"),
        None => text.into_owned(),
    }
}

/// The stop's line for a throw nothing catches. Read only, like the catch's line: a guest `what()`
/// could be untranslated and replace this text, or use up the slice and switch threads here.
fn uncaught_message(e: &Engine, thrown: Thrown) -> String {
    format!(
        "uncaught C++ exception {}, object {:08x}, ThrowInfo {:08x}: nothing above catches it, \
         terminate in the exe",
        describe(e, thrown),
        thrown.object,
        thrown.throw_info
    )
}

/// A `ThrowInfo` written at `at` (mapped by the caller) in the exe's layout, for tests of the
/// units that catch: the object's destructor (0 for none) and its type chain, most derived first,
/// with no displacements. Everything but the `ThrowInfo` is in new blocks.
pub fn write_throw_info(e: &mut Engine, at: u32, destructor: u32, chain: &[&[u8]]) {
    let array = e.mem.alloc(4 + 4 * chain.len() as u32);
    e.mem.set_u32(array, chain.len() as u32);
    for (index, name) in chain.iter().enumerate() {
        let descriptor = e.mem.alloc(8 + name.len() as u32 + 1);
        e.mem.set_cstr(descriptor + 8, name);
        let catchable = e.mem.alloc(0x1c);
        e.mem.set_u32(catchable + 4, descriptor);
        e.mem.set_u32(catchable + 0xc, u32::MAX);
        e.mem.set_u32(array + 4 + 4 * index as u32, catchable);
    }
    e.mem.set_u32(at, 0);
    e.mem.set_u32(at + 4, destructor);
    e.mem.set_u32(at + 8, 0);
    e.mem.set_u32(at + 0xc, array);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::{args, Ret};

    /// A `ThrowInfo` of a `std::exception`-derived type with the destructor `destructor`.
    fn info(e: &mut Engine, destructor: u32) -> u32 {
        let at = e.mem.alloc(0x10);
        write_throw_info(
            e,
            at,
            destructor,
            &[b".?AVbad_alloc@std@@", STD_EXCEPTION_NAME],
        );
        at
    }

    thread_local! {
        static HOOK_CALLS: Cell<u32> = const { Cell::new(0) };
        static PROBING: Cell<bool> = const { Cell::new(false) };
    }

    /// Whether a panic on this thread reaches the counting hook now (another test's
    /// `static_init::run` swaps the hook out for a while, and can put back the one from before
    /// this hook was installed).
    fn probe_reaches_the_hook() -> bool {
        let before = HOOK_CALLS.with(Cell::get);
        PROBING.with(|probing| probing.set(true));
        let _ = catch_unwind(|| panic!("a probe of the panic hook"));
        PROBING.with(|probing| probing.set(false));
        HOOK_CALLS.with(Cell::get) > before
    }

    /// The panic hook calls `action` makes on this thread, whichever tests run beside it: the
    /// count stands only when a panic reached the hook on both sides of `action`.
    fn hook_calls_during(mut action: impl FnMut()) -> u32 {
        for _ in 0..10_000 {
            if probe_reaches_the_hook() {
                let before = HOOK_CALLS.with(Cell::get);
                action();
                let during = HOOK_CALLS.with(Cell::get) - before;
                if probe_reaches_the_hook() {
                    return during;
                }
            } else {
                let before = std::panic::take_hook();
                std::panic::set_hook(Box::new(move |info| {
                    HOOK_CALLS.with(|calls| calls.set(calls.get() + 1));
                    if !PROBING.with(Cell::get) {
                        before(info);
                    }
                }));
            }
            std::thread::yield_now();
        }
        panic!("the counting panic hook never stayed in place");
    }

    fn stop_text(payload: Box<dyn Any + Send>) -> String {
        payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default()
    }

    /// A translated function that throws from a nested call: `0x00de_0001` calls
    /// `0x00de_0002`, whose locals are on the guest stack.
    fn thrower(e: &mut Engine, info: u32) {
        e.register_double(0x00de_0002, move |e, _| {
            e.with_stack(0x100, |e, local| {
                e.mem.set_u32(local.addr(), 0x1234);
                throw(e, local.addr(), info);
            })
        });
        e.register(0x00de_0001, |e, _| e.call(0x00de_0002, &[]));
    }

    #[test]
    fn a_throw_reaches_the_handler_with_the_stacks_back() {
        let mut e = Engine::new();
        let info = info(&mut e, 0);
        thrower(&mut e, info);
        e.call_stack.push(0x0040_0000);
        let (object, chain_ok) = e.with_stack(0x10, |e, _| {
            let mark = e.guest_stack_mark();
            let depth = e.call_stack.len();
            let got = try_catch(
                e,
                |e| {
                    e.call(0x00de_0001, &[]);
                    unreachable!("the throw does not return")
                },
                |e, thrown| {
                    assert_eq!(thrown.throw_info, info);
                    // The object is still where the thrower built it, and the catch block's own
                    // locals go below it.
                    assert_eq!(e.mem.u32(thrown.object), 0x1234);
                    e.with_stack(0x200, |e, local| {
                        assert!(local.addr() < thrown.object);
                        assert_eq!(e.mem.u32(thrown.object), 0x1234);
                    });
                    assert_eq!(e.call_stack.len(), depth, "the catcher's call stack");
                    (
                        thrown.object,
                        caught_as(e, thrown, STD_EXCEPTION_NAME) == Some(thrown.object),
                    )
                },
            );
            assert_eq!(e.guest_stack_mark(), mark, "the guest stack is back");
            got
        });
        assert_ne!(object, 0);
        assert!(chain_ok);
        assert_eq!(e.call_stack, [0x0040_0000]);
    }

    #[test]
    fn a_caught_throw_never_reaches_the_panic_hook() {
        let mut e = Engine::new();
        let info = info(&mut e, 0);
        thrower(&mut e, info);
        let calls = hook_calls_during(|| {
            let reached = try_catch(
                &mut e,
                |e| {
                    e.call(0x00de_0001, &[]);
                    false
                },
                |_, _| true,
            );
            assert!(reached);
        });
        assert_eq!(calls, 0);
    }

    #[test]
    fn a_stop_does_reach_the_panic_hook() {
        // What the test above would see if a caught throw went the way of a stop.
        let calls = hook_calls_during(|| {
            let _ = catch_unwind(|| panic!("a stop"));
        });
        assert_eq!(calls, 1);
    }

    #[test]
    fn every_catch_is_counted_and_a_stop_passing_through_is_not() {
        let caught_here = || CAUGHT_HERE.with(Cell::get);
        let mut e = Engine::new();
        let info = info(&mut e, 0);
        thrower(&mut e, info);
        let (here, everyone) = (caught_here(), caught_throws());
        try_catch(
            &mut e,
            |e| {
                e.call(0x00de_0001, &[]);
            },
            |_, _| {},
        );
        assert_eq!(caught_here(), here + 1);
        assert!(caught_throws() > everyone);
        let here = caught_here();
        let stop = catch_unwind(AssertUnwindSafe(|| {
            try_catch(
                &mut e,
                |_| panic!("a stop"),
                |_, _: Thrown| panic!("the handler is for C++ throws"),
            )
        }));
        assert!(stop.is_err());
        assert_eq!(caught_here(), here);
    }

    #[test]
    fn the_line_for_a_catch_names_the_exception_and_where_with_no_guest_call() {
        let mut e = Engine::new();
        let info = info(&mut e, 0);
        // A `what` slot that would answer something else, were it called.
        e.register(0x00de_0015, |_, _| panic!("what() was called"));
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x00de_0015);
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"it broke");
        let object = e.mem.alloc(0xc);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, text);
        e.call_stack
            .extend_from_slice(&[0x0040_1000, 0x00b0_5420, 0x00de_0001, 0x00de_0002]);
        e.call_log = Some(vec![]);
        let thrown = Thrown {
            object,
            throw_info: info,
        };
        assert_eq!(
            caught_line(&e, 2, thrown),
            "C++ exception std::bad_alloc (a std::exception) \"it broke\" caught by 00b05420, \
             thrown in 00de0002 < 00de0001 < 00b05420 < 00401000"
        );
        assert!(e.call_log.as_ref().unwrap().is_empty());
        // Nothing readable is no fault, and a region the host opened has no catcher to name.
        e.call_stack.clear();
        let lost = Thrown {
            object: 4,
            throw_info: 8,
        };
        assert_eq!(
            caught_line(&e, 0, lost),
            "C++ exception (no type) caught by the host, thrown in "
        );
    }

    #[test]
    fn the_text_of_a_runtime_error_is_its_string() {
        let mut e = Engine::new();
        let chain: [&[u8]; 2] = [b".?AVruntime_error@std@@", STD_EXCEPTION_NAME];
        let at = e.mem.alloc(0x10);
        write_throw_info(&mut e, at, 0, &chain);
        let object = e.mem.alloc(0x28);
        let thrown = Thrown {
            object,
            throw_info: at,
        };
        // Inline below a capacity of 0x10; through the pointer from there on.
        e.mem.set_u32(object + 0x24, 0xf);
        e.mem.set_cstr(object + 0x10, b"short");
        assert_eq!(
            describe(&e, thrown),
            "std::runtime_error (a std::exception) \"short\""
        );
        let long = e.mem.alloc(32);
        e.mem.set_cstr(long, b"a longer message");
        e.mem.set_u32(object + 0x24, 0x1f);
        e.mem.set_u32(object + 0x10, long);
        assert_eq!(
            describe(&e, thrown),
            "std::runtime_error (a std::exception) \"a longer message\""
        );
    }

    #[test]
    fn a_nested_call_works_after_the_catch() {
        let mut e = Engine::new();
        let info = info(&mut e, 0);
        thrower(&mut e, info);
        e.register(0x00de_0010, |e, a| Ret {
            eax: a[0] + e.call_stack.len() as u32,
            ..Ret::default()
        });
        try_catch(
            &mut e,
            |e| {
                e.call(0x00de_0001, &[]);
            },
            |_, _| {},
        );
        // The call stack is empty again here, so the callee sees only itself.
        assert_eq!(e.call(0x00de_0010, &args![40u32]).eax, 41);
        // And a later throw is caught the same way: the region count came back.
        let again = try_catch(
            &mut e,
            |e| {
                e.call(0x00de_0001, &[]);
                0
            },
            |_, _| 7,
        );
        assert_eq!(again, 7);
    }

    #[test]
    fn a_throw_nothing_catches_is_a_stop_naming_the_type_and_the_message() {
        let mut e = Engine::new();
        let at = e.mem.alloc(0x10);
        write_throw_info(
            &mut e,
            at,
            0,
            &[
                b".?AVfailure@ios_base@std@@",
                b".?AVruntime_error@std@@",
                STD_EXCEPTION_NAME,
            ],
        );
        // A `runtime_error`'s string, and a `what` slot that stops if it is called.
        e.register(0x00de_0020, |_, _| panic!("what() was called"));
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x00de_0020);
        let object = e.mem.alloc(0x28);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 0x24, 0xf);
        e.mem.set_cstr(object + 0x10, b"it broke");
        let stop = catch_unwind(AssertUnwindSafe(|| throw(&mut e, object, at))).unwrap_err();
        let text = stop_text(stop);
        assert!(
            text.starts_with("uncaught C++ exception std::ios_base::failure"),
            "{text}"
        );
        assert!(
            text.contains("(a std::runtime_error, std::exception)"),
            "{text}"
        );
        assert!(text.contains("\"it broke\""), "{text}");
    }

    #[test]
    fn a_stop_passes_through_a_region_and_leaves_it_closed() {
        let mut e = Engine::new();
        e.register(0x00de_0031, |_, _| panic!("a stop"));
        e.register(0x00de_0032, |e, _| e.call(0x00de_0031, &[]));
        e.call_stack.push(0x0040_1000);
        let stop = catch_unwind(AssertUnwindSafe(|| {
            try_catch(
                &mut e,
                |e| {
                    e.call(0x00de_0032, &[]);
                },
                |_, _: Thrown| panic!("the handler is for C++ throws"),
            )
        }))
        .unwrap_err();
        assert_eq!(stop_text(stop), "a stop");
        // The call stack is as the stop left it: the host's "(in ...)" reads it.
        assert_eq!(e.call_stack, [0x0040_1000, 0x00de_0032, 0x00de_0031]);
        e.call_stack.clear();
        // No region is open any more: a throw now stops.
        let info = info(&mut e, 0);
        let object = e.mem.alloc(0xc);
        let stop = catch_unwind(AssertUnwindSafe(|| throw(&mut e, object, info))).unwrap_err();
        assert!(stop_text(stop).starts_with("uncaught"));
    }

    #[test]
    fn a_new_throw_out_of_a_catch_block_destroys_the_old_object_before_the_next_catch() {
        let mut e = Engine::new();
        let destroyed = std::rc::Rc::new(RefCell::new(Vec::new()));
        let log = destroyed.clone();
        e.register_double(0x00de_0035, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let info = info(&mut e, 0x00de_0035);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        let saw = destroyed.clone();
        e.call_stack.push(0x0040_1000);
        let caught = try_catch(
            &mut e,
            |e| {
                try_catch(
                    e,
                    |e| throw(e, first, info),
                    move |e, _| throw(e, second, info),
                )
            },
            move |e, thrown| {
                assert_eq!(*saw.borrow(), [first], "the old one is gone already");
                assert_eq!(e.call_stack, [0x0040_1000]);
                thrown.object
            },
        );
        assert_eq!(caught, second);
        assert_eq!(*destroyed.borrow(), [first, second]);
        e.call_stack.clear();
    }

    #[test]
    fn the_object_is_destroyed_once_when_the_catch_block_ends() {
        let mut e = Engine::new();
        let destroyed = std::rc::Rc::new(RefCell::new(Vec::new()));
        let log = destroyed.clone();
        e.register_double(0x00de_0040, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let info = info(&mut e, 0x00de_0040);
        let object = e.mem.alloc(4);
        let saw = destroyed.clone();
        try_catch(
            &mut e,
            |e| throw(e, object, info),
            move |_, thrown| {
                assert_eq!(thrown.object, object);
                assert!(saw.borrow().is_empty(), "alive while the block runs");
            },
        );
        assert_eq!(*destroyed.borrow(), [object]);
    }

    #[test]
    fn a_rethrow_from_a_catch_block_goes_to_the_next_region_and_keeps_the_object() {
        let mut e = Engine::new();
        let destroyed = std::rc::Rc::new(RefCell::new(0));
        let log = destroyed.clone();
        e.register_double(0x00de_0050, move |_, _| {
            *log.borrow_mut() += 1;
            Ret::default()
        });
        let info = info(&mut e, 0x00de_0050);
        let object = e.mem.alloc(4);
        let count = destroyed.clone();
        let outer = try_catch(
            &mut e,
            |e| {
                try_catch(
                    e,
                    |e| throw(e, object, info),
                    // `throw;` from a function the catch block calls (`ios_base::clear`).
                    |e, _| throw(e, 0, 0),
                )
            },
            move |_, thrown| {
                assert_eq!(*count.borrow(), 0, "not destroyed by the rethrowing block");
                thrown
            },
        );
        assert_eq!(
            outer,
            Thrown {
                object,
                throw_info: info
            }
        );
        assert_eq!(*destroyed.borrow(), 1);
    }

    #[test]
    fn a_rethrow_with_no_outer_region_stops_and_with_nothing_handled_stops() {
        let mut e = Engine::new();
        let stop = catch_unwind(AssertUnwindSafe(|| throw(&mut e, 0, 0))).unwrap_err();
        assert!(stop_text(stop).contains("no exception being handled"));
        let at = info(&mut e, 0);
        let object = e.mem.alloc(0xc);
        let stop = catch_unwind(AssertUnwindSafe(|| {
            try_catch(&mut e, |e| throw(e, object, at), |e, _| throw(e, 0, 0))
        }))
        .unwrap_err();
        assert!(stop_text(stop).starts_with("uncaught C++ exception std::bad_alloc"));
    }

    #[test]
    fn a_throw_inside_a_catch_block_can_be_caught_there_and_the_outer_one_goes_on() {
        let mut e = Engine::new();
        let destroyed = std::rc::Rc::new(RefCell::new(Vec::new()));
        let log = destroyed.clone();
        e.register_double(0x00de_0070, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let info = info(&mut e, 0x00de_0070);
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        try_catch(
            &mut e,
            |e| throw(e, first, info),
            move |e, outer| {
                try_catch(e, |e| throw(e, second, info), |_, _| {});
                // The second one is gone; `throw;` here would still be the first.
                assert_eq!(
                    HANDLING.with(|h| h.borrow().last().copied()),
                    Some(outer),
                    "the outer block's exception is current again"
                );
            },
        );
        assert_eq!(*destroyed.borrow(), [second, first]);
    }

    #[test]
    fn the_same_object_caught_inside_its_own_catch_block_is_destroyed_by_the_outer_one() {
        let mut e = Engine::new();
        let destroyed = std::rc::Rc::new(RefCell::new(0));
        let log = destroyed.clone();
        e.register_double(0x00de_0080, move |_, _| {
            *log.borrow_mut() += 1;
            Ret::default()
        });
        let info = info(&mut e, 0x00de_0080);
        let object = e.mem.alloc(4);
        let count = destroyed.clone();
        try_catch(
            &mut e,
            |e| throw(e, object, info),
            move |e, _| {
                try_catch(e, |e| throw(e, 0, 0), |_, _| {});
                assert_eq!(*count.borrow(), 0, "the outer block still has it");
            },
        );
        assert_eq!(*destroyed.borrow(), 1);
    }

    #[test]
    fn matching_tells_a_std_exception_from_anything_else() {
        let mut e = Engine::new();
        let at = e.mem.alloc(0x10);
        write_throw_info(&mut e, at, 0, &[b".?AVOther@@"]);
        let thrown = Thrown {
            object: 0x100,
            throw_info: at,
        };
        assert_eq!(caught_as(&e, thrown, STD_EXCEPTION_NAME), None);
        let std_info = info(&mut e, 0);
        let thrown = Thrown {
            object: 0x100,
            throw_info: std_info,
        };
        assert_eq!(caught_as(&e, thrown, STD_EXCEPTION_NAME), Some(0x100));
    }

    #[test]
    fn the_displacements_of_a_catchable_type_adjust_the_object() {
        let mut e = Engine::new();
        let at = e.mem.alloc(0x10);
        write_throw_info(&mut e, at, 0, &[STD_EXCEPTION_NAME]);
        let array = e.mem.u32(at + 0xc);
        let catchable = e.mem.u32(array + 4);
        // A member displacement.
        e.mem.set_u32(catchable + 8, 8);
        let object = e.mem.alloc(0x40);
        let thrown = Thrown {
            object,
            throw_info: at,
        };
        assert_eq!(caught_as(&e, thrown, STD_EXCEPTION_NAME), Some(object + 8));
        // A virtual base: the table at object + 4, its entry at + 0xc.
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table + 0xc, 0x20);
        e.mem.set_u32(object + 4, table);
        e.mem.set_u32(catchable + 0xc, 4);
        e.mem.set_u32(catchable + 0x10, 0xc);
        assert_eq!(
            caught_as(&e, thrown, STD_EXCEPTION_NAME),
            Some(object + 8 + 4 + 0x20)
        );
    }

    #[test]
    fn the_raise_exception_import_throws_for_a_cxx_record_and_stops_for_other_codes() {
        let raise = 0x00fd_f154;
        let mut e = Engine::new();
        let imports = [crate::exe::Import {
            slot: raise,
            dll: "KERNEL32.dll".into(),
            name: "RaiseException".into(),
        }];
        e.register_imports(&imports);
        let info = info(&mut e, 0);
        let object = e.mem.alloc(4);
        let record = e.mem.alloc(0xc);
        e.mem.set_u32(record, 0x1993_0520);
        e.mem.set_u32(record + 4, object);
        e.mem.set_u32(record + 8, info);
        let got = try_catch(
            &mut e,
            |e| {
                e.call(raise, &args![CXX_EXCEPTION_CODE, 1u32, 3u32, record]);
                panic!("RaiseException with a throw does not return");
            },
            |_, thrown| thrown,
        );
        assert_eq!(
            got,
            Thrown {
                object,
                throw_info: info
            }
        );
        // The thread-naming code comes back at once, inside a region or not.
        e.call(raise, &args![0x406d_1388u32, 0u32, 4u32, record]);
        try_catch(
            &mut e,
            |e| {
                e.call(raise, &args![0x406d_1388u32, 0u32, 4u32, record]);
            },
            |_, _| panic!("a thread name is not a throw"),
        );
        // Any other code stops, and a catch(...) region does not take it (`/EHs`).
        let stop = catch_unwind(AssertUnwindSafe(|| {
            try_catch(
                &mut e,
                |e| {
                    e.call(raise, &args![0xc000_0005u32, 0u32, 0u32, 0u32]);
                },
                |_, _| panic!("catch(...) never takes a structured exception"),
            )
        }))
        .unwrap_err();
        assert!(stop_text(stop).contains("0xc0000005"));
        // A wrong magic number is not a throw either.
        e.mem.set_u32(record, 0x1234);
        let stop = catch_unwind(AssertUnwindSafe(|| {
            e.call(raise, &args![CXX_EXCEPTION_CODE, 1u32, 3u32, record]);
        }))
        .unwrap_err();
        assert!(stop_text(stop).contains("0xe06d7363"));
    }

    /// The cost of a try region around a call against the same call without one, in nanoseconds
    /// per call; `--ignored --nocapture` prints it. Run as `cargo test --release` for a number
    /// that means anything; the measurement of record is in ADR-0013.
    #[test]
    #[ignore = "a measurement, run by hand"]
    fn try_region_speed() {
        let mut e = Engine::new();
        e.register(0x00de_00a0, |e, a| {
            let word = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], word.wrapping_add(1));
            Ret::default()
        });
        let word = e.mem.alloc(4);
        const CALLS: u32 = 50_000_000;
        for _ in 0..3 {
            let start = std::time::Instant::now();
            for _ in 0..CALLS {
                std::hint::black_box(e.call(0x00de_00a0, &[word]));
            }
            let plain = start.elapsed().as_secs_f64() * 1e9 / f64::from(CALLS);
            let start = std::time::Instant::now();
            for _ in 0..CALLS {
                try_catch(
                    &mut e,
                    |e| {
                        std::hint::black_box(e.call(0x00de_00a0, &[word]));
                    },
                    |_, _| {},
                );
            }
            let guarded = start.elapsed().as_secs_f64() * 1e9 / f64::from(CALLS);
            println!(
                "a call: {plain:.2} ns; in a try region: {guarded:.2} ns; the region: {:.2} ns",
                guarded - plain
            );
        }
    }
}
