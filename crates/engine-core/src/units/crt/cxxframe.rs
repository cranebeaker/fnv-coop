//! More C runtime pieces (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525),
//! translated from the exe: the C++ exception-handling frame code of LIBCMT/frame.cpp and
//! ehvecdtr.cpp. `__ArrayUnwind` (`00ec5f70`) with its `__except` filter (`00ec5f93`),
//! `___FrameUnwindFilter` (`00ece381`), `___FrameUnwindToState` (`00ece3ca`), the
//! unexpected-exception call `00ece5fc` with its `catch(...)` block (`00ece62d`),
//! `CallCatchBlock` (`00ece645`) with its `__except` handler (`00ece6de`) and `__finally`
//! body (`00ece76b`), `___BuildCatchObjectHelper` (`00ece7e1`), `___BuildCatchObject`
//! (`00ece960`), `CatchIt` (`00ece9f2`), `FindHandlerForForeignException` (`00ecea60`),
//! `FindHandler` (`00eceb54`), `___InternalCxxFrameHandler` (`00eceeb8`) and
//! `_CallDestructExceptionObject` (`00edf49f`).
//!
//! The structures are the compiler's: the function information (`+0` magic number, `+4` the
//! highest state, `+8` the unwind map of 8-byte entries `{to state, action}`, `+0xC` the count
//! of try blocks, `+0x10` the try map of 0x14-byte entries `{try low, try high, catch high,
//! catch count, handlers}`, `+0x1C` the exception specification, `+0x20` flags), the handlers
//! (0x10 bytes: adjectives, type, catch object's displacement, handler address), the
//! registration node (`+8` the state) and the thread's data (`_getptd`: `+0x80` the
//! translator, `+0x88` the current exception, `+0x8C` its context, `+0x90`
//! `_ProcessingThrow`, `+0x94` the current exception specification, `+0x20C` the rethrow
//! flag).
//!
//! The engine does not model the structured-exception chain at `FS:[0]`, the dispatch of an
//! exception to it, or the jumps that reset `ESP` and `EBP`; each function says what of that it
//! leaves out. Register inputs come first as words. Every call is made by address, as the exe
//! makes it, so a test can double each one. After `terminate` and `_inconsistency`, which do not
//! return in the exe, the instructions after the call follow, as the CPU would run them; where
//! a call the exe never comes back from would fall into other code (`_CxxThrowException`, the
//! unexpected call, `terminate` at the end of a block), the run stops if it returns.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_getptd` (`00ed11de`), `terminate` (`00ed14b4`), `unexpected` (`00ed14ed`) and
/// `_inconsistency` (`00ed1500`), cdecl, no arguments; `_CxxThrowException` (`00ec7894`,
/// stdcall: the object, the throw information).
const GETPTD: u32 = 0x00ed_11de;
const TERMINATE: u32 = 0x00ed_14b4;
const UNEXPECTED: u32 = 0x00ed_14ed;
const INCONSISTENCY: u32 = 0x00ed_1500;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `_CallSettingFrame@12` (`00ed1550`, stdcall: the funclet, the registration node, the
/// non-local-goto code), the `__finally` block of `___FrameUnwindToState` (`00ece490`, no
/// arguments), `__CreateFrameInfo` (`00ec3f87`: frame, object), `__FindAndUnlinkFrame`
/// (`00ec3fda`), `__IsExceptionObjectToBeDestroyed` (`00ec3fb3`), `__DestructExceptionObject`
/// (`00ece4f0`: record, throw flag) and `_CallCatchBlock2` (`00ec402c`: frame, function
/// information, handler, nesting, non-local-goto code).
const CALL_SETTING_FRAME: u32 = 0x00ed_1550;
const FRAME_UNWIND_TO_STATE_FINALLY: u32 = 0x00ec_e490;
const CREATE_FRAME_INFO: u32 = 0x00ec_3f87;
const FIND_AND_UNLINK_FRAME: u32 = 0x00ec_3fda;
const IS_EXCEPTION_OBJECT_TO_BE_DESTROYED: u32 = 0x00ec_3fb3;
const DESTRUCT_EXCEPTION_OBJECT: u32 = 0x00ec_e4f0;
const CALL_CATCH_BLOCK2: u32 = 0x00ec_402c;
/// `_ValidateRead` (`00edfbe8`: address, size), `___AdjustPointer` (`00ece545`: object, the
/// displacement record), `memmove` (`00ec7230`: destination, source, count) and
/// `_CallMemberFunction1` / `2` (`00ec3cd8`: `this`, the member function, then its words).
const VALIDATE_READ: u32 = 0x00ed_fbe8;
const ADJUST_POINTER: u32 = 0x00ec_e545;
const MEMMOVE: u32 = 0x00ec_7230;
const CALL_MEMBER_FUNCTION: u32 = 0x00ec_3cd8;
/// `_UnwindNestedFrames` (`00ec3cdf`, stdcall: frame, record), `_JumpToContinuation`
/// (`00ec3ca6`, stdcall: continuation, frame), `__EncodedNull` (`00ed0f89`),
/// `_CallSETranslator` (`00ec3d9c`, seven words), `_GetRangeOfTrysToCheck` (`00ec3f12`:
/// function information, nesting, state, start out, end out), `___TypeMatch` (`00ece322`:
/// handler, catchable type, throw information), `IsInExceptionSpec` (`00ece581`: the
/// specification as `EDI`, then the record), `type_info::operator==` (`00ec40cc`, thiscall,
/// one word) and `std::bad_exception::bad_exception(const char *)` (`00ece2d2`, thiscall, one
/// word).
const UNWIND_NESTED_FRAMES: u32 = 0x00ec_3cdf;
const JUMP_TO_CONTINUATION: u32 = 0x00ec_3ca6;
const ENCODED_NULL: u32 = 0x00ed_0f89;
const CALL_SE_TRANSLATOR: u32 = 0x00ec_3d9c;
const GET_RANGE_OF_TRYS_TO_CHECK: u32 = 0x00ec_3f12;
const TYPE_MATCH: u32 = 0x00ec_e322;
const IS_IN_EXCEPTION_SPEC: u32 = 0x00ec_e581;
const TYPE_INFO_EQUAL: u32 = 0x00ec_40cc;
const BAD_EXCEPTION_FROM_TEXT: u32 = 0x00ec_e2d2;
/// `_IsNonwritableInCurrentImage` (`00eddcc0`: an address).
const IS_NONWRITABLE_IN_CURRENT_IMAGE: u32 = 0x00ed_dcc0;
/// This file's functions, called by address as the exe calls them.
const FRAME_UNWIND_TO_STATE: u32 = 0x00ec_e3ca;
const CALL_UNEXPECTED: u32 = 0x00ec_e5fc;
const CALL_CATCH_BLOCK: u32 = 0x00ec_e645;
const CALL_CATCH_BLOCK_FINALLY: u32 = 0x00ec_e76b;
const BUILD_CATCH_OBJECT_HELPER: u32 = 0x00ec_e7e1;
const BUILD_CATCH_OBJECT: u32 = 0x00ec_e960;
const CATCH_IT: u32 = 0x00ec_e9f2;
const FIND_HANDLER_FOR_FOREIGN_EXCEPTION: u32 = 0x00ec_ea60;
const FIND_HANDLER: u32 = 0x00ec_eb54;
/// The pointer to `__DestructExceptionObject` that `_CallDestructExceptionObject` calls
/// through (`010f20a0`), `typeid(std::bad_exception)` (`011c1c8c`), the text
/// `"bad exception"` (`010f20b0`) and `std::bad_exception`'s throw information (`0118011c`).
const DESTRUCT_POINTER: u32 = 0x010f_20a0;
const BAD_EXCEPTION_TYPE: u32 = 0x011c_1c8c;
const BAD_EXCEPTION_TEXT: u32 = 0x010f_20b0;
const BAD_EXCEPTION_THROW_INFO: u32 = 0x0118_011c;
/// The MSVC C++ exception code (`'msc' | 0xE0000000`), the COM+ one (`0xE0434F4D`), a
/// breakpoint (`0x80000003`), `STATUS_UNWIND_CONSOLIDATE` (`0x80000026`) and the throw
/// magic numbers.
const CXX_EXCEPTION: u32 = 0xe06d_7363;
const COMPLUS_EXCEPTION: u32 = 0xe043_4f4d;
const BREAKPOINT: u32 = 0x8000_0003;
const UNWIND_CONSOLIDATE: u32 = 0x8000_0026;
const MAGIC_1: u32 = 0x1993_0520;
const MAGIC_2: u32 = 0x1993_0521;
const MAGIC_3: u32 = 0x1993_0522;
/// Fields of the thread's data (`_getptd`).
const PTD_TRANSLATOR: u32 = 0x80;
const PTD_CUR_EXCEPTION: u32 = 0x88;
const PTD_CUR_CONTEXT: u32 = 0x8c;
const PTD_PROCESSING_THROW: u32 = 0x90;
const PTD_CUR_EXC_SPEC: u32 = 0x94;
const PTD_RETHROW: u32 = 0x20c;

fn ptd(e: &mut Engine) -> u32 {
    e.call(GETPTD, &args![]).u32()
}

/// A C++ exception record of a throw (code 0xE06D7363, three parameters, a known magic
/// number), whatever its throw information.
fn is_cxx_throw(e: &Engine, record: u32) -> bool {
    if e.mem.u32(record) != CXX_EXCEPTION || e.mem.u32(record.wrapping_add(0x10)) != 3 {
        return false;
    }
    let magic = e.mem.u32(record.wrapping_add(0x14));
    magic == MAGIC_1 || magic == MAGIC_2 || magic == MAGIC_3
}

/// The registration node's state (`+8`): its low byte sign-extended when the function
/// information's highest state (`+4`) is at most 0x80 (signed), else the whole word.
fn node_state(e: &Engine, node: u32, info: u32) -> i32 {
    if e.mem.i32(info.wrapping_add(4)) > 0x80 {
        e.mem.i32(node.wrapping_add(8))
    } else {
        e.mem.u8(node.wrapping_add(8)) as i8 as i32
    }
}

// Translated from 00ec5f70 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec5f70 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117fc88) is not translated: its __except filter (00ec5f93, array_unwind_filter) and handler (00ec5fbc) run on an exception raised by a destructor, which the engine does not dispatch.
/// `__ArrayUnwind` (LIBCMT/ehvecdtr.cpp; stdcall: the end of the constructed part, the element
/// size, the count, the destructor; `RET 0x10`): while the count, counted down first, is not
/// negative, the address steps back one element and the destructor runs on it (thiscall, the
/// element in `ECX`).
pub fn array_unwind(e: &mut Engine, array: u32, size: u32, count: i32, destructor: u32) {
    let mut array = array;
    let mut count = count;
    loop {
        count = count.wrapping_sub(1);
        if count < 0 {
            break;
        }
        array = array.wrapping_sub(size);
        e.call(destructor, &args![array]);
    }
}

// Translated from 00ec5f93 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` filter of `__ArrayUnwind` (`00ec5f70`; entered with `EBP` = its frame, its
/// one word, as `seh.rs`'s dispatchers enter funclets): the exception pointers (`[EBP-0x14]`)
/// are kept in `[EBP-0x1C]` and their record in `[EBP-0x20]`; a C++ exception (the record's
/// code 0xE06D7363) calls `terminate`, which does not return (the run stops if it does); any
/// other stores 0 (`EXCEPTION_CONTINUE_SEARCH`) in `[EBP-0x24]` and answers it.
pub fn array_unwind_filter(e: &mut Engine, ebp: u32) -> u32 {
    let pointers = e.mem.u32(ebp.wrapping_sub(0x14));
    e.mem.set_u32(ebp.wrapping_sub(0x1c), pointers);
    let record = e.mem.u32(pointers);
    e.mem.set_u32(ebp.wrapping_sub(0x20), record);
    if e.mem.u32(record) == CXX_EXCEPTION {
        e.call(TERMINATE, &args![]);
        panic!("00ec5f93: terminate returned");
    }
    e.mem.set_u32(ebp.wrapping_sub(0x24), 0);
    e.mem.u32(ebp.wrapping_sub(0x24))
}

// Translated from 00ece381 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___FrameUnwindFilter` (cdecl: the exception pointers), the filter of
/// `___FrameUnwindToState`'s guarded call. A COM+ exception (0xE0434F4D) counts the thread's
/// `_ProcessingThrow` (+0x90) down when it is above 0 (signed) and answers 0; a C++ exception
/// clears it and jumps to `terminate` (its answer passed on); anything else answers 0
/// (`EXCEPTION_CONTINUE_SEARCH`).
pub fn frame_unwind_filter(e: &mut Engine, pointers: u32) -> u32 {
    let code = e.mem.u32(e.mem.u32(pointers));
    if code == COMPLUS_EXCEPTION {
        let data = ptd(e);
        if e.mem.i32(data.wrapping_add(PTD_PROCESSING_THROW)) > 0 {
            let at = ptd(e).wrapping_add(PTD_PROCESSING_THROW);
            let n = e.mem.u32(at).wrapping_sub(1);
            e.mem.set_u32(at, n);
        }
    } else if code == CXX_EXCEPTION {
        let data = ptd(e);
        e.mem.set_u32(data.wrapping_add(PTD_PROCESSING_THROW), 0);
        return e.call(TERMINATE, &args![]).u32();
    }
    0
}

// Translated from 00ece3ca (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ece3ca - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180018) is not translated: the __except around each unwind action (filter 00ece44c, ___FrameUnwindFilter; handler 00ece456, which goes on with the next state) runs on an exception raised by an action, which the engine does not dispatch; the __finally block 00ece490 is called as the exe calls it on the way out.
/// `___FrameUnwindToState` (cdecl: registration node, dispatcher context, function
/// information, target state). The node's state ([`node_state`]); the thread's
/// `_ProcessingThrow` (+0x90) is counted up. Until the state is the target: a state below 0 or
/// not below the highest state calls `_inconsistency`; the unwind map's entry for it gives the
/// next state, and when it has an action the node's state becomes the next one and the action
/// runs through `_CallSettingFrame(action, node, 0x103)`. Then the `__finally` block
/// (`00ece490`), `_inconsistency` when the state is not the target, and the node's state
/// (`+8`) becomes the state.
pub fn frame_unwind_to_state(e: &mut Engine, node: u32, _dispatcher: u32, info: u32, target: i32) {
    let mut state = node_state(e, node, info);
    let at = ptd(e).wrapping_add(PTD_PROCESSING_THROW);
    let n = e.mem.u32(at).wrapping_add(1);
    e.mem.set_u32(at, n);
    while state != target {
        if state <= -1 || state >= e.mem.i32(info.wrapping_add(4)) {
            e.call(INCONSISTENCY, &args![]);
        }
        let offset = (state as u32) << 3;
        let entry = e.mem.u32(info.wrapping_add(8)).wrapping_add(offset);
        let next = e.mem.i32(entry);
        if e.mem.u32(entry.wrapping_add(4)) != 0 {
            e.mem.set_u32(node.wrapping_add(8), next as u32);
            let map = e.mem.u32(info.wrapping_add(8));
            let action = e.mem.u32(map.wrapping_add(offset).wrapping_add(4));
            e.call(CALL_SETTING_FRAME, &args![action, node, 0x103u32]);
        }
        state = next;
    }
    e.call(FRAME_UNWIND_TO_STATE_FINALLY, &args![]);
    if state != target {
        e.call(INCONSISTENCY, &args![]);
    }
    e.mem.set_u32(node.wrapping_add(8), state as u32);
}

// Translated from 00ece5fc (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ece5fc - the C++ exception frame (__EH_prolog3_catch 00ece288 with the unwind table 00f34e90: the FS:[0] chain and the state word) is not translated, so its catch(...) block (00ece62d, call_unexpected_catch) does not run on an exception thrown by unexpected; the engine dispatches no C++ exceptions.
/// No Ghidra name (the shape of LIBCMT's `CallUnexpected`; cdecl: the exception
/// specification), called by `FindHandler` for an exception its function's specification does
/// not allow: `_inconsistency` when the thread already has a current specification (+0x94),
/// then `unexpected` and `terminate`. Neither returns in the exe; if they do, the run stops.
pub fn call_unexpected(e: &mut Engine, _spec: u32) {
    let data = ptd(e);
    if e.mem.u32(data.wrapping_add(PTD_CUR_EXC_SPEC)) != 0 {
        e.call(INCONSISTENCY, &args![]);
    }
    e.call(UNEXPECTED, &args![]);
    e.call(TERMINATE, &args![]);
    panic!("00ece5fc: terminate returned");
}

// Translated from 00ece62d (disassembled, FalloutNV.exe 1.4.0.525)
/// The `catch(...)` block of `00ece5fc` (entered with `EBP` = its frame, its one word): the
/// specification (`[EBP+8]`, the function's argument) becomes the thread's current one (+0x94)
/// and the exception is thrown again (`_CxxThrowException(0, 0)`), which does not return (an
/// `INT3` follows; the run stops if it does).
pub fn call_unexpected_catch(e: &mut Engine, ebp: u32) {
    let data = ptd(e);
    let spec = e.mem.u32(ebp.wrapping_add(8));
    e.mem.set_u32(data.wrapping_add(PTD_CUR_EXC_SPEC), spec);
    e.call(CXX_THROW_EXCEPTION, &args![0u32, 0u32]);
    panic!("00ece62d: _CxxThrowException returned");
}

// Translated from 00ece645 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ece645 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 011800b8) is not translated: its __except (filter 00ece6d5, the rethrow filter 00ece4ab; handler 00ece6de, call_catch_block_rethrown) runs on an exception the catch block raises, and its __finally entry 00ece765 (which reloads EDI and ESI before 00ece76b) on an unwind; the engine dispatches neither.
/// `CallCatchBlock` (LIBCMT/frame.cpp; the handler address in `ECX`, then cdecl: record,
/// registration node, context, function information, catch depth, non-local-goto code). The
/// word below the node (`[node-4]`) is kept; a frame information for the thrown object
/// (`record + 0x18`) is made on the stack (`__CreateFrameInfo`); the thread's current
/// exception and context (+0x88, +0x8C) are kept and replaced by the record and the context;
/// `_CallCatchBlock2(node, function information, handler, catch depth, code)` runs the catch
/// block and answers the continuation address. The `__finally` body (`00ece76b`) then puts
/// everything back (abnormal termination 0). Answers the continuation.
#[allow(clippy::too_many_arguments)]
pub fn call_catch_block(
    e: &mut Engine,
    handler: u32,
    record: u32,
    node: u32,
    context: u32,
    info: u32,
    depth: u32,
    code: u32,
) -> u32 {
    e.with_stack(8, |e, frame| {
        let saved = e.mem.u32(node.wrapping_sub(4));
        let object = e.mem.u32(record.wrapping_add(0x18));
        let frame_info = e
            .call(CREATE_FRAME_INFO, &args![frame.addr(), object])
            .u32();
        let data = ptd(e);
        let old_exception = e.mem.u32(data.wrapping_add(PTD_CUR_EXCEPTION));
        let data = ptd(e);
        let old_context = e.mem.u32(data.wrapping_add(PTD_CUR_CONTEXT));
        let data = ptd(e);
        e.mem.set_u32(data.wrapping_add(PTD_CUR_EXCEPTION), record);
        let data = ptd(e);
        e.mem.set_u32(data.wrapping_add(PTD_CUR_CONTEXT), context);
        let continuation = e
            .call(CALL_CATCH_BLOCK2, &args![node, info, handler, depth, code])
            .u32();
        e.call(
            CALL_CATCH_BLOCK_FINALLY,
            &args![
                node,
                record,
                saved,
                frame_info,
                old_exception,
                old_context,
                0u32,
                continuation,
                0u32
            ],
        );
        continuation
    })
}

// Translated from 00ece6de (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` block of `CallCatchBlock` (`00ece645`; entered with `EBP` = its frame, its
/// one word, as `_EH4_TransferToHandler` enters it), run when the catch block rethrew. It takes
/// back the stack the frame kept (`[EBP-0x18]`), clears the thread's rethrow flag (+0x20C) and
/// unwinds the registration node (`[EBP+0xC]`) with the function information (`[EBP+0x14]`):
/// `___FrameUnwindToState(node, 0, info, to)` from the node's state ([`node_state`]) to `to`:
/// for the first try block (in the try map's order, the index kept in `[EBP-0x20]`) whose
/// catch range holds the state (`try high < state <= catch high`, signed), the `to state` of
/// the unwind map's entry after `try high`; else the state itself. Then the continuation
/// (`[EBP-0x1C]`) becomes 0, the try level goes to 0 and then -2, the abnormal-termination flag
/// (`[EBP+0x10]`) to 0, and the `__finally` body (`00ece76b`) runs with the node (`EDI`), the
/// record (`ESI`, `[EBP+8]`) and its frame words; it leaves with the continuation (0) through
/// `__SEH_epilog4`, returning from `CallCatchBlock`.
pub fn call_catch_block_rethrown(e: &mut Engine, ebp: u32) -> u32 {
    let mut r = super::seh::block_regs(e, ebp);
    let data = ptd(e);
    e.mem.set_u32(data.wrapping_add(PTD_RETHROW), 0);
    let info = e.mem.u32(ebp.wrapping_add(0x14));
    let node = e.mem.u32(ebp.wrapping_add(0xc));
    let mut to = node_state(e, node, info);
    let map = e.mem.u32(info.wrapping_add(0x10));
    let index_slot = ebp.wrapping_sub(0x20);
    e.mem.set_u32(index_slot, 0);
    loop {
        let index = e.mem.u32(index_slot);
        if index >= e.mem.u32(info.wrapping_add(0xc)) {
            break;
        }
        let entry = index.wrapping_mul(0x14).wrapping_add(map);
        let high = e.mem.i32(entry.wrapping_add(4));
        if to > high && to <= e.mem.i32(entry.wrapping_add(8)) {
            let unwind = e.mem.u32(info.wrapping_add(8));
            let next = unwind
                .wrapping_add((high as u32).wrapping_mul(8))
                .wrapping_add(8);
            to = e.mem.i32(next);
            break;
        }
        e.mem.set_u32(index_slot, index.wrapping_add(1));
    }
    e.call(FRAME_UNWIND_TO_STATE, &args![node, 0u32, info, to]);
    e.mem.set_u32(ebp.wrapping_sub(0x1c), 0);
    e.mem.set_u32(ebp.wrapping_sub(4), 0);
    let record = e.mem.u32(ebp.wrapping_add(8));
    e.mem.set_u32(ebp.wrapping_sub(4), 0xffff_fffe);
    e.mem.set_u32(ebp.wrapping_add(0x10), 0);
    let frame_word = |e: &Engine, offset: u32| e.mem.u32(ebp.wrapping_sub(offset));
    let words = [
        node,
        record,
        frame_word(e, 0x24),
        frame_word(e, 0x28),
        frame_word(e, 0x2c),
        frame_word(e, 0x30),
        frame_word(e, 0x34),
        frame_word(e, 0x1c),
        e.mem.u32(ebp.wrapping_add(0x10)),
    ];
    e.call(CALL_CATCH_BLOCK_FINALLY, &words);
    r.eax = e.mem.u32(ebp.wrapping_sub(0x1c));
    super::seh::leave_block(e, &mut r, 0x00ec_e75f, 0)
}

// Translated from 00ece76b (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` body of `CallCatchBlock` (`00ece645`; `EDI` the registration node, `ESI`
/// the record, then its frame words in the order it reads them: the kept word `[EBP-0x24]`,
/// the frame information `[EBP-0x28]`, the kept exception and context `[EBP-0x2C]` and
/// `[EBP-0x30]`, the destroyed flag `[EBP-0x34]`, the continuation `[EBP-0x1C]` and the
/// abnormal-termination flag `[EBP+0x10]`): the word below the node is put back, the frame
/// information unlinked (`__FindAndUnlinkFrame`) and the thread's exception and context
/// restored. For a C++ throw with the flag clear and a continuation, an object that no other
/// frame information names (`__IsExceptionObjectToBeDestroyed`) is destroyed
/// (`__DestructExceptionObject(record, abnormal)`).
#[allow(clippy::too_many_arguments)]
pub fn call_catch_block_finally(
    e: &mut Engine,
    node: u32,
    record: u32,
    saved: u32,
    frame_info: u32,
    old_exception: u32,
    old_context: u32,
    destroyed: u32,
    continuation: u32,
    abnormal: u32,
) {
    e.mem.set_u32(node.wrapping_sub(4), saved);
    e.call(FIND_AND_UNLINK_FRAME, &args![frame_info]);
    let data = ptd(e);
    e.mem
        .set_u32(data.wrapping_add(PTD_CUR_EXCEPTION), old_exception);
    let data = ptd(e);
    e.mem
        .set_u32(data.wrapping_add(PTD_CUR_CONTEXT), old_context);
    if !is_cxx_throw(e, record) || destroyed != 0 || continuation == 0 {
        return;
    }
    let object = e.mem.u32(record.wrapping_add(0x18));
    if e.call(IS_EXCEPTION_OBJECT_TO_BE_DESTROYED, &args![object])
        .u32()
        != 0
    {
        e.call(DESTRUCT_EXCEPTION_OBJECT, &args![record, abnormal]);
    }
}

// Translated from 00ece7e1 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ece7e1 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 011800e0) is not translated: its __except (filter 00ece94c, always 1; handler 00ece950, terminate) runs on an exception raised while copying, which the engine does not dispatch.
/// `___BuildCatchObjectHelper` (cdecl: record, registration node, handler, catchable type).
/// Answers 0 unless the handler names a type (+4) with a name (its byte +8 not 0) and either a
/// catch object displacement (+8) or the adjective bit 31. The catch object is the node
/// itself for adjective bit 31, else `node + displacement + 0xC`. The thrown object
/// (`record + 0x18`) and the catch object are checked (`_ValidateRead(x, 1)`; a failure calls
/// `_inconsistency` and answers 0). A reference (adjective 8) stores the thrown object
/// adjusted (`___AdjustPointer` with the catchable type + 8). A simple type (catchable
/// property 1) is copied (`memmove` of its size, +0x14), a 4-byte non-null pointer then
/// adjusted in place. Without a copy constructor (+0x18) the adjusted object is copied. With
/// one (also checked: `_ValidateRead` with one word pushed, which reads only the address) the
/// answer is 1, or 2 for a virtual base (property 4), for the caller to run it.
pub fn build_catch_object_helper(
    e: &mut Engine,
    record: u32,
    node: u32,
    handler: u32,
    catchable: u32,
) -> u32 {
    let handler_type = e.mem.u32(handler.wrapping_add(4));
    if handler_type == 0 || e.mem.u8(handler_type.wrapping_add(8)) == 0 {
        return 0;
    }
    let displacement = e.mem.u32(handler.wrapping_add(8));
    let adjectives = e.mem.u32(handler);
    if displacement == 0 && adjectives & 0x8000_0000 == 0 {
        return 0;
    }
    let target = if (adjectives as i32) < 0 {
        node
    } else {
        displacement.wrapping_add(node).wrapping_add(0xc)
    };
    let valid =
        |e: &mut Engine, address: u32| e.call(VALIDATE_READ, &args![address, 1u32]).u32() != 0;
    let object = |e: &Engine| e.mem.u32(record.wrapping_add(0x18));
    let adjust = |e: &mut Engine, pointer: u32| {
        e.call(ADJUST_POINTER, &args![pointer, catchable.wrapping_add(8)])
            .u32()
    };
    let thrown = object(e);
    if !valid(e, thrown) || !valid(e, target) {
        e.call(INCONSISTENCY, &args![]);
        return 0;
    }
    if adjectives & 8 != 0 {
        let thrown = object(e);
        e.mem.set_u32(target, thrown);
        let adjusted = adjust(e, thrown);
        e.mem.set_u32(target, adjusted);
        return 0;
    }
    if e.mem.u8(catchable) & 1 != 0 {
        let size = e.mem.u32(catchable.wrapping_add(0x14));
        let thrown = object(e);
        e.call(MEMMOVE, &args![target, thrown, size]);
        if e.mem.u32(catchable.wrapping_add(0x14)) == 4 {
            let pointer = e.mem.u32(target);
            if pointer != 0 {
                let adjusted = adjust(e, pointer);
                e.mem.set_u32(target, adjusted);
            }
        }
        return 0;
    }
    if e.mem.u32(catchable.wrapping_add(0x18)) == 0 {
        let size = e.mem.u32(catchable.wrapping_add(0x14));
        let thrown = object(e);
        let adjusted = adjust(e, thrown);
        e.call(MEMMOVE, &args![target, adjusted, size]);
        return 0;
    }
    let copy = e.mem.u32(catchable.wrapping_add(0x18));
    if !valid(e, copy) {
        e.call(INCONSISTENCY, &args![]);
        return 0;
    }
    if e.mem.u8(catchable) & 4 != 0 {
        2
    } else {
        1
    }
}

// Translated from 00ece960 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ece960 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180100) is not translated: its __except (filter 00ece9e5, always 1; handler 00ece9e9, terminate) runs on an exception the copy constructor raises, which the engine does not dispatch.
/// `___BuildCatchObject` (cdecl: record, registration node, handler, catchable type): the
/// catch object is the node for the handler's adjective bit 31, else `node + displacement
/// (+8) + 0xC`. `___BuildCatchObjectHelper` decides; 1 runs the copy constructor (+0x18) on the
/// catch object with the adjusted thrown object (`_CallMemberFunction1`), 2 with the adjusted
/// object and 1 (`_CallMemberFunction2`, a virtual base); otherwise nothing more.
pub fn build_catch_object(e: &mut Engine, record: u32, node: u32, handler: u32, catchable: u32) {
    let target = if e.mem.u32(handler) & 0x8000_0000 != 0 {
        node
    } else {
        e.mem
            .u32(handler.wrapping_add(8))
            .wrapping_add(node)
            .wrapping_add(0xc)
    };
    let kind = e
        .call(
            BUILD_CATCH_OBJECT_HELPER,
            &args![record, node, handler, catchable],
        )
        .u32();
    if kind != 1 && kind != 2 {
        return;
    }
    let thrown = e.mem.u32(record.wrapping_add(0x18));
    let adjusted = e
        .call(ADJUST_POINTER, &args![thrown, catchable.wrapping_add(8)])
        .u32();
    let copy = e.mem.u32(catchable.wrapping_add(0x18));
    if kind == 1 {
        e.call(CALL_MEMBER_FUNCTION, &args![target, copy, adjusted]);
    } else {
        e.call(CALL_MEMBER_FUNCTION, &args![target, copy, adjusted, 1u32]);
    }
}

// Translated from 00ece9f2 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ece9f2 - _JumpToContinuation (00ec3ca6) does not return in the exe (it unlinks FS:[0] and jumps to the continuation with the frame's ESP and EBP); the engine calls it and goes on, as after any call.
/// `CatchIt` (LIBCMT/frame.cpp; `ESI` the registration node, `EBX` the handler, `EDI` the try
/// block, then cdecl: record, context, dispatcher context, function information, catchable
/// type, catch depth, marker node). With a catchable type the catch object is built
/// (`___BuildCatchObject`); the frames above are unwound (`_UnwindNestedFrames` to the marker
/// node, or the node when there is none), then this frame to the try block's low state
/// (`___FrameUnwindToState`); the node's state becomes the try block's high state + 1; the
/// catch block runs (`CallCatchBlock` with the handler's address, +0xC, in `ECX` and code
/// 0x100), and a continuation it answers is jumped to (`_JumpToContinuation(continuation,
/// node)`).
#[allow(clippy::too_many_arguments)]
pub fn catch_it(
    e: &mut Engine,
    node: u32,
    handler: u32,
    try_block: u32,
    record: u32,
    context: u32,
    dispatcher: u32,
    info: u32,
    catchable: u32,
    depth: u32,
    marker: u32,
) {
    if catchable != 0 {
        e.call(BUILD_CATCH_OBJECT, &args![record, node, handler, catchable]);
    }
    let frame = if marker != 0 { marker } else { node };
    e.call(UNWIND_NESTED_FRAMES, &args![frame, record]);
    let low = e.mem.u32(try_block);
    e.call(FRAME_UNWIND_TO_STATE, &args![node, dispatcher, info, low]);
    let high = e.mem.u32(try_block.wrapping_add(4));
    e.mem.set_u32(node.wrapping_add(8), high.wrapping_add(1));
    let address = e.mem.u32(handler.wrapping_add(0xc));
    let continuation = e
        .call(
            CALL_CATCH_BLOCK,
            &args![address, record, node, context, info, depth, 0x100u32],
        )
        .u32();
    if continuation != 0 {
        e.call(JUMP_TO_CONTINUATION, &args![continuation, node]);
    }
}

// Translated from 00ecea60 (disassembled, FalloutNV.exe 1.4.0.525)
/// `FindHandlerForForeignException` (LIBCMT/frame.cpp; cdecl: record, registration node,
/// context, dispatcher context, function information, state, catch depth, marker node). A
/// breakpoint (0x80000003) is left alone. With a translator in the thread's data (+0x80, not
/// `__EncodedNull()`) and a code other than COM+'s, `_CallSETranslator` with the seven words
/// gets the first try; a non-zero answer ends the search. No try blocks calls
/// `_inconsistency`. Over the try blocks `_GetRangeOfTrysToCheck` gives (start and end in the
/// frame), each whose `try low <= state <= try high` (signed) has its last handler checked: one
/// with no type or an empty type name (`catch(...)`) and without adjective 0x40 is taken
/// (`CatchIt` with no catchable type), the search going on after it.
#[allow(clippy::too_many_arguments)]
pub fn find_handler_for_foreign_exception(
    e: &mut Engine,
    record: u32,
    node: u32,
    context: u32,
    dispatcher: u32,
    info: u32,
    state: i32,
    depth: u32,
    marker: u32,
) {
    if e.mem.u32(record) == BREAKPOINT {
        return;
    }
    let data = ptd(e);
    if e.mem.u32(data.wrapping_add(PTD_TRANSLATOR)) != 0 {
        let slot = ptd(e).wrapping_add(PTD_TRANSLATOR);
        let null = e.call(ENCODED_NULL, &args![]).u32();
        if e.mem.u32(slot) != null
            && e.mem.u32(record) != COMPLUS_EXCEPTION
            && e.call(
                CALL_SE_TRANSLATOR,
                &args![record, node, context, dispatcher, info, depth, marker],
            )
            .u32()
                != 0
        {
            return;
        }
    }
    if e.mem.u32(info.wrapping_add(0xc)) == 0 {
        e.call(INCONSISTENCY, &args![]);
    }
    // The frame: the end at EBP-8, the start at EBP-4.
    e.with_stack(8, |e, range| {
        let end_at = range.addr();
        let start_at = end_at.wrapping_add(4);
        let mut entry = e
            .call(
                GET_RANGE_OF_TRYS_TO_CHECK,
                &args![info, depth, state, start_at, end_at],
            )
            .u32();
        if e.mem.u32(start_at) >= e.mem.u32(end_at) {
            return;
        }
        loop {
            if state >= e.mem.i32(entry) && state <= e.mem.i32(entry.wrapping_add(4)) {
                let last = (e.mem.u32(entry.wrapping_add(0xc)) << 4)
                    .wrapping_add(e.mem.u32(entry.wrapping_add(0x10)));
                let handler_type = e.mem.u32(last.wrapping_sub(0xc));
                if handler_type == 0 || e.mem.u8(handler_type.wrapping_add(8)) == 0 {
                    let handler = last.wrapping_sub(0x10);
                    if e.mem.u8(handler) & 0x40 == 0 {
                        e.call(
                            CATCH_IT,
                            &args![
                                node, handler, entry, record, context, dispatcher, info, 0u32,
                                depth, marker
                            ],
                        );
                    }
                }
            }
            let start = e.mem.u32(start_at).wrapping_add(1);
            e.mem.set_u32(start_at, start);
            entry = entry.wrapping_add(0x14);
            if start >= e.mem.u32(end_at) {
                break;
            }
        }
    });
}

// Translated from 00eceb54 (disassembled, FalloutNV.exe 1.4.0.525)
/// `FindHandler` (LIBCMT/frame.cpp; cdecl: record, registration node, context, dispatcher
/// context, function information, recursive (a byte), catch depth, marker node). The node's
/// state ([`node_state`]) below -1 or not below the highest state calls `_inconsistency`.
///
/// A C++ rethrow (a C++ throw without throw information, +0x1C) with no current exception in
/// the thread's data (+0x88) ends the search; otherwise the current exception and context
/// stand for the record and context (`_ValidateRead` of it failing, and it being a rethrow
/// too, call `_inconsistency`). With a current exception specification (+0x94, then cleared)
/// the exception must be in it (`IsInExceptionSpec`); otherwise, unless the specification
/// names `std::bad_exception` (`type_info::operator==` with `011c1c8c`; none calls
/// `terminate`), the exception is destroyed (`__DestructExceptionObject(record, 1)`) and a
/// `std::bad_exception("bad exception")` thrown in its place.
///
/// For a C++ throw, the try blocks `_GetRangeOfTrysToCheck` gives that hold the state (signed
/// `try low <= state <= try high`) have their handlers matched (`___TypeMatch`) against the
/// thrown object's catchable types (throw information +0xC: a count, then the types); a match
/// is taken (`CatchIt`) and the search goes on with the next try block. Then a recursive
/// search destroys the exception (`__DestructExceptionObject(record, 1)`). With nothing found,
/// a function information of magic 0x19930521 or above (the low 29 bits) with an exception
/// specification (+0x1C) that does not allow the exception (`IsInExceptionSpec`) makes the
/// record and context the thread's current ones, unwinds the frames above
/// (`_UnwindNestedFrames` to the marker or the node) and this one to -1, and calls the
/// unexpected call `00ece5fc` with the specification, which does not return.
///
/// Any other exception, in a function with try blocks, goes to
/// `FindHandlerForForeignException` with the state, or to `terminate` and the
/// `std::bad_exception` throw when the search is recursive. Last, a current exception
/// specification left in the thread's data calls `_inconsistency`.
#[allow(clippy::too_many_arguments)]
pub fn find_handler(
    e: &mut Engine,
    record: u32,
    node: u32,
    context: u32,
    dispatcher: u32,
    info: u32,
    recursive: u32,
    depth: u32,
    marker: u32,
) {
    e.with_stack(0x2c, |e, frame| {
        // The frame from EBP-0x2C: +0 the std::bad_exception (0xC bytes), +0x10 the end and
        // +0x1C the start of the try range.
        let exception = frame.addr();
        let end_at = exception.wrapping_add(0x10);
        let start_at = exception.wrapping_add(0x1c);
        let mut record = record;
        let mut context = context;
        let mut found = false;
        let max = e.mem.i32(info.wrapping_add(4));
        let state = node_state(e, node, info);
        if state < -1 || state >= max {
            e.call(INCONSISTENCY, &args![]);
        }
        let recursive = recursive as u8 != 0;
        if is_cxx_throw(e, record) && e.mem.u32(record.wrapping_add(0x1c)) == 0 {
            let data = ptd(e);
            if e.mem.u32(data.wrapping_add(PTD_CUR_EXCEPTION)) == 0 {
                return;
            }
            let data = ptd(e);
            record = e.mem.u32(data.wrapping_add(PTD_CUR_EXCEPTION));
            let data = ptd(e);
            context = e.mem.u32(data.wrapping_add(PTD_CUR_CONTEXT));
            if e.call(VALIDATE_READ, &args![record, 1u32]).u32() == 0 {
                e.call(INCONSISTENCY, &args![]);
            }
            if is_cxx_throw(e, record) && e.mem.u32(record.wrapping_add(0x1c)) == 0 {
                e.call(INCONSISTENCY, &args![]);
            }
            let data = ptd(e);
            if e.mem.u32(data.wrapping_add(PTD_CUR_EXC_SPEC)) != 0 {
                let data = ptd(e);
                let spec = e.mem.u32(data.wrapping_add(PTD_CUR_EXC_SPEC));
                let data = ptd(e);
                e.mem.set_u32(data.wrapping_add(PTD_CUR_EXC_SPEC), 0);
                if e.call(IS_IN_EXCEPTION_SPEC, &args![spec, record]).u8() == 0 {
                    let mut index = 0i32;
                    let mut named = false;
                    if e.mem.i32(spec) > 0 {
                        loop {
                            let types = e.mem.u32(spec.wrapping_add(4));
                            let type_info = e
                                .mem
                                .u32(((index as u32) << 4).wrapping_add(types).wrapping_add(4));
                            if e.call(TYPE_INFO_EQUAL, &args![type_info, BAD_EXCEPTION_TYPE])
                                .u8()
                                != 0
                            {
                                named = true;
                                break;
                            }
                            index += 1;
                            if index >= e.mem.i32(spec) {
                                break;
                            }
                        }
                    }
                    if !named {
                        e.call(TERMINATE, &args![]);
                    }
                    throw_bad_exception(e, record, exception);
                }
            }
        }
        if is_cxx_throw(e, record) {
            if e.mem.u32(info.wrapping_add(0xc)) != 0 {
                let mut entry = e
                    .call(
                        GET_RANGE_OF_TRYS_TO_CHECK,
                        &args![info, depth, state, start_at, end_at],
                    )
                    .u32();
                while e.mem.u32(start_at) < e.mem.u32(end_at) {
                    if e.mem.i32(entry) <= state && state <= e.mem.i32(entry.wrapping_add(4)) {
                        let mut handler = e.mem.u32(entry.wrapping_add(0x10));
                        let mut catches = e.mem.i32(entry.wrapping_add(0xc));
                        'handlers: while catches > 0 {
                            let throw_info = e.mem.u32(record.wrapping_add(0x1c));
                            let array = e.mem.u32(throw_info.wrapping_add(0xc));
                            let mut slot = array.wrapping_add(4);
                            let mut left = e.mem.i32(array);
                            while left > 0 {
                                let catchable = e.mem.u32(slot);
                                let throw_info = e.mem.u32(record.wrapping_add(0x1c));
                                if e.call(TYPE_MATCH, &args![handler, catchable, throw_info])
                                    .u32()
                                    != 0
                                {
                                    found = true;
                                    e.call(
                                        CATCH_IT,
                                        &args![
                                            node, handler, entry, record, context, dispatcher,
                                            info, catchable, depth, marker
                                        ],
                                    );
                                    break 'handlers;
                                }
                                left -= 1;
                                slot = slot.wrapping_add(4);
                            }
                            catches -= 1;
                            handler = handler.wrapping_add(0x10);
                        }
                    }
                    let start = e.mem.u32(start_at).wrapping_add(1);
                    e.mem.set_u32(start_at, start);
                    entry = entry.wrapping_add(0x14);
                }
            }
            if recursive {
                e.call(DESTRUCT_EXCEPTION_OBJECT, &args![record, 1u32]);
            }
            if !found && e.mem.u32(info) & 0x1fff_ffff >= MAGIC_2 {
                let spec = e.mem.u32(info.wrapping_add(0x1c));
                if spec != 0 && e.call(IS_IN_EXCEPTION_SPEC, &args![spec, record]).u8() == 0 {
                    ptd(e);
                    ptd(e);
                    let data = ptd(e);
                    e.mem.set_u32(data.wrapping_add(PTD_CUR_EXCEPTION), record);
                    let data = ptd(e);
                    e.mem.set_u32(data.wrapping_add(PTD_CUR_CONTEXT), context);
                    let frame = if marker != 0 { marker } else { node };
                    e.call(UNWIND_NESTED_FRAMES, &args![frame, record]);
                    e.call(
                        FRAME_UNWIND_TO_STATE,
                        &args![node, dispatcher, info, u32::MAX],
                    );
                    let spec = e.mem.u32(info.wrapping_add(0x1c));
                    e.call(CALL_UNEXPECTED, &args![spec]);
                    panic!("00eceb54: the unexpected call 00ece5fc returned");
                }
            }
        } else if e.mem.u32(info.wrapping_add(0xc)) != 0 {
            if recursive {
                e.call(TERMINATE, &args![]);
                throw_bad_exception(e, record, exception);
            }
            e.call(
                FIND_HANDLER_FOR_FOREIGN_EXCEPTION,
                &args![
                    record,
                    node,
                    context,
                    dispatcher,
                    info,
                    state as u32,
                    depth,
                    marker
                ],
            );
        }
        let data = ptd(e);
        if e.mem.u32(data.wrapping_add(PTD_CUR_EXC_SPEC)) != 0 {
            e.call(INCONSISTENCY, &args![]);
        }
    })
}

/// `FindHandler`'s `00ecec95`: `__DestructExceptionObject(record, 1)`, a `std::bad_exception`
/// from `"bad exception"` in the frame and `_CxxThrowException` of it with `0118011c`, which
/// does not return (the run stops if it does).
fn throw_bad_exception(e: &mut Engine, record: u32, exception: u32) {
    e.call(DESTRUCT_EXCEPTION_OBJECT, &args![record, 1u32]);
    e.call(
        BAD_EXCEPTION_FROM_TEXT,
        &args![exception, BAD_EXCEPTION_TEXT],
    );
    e.call(
        CXX_THROW_EXCEPTION,
        &args![exception, BAD_EXCEPTION_THROW_INFO],
    );
    panic!("00eceb54: _CxxThrowException returned (std::bad_exception)");
}

// Translated from 00eceeb8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___InternalCxxFrameHandler` (LIBCMT/frame.cpp; cdecl: record, registration node, context,
/// dispatcher context, function information, nesting level, marker node, recursive). Answers 1
/// (`ExceptionContinueSearch`) at once for an exception other than C++ and 0x80000026 when the
/// thread is not rethrowing (+0x20C), the function information's magic (low 29 bits) is
/// 0x19930522 or above and its flags (+0x20) have bit 0 (synchronous exceptions only). While
/// unwinding (the record's flags +4 & 0x66) a function with states (+4) at nesting 0 is unwound
/// to -1 (`___FrameUnwindToState`), answering 1. Otherwise a function with no try blocks
/// answers 1 unless its magic is 0x19930521 or above with an exception specification (+0x1C).
/// A C++ exception with at least three parameters and a magic above 0x19930522 whose throw
/// information names a forward-compatibility handler (+8) gets that handler's answer, called
/// with the eight words (the recursive flag as a zero-extended byte); anything else goes to
/// `FindHandler` and answers 1.
#[allow(clippy::too_many_arguments)]
pub fn internal_cxx_frame_handler(
    e: &mut Engine,
    record: u32,
    node: u32,
    context: u32,
    dispatcher: u32,
    info: u32,
    nesting: u32,
    marker: u32,
    recursive: u32,
) -> u32 {
    let data = ptd(e);
    let rethrow = e.mem.u32(data.wrapping_add(PTD_RETHROW));
    let code = e.mem.u32(record);
    if rethrow == 0
        && code != CXX_EXCEPTION
        && code != UNWIND_CONSOLIDATE
        && e.mem.u32(info) & 0x1fff_ffff >= MAGIC_3
        && e.mem.u8(info.wrapping_add(0x20)) & 1 != 0
    {
        return 1;
    }
    if e.mem.u8(record.wrapping_add(4)) & 0x66 != 0 {
        if e.mem.u32(info.wrapping_add(4)) != 0 && nesting == 0 {
            e.call(
                FRAME_UNWIND_TO_STATE,
                &args![node, dispatcher, info, u32::MAX],
            );
        }
        return 1;
    }
    if e.mem.u32(info.wrapping_add(0xc)) == 0
        && (e.mem.u32(info) & 0x1fff_ffff < MAGIC_2 || e.mem.u32(info.wrapping_add(0x1c)) == 0)
    {
        return 1;
    }
    if e.mem.u32(record) == CXX_EXCEPTION
        && e.mem.u32(record.wrapping_add(0x10)) >= 3
        && e.mem.u32(record.wrapping_add(0x14)) > MAGIC_3
    {
        let throw_info = e.mem.u32(record.wrapping_add(0x1c));
        let forward = e.mem.u32(throw_info.wrapping_add(8));
        if forward != 0 {
            let flag = u32::from(recursive as u8);
            return e
                .call(
                    forward,
                    &args![record, node, context, dispatcher, info, nesting, marker, flag],
                )
                .u32();
        }
    }
    e.call(
        FIND_HANDLER,
        &args![record, node, context, dispatcher, info, recursive, nesting, marker],
    );
    1
}

// Translated from 00edf49f (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CallDestructExceptionObject` (cdecl: record, throw flag): for a C++ exception, when the
/// pointer at `010f20a0` (`__DestructExceptionObject`) is set and lies in a read-only part of
/// the image (`_IsNonwritableInCurrentImage(010f20a0)`), it is called with the two words.
pub fn call_destruct_exception_object(e: &mut Engine, record: u32, flag: u32) {
    if e.mem.u32(record) != CXX_EXCEPTION || e.mem.u32(DESTRUCT_POINTER) == 0 {
        return;
    }
    if e.call(IS_NONWRITABLE_IN_CURRENT_IMAGE, &args![DESTRUCT_POINTER])
        .u32()
        == 0
    {
        return;
    }
    let destruct = e.mem.u32(DESTRUCT_POINTER);
    e.call(destruct, &args![record, flag]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_5f70, array_unwind(u32, u32, i32, u32)),
        entry!(0x00ec_5f93, array_unwind_filter(u32) -> u32),
        entry!(0x00ec_e381, frame_unwind_filter(u32) -> u32),
        entry!(0x00ec_e3ca, frame_unwind_to_state(u32, u32, u32, i32)),
        entry!(0x00ec_e5fc, call_unexpected(u32)),
        entry!(0x00ec_e62d, call_unexpected_catch(u32)),
        entry!(
            0x00ec_e645,
            call_catch_block(u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00ec_e6de, call_catch_block_rethrown(u32) -> u32),
        entry!(
            0x00ec_e76b,
            call_catch_block_finally(u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00ec_e7e1, build_catch_object_helper(u32, u32, u32, u32) -> u32),
        entry!(0x00ec_e960, build_catch_object(u32, u32, u32, u32)),
        entry!(
            0x00ec_e9f2,
            catch_it(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00ec_ea60,
            find_handler_for_foreign_exception(u32, u32, u32, u32, u32, i32, u32, u32)
        ),
        entry!(
            0x00ec_eb54,
            find_handler(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00ec_eeb8,
            internal_cxx_frame_handler(u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00ed_f49f, call_destruct_exception_object(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn nothing(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    /// An engine whose `_getptd` answers a zeroed block, which it also gives back.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x220);
        answer(&mut e, GETPTD, data);
        e.call_log = Some(vec![]);
        (e, data)
    }

    /// A C++ throw record (three parameters, the first magic) of `object` with `throw_info`.
    fn cxx_record(e: &mut Engine, object: u32, throw_info: u32) -> u32 {
        let record = e.mem.alloc(0x50);
        e.mem.set_u32(record, CXX_EXCEPTION);
        e.mem.set_u32(record + 0x10, 3);
        e.mem.set_u32(record + 0x14, MAGIC_1);
        e.mem.set_u32(record + 0x18, object);
        e.mem.set_u32(record + 0x1c, throw_info);
        record
    }

    #[test]
    fn array_unwind_destroys_the_elements_backwards() {
        let (mut e, _) = engine();
        let destructor = 0x0700_0000;
        nothing(&mut e, &[destructor]);
        e.call(0x00ec_5f70, &args![0x1030u32, 0x10u32, 3i32, destructor]);
        assert_eq!(
            calls(&e, destructor),
            vec![vec![0x1020], vec![0x1010], vec![0x1000]]
        );
        e.call(0x00ec_5f70, &args![0x1030u32, 0x10u32, 0i32, destructor]);
        assert_eq!(calls(&e, destructor).len(), 3);
        // The filter: anything but a C++ exception continues the search.
        let record = e.mem.alloc(4);
        e.mem.set_u32(record, 0xc000_0005);
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, record);
        let frame = e.mem.alloc(0x40) + 0x30;
        e.mem.set_u32(frame - 0x14, pointers);
        e.mem.set_u32(frame - 0x24, 7);
        assert_eq!(e.call(0x00ec_5f93, &[frame]).u32(), 0);
        assert_eq!(e.mem.u32(frame - 0x1c), pointers);
        assert_eq!(e.mem.u32(frame - 0x20), record);
        assert_eq!(e.mem.u32(frame - 0x24), 0);
    }

    #[test]
    #[should_panic(expected = "terminate returned")]
    fn array_unwind_filter_terminates_on_a_cxx_exception() {
        let (mut e, _) = engine();
        nothing(&mut e, &[TERMINATE]);
        let record = cxx_record(&mut e, 0, 0);
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, record);
        let frame = e.mem.alloc(0x40) + 0x30;
        e.mem.set_u32(frame - 0x14, pointers);
        array_unwind_filter(&mut e, frame);
    }

    #[test]
    fn frame_unwind_filter_counts_throws_down_or_terminates() {
        let (mut e, data) = engine();
        nothing(&mut e, &[TERMINATE]);
        let record = e.mem.alloc(4);
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, record);
        e.mem.set_u32(record, COMPLUS_EXCEPTION);
        e.mem.set_u32(data + PTD_PROCESSING_THROW, 2);
        assert_eq!(e.call(0x00ec_e381, &[pointers]).u32(), 0);
        assert_eq!(e.mem.u32(data + PTD_PROCESSING_THROW), 1);
        e.mem.set_u32(data + PTD_PROCESSING_THROW, 0);
        e.call(0x00ec_e381, &[pointers]);
        assert_eq!(e.mem.u32(data + PTD_PROCESSING_THROW), 0);
        assert!(calls(&e, TERMINATE).is_empty());
        e.mem.set_u32(record, CXX_EXCEPTION);
        e.mem.set_u32(data + PTD_PROCESSING_THROW, 5);
        e.call(0x00ec_e381, &[pointers]);
        assert_eq!(e.mem.u32(data + PTD_PROCESSING_THROW), 0);
        assert_eq!(calls(&e, TERMINATE).len(), 1);
    }

    /// A function information with `states` unwind entries `(to state, action)`.
    fn function_info(e: &mut Engine, states: &[(i32, u32)]) -> u32 {
        let info = e.mem.alloc(0x24);
        let map = e.mem.alloc(8 * states.len() as u32 + 8);
        for (i, &(to, action)) in states.iter().enumerate() {
            e.mem.set_u32(map + 8 * i as u32, to as u32);
            e.mem.set_u32(map + 8 * i as u32 + 4, action);
        }
        e.mem.set_u32(info, MAGIC_1);
        e.mem.set_u32(info + 4, states.len() as u32);
        e.mem.set_u32(info + 8, map);
        info
    }

    #[test]
    fn frame_unwind_to_state_runs_the_actions_down_to_the_target() {
        let (mut e, data) = engine();
        nothing(
            &mut e,
            &[
                CALL_SETTING_FRAME,
                FRAME_UNWIND_TO_STATE_FINALLY,
                INCONSISTENCY,
            ],
        );
        let info = function_info(&mut e, &[(-1, 0x0800_0020), (0, 0), (1, 0x0800_0010)]);
        let node = e.mem.alloc(0xc);
        // Few states: the low byte only, sign-extended.
        e.mem.set_u32(node + 8, 0xffff_ff02);
        e.call(0x00ec_e3ca, &args![node, 0x55u32, info, -1i32]);
        assert_eq!(
            calls(&e, CALL_SETTING_FRAME),
            vec![
                vec![0x0800_0010, node, 0x103],
                vec![0x0800_0020, node, 0x103]
            ]
        );
        assert_eq!(e.mem.u32(node + 8), u32::MAX);
        assert_eq!(e.mem.u32(data + PTD_PROCESSING_THROW), 1);
        assert_eq!(calls(&e, FRAME_UNWIND_TO_STATE_FINALLY).len(), 1);
        assert!(calls(&e, INCONSISTENCY).is_empty());
        // A state past the highest one is an inconsistency.
        e.mem.set_u32(node + 8, 1);
        e.mem.set_u32(info + 4, 1);
        e.call(0x00ec_e3ca, &args![node, 0u32, info, 0i32]);
        assert_eq!(calls(&e, INCONSISTENCY).len(), 1);
        assert_eq!(e.mem.u32(node + 8), 0);
    }

    /// A handler with `adjectives`, a named type and the catch object at displacement 0x10.
    fn handler(e: &mut Engine, adjectives: u32) -> u32 {
        let handler = e.mem.alloc(0x10);
        let type_descriptor = e.mem.alloc(0x10);
        e.mem.set_u8(type_descriptor + 8, b'.');
        e.mem.set_u32(handler, adjectives);
        e.mem.set_u32(handler + 4, type_descriptor);
        e.mem.set_u32(handler + 8, 0x10);
        e.mem.set_u32(handler + 0xc, 0x0040_9000);
        handler
    }

    fn catchable(e: &mut Engine, properties: u32, size: u32, copy: u32) -> u32 {
        let catchable = e.mem.alloc(0x1c);
        e.mem.set_u32(catchable, properties);
        e.mem.set_u32(catchable + 0x14, size);
        e.mem.set_u32(catchable + 0x18, copy);
        catchable
    }

    #[test]
    fn build_catch_object_by_reference_value_and_copy_constructor() {
        let (mut e, _) = engine();
        nothing(&mut e, &[CALL_MEMBER_FUNCTION, INCONSISTENCY]);
        // ___AdjustPointer doubled: the object plus 0x100.
        e.register(ADJUST_POINTER, |_, a| (a[0] + 0x100).into_ret());
        let object = e.mem.alloc(0x110);
        e.mem.write(object, &[1, 2, 3, 4, 5, 6, 7, 8]);
        let record = cxx_record(&mut e, object, 0);
        let node = e.mem.alloc(0x40);
        let target = node + 0x1c;

        // A reference: the adjusted object.
        let by_reference = handler(&mut e, 8);
        let simple = catchable(&mut e, 1, 8, 0);
        e.call(0x00ec_e960, &[record, node, by_reference, simple]);
        assert_eq!(e.mem.u32(target), object + 0x100);
        // A simple type: its bytes; a 4-byte pointer adjusted after.
        let by_value = handler(&mut e, 0);
        e.call(0x00ec_e960, &[record, node, by_value, simple]);
        assert_eq!(e.mem.bytes(target, 8), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let pointer = catchable(&mut e, 1, 4, 0);
        e.call(0x00ec_e960, &[record, node, by_value, pointer]);
        assert_eq!(e.mem.u32(target), 0x0403_0201 + 0x100);
        // No copy constructor: the adjusted object's bytes.
        let plain = catchable(&mut e, 0, 4, 0);
        e.mem.set_u32(object + 0x100, 0x1122_3344);
        e.call(0x00ec_e960, &[record, node, by_value, plain]);
        assert_eq!(e.mem.u32(target), 0x1122_3344);
        assert!(calls(&e, CALL_MEMBER_FUNCTION).is_empty());
        // A copy constructor: run on the catch object (bit 31: the node itself).
        let constructed = catchable(&mut e, 0, 8, 0x0900_0000);
        assert_eq!(
            e.call(0x00ec_e7e1, &[record, node, by_value, constructed])
                .u32(),
            1
        );
        e.call(0x00ec_e960, &[record, node, by_value, constructed]);
        let whole = handler(&mut e, 0x8000_0000);
        let virtual_base = catchable(&mut e, 4, 8, 0x0900_0000);
        e.call(0x00ec_e960, &[record, node, whole, virtual_base]);
        assert_eq!(
            calls(&e, CALL_MEMBER_FUNCTION),
            vec![
                vec![target, 0x0900_0000, object + 0x100],
                vec![node, 0x0900_0000, object + 0x100, 1]
            ]
        );
        assert!(calls(&e, INCONSISTENCY).is_empty());
        // No type name, or neither a displacement nor bit 31: nothing.
        let unnamed = handler(&mut e, 0);
        let type_descriptor = e.mem.u32(unnamed + 4);
        e.mem.set_u8(type_descriptor + 8, 0);
        assert_eq!(
            e.call(0x00ec_e7e1, &[record, node, unnamed, constructed])
                .u32(),
            0
        );
        e.mem.set_u32(by_value + 8, 0);
        assert_eq!(
            e.call(0x00ec_e7e1, &[record, node, by_value, constructed])
                .u32(),
            0
        );
        // A null thrown object fails _ValidateRead.
        e.mem.set_u32(record + 0x18, 0);
        assert_eq!(
            e.call(0x00ec_e7e1, &[record, node, by_reference, simple])
                .u32(),
            0
        );
        assert_eq!(calls(&e, INCONSISTENCY).len(), 1);
    }

    #[test]
    fn catch_it_builds_unwinds_runs_the_block_and_continues() {
        let (mut e, _) = engine();
        nothing(
            &mut e,
            &[
                BUILD_CATCH_OBJECT,
                UNWIND_NESTED_FRAMES,
                FRAME_UNWIND_TO_STATE,
                JUMP_TO_CONTINUATION,
            ],
        );
        answer(&mut e, CALL_CATCH_BLOCK, 0x0040_1234);
        let node = e.mem.alloc(0xc);
        let handler = handler(&mut e, 0);
        let try_block = e.mem.alloc(0x14);
        e.mem.set_u32(try_block, 2);
        e.mem.set_u32(try_block + 4, 3);
        e.call(
            0x00ec_e9f2,
            &[node, handler, try_block, 0x10, 0x20, 0x30, 0x40, 0x50, 1, 0],
        );
        assert_eq!(
            calls(&e, BUILD_CATCH_OBJECT),
            vec![vec![0x10, node, handler, 0x50]]
        );
        assert_eq!(calls(&e, UNWIND_NESTED_FRAMES), vec![vec![node, 0x10]]);
        assert_eq!(
            calls(&e, FRAME_UNWIND_TO_STATE),
            vec![vec![node, 0x30, 0x40, 2]]
        );
        assert_eq!(e.mem.u32(node + 8), 4);
        assert_eq!(
            calls(&e, CALL_CATCH_BLOCK),
            vec![vec![0x0040_9000, 0x10, node, 0x20, 0x40, 1, 0x100]]
        );
        assert_eq!(
            calls(&e, JUMP_TO_CONTINUATION),
            vec![vec![0x0040_1234, node]]
        );
        // No catchable type: nothing built; a marker node is unwound to; no continuation.
        answer(&mut e, CALL_CATCH_BLOCK, 0);
        e.call(
            0x00ec_e9f2,
            &[node, handler, try_block, 0x10, 0x20, 0x30, 0x40, 0, 1, 0x77],
        );
        assert_eq!(calls(&e, BUILD_CATCH_OBJECT).len(), 1);
        assert_eq!(calls(&e, UNWIND_NESTED_FRAMES)[1], vec![0x77, 0x10]);
        assert_eq!(calls(&e, JUMP_TO_CONTINUATION).len(), 1);
    }

    #[test]
    fn call_catch_block_swaps_the_current_exception_and_restores_it() {
        let (mut e, data) = engine();
        e.register(CREATE_FRAME_INFO, |_, a| a[0].into_ret());
        nothing(&mut e, &[FIND_AND_UNLINK_FRAME, DESTRUCT_EXCEPTION_OBJECT]);
        answer(&mut e, IS_EXCEPTION_OBJECT_TO_BE_DESTROYED, 1);
        let node = e.mem.alloc(0x10) + 4;
        e.mem.set_u32(node - 4, 0x77);
        let record = cxx_record(&mut e, 0x5000, 0);
        e.mem.set_u32(data + PTD_CUR_EXCEPTION, 0xaa);
        e.mem.set_u32(data + PTD_CUR_CONTEXT, 0xbb);
        e.register_double(CALL_CATCH_BLOCK2, move |e, _| {
            // The catch block sees the record and context as current; it moves [node-4].
            assert_eq!(e.mem.u32(data + PTD_CUR_EXCEPTION), record);
            assert_eq!(e.mem.u32(data + PTD_CUR_CONTEXT), 0xcc);
            e.mem.set_u32(node - 4, 0x99);
            0x0040_5678u32.into_ret()
        });
        let continuation = e
            .call(
                0x00ec_e645,
                &[0x0040_9000, record, node, 0xcc, 0x40, 2, 0x100],
            )
            .u32();
        assert_eq!(continuation, 0x0040_5678);
        assert_eq!(
            calls(&e, CALL_CATCH_BLOCK2),
            vec![vec![node, 0x40, 0x0040_9000, 2, 0x100]]
        );
        let frame_info = calls(&e, CREATE_FRAME_INFO)[0][0];
        assert_eq!(calls(&e, CREATE_FRAME_INFO)[0][1], 0x5000);
        assert_eq!(calls(&e, FIND_AND_UNLINK_FRAME), vec![vec![frame_info]]);
        assert_eq!(e.mem.u32(node - 4), 0x77);
        assert_eq!(e.mem.u32(data + PTD_CUR_EXCEPTION), 0xaa);
        assert_eq!(e.mem.u32(data + PTD_CUR_CONTEXT), 0xbb);
        assert_eq!(calls(&e, DESTRUCT_EXCEPTION_OBJECT), vec![vec![record, 0]]);
        // No continuation, or an object another frame still holds: not destroyed.
        call_catch_block_finally(&mut e, node, record, 0x77, 0, 0, 0, 0, 0, 0);
        answer(&mut e, IS_EXCEPTION_OBJECT_TO_BE_DESTROYED, 0);
        call_catch_block_finally(&mut e, node, record, 0x77, 0, 0, 0, 0, 1, 0);
        assert_eq!(calls(&e, DESTRUCT_EXCEPTION_OBJECT).len(), 1);
    }

    #[test]
    fn call_catch_block_rethrown_unwinds_to_the_catch_s_outer_state() {
        let (mut e, data) = engine();
        nothing(&mut e, &[FRAME_UNWIND_TO_STATE]);
        let info = function_info(&mut e, &[(-1, 0), (0, 0), (7, 0), (2, 0)]);
        let map = e.mem.alloc(0x28);
        // Try 0: try 0..1, catches up to 3; try 1 (later in the map) is not reached.
        for (i, (low, high, catch)) in [(0u32, 1u32, 3u32), (0, 4, 5)].iter().enumerate() {
            let entry = map + 0x14 * i as u32;
            e.mem.set_u32(entry, *low);
            e.mem.set_u32(entry + 4, *high);
            e.mem.set_u32(entry + 8, *catch);
        }
        e.mem.set_u32(info + 0xc, 2);
        e.mem.set_u32(info + 0x10, map);
        e.mem.set_u32(data + PTD_RETHROW, 1);
        nothing(&mut e, &[CALL_CATCH_BLOCK_FINALLY]);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, 3);
        // CallCatchBlock's frame: its arguments above EBP, its locals below, and the stack the
        // frame kept (`[EBP-0x18]`) holding the /GS word and the saved EDI, ESI, EBX.
        let block = e.mem.alloc(0x100);
        let ebp = block + 0x80;
        let frame = |e: &mut Engine, ebp: u32| {
            e.mem.set_u32(ebp, 0xe0);
            e.mem.set_u32(ebp + 8, 0x5000);
            e.mem.set_u32(ebp + 0xc, node);
            e.mem.set_u32(ebp + 0x10, 1);
            e.mem.set_u32(ebp + 0x14, info);
            for (offset, word) in [(0x24u32, 0x24u32), (0x28, 0x28), (0x2c, 0x2c), (0x30, 0x30)] {
                e.mem.set_u32(ebp - offset, word);
            }
            e.mem.set_u32(ebp - 0x34, 0);
            e.mem.set_u32(ebp - 0x1c, 0x0040_7777);
            let esp = ebp - 0x60;
            e.mem.set_u32(esp + 4, 0xd1);
            e.mem.set_u32(esp + 8, 0x51);
            e.mem.set_u32(esp + 0xc, 0xb0);
            e.mem.set_u32(ebp - 0x18, esp);
        };
        frame(&mut e, ebp);
        assert_eq!(e.call(0x00ec_e6de, &[ebp]).u32(), 0);
        assert_eq!(e.mem.u32(data + PTD_RETHROW), 0);
        // The unwind entry after try high (1) is state 2: to state 7; the loop stopped at try 0.
        assert_eq!(
            calls(&e, FRAME_UNWIND_TO_STATE),
            vec![vec![node, 0, info, 7]]
        );
        assert_eq!(e.mem.u32(ebp - 0x20), 0);
        // The continuation cleared, out of the try block, no abnormal termination, and the
        // __finally body run with the node, the record and the frame words.
        assert_eq!(e.mem.u32(ebp - 0x1c), 0);
        assert_eq!(e.mem.u32(ebp - 4), 0xffff_fffe);
        assert_eq!(e.mem.u32(ebp + 0x10), 0);
        assert_eq!(
            calls(&e, CALL_CATCH_BLOCK_FINALLY),
            vec![vec![node, 0x5000, 0x24, 0x28, 0x2c, 0x30, 0, 0, 0]]
        );
        // __SEH_epilog4's return address lands over the saved EBP's slot.
        assert_eq!(e.mem.u32(ebp), 0x00ec_e75f);
        // A state outside every catch range unwinds to itself, after both tries were looked at.
        frame(&mut e, ebp);
        e.mem.set_u32(node + 8, 1);
        call_catch_block_rethrown(&mut e, ebp);
        assert_eq!(calls(&e, FRAME_UNWIND_TO_STATE)[1], vec![node, 0, info, 1]);
        assert_eq!(e.mem.u32(ebp - 0x20), 2);
    }

    /// A C++ throw of an object with two catchable types, and a function with one try block
    /// (states 0..=1) of two handlers: (record, info, try map, handlers, the catchable types).
    fn throw_and_function(e: &mut Engine) -> (u32, u32, u32, u32, [u32; 2]) {
        let types = [e.mem.alloc(0x1c), e.mem.alloc(0x1c)];
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array, 2);
        e.mem.set_u32(array + 4, types[0]);
        e.mem.set_u32(array + 8, types[1]);
        let throw_info = e.mem.alloc(0x10);
        e.mem.set_u32(throw_info + 0xc, array);
        let record = cxx_record(e, 0x5000, throw_info);
        let info = function_info(e, &[(-1, 0), (0, 0), (0, 0), (0, 0)]);
        let handlers = e.mem.alloc(0x20);
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 1);
        e.mem.set_u32(map + 8, 3);
        e.mem.set_u32(map + 0xc, 2);
        e.mem.set_u32(map + 0x10, handlers);
        e.mem.set_u32(info + 0xc, 1);
        e.mem.set_u32(info + 0x10, map);
        (record, info, map, handlers, types)
    }

    /// `_GetRangeOfTrysToCheck` doubled: the range 0..1 and the try map.
    fn one_try_range(e: &mut Engine, map: u32) {
        e.register_double(GET_RANGE_OF_TRYS_TO_CHECK, move |e, a| {
            e.mem.set_u32(a[3], 0);
            e.mem.set_u32(a[4], 1);
            map.into_ret()
        });
    }

    #[test]
    fn find_handler_takes_the_first_matching_handler() {
        let (mut e, _) = engine();
        nothing(
            &mut e,
            &[CATCH_IT, INCONSISTENCY, DESTRUCT_EXCEPTION_OBJECT],
        );
        let (record, info, map, handlers, types) = throw_and_function(&mut e);
        one_try_range(&mut e, map);
        // The second handler matches the second catchable type.
        let wanted = (handlers + 0x10, types[1]);
        e.register_double(TYPE_MATCH, move |_, a| {
            u32::from((a[0], a[1]) == wanted).into_ret()
        });
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, 1);
        e.call(0x00ec_eb54, &[record, node, 0x20, 0x30, info, 0, 5, 0x77]);
        assert_eq!(calls(&e, GET_RANGE_OF_TRYS_TO_CHECK)[0][..3], [info, 5, 1]);
        assert_eq!(
            calls(&e, CATCH_IT),
            vec![vec![
                node,
                handlers + 0x10,
                map,
                record,
                0x20,
                0x30,
                info,
                types[1],
                5,
                0x77
            ]]
        );
        assert_eq!(calls(&e, TYPE_MATCH).len(), 4);
        assert!(calls(&e, INCONSISTENCY).is_empty());
        // A state outside the try block matches nothing; recursive destroys the exception.
        e.mem.set_u32(node + 8, 2);
        e.call(0x00ec_eb54, &[record, node, 0x20, 0x30, info, 1, 5, 0x77]);
        assert_eq!(calls(&e, CATCH_IT).len(), 1);
        assert_eq!(calls(&e, DESTRUCT_EXCEPTION_OBJECT), vec![vec![record, 1]]);
    }

    #[test]
    fn find_handler_sends_other_exceptions_to_the_foreign_search() {
        let (mut e, data) = engine();
        nothing(&mut e, &[FIND_HANDLER_FOR_FOREIGN_EXCEPTION, INCONSISTENCY]);
        let (record, info, _, _, _) = throw_and_function(&mut e);
        e.mem.set_u32(record, 0xc000_0005);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, 0xff);
        e.call(0x00ec_eb54, &[record, node, 0x20, 0x30, info, 0, 5, 0]);
        assert_eq!(
            calls(&e, FIND_HANDLER_FOR_FOREIGN_EXCEPTION),
            vec![vec![record, node, 0x20, 0x30, info, u32::MAX, 5, 0]]
        );
        // A rethrow with no current exception ends the search at once.
        let rethrow = cxx_record(&mut e, 0, 0);
        e.call(0x00ec_eb54, &[rethrow, node, 0x20, 0x30, info, 0, 5, 0]);
        assert_eq!(calls(&e, FIND_HANDLER_FOR_FOREIGN_EXCEPTION).len(), 1);
        // A current exception specification left behind is an inconsistency.
        e.mem.set_u32(data + PTD_CUR_EXC_SPEC, 1);
        e.call(0x00ec_eb54, &[record, node, 0x20, 0x30, info, 0, 5, 0]);
        assert_eq!(calls(&e, INCONSISTENCY).len(), 1);
    }

    #[test]
    #[should_panic(expected = "unexpected call")]
    fn find_handler_calls_unexpected_for_an_exception_the_spec_forbids() {
        let (mut e, data) = engine();
        nothing(
            &mut e,
            &[UNWIND_NESTED_FRAMES, FRAME_UNWIND_TO_STATE, INCONSISTENCY],
        );
        answer(&mut e, IS_IN_EXCEPTION_SPEC, 0);
        let (record, info, _, _, _) = throw_and_function(&mut e);
        e.mem.set_u32(info, MAGIC_2);
        e.mem.set_u32(info + 0xc, 0);
        e.mem.set_u32(info + 0x1c, 0x0123_0000);
        let node = e.mem.alloc(0xc);
        e.register_double(CALL_UNEXPECTED, move |e, a| {
            assert_eq!(a, [0x0123_0000]);
            assert_eq!(e.mem.u32(data + PTD_CUR_EXCEPTION), record);
            assert_eq!(e.mem.u32(data + PTD_CUR_CONTEXT), 0x20);
            Ret::default()
        });
        find_handler(&mut e, record, node, 0x20, 0x30, info, 0, 5, 0);
    }

    #[test]
    fn find_handler_for_foreign_exception_takes_catch_all_handlers() {
        let (mut e, data) = engine();
        nothing(&mut e, &[CATCH_IT, INCONSISTENCY]);
        let (record, info, map, handlers, _) = throw_and_function(&mut e);
        e.mem.set_u32(record, 0xc000_0005);
        one_try_range(&mut e, map);
        // The last handler is catch(...): no type.
        e.mem.set_u32(handlers + 0x14, 0);
        e.call(0x00ec_ea60, &[record, 0x60, 0x20, 0x30, info, 1, 5, 0]);
        assert_eq!(
            calls(&e, CATCH_IT),
            vec![vec![
                0x60,
                handlers + 0x10,
                map,
                record,
                0x20,
                0x30,
                info,
                0,
                5,
                0
            ]]
        );
        // Adjective 0x40 skips it; a state outside the block too.
        e.mem.set_u32(handlers + 0x10, 0x40);
        e.call(0x00ec_ea60, &[record, 0x60, 0x20, 0x30, info, 1, 5, 0]);
        e.mem.set_u32(handlers + 0x10, 0);
        e.call(0x00ec_ea60, &[record, 0x60, 0x20, 0x30, info, 2, 5, 0]);
        assert_eq!(calls(&e, CATCH_IT).len(), 1);
        // A translator that takes it ends the search; a breakpoint is left alone.
        e.mem.set_u32(data + PTD_TRANSLATOR, 0x0900_0000);
        answer(&mut e, ENCODED_NULL, 0x1234);
        answer(&mut e, CALL_SE_TRANSLATOR, 1);
        e.call(0x00ec_ea60, &[record, 0x60, 0x20, 0x30, info, 1, 5, 0]);
        assert_eq!(
            calls(&e, CALL_SE_TRANSLATOR),
            vec![vec![record, 0x60, 0x20, 0x30, info, 5, 0]]
        );
        e.mem.set_u32(record, BREAKPOINT);
        e.call(0x00ec_ea60, &[record, 0x60, 0x20, 0x30, info, 1, 5, 0]);
        assert_eq!(calls(&e, CALL_SE_TRANSLATOR).len(), 1);
        assert_eq!(calls(&e, CATCH_IT).len(), 1);
        assert!(calls(&e, INCONSISTENCY).is_empty());
    }

    #[test]
    fn internal_cxx_frame_handler_unwinds_searches_or_forwards() {
        let (mut e, _) = engine();
        nothing(&mut e, &[FRAME_UNWIND_TO_STATE, FIND_HANDLER]);
        let (record, info, _, _, _) = throw_and_function(&mut e);
        let words = |nesting: u32, recursive: u32| {
            [record, 0x60, 0x20, 0x30, info, nesting, 0x77, recursive]
        };
        // Unwinding at nesting 0: to -1.
        e.mem.set_u32(record + 4, 2);
        assert_eq!(e.call(0x00ec_eeb8, &words(0, 0x101)).u32(), 1);
        assert_eq!(
            calls(&e, FRAME_UNWIND_TO_STATE),
            vec![vec![0x60, 0x30, info, u32::MAX]]
        );
        e.call(0x00ec_eeb8, &words(1, 0x101));
        assert_eq!(calls(&e, FRAME_UNWIND_TO_STATE).len(), 1);
        // A search: FindHandler with the recursive word before the nesting.
        e.mem.set_u32(record + 4, 0);
        assert_eq!(e.call(0x00ec_eeb8, &words(2, 0x101)).u32(), 1);
        assert_eq!(
            calls(&e, FIND_HANDLER),
            vec![vec![record, 0x60, 0x20, 0x30, info, 0x101, 2, 0x77]]
        );
        // A newer magic with a forward-compatibility handler: its answer.
        let forward = 0x0900_0000;
        answer(&mut e, forward, 7);
        e.mem.set_u32(record + 0x14, MAGIC_3 + 1);
        let throw_info = e.mem.u32(record + 0x1c);
        e.mem.set_u32(throw_info + 8, forward);
        assert_eq!(e.call(0x00ec_eeb8, &words(2, 0x101)).u32(), 7);
        assert_eq!(
            calls(&e, forward),
            vec![vec![record, 0x60, 0x20, 0x30, info, 2, 0x77, 1]]
        );
        // Synchronous-only functions leave other exceptions alone.
        e.mem.set_u32(record, 0xc000_0005);
        e.mem.set_u32(info, MAGIC_3);
        e.mem.set_u32(info + 0x20, 1);
        assert_eq!(e.call(0x00ec_eeb8, &words(2, 0)).u32(), 1);
        assert_eq!(calls(&e, FIND_HANDLER).len(), 1);
        // No try blocks and no specification: nothing to do.
        e.mem.set_u32(info + 0x20, 0);
        e.mem.set_u32(info + 0xc, 0);
        e.call(0x00ec_eeb8, &words(2, 0));
        assert_eq!(calls(&e, FIND_HANDLER).len(), 1);
    }

    #[test]
    fn call_destruct_exception_object_checks_the_pointer() {
        let (mut e, _) = engine();
        e.map(0x010f_2000, 0x1000);
        let destruct = 0x0900_1000;
        nothing(&mut e, &[destruct]);
        answer(&mut e, IS_NONWRITABLE_IN_CURRENT_IMAGE, 1);
        let record = cxx_record(&mut e, 0, 0);
        e.call(0x00ed_f49f, &[record, 1]);
        assert!(calls(&e, IS_NONWRITABLE_IN_CURRENT_IMAGE).is_empty());
        e.mem.set_u32(DESTRUCT_POINTER, destruct);
        e.call(0x00ed_f49f, &[record, 1]);
        assert_eq!(
            calls(&e, IS_NONWRITABLE_IN_CURRENT_IMAGE),
            vec![vec![DESTRUCT_POINTER]]
        );
        assert_eq!(calls(&e, destruct), vec![vec![record, 1]]);
        answer(&mut e, IS_NONWRITABLE_IN_CURRENT_IMAGE, 0);
        e.call(0x00ed_f49f, &[record, 1]);
        e.mem.set_u32(record, 0xc000_0005);
        e.call(0x00ed_f49f, &[record, 1]);
        assert_eq!(calls(&e, destruct).len(), 1);
    }

    #[test]
    #[should_panic(expected = "terminate returned")]
    fn call_unexpected_runs_unexpected_then_terminate() {
        let (mut e, data) = engine();
        nothing(&mut e, &[UNEXPECTED, INCONSISTENCY]);
        e.mem.set_u32(data + PTD_CUR_EXC_SPEC, 1);
        e.register_double(TERMINATE, |e, _| {
            let order: Vec<u32> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .map(|c| c.0)
                .filter(|a| [INCONSISTENCY, UNEXPECTED].contains(a))
                .collect();
            assert_eq!(order, [INCONSISTENCY, UNEXPECTED]);
            Ret::default()
        });
        e.call(0x00ec_e5fc, &[0x40]);
    }

    #[test]
    #[should_panic(expected = "_CxxThrowException returned")]
    fn call_unexpected_catch_rethrows_with_the_spec_current() {
        let (mut e, data) = engine();
        e.register_double(CXX_THROW_EXCEPTION, move |e, a| {
            assert_eq!(a, [0, 0]);
            assert_eq!(e.mem.u32(data + PTD_CUR_EXC_SPEC), 0x40);
            Ret::default()
        });
        let frame = e.mem.alloc(0x10);
        e.mem.set_u32(frame + 8, 0x40);
        e.call(0x00ec_e62d, &[frame]);
    }
}
