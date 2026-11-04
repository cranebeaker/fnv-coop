//! `(unplaced)/run_009f98e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009f9c80`: the jump target of `009f9d50`.
const JUMP_TARGET_009F9D50: u32 = 0x009f_9c80;

// Translated from 009f9d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `009f9c80`, passing on `this` and its one argument; returns the target result word. Return type (u32): the target `009f9c80` returns EAX at both returns (0, or `0x8007000e`); its meaning is unconfirmed.
pub fn fn_009f9d50(e: &mut Engine, this: Ptr, arg: Ptr) -> u32 {
    e.call(JUMP_TARGET_009F9D50, &args![this, arg]).u32()
}

// Translated from 009fdd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `009fddd0`. `009fddd0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_009fdd50(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x009f_ddd0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// The slot `009f9968` jumps through.
const IMPORT_SLOT_009F9968: u32 = 0x00fd_f3c8;

// Translated from 009f9968 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3c8]` (the import slot; Ghidra labels the import `D3DPERF_SetOptions`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 004dad63 (`PUSH 1`, no stack cleanup after the call).
pub fn fn_009f9968(e: &mut Engine, options: u32) {
    e.call(IMPORT_SLOT_009F9968, &args![options]);
}

/// The slot `009fead0` jumps through.
const POINTER_CELL_009FEAD0: u32 = 0x011a_6238;

// Translated from 009fead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011a6238]`: unnamed in the engine map. The slot holds a function pointer (`0x011a6238` is a data cell, not an import slot); the 2 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 004db36f (`ADD ESP,0x8`); the second word is the address of a local, the first is 0; the result is tested against zero.
pub fn fn_009fead0(e: &mut Engine, word_0: u32, word_1: Ptr) -> u32 {
    let target = e.global::<u32>(POINTER_CELL_009FEAD0);
    e.call(target, &args![word_0, word_1]).u32()
}

/// The slot `009fead6` jumps through.
const POINTER_CELL_009FEAD6: u32 = 0x011a_6240;

// Translated from 009fead6 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011a6240]`: unnamed in the engine map. The slot holds a function pointer (`0x011a6240` is a data cell, not an import slot); the 2 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 004db3e4 (`ADD ESP,0x8`); the first word is a loop index, the second the address of a local array element; the result is tested against zero.
pub fn fn_009fead6(e: &mut Engine, word_0: u32, word_1: Ptr) -> u32 {
    let target = e.global::<u32>(POINTER_CELL_009FEAD6);
    e.call(target, &args![word_0, word_1]).u32()
}

/// The slot `009feae2` jumps through.
const POINTER_CELL_009FEAE2: u32 = 0x011a_6250;

// Translated from 009feae2 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011a6250]`: unnamed in the engine map. The slot holds a function pointer (`0x011a6250` is a data cell, not an import slot); the 2 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 004db444 (`ADD ESP,0x8`); both words are addresses of locals; the result is tested against zero.
pub fn fn_009feae2(e: &mut Engine, word_0: Ptr, word_1: Ptr) -> u32 {
    let target = e.global::<u32>(POINTER_CELL_009FEAE2);
    e.call(target, &args![word_0, word_1]).u32()
}

/// The slot `009feae8` jumps through.
const POINTER_CELL_009FEAE8: u32 = 0x011a_6258;

// Translated from 009feae8 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011a6258]`: unnamed in the engine map. The slot holds a function pointer (`0x011a6258` is a data cell, not an import slot); the 2 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 004db41a (`ADD ESP,0x8`); both words are addresses of locals; the result is tested against zero.
pub fn fn_009feae8(e: &mut Engine, word_0: Ptr, word_1: Ptr) -> u32 {
    let target = e.global::<u32>(POINTER_CELL_009FEAE8);
    e.call(target, &args![word_0, word_1]).u32()
}

// Batch b009: the DirectShow-style base classes this unit holds (media
// types, pins, filters, enumerators, generic lists, an asynchronous reader).
// Class names below come only from the RTTI records of the vtables these
// functions install (`CEnumPins`, `CEnumMediaTypes`, `CAsyncOutputPin`,
// `CAsyncReader`); the other functions are named by address.

/// `_memset` (engine map, LIBCMT), cdecl.
const MEMSET: u32 = 0x00ec_61c0;
/// `_memcpy` (engine map, LIBCMT), cdecl.
const MEMCPY: u32 = 0x00ec_44d0;
/// `__alldiv` (engine map, LIBCMT): signed 64-bit division, callee pops.
const ALLDIV: u32 = 0x00ec_c250;
/// `__allmul` (engine map, LIBCMT): 64-bit multiplication, callee pops.
const ALLMUL: u32 = 0x00ec_c300;
/// `operator new` (crt.rs), cdecl.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (platform.rs), cdecl.
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Import slots (Ghidra's labels for them).
/// `XInputGetState`.
const XINPUT_GET_STATE_SLOT: u32 = 0x00fd_f394;
/// `CoTaskMemAlloc`.
const CO_TASK_MEM_ALLOC: u32 = 0x00fd_f460;
/// `CoTaskMemFree`.
const CO_TASK_MEM_FREE: u32 = 0x00fd_f45c;
/// `InitializeCriticalSection`.
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// `DeleteCriticalSection`.
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// `EnterCriticalSection`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// `LeaveCriticalSection`.
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `InterlockedIncrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement`.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `FreeLibrary`.
const FREE_LIBRARY: u32 = 0x00fd_f0a8;
/// `CreateEventA`.
const CREATE_EVENT_A: u32 = 0x00fd_f0cc;
/// `CloseHandle`.
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// `SetEvent`.
const SET_EVENT: u32 = 0x00fd_f0d0;
/// `ResetEvent`.
const RESET_EVENT: u32 = 0x00fd_f1cc;
/// `WaitForSingleObject`.
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
/// `LoadLibraryA`.
const LOAD_LIBRARY_A: u32 = 0x00fd_f0b0;
/// `GetProcAddress`.
const GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;

// HRESULT values the code returns (Windows SDK names).
const E_POINTER: u32 = 0x8000_4003;
const E_OUTOFMEMORY: u32 = 0x8007_000e;
const E_INVALIDARG: u32 = 0x8007_0057;
const E_NOINTERFACE: u32 = 0x8000_4002;
const E_NOTIMPL: u32 = 0x8000_4001;
const E_FAIL: u32 = 0x8000_4005;
const E_UNEXPECTED: u32 = 0x8000_ffff;
const VFW_S_NO_MORE_ITEMS: u32 = 0x0004_0103;
const VFW_E_ENUM_OUT_OF_SYNC: u32 = 0x8004_0203;
const VFW_E_ALREADY_CONNECTED: u32 = 0x8004_0204;
const VFW_E_NO_ACCEPTABLE_TYPES: u32 = 0x8004_0207;
const VFW_E_INVALID_DIRECTION: u32 = 0x8004_0208;
const VFW_E_NOT_CONNECTED: u32 = 0x8004_0209;
const VFW_E_BADALIGN: u32 = 0x8004_020e;
const VFW_E_NOT_STOPPED: u32 = 0x8004_0224;
const VFW_E_WRONG_STATE: u32 = 0x8004_0227;
const VFW_E_TYPE_NOT_ACCEPTED: u32 = 0x8004_022a;
const VFW_E_TIMEOUT: u32 = 0x8004_022e;
/// The HRESULT `009fd460` returns when its flag is clear (SDK name not
/// confirmed here).
const STATUS_80040266: u32 = 0x8004_0266;

// Interface ids (16-byte GUIDs in `.rdata`) the code compares against.
const INTERFACE_ID_01093B4C: u32 = 0x0109_3b4c;
const INTERFACE_ID_01093B6C: u32 = 0x0109_3b6c;
const INTERFACE_ID_01093B7C: u32 = 0x0109_3b7c;
const INTERFACE_ID_01093B9C: u32 = 0x0109_3b9c;
const INTERFACE_ID_01093BEC: u32 = 0x0109_3bec;
const INTERFACE_ID_01093BFC: u32 = 0x0109_3bfc;
const INTERFACE_ID_01093C3C: u32 = 0x0109_3c3c;
const INTERFACE_ID_01093F04: u32 = 0x0109_3f04;
const INTERFACE_ID_01093F14: u32 = 0x0109_3f14;
const INTERFACE_ID_01093F24: u32 = 0x0109_3f24;

// Vtables (class from the RTTI record before each).
/// `CEnumPins` (RTTI).
const ENUM_PINS_VTABLE: u32 = 0x0109_3c50;
/// `CEnumMediaTypes` (RTTI).
const ENUM_MEDIA_TYPES_VTABLE: u32 = 0x0109_3c74;
/// `CAsyncOutputPin` (RTTI), its four vtables at +0, +8, +0x14, +0x18.
const ASYNC_OUTPUT_PIN_VTABLE: u32 = 0x0109_3e24;
const ASYNC_OUTPUT_PIN_VTABLE_8: u32 = 0x0109_3de8;
const ASYNC_OUTPUT_PIN_VTABLE_14: u32 = 0x0109_3d9c;
const ASYNC_OUTPUT_PIN_VTABLE_18: u32 = 0x0109_3d84;
/// `CAsyncReader` (RTTI), its three vtables at +0, +0xc, +0x10.
const ASYNC_READER_VTABLE: u32 = 0x0109_3eac;
const ASYNC_READER_VTABLE_C: u32 = 0x0109_3e6c;
const ASYNC_READER_VTABLE_10: u32 = 0x0109_3e54;

/// The 16-byte id `009fddf0` passes to the filter base (copied to +0x28).
const CLASS_ID_01093D70: u32 = 0x0109_3d70;
/// The wide name `009fdd70` passes to the pin base.
const NAME_STRING_01093ED4: u32 = 0x0109_3ed4;

/// The object count `009fce60`/`009fce80` keep, and the module handle
/// `009fce80` frees when it drops to zero.
const OBJECT_COUNT: u32 = 0x011f_3298;
const OBJECT_COUNT_MODULE: u32 = 0x011f_3294;

/// `009fea2a`'s globals: done flag, status, the first table entry's cell,
/// and the (function cell, id) table that starts at `011a6228`.
const LIBRARY_READY: u32 = 0x011a_621c;
const LIBRARY_STATUS: u32 = 0x011a_6220;
const LIBRARY_FIRST_CELL: u32 = 0x011a_6224;
const LIBRARY_TABLE: u32 = 0x011a_6228;
/// `"nvapi.dll"` and `"nvapi_QueryInterface"` (`.rdata`).
const LIBRARY_NAME: u32 = 0x0127_3000;
const LIBRARY_PROC_NAME: u32 = 0x0127_300a;
/// The id word `009fea2a` passes first to the procedure it resolves.
const LIBRARY_FIRST_ID: u32 = 0x0150_e828;
/// The table cell `009febd8` jumps through.
const LIBRARY_CELL_009FEBD8: u32 = 0x011a_6398;

/// Callees outside this file's translations, by address.
const HEAD_POSITION: u32 = 0x009f_d0f0;
const LIST_GET_AT: u32 = 0x009f_d1a0;
const LIST_GET_NEXT: u32 = 0x009f_d180;
const LIST_APPEND_009FD2D0: u32 = 0x009f_d2d0;
const ENUM_PINS_REFRESH: u32 = 0x009f_a0d0;
const TYPE_TEST_009F9A50: u32 = 0x009f_9a50;
const TYPE_TEST_009F9A80: u32 = 0x009f_9a80;
const WIDE_STRING_DUPLICATE: u32 = 0x009f_d0a0;
const ALLOCATOR_MAKE_009FD5A0: u32 = 0x009f_d5a0;
const REQUEST_FINISH_009FE3F0: u32 = 0x009f_e3f0;
const RETURN_ZERO_00A38DE0: u32 = 0x00a3_8de0;
const RETURN_ZERO_00E7CD40: u32 = 0x00e7_cd40;

/// 100-nanosecond units per second (`0x989680`).
const TIME_UNITS: u32 = 10_000_000;

/// The word at `base + offset`.
fn field(e: &Engine, base: Ptr, offset: u32) -> u32 {
    e.mem.u32(base.addr().wrapping_add(offset))
}

/// Writes the word at `base + offset`.
fn set_field(e: &mut Engine, base: Ptr, offset: u32, value: u32) {
    e.mem.set_u32(base.addr().wrapping_add(offset), value);
}

/// `base + offset` as a pointer.
fn offset(base: Ptr, by: u32) -> Ptr {
    Ptr::new(base.addr().wrapping_add(by))
}

/// `base - by` as a pointer (`ADD reg,-by` / `LEA reg,[reg - by]`).
fn back(base: Ptr, by: u32) -> Ptr {
    Ptr::new(base.addr().wrapping_sub(by))
}

/// An HRESULT's sign test (`TEST EAX,EAX; JL`).
fn failed(hr: u32) -> bool {
    (hr as i32) < 0
}

fn enter(e: &mut Engine, section: u32) {
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
}

fn leave(e: &mut Engine, section: u32) {
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

/// The byte size of `count` wide characters as the exe computes it for
/// `operator new`: `MUL` by 2, and all ones when the product overflows
/// (`SETO`/`NEG`/`OR`).
fn wide_bytes(count: u32) -> u32 {
    count.checked_mul(2).unwrap_or(u32::MAX)
}

/// Copies the wide string `name` into a new block (`009fd080` for its
/// length, `operator new`, `memcpy` of the characters and the terminator)
/// and returns the block, or 0 when the allocation failed. Shared by
/// `009f9e50` and `009fc890`, which inline the same code.
fn duplicate_name(e: &mut Engine, name: Ptr) -> u32 {
    let count = fn_009fd080(e, name).wrapping_add(1);
    let block = e.call(OPERATOR_NEW, &args![wide_bytes(count)]).u32();
    if block != 0 {
        // LEA ECX,[ESI + ESI*1]: the copy size wraps instead.
        e.call(MEMCPY, &args![block, name, count.wrapping_add(count)]);
    }
    block
}

// Translated from 009f996e (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf394]` (Ghidra labels the import
/// `XInputGetState`): stdcall, the two words (the caller at 00a2304c pushes
/// a user index and the address of a state block) go unchanged to the import
/// and its result comes back.
pub fn fn_009f996e(e: &mut Engine, user_index: u32, state: Ptr) -> u32 {
    e.call(XINPUT_GET_STATE_SLOT, &args![user_index, state])
        .u32()
}

// Translated from 009f9980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares two 16-byte ids (stdcall, `RET 8`): word by word while four or
/// more bytes are left, then byte by byte (unrolled four times) from the
/// first differing word. The byte difference is folded to 1 or -1 and the
/// result is `SETZ` of that: 1 when all 16 bytes match, 0 otherwise.
pub fn fn_009f9980(e: &mut Engine, first: Ptr, second: Ptr) -> u32 {
    let mut a = first.addr();
    let mut b = second.addr();
    let mut left: u32 = 0x10;
    let mut word_differs = false;
    loop {
        if e.mem.u32(a) != e.mem.u32(b) {
            word_differs = true;
            break;
        }
        left -= 4;
        b = b.wrapping_add(4);
        a = a.wrapping_add(4);
        if left < 4 {
            break;
        }
    }
    if !word_differs && left == 0 {
        return 1;
    }
    let mut difference: i32 = 0;
    for step in 0..4 {
        difference = i32::from(e.mem.u8(a)) - i32::from(e.mem.u8(b));
        // The fourth compare has no count test after it.
        if difference != 0 || step == 3 {
            break;
        }
        left -= 1;
        b = b.wrapping_add(1);
        a = a.wrapping_add(1);
        if left == 0 {
            break;
        }
    }
    // MOV EAX,1 / OR EAX,-1 by the difference's sign (never 0), then SETZ.
    u32::from(difference == 0)
}

// Translated from 009f9a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a 0x48-byte media-type block (`memset`) and sets the words at
/// +0x28 and +0x20 to 1. `this` in `ECX`, no stack words.
pub fn fn_009f9a30(e: &mut Engine, this: Ptr) {
    e.call(MEMSET, &args![this, 0u32, 0x48u32]);
    set_field(e, this, 0x28, 1);
    set_field(e, this, 0x20, 1);
}

// Translated from 009f9bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 0x48-byte media-type block `source` into `target` (`REP
/// MOVSD`, 0x12 words); when the copied size at +0x40 is non-zero, gives
/// `target` its own `CoTaskMemAlloc` copy of the format block (+0x44), or
/// clears the size and returns `E_OUTOFMEMORY` when that fails; then adds a
/// reference (vtable +4) to the object at +0x3c, if any. Returns 0.
pub fn fn_009f9bb0(e: &mut Engine, target: Ptr, source: Ptr) -> u32 {
    for i in 0..0x12u32 {
        let w = field(e, source, 4 * i);
        set_field(e, target, 4 * i, w);
    }
    let size = field(e, source, 0x40);
    if size != 0 {
        let block = e.call(CO_TASK_MEM_ALLOC, &args![size]).u32();
        set_field(e, target, 0x44, block);
        if block == 0 {
            set_field(e, target, 0x40, 0);
            return E_OUTOFMEMORY;
        }
        let format = field(e, source, 0x44);
        let length = field(e, target, 0x40);
        e.call(MEMCPY, &args![block, format, length]);
    }
    let unknown = field(e, target, 0x3c);
    if unknown != 0 {
        e.vcall(unknown, 0x4, &args![]);
    }
    0
}

// Translated from 009f9c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a media-type block's contents (stdcall, one word): the format
/// block (+0x44, `CoTaskMemFree`) when its size (+0x40) is non-zero, both
/// words then cleared; and releases (vtable +8) and clears the object at
/// +0x3c.
pub fn fn_009f9c20(e: &mut Engine, media_type: Ptr) {
    if field(e, media_type, 0x40) != 0 {
        let format = field(e, media_type, 0x44);
        e.call(CO_TASK_MEM_FREE, &args![format]);
        set_field(e, media_type, 0x40, 0);
        set_field(e, media_type, 0x44, 0);
    }
    let unknown = field(e, media_type, 0x3c);
    if unknown != 0 {
        e.vcall(unknown, 0x8, &args![]);
        set_field(e, media_type, 0x3c, 0);
    }
}

// Translated from 009f9c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this` (`ECX`) passed to `009f9c20`: the media-type block's destructor
/// shape.
pub fn fn_009f9c60(e: &mut Engine, this: Ptr) {
    fn_009f9c20(e, this);
}

// Translated from 009f9c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Media-type block constructor shape: `009f9a30` on `this`, returns `this`.
pub fn fn_009f9c70(e: &mut Engine, this: Ptr) -> Ptr {
    fn_009f9a30(e, this);
    this
}

// Translated from 009f9c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the media-type block `source` to `this` (thiscall, `RET 4`):
/// unless they are the same block, frees `this`'s contents (`009f9c20`) and
/// copies (`009f9bb0`); a failed copy gives `E_OUTOFMEMORY`. Returns 0
/// otherwise.
pub fn fn_009f9c80(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    if source != this {
        fn_009f9c20(e, this);
        if failed(fn_009f9bb0(e, this, source)) {
            return E_OUTOFMEMORY;
        }
    }
    0
}

// Translated from 009f9cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes a task-allocated media-type block (stdcall, one word): when not
/// null, frees its contents (`009f9c20`) and the block (`CoTaskMemFree`).
pub fn fn_009f9cb0(e: &mut Engine, media_type: Ptr) {
    if !media_type.is_null() {
        fn_009f9c20(e, media_type);
        e.call(CO_TASK_MEM_FREE, &args![media_type]);
    }
}

// Translated from 009f9d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009f9c80` (assign `source` to `this`), returning `this`.
pub fn fn_009f9d10(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    fn_009f9c80(e, this, source);
    this
}

// Translated from 009f9d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009f9d10` on `this` and `source`, returning `this`.
pub fn fn_009f9d30(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    fn_009f9d10(e, this, source);
    this
}

// Translated from 009f9d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 16 bytes at `this + 0x1c` to `out` (stdcall, `RET 8`):
/// `E_POINTER` for a null `out`, otherwise 0.
pub fn fn_009f9d60(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    for i in 0..4u32 {
        let w = field(e, this, 0x1c + 4 * i);
        set_field(e, out, 4 * i, w);
    }
    0
}

// Translated from 009f9d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word at `this + 8` in `out` (stdcall, `RET 0xc`; the middle
/// word is not read): `E_POINTER` for a null `out`, otherwise 0.
pub fn fn_009f9d90(e: &mut Engine, this: Ptr, _unused_1: u32, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let value = field(e, this, 0x8);
    e.mem.set_u32(out.addr(), value);
    0
}

// Translated from 009f9df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a name-and-object block (stdcall, `RET 8`): the wide name at
/// `this + 0x30` copied into the first 0x80 characters (`009fcff0`), or an
/// empty string; the object at `this + 0x34` stored at `out + 0x100` with a
/// reference added (vtable +4) when not null. `E_POINTER` for a null `out`,
/// otherwise 0.
pub fn fn_009f9df0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let name = field(e, this, 0x30);
    if name != 0 {
        fn_009fcff0(e, out, Ptr::new(name), 0x80);
    } else {
        e.mem.set_u16(out.addr(), 0);
    }
    let object = field(e, this, 0x34);
    set_field(e, out, 0x100, object);
    if field(e, this, 0x34) != 0 {
        let object = field(e, this, 0x34);
        e.vcall(object, 0x4, &args![]);
    }
    0
}

// Translated from 009f9e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f9e50 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Joins an owner (stdcall, `RET 0xc`), under the critical section at
/// `this + 0x2c`: stores `owner` at +0x34; asks it (vtable +0, the
/// interface id at `01093b6c`) for an interface into +0x38 and on success
/// releases that interface again (vtable +8), or clears +0x38 for a null
/// owner; deletes the old wide name at +0x30 and stores a copy of `name`
/// (when not null; 0 if its allocation failed). Returns 0.
pub fn fn_009f9e50(e: &mut Engine, this: Ptr, owner: Ptr, name: Ptr) -> u32 {
    let section = field(e, this, 0x2c);
    enter(e, section);
    set_field(e, this, 0x34, owner.addr());
    if owner.is_null() {
        set_field(e, this, 0x38, 0);
    } else {
        let slot = offset(this, 0x38);
        let hr = e
            .vcall(owner.addr(), 0x0, &args![INTERFACE_ID_01093B6C, slot])
            .u32();
        if !failed(hr) {
            let sink = field(e, this, 0x38);
            e.vcall(sink, 0x8, &args![]);
        }
    }
    let old = field(e, this, 0x30);
    if old != 0 {
        e.call(OPERATOR_DELETE, &args![old]);
        set_field(e, this, 0x30, 0);
    }
    if !name.is_null() {
        let block = duplicate_name(e, name);
        set_field(e, this, 0x30, block);
    }
    leave(e, section);
    0
}

// Translated from 009f9f70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f9f70 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor shape of the `CEnumPins` object (RTTI of the vtable it
/// installs at +0): releases (vtable +8) the interface at `+0xc` of the
/// object held at `this + 0xc`, then runs `009fd320` on the list at
/// `this + 0x18`.
pub fn fn_009f9f70(e: &mut Engine, this: Ptr) {
    set_field(e, this, 0x0, ENUM_PINS_VTABLE);
    let owner = field(e, this, 0xc);
    e.vcall(owner.wrapping_add(0xc), 0x8, &args![]);
    fn_009fd320(e, offset(this, 0x18));
}

// Translated from 009fa060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reference release (stdcall, `RET 4`): `InterlockedDecrement` of the
/// count at `this + 0x14`; at zero (and a non-null `this`) the object's
/// vtable +0x1c is called with 1 (thiscall). Returns the new count.
pub fn fn_009fa060(e: &mut Engine, this: Ptr) -> u32 {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![offset(this, 0x14)])
        .u32();
    if count == 0 && !this.is_null() {
        e.vcall(this.addr(), 0x1c, &args![1u32]);
    }
    count
}

// Translated from 009fa100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor shape of the `CEnumMediaTypes` object (RTTI of the vtable it
/// installs at +0): releases (vtable +8) the interface at `+0xc` of the pin
/// held at `this + 8`.
pub fn fn_009fa100(e: &mut Engine, this: Ptr) {
    set_field(e, this, 0x0, ENUM_MEDIA_TYPES_VTABLE);
    let pin = field(e, this, 0x8);
    e.vcall(pin.wrapping_add(0xc), 0x8, &args![]);
}

// Translated from 009fa1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reference release (stdcall, `RET 4`) with the count at `this + 0x10`:
/// `InterlockedDecrement`; at zero (and a non-null `this`) the object's
/// vtable +0x1c is called with 1 (thiscall). Returns the new count.
pub fn fn_009fa1a0(e: &mut Engine, this: Ptr) -> u32 {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![offset(this, 0x10)])
        .u32();
    if count == 0 && !this.is_null() {
        e.vcall(this.addr(), 0x1c, &args![1u32]);
    }
    count
}

// Translated from 009fa1f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fa1f0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Pin destructor shape: deletes the wide name at `this + 0x14`, frees the
/// media-type block at `this + 0x34` (`009f9c60`) and drops the object
/// count (`009fce80`).
pub fn fn_009fa1f0(e: &mut Engine, this: Ptr) {
    let name = field(e, this, 0x14);
    e.call(OPERATOR_DELETE, &args![name]);
    fn_009f9c60(e, offset(this, 0x34));
    fn_009fce80(e);
}

/// `009fceb0` on `this + by` (or on 0 for a null `this`), as the
/// interface lookups below write it.
fn interface_of(e: &mut Engine, this: Ptr, by: u32, out: Ptr) -> u32 {
    if !this.is_null() {
        fn_009fceb0(e, offset(this, by), out)
    } else {
        fn_009fceb0(e, Ptr::NULL, out)
    }
}

// Translated from 009fa260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Interface lookup (stdcall, `RET 0xc`): for the id at `01093c3c` hands
/// out `this + 0xc`, for the id at `01093b7c` `this + 0x10` (each through
/// `009fceb0`, 0 for a null `this`); any other id goes to `009fcf10`.
pub fn fn_009fa260(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    if fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093C3C)) != 0 {
        return interface_of(e, this, 0xc, out);
    }
    if fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093B7C)) == 0 {
        return fn_009fcf10(e, this, id, out);
    }
    interface_of(e, this, 0x10, out)
}

// Translated from 009fa2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Delegating thunk: replaces its one stack word with `[this + 0x28] + 0xc`
/// and jumps to that interface's vtable +4 (the reference-add slot of the
/// COM layout; one word).
pub fn fn_009fa2f0(e: &mut Engine, this: Ptr) -> u32 {
    let target = field(e, this, 0x28).wrapping_add(0xc);
    e.vcall(target, 0x4, &args![]).u32()
}

// Translated from 009fa310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Delegating thunk: replaces its one stack word with `[this + 0x28] + 0xc`
/// and jumps to that interface's vtable +8 (the release slot of the COM
/// layout; one word).
pub fn fn_009fa310(e: &mut Engine, this: Ptr) -> u32 {
    let target = field(e, this, 0x28).wrapping_add(0xc);
    e.vcall(target, 0x8, &args![]).u32()
}

// Translated from 009fa330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `source` to the media-type block at `this + 0x34` (`009f9d50`)
/// and returns its result when negative, 0 otherwise (`SETGE`/`SUB`/`AND`).
pub fn fn_009fa330(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    let hr = fn_009f9d50(e, offset(this, 0x34), source);
    if failed(hr) {
        hr
    } else {
        0
    }
}

// Translated from 009fa350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks `pin` (vtable +0x24, stdcall) for a word, written over the stack
/// slot that held `pin`, and returns `VFW_E_INVALID_DIRECTION` when that
/// word equals `this + 0x1c`, 0 otherwise.
pub fn fn_009fa350(e: &mut Engine, this: Ptr, pin: Ptr) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), pin.addr());
        e.vcall(pin.addr(), 0x24, &args![slot]);
        if e.mem.u32(slot.addr()) != field(e, this, 0x1c) {
            0
        } else {
            VFW_E_INVALID_DIRECTION
        }
    })
}

// Translated from 009fa380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Disconnect shape (stdcall, `RET 4`): 1 when nothing is held at
/// `this + 0x18`; otherwise calls `this`'s vtable +0x2c (thiscall) and on
/// success releases (vtable +8) and clears the held object and returns 0;
/// a negative result is returned as is.
pub fn fn_009fa380(e: &mut Engine, this: Ptr) -> u32 {
    if field(e, this, 0x18) == 0 {
        return 1;
    }
    let hr = e.vcall(this.addr(), 0x2c, &args![]).u32();
    if failed(hr) {
        return hr;
    }
    let held = field(e, this, 0x18);
    e.vcall(held, 0x8, &args![]);
    set_field(e, this, 0x18, 0);
    0
}

// Translated from 009fa3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands out the object at `this + 0xc` (stdcall, `RET 8`): `E_POINTER` for
/// a null `out`; otherwise stores it, adds a reference (vtable +4) and
/// returns 0, or `VFW_E_NOT_CONNECTED` when it is null.
pub fn fn_009fa3c0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let object = field(e, this, 0xc);
    e.mem.set_u32(out.addr(), object);
    if object != 0 {
        e.vcall(object, 0x4, &args![]);
        return 0;
    }
    VFW_E_NOT_CONNECTED
}

// Translated from 009fa400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a pin-information block (stdcall, `RET 8`): +0 the interface at
/// `+0xc` of the object at `this + 0x1c` (0 when none) with a reference
/// added (vtable +4); +8 the wide name at `this + 8` (`009fcff0`, 0x80
/// characters) or an empty string; +4 the word at `this + 0x10`.
/// `E_POINTER` for a null `out`, otherwise 0.
pub fn fn_009fa400(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let owner = field(e, this, 0x1c);
    let interface = if owner != 0 {
        owner.wrapping_add(0xc)
    } else {
        0
    };
    e.mem.set_u32(out.addr(), interface);
    if field(e, this, 0x1c) != 0 {
        let owner = field(e, this, 0x1c);
        e.vcall(owner.wrapping_add(0xc), 0x4, &args![]);
    }
    let name = field(e, this, 0x8);
    if name != 0 {
        fn_009fcff0(e, offset(out, 0x8), Ptr::new(name), 0x80);
    } else {
        e.mem.set_u16(out.addr().wrapping_add(0x8), 0);
    }
    let direction = field(e, this, 0x10);
    set_field(e, out, 0x4, direction);
    0
}

// Translated from 009fa480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word at `this + 0x10` in `out` (stdcall, `RET 8`):
/// `E_POINTER` for a null `out`, otherwise 0.
pub fn fn_009fa480(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let value = field(e, this, 0x10);
    e.mem.set_u32(out.addr(), value);
    0
}

// Translated from 009fa4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at `this + 0x24` and returns 0.
pub fn fn_009fa4d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u8(this.addr().wrapping_add(0x24), 0);
    0
}

// Translated from 009faba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `E_NOTIMPL` (stdcall, `RET 0xc`; none of its three words is read).
pub fn fn_009faba0(_e: &mut Engine, _unused_0: u32, _unused_1: u32, _unused_2: u32) -> u32 {
    E_NOTIMPL
}

// Translated from 009fb480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Interface lookup (stdcall, `RET 0xc`): the ids at `01093bec`, `01093bfc`
/// and `01093f24` hand out `this + 0xc`, the id at `01093b9c` `this + 0x10`
/// (each through `009fceb0`, 0 for a null `this`); any other id goes to
/// `009fcf10`.
pub fn fn_009fb480(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    if fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093BEC)) != 0 {
        return interface_of(e, this, 0xc, out);
    }
    if fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093BFC)) != 0 {
        return interface_of(e, this, 0xc, out);
    }
    if fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093F24)) != 0 {
        return interface_of(e, this, 0xc, out);
    }
    if fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093B9C)) != 0 {
        return interface_of(e, this, 0x10, out);
    }
    fn_009fcf10(e, this, id, out)
}

// Translated from 009fb550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fb550 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Filter destructor shape: deletes the wide name at `this + 0x3c`,
/// releases (vtable +8) and clears the object at `this + 0x18` when set, and
/// drops the object count (`009fce80`).
pub fn fn_009fb550(e: &mut Engine, this: Ptr) {
    let name = field(e, this, 0x3c);
    e.call(OPERATOR_DELETE, &args![name]);
    let held = field(e, this, 0x18);
    if held != 0 {
        e.vcall(held, 0x8, &args![]);
        set_field(e, this, 0x18, 0);
    }
    fn_009fce80(e);
}

// Translated from 009fb5d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fb5d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Replaces the object at `this + 0xc` with `object` (stdcall, `RET 8`),
/// under the critical section at `this + 0x2c`: a reference is added to
/// `object` (vtable +4) when not null and the old one released (vtable +8)
/// when set. Returns 0.
pub fn fn_009fb5d0(e: &mut Engine, this: Ptr, object: Ptr) -> u32 {
    let section = field(e, this, 0x2c);
    enter(e, section);
    if !object.is_null() {
        e.vcall(object.addr(), 0x4, &args![]);
    }
    let old = field(e, this, 0xc);
    if old != 0 {
        e.vcall(old, 0x8, &args![]);
    }
    set_field(e, this, 0xc, object.addr());
    leave(e, section);
    0
}

/// The pin count (vtable +0x18) and the pin at an index (vtable +0x1c) of
/// the filter object `filter` (thiscall).
fn pin_count(e: &mut Engine, filter: Ptr) -> i32 {
    e.vcall(filter.addr(), 0x18, &args![]).i32()
}

fn pin_at(e: &mut Engine, filter: Ptr, index: i32) -> u32 {
    e.vcall(filter.addr(), 0x1c, &args![index]).u32()
}

// Translated from 009fb6f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fb6f0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Stop shape (stdcall, `RET 4`; `this` is the interface at `+0xc` of the
/// filter object): under the critical section at `this + 0x2c`, when the
/// state word at `this + 8` is non-zero, calls vtable +0x18 (thiscall) of
/// every pin of the filter whose `+0x18` is set and keeps the first
/// negative result; the state is set to 0. Returns that result (0 if none).
pub fn fn_009fb6f0(e: &mut Engine, this: Ptr) -> u32 {
    let section = field(e, this, 0x2c);
    enter(e, section);
    let mut result: u32 = 0;
    if field(e, this, 0x8) != 0 {
        let filter = back(this, 0xc);
        let count = pin_count(e, filter);
        let mut index: i32 = 0;
        while index < count {
            let pin = pin_at(e, filter, index);
            if field(e, Ptr::new(pin), 0x18) != 0 {
                let hr = e.vcall(pin, 0x18, &args![]).u32();
                if failed(hr) && !failed(result) {
                    result = hr;
                }
            }
            index += 1;
        }
    }
    set_field(e, this, 0x8, 0);
    leave(e, section);
    result
}

// Translated from 009fb7a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fb7a0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Pause shape (stdcall, `RET 4`; `this` is the interface at `+0xc` of the
/// filter object): under the critical section at `this + 0x2c`, when the
/// state word at `this + 8` is 0, calls vtable +0x14 (thiscall) of every pin
/// whose `+0x18` is set and returns the first negative result at once
/// (state unchanged); otherwise sets the state to 1 and returns 0.
pub fn fn_009fb7a0(e: &mut Engine, this: Ptr) -> u32 {
    let section = field(e, this, 0x2c);
    enter(e, section);
    if field(e, this, 0x8) == 0 {
        let filter = back(this, 0xc);
        let count = pin_count(e, filter);
        let mut index: i32 = 0;
        while index < count {
            let pin = pin_at(e, filter, index);
            if field(e, Ptr::new(pin), 0x18) != 0 {
                let hr = e.vcall(pin, 0x14, &args![]).u32();
                if failed(hr) {
                    leave(e, section);
                    return hr;
                }
            }
            index += 1;
        }
    }
    set_field(e, this, 0x8, 1);
    leave(e, section);
    0
}

// Translated from 009fb860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fb860 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Run shape (stdcall, `RET 0xc`: `this` and a 64-bit start time; `this`
/// is the interface at `+0xc` of the filter object): under the critical
/// section at `this + 0x2c`, stores the start time at +0x14/+0x18; from
/// state 0 first calls `this`'s vtable +0x14 (stdcall) and returns a
/// negative result; unless the state is already 2, calls vtable +0x1c
/// (thiscall, the start time) of every pin whose `+0x18` is set and returns
/// the first negative result at once; then sets the state to 2 and
/// returns 0.
pub fn fn_009fb860(e: &mut Engine, this: Ptr, start: u64) -> u32 {
    let section = field(e, this, 0x2c);
    enter(e, section);
    set_field(e, this, 0x14, start as u32);
    set_field(e, this, 0x18, (start >> 32) as u32);
    if field(e, this, 0x8) == 0 {
        let hr = e.vcall(this.addr(), 0x14, &args![]).u32();
        if failed(hr) {
            leave(e, section);
            return hr;
        }
    }
    if field(e, this, 0x8) != 2 {
        let filter = back(this, 0xc);
        let count = pin_count(e, filter);
        let mut index: i32 = 0;
        while index < count {
            let pin = pin_at(e, filter, index);
            if field(e, Ptr::new(pin), 0x18) != 0 {
                let hr = e.vcall(pin, 0x1c, &args![start]).u32();
                if failed(hr) {
                    leave(e, section);
                    return hr;
                }
            }
            index += 1;
        }
    }
    set_field(e, this, 0x8, 2);
    leave(e, section);
    0
}

// Translated from 009fbb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the `CEnumPins` object: `009f9f70`, then
/// `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_009fbb30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009f9f70(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009fbb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pin enumerator next (stdcall, `RET 0x10`): `E_POINTER` for a null
/// `pins`; with a null `fetched` only a count of 0 or 1 is allowed
/// (`E_INVALIDARG`), otherwise `*fetched` is cleared first. Refreshes
/// (`009fa0d0`) when the filter's version (vtable +0x14 of the object at
/// `this + 0xc`) differs from `this + 0x10`. Takes up to `count` pins from
/// the position at `this + 4` towards the total at `this + 8`: each pin
/// (filter vtable +0x1c) not yet in the list at `this + 0x18` (`009fd1c0`)
/// is handed out as its interface at +0xc with a reference added (vtable
/// +4) and added to the list (`009fd270`); a null pin gives
/// `VFW_E_ENUM_OUT_OF_SYNC`. Returns 1 when nothing could be taken or fewer
/// than `count` were handed out, 0 otherwise; `*fetched` gets the number
/// handed out.
pub fn fn_009fbb50(e: &mut Engine, this: Ptr, count: u32, pins: Ptr, fetched: Ptr) -> u32 {
    if pins.is_null() {
        return E_POINTER;
    }
    if !fetched.is_null() {
        e.mem.set_u32(fetched.addr(), 0);
    } else if count > 1 {
        return E_INVALIDARG;
    }
    let mut handed: u32 = 0;
    let filter = field(e, this, 0xc);
    let version = e.vcall(filter, 0x14, &args![]).u32();
    if version != field(e, this, 0x10) {
        e.call(ENUM_PINS_REFRESH, &args![this]);
    }
    let mut left = field(e, this, 0x8).wrapping_sub(field(e, this, 0x4)) as i32;
    if left >= count as i32 {
        left = count as i32;
    }
    if left == 0 {
        return 1;
    }
    let mut cursor = pins.addr();
    loop {
        let position = field(e, this, 0x4);
        if field(e, this, 0x8).wrapping_sub(position) == 0 {
            break;
        }
        let filter = field(e, this, 0xc);
        set_field(e, this, 0x4, position.wrapping_add(1));
        let pin = e.vcall(filter, 0x1c, &args![position]).u32();
        if pin == 0 {
            return VFW_E_ENUM_OUT_OF_SYNC;
        }
        let list = offset(this, 0x18);
        if fn_009fd1c0(e, list, Ptr::new(pin)).is_null() {
            let interface = pin.wrapping_add(0xc);
            e.mem.set_u32(cursor, interface);
            e.vcall(interface, 0x4, &args![]);
            handed = handed.wrapping_add(1);
            cursor = cursor.wrapping_add(4);
            fn_009fd270(e, list, Ptr::new(pin));
            left = left.wrapping_sub(1);
        }
        if left == 0 {
            break;
        }
    }
    if !fetched.is_null() {
        e.mem.set_u32(fetched.addr(), handed);
    }
    u32::from(count != handed)
}

// Translated from 009fbca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the `CEnumMediaTypes` object: `009fa100`, then
/// `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_009fbca0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009fa100(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009fbcc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fbcc0 - C++ exception unwinding (FS:[0] frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// Media-type enumerator next (stdcall, `RET 0x10`): `E_POINTER` for a
/// null `types`; `VFW_E_ENUM_OUT_OF_SYNC` when the pin's version (vtable
/// +0x10 of the pin at `this + 8`) differs from `this + 0xc`; with a null
/// `fetched` only a count of 0 or 1 is allowed (`E_INVALIDARG`), otherwise
/// `*fetched` is cleared first. For each of `count` types: a local
/// media-type block (`009f9c70`) is filled by the pin's vtable +0x34
/// (thiscall, the position at `this + 4`, then incremented); on success it
/// is copied into a new `CoTaskMemAlloc` block of 0x48 bytes handed out at
/// the cursor (the local's +0x3c..+0x44 cleared so only the copy owns
/// them); the local is freed (`009f9c60`). Stops at a failed fill or
/// allocation. `*fetched` gets the number handed out; returns 1 when not all
/// `count` were, 0 otherwise.
pub fn fn_009fbcc0(e: &mut Engine, this: Ptr, count: u32, types: Ptr, fetched: Ptr) -> u32 {
    if types.is_null() {
        return E_POINTER;
    }
    let pin = field(e, this, 0x8);
    let version = e.vcall(pin, 0x10, &args![]).u32();
    if version != field(e, this, 0xc) {
        return VFW_E_ENUM_OUT_OF_SYNC;
    }
    let mut left = count;
    if !fetched.is_null() {
        e.mem.set_u32(fetched.addr(), 0);
    } else if left > 1 {
        return E_INVALIDARG;
    }
    let mut handed: u32 = 0;
    let mut cursor = types.addr();
    while left != 0 {
        let more = e.with_stack(0x48, |e, local| {
            fn_009f9c70(e, local);
            let position = field(e, this, 0x4);
            set_field(e, this, 0x4, position.wrapping_add(1));
            let pin = field(e, this, 0x8);
            let hr = e.vcall(pin, 0x34, &args![position, local]).u32();
            if hr != 0 {
                fn_009f9c60(e, local);
                return false;
            }
            let block = e.call(CO_TASK_MEM_ALLOC, &args![0x48u32]).u32();
            e.mem.set_u32(cursor, block);
            if block == 0 {
                fn_009f9c60(e, local);
                return false;
            }
            cursor = cursor.wrapping_add(4);
            handed = handed.wrapping_add(1);
            for i in 0..0x12u32 {
                let w = field(e, local, 4 * i);
                e.mem.set_u32(block.wrapping_add(4 * i), w);
            }
            set_field(e, local, 0x44, 0);
            set_field(e, local, 0x40, 0);
            set_field(e, local, 0x3c, 0);
            left = left.wrapping_sub(1);
            fn_009f9c60(e, local);
            true
        });
        if !more {
            break;
        }
    }
    if !fetched.is_null() {
        e.mem.set_u32(fetched.addr(), handed);
    }
    u32::from(left != 0)
}

/// The three results `009fbfc0` and `009fc0b0` do not keep as their own
/// error: `E_FAIL`, `E_INVALIDARG`, `VFW_E_TYPE_NOT_ACCEPTED`.
fn is_generic_failure(hr: u32) -> bool {
    hr == E_FAIL || hr == E_INVALIDARG || hr == VFW_E_TYPE_NOT_ACCEPTED
}

// Translated from 009fbf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Connection attempt (thiscall, `RET 8`): `this`'s vtable +0x28 (thiscall,
/// `receiver`) must succeed, else vtable +0x2c and that result; vtable +0x20
/// (`media_type`) must give exactly 0: a positive result, `E_FAIL` or
/// `E_INVALIDARG` become `VFW_E_TYPE_NOT_ACCEPTED`. Then the receiver is
/// stored at `this + 0x18` with a reference added (vtable +4), `this`'s
/// vtable +0x24 (`media_type`), the receiver's vtable +0x10 (stdcall, the
/// interface at `this + 0xc` and `media_type`) and `this`'s vtable +0x30
/// (`receiver`) must succeed; the last one's result is returned (after a
/// failure there the receiver's vtable +0x14 runs too). Any failure after
/// the first calls vtable +0x2c and releases and clears `this + 0x18`.
pub fn fn_009fbf00(e: &mut Engine, this: Ptr, receiver: Ptr, media_type: Ptr) -> u32 {
    let hr = e.vcall(this.addr(), 0x28, &args![receiver]).u32();
    if failed(hr) {
        e.vcall(this.addr(), 0x2c, &args![]);
        return hr;
    }
    let mut hr = e.vcall(this.addr(), 0x20, &args![media_type]).u32();
    if hr == 0 {
        set_field(e, this, 0x18, receiver.addr());
        e.vcall(receiver.addr(), 0x4, &args![]);
        hr = e.vcall(this.addr(), 0x24, &args![media_type]).u32();
        if !failed(hr) {
            let interface = offset(this, 0xc);
            hr = e
                .vcall(receiver.addr(), 0x10, &args![interface, media_type])
                .u32();
            if !failed(hr) {
                hr = e.vcall(this.addr(), 0x30, &args![receiver]).u32();
                if !failed(hr) {
                    return hr;
                }
                e.vcall(receiver.addr(), 0x14, &args![]);
            }
        }
    } else if !failed(hr) || hr == E_FAIL || hr == E_INVALIDARG {
        hr = VFW_E_TYPE_NOT_ACCEPTED;
    }
    e.vcall(this.addr(), 0x2c, &args![]);
    let held = field(e, this, 0x18);
    if held != 0 {
        e.vcall(held, 0x8, &args![]);
        set_field(e, this, 0x18, 0);
    }
    hr
}

// Translated from 009fbfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries the media types of an enumerator (thiscall, `RET 0xc`): resets it
/// (vtable +0x14, stdcall; a negative result is returned); then for each
/// type fetched one at a time (vtable +0xc: count 1, the type pointer
/// written over the stack slot of `enumerator`, a fetched count in a local)
/// while that call returns 0: a type for which `009f9a80` (thiscall on the
/// type, `media_type`) gives 0, when `media_type` is not null, is
/// `VFW_E_NO_ACCEPTABLE_TYPES`, otherwise the connection is attempted
/// (`009fbf00`); the type is deleted (`009f9cb0`); a result of 0 returns 0.
/// The first failure that is not one of `is_generic_failure`'s is kept and
/// returned at the end, else `VFW_E_NO_ACCEPTABLE_TYPES`.
pub fn fn_009fbfc0(
    e: &mut Engine,
    this: Ptr,
    receiver: Ptr,
    media_type: Ptr,
    enumerator: Ptr,
) -> u32 {
    let hr = e.vcall(enumerator.addr(), 0x14, &args![]).u32();
    if failed(hr) {
        return hr;
    }
    e.with_stack(8, |e, locals| {
        // locals + 0: the fetched count; locals + 4: the stack slot of
        // `enumerator`, which receives each fetched type.
        let fetched_count = locals;
        let type_slot = offset(locals, 0x4);
        e.mem.set_u32(type_slot.addr(), 0);
        e.mem.set_u32(fetched_count.addr(), 0);
        let mut kept: u32 = 0;
        let mut hr = e
            .vcall(
                enumerator.addr(),
                0xc,
                &args![1u32, type_slot, fetched_count],
            )
            .u32();
        while hr == 0 {
            let fetched = Ptr::new(e.mem.u32(type_slot.addr()));
            let result;
            if !media_type.is_null()
                && e.call(TYPE_TEST_009F9A80, &args![fetched, media_type])
                    .u32()
                    == 0
            {
                result = VFW_E_NO_ACCEPTABLE_TYPES;
            } else {
                result = fn_009fbf00(e, this, receiver, fetched);
                if failed(result) && !failed(kept) && !is_generic_failure(result) {
                    kept = result;
                }
            }
            fn_009f9cb0(e, fetched);
            if result == 0 {
                return 0;
            }
            hr = e
                .vcall(
                    enumerator.addr(),
                    0xc,
                    &args![1u32, type_slot, fetched_count],
                )
                .u32();
        }
        if kept == 0 {
            kept = VFW_E_NO_ACCEPTABLE_TYPES;
        }
        kept
    })
}

// Translated from 009fc0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agrees a media type (thiscall, `RET 8`): a non-null `media_type` for
/// which `009f9a50` (thiscall, no stack words) gives 0 goes straight to
/// `009fbf00`. Otherwise
/// two passes: the pass whose number equals the byte at `this + 0x26` asks
/// the receiver for its enumerator (vtable +0x30, stdcall), the other asks
/// the interface at `this + 0xc`; each enumerator is tried (`009fbfc0`) and
/// released (vtable +8); success returns 0. The last failure that is not one
/// of `is_generic_failure`'s is returned, else `VFW_E_NO_ACCEPTABLE_TYPES`.
pub fn fn_009fc0b0(e: &mut Engine, this: Ptr, receiver: Ptr, media_type: Ptr) -> u32 {
    if !media_type.is_null() && e.call(TYPE_TEST_009F9A50, &args![media_type]).u32() == 0 {
        return fn_009fbf00(e, this, receiver, media_type);
    }
    e.with_stack(4, |e, enumerator_slot| {
        let mut result = VFW_E_NO_ACCEPTABLE_TYPES;
        let first_pass = u32::from(e.mem.u8(this.addr().wrapping_add(0x26)));
        let mut pass: u32 = 0;
        while pass < 2 {
            let hr = if pass == first_pass {
                e.vcall(receiver.addr(), 0x30, &args![enumerator_slot])
                    .u32()
            } else {
                let interface = this.addr().wrapping_add(0xc);
                e.vcall(interface, 0x30, &args![enumerator_slot]).u32()
            };
            if !failed(hr) {
                let enumerator = Ptr::new(e.mem.u32(enumerator_slot.addr()));
                let tried = fn_009fbfc0(e, this, receiver, media_type, enumerator);
                let enumerator = e.mem.u32(enumerator_slot.addr());
                e.vcall(enumerator, 0x8, &args![]);
                if !failed(tried) {
                    return 0;
                }
                if !is_generic_failure(tried) {
                    result = tried;
                }
            }
            pass += 1;
        }
        result
    })
}

// Translated from 009fc300 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fc300 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Disconnect guard (stdcall, `RET 4`; `this` is an interface at `+0xc` of
/// its pin): under the critical section at `this + 0x14`,
/// `VFW_E_NOT_STOPPED` when the word at `+0x14` of the object at
/// `this + 0x1c` is non-zero; otherwise `009fa380` on the pin.
pub fn fn_009fc300(e: &mut Engine, this: Ptr) -> u32 {
    let section = field(e, this, 0x14);
    enter(e, section);
    let owner = field(e, this, 0x1c);
    if field(e, Ptr::new(owner), 0x14) != 0 {
        leave(e, section);
        return VFW_E_NOT_STOPPED;
    }
    let hr = fn_009fa380(e, back(this, 0xc));
    leave(e, section);
    hr
}

// Translated from 009fc450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: replaces its first stack word with the wide name at `this + 8`
/// and jumps to `009fd0a0` (stdcall, `RET 8`: the name and `out`).
pub fn fn_009fc450(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let name = field(e, this, 0x8);
    e.call(WIDE_STRING_DUPLICATE, &args![name, out]).u32()
}

// Translated from 009fc5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Filter constructor shape (thiscall, `RET 0x14`): the base `009fcee0`
/// (`name`, `outer`); clears +0x14, +0x18, +0x20, +0x24; copies the 16 bytes
/// at `class_id` to +0x28; stores `lock` at +0x38; clears +0x3c..+0x44 and
/// sets +0x48 to 1. The fifth word is not read. Returns `this`.
pub fn fn_009fc5a0(
    e: &mut Engine,
    this: Ptr,
    name: Ptr,
    outer: Ptr,
    lock: Ptr,
    class_id: Ptr,
    _unused_5: u32,
) -> Ptr {
    fn_009fcee0(e, this, name, outer);
    set_field(e, this, 0x14, 0);
    set_field(e, this, 0x18, 0);
    set_field(e, this, 0x20, 0);
    set_field(e, this, 0x24, 0);
    for i in 0..4u32 {
        let w = field(e, class_id, 4 * i);
        set_field(e, this, 0x28 + 4 * i, w);
    }
    set_field(e, this, 0x38, lock.addr());
    set_field(e, this, 0x3c, 0);
    set_field(e, this, 0x40, 0);
    set_field(e, this, 0x44, 0);
    set_field(e, this, 0x48, 1);
    this
}

// Translated from 009fc600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fc600 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CEnumPins` constructor shape (RTTI of the vtable it installs; thiscall,
/// `RET 8`): position and total 0, `filter` at +0xc, +0x14 = 1, the list at
/// +0x18 initialised (`009fd120`), a reference added to the filter's
/// interface at +0xc (vtable +4). Without a `source` enumerator the version
/// (+0x10) and total (+8) come from the filter (vtable +0x14 and +0x18);
/// with one its position, total and version are copied and its list
/// appended (`009fd2d0`). Returns `this`.
pub fn fn_009fc600(e: &mut Engine, this: Ptr, filter: Ptr, source: Ptr) -> Ptr {
    set_field(e, this, 0x0, ENUM_PINS_VTABLE);
    set_field(e, this, 0x4, 0);
    set_field(e, this, 0x8, 0);
    set_field(e, this, 0xc, filter.addr());
    set_field(e, this, 0x14, 1);
    fn_009fd120(e, offset(this, 0x18), 0);
    let held = field(e, this, 0xc);
    e.vcall(held.wrapping_add(0xc), 0x4, &args![]);
    if source.is_null() {
        let held = field(e, this, 0xc);
        let version = e.vcall(held, 0x14, &args![]).u32();
        set_field(e, this, 0x10, version);
        let held = field(e, this, 0xc);
        let total = e.vcall(held, 0x18, &args![]).u32();
        set_field(e, this, 0x8, total);
    } else {
        let position = field(e, source, 0x4);
        set_field(e, this, 0x4, position);
        let total = field(e, source, 0x8);
        set_field(e, this, 0x8, total);
        let version = field(e, source, 0x10);
        set_field(e, this, 0x10, version);
        e.call(
            LIST_APPEND_009FD2D0,
            &args![offset(this, 0x18), offset(source, 0x18)],
        );
    }
    this
}

// Translated from 009fc780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CEnumMediaTypes` constructor shape (RTTI of the vtable it installs;
/// thiscall, `RET 8`): `pin` at +8, position 0, +0x10 = 1, a reference added
/// to the pin's interface at +0xc (vtable +4); the version (+0xc) comes from
/// the pin (vtable +0x10) or, with a `source` enumerator, its position and
/// version are copied. Returns `this`.
pub fn fn_009fc780(e: &mut Engine, this: Ptr, pin: Ptr, source: Ptr) -> Ptr {
    set_field(e, this, 0x8, pin.addr());
    set_field(e, this, 0x0, ENUM_MEDIA_TYPES_VTABLE);
    set_field(e, this, 0x4, 0);
    set_field(e, this, 0x10, 1);
    e.vcall(pin.addr().wrapping_add(0xc), 0x4, &args![]);
    if source.is_null() {
        let held = field(e, this, 0x8);
        let version = e.vcall(held, 0x10, &args![]).u32();
        set_field(e, this, 0xc, version);
    } else {
        let position = field(e, source, 0x4);
        set_field(e, this, 0x4, position);
        let version = field(e, source, 0xc);
        set_field(e, this, 0xc, version);
    }
    this
}

// Translated from 009fc890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fc890 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Pin constructor shape (thiscall, `RET 0x18`): the base `009fcee0`
/// (`object_name`, no outer object); `direction` at +0x1c, +0x14/+0x18
/// cleared, `lock` at +0x20, the bytes +0x24..+0x26 cleared, `filter` at
/// +0x28, +0x2c = 0, +0x30 = 1; the media-type block at +0x34
/// (`009f9c70`); +0x80/+0x84 = 0, +0x88 = -1, +0x8c = 0x7fffffff, the double
/// 1.0 at +0x90 (`FLD1`/`FSTP`); a copy of the wide `name` at +0x14 when not
/// null. The fourth word is not read. Returns `this`.
pub fn fn_009fc890(
    e: &mut Engine,
    this: Ptr,
    object_name: Ptr,
    filter: Ptr,
    lock: Ptr,
    _unused_4: u32,
    name: Ptr,
    direction: u32,
) -> Ptr {
    fn_009fcee0(e, this, object_name, Ptr::NULL);
    set_field(e, this, 0x1c, direction);
    set_field(e, this, 0x14, 0);
    set_field(e, this, 0x18, 0);
    set_field(e, this, 0x20, lock.addr());
    e.mem.set_u8(this.addr().wrapping_add(0x24), 0);
    e.mem.set_u8(this.addr().wrapping_add(0x25), 0);
    e.mem.set_u8(this.addr().wrapping_add(0x26), 0);
    set_field(e, this, 0x28, filter.addr());
    set_field(e, this, 0x2c, 0);
    set_field(e, this, 0x30, 1);
    fn_009f9c70(e, offset(this, 0x34));
    set_field(e, this, 0x80, 0);
    set_field(e, this, 0x84, 0);
    set_field(e, this, 0x88, 0xffff_ffff);
    set_field(e, this, 0x8c, 0x7fff_ffff);
    e.mem.set_f64(this.addr().wrapping_add(0x90), 1.0);
    if !name.is_null() {
        let block = duplicate_name(e, name);
        set_field(e, this, 0x14, block);
    }
    this
}

// Translated from 009fc990 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fc990 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Connect shape (stdcall, `RET 0xc`; `this` is the interface at `+0xc` of
/// its pin): `E_POINTER` for a null `receiver`; under the critical section
/// at `this + 0x14`: `VFW_E_ALREADY_CONNECTED` when `this + 0xc` is set;
/// `VFW_E_NOT_STOPPED` when the word at `+0x14` of the object at
/// `this + 0x1c` is set and the byte at `this + 0x19` is clear; otherwise
/// agrees a type (`009fc0b0` on the pin) and on failure calls the pin's
/// vtable +0x2c (thiscall) and returns that failure; 0 on success.
pub fn fn_009fc990(e: &mut Engine, this: Ptr, receiver: Ptr, media_type: Ptr) -> u32 {
    if receiver.is_null() {
        return E_POINTER;
    }
    let section = field(e, this, 0x14);
    enter(e, section);
    if field(e, this, 0xc) != 0 {
        leave(e, section);
        return VFW_E_ALREADY_CONNECTED;
    }
    let owner = field(e, this, 0x1c);
    if field(e, Ptr::new(owner), 0x14) != 0 && e.mem.u8(this.addr().wrapping_add(0x19)) == 0 {
        leave(e, section);
        return VFW_E_NOT_STOPPED;
    }
    let pin = back(this, 0xc);
    let hr = fn_009fc0b0(e, pin, receiver, media_type);
    if failed(hr) {
        e.vcall(pin.addr(), 0x2c, &args![]);
        leave(e, section);
        return hr;
    }
    leave(e, section);
    0
}

/// `0 -> E_OUTOFMEMORY`, anything else -> 0 (`NEG`/`SBB`/`AND`/`ADD` with
/// 0x7ff8fff2 and 0x8007000e, wrapping).
fn allocated_or_out_of_memory(block: u32) -> u32 {
    let mask = if block != 0 { 0xffff_ffffu32 } else { 0 };
    (mask & 0x7ff8_fff2).wrapping_add(E_OUTOFMEMORY)
}

// Translated from 009fcab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fcab0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Makes a media-type enumerator (stdcall, `RET 8`): `E_POINTER` for a null
/// `out`; a 0x14-byte block (`operator new`) constructed by `009fc780` for
/// the pin at `this - 0xc` (no source) is stored in `out` (0 when the
/// allocation failed); returns 0, or `E_OUTOFMEMORY` for 0.
pub fn fn_009fcab0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let made = if block != 0 {
        fn_009fc780(e, Ptr::new(block), back(this, 0xc), Ptr::NULL).addr()
    } else {
        0
    };
    e.mem.set_u32(out.addr(), made);
    allocated_or_out_of_memory(made)
}

// Translated from 009fcdc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fcdc0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Makes a pin enumerator (stdcall, `RET 8`): `E_POINTER` for a null `out`;
/// a 0x30-byte block (`operator new`) constructed by `009fc600` for the
/// filter at `this - 0xc` (no source) is stored in `out` (0 when the
/// allocation failed); returns 0, or `E_OUTOFMEMORY` for 0.
pub fn fn_009fcdc0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let block = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
    let made = if block != 0 {
        fn_009fc600(e, Ptr::new(block), back(this, 0xc), Ptr::NULL).addr()
    } else {
        0
    };
    e.mem.set_u32(out.addr(), made);
    allocated_or_out_of_memory(made)
}

// Translated from 009fce60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counted-object constructor shape (thiscall, `RET 4`; the stack word is
/// not read): `InterlockedIncrement` of the count at `011f3298`. Returns
/// `this`.
pub fn fn_009fce60(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    e.call(INTERLOCKED_INCREMENT, &args![OBJECT_COUNT]);
    this
}

// Translated from 009fce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counted-object destructor shape (no words: callers set `ECX`, which it
/// never reads): `InterlockedDecrement` of the count at `011f3298`; at zero
/// a module handle held at `011f3294` is freed (`FreeLibrary`) and cleared.
pub fn fn_009fce80(e: &mut Engine) {
    let count = e.call(INTERLOCKED_DECREMENT, &args![OBJECT_COUNT]).u32();
    if count == 0 {
        let module = e.global::<u32>(OBJECT_COUNT_MODULE);
        if module != 0 {
            e.call(FREE_LIBRARY, &args![module]);
            e.set_global(OBJECT_COUNT_MODULE, 0u32);
        }
    }
}

// Translated from 009fceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands out an interface (stdcall, `RET 8`): `E_POINTER` for a null `out`;
/// otherwise stores `interface`, adds a reference (vtable +4) and returns 0.
pub fn fn_009fceb0(e: &mut Engine, interface: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    e.mem.set_u32(out.addr(), interface.addr());
    e.vcall(interface.addr(), 0x4, &args![]);
    0
}

// Translated from 009fcee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reference-counted base constructor shape (thiscall, `RET 8`): the
/// counted-object base on `this + 4` (`009fce60`, passing `name`), then
/// `outer` (or `this` when null) at +4 and the count at +8 cleared.
/// Returns `this`.
pub fn fn_009fcee0(e: &mut Engine, this: Ptr, name: Ptr, outer: Ptr) -> Ptr {
    let base = offset(this, 0x4);
    fn_009fce60(e, base, name.addr());
    let outer = if outer.is_null() { this } else { outer };
    set_field(e, this, 0x4, outer.addr());
    set_field(e, this, 0x8, 0);
    this
}

// Translated from 009fcf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base interface lookup (stdcall, `RET 0xc`): `E_POINTER` for a null
/// `out`; for the id at `01093f14` hands out `this` (`009fceb0`, its result
/// ignored) and returns 0; otherwise clears `out` and returns
/// `E_NOINTERFACE`.
pub fn fn_009fcf10(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    if fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093F14)) != 0 {
        fn_009fceb0(e, this, out);
        return 0;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

// Translated from 009fcf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reference add (stdcall, `RET 4`): `InterlockedIncrement` of the count at
/// `this + 8`, then returns the count read back, at least 1.
pub fn fn_009fcf60(e: &mut Engine, this: Ptr) -> u32 {
    let count = offset(this, 0x8);
    e.call(INTERLOCKED_INCREMENT, &args![count]);
    let value = e.mem.u32(count.addr());
    if value > 1 {
        value
    } else {
        1
    }
}

// Translated from 009fcf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reference release (stdcall, `RET 4`): `InterlockedDecrement` of the
/// count at `this + 8`; at zero the count is set back to 1 (`ADD 1`), the
/// object's vtable +0xc is called with 1 (thiscall) and 0 returned;
/// otherwise the count read back, at least 1.
pub fn fn_009fcf80(e: &mut Engine, this: Ptr) -> u32 {
    let count = offset(this, 0x8);
    if e.call(INTERLOCKED_DECREMENT, &args![count]).u32() == 0 {
        let value = e.mem.u32(count.addr()).wrapping_add(1);
        e.mem.set_u32(count.addr(), value);
        e.vcall(this.addr(), 0xc, &args![1u32]);
        return 0;
    }
    let value = e.mem.u32(count.addr());
    if value > 1 {
        value
    } else {
        1
    }
}

// Translated from 009fcfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Event-holder constructor shape (thiscall, `RET 4`): stores
/// `CreateEventA(0, manual_reset, 0, 0)` at `this`. Returns `this`.
pub fn fn_009fcfc0(e: &mut Engine, this: Ptr, manual_reset: u32) -> Ptr {
    let event = e
        .call(CREATE_EVENT_A, &args![0u32, manual_reset, 0u32, 0u32])
        .u32();
    e.mem.set_u32(this.addr(), event);
    this
}

// Translated from 009fcfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Event-holder destructor shape: `CloseHandle` of the handle at `this`
/// when it is set.
pub fn fn_009fcfe0(e: &mut Engine, this: Ptr) {
    let handle = e.mem.u32(this.addr());
    if handle != 0 {
        e.call(CLOSE_HANDLE, &args![handle]);
    }
}

// Translated from 009fcff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bounded wide-string copy (stdcall, `RET 0xc`): nothing for a `count` of
/// 0; otherwise copies characters up to and including the terminator, and
/// writes a terminator in place of the `count`-th character when the source
/// is longer. Returns `target`.
pub fn fn_009fcff0(e: &mut Engine, target: Ptr, source: Ptr, count: u32) -> Ptr {
    if count == 0 {
        return target;
    }
    let mut to = target.addr();
    let mut from = source.addr();
    let mut left = count;
    loop {
        left -= 1;
        if left == 0 {
            e.mem.set_u16(to, 0);
            return target;
        }
        let ch = e.mem.u16(from);
        e.mem.set_u16(to, ch);
        to = to.wrapping_add(2);
        from = from.wrapping_add(2);
        if ch == 0 {
            return target;
        }
    }
}

// Translated from 009fd080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wide-string length (stdcall, `RET 4`): the number of 16-bit characters
/// before the terminator.
pub fn fn_009fd080(e: &mut Engine, text: Ptr) -> u32 {
    let mut length: u32 = 0;
    while e.mem.u16(text.addr().wrapping_add(length.wrapping_mul(2))) != 0 {
        length = length.wrapping_add(1);
    }
    length
}

// Translated from 009fd0a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 009fd0a0 to the `RET 8` at 009fd0ea, inside the body Ghidra
/// gives `009fc450`): the wide-string duplicate the thunk [`fn_009fc450`] jumps to, `stdcall`
/// with two words (the text and `out`). A null `out` answers `E_POINTER` (0x80004003).
/// Otherwise the length ([`fn_009fd080`]) gives `2 * length + 2` bytes, `CoTaskMemAlloc` makes
/// the block and `*out` takes it (null included); a null block answers `E_OUTOFMEMORY`
/// (0x8007000e), else the text with its terminator is copied in (`memcpy`) and the answer is 0.
pub fn fn_009fd0a0(e: &mut Engine, text: Ptr, out: Ptr) -> u32 {
    if out.addr() == 0 {
        return 0x8000_4003;
    }
    let length = e.call(0x009f_d080, &args![text]).u32();
    let bytes = length.wrapping_mul(2).wrapping_add(2);
    let block = e.call(CO_TASK_MEM_ALLOC, &args![bytes]).u32();
    e.mem.set_u32(out.addr(), block);
    if block == 0 {
        return 0x8007_000e;
    }
    e.call(MEMCPY, &args![block, text, bytes]);
    0
}

// Translated from 009fd120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Generic list constructor shape (thiscall, `RET 4`; the stack word is not
/// read): head, tail and count cleared, +0xc = 10 (the spare-node limit),
/// spare count and spare list cleared. Returns `this`.
pub fn fn_009fd120(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    set_field(e, this, 0x0, 0);
    set_field(e, this, 0x4, 0);
    set_field(e, this, 0x8, 0);
    set_field(e, this, 0xc, 0xa);
    set_field(e, this, 0x10, 0);
    set_field(e, this, 0x14, 0);
    this
}

// Translated from 009fd140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Generic list clear: deletes every node from the head (next at +4) and
/// clears count, tail and head.
pub fn fn_009fd140(e: &mut Engine, this: Ptr) {
    let mut node = field(e, this, 0x0);
    while node != 0 {
        let current = node;
        node = e.mem.u32(node.wrapping_add(0x4));
        e.call(OPERATOR_DELETE, &args![current]);
    }
    set_field(e, this, 0x8, 0);
    set_field(e, this, 0x4, 0);
    set_field(e, this, 0x0, 0);
}

// Translated from 009fd1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Generic list find (thiscall, `RET 4`): walks the positions from the head
/// (`009fd0f0`) and returns the first whose object (`009fd1a0`) is
/// `object`, or 0. The step after a null position would restart at the
/// head (`[this]`); the walk stops before one.
pub fn fn_009fd1c0(e: &mut Engine, this: Ptr, object: Ptr) -> Ptr {
    let mut position = e.call(HEAD_POSITION, &args![this]).u32();
    while position != 0 {
        let held = e.call(LIST_GET_AT, &args![position]).u32();
        if held == object.addr() {
            return Ptr::new(position);
        }
        position = if position == 0 {
            field(e, this, 0x0)
        } else {
            e.mem.u32(position.wrapping_add(0x4))
        };
    }
    Ptr::NULL
}

// Translated from 009fd200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Generic list remove (thiscall, `RET 4`): 0 for a null position;
/// otherwise unlinks the node (previous at +0, next at +4, fixing head or
/// tail), keeps it on the spare list (+0x14, count +0x10) while that is
/// below the limit at +0xc, or deletes it; the count at +8 drops by one.
/// Returns the node's object (+8).
pub fn fn_009fd200(e: &mut Engine, this: Ptr, position: Ptr) -> u32 {
    if position.is_null() {
        return 0;
    }
    let node = position;
    let previous = field(e, node, 0x0);
    if previous == 0 {
        let next = field(e, node, 0x4);
        set_field(e, this, 0x0, next);
    } else {
        let next = field(e, node, 0x4);
        e.mem.set_u32(previous.wrapping_add(0x4), next);
    }
    let next = field(e, node, 0x4);
    if next == 0 {
        let previous = field(e, node, 0x0);
        set_field(e, this, 0x4, previous);
    } else {
        let previous = field(e, node, 0x0);
        e.mem.set_u32(next, previous);
    }
    let object = field(e, node, 0x8);
    if (field(e, this, 0x10) as i32) < (field(e, this, 0xc) as i32) {
        let spare = field(e, this, 0x14);
        set_field(e, node, 0x4, spare);
        let spares = field(e, this, 0x10).wrapping_add(1);
        set_field(e, this, 0x10, spares);
        set_field(e, this, 0x14, node.addr());
        let count = field(e, this, 0x8).wrapping_sub(1);
        set_field(e, this, 0x8, count);
        return object;
    }
    e.call(OPERATOR_DELETE, &args![node]);
    let count = field(e, this, 0x8).wrapping_sub(1);
    set_field(e, this, 0x8, count);
    object
}

// Translated from 009fd270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Generic list add-tail (thiscall, `RET 4`): takes a spare node (+0x14,
/// count +0x10) or a new 0xc-byte one (`operator new`; 0 returned when that
/// fails); the node holds `object` (+8), no next, the old tail as previous;
/// it becomes the tail (and the head of an empty list) and the count at +8
/// grows. Returns the node.
pub fn fn_009fd270(e: &mut Engine, this: Ptr, object: Ptr) -> Ptr {
    let mut node = field(e, this, 0x14);
    if node != 0 {
        let next = e.mem.u32(node.wrapping_add(0x4));
        let spares = field(e, this, 0x10).wrapping_sub(1);
        set_field(e, this, 0x10, spares);
        set_field(e, this, 0x14, next);
    } else {
        node = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        if node == 0 {
            return Ptr::NULL;
        }
    }
    e.mem.set_u32(node.wrapping_add(0x8), object.addr());
    e.mem.set_u32(node.wrapping_add(0x4), 0);
    let tail = field(e, this, 0x4);
    e.mem.set_u32(node, tail);
    let tail = field(e, this, 0x4);
    if tail == 0 {
        let count = field(e, this, 0x8).wrapping_add(1);
        set_field(e, this, 0x8, count);
        set_field(e, this, 0x0, node);
        set_field(e, this, 0x4, node);
        return Ptr::new(node);
    }
    e.mem.set_u32(tail.wrapping_add(0x4), node);
    let count = field(e, this, 0x8).wrapping_add(1);
    set_field(e, this, 0x8, count);
    set_field(e, this, 0x4, node);
    Ptr::new(node)
}

// Translated from 009fd320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fd320 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Generic list destructor shape: clears the list (`009fd140`), then
/// deletes every spare node (from +0x14, next at +4).
pub fn fn_009fd320(e: &mut Engine, this: Ptr) {
    fn_009fd140(e, this);
    let mut node = field(e, this, 0x14);
    while node != 0 {
        let current = node;
        node = e.mem.u32(node.wrapping_add(0x4));
        e.call(OPERATOR_DELETE, &args![current]);
    }
}

// Translated from 009fd390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Generic list remove-head: `009fd200` on the head position; returns its
/// object.
pub fn fn_009fd390(e: &mut Engine, this: Ptr) -> u32 {
    let head = field(e, this, 0x0);
    fn_009fd200(e, this, Ptr::new(head))
}

// Translated from 009fd3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Alignment test (thiscall, `RET 4`): 1 when `value` is a multiple of the
/// alignment (vtable +0x10, thiscall, of the object at `this + 0x18`; the
/// test is `value & (alignment - 1)`), 0 otherwise.
pub fn fn_009fd3a0(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let allocator = field(e, this, 0x18);
    let alignment = e.vcall(allocator, 0x10, &args![]).u32();
    u32::from(alignment.wrapping_sub(1) & value == 0)
}

// Translated from 009fd3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CAsyncOutputPin` destructor shape (RTTI of the vtables it installs at
/// +0, +8, +0x14, +0x18), then jumps to the pin destructor shape `009fa1f0`
/// with `ECX = this + 8`.
pub fn fn_009fd3c0(e: &mut Engine, this: Ptr) {
    set_field(e, this, 0x0, ASYNC_OUTPUT_PIN_VTABLE);
    set_field(e, this, 0x8, ASYNC_OUTPUT_PIN_VTABLE_8);
    set_field(e, this, 0x14, ASYNC_OUTPUT_PIN_VTABLE_14);
    set_field(e, this, 0x18, ASYNC_OUTPUT_PIN_VTABLE_18);
    fn_009fa1f0(e, offset(this, 0x8));
}

// Translated from 009fd3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Delegating thunk: replaces its first stack word with the object at
/// `this + 0xc` and jumps to that object's vtable +0 (the interface-query
/// slot of the COM layout: the object, an id and an out pointer).
pub fn fn_009fd3f0(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    let target = field(e, this, 0xc);
    e.vcall(target, 0x0, &args![id, out]).u32()
}

// Translated from 009fd410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Delegating thunk: replaces its stack word with the object at
/// `this + 0xc` and jumps to its vtable +4 (the reference-add slot of the
/// COM layout; one word).
pub fn fn_009fd410(e: &mut Engine, this: Ptr) -> u32 {
    let target = field(e, this, 0xc);
    e.vcall(target, 0x4, &args![]).u32()
}

// Translated from 009fd430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Delegating thunk: replaces its stack word with the object at
/// `this + 0xc` and jumps to its vtable +8 (the release slot of the COM
/// layout; one word).
pub fn fn_009fd430(e: &mut Engine, this: Ptr) -> u32 {
    let target = field(e, this, 0xc);
    e.vcall(target, 0x8, &args![]).u32()
}

// Translated from 009fd450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `this + 0xa0`, then jumps to `009fa350` (thiscall,
/// `pin`).
pub fn fn_009fd450(e: &mut Engine, this: Ptr, pin: Ptr) -> u32 {
    set_field(e, this, 0xa0, 0);
    fn_009fa350(e, this, pin)
}

// Translated from 009fd460 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the word at `this + 0xa0` is set, jumps to `00a38de0` (thiscall,
/// one word); otherwise returns `0x80040266`.
pub fn fn_009fd460(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    if field(e, this, 0xa0) != 0 {
        return e.call(RETURN_ZERO_00A38DE0, &args![this, value]).u32();
    }
    STATUS_80040266
}

// Translated from 009fd480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `this + 0xa0`, then jumps to `00e7cd40` (thiscall, no
/// stack words).
pub fn fn_009fd480(e: &mut Engine, this: Ptr) -> u32 {
    set_field(e, this, 0xa0, 0);
    e.call(RETURN_ZERO_00E7CD40, &args![this]).u32()
}

// Translated from 009fd490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Interface lookup (stdcall, `RET 0xc`): `E_POINTER` for a null `out`;
/// for the id at `01093b4c` sets the word at `this + 0xa0` to 1 and hands
/// out `this - 8` (`009fceb0`); any other id goes to `009fa260`.
pub fn fn_009fd490(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    if fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093B4C)) != 0 {
        set_field(e, this, 0xa0, 1);
        return fn_009fceb0(e, back(this, 0x8), out);
    }
    fn_009fa260(e, this, id, out)
}

// Translated from 009fd4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Media type at a position (thiscall, `RET 8`): `E_INVALIDARG` for a
/// negative position, `VFW_S_NO_MORE_ITEMS` for a positive one; for 0,
/// `E_POINTER` for a null `media_type`, `E_UNEXPECTED` when nothing is held
/// at `this + 0x98`, else the block at `+0x1a0` of that object is assigned
/// to `media_type` (`009f9d30`) and 0 returned.
pub fn fn_009fd4e0(e: &mut Engine, this: Ptr, position: i32, media_type: Ptr) -> u32 {
    if position < 0 {
        return E_INVALIDARG;
    }
    if position > 0 {
        return VFW_S_NO_MORE_ITEMS;
    }
    if media_type.is_null() {
        return E_POINTER;
    }
    let owner = field(e, this, 0x98);
    if owner == 0 {
        return E_UNEXPECTED;
    }
    fn_009f9d30(e, media_type, Ptr::new(owner.wrapping_add(0x1a0)));
    0
}

// Translated from 009fd530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Media type check (thiscall, `RET 4`), under the critical section at
/// `this + 0x20`: 0 when the first 16 bytes of `media_type` match those at
/// `+0x1a0` of the object at `this + 0x98` and that object's id at `+0x1b0`
/// is the id at `01093f04` or equals the 16 bytes at `media_type + 0x10`;
/// 1 otherwise.
pub fn fn_009fd530(e: &mut Engine, this: Ptr, media_type: Ptr) -> u32 {
    let section = field(e, this, 0x20);
    enter(e, section);
    let owner = Ptr::new(field(e, this, 0x98));
    let accepted = fn_009f9980(e, offset(owner, 0x1a0), media_type) != 0 && {
        let subtype = offset(owner, 0x1b0);
        fn_009f9980(e, subtype, Ptr::new(INTERFACE_ID_01093F04)) != 0
            || fn_009f9980(e, subtype, offset(media_type, 0x10)) != 0
    };
    leave(e, section);
    if accepted {
        0
    } else {
        1
    }
}

// Translated from 009fd6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocator negotiation (stdcall, `RET 0x10`; `this` is the first word,
/// `ECX` is not read): `E_POINTER` for a null `allocator`, `request` or
/// `out`. A non-zero alignment at `request + 8` that is not a multiple of
/// the reader's alignment (`this + 0xa4`: its object at +0x18, vtable
/// +0x10) is replaced by that alignment (`009fe1a0`). The offered allocator
/// gets the request (vtable +0xc, stdcall, `request` and a local properties
/// block); when that succeeds and the resulting alignment (local +8) passes
/// `009fd3a0`, it is handed out with a reference added (vtable +4). Else a
/// new allocator (`009fd5a0`, written over the stack slot of `request`) is
/// tried the same way and handed out as is; when that fails too it is
/// released (vtable +8) and the failure, or `VFW_E_BADALIGN` for a success
/// with a bad alignment, is returned. A failed `009fd5a0` is returned as is.
pub fn fn_009fd6d0(e: &mut Engine, this: Ptr, allocator: Ptr, request: Ptr, out: Ptr) -> u32 {
    if allocator.is_null() {
        return E_POINTER;
    }
    if request.is_null() || out.is_null() {
        return E_POINTER;
    }
    let alignment = field(e, request, 0x8);
    let fix = if alignment != 0 {
        let reader = field(e, this, 0xa4);
        let held = field(e, Ptr::new(reader), 0x18);
        let required = e.vcall(held, 0x10, &args![]).u32();
        alignment & required.wrapping_sub(1) != 0
    } else {
        true
    };
    if fix {
        let reader = Ptr::new(field(e, this, 0xa4));
        fn_009fe1a0(e, reader, offset(request, 0x8));
    }
    e.with_stack(0x14, |e, locals| {
        // locals + 0..0x10: the actual properties; locals + 0x10: the stack
        // slot of `request`, reused for the new allocator.
        let actual = locals;
        let new_slot = offset(locals, 0x10);
        e.mem.set_u32(new_slot.addr(), request.addr());
        let hr = e
            .vcall(allocator.addr(), 0xc, &args![request, actual])
            .u32();
        if !failed(hr) {
            let reader = Ptr::new(field(e, this, 0xa4));
            let actual_alignment = field(e, actual, 0x8);
            if fn_009fd3a0(e, reader, actual_alignment) != 0 {
                e.vcall(allocator.addr(), 0x4, &args![]);
                e.mem.set_u32(out.addr(), allocator.addr());
                return 0;
            }
        }
        let hr = e.call(ALLOCATOR_MAKE_009FD5A0, &args![new_slot]).u32();
        if failed(hr) {
            return hr;
        }
        let created = e.mem.u32(new_slot.addr());
        let mut hr = e.vcall(created, 0xc, &args![request, actual]).u32();
        if !failed(hr) {
            let reader = Ptr::new(field(e, this, 0xa4));
            let actual_alignment = field(e, actual, 0x8);
            if fn_009fd3a0(e, reader, actual_alignment) != 0 {
                let created = e.mem.u32(new_slot.addr());
                e.mem.set_u32(out.addr(), created);
                return 0;
            }
        }
        let created = e.mem.u32(new_slot.addr());
        e.vcall(created, 0x8, &args![]);
        if !failed(hr) {
            hr = VFW_E_BADALIGN;
        }
        hr
    })
}

/// The 64-bit value at `at`.
fn quad(e: &Engine, at: u32) -> u64 {
    e.mem.u64(at)
}

// Translated from 009fd970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sample request (stdcall, `RET 8`): `E_POINTER` for a null `sample`. Its
/// times (vtable +0x14, start and end, 100 ns units) must be read; start
/// and length are taken in seconds (`__alldiv` by 10,000,000, the length
/// truncated to its low word). When start + length passes the reader's
/// total (`009fe090` on the reader at `this + 0xa4`), the total is rounded
/// up to the reader's alignment (`009fe1a0`, written over the stack slot of
/// `sample`); if the end still passes it, the length becomes total - start
/// and the end time total * 10,000,000 (`__allmul`), set on the sample
/// (vtable +0x18). Then the sample's buffer (vtable +0xc) must be read; the
/// read itself is `009fdfe0` (start, length, buffer, a local actual length,
/// `sample`), and the actual length goes to the sample (vtable +0x30).
/// Returns `009fdfe0`'s result, or the first failure.
pub fn fn_009fd970(e: &mut Engine, this: Ptr, sample: Ptr) -> u32 {
    if sample.is_null() {
        return E_POINTER;
    }
    // The frame mirrors the exe's: +4 buffer, +8 actual length, +0xc total
    // (8 bytes), +0x14 end time, +0x24 start time, +0x34 the word `009fe090`
    // gets, +0x44 the stack slot of `sample`.
    e.with_stack(0x48, |e, frame| {
        let f = frame.addr();
        let buffer = f + 0x4;
        let length = f + 0x8;
        let total = f + 0xc;
        let end = f + 0x14;
        let start = f + 0x24;
        let other = f + 0x34;
        let sample_slot = f + 0x44;
        e.mem.set_u32(sample_slot, sample.addr());
        let s = sample.addr();
        let hr = e.vcall(s, 0x14, &args![start, end]).u32();
        if failed(hr) {
            return hr;
        }
        let start_time = quad(e, start);
        let first = e.call(ALLDIV, &args![start_time, TIME_UNITS, 0u32]).u64();
        let span_time = quad(e, end).wrapping_sub(start_time);
        let span = e.call(ALLDIV, &args![span_time, TIME_UNITS, 0u32]).u64();
        let mut span_low = span as u32;
        let reader = Ptr::new(field(e, this, 0xa4));
        fn_009fe090(e, reader, Ptr::new(total), Ptr::new(other));
        // CDQ of the low word, ADD/ADC with the first second.
        let finish = i64::from(span_low as i32).wrapping_add(first as i64);
        if finish > quad(e, total) as i64 {
            let reader = Ptr::new(field(e, this, 0xa4));
            fn_009fe1a0(e, reader, Ptr::new(sample_slot));
            let alignment = e.mem.u32(sample_slot);
            let rounded = (quad(e, total) as i64)
                .wrapping_add(i64::from(alignment as i32))
                .wrapping_sub(1)
                & i64::from(!alignment.wrapping_sub(1) as i32);
            e.mem.set_u64(total, rounded as u64);
            if finish > rounded {
                let end_time = e
                    .call(ALLMUL, &args![rounded as u64, TIME_UNITS, 0u32])
                    .u64();
                span_low = (rounded as u32).wrapping_sub(first as u32);
                e.mem.set_u64(end, end_time);
                e.vcall(s, 0x18, &args![start, end]);
            }
        }
        let hr = e.vcall(s, 0xc, &args![buffer]).u32();
        if failed(hr) {
            return hr;
        }
        let data = e.mem.u32(buffer);
        let reader = Ptr::new(field(e, this, 0xa4));
        let result = fn_009fdfe0(
            e,
            reader,
            first as u32,
            (first >> 32) as u32,
            span_low,
            data,
            Ptr::new(length),
            s,
        );
        let actual = e.mem.u32(length);
        e.vcall(s, 0x30, &args![actual]);
        result
    })
}

// Translated from 009fdaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits for a finished sample (stdcall, `RET 0x10`): `E_POINTER` for a
/// null `out`; `009fe570` on the reader at `this + 0xa4` (`timeout`, the
/// sample into a local cleared first, `user`, and the length written over
/// the stack slot of `user`); on success the length goes to the sample
/// (vtable +0x30). The sample is stored in `out` either way; returns
/// `009fe570`'s result.
pub fn fn_009fdaf0(e: &mut Engine, this: Ptr, timeout: u32, out: Ptr, user: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    e.with_stack(8, |e, locals| {
        // locals + 0: the stack slot of `out` (the sample); locals + 4:
        // the stack slot of `user` (the length).
        let sample_slot = locals;
        let length_slot = offset(locals, 0x4);
        e.mem.set_u32(length_slot.addr(), user.addr());
        e.mem.set_u32(sample_slot.addr(), 0);
        let reader = Ptr::new(field(e, this, 0xa4));
        let hr = fn_009fe570(e, reader, timeout, sample_slot, user, length_slot);
        if !failed(hr) {
            let sample = e.mem.u32(sample_slot.addr());
            let length = e.mem.u32(length_slot.addr());
            e.vcall(sample, 0x30, &args![length]);
        }
        let sample = e.mem.u32(sample_slot.addr());
        e.mem.set_u32(out.addr(), sample);
        hr
    })
}

// Translated from 009fdb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its four words to `009fe100` on the reader at `this + 0xa4`
/// (stdcall, `RET 0x14`).
pub fn fn_009fdb60(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
) -> u32 {
    let reader = Ptr::new(field(e, this, 0xa4));
    fn_009fe100(e, reader, word_1, word_2, word_3, word_4)
}

// Translated from 009fdb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its two words to `009fe090` on the reader at `this + 0xa4`
/// (stdcall, `RET 0xc`).
pub fn fn_009fdb90(e: &mut Engine, this: Ptr, out: Ptr, word_2: Ptr) -> u32 {
    let reader = Ptr::new(field(e, this, 0xa4));
    fn_009fe090(e, reader, out, word_2)
}

// Translated from 009fdbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009fe690` on the reader at `this + 0xa4` (stdcall, `RET 4`).
pub fn fn_009fdbb0(e: &mut Engine, this: Ptr) -> u32 {
    let reader = Ptr::new(field(e, this, 0xa4));
    fn_009fe690(e, reader)
}

// Translated from 009fdbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009fe2a0` on the reader at `this + 0xa4` (stdcall, `RET 4`).
pub fn fn_009fdbd0(e: &mut Engine, this: Ptr) -> u32 {
    let reader = Ptr::new(field(e, this, 0xa4));
    fn_009fe2a0(e, reader)
}

// Translated from 009fdbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its two words to vtable +0x24 (thiscall) of the object at
/// `this + 0x8c` (stdcall, `RET 0xc`).
pub fn fn_009fdbf0(e: &mut Engine, this: Ptr, word_1: u32, word_2: u32) -> u32 {
    let target = field(e, this, 0x8c);
    e.vcall(target, 0x24, &args![word_1, word_2]).u32()
}

// Translated from 009fdc10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fdc10 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CAsyncReader` destructor shape (RTTI of the vtables it installs at +0,
/// +0xc, +0x10): frees the media-type block at +0x1a0 (`009f9c60`), destroys
/// the output pin at +0xf0 (`009fd3c0`) and the reader at +0x68
/// (`009fe870`), deletes the critical section at +0x50, then the filter
/// destructor shape `009fb550`.
pub fn fn_009fdc10(e: &mut Engine, this: Ptr) {
    set_field(e, this, 0x0, ASYNC_READER_VTABLE);
    set_field(e, this, 0xc, ASYNC_READER_VTABLE_C);
    set_field(e, this, 0x10, ASYNC_READER_VTABLE_10);
    fn_009f9c60(e, offset(this, 0x1a0));
    fn_009fd3c0(e, offset(this, 0xf0));
    fn_009fe870(e, offset(this, 0x68));
    e.call(DELETE_CRITICAL_SECTION, &args![offset(this, 0x50)]);
    fn_009fb550(e, this);
}

// Translated from 009fdcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pin at an index (thiscall, `RET 4`): `this + 0xf8` when the pin count
/// (vtable +0x18, thiscall) is positive, `index` is 0 and `this + 0xf0` is
/// not null; 0 otherwise.
pub fn fn_009fdcb0(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let count = e.vcall(this.addr(), 0x18, &args![]).i32();
    if count > 0 && index == 0 && this.addr().wrapping_add(0xf0) != 0 {
        return offset(this, 0xf8);
    }
    Ptr::NULL
}

// Translated from 009fdce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjusts its first stack word by -0x14 and jumps to `009fd3f0`.
pub fn fn_009fdce0(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    fn_009fd3f0(e, back(this, 0x14), id, out)
}

// Translated from 009fdd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjusts its stack word by -0x14 and jumps to `009fd430`.
pub fn fn_009fdd10(e: &mut Engine, this: Ptr) -> u32 {
    fn_009fd430(e, back(this, 0x14))
}

// Translated from 009fdd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjusts its stack word by -0x14 and jumps to `009fd410`.
pub fn fn_009fdd40(e: &mut Engine, this: Ptr) -> u32 {
    fn_009fd410(e, back(this, 0x14))
}

// Translated from 009fdd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CAsyncOutputPin` constructor shape (RTTI of the vtables it installs;
/// thiscall, `RET 0x10`): the pin base on `this + 8` (`009fc890`: no object
/// name, `filter`, `lock`, `status`, the wide name at `01093ed4`, direction
/// 1), then the vtables at +0, +8, +0x14, +0x18, `filter` at +0xa0 and
/// `reader` at +0xa4. Returns `this`.
pub fn fn_009fdd70(
    e: &mut Engine,
    this: Ptr,
    status: u32,
    filter: Ptr,
    reader: Ptr,
    lock: Ptr,
) -> Ptr {
    let pin = offset(this, 0x8);
    fn_009fc890(
        e,
        pin,
        Ptr::NULL,
        filter,
        lock,
        status,
        Ptr::new(NAME_STRING_01093ED4),
        1,
    );
    set_field(e, this, 0x8, ASYNC_OUTPUT_PIN_VTABLE_8);
    set_field(e, this, 0xa0, filter.addr());
    set_field(e, this, 0x0, ASYNC_OUTPUT_PIN_VTABLE);
    set_field(e, this, 0x14, ASYNC_OUTPUT_PIN_VTABLE_14);
    set_field(e, this, 0x18, ASYNC_OUTPUT_PIN_VTABLE_18);
    set_field(e, this, 0xa4, reader.addr());
    this
}

// Translated from 009fddf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fddf0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CAsyncReader` constructor shape (RTTI of the vtables it installs;
/// thiscall, `RET 0x10`): the filter base (`009fc5a0`: `name`, `outer`, the
/// critical section at +0x50, the id at `01093d70`, no status), the
/// vtables at +0, +0xc, +0x10, the critical section at +0x50 initialised,
/// the reader at +0x68 (`009fe1d0`, `stream`), the output pin at +0xf0
/// (`009fdd70`: `status`, `this`, the reader, the critical section) and the
/// media-type block at +0x1a0 (`009f9c70`). Returns `this`.
pub fn fn_009fddf0(
    e: &mut Engine,
    this: Ptr,
    name: Ptr,
    outer: Ptr,
    stream: Ptr,
    status: u32,
) -> Ptr {
    let section = offset(this, 0x50);
    fn_009fc5a0(
        e,
        this,
        name,
        outer,
        section,
        Ptr::new(CLASS_ID_01093D70),
        0,
    );
    set_field(e, this, 0x0, ASYNC_READER_VTABLE);
    set_field(e, this, 0xc, ASYNC_READER_VTABLE_C);
    set_field(e, this, 0x10, ASYNC_READER_VTABLE_10);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![section]);
    let reader = offset(this, 0x68);
    fn_009fe1d0(e, reader, stream);
    fn_009fdd70(e, offset(this, 0xf0), status, this, reader, section);
    fn_009f9c70(e, offset(this, 0x1a0));
    this
}

// Translated from 009fded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a 0x28-byte request (thiscall, `RET 0x24`): its nine words at
/// +0, +4, +8, +0xc, +0x14, +0x10, +0x18, +0x1c, +0x20 (the fifth word goes
/// to +0x14, the sixth to +0x10), and +0x24 = `VFW_E_TIMEOUT`. Returns 0.
pub fn fn_009fded0(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
    word_8: u32,
    word_9: u32,
) -> u32 {
    set_field(e, this, 0x0, word_1);
    set_field(e, this, 0x8, word_3);
    set_field(e, this, 0x4, word_2);
    set_field(e, this, 0x14, word_5);
    set_field(e, this, 0xc, word_4);
    set_field(e, this, 0x18, word_7);
    set_field(e, this, 0x10, word_6);
    set_field(e, this, 0x20, word_9);
    set_field(e, this, 0x1c, word_8);
    set_field(e, this, 0x24, VFW_E_TIMEOUT);
    0
}

// Translated from 009fdf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a request (thiscall, no stack words) on the object at `this + 4`:
/// vtable +0x14 first; vtable +4 (the words at +8, +0xc) and, when that
/// gives 0, vtable +8 (the words at +0x18, +0x14, +0x10 and a local for the
/// length), each result kept at +0x24; a result of 0x40000 with an object at
/// +0x1c calls that object's vtable +0x40 (stdcall, 1) and becomes 0. On
/// success a new length is stored at +0x14 with result 1, an unchanged one
/// gives 0. vtable +0x18 runs last; returns the result at +0x24.
pub fn fn_009fdf20(e: &mut Engine, this: Ptr) -> u32 {
    let target = field(e, this, 0x4);
    e.vcall(target, 0x14, &args![]);
    let target = field(e, this, 0x4);
    let first = field(e, this, 0x8);
    let second = field(e, this, 0xc);
    let hr = e.vcall(target, 0x4, &args![first, second]).u32();
    set_field(e, this, 0x24, hr);
    if hr == 0 {
        e.with_stack(4, |e, length| {
            let target = field(e, this, 0x4);
            let data = field(e, this, 0x18);
            let size = field(e, this, 0x14);
            let mode = field(e, this, 0x10);
            let hr = e.vcall(target, 0x8, &args![data, size, mode, length]).u32();
            set_field(e, this, 0x24, hr);
            if hr == 0x0004_0000 {
                let sample = field(e, this, 0x1c);
                if sample != 0 {
                    e.vcall(sample, 0x40, &args![1u32]);
                    set_field(e, this, 0x24, 0);
                }
            }
            if failed(field(e, this, 0x24)) {
                return;
            }
            let new_length = e.mem.u32(length.addr());
            if new_length != field(e, this, 0x14) {
                set_field(e, this, 0x14, new_length);
                set_field(e, this, 0x24, 1);
            } else {
                set_field(e, this, 0x24, 0);
            }
        });
    }
    // Both exits (009fdfa5, 009fdfbc) call vtable +0x18 and return +0x24.
    let target = field(e, this, 0x4);
    e.vcall(target, 0x18, &args![]);
    field(e, this, 0x24)
}

/// The alignment of the reader `this` (vtable +0x10, thiscall, of its
/// object at +0x18).
fn reader_alignment(e: &mut Engine, this: Ptr) -> u32 {
    let held = field(e, this, 0x18);
    e.vcall(held, 0x10, &args![]).u32()
}

// Translated from 009fdfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Synchronous aligned read (thiscall, `RET 0x18`): `E_POINTER` for a null
/// `out_length`; `VFW_E_BADALIGN` unless the position's low word, `length`
/// and `buffer` are multiples of the alignment (asked again for each). A
/// local request (`009fded0`: `this`, the object at +0x18, the position,
/// `length`, 1, `buffer`, `sample`, 0) is run (`009fdf20`), its length
/// stored in `out_length`, and its result returned.
pub fn fn_009fdfe0(
    e: &mut Engine,
    this: Ptr,
    position_low: u32,
    position_high: u32,
    length: u32,
    buffer: u32,
    out_length: Ptr,
    sample: u32,
) -> u32 {
    if out_length.is_null() {
        return E_POINTER;
    }
    if position_low & reader_alignment(e, this).wrapping_sub(1) != 0 {
        return VFW_E_BADALIGN;
    }
    if length & reader_alignment(e, this).wrapping_sub(1) != 0 {
        return VFW_E_BADALIGN;
    }
    if buffer & reader_alignment(e, this).wrapping_sub(1) != 0 {
        return VFW_E_BADALIGN;
    }
    e.with_stack(0x28, |e, request| {
        let held = field(e, this, 0x18);
        let hr = fn_009fded0(
            e,
            request,
            this.addr(),
            held,
            position_low,
            position_high,
            length,
            1,
            buffer,
            sample,
            0,
        );
        if failed(hr) {
            return hr;
        }
        let result = fn_009fdf20(e, request);
        let done = field(e, request, 0x14);
        e.mem.set_u32(out_length.addr(), done);
        result
    })
}

// Translated from 009fe090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the 64-bit result (`EDX:EAX`) of vtable +0xc (thiscall, `word`)
/// of the object at `this + 0x18` in `out` (thiscall, `RET 8`):
/// `E_POINTER` for a null `out`, otherwise 0.
pub fn fn_009fe090(e: &mut Engine, this: Ptr, out: Ptr, word: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let held = field(e, this, 0x18);
    let value = e.vcall(held, 0xc, &args![word]);
    e.mem.set_u32(out.addr(), value.eax);
    e.mem.set_u32(out.addr().wrapping_add(4), value.edx);
    0
}

// Translated from 009fe0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Signals the event at `this + 0x7c`; when a thread handle is held at
/// +0x80, waits for it without a time limit, closes and clears it.
/// Returns 0.
pub fn fn_009fe0c0(e: &mut Engine, this: Ptr) -> u32 {
    let event = field(e, this, 0x7c);
    e.call(SET_EVENT, &args![event]);
    let thread = field(e, this, 0x80);
    if thread != 0 {
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![thread, 0xffff_ffffu32]);
        let thread = field(e, this, 0x80);
        e.call(CLOSE_HANDLE, &args![thread]);
        set_field(e, this, 0x80, 0);
    }
    0
}

// Translated from 009fe100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queued read (thiscall, `RET 0x10`): when the position's low word,
/// `length` and `buffer` are multiples of the alignment (asked for each),
/// reads synchronously (`009fdfe0`: position, `length`, `buffer`, the length
/// written over the stack slot of `buffer`, no sample). Otherwise a local
/// request (`009fded0`: `this`, the object at +0x18, the position,
/// `length`, 0, `buffer`, 0, 0) is run (`009fdf20`) and its result returned.
pub fn fn_009fe100(
    e: &mut Engine,
    this: Ptr,
    position_low: u32,
    position_high: u32,
    length: u32,
    buffer: u32,
) -> u32 {
    let aligned = position_low & reader_alignment(e, this).wrapping_sub(1) == 0
        && length & reader_alignment(e, this).wrapping_sub(1) == 0
        && buffer & reader_alignment(e, this).wrapping_sub(1) == 0;
    if aligned {
        return e.with_stack(4, |e, buffer_slot| {
            e.mem.set_u32(buffer_slot.addr(), buffer);
            fn_009fdfe0(
                e,
                this,
                position_low,
                position_high,
                length,
                buffer,
                buffer_slot,
                0,
            )
        });
    }
    e.with_stack(0x28, |e, request| {
        let held = field(e, this, 0x18);
        let hr = fn_009fded0(
            e,
            request,
            this.addr(),
            held,
            position_low,
            position_high,
            length,
            0,
            buffer,
            0,
            0,
        );
        if failed(hr) {
            return hr;
        }
        fn_009fdf20(e, request)
    })
}

// Translated from 009fe1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the alignment (vtable +0x10 of the object at `this + 0x18`) in
/// `out` (thiscall, `RET 4`): `E_POINTER` for a null `out`, otherwise 0.
pub fn fn_009fe1a0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let alignment = reader_alignment(e, this);
    e.mem.set_u32(out.addr(), alignment);
    0
}

// Translated from 009fe1d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fe1d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Reader constructor shape (thiscall, `RET 4`): critical sections at +0
/// and +0x1c, `stream` at +0x18, +0x34 cleared, lists at +0x38 and +0x50
/// (`009fd120`), events at +0x68 (manual), +0x6c (manual), +0x78 (auto) and
/// +0x7c (manual) (`009fcfc0`), +0x70/+0x74/+0x80 cleared. Returns `this`.
pub fn fn_009fe1d0(e: &mut Engine, this: Ptr, stream: Ptr) -> Ptr {
    e.call(INITIALIZE_CRITICAL_SECTION, &args![this]);
    set_field(e, this, 0x18, stream.addr());
    e.call(INITIALIZE_CRITICAL_SECTION, &args![offset(this, 0x1c)]);
    set_field(e, this, 0x34, 0);
    fn_009fd120(e, offset(this, 0x38), 0);
    fn_009fd120(e, offset(this, 0x50), 0);
    fn_009fcfc0(e, offset(this, 0x68), 1);
    fn_009fcfc0(e, offset(this, 0x6c), 1);
    set_field(e, this, 0x70, 0);
    set_field(e, this, 0x74, 0);
    fn_009fcfc0(e, offset(this, 0x78), 0);
    fn_009fcfc0(e, offset(this, 0x7c), 1);
    set_field(e, this, 0x80, 0);
    this
}

// Translated from 009fe2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the critical section at `this + 0x1c`: clears +0x34 and signals
/// the event at +0x6c when the count at +0x58 is positive, resets it
/// otherwise. Returns 0.
pub fn fn_009fe2a0(e: &mut Engine, this: Ptr) -> u32 {
    let section = this.addr().wrapping_add(0x1c);
    enter(e, section);
    let pending = field(e, this, 0x58) as i32;
    set_field(e, this, 0x34, 0);
    let event = field(e, this, 0x6c);
    if pending > 0 {
        e.call(SET_EVENT, &args![event]);
    } else {
        e.call(RESET_EVENT, &args![event]);
    }
    leave(e, section);
    0
}

// Translated from 009fe2f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fe2f0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Takes the first queued request (under the critical section at
/// `this + 0x1c`): removes the head of the list at +0x38 (`009fd390`) and
/// resets the event at +0x68 when that list's count (+0x40) is then 0.
/// Returns the request (0 when none).
pub fn fn_009fe2f0(e: &mut Engine, this: Ptr) -> u32 {
    let section = this.addr().wrapping_add(0x1c);
    enter(e, section);
    let request = fn_009fd390(e, offset(this, 0x38));
    if field(e, this, 0x40) == 0 {
        let event = field(e, this, 0x68);
        e.call(RESET_EVENT, &args![event]);
    }
    leave(e, section);
    request
}

// Translated from 009fe370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fe370 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Takes the first finished request (under the critical section at
/// `this + 0x1c`): removes the head of the list at +0x50 (`009fd390`); when
/// that list's count (+0x58) is then 0, resets the event at +0x6c unless
/// +0x34 is set and +0x74 is clear. Returns the request (0 when none).
pub fn fn_009fe370(e: &mut Engine, this: Ptr) -> u32 {
    let section = this.addr().wrapping_add(0x1c);
    enter(e, section);
    let request = fn_009fd390(e, offset(this, 0x50));
    if field(e, this, 0x58) == 0 && (field(e, this, 0x34) == 0 || field(e, this, 0x74) != 0) {
        let event = field(e, this, 0x6c);
        e.call(RESET_EVENT, &args![event]);
    }
    leave(e, section);
    request
}

// Translated from 009fe570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits for a finished request (thiscall, `RET 0x10`): `E_POINTER` when
/// `out_sample`, `out_user` or `out_length` is null; clears `*out_sample`;
/// waits on the event at +0x6c (`timeout`; any other result than 0 is
/// `VFW_E_TIMEOUT`), takes a finished request (`009fe370`), and while there
/// is none returns `VFW_E_WRONG_STATE` when +0x34 is set and +0x74 clear
/// (checked under the critical section at +0x1c) or waits again. A request
/// with result 1 is checked: its position (+8/+0xc) plus its length (+0x14,
/// sign-extended) must equal the 64-bit value of vtable +0xc (thiscall, 0)
/// of the object at +0x18, else `E_FAIL`, otherwise 0. The request's length,
/// sample (+0x1c) and user word (+0x20) are handed out and the request is
/// deleted. Returns the (checked) result.
pub fn fn_009fe570(
    e: &mut Engine,
    this: Ptr,
    timeout: u32,
    out_sample: Ptr,
    out_user: Ptr,
    out_length: Ptr,
) -> u32 {
    if out_sample.is_null() || out_user.is_null() || out_length.is_null() {
        return E_POINTER;
    }
    e.mem.set_u32(out_sample.addr(), 0);
    let event = field(e, this, 0x6c);
    if e.call(WAIT_FOR_SINGLE_OBJECT, &args![event, timeout]).u32() != 0 {
        return VFW_E_TIMEOUT;
    }
    let request = loop {
        let request = fn_009fe370(e, this);
        if request != 0 {
            break Ptr::new(request);
        }
        let section = this.addr().wrapping_add(0x1c);
        enter(e, section);
        if field(e, this, 0x34) != 0 && field(e, this, 0x74) == 0 {
            leave(e, section);
            return VFW_E_WRONG_STATE;
        }
        leave(e, section);
        let event = field(e, this, 0x6c);
        if e.call(WAIT_FOR_SINGLE_OBJECT, &args![event, timeout]).u32() != 0 {
            return VFW_E_TIMEOUT;
        }
    };
    let mut result = field(e, request, 0x24);
    if result == 1 {
        let position_high = field(e, request, 0xc);
        let held = field(e, this, 0x18);
        let length = field(e, request, 0x14);
        let position_low = field(e, request, 0x8);
        let actual = e.vcall(held, 0xc, &args![0u32]).u64();
        let position = u64::from(position_low) | u64::from(position_high) << 32;
        let expected = position.wrapping_add(i64::from(length as i32) as u64);
        result = if expected == actual { 0 } else { E_FAIL };
    }
    let length = field(e, request, 0x14);
    e.mem.set_u32(out_length.addr(), length);
    let sample = field(e, request, 0x1c);
    e.mem.set_u32(out_sample.addr(), sample);
    let user = field(e, request, 0x20);
    e.mem.set_u32(out_user.addr(), user);
    e.call(OPERATOR_DELETE, &args![request]);
    result
}

// Translated from 009fe690 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fe690 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Flush shape (under the critical section at `this + 0x1c`): sets +0x34,
/// completes every queued request (`009fe2f0`, `009fe3f0`); when the count
/// at +0x70 is positive, sets +0x74 and waits (without a time limit, outside
/// the section) on the event at +0x78 until that count is 0, then clears
/// +0x74; signals the event at +0x6c. Returns 0.
pub fn fn_009fe690(e: &mut Engine, this: Ptr) -> u32 {
    let section = this.addr().wrapping_add(0x1c);
    enter(e, section);
    set_field(e, this, 0x34, 1);
    let mut request = fn_009fe2f0(e, this);
    while request != 0 {
        e.call(REQUEST_FINISH_009FE3F0, &args![this, request]);
        request = fn_009fe2f0(e, this);
    }
    if (field(e, this, 0x70) as i32) > 0 {
        set_field(e, this, 0x74, 1);
        leave(e, section);
        let event = field(e, this, 0x78);
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![event, 0xffff_ffffu32]);
        enter(e, section);
        while field(e, this, 0x70) != 0 {
            leave(e, section);
            let event = field(e, this, 0x78);
            e.call(WAIT_FOR_SINGLE_OBJECT, &args![event, 0xffff_ffffu32]);
            enter(e, section);
        }
        let event = field(e, this, 0x6c);
        set_field(e, this, 0x74, 0);
        e.call(SET_EVENT, &args![event]);
        leave(e, section);
    } else {
        let event = field(e, this, 0x6c);
        e.call(SET_EVENT, &args![event]);
        leave(e, section);
    }
    0
}

// Translated from 009fe870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fe870 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Reader destructor shape: flushes (`009fe690`), stops the worker
/// (`009fe0c0`), deletes every object left on the list at +0x50 (walked
/// with `009fd180` from its head), clears that list (`009fd140`), closes the
/// events at +0x7c, +0x78, +0x6c, +0x68 (`009fcfe0`), destroys the lists at
/// +0x50 and +0x38 (`009fd320`) and deletes the critical sections at +0x1c
/// and +0.
pub fn fn_009fe870(e: &mut Engine, this: Ptr) {
    fn_009fe690(e, this);
    fn_009fe0c0(e, this);
    let list = offset(this, 0x50);
    let head = field(e, this, 0x50);
    if head != 0 {
        e.with_stack(4, |e, position| {
            e.mem.set_u32(position.addr(), head);
            loop {
                let object = e.call(LIST_GET_NEXT, &args![position]).u32();
                e.call(OPERATOR_DELETE, &args![object]);
                if e.mem.u32(position.addr()) == 0 {
                    break;
                }
            }
        });
    }
    fn_009fd140(e, list);
    fn_009fcfe0(e, offset(this, 0x7c));
    fn_009fcfe0(e, offset(this, 0x78));
    fn_009fcfe0(e, offset(this, 0x6c));
    fn_009fcfe0(e, offset(this, 0x68));
    fn_009fd320(e, list);
    fn_009fd320(e, offset(this, 0x38));
    e.call(DELETE_CRITICAL_SECTION, &args![offset(this, 0x1c)]);
    e.call(DELETE_CRITICAL_SECTION, &args![this]);
}

// Translated from 009fea2a (decompiled, FalloutNV.exe 1.4.0.525)
/// One-time library start-up (no words; `PUSHAD`/`POPAD` around the body):
/// returns 0 when the done flag at `011a621c` is set. Otherwise loads
/// `nvapi.dll` (`LoadLibraryA`) with status -2; resolves
/// `nvapi_QueryInterface` (`GetProcAddress`), asks it (cdecl, one word) for
/// the function of the id at `0150e828` and calls that (no words); when all
/// of that works and the call's result is not negative, the status becomes
/// 0, the first cell -3, and every (cell, id) pair of the table at
/// `011a6228` up to a zero id gets the function the procedure gives for its
/// id (cells whose answer is 0 keep theirs). A missing procedure or
/// function, or a negative result, gives status -1 (a failed load leaves
/// -2). Status 0 sets the done flag. Returns the status.
pub fn fn_009fea2a(e: &mut Engine) -> u32 {
    if e.global::<u32>(LIBRARY_READY) != 0 {
        return 0;
    }
    let module = e.call(LOAD_LIBRARY_A, &args![LIBRARY_NAME]).u32();
    e.set_global(LIBRARY_STATUS, 0xffff_fffeu32);
    if module != 0 {
        let query = e
            .call(GET_PROC_ADDRESS, &args![module, LIBRARY_PROC_NAME])
            .u32();
        let mut ok = false;
        if query != 0 {
            let first = e.call(query, &args![LIBRARY_FIRST_ID]).u32();
            if first != 0 && !failed(e.call(first, &args![]).u32()) {
                ok = true;
            }
        }
        if ok {
            e.set_global(LIBRARY_STATUS, 0u32);
            e.set_global(LIBRARY_FIRST_CELL, 0xffff_fffdu32);
            let mut entry = LIBRARY_TABLE;
            loop {
                let id = e.global::<u32>(entry.wrapping_add(4));
                if id == 0 {
                    break;
                }
                let function = e.call(query, &args![id]).u32();
                if function != 0 {
                    e.set_global(entry, function);
                }
                entry = entry.wrapping_add(8);
            }
        } else {
            e.set_global(LIBRARY_STATUS, 0xffff_ffffu32);
        }
    }
    let status = e.global::<u32>(LIBRARY_STATUS);
    if status == 0 {
        e.set_global(LIBRARY_READY, 1u32);
    }
    status
}

// Translated from 009febd8 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011a6398]`: a cell of `009fea2a`'s table (a function
/// pointer, not an import slot); the two words (cdecl: the caller at
/// 004db479 pops 8 bytes) go unchanged to the function it holds.
pub fn fn_009febd8(e: &mut Engine, word_0: u32, word_1: Ptr) -> u32 {
    let target = e.global::<u32>(LIBRARY_CELL_009FEBD8);
    e.call(target, &args![word_0, word_1]).u32()
}

// Batch n078: the rest of this unit. Class names again come only from the
// RTTI records of the vtables (`CEnumPins` `01093c50`, `CEnumMediaTypes`
// `01093c74`, `CMediaSample` `01093c9c`, `CMemAllocator` `01093cf8` and
// `01093d28`, `CAsyncOutputPin` `01093d84`/`01093d9c`, `CAsyncReader`
// `01093e54`/`01093e6c`/`01093eac`, `CBSAReader` `0107cd6c`/`0107cdac`,
// `TrespassPackage` `010939ec`); slot offsets below are byte offsets into
// those vtables.

/// `TESPackage::InitLoadGame` (engine map), thiscall with one word.
const PACKAGE_INIT_LOAD_GAME: u32 = 0x0067_9a90;
/// `004839c0` (cdecl, one word; `fallout shared/tesform.cpp` in the engine
/// map): 0, or the word `00853130` gives for its argument.
const FORM_LOOKUP: u32 = 0x0048_39c0;

// Import slots (Ghidra's labels for them).
/// `GetSystemInfo`.
const GET_SYSTEM_INFO: u32 = 0x00fd_f0f8;
/// `VirtualAlloc`.
const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
/// `VirtualFree`.
const VIRTUAL_FREE: u32 = 0x00fd_f10c;
/// `ReleaseSemaphore`.
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
/// `CreateSemaphoreA`.
const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
/// `CoInitialize`.
const CO_INITIALIZE: u32 = 0x00fd_f454;
/// `CoCreateInstance`.
const CO_CREATE_INSTANCE: u32 = 0x00fd_f46c;
/// `CoFreeUnusedLibraries`.
const CO_FREE_UNUSED_LIBRARIES: u32 = 0x00fd_f468;
/// `CoUninitialize`.
const CO_UNINITIALIZE: u32 = 0x00fd_f464;
/// `WaitForMultipleObjects`.
const WAIT_FOR_MULTIPLE_OBJECTS: u32 = 0x00fd_f108;
/// `CreateThread`.
const CREATE_THREAD: u32 = 0x00fd_f124;
/// `GetLastError`.
const GET_LAST_ERROR: u32 = 0x00fd_f094;

// More HRESULT values (Windows SDK names).
const VFW_E_BUFFER_OVERFLOW: u32 = 0x8004_020d;
const VFW_E_ALREADY_COMMITTED: u32 = 0x8004_020f;
const VFW_E_BUFFERS_OUTSTANDING: u32 = 0x8004_0210;
const VFW_E_NOT_COMMITTED: u32 = 0x8004_0211;
const VFW_E_SIZENOTSET: u32 = 0x8004_0212;
const VFW_E_NO_CLOCK: u32 = 0x8004_0213;
const VFW_E_NOT_FOUND: u32 = 0x8004_0216;
const VFW_E_SAMPLE_TIME_NOT_SET: u32 = 0x8004_0249;
const VFW_E_MEDIA_TIME_NOT_SET: u32 = 0x8004_0251;
const VFW_S_NO_STOP_TIME: u32 = 0x0004_0270;
/// The HRESULT of Win32 error 2 (`ERROR_FILE_NOT_FOUND`), which the filter
/// registration treats as success.
const HRESULT_FILE_NOT_FOUND: u32 = 0x8007_0002;

// More 16-byte ids in `.rdata` (first word in each comment).
/// `56a86892-...`.
const INTERFACE_ID_01093C2C: u32 = 0x0109_3c2c;
/// `89c31040-...`.
const INTERFACE_ID_01093C1C: u32 = 0x0109_3c1c;
/// `56a8689a-...`.
const INTERFACE_ID_01093BDC: u32 = 0x0109_3bdc;
/// `36b73884-...`.
const INTERFACE_ID_01093BCC: u32 = 0x0109_3bcc;
/// `56a8689c-...`.
const INTERFACE_ID_01093BBC: u32 = 0x0109_3bbc;
/// `379a0cf0-...`.
const INTERFACE_ID_01093BAC: u32 = 0x0109_3bac;
/// `56a868a3-...`.
const INTERFACE_ID_01093B8C: u32 = 0x0109_3b8c;
/// `e436ebb2-...`: the class `009fba40`/`009fbab0` create.
const CLASS_ID_0107CB08: u32 = 0x0107_cb08;

/// `CMediaSample` (RTTI).
const MEDIA_SAMPLE_VTABLE: u32 = 0x0109_3c9c;
/// `CMemAllocator` (RTTI), its vtables at +0 and +0xc.
const MEM_ALLOCATOR_VTABLE: u32 = 0x0109_3d28;
const MEM_ALLOCATOR_VTABLE_C: u32 = 0x0109_3cf8;

/// The thread routine `009fe780` starts (`009fe560`, a function Ghidra did
/// not create).
const THREAD_ROUTINE: u32 = 0x009f_e560;

/// The 16-byte id at `at` as the four words a by-value push puts on the
/// stack (first word at the lowest address).
fn id_words(e: &Engine, at: u32) -> [u32; 4] {
    [
        e.mem.u32(at),
        e.mem.u32(at.wrapping_add(4)),
        e.mem.u32(at.wrapping_add(8)),
        e.mem.u32(at.wrapping_add(0xc)),
    ]
}

/// The critical section the `CMemAllocator` interface methods lock: `this +
/// 4` (the object's +0x10), or 0 when the object (`this - 0xc`) is null.
fn allocator_lock(this: Ptr) -> u32 {
    if this.addr().wrapping_sub(0xc) != 0 {
        this.addr().wrapping_add(0x4)
    } else {
        0
    }
}

/// The byte at `base + offset`.
fn byte_field(e: &Engine, base: Ptr, offset: u32) -> u8 {
    e.mem.u8(base.addr().wrapping_add(offset))
}

// Translated from 009f98e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x64 of the `TrespassPackage` vtable (thiscall, `RET 4`):
/// `TESPackage::InitLoadGame` (`00679a90`) with the stack word, then the
/// words at +0x88 and +0x8c are each replaced by `004839c0` of them (0
/// stays 0).
pub fn fn_009f98e0(e: &mut Engine, this: Ptr, data: u32) {
    e.call(PACKAGE_INIT_LOAD_GAME, &args![this, data]);
    for at in [0x88u32, 0x8c] {
        let held = field(e, this, at);
        let value = if held != 0 {
            e.call(FORM_LOOKUP, &args![held]).u32()
        } else {
            0
        };
        set_field(e, this, at, value);
    }
}

// Translated from 009f9a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Media-type test (thiscall on the type, no stack words; called by
/// `009fc0b0`): 1 when the 16-byte id at +0 or the one at +0x2c equals the
/// id at `01093f04` (sixteen zero bytes in the exe's data), else 0.
pub fn fn_009f9a50(e: &mut Engine, media_type: Ptr) -> u32 {
    let any = Ptr::new(INTERFACE_ID_01093F04);
    if fn_009f9980(e, media_type, any) == 0 && fn_009f9980(e, offset(media_type, 0x2c), any) == 0 {
        return 0;
    }
    1
}

// Translated from 009f9a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Media-type match (thiscall, `RET 4`; called by `009fbfc0`): 0 unless the
/// ids at +0 and +0x10 of `other` are each the id at `01093f04` or equal to
/// `this`'s; then 1 when `other`'s id at +0x2c is the `01093f04` id, else
/// that id must equal `this`'s, the sizes at +0x40 must match and the
/// blocks at +0x44 must hold the same bytes (compared word by word, then
/// byte by byte from the first differing word or the tail).
pub fn fn_009f9a80(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    let any = Ptr::new(INTERFACE_ID_01093F04);
    if fn_009f9980(e, other, any) == 0 && fn_009f9980(e, this, other) == 0 {
        return 0;
    }
    let other_sub = offset(other, 0x10);
    if fn_009f9980(e, other_sub, any) == 0 && fn_009f9980(e, offset(this, 0x10), other_sub) == 0 {
        return 0;
    }
    let other_format = offset(other, 0x2c);
    if fn_009f9980(e, other_format, any) != 0 {
        return 1;
    }
    if fn_009f9980(e, offset(this, 0x2c), other_format) == 0 {
        return 0;
    }
    let mut left = field(e, this, 0x40);
    if left != field(e, other, 0x40) {
        return 0;
    }
    if left == 0 {
        return 1;
    }
    let mut b = field(e, other, 0x44);
    let mut a = field(e, this, 0x44);
    let mut word_differs = false;
    while left >= 4 {
        if e.mem.u32(a) != e.mem.u32(b) {
            word_differs = true;
            break;
        }
        left -= 4;
        b = b.wrapping_add(4);
        a = a.wrapping_add(4);
    }
    if !word_differs && left == 0 {
        return 1;
    }
    let mut difference: i32 = 0;
    for step in 0..4 {
        difference = i32::from(e.mem.u8(a)) - i32::from(e.mem.u8(b));
        // The fourth compare has no count test after it.
        if difference != 0 || step == 3 {
            break;
        }
        left -= 1;
        b = b.wrapping_add(1);
        a = a.wrapping_add(1);
        if left == 0 {
            break;
        }
    }
    u32::from(difference == 0)
}

// Translated from 009f9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Media-type duplicate (stdcall, `RET 4`): a 0x48-byte block from
/// `CoTaskMemAlloc` gets a copy of `source` (`009f9bb0`); a failed copy
/// frees the block again (`CoTaskMemFree`). Returns the block, or 0.
pub fn fn_009f9cd0(e: &mut Engine, source: Ptr) -> Ptr {
    let block = e.call(CO_TASK_MEM_ALLOC, &args![0x48u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    if !failed(fn_009f9bb0(e, Ptr::new(block), source)) {
        return Ptr::new(block);
    }
    e.call(CO_TASK_MEM_FREE, &args![block]);
    Ptr::NULL
}

// Translated from 009f9db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x10 of the `CAsyncReader` and `CBSAReader` vtables (thiscall,
/// `RET 4`): `VFW_E_NO_CLOCK` without the clock at +0x18; otherwise asks
/// it for the time into `out` (vtable +0xc) and, on success, subtracts the
/// 64-bit start time at +0x20 from it and returns 0. A failure is returned
/// as is.
pub fn fn_009f9db0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let clock = field(e, this, 0x18);
    if clock == 0 {
        return VFW_E_NO_CLOCK;
    }
    let hr = e.vcall(clock, 0xc, &args![out]).u32();
    if failed(hr) {
        return hr;
    }
    // SUB [ESI],EAX then SBB [ESI+4],ECX.
    let low = e.mem.u32(out.addr());
    let start_low = field(e, this, 0x20);
    e.mem.set_u32(out.addr(), low.wrapping_sub(start_low));
    let borrow = u32::from(low < start_low);
    let start_high = field(e, this, 0x24);
    let high = e.mem.u32(out.addr().wrapping_add(4));
    e.mem.set_u32(
        out.addr().wrapping_add(4),
        high.wrapping_sub(start_high).wrapping_sub(borrow),
    );
    0
}

// Translated from 009f9f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `E_NOTIMPL` (slot +0x38 of the `CAsyncReader` and `CBSAReader`
/// vtables; stdcall, `RET 8`: neither word is read).
pub fn fn_009f9f40(_e: &mut Engine, _unused_0: u32, _unused_1: u32) -> u32 {
    E_NOTIMPL
}

/// The shared shape of `009f9fe0` and `009fa120`: `E_POINTER` for a null
/// `out`; the ids `own` and `01093f14` hand out `this` (`009fceb0`); any
/// other clears `out` and gives `E_NOINTERFACE`.
fn enumerator_lookup(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr, own: u32) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    if fn_009f9980(e, id, Ptr::new(own)) == 0
        && fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093F14)) == 0
    {
        e.mem.set_u32(out.addr(), 0);
        return E_NOINTERFACE;
    }
    fn_009fceb0(e, this, out)
}

// Translated from 009f9fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0 of the `CEnumPins` vtable (stdcall, `RET 0xc`): `E_POINTER` for
/// a null `out`; the ids at `01093c2c` and `01093f14` hand out `this`
/// (`009fceb0`); any other id clears `out` and returns `E_NOINTERFACE`.
pub fn fn_009f9fe0(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    enumerator_lookup(e, this, id, out, INTERFACE_ID_01093C2C)
}

// Translated from 009fa040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +4 of the `CEnumPins` vtable (stdcall, `RET 4` through the jump):
/// `InterlockedIncrement` of the count at `this + 0x14`, whose result it
/// returns.
pub fn fn_009fa040(e: &mut Engine, this: Ptr) -> u32 {
    e.call(INTERLOCKED_INCREMENT, &args![offset(this, 0x14)])
        .u32()
}

// Translated from 009fa090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x14 of the `CEnumPins` vtable (stdcall, `RET 4`): the filter at
/// +0xc gives the version (vtable +0x14, to +0x10) and the total (vtable
/// +0x18, to +8), the position (+4) goes back to 0 and the list at +0x18 is
/// cleared (`009fd140`). Returns 0.
pub fn fn_009fa090(e: &mut Engine, this: Ptr) -> u32 {
    fn_009fa0d0(e, this);
    fn_009fd140(e, offset(this, 0x18));
    0
}

// Translated from 009fa0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CEnumPins` refresh (stdcall, `RET 4`; called by `009fbb50`): the
/// filter at +0xc gives the version (vtable +0x14, to +0x10) and the total
/// (vtable +0x18, to +8), and the position (+4) goes back to 0. Returns 0.
pub fn fn_009fa0d0(e: &mut Engine, this: Ptr) -> u32 {
    let filter = field(e, this, 0xc);
    let version = e.vcall(filter, 0x14, &args![]).u32();
    set_field(e, this, 0x10, version);
    let filter = field(e, this, 0xc);
    let total = e.vcall(filter, 0x18, &args![]).u32();
    set_field(e, this, 0x8, total);
    set_field(e, this, 0x4, 0);
    0
}

// Translated from 009fa120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0 of the `CEnumMediaTypes` vtable (stdcall, `RET 0xc`):
/// `E_POINTER` for a null `out`; the ids at `01093c1c` and `01093f14` hand
/// out `this` (`009fceb0`); any other id clears `out` and returns
/// `E_NOINTERFACE`.
pub fn fn_009fa120(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    enumerator_lookup(e, this, id, out, INTERFACE_ID_01093C1C)
}

// Translated from 009fa180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +4 of the `CEnumMediaTypes` vtable (stdcall, `RET 4` through the
/// jump): `InterlockedIncrement` of the count at `this + 0x10`, whose result
/// it returns.
pub fn fn_009fa180(e: &mut Engine, this: Ptr) -> u32 {
    e.call(INTERLOCKED_INCREMENT, &args![offset(this, 0x10)])
        .u32()
}

// Translated from 009fa1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x14 of the `CEnumMediaTypes` vtable (stdcall, `RET 4`): the
/// position (+4) goes back to 0 and the pin at +8 gives the version again
/// (vtable +0x10, to +0xc). Returns 0.
pub fn fn_009fa1d0(e: &mut Engine, this: Ptr) -> u32 {
    let pin = field(e, this, 0x8);
    set_field(e, this, 0x4, 0);
    let version = e.vcall(pin, 0x10, &args![]).u32();
    set_field(e, this, 0xc, version);
    0
}

// Translated from 009fa4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x2c of the `CAsyncOutputPin` vtable at `01093d9c` (stdcall,
/// `RET 8`): `E_POINTER` for a null `out`; otherwise the object at
/// `this - 0xc` is asked (vtable +0x20, thiscall with `out`), and a
/// negative answer becomes 1 (`S_FALSE`); any other answer is returned.
pub fn fn_009fa4a0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let hr = e.vcall(back(this, 0xc).addr(), 0x20, &args![out]).u32();
    if failed(hr) {
        1
    } else {
        hr
    }
}

// Translated from 009fa4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x10 of the `CAsyncOutputPin` vtable at `01093d84` (stdcall,
/// `RET 8`): stores `value` at +0x1c under the critical section held at
/// +0x10. Returns 0.
pub fn fn_009fa4e0(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let section = field(e, this, 0x10);
    enter(e, section);
    set_field(e, this, 0x1c, value);
    leave(e, section);
    0
}

// Translated from 009fa510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `E_NOTIMPL` (slot +0xc of the `CAsyncOutputPin` vtable at
/// `01093d84`; stdcall, `RET 0x20`: none of its eight words is read).
pub fn fn_009fa510(
    _e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
) -> u32 {
    E_NOTIMPL
}

// Translated from 009fa520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x44 of the `CAsyncOutputPin` vtable at `01093d9c` (stdcall,
/// `RET 0x1c`): stores its four words at +0x74, +0x78, +0x7c and +0x80 and
/// the `double` after them at +0x84 (`FLD`/`FSTP qword`, which quiets a
/// signalling NaN). Returns 0.
pub fn fn_009fa520(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    rate: f64,
) -> u32 {
    let loaded = x87_load64(rate.to_bits());
    set_field(e, this, 0x74, word_1);
    set_field(e, this, 0x78, word_2);
    set_field(e, this, 0x7c, word_3);
    set_field(e, this, 0x80, word_4);
    e.mem
        .set_u64(this.addr().wrapping_add(0x84), loaded.to_bits());
    0
}

// Translated from 009fa560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CMediaSample` destructor shape (thiscall, no stack words; RTTI of the
/// vtable it installs): frees the media type held at +0x3c (`009f9cb0`)
/// when there is one.
pub fn fn_009fa560(e: &mut Engine, this: Ptr) {
    set_field(e, this, 0x0, MEDIA_SAMPLE_VTABLE);
    let media_type = field(e, this, 0x3c);
    if media_type != 0 {
        fn_009f9cb0(e, Ptr::new(media_type));
    }
}

// Translated from 009fa580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0 of the `CMediaSample` vtable (stdcall, `RET 0xc`): the ids at
/// `01093bdc`, `01093bcc` and `01093f14` hand out `this` (`009fceb0`);
/// any other returns `E_NOINTERFACE` (`out` untouched).
pub fn fn_009fa580(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    for own in [
        INTERFACE_ID_01093BDC,
        INTERFACE_ID_01093BCC,
        INTERFACE_ID_01093F14,
    ] {
        if fn_009f9980(e, id, Ptr::new(own)) != 0 {
            return fn_009fceb0(e, this, out);
        }
    }
    E_NOINTERFACE
}

// Translated from 009fa5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +4 of the `CMediaSample` vtable (stdcall, `RET 4` through the
/// jump): `InterlockedIncrement` of the count at `this + 0x44`, whose result
/// it returns.
pub fn fn_009fa5d0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(INTERLOCKED_INCREMENT, &args![offset(this, 0x44)])
        .u32()
}

// Translated from 009fa5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +8 of the `CMediaSample` vtable (stdcall, `RET 4`): a count (+0x44)
/// of exactly 1 is cleared without the interlocked call; otherwise
/// `InterlockedDecrement`, and a nonzero result is returned. At zero the
/// sample's media type is dropped (its vtable +0x38 with 0, when flag 8 at
/// +4 is set), the words at +4, +8 and +0x40 are cleared and the sample
/// goes back to its allocator (the interface at +0xc of the object at
/// +0x18, vtable +0x20 with the sample). Returns 0 then.
pub fn fn_009fa5f0(e: &mut Engine, this: Ptr) -> u32 {
    let remaining = if field(e, this, 0x44) == 1 {
        set_field(e, this, 0x44, 0);
        0
    } else {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![offset(this, 0x44)])
            .u32();
        if count != 0 {
            return count;
        }
        0
    };
    if byte_field(e, this, 0x4) & 0x8 != 0 {
        e.vcall(this.addr(), 0x38, &args![0u32]);
    }
    let allocator = field(e, this, 0x18).wrapping_add(0xc);
    set_field(e, this, 0x4, 0);
    set_field(e, this, 0x8, 0);
    set_field(e, this, 0x40, 0);
    e.vcall(allocator, 0x20, &args![this]);
    remaining
}

// Translated from 009fa650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0xc of the `CMediaSample` vtable (stdcall, `RET 8`): stores the
/// buffer address (+0xc) in `out`. Returns 0.
pub fn fn_009fa650(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let buffer = field(e, this, 0xc);
    e.mem.set_u32(out.addr(), buffer);
    0
}

// Translated from 009fa670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x10 of the `CMediaSample` vtable (stdcall, `RET 4`): the buffer
/// size at +0x14.
pub fn fn_009fa670(e: &mut Engine, this: Ptr) -> u32 {
    field(e, this, 0x14)
}

// Translated from 009fa680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x14 of the `CMediaSample` vtable (stdcall, `RET 0xc`): with flag
/// 0x100 at +4 the start (+0x20) and end (+0x28) times go to `start` and
/// `end` and 0 is returned; with only flag 0x10 the start goes to `start`,
/// start + 1 to `end`, and `VFW_S_NO_STOP_TIME` is returned; with neither,
/// `VFW_E_SAMPLE_TIME_NOT_SET`.
pub fn fn_009fa680(e: &mut Engine, this: Ptr, start: Ptr, end: Ptr) -> u32 {
    let flags = field(e, this, 0x4);
    let s = start.addr();
    let t = end.addr();
    if flags & 0x100 == 0 {
        if flags & 0x10 == 0 {
            return VFW_E_SAMPLE_TIME_NOT_SET;
        }
        let low = field(e, this, 0x20);
        e.mem.set_u32(s, low);
        let high = field(e, this, 0x24);
        e.mem.set_u32(s.wrapping_add(4), high);
        let low = field(e, this, 0x20);
        let high = field(e, this, 0x24);
        let next = (u64::from(high) << 32 | u64::from(low)).wrapping_add(1);
        e.mem.set_u32(t, next as u32);
        e.mem.set_u32(t.wrapping_add(4), (next >> 32) as u32);
        return VFW_S_NO_STOP_TIME;
    }
    let low = field(e, this, 0x20);
    e.mem.set_u32(s, low);
    let high = field(e, this, 0x24);
    e.mem.set_u32(s.wrapping_add(4), high);
    let low = field(e, this, 0x28);
    e.mem.set_u32(t, low);
    let high = field(e, this, 0x2c);
    e.mem.set_u32(t.wrapping_add(4), high);
    0
}

// Translated from 009fa6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x18 of the `CMediaSample` vtable (stdcall, `RET 0xc`): without
/// `start` clears flags 0x110 at +4; with `start` only, stores it at +0x20,
/// clears flag 0x100 and sets 0x10; with both, stores them at +0x20 and
/// +0x28 and sets 0x110. Returns 0.
pub fn fn_009fa6f0(e: &mut Engine, this: Ptr, start: Ptr, end: Ptr) -> u32 {
    if start.is_null() {
        let flags = field(e, this, 0x4) & 0xffff_feef;
        set_field(e, this, 0x4, flags);
        return 0;
    }
    let s = start.addr();
    if end.is_null() {
        let low = e.mem.u32(s);
        set_field(e, this, 0x20, low);
        let flags = field(e, this, 0x4);
        let high = e.mem.u32(s.wrapping_add(4));
        set_field(e, this, 0x24, high);
        set_field(e, this, 0x4, (flags & 0xffff_feff) | 0x10);
        return 0;
    }
    let t = end.addr();
    let low = e.mem.u32(s);
    set_field(e, this, 0x20, low);
    let high = e.mem.u32(s.wrapping_add(4));
    set_field(e, this, 0x24, high);
    let low = e.mem.u32(t);
    set_field(e, this, 0x28, low);
    let high = e.mem.u32(t.wrapping_add(4));
    let flags = field(e, this, 0x4) | 0x110;
    set_field(e, this, 0x4, flags);
    set_field(e, this, 0x2c, high);
    0
}

// Translated from 009fa760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x44 of the `CMediaSample` vtable (stdcall, `RET 0xc`): without
/// flag 0x20 at +4 `VFW_E_MEDIA_TIME_NOT_SET`; otherwise the 64-bit media
/// start (+0x30) goes to `start` and start + the signed length at +0x38 to
/// `end`. Returns 0.
pub fn fn_009fa760(e: &mut Engine, this: Ptr, start: Ptr, end: Ptr) -> u32 {
    if byte_field(e, this, 0x4) & 0x20 == 0 {
        return VFW_E_MEDIA_TIME_NOT_SET;
    }
    let s = start.addr();
    let low = field(e, this, 0x30);
    e.mem.set_u32(s, low);
    let high = field(e, this, 0x34);
    e.mem.set_u32(s.wrapping_add(4), high);
    // CDQ of +0x38, ADD/ADC with +0x30/+0x34.
    let length = field(e, this, 0x38);
    let low = field(e, this, 0x30);
    let sum_low = length.wrapping_add(low);
    let carry = u32::from(sum_low < length);
    let sign: u32 = if (length as i32) < 0 { 0xffff_ffff } else { 0 };
    let high = field(e, this, 0x34);
    let sum_high = sign.wrapping_add(high).wrapping_add(carry);
    let t = end.addr();
    e.mem.set_u32(t, sum_low);
    e.mem.set_u32(t.wrapping_add(4), sum_high);
    0
}

// Translated from 009fa7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x48 of the `CMediaSample` vtable (stdcall, `RET 0xc`): without
/// `start` clears flag 0x20 at +4; otherwise stores `start` at +0x30, the
/// low-word difference `end - start` at +0x38 (`end` is read without a null
/// test) and sets flag 0x20. Returns 0.
pub fn fn_009fa7a0(e: &mut Engine, this: Ptr, start: Ptr, end: Ptr) -> u32 {
    if start.is_null() {
        let flags = field(e, this, 0x4) & 0xffff_ffdf;
        set_field(e, this, 0x4, flags);
        return 0;
    }
    let s = start.addr();
    let low = e.mem.u32(s);
    set_field(e, this, 0x30, low);
    let high = e.mem.u32(s.wrapping_add(4));
    set_field(e, this, 0x34, high);
    let length = e.mem.u32(end.addr()).wrapping_sub(e.mem.u32(s));
    let flags = field(e, this, 0x4) | 0x20;
    set_field(e, this, 0x4, flags);
    set_field(e, this, 0x38, length);
    0
}

/// Inverted flag bit `bit` of the byte at +4 (`NOT`/`MOVZX`/`SHR`/`AND 1`).
fn flag_clear(e: &Engine, this: Ptr, bit: u32) -> u32 {
    (u32::from(!byte_field(e, this, 0x4)) >> bit) & 1
}

/// Sets (`value` nonzero) or clears `mask` in the word at +4. Returns 0.
fn flag_set(e: &mut Engine, this: Ptr, value: u32, mask: u32) -> u32 {
    let flags = field(e, this, 0x4);
    let flags = if value != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    set_field(e, this, 0x4, flags);
    0
}

// Translated from 009fa7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x1c of the `CMediaSample` vtable (stdcall, `RET 4`): 1 when flag
/// 1 at +4 is clear, else 0.
pub fn fn_009fa7e0(e: &mut Engine, this: Ptr) -> u32 {
    flag_clear(e, this, 0)
}

// Translated from 009fa800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x20 of the `CMediaSample` vtable (stdcall, `RET 8`): sets flag 1
/// at +4 for a nonzero `value`, clears it otherwise. Returns 0.
pub fn fn_009fa800(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    flag_set(e, this, value, 0x1)
}

// Translated from 009fa820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x3c of the `CMediaSample` vtable (stdcall, `RET 4`): 1 when flag
/// 4 at +4 is clear, else 0.
pub fn fn_009fa820(e: &mut Engine, this: Ptr) -> u32 {
    flag_clear(e, this, 2)
}

// Translated from 009fa840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x40 of the `CMediaSample` vtable (stdcall, `RET 8`): sets flag 4
/// at +4 for a nonzero `value`, clears it otherwise. Returns 0.
pub fn fn_009fa840(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    flag_set(e, this, value, 0x4)
}

// Translated from 009fa860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x24 of the `CMediaSample` vtable (stdcall, `RET 4`): 1 when flag
/// 2 at +4 is clear, else 0.
pub fn fn_009fa860(e: &mut Engine, this: Ptr) -> u32 {
    flag_clear(e, this, 1)
}

// Translated from 009fa880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x28 of the `CMediaSample` vtable (stdcall, `RET 8`): sets flag 2
/// at +4 for a nonzero `value`, clears it otherwise. Returns 0.
pub fn fn_009fa880(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    flag_set(e, this, value, 0x2)
}

// Translated from 009fa8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x2c of the `CMediaSample` vtable (stdcall, `RET 4`): the data
/// length at +0x10.
pub fn fn_009fa8a0(e: &mut Engine, this: Ptr) -> u32 {
    field(e, this, 0x10)
}

// Translated from 009fa8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x30 of the `CMediaSample` vtable (stdcall, `RET 8`):
/// `VFW_E_BUFFER_OVERFLOW` when `length` is above the buffer size at +0x14
/// (signed); otherwise stores it at +0x10 and returns 0.
pub fn fn_009fa8b0(e: &mut Engine, this: Ptr, length: i32) -> u32 {
    if length > field(e, this, 0x14) as i32 {
        return VFW_E_BUFFER_OVERFLOW;
    }
    set_field(e, this, 0x10, length as u32);
    0
}

// Translated from 009fa8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x34 of the `CMediaSample` vtable (stdcall, `RET 8`): without flag
/// 8 at +4 clears `out` and returns 1 (`S_FALSE`); otherwise stores a copy
/// of the media type at +0x3c (`009f9cd0`) in `out` and returns 0, or
/// `E_OUTOFMEMORY` when the copy is 0.
pub fn fn_009fa8d0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if byte_field(e, this, 0x4) & 0x8 == 0 {
        e.mem.set_u32(out.addr(), 0);
        return 1;
    }
    let media_type = field(e, this, 0x3c);
    let copy = fn_009f9cd0(e, Ptr::new(media_type)).addr();
    e.mem.set_u32(out.addr(), copy);
    allocated_or_out_of_memory(copy)
}

// Translated from 009fa910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x38 of the `CMediaSample` vtable (stdcall, `RET 8`): frees the
/// media type held at +0x3c (`009f9cb0`) and clears it; without a new
/// `media_type` clears flag 8 at +4 and returns 0; otherwise holds a copy
/// (`009f9cd0`) and sets flag 8 (0 returned), or clears the flag and
/// returns `E_OUTOFMEMORY` when the copy fails.
pub fn fn_009fa910(e: &mut Engine, this: Ptr, media_type: Ptr) -> u32 {
    let held = field(e, this, 0x3c);
    if held != 0 {
        fn_009f9cb0(e, Ptr::new(held));
        set_field(e, this, 0x3c, 0);
    }
    if media_type.is_null() {
        let flags = field(e, this, 0x4) & 0xffff_fff7;
        set_field(e, this, 0x4, flags);
        // EAX still holds the null `media_type`.
        return 0;
    }
    let copy = fn_009f9cd0(e, media_type).addr();
    set_field(e, this, 0x3c, copy);
    if copy == 0 {
        let flags = field(e, this, 0x4) & 0xffff_fff7;
        set_field(e, this, 0x4, flags);
        return E_OUTOFMEMORY;
    }
    let flags = field(e, this, 0x4) | 0x8;
    set_field(e, this, 0x4, flags);
    0
}

// Translated from 009fa960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x4c of the `CMediaSample` vtable (stdcall, `RET 0xc`): nothing
/// for a `size` of 0 (0 returned), `E_POINTER` for a null `out`; otherwise
/// a 0x30-byte property block is built on the stack and its first
/// min(`size`, 0x30) bytes copied to `out` (`memcpy`): +0 that count, +4
/// the word at +8, +8 the flags (+4) without 0x20, +0xc the length (+0x10),
/// +0x10/+0x14 the start time (+0x20), +0x18/+0x1c the end time (+0x28),
/// +0x20 the word at +0x40, +0x24 the media type (+0x3c, when flag 8 is
/// set, else 0), +0x28 the buffer (+0xc), +0x2c its size (+0x14).
/// Returns 0.
pub fn fn_009fa960(e: &mut Engine, this: Ptr, size: u32, out: Ptr) -> u32 {
    if size == 0 {
        return 0;
    }
    if out.is_null() {
        return E_POINTER;
    }
    let count = size.min(0x30);
    e.with_stack(0x30, |e, local| {
        let l = local.addr();
        let flags = field(e, this, 0x4);
        e.mem.set_u32(l + 0x8, flags & 0xffff_ffdf);
        for (to, from) in [
            (0x4u32, 0x8u32),
            (0x28, 0xc),
            (0x2c, 0x14),
            (0xc, 0x10),
            (0x10, 0x20),
            (0x14, 0x24),
            (0x18, 0x28),
            (0x1c, 0x2c),
            (0x20, 0x40),
        ] {
            let value = field(e, this, from);
            e.mem.set_u32(l + to, value);
        }
        e.mem.set_u32(l, count);
        let media_type = if flags & 0x8 != 0 {
            field(e, this, 0x3c)
        } else {
            0
        };
        e.mem.set_u32(l + 0x24, media_type);
        e.call(MEMCPY, &args![out, local, count]);
    });
    0
}

// Translated from 009faa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x50 of the `CMediaSample` vtable (stdcall, `RET 0xc`): 0 for a
/// `size` below 4, `E_POINTER` for a null `properties`. The block's own
/// size (+0) caps `size`; either above 0x30 is `E_INVALIDARG`, as are
/// (each test only when `size` covers the field) flags (+8) with a bit of
/// 0xfffffe20 set, flag 0x10 on a sample without it when `size` is below
/// 0x20, a buffer (+0x28) other than this one's (+0xc), a buffer size
/// (+0x2c) other than this one's (+0x14) and a length (+0xc) above that
/// size (signed; a zero size compares as 0). A new media type (+0x24, when
/// flag 8 is set and `size` reaches 0x28) is copied first (`009f9cd0`; a
/// null one is `E_POINTER`, a failed copy `E_OUTOFMEMORY`). Then, as far as
/// `size` reaches: +0x20 to +0x40; the flags (keeping 0x20) to +4 and +4 to
/// +8 (or only +4 to +8 for a size of 8 to 0xb); +0xc to +0x10; the end
/// time +0x18 to +0x28; the start time +0x10 to +0x20; the copied type to
/// +0x3c (the old one freed, `009f9cb0`). Flag 8 at +4 ends set when a type
/// is held. Returns 0.
pub fn fn_009faa20(e: &mut Engine, this: Ptr, size: u32, properties: Ptr) -> u32 {
    if size < 4 {
        return 0;
    }
    if properties.is_null() {
        return E_POINTER;
    }
    let declared = field(e, properties, 0x0);
    let size = size.min(declared);
    if size > 0x30 || declared > 0x30 {
        return E_INVALIDARG;
    }
    if size >= 0xc {
        let flags = field(e, properties, 0x8);
        if flags & 0xffff_fe20 != 0 {
            return E_INVALIDARG;
        }
        if flags & 0x10 != 0 && byte_field(e, this, 0x4) & 0x10 == 0 && size < 0x20 {
            return E_INVALIDARG;
        }
    }
    if size >= 0x2c {
        let buffer = field(e, properties, 0x28);
        if buffer != 0 && buffer != field(e, this, 0xc) {
            return E_INVALIDARG;
        }
    }
    if size >= 0x30 {
        let buffer_size = field(e, properties, 0x2c);
        if buffer_size != 0 && buffer_size != field(e, this, 0x14) {
            return E_INVALIDARG;
        }
        // The exe tests `size >= 0x10` again here (always true).
        if size >= 0x10 && (buffer_size as i32) < field(e, properties, 0xc) as i32 {
            return E_INVALIDARG;
        }
    }
    let mut copy = 0u32;
    if size >= 0x28 && byte_field(e, properties, 0x8) & 0x8 != 0 {
        let media_type = field(e, properties, 0x24);
        if media_type == 0 {
            return E_POINTER;
        }
        copy = fn_009f9cd0(e, Ptr::new(media_type)).addr();
        if copy == 0 {
            return E_OUTOFMEMORY;
        }
    }
    if size >= 0x24 {
        let stream = field(e, properties, 0x20);
        set_field(e, this, 0x40, stream);
    }
    if size >= 0xc {
        let flags = (field(e, this, 0x4) & 0x20) | field(e, properties, 0x8);
        set_field(e, this, 0x4, flags);
        let specific = field(e, properties, 0x4);
        set_field(e, this, 0x8, specific);
    } else if size >= 0x8 {
        let specific = field(e, properties, 0x4);
        set_field(e, this, 0x8, specific);
    }
    if size >= 0x10 {
        let length = field(e, properties, 0xc);
        set_field(e, this, 0x10, length);
    }
    if size >= 0x20 {
        let low = field(e, properties, 0x18);
        set_field(e, this, 0x28, low);
        let high = field(e, properties, 0x1c);
        set_field(e, this, 0x2c, high);
    }
    if size >= 0x18 {
        let low = field(e, properties, 0x10);
        set_field(e, this, 0x20, low);
        let high = field(e, properties, 0x14);
        set_field(e, this, 0x24, high);
    }
    if size >= 0x28 && byte_field(e, properties, 0x8) & 0x8 != 0 {
        let held = field(e, this, 0x3c);
        if held != 0 {
            fn_009f9cb0(e, Ptr::new(held));
        }
        set_field(e, this, 0x3c, copy);
    }
    let flags = field(e, this, 0x4);
    let flags = if field(e, this, 0x3c) != 0 {
        flags | 0x8
    } else {
        flags & 0xffff_fff7
    };
    set_field(e, this, 0x4, flags);
    0
}

// Translated from 009fabb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fabb0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Allocator base destructor shape (thiscall, no stack words; called by the
/// `CMemAllocator` destructor `009fb1c0`): closes the handle at +0x30 when
/// set (`CloseHandle`), releases (vtable +8) the object at +0x58 when set,
/// deletes the critical section at +0x10 and drops the object count
/// (`009fce80`).
pub fn fn_009fabb0(e: &mut Engine, this: Ptr) {
    let handle = field(e, this, 0x30);
    if handle != 0 {
        e.call(CLOSE_HANDLE, &args![handle]);
    }
    if field(e, this, 0x58) != 0 {
        let held = field(e, this, 0x58);
        e.vcall(held, 0x8, &args![]);
    }
    e.call(DELETE_CRITICAL_SECTION, &args![offset(this, 0x10)]);
    fn_009fce80(e);
}

// Translated from 009fac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0 of the `CMemAllocator` vtable at `01093d28` (stdcall,
/// `RET 0xc`): the id at `01093bbc`, and the id at `01093bac` while the
/// word at +0x5c is set, hand out the interface at +0xc (`009fceb0`, 0 for
/// a null `this`); any other id goes to `009fcf10`.
pub fn fn_009fac30(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    let hands_out = fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093BBC)) != 0
        || (fn_009f9980(e, id, Ptr::new(INTERFACE_ID_01093BAC)) != 0 && field(e, this, 0x5c) != 0);
    if !hands_out {
        return fn_009fcf10(e, this, id, out);
    }
    interface_of(e, this, 0xc, out)
}

// Translated from 009faca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x10 of the `CMemAllocator` vtable at `01093cf8` (stdcall,
/// `RET 8`; `this` is the interface at object + 0xc): `E_POINTER` for a
/// null `out`; otherwise, under the allocator's lock, fills `out` with the
/// count (+0x2c), buffer size (+0x34), alignment (+0x38) and prefix
/// (+0x3c). Returns 0.
pub fn fn_009faca0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let lock = allocator_lock(this);
    enter(e, lock);
    for (to, from) in [(0x4u32, 0x34u32), (0x0, 0x2c), (0x8, 0x38), (0xc, 0x3c)] {
        let value = field(e, this, from);
        set_field(e, out, to, value);
    }
    leave(e, lock);
    0
}

// Translated from 009fad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x1c of the `CMemAllocator` vtable at `01093cf8` (stdcall,
/// `RET 0x14`; the two time words are not read): clears `out`, then under
/// the lock `VFW_E_NOT_COMMITTED` while +0x44 is 0; takes the first free
/// sample (+0x1c, next at the sample's +0x1c, free count +0x20 down), marks
/// it (+0x44 = 1) and returns it in `out` with 0. Without a free sample the
/// waiting count (+0x28) grows; with flag 4 in `flags` that is
/// `VFW_E_TIMEOUT`, otherwise it waits on the event at +0x24 (no timeout)
/// and tries again.
pub fn fn_009fad00(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    flags: u32,
) -> u32 {
    e.mem.set_u32(out.addr(), 0);
    loop {
        let lock = allocator_lock(this);
        enter(e, lock);
        if field(e, this, 0x44) == 0 {
            leave(e, lock);
            return VFW_E_NOT_COMMITTED;
        }
        let sample = field(e, this, 0x1c);
        if sample != 0 {
            let next = e.mem.u32(sample.wrapping_add(0x1c));
            let free = field(e, this, 0x20).wrapping_sub(1);
            set_field(e, this, 0x20, free);
            set_field(e, this, 0x1c, next);
        } else {
            let waiting = field(e, this, 0x28).wrapping_add(1);
            set_field(e, this, 0x28, waiting);
        }
        leave(e, lock);
        if sample != 0 {
            e.mem.set_u32(sample.wrapping_add(0x44), 1);
            e.mem.set_u32(out.addr(), sample);
            return 0;
        }
        if flags & 0x4 != 0 {
            return VFW_E_TIMEOUT;
        }
        let event = field(e, this, 0x24);
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![event, 0xffff_ffffu32]);
    }
}

// Translated from 009fada0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fada0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x24 of the `CMemAllocator` vtable at `01093cf8` (stdcall,
/// `RET 8`): under the lock adds a reference to `notify` (vtable +4, when
/// set), releases the one held at +0x4c (vtable +8, when set) and holds
/// `notify` there. Returns 0.
pub fn fn_009fada0(e: &mut Engine, this: Ptr, notify: Ptr) -> u32 {
    let lock = allocator_lock(this);
    enter(e, lock);
    if !notify.is_null() {
        e.vcall(notify.addr(), 0x4, &args![]);
    }
    let held = field(e, this, 0x4c);
    if held != 0 {
        e.vcall(held, 0x8, &args![]);
    }
    set_field(e, this, 0x4c, notify.addr());
    leave(e, lock);
    0
}

// Translated from 009fae30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x28 of the `CMemAllocator` vtable at `01093cf8` (stdcall,
/// `RET 8`): under the lock stores count (+0x2c) - allocated (+0x30) + free
/// (+0x20) in `out` (no null test). Returns 0.
pub fn fn_009fae30(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let lock = allocator_lock(this);
    enter(e, lock);
    let value = field(e, this, 0x2c)
        .wrapping_sub(field(e, this, 0x30))
        .wrapping_add(field(e, this, 0x20));
    e.mem.set_u32(out.addr(), value);
    leave(e, lock);
    0
}

// Translated from 009fae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocator wake-up (thiscall on the object, no stack words; called by
/// `009faf50` and `009fc480`): when the waiting count at +0x34 is set,
/// `ReleaseSemaphore` of the handle at +0x30 by that count (no previous
/// count wanted) and the count cleared.
pub fn fn_009fae70(e: &mut Engine, this: Ptr) {
    let waiting = field(e, this, 0x34);
    if waiting != 0 {
        let semaphore = field(e, this, 0x30);
        e.call(RELEASE_SEMAPHORE, &args![semaphore, waiting, 0u32]);
        set_field(e, this, 0x34, 0);
    }
}

// Translated from 009fae90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fae90 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x14 of the `CMemAllocator` vtable at `01093cf8` (stdcall,
/// `RET 4`): under the lock, when not yet committed (+0x44 is 0) sets it;
/// a pending decommit (+0x48) is then just cancelled, otherwise the object
/// (`this - 0xc`) allocates (vtable +0x14, thiscall) and on success a
/// reference is added (vtable +4 of `this`). A failed allocation clears the
/// commit flag again and is returned; otherwise 0.
pub fn fn_009fae90(e: &mut Engine, this: Ptr) -> u32 {
    let lock = allocator_lock(this);
    enter(e, lock);
    if field(e, this, 0x44) == 0 {
        let pending = field(e, this, 0x48);
        set_field(e, this, 0x44, 1);
        if pending != 0 {
            set_field(e, this, 0x48, 0);
        } else {
            let hr = e.vcall(back(this, 0xc).addr(), 0x14, &args![]).u32();
            if failed(hr) {
                set_field(e, this, 0x44, 0);
                leave(e, lock);
                return hr;
            }
            e.vcall(this.addr(), 0x4, &args![]);
        }
    }
    leave(e, lock);
    0
}

// Translated from 009faf50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009faf50 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x18 of the `CMemAllocator` vtable at `01093cf8` (stdcall,
/// `RET 4`; also called by `009fb1c0`): nothing while neither committed
/// (+0x44) nor a decommit pending (+0x48). Otherwise clears the commit;
/// with fewer free samples (+0x20) than allocated (+0x30, signed) the
/// decommit is left pending, else it is cleared, the object (`this - 0xc`)
/// frees (vtable +0x10, thiscall) and a reference is released after the
/// lock (vtable +8 of `this`). Waiters are woken (`009fae70`) in both
/// cases. Returns 0.
pub fn fn_009faf50(e: &mut Engine, this: Ptr) -> u32 {
    let lock = allocator_lock(this);
    enter(e, lock);
    if field(e, this, 0x44) == 0 && field(e, this, 0x48) == 0 {
        leave(e, lock);
        return 0;
    }
    let free = field(e, this, 0x20) as i32;
    let allocated = field(e, this, 0x30) as i32;
    set_field(e, this, 0x44, 0);
    let release = if free < allocated {
        set_field(e, this, 0x48, 1);
        false
    } else {
        set_field(e, this, 0x48, 0);
        e.vcall(back(this, 0xc).addr(), 0x10, &args![]);
        true
    };
    fn_009fae70(e, back(this, 0xc));
    leave(e, lock);
    if release {
        e.vcall(this.addr(), 0x8, &args![]);
    }
    0
}

// Translated from 009fb020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocator size check (thiscall on the object, no stack words; called by
/// `009fcc60`): `VFW_E_SIZENOTSET` unless the count (+0x38), buffer size
/// (+0x40) and alignment (+0x44) are all positive (signed); then 1 when the
/// word at +0x4c is 0, else 0.
pub fn fn_009fb020(e: &mut Engine, this: Ptr) -> u32 {
    if field(e, this, 0x38) as i32 > 0
        && field(e, this, 0x40) as i32 > 0
        && field(e, this, 0x44) as i32 > 0
    {
        return u32::from(field(e, this, 0x4c) == 0);
    }
    VFW_E_SIZENOTSET
}

// Translated from 009fb050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0xc of the `CMemAllocator` vtable at `01093cf8` (stdcall,
/// `RET 0xc`): `E_POINTER` for a null `actual`; otherwise under the lock
/// clears `actual` (16 bytes) and reads `GetSystemInfo` into a local. The
/// requested alignment (`request` +8) must be nonzero and divide the
/// allocation granularity (`VFW_E_BADALIGN`), the allocator must not be
/// committed (+0x44 == 1: `VFW_E_ALREADY_COMMITTED`) and no buffers may be
/// out (free +0x20 below allocated +0x30, signed:
/// `VFW_E_BUFFERS_OUTSTANDING`). The buffer size plus prefix (`request` +4
/// and +0xc) is rounded up to the alignment (signed `IDIV`); size minus
/// prefix, the count (+0), the alignment and the prefix go to +0x34,
/// +0x2c, +0x38, +0x3c and to `actual` (+4, +0, +8, +0xc); +0x40 is set.
/// Returns 0.
pub fn fn_009fb050(e: &mut Engine, this: Ptr, request: Ptr, actual: Ptr) -> u32 {
    if actual.is_null() {
        return E_POINTER;
    }
    let lock = allocator_lock(this);
    enter(e, lock);
    for at in [0x0u32, 0x4, 0x8, 0xc] {
        set_field(e, actual, at, 0);
    }
    e.with_stack(0x24, |e, info| {
        e.call(GET_SYSTEM_INFO, &args![info]);
        let alignment = field(e, request, 0x8);
        // SYSTEM_INFO +0x1c: the allocation granularity.
        if alignment == 0 || field(e, info, 0x1c) & alignment.wrapping_sub(1) != 0 {
            leave(e, lock);
            return VFW_E_BADALIGN;
        }
        if field(e, this, 0x44) == 1 {
            leave(e, lock);
            return VFW_E_ALREADY_COMMITTED;
        }
        if (field(e, this, 0x20) as i32) < field(e, this, 0x30) as i32 {
            leave(e, lock);
            return VFW_E_BUFFERS_OUTSTANDING;
        }
        let mut size = field(e, request, 0x4).wrapping_add(field(e, request, 0xc));
        let remainder = (size as i32).wrapping_rem(alignment as i32) as u32;
        if remainder != 0 {
            size = size.wrapping_add(alignment.wrapping_sub(remainder));
        }
        let buffer = size.wrapping_sub(field(e, request, 0xc));
        set_field(e, this, 0x34, buffer);
        set_field(e, actual, 0x4, buffer);
        for (from, to) in [(0x0u32, 0x2cu32), (0x8, 0x38), (0xc, 0x3c)] {
            let value = field(e, request, from);
            set_field(e, this, to, value);
            set_field(e, actual, from, value);
        }
        set_field(e, this, 0x40, 1);
        leave(e, lock);
        0
    })
}

// Translated from 009fb170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocator free-all (thiscall on the object, no stack words; called by
/// `009fb1c0` and `009fcc60`): deletes every sample of the list at +0x28
/// (next at the sample's +0x1c, count +0x2c down; the sample's vtable +0x54
/// with 1), clears +0x3c and releases the block at +0x60 (`VirtualFree`,
/// size 0, `MEM_RELEASE` 0x8000) when there is one.
pub fn fn_009fb170(e: &mut Engine, this: Ptr) {
    loop {
        let sample = field(e, this, 0x28);
        if sample == 0 {
            break;
        }
        let next = e.mem.u32(sample.wrapping_add(0x1c));
        let count = field(e, this, 0x2c).wrapping_sub(1);
        set_field(e, this, 0x2c, count);
        set_field(e, this, 0x28, next);
        e.vcall(sample, 0x54, &args![1u32]);
    }
    let block = field(e, this, 0x60);
    set_field(e, this, 0x3c, 0);
    if block != 0 {
        e.call(VIRTUAL_FREE, &args![block, 0u32, 0x8000u32]);
        set_field(e, this, 0x60, 0);
    }
}

// Translated from 009fb1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fb1c0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CMemAllocator` destructor shape (thiscall, no stack words; RTTI of the
/// vtables it installs at +0 and +0xc): decommits (`009faf50` on the
/// interface at +0xc), frees everything (`009fb170`) and runs the base
/// destructor (`009fabb0`).
pub fn fn_009fb1c0(e: &mut Engine, this: Ptr) {
    set_field(e, this, 0x0, MEM_ALLOCATOR_VTABLE);
    set_field(e, this, 0xc, MEM_ALLOCATOR_VTABLE_C);
    fn_009faf50(e, offset(this, 0xc));
    fn_009fb170(e, this);
    fn_009fabb0(e, this);
}

// Translated from 009fb230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0 of the `CMemAllocator` vtable at `01093cf8`: replaces its first
/// stack word with the outer object at `this - 8` and jumps to that
/// object's vtable +0 (the interface query: the object, an id and an out
/// pointer).
pub fn fn_009fb230(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    let outer = e.mem.u32(this.addr().wrapping_sub(0x8));
    e.vcall(outer, 0x0, &args![id, out]).u32()
}

// Translated from 009fb250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +4 of the `CMemAllocator` vtable at `01093cf8`: replaces its stack
/// word with the outer object at `this - 8` and jumps to its vtable +4 (the
/// reference add; one word).
pub fn fn_009fb250(e: &mut Engine, this: Ptr) -> u32 {
    let outer = e.mem.u32(this.addr().wrapping_sub(0x8));
    e.vcall(outer, 0x4, &args![]).u32()
}

// Translated from 009fb270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +8 of the `CMemAllocator` vtable at `01093cf8`: replaces its stack
/// word with the outer object at `this - 8` and jumps to its vtable +8 (the
/// release; one word).
pub fn fn_009fb270(e: &mut Engine, this: Ptr) -> u32 {
    let outer = e.mem.u32(this.addr().wrapping_sub(0x8));
    e.vcall(outer, 0x8, &args![]).u32()
}

// Translated from 009fb290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Filter (un)registration through a mapper (stdcall, `RET 0xc`; called by
/// `009fba40` with 1 and `009fbab0` with 0): 1 for a null `setup`.
/// Otherwise the mapper (stdcall methods) first drops the class whose
/// 16-byte id `setup` +0 points at (vtable +0x1c, the id by value); with
/// `register` set it then adds it (vtable +0xc: id, name +4, merit +8) and,
/// while the calls succeed, each of the +0xc pins of the 0x24-byte array at
/// +0x10 (vtable +0x14: id, the pin's words +0 to +0x10, the id its +0x14
/// points at, its +0x18) and each of a pin's +0x1c (major, minor) id
/// pointer pairs at +0x20 (vtable +0x18: id, the pin's name, both ids by
/// value). The last result is returned, with 0x80070002 turned into 0.
pub fn fn_009fb290(e: &mut Engine, setup: Ptr, mapper: Ptr, register: u32) -> u32 {
    if setup.is_null() {
        return 1;
    }
    let m = mapper.addr();
    let id = id_words(e, field(e, setup, 0x0));
    let mut hr = e.vcall(m, 0x1c, &args![id[0], id[1], id[2], id[3]]).u32();
    if register != 0 {
        let merit = field(e, setup, 0x8);
        let name = field(e, setup, 0x4);
        let id = id_words(e, field(e, setup, 0x0));
        hr = e
            .vcall(m, 0xc, &args![id[0], id[1], id[2], id[3], name, merit])
            .u32();
        if !failed(hr) && field(e, setup, 0xc) != 0 {
            let mut pin_index: u32 = 0;
            let mut pin_offset: u32 = 0;
            'pins: loop {
                let pin = field(e, setup, 0x10).wrapping_add(pin_offset);
                let connects_pin = e.mem.u32(pin.wrapping_add(0x18));
                let connects = id_words(e, e.mem.u32(pin.wrapping_add(0x14)));
                let many = e.mem.u32(pin.wrapping_add(0x10));
                let zero = e.mem.u32(pin.wrapping_add(0xc));
                let output = e.mem.u32(pin.wrapping_add(0x8));
                let rendered = e.mem.u32(pin.wrapping_add(0x4));
                let name = e.mem.u32(pin.wrapping_add(0x0));
                let id = id_words(e, field(e, setup, 0x0));
                hr = e
                    .vcall(
                        m,
                        0x14,
                        &args![
                            id[0],
                            id[1],
                            id[2],
                            id[3],
                            name,
                            rendered,
                            output,
                            zero,
                            many,
                            connects[0],
                            connects[1],
                            connects[2],
                            connects[3],
                            connects_pin
                        ],
                    )
                    .u32();
                if failed(hr) {
                    break;
                }
                let pins = field(e, setup, 0x10);
                if e.mem.u32(pin_offset.wrapping_add(pins).wrapping_add(0x1c)) != 0 {
                    let mut type_index: u32 = 0;
                    loop {
                        let pins = field(e, setup, 0x10);
                        let types = e.mem.u32(pin_offset.wrapping_add(pins).wrapping_add(0x20));
                        let entry = types.wrapping_add(type_index.wrapping_mul(8));
                        let minor = id_words(e, e.mem.u32(entry.wrapping_add(4)));
                        let major = id_words(e, e.mem.u32(entry));
                        let name = e.mem.u32(pin_offset.wrapping_add(pins));
                        let id = id_words(e, field(e, setup, 0x0));
                        hr = e
                            .vcall(
                                m,
                                0x18,
                                &args![
                                    id[0], id[1], id[2], id[3], name, major[0], major[1], major[2],
                                    major[3], minor[0], minor[1], minor[2], minor[3]
                                ],
                            )
                            .u32();
                        if failed(hr) {
                            break 'pins;
                        }
                        type_index = type_index.wrapping_add(1);
                        let pins = field(e, setup, 0x10);
                        if type_index >= e.mem.u32(pin_offset.wrapping_add(pins).wrapping_add(0x1c))
                        {
                            break;
                        }
                    }
                }
                pin_index = pin_index.wrapping_add(1);
                pin_offset = pin_offset.wrapping_add(0x24);
                if pin_index >= field(e, setup, 0xc) {
                    break;
                }
            }
        }
    }
    if hr == HRESULT_FILE_NOT_FOUND {
        0
    } else {
        hr
    }
}

// Translated from 009fb650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fb650 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x24 of the `CAsyncReader` vtable at `01093e6c` (and of
/// `CBSAReader`'s; stdcall, `RET 8`): `E_POINTER` for a null `out`;
/// otherwise, under the critical section held at +0x2c, adds a reference
/// to the object at +0xc (vtable +4, when set) and stores it in `out`.
/// Returns 0.
pub fn fn_009fb650(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let section = field(e, this, 0x2c);
    enter(e, section);
    if field(e, this, 0xc) != 0 {
        let held = field(e, this, 0xc);
        e.vcall(held, 0x4, &args![]);
    }
    let held = field(e, this, 0xc);
    e.mem.set_u32(out.addr(), held);
    leave(e, section);
    0
}

// Translated from 009fb960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fb960 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x2c of the `CAsyncReader` vtable at `01093e6c` (and of
/// `CBSAReader`'s; stdcall, `RET 0xc`): `E_POINTER` for a null `out`;
/// otherwise, under the critical section held at +0x2c, walks the pins of
/// the object at `this - 0xc` (count vtable +0x18, pin vtable +0x1c with
/// the index; thiscall) and for the first whose wide name (+0x14) equals
/// `name` (`009fd030`) stores its interface (+0xc) in `out`, adds a
/// reference (vtable +4) and returns 0. None: clears `out`,
/// `VFW_E_NOT_FOUND`.
pub fn fn_009fb960(e: &mut Engine, this: Ptr, name: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let section = field(e, this, 0x2c);
    enter(e, section);
    let filter = back(this, 0xc).addr();
    let count = e.vcall(filter, 0x18, &args![]).u32() as i32;
    let mut index: i32 = 0;
    while index < count {
        let pin = e.vcall(filter, 0x1c, &args![index]).u32();
        let pin_name = e.mem.u32(pin.wrapping_add(0x14));
        if fn_009fd030(e, Ptr::new(pin_name), name) == 0 {
            let interface = pin.wrapping_add(0xc);
            e.mem.set_u32(out.addr(), interface);
            e.vcall(interface, 0x4, &args![]);
            leave(e, section);
            return 0;
        }
        index += 1;
    }
    e.mem.set_u32(out.addr(), 0);
    leave(e, section);
    VFW_E_NOT_FOUND
}

/// The shared body of `009fba40` and `009fbab0`: the setup block from the
/// object at `this - 0x10` (vtable +0x20, thiscall; 1 returned at once when
/// it is 0), `CoInitialize(0)`, the mapper from `CoCreateInstance` (class
/// `0107cb08`, no outer object, context 1, interface `01093b8c`, written
/// over the stack slot of `this`), `009fb290` with `register` and a release
/// of the mapper (vtable +8) when that worked, `CoFreeUnusedLibraries`,
/// `CoUninitialize`. Gives the setup test's 1 or the last result.
fn register_filter(e: &mut Engine, this: Ptr, register: u32) -> Option<u32> {
    let setup = e.vcall(back(this, 0x10).addr(), 0x20, &args![]).u32();
    if setup == 0 {
        return None;
    }
    e.call(CO_INITIALIZE, &args![0u32]);
    let hr = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), this.addr());
        let hr = e
            .call(
                CO_CREATE_INSTANCE,
                &args![CLASS_ID_0107CB08, 0u32, 1u32, INTERFACE_ID_01093B8C, slot],
            )
            .u32();
        if failed(hr) {
            return hr;
        }
        let mapper = e.mem.u32(slot.addr());
        let hr = fn_009fb290(e, Ptr::new(setup), Ptr::new(mapper), register);
        let mapper = e.mem.u32(slot.addr());
        e.vcall(mapper, 0x8, &args![]);
        hr
    });
    e.call(CO_FREE_UNUSED_LIBRARIES, &args![]);
    e.call(CO_UNINITIALIZE, &args![]);
    Some(hr)
}

// Translated from 009fba40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0xc of the `CAsyncReader` vtable at `01093e54` (stdcall, `RET 4`):
/// registers the filter (`009fb290` with 1 through the mapper class
/// `0107cb08`, see `register_filter`). Returns 1 without a setup block,
/// else 0 whatever the registration gave.
pub fn fn_009fba40(e: &mut Engine, this: Ptr) -> u32 {
    match register_filter(e, this, 1) {
        None => 1,
        Some(_) => 0,
    }
}

// Translated from 009fbab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x10 of the `CAsyncReader` vtable at `01093e54` (stdcall,
/// `RET 4`): unregisters the filter (`009fb290` with 0, see
/// `register_filter`). Returns 1 without a setup block; otherwise the
/// `CoCreateInstance` failure or `009fb290`'s result, 0x80070002 turned
/// into 0.
pub fn fn_009fbab0(e: &mut Engine, this: Ptr) -> u32 {
    match register_filter(e, this, 0) {
        None => 1,
        Some(hr) if hr == HRESULT_FILE_NOT_FOUND => 0,
        Some(hr) => hr,
    }
}

// Translated from 009fbc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x10 of the `CEnumPins` vtable (stdcall, `RET 8`):
/// `VFW_E_ENUM_OUT_OF_SYNC` when the filter's version (+0xc, vtable +0x14)
/// is not the one held at +0x10; 1 (`S_FALSE`) when `count` is more than
/// the pins left (total +8 - position +4, unsigned); otherwise moves the
/// position on by `count` and returns 0.
pub fn fn_009fbc50(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    let filter = field(e, this, 0xc);
    if e.vcall(filter, 0x14, &args![]).u32() != field(e, this, 0x10) {
        return VFW_E_ENUM_OUT_OF_SYNC;
    }
    let position = field(e, this, 0x4);
    let left = field(e, this, 0x8).wrapping_sub(position);
    if count > left {
        return 1;
    }
    set_field(e, this, 0x4, position.wrapping_add(count));
    0
}

// Translated from 009fbe40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fbe40 - C++ exception unwinding (FS:[0] frame state) and the /GS cookie check (00ec408c) are not translated.
/// Slot +0x10 of the `CEnumMediaTypes` vtable (stdcall, `RET 8`): 0 for a
/// `count` of 0; `VFW_E_ENUM_OUT_OF_SYNC` when the pin's version (+8,
/// vtable +0x10) is not the one at +0xc; otherwise moves the position (+4)
/// on by `count` and asks the pin for the type at position - 1 (vtable
/// +0x34, thiscall, into a stack media type made by `009f9c70` and freed by
/// `009f9c60`). Returns 1 when that answer is nonzero, else 0.
pub fn fn_009fbe40(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    let pin = field(e, this, 0x8);
    if e.vcall(pin, 0x10, &args![]).u32() != field(e, this, 0xc) {
        return VFW_E_ENUM_OUT_OF_SYNC;
    }
    let position = field(e, this, 0x4).wrapping_add(count);
    set_field(e, this, 0x4, position);
    e.with_stack(0x48, |e, media_type| {
        fn_009f9c70(e, media_type);
        let pin = field(e, this, 0x8);
        let at = field(e, this, 0x4).wrapping_sub(1);
        let hr = e.vcall(pin, 0x34, &args![at, media_type]).u32();
        fn_009f9c60(e, media_type);
        u32::from(hr != 0)
    })
}

// Translated from 009fc190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fc190 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x10 of the `CAsyncOutputPin` vtable at `01093d9c` (stdcall,
/// `RET 0xc`; `this` is the interface at pin + 0xc): `E_POINTER` for a
/// null `connector` or `media_type`. Under the critical section held at
/// +0x14: `VFW_E_ALREADY_CONNECTED` while +0xc holds a pin;
/// `VFW_E_NOT_STOPPED` while the filter (+0x1c) has a state (+0x14) and
/// the byte at +0x19 is 0. The pin object (`this - 0xc`, thiscall) then
/// checks the connector (vtable +0x28) and the type (vtable +0x20); a
/// failure breaks the connection (vtable +0x2c) and is returned (a nonzero
/// type answer that is not a failure, `E_FAIL` or `E_INVALIDARG` becomes
/// `VFW_E_TYPE_NOT_ACCEPTED`). Then the connector is held at +0xc with a
/// reference, the type set (vtable +0x24) and the connection completed
/// (vtable +0x30); a failure there releases and clears the connector,
/// breaks the connection and is returned. Returns 0.
pub fn fn_009fc190(e: &mut Engine, this: Ptr, connector: Ptr, media_type: Ptr) -> u32 {
    if connector.is_null() || media_type.is_null() {
        return E_POINTER;
    }
    let section = field(e, this, 0x14);
    enter(e, section);
    if field(e, this, 0xc) != 0 {
        leave(e, section);
        return VFW_E_ALREADY_CONNECTED;
    }
    let filter = field(e, this, 0x1c);
    if e.mem.u32(filter.wrapping_add(0x14)) != 0 && byte_field(e, this, 0x19) == 0 {
        leave(e, section);
        return VFW_E_NOT_STOPPED;
    }
    let pin = back(this, 0xc).addr();
    let hr = e.vcall(pin, 0x28, &args![connector]).u32();
    if failed(hr) {
        e.vcall(pin, 0x2c, &args![]);
        leave(e, section);
        return hr;
    }
    let mut hr = e.vcall(pin, 0x20, &args![media_type]).u32();
    if hr != 0 {
        e.vcall(pin, 0x2c, &args![]);
        if !failed(hr) || hr == E_FAIL || hr == E_INVALIDARG {
            hr = VFW_E_TYPE_NOT_ACCEPTED;
        }
        leave(e, section);
        return hr;
    }
    set_field(e, this, 0xc, connector.addr());
    e.vcall(connector.addr(), 0x4, &args![]);
    let mut hr = e.vcall(pin, 0x24, &args![media_type]).u32();
    if !failed(hr) {
        hr = e.vcall(pin, 0x30, &args![connector]).u32();
        if !failed(hr) {
            leave(e, section);
            return 0;
        }
    }
    let held = field(e, this, 0xc);
    e.vcall(held, 0x8, &args![]);
    set_field(e, this, 0xc, 0);
    e.vcall(pin, 0x2c, &args![]);
    leave(e, section);
    hr
}

// Translated from 009fc390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fc390 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x1c of the `CAsyncOutputPin` vtable at `01093d9c` (stdcall,
/// `RET 8`): `E_POINTER` for a null `out`; otherwise, under the critical
/// section held at +0x14, copies the media type at +0x28 to `out`
/// (`009f9bb0`, its result ignored) and returns 0 while connected (+0xc),
/// or clears `out` (`009f9a30`) and returns `VFW_E_NOT_CONNECTED`.
pub fn fn_009fc390(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let section = field(e, this, 0x14);
    enter(e, section);
    if field(e, this, 0xc) != 0 {
        fn_009f9bb0(e, out, offset(this, 0x28));
        leave(e, section);
        return 0;
    }
    fn_009f9a30(e, out);
    leave(e, section);
    VFW_E_NOT_CONNECTED
}

// Translated from 009fc460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x54 of the `CMediaSample` vtable (thiscall, `RET 4`): the
/// destructor (`009fa560`), then `operator delete` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_009fc460(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009fa560(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009fc480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fc480 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x20 of the `CMemAllocator` vtable at `01093cf8` (stdcall,
/// `RET 8`): `E_POINTER` for a null `sample`. Under the lock the sample
/// goes to the front of the free list (+0x1c, link at the sample's +0x1c;
/// free count +0x20 up), waiters are woken (`009fae70`, when +0x28 is set)
/// and, with a decommit pending (+0x48) and every sample back (free count
/// equal to +0x30), the object (`this - 0xc`) frees (vtable +0x10,
/// thiscall) and the decommit is cleared. After the lock the notify object
/// at +0x4c is told (vtable +0xc) and, after such a free, a reference is
/// released (vtable +8 of `this`). Returns 0.
pub fn fn_009fc480(e: &mut Engine, this: Ptr, sample: Ptr) -> u32 {
    if sample.is_null() {
        return E_POINTER;
    }
    let lock = allocator_lock(this);
    enter(e, lock);
    let first = field(e, this, 0x1c);
    set_field(e, sample, 0x1c, first);
    set_field(e, this, 0x1c, sample.addr());
    let free = field(e, this, 0x20).wrapping_add(1);
    set_field(e, this, 0x20, free);
    if field(e, this, 0x28) != 0 {
        fn_009fae70(e, back(this, 0xc));
    }
    let free = field(e, this, 0x20);
    let release = if field(e, this, 0x48) != 0 && free == field(e, this, 0x30) {
        e.vcall(back(this, 0xc).addr(), 0x10, &args![]);
        set_field(e, this, 0x48, 0);
        true
    } else {
        false
    };
    leave(e, lock);
    let notify = field(e, this, 0x4c);
    if notify != 0 {
        e.vcall(notify, 0xc, &args![]);
    }
    if release {
        e.vcall(this.addr(), 0x8, &args![]);
    }
    0
}

// Translated from 009fc580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0xc of the `CMemAllocator` vtable at `01093d28` (thiscall,
/// `RET 4`): the destructor (`009fb1c0`), then `operator delete` when bit
/// 0 of `flags` is set. Returns `this`.
pub fn fn_009fc580(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009fb1c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009fc6d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fc6d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x18 of the `CEnumPins` vtable (stdcall, `RET 8`): `E_POINTER`
/// for a null `out`; `VFW_E_ENUM_OUT_OF_SYNC` (with `out` cleared) when the
/// filter's version (+0xc, vtable +0x14) is not the one at +0x10;
/// otherwise a new 0x30-byte enumerator made from this one (`009fc600`
/// with the filter and `this`) goes to `out`: 0, or `E_OUTOFMEMORY` when
/// the allocation failed.
pub fn fn_009fc6d0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let filter = field(e, this, 0xc);
    if e.vcall(filter, 0x14, &args![]).u32() != field(e, this, 0x10) {
        e.mem.set_u32(out.addr(), 0);
        return VFW_E_ENUM_OUT_OF_SYNC;
    }
    let block = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
    let made = if block != 0 {
        let filter = field(e, this, 0xc);
        fn_009fc600(e, Ptr::new(block), Ptr::new(filter), this).addr()
    } else {
        0
    };
    e.mem.set_u32(out.addr(), made);
    if made == 0 {
        E_OUTOFMEMORY
    } else {
        0
    }
}

// Translated from 009fc7e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fc7e0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x18 of the `CEnumMediaTypes` vtable (stdcall, `RET 8`):
/// `E_POINTER` for a null `out`; `VFW_E_ENUM_OUT_OF_SYNC` (with `out`
/// cleared) when the pin's version (+8, vtable +0x10) is not the one at
/// +0xc; otherwise a new 0x14-byte enumerator made from this one
/// (`009fc780` with the pin and `this`) goes to `out`: 0, or
/// `E_OUTOFMEMORY` when the allocation failed.
pub fn fn_009fc7e0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    let pin = field(e, this, 0x8);
    if e.vcall(pin, 0x10, &args![]).u32() != field(e, this, 0xc) {
        e.mem.set_u32(out.addr(), 0);
        return VFW_E_ENUM_OUT_OF_SYNC;
    }
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let made = if block != 0 {
        let pin = field(e, this, 0x8);
        fn_009fc780(e, Ptr::new(block), Ptr::new(pin), this).addr()
    } else {
        0
    };
    e.mem.set_u32(out.addr(), made);
    if made == 0 {
        E_OUTOFMEMORY
    } else {
        0
    }
}

// Translated from 009fcb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CMediaSample` constructor shape (thiscall, `RET 0x14`; RTTI of the
/// vtable it installs; the first and third words are not read): the buffer
/// at +0xc, its size at +0x10 and +0x14, the allocator at +0x18, and +4,
/// +8, +0x3c, +0x40, +0x44 cleared. Returns `this`.
pub fn fn_009fcb50(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    allocator: Ptr,
    _unused_3: u32,
    buffer: u32,
    length: u32,
) -> Ptr {
    set_field(e, this, 0xc, buffer);
    set_field(e, this, 0x10, length);
    set_field(e, this, 0x14, length);
    set_field(e, this, 0x0, MEDIA_SAMPLE_VTABLE);
    set_field(e, this, 0x4, 0);
    set_field(e, this, 0x8, 0);
    set_field(e, this, 0x18, allocator.addr());
    set_field(e, this, 0x3c, 0);
    set_field(e, this, 0x40, 0);
    set_field(e, this, 0x44, 0);
    this
}

// Translated from 009fcb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocator base constructor shape (thiscall, `RET 0x14`; called by
/// `009fcc20`): the reference-counted base (`009fcee0` with `name` and
/// `outer`), `InitializeCriticalSection` at +0x10, +0x28 to +0x58 cleared
/// and `callback` at +0x5c. With `semaphore` set a semaphore (count 0,
/// maximum 0x7fffffff, unnamed; `CreateSemaphoreA`) goes to +0x30, and
/// `E_OUTOFMEMORY` to `*status` when it is 0. Returns `this`.
pub fn fn_009fcb90(
    e: &mut Engine,
    this: Ptr,
    name: Ptr,
    outer: Ptr,
    status: Ptr,
    semaphore: u32,
    callback: u32,
) -> Ptr {
    fn_009fcee0(e, this, name, outer);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![offset(this, 0x10)]);
    for at in (0x28u32..=0x58).step_by(4) {
        set_field(e, this, at, 0);
    }
    set_field(e, this, 0x5c, callback);
    if semaphore != 0 {
        let handle = e
            .call(CREATE_SEMAPHORE_A, &args![0u32, 0u32, 0x7fff_ffffu32, 0u32])
            .u32();
        set_field(e, this, 0x30, handle);
        if handle == 0 {
            e.mem.set_u32(status.addr(), E_OUTOFMEMORY);
        }
    }
    this
}

// Translated from 009fcc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CMemAllocator` constructor shape (thiscall, `RET 0xc`; called by
/// `009fd5a0`; RTTI of the vtables it installs at +0 and +0xc): the
/// allocator base (`009fcb90` with `name`, `outer`, `status`, 1, 1) and the
/// block at +0x60 cleared. Returns `this`.
pub fn fn_009fcc20(e: &mut Engine, this: Ptr, name: Ptr, outer: Ptr, status: Ptr) -> Ptr {
    fn_009fcb90(e, this, name, outer, status, 1, 1);
    set_field(e, this, 0x0, MEM_ALLOCATOR_VTABLE);
    set_field(e, this, 0xc, MEM_ALLOCATOR_VTABLE_C);
    set_field(e, this, 0x60, 0);
    this
}

// Translated from 009fcc60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fcc60 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Slot +0x14 of the `CMemAllocator` vtable at `01093d28` (thiscall, no
/// stack words): under the lock at +0x10 (0 for a null `this`) the size
/// check (`009fb020`) is returned when it fails, and 1 means nothing to do
/// (0 returned). Otherwise any old block is freed (`009fb170`, when +0x60
/// is set); the buffer size (+0x40) plus prefix (+0x48) is rounded up to an
/// alignment (+0x44) above 1 (signed `IDIV`); count (+0x38) times that is
/// committed read/write (`VirtualAlloc` 0, size, 0x1000, 4) to +0x60
/// (`E_OUTOFMEMORY` when 0); then, while the made count (+0x3c) is below
/// the count, a 0x48-byte sample (`operator new`, `E_OUTOFMEMORY` when it
/// fails) is constructed over the next stretch (`009fcb50`: no name, this
/// allocator, the address of the lock's stack slot, buffer + prefix, the
/// size) and put on the list at +0x28 (link at its +0x1c; +0x2c and +0x3c
/// up). +0x4c is cleared at the end. Returns 0.
pub fn fn_009fcc60(e: &mut Engine, this: Ptr) -> u32 {
    let lock = if !this.is_null() {
        this.addr().wrapping_add(0x10)
    } else {
        0
    };
    e.with_stack(4, |e, lock_slot| {
        e.mem.set_u32(lock_slot.addr(), lock);
        enter(e, lock);
        let hr = fn_009fb020(e, this);
        if failed(hr) {
            leave(e, lock);
            return hr;
        }
        if hr == 1 {
            leave(e, lock);
            return 0;
        }
        if field(e, this, 0x60) != 0 {
            fn_009fb170(e, this);
        }
        let mut size = field(e, this, 0x40).wrapping_add(field(e, this, 0x48));
        let alignment = field(e, this, 0x44);
        if alignment as i32 > 1 {
            let remainder = (size as i32).wrapping_rem(alignment as i32) as u32;
            if remainder != 0 {
                size = size.wrapping_add(alignment.wrapping_sub(remainder));
            }
        }
        let total = field(e, this, 0x38).wrapping_mul(size);
        let block = e
            .call(VIRTUAL_ALLOC, &args![0u32, total, 0x1000u32, 0x4u32])
            .u32();
        set_field(e, this, 0x60, block);
        if block == 0 {
            leave(e, lock);
            return E_OUTOFMEMORY;
        }
        let mut cursor = block;
        while (field(e, this, 0x3c) as i32) < field(e, this, 0x38) as i32 {
            let memory = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
            if memory == 0 {
                leave(e, lock);
                return E_OUTOFMEMORY;
            }
            let length = field(e, this, 0x40);
            let data = field(e, this, 0x48).wrapping_add(cursor);
            let sample =
                fn_009fcb50(e, Ptr::new(memory), 0, this, lock_slot.addr(), data, length).addr();
            if sample == 0 {
                leave(e, lock);
                return E_OUTOFMEMORY;
            }
            let first = field(e, this, 0x28);
            e.mem.set_u32(sample.wrapping_add(0x1c), first);
            set_field(e, this, 0x28, sample);
            let count = field(e, this, 0x2c).wrapping_add(1);
            set_field(e, this, 0x2c, count);
            let made = field(e, this, 0x3c).wrapping_add(1);
            set_field(e, this, 0x3c, made);
            cursor = cursor.wrapping_add(size);
        }
        set_field(e, this, 0x4c, 0);
        leave(e, lock);
        0
    })
}

// Translated from 009fd030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wide-string compare (stdcall, `RET 8`): 0 when both strings hold the
/// same characters up to the terminator, else the first differing pair's
/// difference (`first` - `second`, as unsigned 16-bit values).
pub fn fn_009fd030(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let mut a = first.addr();
    let mut b = second.addr();
    loop {
        let x = e.mem.u16(a);
        let y = e.mem.u16(b);
        if x != y {
            return i32::from(x) - i32::from(y);
        }
        a = a.wrapping_add(2);
        if x == 0 {
            return 0;
        }
        b = b.wrapping_add(2);
        if y == 0 {
            return 0;
        }
    }
}

// Translated from 009fd180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Generic list step (the caller's `ECX`, the list, is not read; `RET 4`):
/// 0 when the position at `*position` is 0; otherwise moves `*position` to
/// the node's next (+4) and returns the node's object (+8).
pub fn fn_009fd180(e: &mut Engine, position: Ptr) -> u32 {
    let node = e.mem.u32(position.addr());
    if node == 0 {
        return 0;
    }
    let next = e.mem.u32(node.wrapping_add(0x4));
    e.mem.set_u32(position.addr(), next);
    e.mem.u32(node.wrapping_add(0x8))
}

// Translated from 009fd1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Generic list object at a position (the caller's `ECX` is not read;
/// `RET 4`): the node's object (+8), or 0 for a null node.
pub fn fn_009fd1a0(e: &mut Engine, node: Ptr) -> u32 {
    if node.is_null() {
        return 0;
    }
    field(e, node, 0x8)
}

// Translated from 009fd2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Generic list append (thiscall, `RET 4`): walks `other` from its head
/// (`009fd0f0`, the position kept in the stack slot of `other`) and adds
/// each object at this list's tail (`009fd270`). Returns 1, or 0 as soon as
/// an add fails.
pub fn fn_009fd2d0(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    let head = e.call(HEAD_POSITION, &args![other]).u32();
    if head == 0 {
        return 1;
    }
    e.with_stack(4, |e, position| {
        e.mem.set_u32(position.addr(), head);
        loop {
            let object = fn_009fd180(e, position);
            if fn_009fd270(e, this, Ptr::new(object)).is_null() {
                return 0;
            }
            if e.mem.u32(position.addr()) == 0 {
                return 1;
            }
        }
    })
}

// Translated from 009fd5a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fd5a0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Allocator factory (stdcall, `RET 4`; called by `009fd6d0`): `E_POINTER`
/// for a null `out`; otherwise clears `out`, constructs a 0x64-byte
/// `CMemAllocator` (`009fcc20`: no name, no outer, the status kept in the
/// stack slot of `out`, 0 at first); `E_OUTOFMEMORY` when that is 0. A
/// failed status deletes it again (vtable +0xc with 1) and is returned;
/// otherwise its interface at +0xc is asked for the id at `01093bbc` into
/// `out` (vtable +0), and a failure deletes it and returns
/// `E_NOINTERFACE`. Returns 0.
pub fn fn_009fd5a0(e: &mut Engine, out: Ptr) -> u32 {
    if out.is_null() {
        return E_POINTER;
    }
    e.with_stack(4, |e, status| {
        e.mem.set_u32(status.addr(), 0);
        e.mem.set_u32(out.addr(), 0);
        let memory = e.call(OPERATOR_NEW, &args![0x64u32]).u32();
        let allocator = if memory != 0 {
            fn_009fcc20(e, Ptr::new(memory), Ptr::NULL, Ptr::NULL, status).addr()
        } else {
            0
        };
        if allocator == 0 {
            return E_OUTOFMEMORY;
        }
        if failed(e.mem.u32(status.addr())) {
            e.vcall(allocator, 0xc, &args![1u32]);
            return e.mem.u32(status.addr());
        }
        let interface = allocator.wrapping_add(0xc);
        let hr = e
            .vcall(interface, 0x0, &args![INTERFACE_ID_01093BBC, out])
            .u32();
        e.mem.set_u32(status.addr(), hr);
        if failed(hr) {
            e.vcall(allocator, 0xc, &args![1u32]);
            return E_NOINTERFACE;
        }
        0
    })
}

// Translated from 009fd7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x10 of the `CAsyncOutputPin` vtable at `01093e24` (stdcall,
/// `RET 0xc`): queues a read for `sample`. `E_POINTER` for a null
/// `sample`. Its times (vtable +0x14, start and end, 100 ns units) must be
/// read; start and length are taken in seconds (`__alldiv` by 10,000,000,
/// the length truncated to its low word). When start + length passes the
/// reader's total (`009fe090` on the reader at `this + 0xa4`), the total
/// is rounded up to the reader's alignment (`009fe1a0`); if the end still
/// passes it, the length becomes total - start and the end time total *
/// 10,000,000 (`__allmul`), set on the sample (vtable +0x18). Then the
/// sample's buffer (vtable +0xc) must be read, and the request goes to
/// `009fe960` (start, length, aligned 1, buffer, `sample`, `user`).
/// Returns its result, or the first failure.
pub fn fn_009fd7f0(e: &mut Engine, this: Ptr, sample: Ptr, user: u32) -> u32 {
    if sample.is_null() {
        return E_POINTER;
    }
    // The frame mirrors the exe's: +0 alignment, +4 buffer, +8 total
    // (8 bytes), +0x18 end time, +0x20 start time, +0x28 the second word
    // `009fe090` gets (8 bytes).
    e.with_stack(0x30, |e, frame| {
        let f = frame.addr();
        let alignment = f;
        let buffer = f + 0x4;
        let total = f + 0x8;
        let end = f + 0x18;
        let start = f + 0x20;
        let other = f + 0x28;
        let s = sample.addr();
        let hr = e.vcall(s, 0x14, &args![start, end]).u32();
        if failed(hr) {
            return hr;
        }
        let start_time = quad(e, start);
        let first = e.call(ALLDIV, &args![start_time, TIME_UNITS, 0u32]).u64();
        let span_time = quad(e, end).wrapping_sub(start_time);
        let span = e.call(ALLDIV, &args![span_time, TIME_UNITS, 0u32]).u64();
        let mut span_low = span as u32;
        e.mem.set_u64(total, 0);
        e.mem.set_u64(other, 0);
        let reader = Ptr::new(field(e, this, 0xa4));
        fn_009fe090(e, reader, Ptr::new(total), Ptr::new(other));
        // CDQ of the low word, ADD/ADC with the first second.
        let finish = i64::from(span_low as i32).wrapping_add(first as i64);
        if finish > quad(e, total) as i64 {
            let reader = Ptr::new(field(e, this, 0xa4));
            fn_009fe1a0(e, reader, Ptr::new(alignment));
            let unit = e.mem.u32(alignment);
            let rounded = (quad(e, total) as i64)
                .wrapping_add(i64::from(unit as i32))
                .wrapping_sub(1)
                & i64::from(!unit.wrapping_sub(1) as i32);
            e.mem.set_u64(total, rounded as u64);
            if finish > rounded {
                span_low = (rounded as u32).wrapping_sub(first as u32);
                let end_time = e
                    .call(ALLMUL, &args![rounded as u64, TIME_UNITS, 0u32])
                    .u64();
                e.mem.set_u64(end, end_time);
                e.vcall(s, 0x18, &args![start, end]);
            }
        }
        let hr = e.vcall(s, 0xc, &args![buffer]).u32();
        if failed(hr) {
            return hr;
        }
        let data = e.mem.u32(buffer);
        let reader = Ptr::new(field(e, this, 0xa4));
        fn_009fe960(
            e,
            reader,
            first as u32,
            (first >> 32) as u32,
            span_low,
            1,
            data,
            s,
            user,
        )
    })
}

// Translated from 009fdcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0 of the `CAsyncOutputPin` vtable at `01093d84`: moves its first
/// stack word back by 0x18 (`SUB [ESP+4],0x18`) and jumps to `009fd3f0`
/// with the other two words.
pub fn fn_009fdcf0(e: &mut Engine, this: Ptr, id: Ptr, out: Ptr) -> u32 {
    fn_009fd3f0(e, back(this, 0x18), id, out)
}

// Translated from 009fdd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x40 of the `CAsyncOutputPin` vtable at `01093d9c`: moves its
/// stack word back by 0x14 and jumps to `009fdbd0`.
pub fn fn_009fdd00(e: &mut Engine, this: Ptr) -> u32 {
    fn_009fdbd0(e, back(this, 0x14))
}

// Translated from 009fdd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +8 of the `CAsyncOutputPin` vtable at `01093d84`: moves its stack
/// word back by 0x18 and jumps to `009fd430`.
pub fn fn_009fdd20(e: &mut Engine, this: Ptr) -> u32 {
    fn_009fd430(e, back(this, 0x18))
}

// Translated from 009fdd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x3c of the `CAsyncOutputPin` vtable at `01093d9c`: moves its
/// stack word back by 0x14 and jumps to `009fdbb0`.
pub fn fn_009fdd30(e: &mut Engine, this: Ptr) -> u32 {
    fn_009fdbb0(e, back(this, 0x14))
}

// Translated from 009fdd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +4 of the `CAsyncOutputPin` vtable at `01093d84`: moves its stack
/// word back by 0x18 and jumps to `009fd410`.
pub fn fn_009fdd60(e: &mut Engine, this: Ptr) -> u32 {
    fn_009fd410(e, back(this, 0x18))
}

// Translated from 009fddd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CAsyncOutputPin` deleting destructor (thiscall, `RET 4`; reached
/// through `009fdd50`): the destructor (`009fd3c0`), then `operator delete`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn fn_009fddd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009fd3c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009fdeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0xc of the `CAsyncReader` vtable at `01093eac` (thiscall,
/// `RET 4`): the destructor (`009fdc10`), then `operator delete` when bit
/// 0 of `flags` is set. Returns `this`.
pub fn fn_009fdeb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009fdc10(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009fe3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finished-request queue add (thiscall, `RET 4`; called by `009fe420` and
/// `009fe690`): puts `request` at the tail of the list at +0x50
/// (`009fd270`) and signals the event at +0x6c (`SetEvent`), returning 0;
/// `E_OUTOFMEMORY` when the add fails.
pub fn fn_009fe3f0(e: &mut Engine, this: Ptr, request: Ptr) -> u32 {
    if fn_009fd270(e, offset(this, 0x50), request).is_null() {
        return E_OUTOFMEMORY;
    }
    let event = field(e, this, 0x6c);
    e.call(SET_EVENT, &args![event]);
    0
}

// Translated from 009fe420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fe420 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Request loop (thiscall, no stack words; called by `009fe500`): under the
/// critical section at +0x1c takes each queued request (`009fe2f0`) until
/// none is left; for each the busy count at +0x70 goes up, the request runs
/// outside the lock (`009fdf20`), then back under the lock it is queued as
/// finished (`009fe3f0`), the busy count goes down and, at zero with a
/// waiter flagged at +0x74, the event at +0x78 is signalled (`SetEvent`).
pub fn fn_009fe420(e: &mut Engine, this: Ptr) {
    let section = this.addr().wrapping_add(0x1c);
    enter(e, section);
    let mut request = fn_009fe2f0(e, this);
    while request != 0 {
        let busy = field(e, this, 0x70).wrapping_add(1);
        set_field(e, this, 0x70, busy);
        leave(e, section);
        fn_009fdf20(e, Ptr::new(request));
        enter(e, section);
        fn_009fe3f0(e, this, Ptr::new(request));
        let busy = field(e, this, 0x70).wrapping_sub(1);
        set_field(e, this, 0x70, busy);
        if busy == 0 && field(e, this, 0x74) != 0 {
            let event = field(e, this, 0x78);
            e.call(SET_EVENT, &args![event]);
        }
        leave(e, section);
        enter(e, section);
        request = fn_009fe2f0(e, this);
    }
    leave(e, section);
}

// Translated from 009fe500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reader thread body (thiscall, no stack words; called by `009fe560`):
/// waits on the two handles at +0x7c and +0x68 (`WaitForMultipleObjects`,
/// any, no timeout) and runs the request loop (`009fe420`) each time the
/// second one is signalled (result 1). Returns 0 on any other result.
pub fn fn_009fe500(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(8, |e, handles| {
        let stop = field(e, this, 0x7c);
        let work = field(e, this, 0x68);
        e.mem.set_u32(handles.addr(), stop);
        e.mem.set_u32(handles.addr().wrapping_add(4), work);
        while e
            .call(
                WAIT_FOR_MULTIPLE_OBJECTS,
                &args![2u32, handles, 0u32, 0xffff_ffffu32],
            )
            .u32()
            == 1
        {
            fn_009fe420(e, this);
        }
    });
    0
}

// Translated from 009fe560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The thread routine `009fe780` passes to `CreateThread` (no Ghidra
/// function; bytes `8b 4c 24 04 e8 ..`: stdcall, `RET 4`): the reader
/// thread body (`009fe500`) on its parameter, whose result it returns.
pub fn fn_009fe560(e: &mut Engine, this: Ptr) -> u32 {
    fn_009fe500(e, this)
}

// Translated from 009fe780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reader thread start (thiscall, no stack words; called by `009fe7e0`): 0
/// when a thread is held at +0x80; otherwise resets the event at +0x7c
/// (`ResetEvent`) and starts `009fe560` on `this` (`CreateThread`: no
/// attributes, default stack, flags 0, the id into a stack slot) into
/// +0x80. A failed start returns `GetLastError` as an HRESULT
/// (`0x80070000 | error & 0xffff`) when it is positive, else as is.
pub fn fn_009fe780(e: &mut Engine, this: Ptr) -> u32 {
    if field(e, this, 0x80) != 0 {
        return 0;
    }
    let event = field(e, this, 0x7c);
    e.call(RESET_EVENT, &args![event]);
    e.with_stack(4, |e, thread_id| {
        e.mem.set_u32(thread_id.addr(), this.addr());
        let thread = e
            .call(
                CREATE_THREAD,
                &args![0u32, 0u32, THREAD_ROUTINE, this, 0u32, thread_id],
            )
            .u32();
        set_field(e, this, 0x80, thread);
        if thread != 0 {
            return 0;
        }
        let error = e.call(GET_LAST_ERROR, &args![]).u32();
        if error as i32 > 0 {
            (error & 0xffff) | 0x8007_0000
        } else {
            error
        }
    })
}

// Translated from 009fe7e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009fe7e0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Request queue add (thiscall, `RET 4`; called by `009fe960`): under the
/// critical section at +0x1c, `VFW_E_WRONG_STATE` while +0x34 is set;
/// otherwise puts `request` at the tail of the list at +0x38 (`009fd270`,
/// `E_OUTOFMEMORY` when that fails), signals the event at +0x68
/// (`SetEvent`) and returns the thread start's result (`009fe780`).
pub fn fn_009fe7e0(e: &mut Engine, this: Ptr, request: Ptr) -> u32 {
    let section = this.addr().wrapping_add(0x1c);
    enter(e, section);
    let hr = if field(e, this, 0x34) != 0 {
        VFW_E_WRONG_STATE
    } else if !fn_009fd270(e, offset(this, 0x38), request).is_null() {
        let event = field(e, this, 0x68);
        e.call(SET_EVENT, &args![event]);
        fn_009fe780(e, this)
    } else {
        E_OUTOFMEMORY
    };
    leave(e, section);
    hr
}

// Translated from 009fe960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Request (thiscall, `RET 0x1c`; called by `009fd7f0`): with `aligned`
/// set, the position's low word, `length` and `buffer` must each be a
/// multiple of the file's alignment (the object at +0x18, vtable +0x10,
/// asked each time; `VFW_E_BADALIGN`). A 0x28-byte request (`operator
/// new`, `E_OUTOFMEMORY` when it fails) is filled (`009fded0`: `this`, the
/// file, the seven words) and queued (`009fe7e0`); a failure of either
/// deletes it and is returned. Returns the queue's result.
pub fn fn_009fe960(
    e: &mut Engine,
    this: Ptr,
    position_low: u32,
    position_high: u32,
    length: u32,
    aligned: u32,
    buffer: u32,
    sample: u32,
    user: u32,
) -> u32 {
    if aligned != 0 {
        for value in [position_low, length, buffer] {
            let file = field(e, this, 0x18);
            let unit = e.vcall(file, 0x10, &args![]).u32();
            if value & unit.wrapping_sub(1) != 0 {
                return VFW_E_BADALIGN;
            }
        }
    }
    let block = e.call(OPERATOR_NEW, &args![0x28u32]).u32();
    if block == 0 {
        return E_OUTOFMEMORY;
    }
    let file = field(e, this, 0x18);
    let mut hr = fn_009fded0(
        e,
        Ptr::new(block),
        this.addr(),
        file,
        position_low,
        position_high,
        length,
        aligned,
        buffer,
        sample,
        user,
    );
    if !failed(hr) {
        hr = fn_009fe7e0(e, this, Ptr::new(block));
        if !failed(hr) {
            return hr;
        }
    }
    e.call(OPERATOR_DELETE, &args![block]);
    hr
}

// Translated from 009fea24 (decompiled, FalloutNV.exe 1.4.0.525)
/// The stub `009fea2a`'s table cells hold until they are resolved (the
/// jumps `009fead0` and the others reach it; `RET`, so the caller's words
/// stay): returns the word at `011a6224` (`009fea2a`'s first cell).
pub fn fn_009fea24(e: &mut Engine) -> u32 {
    e.global::<u32>(LIBRARY_FIRST_CELL)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_9d50, fn_009f9d50(Ptr, Ptr) -> u32),
        entry!(0x009f_dd50, fn_009fdd50(Ptr, u32) -> u32),
        entry!(0x009f_9968, fn_009f9968(u32)),
        entry!(0x009f_ead0, fn_009fead0(u32, Ptr) -> u32),
        entry!(0x009f_ead6, fn_009fead6(u32, Ptr) -> u32),
        entry!(0x009f_eae2, fn_009feae2(Ptr, Ptr) -> u32),
        entry!(0x009f_eae8, fn_009feae8(Ptr, Ptr) -> u32),
        entry!(0x009f_996e, fn_009f996e(u32, Ptr) -> u32),
        entry!(0x009f_9980, fn_009f9980(Ptr, Ptr) -> u32),
        entry!(0x009f_9a30, fn_009f9a30(Ptr)),
        entry!(0x009f_9bb0, fn_009f9bb0(Ptr, Ptr) -> u32),
        entry!(0x009f_9c20, fn_009f9c20(Ptr)),
        entry!(0x009f_9c60, fn_009f9c60(Ptr)),
        entry!(0x009f_9c70, fn_009f9c70(Ptr) -> Ptr),
        entry!(0x009f_9c80, fn_009f9c80(Ptr, Ptr) -> u32),
        entry!(0x009f_9cb0, fn_009f9cb0(Ptr)),
        entry!(0x009f_9d10, fn_009f9d10(Ptr, Ptr) -> Ptr),
        entry!(0x009f_9d30, fn_009f9d30(Ptr, Ptr) -> Ptr),
        entry!(0x009f_9d60, fn_009f9d60(Ptr, Ptr) -> u32),
        entry!(0x009f_9d90, fn_009f9d90(Ptr, u32, Ptr) -> u32),
        entry!(0x009f_9df0, fn_009f9df0(Ptr, Ptr) -> u32),
        entry!(0x009f_9e50, fn_009f9e50(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_9f70, fn_009f9f70(Ptr)),
        entry!(0x009f_a060, fn_009fa060(Ptr) -> u32),
        entry!(0x009f_a100, fn_009fa100(Ptr)),
        entry!(0x009f_a1a0, fn_009fa1a0(Ptr) -> u32),
        entry!(0x009f_a1f0, fn_009fa1f0(Ptr)),
        entry!(0x009f_a260, fn_009fa260(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_a2f0, fn_009fa2f0(Ptr) -> u32),
        entry!(0x009f_a310, fn_009fa310(Ptr) -> u32),
        entry!(0x009f_a330, fn_009fa330(Ptr, Ptr) -> u32),
        entry!(0x009f_a350, fn_009fa350(Ptr, Ptr) -> u32),
        entry!(0x009f_a380, fn_009fa380(Ptr) -> u32),
        entry!(0x009f_a3c0, fn_009fa3c0(Ptr, Ptr) -> u32),
        entry!(0x009f_a400, fn_009fa400(Ptr, Ptr) -> u32),
        entry!(0x009f_a480, fn_009fa480(Ptr, Ptr) -> u32),
        entry!(0x009f_a4d0, fn_009fa4d0(Ptr) -> u32),
        entry!(0x009f_aba0, fn_009faba0(u32, u32, u32) -> u32),
        entry!(0x009f_b480, fn_009fb480(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_b550, fn_009fb550(Ptr)),
        entry!(0x009f_b5d0, fn_009fb5d0(Ptr, Ptr) -> u32),
        entry!(0x009f_b6f0, fn_009fb6f0(Ptr) -> u32),
        entry!(0x009f_b7a0, fn_009fb7a0(Ptr) -> u32),
        entry!(0x009f_b860, fn_009fb860(Ptr, u64) -> u32),
        entry!(0x009f_bb30, fn_009fbb30(Ptr, u32) -> Ptr),
        entry!(0x009f_bb50, fn_009fbb50(Ptr, u32, Ptr, Ptr) -> u32),
        entry!(0x009f_bca0, fn_009fbca0(Ptr, u32) -> Ptr),
        entry!(0x009f_bcc0, fn_009fbcc0(Ptr, u32, Ptr, Ptr) -> u32),
        entry!(0x009f_bf00, fn_009fbf00(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_bfc0, fn_009fbfc0(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_c0b0, fn_009fc0b0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_c300, fn_009fc300(Ptr) -> u32),
        entry!(0x009f_c450, fn_009fc450(Ptr, Ptr) -> u32),
        entry!(0x009f_c5a0, fn_009fc5a0(Ptr, Ptr, Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(0x009f_c600, fn_009fc600(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x009f_c780, fn_009fc780(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x009f_c890, fn_009fc890(Ptr, Ptr, Ptr, Ptr, u32, Ptr, u32) -> Ptr),
        entry!(0x009f_c990, fn_009fc990(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_cab0, fn_009fcab0(Ptr, Ptr) -> u32),
        entry!(0x009f_cdc0, fn_009fcdc0(Ptr, Ptr) -> u32),
        entry!(0x009f_ce60, fn_009fce60(Ptr, u32) -> Ptr),
        entry!(0x009f_ce80, fn_009fce80()),
        entry!(0x009f_ceb0, fn_009fceb0(Ptr, Ptr) -> u32),
        entry!(0x009f_cee0, fn_009fcee0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x009f_cf10, fn_009fcf10(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_cf60, fn_009fcf60(Ptr) -> u32),
        entry!(0x009f_cf80, fn_009fcf80(Ptr) -> u32),
        entry!(0x009f_cfc0, fn_009fcfc0(Ptr, u32) -> Ptr),
        entry!(0x009f_cfe0, fn_009fcfe0(Ptr)),
        entry!(0x009f_cff0, fn_009fcff0(Ptr, Ptr, u32) -> Ptr),
        entry!(0x009f_d080, fn_009fd080(Ptr) -> u32),
        entry!(0x009f_d0a0, fn_009fd0a0(Ptr, Ptr) -> u32),
        entry!(0x009f_d120, fn_009fd120(Ptr, u32) -> Ptr),
        entry!(0x009f_d140, fn_009fd140(Ptr)),
        entry!(0x009f_d1c0, fn_009fd1c0(Ptr, Ptr) -> Ptr),
        entry!(0x009f_d200, fn_009fd200(Ptr, Ptr) -> u32),
        entry!(0x009f_d270, fn_009fd270(Ptr, Ptr) -> Ptr),
        entry!(0x009f_d320, fn_009fd320(Ptr)),
        entry!(0x009f_d390, fn_009fd390(Ptr) -> u32),
        entry!(0x009f_d3a0, fn_009fd3a0(Ptr, u32) -> u32),
        entry!(0x009f_d3c0, fn_009fd3c0(Ptr)),
        entry!(0x009f_d3f0, fn_009fd3f0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_d410, fn_009fd410(Ptr) -> u32),
        entry!(0x009f_d430, fn_009fd430(Ptr) -> u32),
        entry!(0x009f_d450, fn_009fd450(Ptr, Ptr) -> u32),
        entry!(0x009f_d460, fn_009fd460(Ptr, u32) -> u32),
        entry!(0x009f_d480, fn_009fd480(Ptr) -> u32),
        entry!(0x009f_d490, fn_009fd490(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_d4e0, fn_009fd4e0(Ptr, i32, Ptr) -> u32),
        entry!(0x009f_d530, fn_009fd530(Ptr, Ptr) -> u32),
        entry!(0x009f_d6d0, fn_009fd6d0(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_d970, fn_009fd970(Ptr, Ptr) -> u32),
        entry!(0x009f_daf0, fn_009fdaf0(Ptr, u32, Ptr, Ptr) -> u32),
        entry!(0x009f_db60, fn_009fdb60(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x009f_db90, fn_009fdb90(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_dbb0, fn_009fdbb0(Ptr) -> u32),
        entry!(0x009f_dbd0, fn_009fdbd0(Ptr) -> u32),
        entry!(0x009f_dbf0, fn_009fdbf0(Ptr, u32, u32) -> u32),
        entry!(0x009f_dc10, fn_009fdc10(Ptr)),
        entry!(0x009f_dcb0, fn_009fdcb0(Ptr, u32) -> Ptr),
        entry!(0x009f_dce0, fn_009fdce0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_dd10, fn_009fdd10(Ptr) -> u32),
        entry!(0x009f_dd40, fn_009fdd40(Ptr) -> u32),
        entry!(0x009f_dd70, fn_009fdd70(Ptr, u32, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x009f_ddf0, fn_009fddf0(Ptr, Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(0x009f_ded0, fn_009fded0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x009f_df20, fn_009fdf20(Ptr) -> u32),
        entry!(0x009f_dfe0, fn_009fdfe0(Ptr, u32, u32, u32, u32, Ptr, u32) -> u32),
        entry!(0x009f_e090, fn_009fe090(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_e0c0, fn_009fe0c0(Ptr) -> u32),
        entry!(0x009f_e100, fn_009fe100(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x009f_e1a0, fn_009fe1a0(Ptr, Ptr) -> u32),
        entry!(0x009f_e1d0, fn_009fe1d0(Ptr, Ptr) -> Ptr),
        entry!(0x009f_e2a0, fn_009fe2a0(Ptr) -> u32),
        entry!(0x009f_e2f0, fn_009fe2f0(Ptr) -> u32),
        entry!(0x009f_e370, fn_009fe370(Ptr) -> u32),
        entry!(0x009f_e570, fn_009fe570(Ptr, u32, Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_e690, fn_009fe690(Ptr) -> u32),
        entry!(0x009f_e870, fn_009fe870(Ptr)),
        entry!(0x009f_ea2a, fn_009fea2a() -> u32),
        entry!(0x009f_ebd8, fn_009febd8(u32, Ptr) -> u32),
        entry!(0x009f_98e0, fn_009f98e0(Ptr, u32)),
        entry!(0x009f_9a50, fn_009f9a50(Ptr) -> u32),
        entry!(0x009f_9a80, fn_009f9a80(Ptr, Ptr) -> u32),
        entry!(0x009f_9cd0, fn_009f9cd0(Ptr) -> Ptr),
        entry!(0x009f_9db0, fn_009f9db0(Ptr, Ptr) -> u32),
        entry!(0x009f_9f40, fn_009f9f40(u32, u32) -> u32),
        entry!(0x009f_9fe0, fn_009f9fe0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_a040, fn_009fa040(Ptr) -> u32),
        entry!(0x009f_a090, fn_009fa090(Ptr) -> u32),
        entry!(0x009f_a0d0, fn_009fa0d0(Ptr) -> u32),
        entry!(0x009f_a120, fn_009fa120(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_a180, fn_009fa180(Ptr) -> u32),
        entry!(0x009f_a1d0, fn_009fa1d0(Ptr) -> u32),
        entry!(0x009f_a4a0, fn_009fa4a0(Ptr, Ptr) -> u32),
        entry!(0x009f_a4e0, fn_009fa4e0(Ptr, u32) -> u32),
        entry!(0x009f_a510, fn_009fa510(u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x009f_a520, fn_009fa520(Ptr, u32, u32, u32, u32, f64) -> u32),
        entry!(0x009f_a560, fn_009fa560(Ptr)),
        entry!(0x009f_a580, fn_009fa580(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_a5d0, fn_009fa5d0(Ptr) -> u32),
        entry!(0x009f_a5f0, fn_009fa5f0(Ptr) -> u32),
        entry!(0x009f_a650, fn_009fa650(Ptr, Ptr) -> u32),
        entry!(0x009f_a670, fn_009fa670(Ptr) -> u32),
        entry!(0x009f_a680, fn_009fa680(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_a6f0, fn_009fa6f0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_a760, fn_009fa760(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_a7a0, fn_009fa7a0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_a7e0, fn_009fa7e0(Ptr) -> u32),
        entry!(0x009f_a800, fn_009fa800(Ptr, u32) -> u32),
        entry!(0x009f_a820, fn_009fa820(Ptr) -> u32),
        entry!(0x009f_a840, fn_009fa840(Ptr, u32) -> u32),
        entry!(0x009f_a860, fn_009fa860(Ptr) -> u32),
        entry!(0x009f_a880, fn_009fa880(Ptr, u32) -> u32),
        entry!(0x009f_a8a0, fn_009fa8a0(Ptr) -> u32),
        entry!(0x009f_a8b0, fn_009fa8b0(Ptr, i32) -> u32),
        entry!(0x009f_a8d0, fn_009fa8d0(Ptr, Ptr) -> u32),
        entry!(0x009f_a910, fn_009fa910(Ptr, Ptr) -> u32),
        entry!(0x009f_a960, fn_009fa960(Ptr, u32, Ptr) -> u32),
        entry!(0x009f_aa20, fn_009faa20(Ptr, u32, Ptr) -> u32),
        entry!(0x009f_abb0, fn_009fabb0(Ptr)),
        entry!(0x009f_ac30, fn_009fac30(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_aca0, fn_009faca0(Ptr, Ptr) -> u32),
        entry!(0x009f_ad00, fn_009fad00(Ptr, Ptr, u32, u32, u32) -> u32),
        entry!(0x009f_ada0, fn_009fada0(Ptr, Ptr) -> u32),
        entry!(0x009f_ae30, fn_009fae30(Ptr, Ptr) -> u32),
        entry!(0x009f_ae70, fn_009fae70(Ptr)),
        entry!(0x009f_ae90, fn_009fae90(Ptr) -> u32),
        entry!(0x009f_af50, fn_009faf50(Ptr) -> u32),
        entry!(0x009f_b020, fn_009fb020(Ptr) -> u32),
        entry!(0x009f_b050, fn_009fb050(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_b170, fn_009fb170(Ptr)),
        entry!(0x009f_b1c0, fn_009fb1c0(Ptr)),
        entry!(0x009f_b230, fn_009fb230(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_b250, fn_009fb250(Ptr) -> u32),
        entry!(0x009f_b270, fn_009fb270(Ptr) -> u32),
        entry!(0x009f_b290, fn_009fb290(Ptr, Ptr, u32) -> u32),
        entry!(0x009f_b650, fn_009fb650(Ptr, Ptr) -> u32),
        entry!(0x009f_b960, fn_009fb960(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_ba40, fn_009fba40(Ptr) -> u32),
        entry!(0x009f_bab0, fn_009fbab0(Ptr) -> u32),
        entry!(0x009f_bc50, fn_009fbc50(Ptr, u32) -> u32),
        entry!(0x009f_be40, fn_009fbe40(Ptr, u32) -> u32),
        entry!(0x009f_c190, fn_009fc190(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_c390, fn_009fc390(Ptr, Ptr) -> u32),
        entry!(0x009f_c460, fn_009fc460(Ptr, u32) -> Ptr),
        entry!(0x009f_c480, fn_009fc480(Ptr, Ptr) -> u32),
        entry!(0x009f_c580, fn_009fc580(Ptr, u32) -> Ptr),
        entry!(0x009f_c6d0, fn_009fc6d0(Ptr, Ptr) -> u32),
        entry!(0x009f_c7e0, fn_009fc7e0(Ptr, Ptr) -> u32),
        entry!(0x009f_cb50, fn_009fcb50(Ptr, u32, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x009f_cb90, fn_009fcb90(Ptr, Ptr, Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x009f_cc20, fn_009fcc20(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x009f_cc60, fn_009fcc60(Ptr) -> u32),
        entry!(0x009f_d030, fn_009fd030(Ptr, Ptr) -> i32),
        entry!(0x009f_d180, fn_009fd180(Ptr) -> u32),
        entry!(0x009f_d1a0, fn_009fd1a0(Ptr) -> u32),
        entry!(0x009f_d2d0, fn_009fd2d0(Ptr, Ptr) -> u32),
        entry!(0x009f_d5a0, fn_009fd5a0(Ptr) -> u32),
        entry!(0x009f_d7f0, fn_009fd7f0(Ptr, Ptr, u32) -> u32),
        entry!(0x009f_dcf0, fn_009fdcf0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x009f_dd00, fn_009fdd00(Ptr) -> u32),
        entry!(0x009f_dd20, fn_009fdd20(Ptr) -> u32),
        entry!(0x009f_dd30, fn_009fdd30(Ptr) -> u32),
        entry!(0x009f_dd60, fn_009fdd60(Ptr) -> u32),
        entry!(0x009f_ddd0, fn_009fddd0(Ptr, u32) -> Ptr),
        entry!(0x009f_deb0, fn_009fdeb0(Ptr, u32) -> Ptr),
        entry!(0x009f_e3f0, fn_009fe3f0(Ptr, Ptr) -> u32),
        entry!(0x009f_e420, fn_009fe420(Ptr)),
        entry!(0x009f_e500, fn_009fe500(Ptr) -> u32),
        entry!(0x009f_e560, fn_009fe560(Ptr) -> u32),
        entry!(0x009f_e780, fn_009fe780(Ptr) -> u32),
        entry!(0x009f_e7e0, fn_009fe7e0(Ptr, Ptr) -> u32),
        entry!(0x009f_e960, fn_009fe960(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x009f_ea24, fn_009fea24() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_009f9d50_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x009f_9d50;
        const TARGET: u32 = 0x009f_9c80;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 0x8007_000e,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(),
            0x8007_000e
        );
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_009fdd50() {
        assert_eq!(
            check(0x009f_dd50, 0x009f_ddd0, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod a06_thunk_tests {
    use super::*;

    /// Calls `thunk` with `words`, with a double over the import `slot` that
    /// returns `0xdeadbeef`; checks that the double ran once with exactly
    /// those words and that the thunk returned `expected`.
    fn import(thunk: u32, slot: u32, words: &[u32], expected: u32) {
        let mut e = Engine::new();
        e.register(slot, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, words);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == slot).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, words);
        assert_eq!(result.u32(), expected);
    }

    /// Calls `thunk` with `words`, with the data cell `cell` holding the
    /// address of a double that returns `0xdeadbeef`; checks that the double
    /// ran once with exactly those words and that the thunk returned `expected`.
    fn pointer(thunk: u32, cell: u32, words: &[u32], expected: u32) {
        const TARGET: u32 = 0x00aa_1b60;
        let mut e = Engine::new();
        e.map(cell & !0xfff, 0x1000);
        e.set_global(cell, TARGET);
        e.register(TARGET, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, words);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, words);
        assert_eq!(result.u32(), expected);
    }

    #[test]
    fn thunk_009f9968() {
        import(0x009f_9968, IMPORT_SLOT_009F9968, &[0x1001], 0);
    }

    #[test]
    fn thunk_009fead0() {
        pointer(
            0x009f_ead0,
            POINTER_CELL_009FEAD0,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_009fead6() {
        pointer(
            0x009f_ead6,
            POINTER_CELL_009FEAD6,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_009feae2() {
        pointer(
            0x009f_eae2,
            POINTER_CELL_009FEAE2,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_009feae8() {
        pointer(
            0x009f_eae8,
            POINTER_CELL_009FEAE8,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }
}

#[cfg(test)]
mod b009_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An engine with doubles over the CRT helpers and the imports these
    /// functions call, the call log on, the interface ids mapped (each id's
    /// four words are its address xor 0..3) and the object count mapped.
    fn rig() -> Engine {
        let mut e = Engine::new();
        for slot in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            INITIALIZE_CRITICAL_SECTION,
            DELETE_CRITICAL_SECTION,
            CO_TASK_MEM_FREE,
            OPERATOR_DELETE,
            FREE_LIBRARY,
            CLOSE_HANDLE,
            SET_EVENT,
            RESET_EVENT,
        ] {
            e.register(slot, noop);
        }
        e.register(CO_TASK_MEM_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            ret(a[0])
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            ret(v)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            ret(v)
        });
        e.map(0x0109_3000, 0x1000);
        for id in [
            INTERFACE_ID_01093B4C,
            INTERFACE_ID_01093B6C,
            INTERFACE_ID_01093B7C,
            INTERFACE_ID_01093B9C,
            INTERFACE_ID_01093BEC,
            INTERFACE_ID_01093BFC,
            INTERFACE_ID_01093C3C,
            INTERFACE_ID_01093F04,
            INTERFACE_ID_01093F14,
            INTERFACE_ID_01093F24,
            CLASS_ID_01093D70,
        ] {
            for i in 0..4 {
                e.mem.set_u32(id + 4 * i, id ^ i);
            }
        }
        e.map(0x011f_3000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A zeroed object of `size` bytes whose vtable slot at byte offset `s`
    /// (up to 0x7c) points at `base + s`.
    fn object(e: &mut Engine, base: u32, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for s in (0..0x80).step_by(4) {
            e.mem.set_u32(vtable + s, base + s);
        }
        let this = e.mem.alloc(size);
        e.mem.set_u32(this, vtable);
        this
    }

    /// A heap copy of the 16-byte id at `id`.
    fn id_copy(e: &mut Engine, id: u32) -> u32 {
        let copy = e.mem.alloc(0x10);
        for i in 0..4 {
            let w = e.mem.u32(id + 4 * i);
            e.mem.set_u32(copy + 4 * i, w);
        }
        copy
    }

    /// A heap wide string.
    fn wide(e: &mut Engine, text: &str) -> u32 {
        let units: Vec<u16> = text.encode_utf16().collect();
        let at = e.mem.alloc(2 * units.len() as u32 + 2);
        for (i, u) in units.iter().enumerate() {
            e.mem.set_u16(at + 2 * i as u32, *u);
        }
        at
    }

    fn read_wide(e: &Engine, at: u32) -> String {
        let mut units = vec![];
        let mut p = at;
        loop {
            let u = e.mem.u16(p);
            if u == 0 {
                break;
            }
            units.push(u);
            p += 2;
        }
        String::from_utf16(&units).unwrap()
    }

    const OBJ: u32 = 0x7000_0000;
    const OBJ2: u32 = 0x7000_1000;
    const OBJ3: u32 = 0x7000_2000;

    #[test]
    fn xinput_thunk_009f996e_forwards_both_words() {
        let mut e = rig();
        e.register(XINPUT_GET_STATE_SLOT, |_, _| ret(0x48f));
        assert_eq!(e.call(0x009f_996e, &args![1u32, 0x2000u32]).u32(), 0x48f);
        assert_eq!(calls(&e, XINPUT_GET_STATE_SLOT), vec![vec![1, 0x2000]]);
    }

    #[test]
    fn id_compare_009f9980_is_one_only_for_equal_ids() {
        let mut e = rig();
        let a = id_copy(&mut e, INTERFACE_ID_01093B4C);
        let b = id_copy(&mut e, INTERFACE_ID_01093B4C);
        assert_eq!(e.call(0x009f_9980, &args![a, b]).u32(), 1);
        e.mem.set_u8(b + 5, e.mem.u8(b + 5) ^ 0x10);
        assert_eq!(e.call(0x009f_9980, &args![a, b]).u32(), 0);
        let c = id_copy(&mut e, INTERFACE_ID_01093B4C);
        e.mem.set_u8(c + 15, e.mem.u8(c + 15).wrapping_sub(1));
        assert_eq!(e.call(0x009f_9980, &args![a, c]).u32(), 0);
        assert_eq!(e.call(0x009f_9980, &args![c, a]).u32(), 0);
    }

    #[test]
    fn media_type_clear_009f9a30_zeroes_and_sets_two_flags() {
        let mut e = rig();
        let mt = e.mem.alloc(0x48);
        for i in 0..0x12 {
            e.mem.set_u32(mt + 4 * i, 0xffff_ffff);
        }
        e.call(0x009f_9a30, &args![mt]);
        for i in 0..0x12 {
            let expected = if i == 8 || i == 10 { 1 } else { 0 };
            assert_eq!(e.mem.u32(mt + 4 * i), expected, "word {i}");
        }
        assert_eq!(calls(&e, MEMSET), vec![vec![mt, 0, 0x48]]);
    }

    #[test]
    fn media_type_copy_009f9bb0_duplicates_the_format_and_adds_a_reference() {
        let mut e = rig();
        e.register(OBJ + 4, noop);
        let unk = object(&mut e, OBJ, 8);
        let format = e.mem.alloc(4);
        e.mem.set_u32(format, 0xabcd_1234);
        let src = e.mem.alloc(0x48);
        e.mem.set_u32(src, 0x11);
        e.mem.set_u32(src + 0x3c, unk);
        e.mem.set_u32(src + 0x40, 4);
        e.mem.set_u32(src + 0x44, format);
        let dst = e.mem.alloc(0x48);
        assert_eq!(e.call(0x009f_9bb0, &args![dst, src]).u32(), 0);
        assert_eq!(e.mem.u32(dst), 0x11);
        let copy = e.mem.u32(dst + 0x44);
        assert_ne!(copy, format);
        assert_eq!(e.mem.u32(copy), 0xabcd_1234);
        assert_eq!(calls(&e, OBJ + 4), vec![vec![unk]]);
        // A failed allocation clears the size and reports out of memory.
        e.register(CO_TASK_MEM_ALLOC, |_, _| ret(0));
        let dst = e.mem.alloc(0x48);
        assert_eq!(e.call(0x009f_9bb0, &args![dst, src]).u32(), E_OUTOFMEMORY);
        assert_eq!(e.mem.u32(dst + 0x40), 0);
        assert_eq!(calls(&e, OBJ + 4).len(), 1);
    }

    #[test]
    fn media_type_free_009f9c20_frees_the_format_and_releases() {
        let mut e = rig();
        e.register(OBJ + 8, noop);
        let unk = object(&mut e, OBJ, 8);
        let mt = e.mem.alloc(0x48);
        e.mem.set_u32(mt + 0x3c, unk);
        e.mem.set_u32(mt + 0x40, 8);
        e.mem.set_u32(mt + 0x44, 0x5000);
        e.call(0x009f_9c20, &args![mt]);
        assert_eq!(calls(&e, CO_TASK_MEM_FREE), vec![vec![0x5000]]);
        assert_eq!(calls(&e, OBJ + 8), vec![vec![unk]]);
        assert_eq!(e.mem.u32(mt + 0x3c), 0);
        assert_eq!(e.mem.u32(mt + 0x40), 0);
        assert_eq!(e.mem.u32(mt + 0x44), 0);
        // Nothing held: nothing freed.
        e.call(0x009f_9c20, &args![mt]);
        assert_eq!(calls(&e, CO_TASK_MEM_FREE).len(), 1);
    }

    #[test]
    fn media_type_destructor_009f9c60_frees_the_contents() {
        let mut e = rig();
        let mt = e.mem.alloc(0x48);
        e.mem.set_u32(mt + 0x40, 8);
        e.mem.set_u32(mt + 0x44, 0x5000);
        e.call(0x009f_9c60, &args![mt]);
        assert_eq!(calls(&e, CO_TASK_MEM_FREE), vec![vec![0x5000]]);
    }

    #[test]
    fn media_type_constructor_009f9c70_returns_this() {
        let mut e = rig();
        let mt = e.mem.alloc(0x48);
        e.mem.set_u32(mt + 0x40, 9);
        assert_eq!(e.call(0x009f_9c70, &args![mt]).u32(), mt);
        assert_eq!(e.mem.u32(mt + 0x40), 0);
        assert_eq!(e.mem.u32(mt + 0x20), 1);
    }

    #[test]
    fn media_type_assign_009f9c80_skips_self_and_reports_failed_copies() {
        let mut e = rig();
        let a = e.mem.alloc(0x48);
        assert_eq!(e.call(0x009f_9c80, &args![a, a]).u32(), 0);
        assert!(calls(&e, CO_TASK_MEM_FREE).is_empty());
        let b = e.mem.alloc(0x48);
        e.mem.set_u32(b + 0x10, 0x77);
        assert_eq!(e.call(0x009f_9c80, &args![a, b]).u32(), 0);
        assert_eq!(e.mem.u32(a + 0x10), 0x77);
        e.mem.set_u32(b + 0x40, 4);
        let format = e.mem.alloc(4);
        e.mem.set_u32(b + 0x44, format);
        e.register(CO_TASK_MEM_ALLOC, |_, _| ret(0));
        assert_eq!(e.call(0x009f_9c80, &args![a, b]).u32(), E_OUTOFMEMORY);
    }

    #[test]
    fn media_type_delete_009f9cb0_frees_contents_then_block() {
        let mut e = rig();
        e.call(0x009f_9cb0, &args![0u32]);
        assert!(calls(&e, CO_TASK_MEM_FREE).is_empty());
        let mt = e.mem.alloc(0x48);
        e.mem.set_u32(mt + 0x40, 4);
        e.mem.set_u32(mt + 0x44, 0x6000);
        e.call(0x009f_9cb0, &args![mt]);
        assert_eq!(calls(&e, CO_TASK_MEM_FREE), vec![vec![0x6000], vec![mt]]);
    }

    #[test]
    fn media_type_assign_009f9d10_returns_this() {
        let mut e = rig();
        let a = e.mem.alloc(0x48);
        let b = e.mem.alloc(0x48);
        e.mem.set_u32(b + 4, 0x99);
        assert_eq!(e.call(0x009f_9d10, &args![a, b]).u32(), a);
        assert_eq!(e.mem.u32(a + 4), 0x99);
    }

    #[test]
    fn media_type_assign_009f9d30_returns_this() {
        let mut e = rig();
        let a = e.mem.alloc(0x48);
        let b = e.mem.alloc(0x48);
        e.mem.set_u32(b + 8, 0x98);
        assert_eq!(e.call(0x009f_9d30, &args![a, b]).u32(), a);
        assert_eq!(e.mem.u32(a + 8), 0x98);
    }

    #[test]
    fn copy_16_bytes_009f9d60() {
        let mut e = rig();
        let this = e.mem.alloc(0x30);
        for i in 0..4 {
            e.mem.set_u32(this + 0x1c + 4 * i, 0x10 + i);
        }
        assert_eq!(e.call(0x009f_9d60, &args![this, 0u32]).u32(), E_POINTER);
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x009f_9d60, &args![this, out]).u32(), 0);
        for i in 0..4 {
            assert_eq!(e.mem.u32(out + 4 * i), 0x10 + i);
        }
    }

    #[test]
    fn word_at_8_009f9d90() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x42);
        assert_eq!(
            e.call(0x009f_9d90, &args![this, 5u32, 0u32]).u32(),
            E_POINTER
        );
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_9d90, &args![this, 5u32, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 0x42);
    }

    #[test]
    fn name_and_object_009f9df0() {
        let mut e = rig();
        e.register(OBJ + 4, noop);
        let graph = object(&mut e, OBJ, 8);
        let this = e.mem.alloc(0x40);
        let name = wide(&mut e, "Reader");
        e.mem.set_u32(this + 0x30, name);
        e.mem.set_u32(this + 0x34, graph);
        let out = e.mem.alloc(0x104);
        assert_eq!(e.call(0x009f_9df0, &args![this, out]).u32(), 0);
        assert_eq!(read_wide(&e, out), "Reader");
        assert_eq!(e.mem.u32(out + 0x100), graph);
        assert_eq!(calls(&e, OBJ + 4), vec![vec![graph]]);
        // No name, no object.
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(out, 0x41);
        assert_eq!(e.call(0x009f_9df0, &args![this, out]).u32(), 0);
        assert_eq!(e.mem.u16(out), 0);
        assert_eq!(e.mem.u32(out + 0x100), 0);
        assert_eq!(e.call(0x009f_9df0, &args![this, 0u32]).u32(), E_POINTER);
    }

    #[test]
    fn join_009f9e50_queries_the_owner_and_copies_the_name() {
        let mut e = rig();
        // The interface the owner hands out: its vtable +8 releases it.
        let sink = object(&mut e, OBJ2, 8);
        e.register(OBJ2 + 8, noop);
        e.register_double(OBJ, move |e, a| {
            e.mem.set_u32(a[2], sink);
            ret(0)
        });
        let owner = object(&mut e, OBJ, 8);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2c, 0x4444);
        e.mem.set_u32(this + 0x30, 0x5555);
        let name = wide(&mut e, "Filter");
        assert_eq!(e.call(0x009f_9e50, &args![this, owner, name]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x34), owner);
        assert_eq!(
            calls(&e, OBJ),
            vec![vec![owner, INTERFACE_ID_01093B6C, this + 0x38]]
        );
        assert_eq!(calls(&e, OBJ2 + 8), vec![vec![sink]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x5555]]);
        let copy = e.mem.u32(this + 0x30);
        assert_ne!(copy, name);
        assert_eq!(read_wide(&e, copy), "Filter");
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![14]]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![0x4444]]);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![0x4444]]);
        // A null owner clears +0x38; a null name leaves +0x30 cleared.
        assert_eq!(e.call(0x009f_9e50, &args![this, 0u32, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }

    #[test]
    fn enum_pins_destructor_009f9f70_releases_and_frees_the_list() {
        let mut e = rig();
        e.register(OBJ + 8, noop);
        let filter = e.mem.alloc(0x20);
        let iface = object(&mut e, OBJ, 8);
        // The interface sits at +0xc of the filter.
        let iface_vtable = e.mem.u32(iface);
        e.mem.set_u32(filter + 0xc, iface_vtable);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, filter);
        let spare = e.mem.alloc(0xc);
        e.mem.set_u32(this + 0x18 + 0x14, spare);
        e.call(0x009f_9f70, &args![this]);
        assert_eq!(e.mem.u32(this), ENUM_PINS_VTABLE);
        assert_eq!(calls(&e, OBJ + 8), vec![vec![filter + 0xc]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![spare]]);
    }

    fn release_test(addr: u32, count_at: u32) {
        let mut e = rig();
        e.register(OBJ + 0x1c, noop);
        let this = object(&mut e, OBJ, 0x20);
        e.mem.set_u32(this + count_at, 2);
        assert_eq!(e.call(addr, &args![this]).u32(), 1);
        assert!(calls(&e, OBJ + 0x1c).is_empty());
        assert_eq!(e.call(addr, &args![this]).u32(), 0);
        assert_eq!(calls(&e, OBJ + 0x1c), vec![vec![this, 1]]);
    }

    #[test]
    fn release_009fa060_deletes_at_zero() {
        release_test(0x009f_a060, 0x14);
    }

    #[test]
    fn enum_media_types_destructor_009fa100_releases_the_pin() {
        let mut e = rig();
        e.register(OBJ + 8, noop);
        let iface = object(&mut e, OBJ, 8);
        let pin = e.mem.alloc(0x20);
        let v = e.mem.u32(iface);
        e.mem.set_u32(pin + 0xc, v);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, pin);
        e.call(0x009f_a100, &args![this]);
        assert_eq!(e.mem.u32(this), ENUM_MEDIA_TYPES_VTABLE);
        assert_eq!(calls(&e, OBJ + 8), vec![vec![pin + 0xc]]);
    }

    #[test]
    fn release_009fa1a0_deletes_at_zero() {
        release_test(0x009f_a1a0, 0x10);
    }

    #[test]
    fn pin_destructor_009fa1f0_frees_name_type_and_count() {
        let mut e = rig();
        e.set_global(OBJECT_COUNT, 2u32);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x14, 0x1234);
        e.mem.set_u32(this + 0x34 + 0x40, 4);
        e.mem.set_u32(this + 0x34 + 0x44, 0x4321);
        e.call(0x009f_a1f0, &args![this]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x1234]]);
        assert_eq!(calls(&e, CO_TASK_MEM_FREE), vec![vec![0x4321]]);
        assert_eq!(e.global::<u32>(OBJECT_COUNT), 1);
    }

    #[test]
    fn lookup_009fa260_hands_out_the_two_interfaces() {
        let mut e = rig();
        e.register(OBJ + 4, noop);
        let this = e.mem.alloc(0x20);
        let v = e.mem.alloc(0x10);
        e.mem.set_u32(v + 4, OBJ + 4);
        e.mem.set_u32(this, v);
        e.mem.set_u32(this + 0xc, v);
        e.mem.set_u32(this + 0x10, v);
        let out = e.mem.alloc(4);
        let id = id_copy(&mut e, INTERFACE_ID_01093C3C);
        assert_eq!(e.call(0x009f_a260, &args![this, id, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), this + 0xc);
        let id = id_copy(&mut e, INTERFACE_ID_01093B7C);
        assert_eq!(e.call(0x009f_a260, &args![this, id, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), this + 0x10);
        let id = id_copy(&mut e, INTERFACE_ID_01093F14);
        assert_eq!(e.call(0x009f_a260, &args![this, id, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), this);
        let id = id_copy(&mut e, INTERFACE_ID_01093B4C);
        assert_eq!(
            e.call(0x009f_a260, &args![this, id, out]).u32(),
            E_NOINTERFACE
        );
        assert_eq!(e.mem.u32(out), 0);
    }

    fn delegating_test(addr: u32, slot: u32) {
        let mut e = rig();
        e.register(OBJ + slot, |_, _| ret(3));
        let iface = object(&mut e, OBJ, 8);
        let owner = e.mem.alloc(0x20);
        let v = e.mem.u32(iface);
        e.mem.set_u32(owner + 0xc, v);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x28, owner);
        assert_eq!(e.call(addr, &args![this]).u32(), 3);
        assert_eq!(calls(&e, OBJ + slot), vec![vec![owner + 0xc]]);
    }

    #[test]
    fn delegating_add_009fa2f0() {
        delegating_test(0x009f_a2f0, 4);
    }

    #[test]
    fn delegating_release_009fa310() {
        delegating_test(0x009f_a310, 8);
    }

    #[test]
    fn assign_009fa330_keeps_only_failures() {
        let mut e = rig();
        let this = e.mem.alloc(0x80);
        e.register(0x009f_9c80, |_, _| ret(E_OUTOFMEMORY));
        assert_eq!(
            e.call(0x009f_a330, &args![this, 0x10u32]).u32(),
            E_OUTOFMEMORY
        );
        assert_eq!(calls(&e, 0x009f_9c80), vec![vec![this + 0x34, 0x10]]);
        e.register(0x009f_9c80, |_, _| ret(1));
        assert_eq!(e.call(0x009f_a330, &args![this, 0x10u32]).u32(), 0);
    }

    #[test]
    fn direction_check_009fa350() {
        let mut e = rig();
        e.register(OBJ + 0x24, |e, a| {
            e.mem.set_u32(a[1], 1);
            ret(0)
        });
        let pin = object(&mut e, OBJ, 8);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x1c, 1);
        assert_eq!(
            e.call(0x009f_a350, &args![this, pin]).u32(),
            VFW_E_INVALID_DIRECTION
        );
        e.mem.set_u32(this + 0x1c, 0);
        assert_eq!(e.call(0x009f_a350, &args![this, pin]).u32(), 0);
    }

    #[test]
    fn disconnect_009fa380() {
        let mut e = rig();
        e.register(OBJ + 0x2c, |_, _| ret(0));
        e.register(OBJ2 + 8, noop);
        let this = object(&mut e, OBJ, 0x20);
        assert_eq!(e.call(0x009f_a380, &args![this]).u32(), 1);
        let held = object(&mut e, OBJ2, 8);
        e.mem.set_u32(this + 0x18, held);
        assert_eq!(e.call(0x009f_a380, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(calls(&e, OBJ2 + 8), vec![vec![held]]);
        e.mem.set_u32(this + 0x18, held);
        e.register(OBJ + 0x2c, |_, _| ret(E_FAIL));
        assert_eq!(e.call(0x009f_a380, &args![this]).u32(), E_FAIL);
        assert_eq!(e.mem.u32(this + 0x18), held);
    }

    #[test]
    fn connected_to_009fa3c0() {
        let mut e = rig();
        e.register(OBJ + 4, noop);
        let this = e.mem.alloc(0x20);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_a3c0, &args![this, 0u32]).u32(), E_POINTER);
        assert_eq!(
            e.call(0x009f_a3c0, &args![this, out]).u32(),
            VFW_E_NOT_CONNECTED
        );
        let peer = object(&mut e, OBJ, 8);
        e.mem.set_u32(this + 0xc, peer);
        assert_eq!(e.call(0x009f_a3c0, &args![this, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), peer);
        assert_eq!(calls(&e, OBJ + 4), vec![vec![peer]]);
    }

    #[test]
    fn pin_info_009fa400() {
        let mut e = rig();
        e.register(OBJ + 4, noop);
        let iface = object(&mut e, OBJ, 8);
        let filter = e.mem.alloc(0x20);
        let v = e.mem.u32(iface);
        e.mem.set_u32(filter + 0xc, v);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x1c, filter);
        e.mem.set_u32(this + 0x10, 1);
        let name = wide(&mut e, "Output");
        e.mem.set_u32(this + 8, name);
        let out = e.mem.alloc(0x108);
        assert_eq!(e.call(0x009f_a400, &args![this, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), filter + 0xc);
        assert_eq!(e.mem.u32(out + 4), 1);
        assert_eq!(read_wide(&e, out + 8), "Output");
        assert_eq!(calls(&e, OBJ + 4), vec![vec![filter + 0xc]]);
        let bare = e.mem.alloc(0x20);
        e.mem.set_u16(out + 8, 0x41);
        assert_eq!(e.call(0x009f_a400, &args![bare, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u16(out + 8), 0);
        assert_eq!(e.call(0x009f_a400, &args![bare, 0u32]).u32(), E_POINTER);
    }

    #[test]
    fn word_at_10_009fa480() {
        let mut e = rig();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 2);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_a480, &args![this, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 2);
        assert_eq!(e.call(0x009f_a480, &args![this, 0u32]).u32(), E_POINTER);
    }

    #[test]
    fn clear_byte_009fa4d0() {
        let mut e = rig();
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x24, 0xffff_ffff);
        assert_eq!(e.call(0x009f_a4d0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0xffff_ff00);
    }

    #[test]
    fn not_implemented_009faba0() {
        let mut e = rig();
        assert_eq!(
            e.call(0x009f_aba0, &args![1u32, 2u32, 3u32]).u32(),
            E_NOTIMPL
        );
    }

    #[test]
    fn lookup_009fb480_hands_out_by_id() {
        let mut e = rig();
        e.register(OBJ + 4, noop);
        let this = e.mem.alloc(0x20);
        let v = e.mem.alloc(0x10);
        e.mem.set_u32(v + 4, OBJ + 4);
        e.mem.set_u32(this, v);
        e.mem.set_u32(this + 0xc, v);
        e.mem.set_u32(this + 0x10, v);
        let out = e.mem.alloc(4);
        for (id, at) in [
            (INTERFACE_ID_01093BEC, 0xc),
            (INTERFACE_ID_01093BFC, 0xc),
            (INTERFACE_ID_01093F24, 0xc),
            (INTERFACE_ID_01093B9C, 0x10),
            (INTERFACE_ID_01093F14, 0),
        ] {
            let id = id_copy(&mut e, id);
            assert_eq!(e.call(0x009f_b480, &args![this, id, out]).u32(), 0);
            assert_eq!(e.mem.u32(out), this + at);
        }
        let id = id_copy(&mut e, INTERFACE_ID_01093C3C);
        assert_eq!(
            e.call(0x009f_b480, &args![this, id, out]).u32(),
            E_NOINTERFACE
        );
    }

    #[test]
    fn filter_destructor_009fb550() {
        let mut e = rig();
        e.set_global(OBJECT_COUNT, 1u32);
        e.set_global(OBJECT_COUNT_MODULE, 0x0bad_0000u32);
        e.register(OBJ + 8, noop);
        let held = object(&mut e, OBJ, 8);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x3c, 0x3333);
        e.mem.set_u32(this + 0x18, held);
        e.call(0x009f_b550, &args![this]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x3333]]);
        assert_eq!(calls(&e, OBJ + 8), vec![vec![held]]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(calls(&e, FREE_LIBRARY), vec![vec![0x0bad_0000]]);
        assert_eq!(e.global::<u32>(OBJECT_COUNT_MODULE), 0);
    }

    #[test]
    fn set_object_009fb5d0_swaps_references() {
        let mut e = rig();
        e.register(OBJ + 4, noop);
        e.register(OBJ2 + 8, noop);
        let new = object(&mut e, OBJ, 8);
        let old = object(&mut e, OBJ2, 8);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, old);
        e.mem.set_u32(this + 0x2c, 0x4444);
        assert_eq!(e.call(0x009f_b5d0, &args![this, new]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0xc), new);
        assert_eq!(calls(&e, OBJ + 4), vec![vec![new]]);
        assert_eq!(calls(&e, OBJ2 + 8), vec![vec![old]]);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![0x4444]]);
    }

    /// A filter object (its interface at +0xc is `this`) with two pins:
    /// the first connected (+0x18 set), the second not. Pin slots at OBJ2.
    fn filter_with_pins(e: &mut Engine) -> (u32, [u32; 2]) {
        let pins = [object(e, OBJ2, 0x20), object(e, OBJ2, 0x20)];
        e.mem.set_u32(pins[0] + 0x18, 1);
        let filter = object(e, OBJ, 0x40);
        e.register(OBJ + 0x18, |_, _| ret(2));
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, pins[0]);
        e.mem.set_u32(list + 4, pins[1]);
        e.register_double(OBJ + 0x1c, move |e, a| ret(e.mem.u32(list + 4 * a[1])));
        e.mem.set_u32(filter + 0xc + 0x2c, 0x4444);
        (filter, pins)
    }

    #[test]
    fn stop_009fb6f0_deactivates_connected_pins() {
        let mut e = rig();
        let (filter, pins) = filter_with_pins(&mut e);
        e.register(OBJ2 + 0x18, |_, _| ret(E_FAIL));
        let this = filter + 0xc;
        e.mem.set_u32(this + 8, 2);
        assert_eq!(e.call(0x009f_b6f0, &args![this]).u32(), E_FAIL);
        assert_eq!(calls(&e, OBJ2 + 0x18), vec![vec![pins[0]]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        // Already stopped: no pin is called.
        assert_eq!(e.call(0x009f_b6f0, &args![this]).u32(), 0);
        assert_eq!(calls(&e, OBJ2 + 0x18).len(), 1);
    }

    #[test]
    fn pause_009fb7a0_activates_from_stopped() {
        let mut e = rig();
        let (filter, pins) = filter_with_pins(&mut e);
        e.register(OBJ2 + 0x14, |_, _| ret(0));
        let this = filter + 0xc;
        assert_eq!(e.call(0x009f_b7a0, &args![this]).u32(), 0);
        assert_eq!(calls(&e, OBJ2 + 0x14), vec![vec![pins[0]]]);
        assert_eq!(e.mem.u32(this + 8), 1);
        e.mem.set_u32(this + 8, 0);
        e.register(OBJ2 + 0x14, |_, _| ret(E_FAIL));
        assert_eq!(e.call(0x009f_b7a0, &args![this]).u32(), E_FAIL);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn run_009fb860_pauses_then_runs_the_pins() {
        let mut e = rig();
        let (filter, pins) = filter_with_pins(&mut e);
        let this = filter + 0xc;
        // `this`'s own vtable +0x14 (stdcall on the interface).
        let iface_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(iface_vtable + 0x14, OBJ3 + 0x14);
        e.mem.set_u32(this, iface_vtable);
        e.register(OBJ3 + 0x14, |_, _| ret(0));
        e.register(OBJ2 + 0x1c, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_b860, &args![this, 0x1122_3344_5566_7788u64])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(this + 0x14), 0x5566_7788);
        assert_eq!(e.mem.u32(this + 0x18), 0x1122_3344);
        assert_eq!(calls(&e, OBJ3 + 0x14), vec![vec![this]]);
        assert_eq!(
            calls(&e, OBJ2 + 0x1c),
            vec![vec![pins[0], 0x5566_7788, 0x1122_3344]]
        );
        assert_eq!(e.mem.u32(this + 8), 2);
        // Running already: nothing more is called.
        assert_eq!(e.call(0x009f_b860, &args![this, 5u64]).u32(), 0);
        assert_eq!(calls(&e, OBJ2 + 0x1c).len(), 1);
        // A failed pause is returned.
        e.mem.set_u32(this + 8, 0);
        e.register(OBJ3 + 0x14, |_, _| ret(E_FAIL));
        assert_eq!(e.call(0x009f_b860, &args![this, 5u64]).u32(), E_FAIL);
    }

    #[test]
    fn enum_pins_deleting_destructor_009fbb30() {
        let mut e = rig();
        e.register(OBJ + 8, noop);
        let iface = object(&mut e, OBJ, 8);
        let filter = e.mem.alloc(0x20);
        let v = e.mem.u32(iface);
        e.mem.set_u32(filter + 0xc, v);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, filter);
        assert_eq!(e.call(0x009f_bb30, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x009f_bb30, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn enum_pins_next_009fbb50_hands_out_new_pins() {
        let mut e = rig();
        e.register(HEAD_POSITION, |e, a| ret(e.mem.u32(a[0])));
        e.register(LIST_GET_AT, |e, a| {
            ret(if a[0] == 0 { 0 } else { e.mem.u32(a[0] + 8) })
        });
        e.register(OBJ2 + 4, noop);
        let pins = [object(&mut e, OBJ2, 0x20), object(&mut e, OBJ2, 0x20)];
        for pin in pins {
            // The interface handed out (+0xc) shares the pin's vtable.
            let v = e.mem.u32(pin);
            e.mem.set_u32(pin + 0xc, v);
        }
        let filter = object(&mut e, OBJ, 0x20);
        e.register(OBJ + 0x14, |_, _| ret(7));
        e.register_double(OBJ + 0x1c, move |_, a| ret(pins[a[1] as usize]));
        e.register(ENUM_PINS_REFRESH, |e, a| {
            e.mem.set_u32(a[0] + 0x10, 7);
            ret(0)
        });
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 8, 2);
        e.mem.set_u32(this + 0xc, filter);
        e.mem.set_u32(this + 0x18 + 0xc, 10);
        let out = e.mem.alloc(0x10);
        let fetched = e.mem.alloc(4);
        assert_eq!(
            e.call(0x009f_bb50, &args![this, 3u32, out, fetched]).u32(),
            1
        );
        assert_eq!(calls(&e, ENUM_PINS_REFRESH), vec![vec![this]]);
        assert_eq!(e.mem.u32(fetched), 2);
        assert_eq!(e.mem.u32(out), pins[0] + 0xc);
        assert_eq!(e.mem.u32(out + 4), pins[1] + 0xc);
        assert_eq!(e.mem.u32(this + 0x18 + 8), 2);
        assert_eq!(e.mem.u32(this + 4), 2);
        // Nothing left: S_FALSE at once, fetched cleared.
        assert_eq!(
            e.call(0x009f_bb50, &args![this, 1u32, out, fetched]).u32(),
            1
        );
        assert_eq!(e.mem.u32(fetched), 0);
        assert_eq!(
            e.call(0x009f_bb50, &args![this, 2u32, out, 0u32]).u32(),
            E_INVALIDARG
        );
        assert_eq!(
            e.call(0x009f_bb50, &args![this, 1u32, 0u32, 0u32]).u32(),
            E_POINTER
        );
        // A null pin from the filter is out of sync.
        e.mem.set_u32(this + 4, 0);
        e.register(OBJ + 0x1c, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_bb50, &args![this, 1u32, out, 0u32]).u32(),
            VFW_E_ENUM_OUT_OF_SYNC
        );
    }

    #[test]
    fn enum_media_types_deleting_destructor_009fbca0() {
        let mut e = rig();
        e.register(OBJ + 8, noop);
        let iface = object(&mut e, OBJ, 8);
        let pin = e.mem.alloc(0x20);
        let v = e.mem.u32(iface);
        e.mem.set_u32(pin + 0xc, v);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, pin);
        assert_eq!(e.call(0x009f_bca0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn enum_media_types_next_009fbcc0_copies_types_out() {
        let mut e = rig();
        let pin = object(&mut e, OBJ, 0x20);
        e.register(OBJ + 0x10, |_, _| ret(3));
        e.register(OBJ + 0x34, |e, a| {
            if a[1] >= 1 {
                return ret(1);
            }
            e.mem.set_u32(a[2], 0xa0 + a[1]);
            e.mem.set_u32(a[2] + 0x3c, 0x5150);
            ret(0)
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, pin);
        e.mem.set_u32(this + 0xc, 3);
        let out = e.mem.alloc(8);
        let fetched = e.mem.alloc(4);
        assert_eq!(
            e.call(0x009f_bcc0, &args![this, 2u32, out, fetched]).u32(),
            1
        );
        assert_eq!(e.mem.u32(fetched), 1);
        let copy = e.mem.u32(out);
        assert_eq!(e.mem.u32(copy), 0xa0);
        assert_eq!(e.mem.u32(copy + 0x3c), 0x5150);
        assert_eq!(e.mem.u32(this + 4), 2);
        assert_eq!(calls(&e, CO_TASK_MEM_ALLOC), vec![vec![0x48]]);
        // Version mismatch and argument checks.
        e.mem.set_u32(this + 0xc, 4);
        assert_eq!(
            e.call(0x009f_bcc0, &args![this, 1u32, out, fetched]).u32(),
            VFW_E_ENUM_OUT_OF_SYNC
        );
        e.mem.set_u32(this + 0xc, 3);
        assert_eq!(
            e.call(0x009f_bcc0, &args![this, 2u32, out, 0u32]).u32(),
            E_INVALIDARG
        );
        assert_eq!(
            e.call(0x009f_bcc0, &args![this, 2u32, 0u32, 0u32]).u32(),
            E_POINTER
        );
    }

    /// A pin object (slots at OBJ) and a receiver (slots at OBJ2) for the
    /// connection functions.
    fn pin_and_receiver(e: &mut Engine) -> (u32, u32) {
        let pin = object(e, OBJ, 0x40);
        let receiver = object(e, OBJ2, 0x20);
        for slot in [0x2c, 0x20, 0x24, 0x28, 0x30] {
            e.register(OBJ + slot, |_, _| ret(0));
        }
        for slot in [0x4, 0x8, 0x10, 0x14] {
            e.register(OBJ2 + slot, |_, _| ret(0));
        }
        (pin, receiver)
    }

    #[test]
    fn attempt_009fbf00_connects_or_cleans_up() {
        let mut e = rig();
        let (pin, receiver) = pin_and_receiver(&mut e);
        assert_eq!(e.call(0x009f_bf00, &args![pin, receiver, 0x60u32]).u32(), 0);
        assert_eq!(e.mem.u32(pin + 0x18), receiver);
        assert_eq!(
            calls(&e, OBJ2 + 0x10),
            vec![vec![receiver, pin + 0xc, 0x60]]
        );
        assert!(calls(&e, OBJ + 0x2c).is_empty());
        // A positive type check becomes "type not accepted".
        e.register(OBJ + 0x20, |_, _| ret(1));
        assert_eq!(
            e.call(0x009f_bf00, &args![pin, receiver, 0x60u32]).u32(),
            VFW_E_TYPE_NOT_ACCEPTED
        );
        assert_eq!(e.mem.u32(pin + 0x18), 0);
        assert_eq!(calls(&e, OBJ2 + 8), vec![vec![receiver]]);
        // A failed completion disconnects the receiver too.
        e.register(OBJ + 0x20, |_, _| ret(0));
        e.register(OBJ + 0x30, |_, _| ret(0x8000_0001));
        assert_eq!(
            e.call(0x009f_bf00, &args![pin, receiver, 0x60u32]).u32(),
            0x8000_0001
        );
        assert_eq!(calls(&e, OBJ2 + 0x14), vec![vec![receiver]]);
        // A failed connect check returns at once.
        e.register(OBJ + 0x28, |_, _| ret(E_FAIL));
        assert_eq!(
            e.call(0x009f_bf00, &args![pin, receiver, 0x60u32]).u32(),
            E_FAIL
        );
    }

    /// An enumerator (slots at OBJ3) handing out `count` fresh media types.
    fn enumerator(e: &mut Engine, count: u32) -> u32 {
        let en = object(e, OBJ3, 0x10);
        e.register(OBJ3 + 0x14, |_, _| ret(0));
        e.register(OBJ3 + 8, noop);
        let mut handed = 0;
        e.register_double(OBJ3 + 0xc, move |e, a| {
            if handed == count {
                return ret(1);
            }
            handed += 1;
            let mt = e.mem.alloc(0x48);
            e.mem.set_u32(a[2], mt);
            e.mem.set_u32(a[3], 1);
            ret(0)
        });
        en
    }

    #[test]
    fn try_types_009fbfc0_keeps_the_first_specific_failure() {
        let mut e = rig();
        let (pin, receiver) = pin_and_receiver(&mut e);
        let en = enumerator(&mut e, 2);
        e.register(OBJ + 0x28, |_, _| ret(0x8000_0002));
        assert_eq!(
            e.call(0x009f_bfc0, &args![pin, receiver, 0u32, en]).u32(),
            0x8000_0002
        );
        assert_eq!(calls(&e, CO_TASK_MEM_FREE).len(), 2);
        // A type the test rejects is not tried.
        let en = enumerator(&mut e, 1);
        e.register(TYPE_TEST_009F9A80, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_bfc0, &args![pin, receiver, 0x60u32, en])
                .u32(),
            VFW_E_NO_ACCEPTABLE_TYPES
        );
        // An accepted type returns 0.
        let en = enumerator(&mut e, 3);
        e.register(TYPE_TEST_009F9A80, |_, _| ret(1));
        e.register(OBJ + 0x28, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_bfc0, &args![pin, receiver, 0x60u32, en])
                .u32(),
            0
        );
        // A failed reset is returned.
        e.register(OBJ3 + 0x14, |_, _| ret(E_FAIL));
        assert_eq!(
            e.call(0x009f_bfc0, &args![pin, receiver, 0u32, en]).u32(),
            E_FAIL
        );
    }

    #[test]
    fn agree_009fc0b0_tries_both_enumerators() {
        let mut e = rig();
        let (pin, receiver) = pin_and_receiver(&mut e);
        // A fully given type goes straight to the attempt.
        e.register(TYPE_TEST_009F9A50, |_, _| ret(0));
        assert_eq!(e.call(0x009f_c0b0, &args![pin, receiver, 0x60u32]).u32(), 0);
        assert_eq!(calls(&e, OBJ2 + 0x30).len(), 0);
        // Otherwise: the receiver's enumerator first (byte +0x26 is 0),
        // then the pin's own interface at +0xc.
        e.register(TYPE_TEST_009F9A50, |_, _| ret(1));
        let iface_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(iface_vtable + 0x30, OBJ3 + 0x30);
        e.mem.set_u32(pin + 0xc, iface_vtable);
        e.register_double(OBJ2 + 0x30, move |e, a| {
            let en = object(e, OBJ3, 0x10);
            e.mem.set_u32(a[1], en);
            ret(0)
        });
        e.register(OBJ3 + 0x30, |_, _| ret(E_FAIL));
        e.register(OBJ3 + 0x14, |_, _| ret(0));
        e.register(OBJ3 + 0xc, |_, _| ret(1));
        e.register(OBJ3 + 8, noop);
        assert_eq!(
            e.call(0x009f_c0b0, &args![pin, receiver, 0x60u32]).u32(),
            VFW_E_NO_ACCEPTABLE_TYPES
        );
        assert_eq!(calls(&e, OBJ2 + 0x30).len(), 1);
        let own = calls(&e, OBJ3 + 0x30);
        assert_eq!(own.len(), 1);
        assert_eq!(own[0][0], pin + 0xc);
        assert_eq!(calls(&e, OBJ3 + 8).len(), 1);
    }

    #[test]
    fn disconnect_guard_009fc300() {
        let mut e = rig();
        let pin = object(&mut e, OBJ, 0x40);
        let filter = e.mem.alloc(0x20);
        let this = pin + 0xc;
        e.mem.set_u32(this + 0x14, 0x4444);
        e.mem.set_u32(this + 0x1c, filter);
        e.mem.set_u32(filter + 0x14, 1);
        assert_eq!(e.call(0x009f_c300, &args![this]).u32(), VFW_E_NOT_STOPPED);
        e.mem.set_u32(filter + 0x14, 0);
        assert_eq!(e.call(0x009f_c300, &args![this]).u32(), 1);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 2);
    }

    #[test]
    fn name_thunk_009fc450_passes_the_name() {
        let mut e = rig();
        e.register(WIDE_STRING_DUPLICATE, |_, _| ret(0));
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234);
        assert_eq!(e.call(0x009f_c450, &args![this, 0x88u32]).u32(), 0);
        assert_eq!(calls(&e, WIDE_STRING_DUPLICATE), vec![vec![0x1234, 0x88]]);
    }

    #[test]
    fn filter_constructor_009fc5a0() {
        let mut e = rig();
        let this = e.mem.alloc(0x50);
        for i in 0..0x14 {
            e.mem.set_u32(this + 4 * i, 0xeeee_eeee);
        }
        let id = id_copy(&mut e, CLASS_ID_01093D70);
        assert_eq!(
            e.call(
                0x009f_c5a0,
                &args![this, 0u32, 0x900u32, 0x800u32, id, 0u32]
            )
            .u32(),
            this
        );
        assert_eq!(e.mem.u32(this + 4), 0x900);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0xeeee_eeee);
        assert_eq!(e.mem.u32(this + 0x28), CLASS_ID_01093D70);
        assert_eq!(e.mem.u32(this + 0x34), CLASS_ID_01093D70 ^ 3);
        assert_eq!(e.mem.u32(this + 0x38), 0x800);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x48), 1);
        assert_eq!(e.global::<u32>(OBJECT_COUNT), 1);
    }

    #[test]
    fn enum_pins_constructor_009fc600() {
        let mut e = rig();
        let filter = object(&mut e, OBJ, 0x20);
        let v = e.mem.u32(filter);
        e.mem.set_u32(filter + 0xc, v);
        e.register(OBJ + 4, noop);
        e.register(OBJ + 0x14, |_, _| ret(5));
        e.register(OBJ + 0x18, |_, _| ret(3));
        let this = e.mem.alloc(0x30);
        assert_eq!(e.call(0x009f_c600, &args![this, filter, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), ENUM_PINS_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0x10), 5);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        assert_eq!(e.mem.u32(this + 0x18 + 0xc), 10);
        assert_eq!(calls(&e, OBJ + 4), vec![vec![filter + 0xc]]);
        // A copy takes the source's position, total, version and list.
        e.register(LIST_APPEND_009FD2D0, |_, _| ret(1));
        let source = e.mem.alloc(0x30);
        e.mem.set_u32(source + 4, 1);
        e.mem.set_u32(source + 8, 2);
        e.mem.set_u32(source + 0x10, 9);
        let copy = e.mem.alloc(0x30);
        e.call(0x009f_c600, &args![copy, filter, source]);
        assert_eq!(e.mem.u32(copy + 4), 1);
        assert_eq!(e.mem.u32(copy + 8), 2);
        assert_eq!(e.mem.u32(copy + 0x10), 9);
        assert_eq!(
            calls(&e, LIST_APPEND_009FD2D0),
            vec![vec![copy + 0x18, source + 0x18]]
        );
    }

    #[test]
    fn enum_media_types_constructor_009fc780() {
        let mut e = rig();
        let pin = object(&mut e, OBJ, 0x20);
        let v = e.mem.u32(pin);
        e.mem.set_u32(pin + 0xc, v);
        e.register(OBJ + 4, noop);
        e.register(OBJ + 0x10, |_, _| ret(6));
        let this = e.mem.alloc(0x14);
        assert_eq!(e.call(0x009f_c780, &args![this, pin, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), ENUM_MEDIA_TYPES_VTABLE);
        assert_eq!(e.mem.u32(this + 8), pin);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        let source = e.mem.alloc(0x14);
        e.mem.set_u32(source + 4, 2);
        e.mem.set_u32(source + 0xc, 8);
        let copy = e.mem.alloc(0x14);
        e.call(0x009f_c780, &args![copy, pin, source]);
        assert_eq!(e.mem.u32(copy + 4), 2);
        assert_eq!(e.mem.u32(copy + 0xc), 8);
        assert_eq!(calls(&e, OBJ + 0x10).len(), 1);
    }

    #[test]
    fn pin_constructor_009fc890() {
        let mut e = rig();
        let this = e.mem.alloc(0xa0);
        let name = wide(&mut e, "In");
        assert_eq!(
            e.call(
                0x009f_c890,
                &args![this, 0x10u32, 0x20u32, 0x30u32, 0x40u32, name, 1u32]
            )
            .u32(),
            this
        );
        assert_eq!(e.mem.u32(this + 4), this);
        assert_eq!(e.mem.u32(this + 0x1c), 1);
        assert_eq!(e.mem.u32(this + 0x20), 0x30);
        assert_eq!(e.mem.u32(this + 0x28), 0x20);
        assert_eq!(e.mem.u32(this + 0x30), 1);
        assert_eq!(e.mem.u32(this + 0x34 + 0x20), 1);
        assert_eq!(e.mem.u32(this + 0x88), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x8c), 0x7fff_ffff);
        assert_eq!(e.mem.f64(this + 0x90), 1.0);
        assert_eq!(read_wide(&e, e.mem.u32(this + 0x14)), "In");
        let bare = e.mem.alloc(0xa0);
        e.call(
            0x009f_c890,
            &args![bare, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(e.mem.u32(bare + 0x14), 0);
        assert_eq!(calls(&e, OPERATOR_NEW).len(), 1);
    }

    #[test]
    fn connect_009fc990_checks_state_then_agrees() {
        let mut e = rig();
        let (pin, receiver) = pin_and_receiver(&mut e);
        let filter = e.mem.alloc(0x20);
        let this = pin + 0xc;
        e.mem.set_u32(this + 0x14, 0x4444);
        e.mem.set_u32(this + 0x1c, filter);
        assert_eq!(
            e.call(0x009f_c990, &args![this, 0u32, 0u32]).u32(),
            E_POINTER
        );
        e.mem.set_u32(this + 0xc, 1);
        assert_eq!(
            e.call(0x009f_c990, &args![this, receiver, 0u32]).u32(),
            VFW_E_ALREADY_CONNECTED
        );
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(filter + 0x14, 1);
        assert_eq!(
            e.call(0x009f_c990, &args![this, receiver, 0u32]).u32(),
            VFW_E_NOT_STOPPED
        );
        e.mem.set_u8(this + 0x19, 1);
        e.register(TYPE_TEST_009F9A50, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_c990, &args![this, receiver, 0x60u32]).u32(),
            0
        );
        assert_eq!(e.mem.u32(pin + 0x18), receiver);
        e.mem.set_u32(pin + 0x18, 0);
        e.register(OBJ + 0x28, |_, _| ret(E_FAIL));
        assert_eq!(
            e.call(0x009f_c990, &args![this, receiver, 0x60u32]).u32(),
            E_FAIL
        );
        assert_eq!(calls(&e, OBJ + 0x2c).len(), 2);
        assert_eq!(
            calls(&e, ENTER_CRITICAL_SECTION).len(),
            calls(&e, LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn make_media_type_enumerator_009fcab0() {
        let mut e = rig();
        let pin = object(&mut e, OBJ, 0x20);
        let v = e.mem.u32(pin);
        e.mem.set_u32(pin + 0xc, v);
        e.register(OBJ + 4, noop);
        e.register(OBJ + 0x10, |_, _| ret(6));
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_cab0, &args![pin + 0xc, out]).u32(), 0);
        let made = e.mem.u32(out);
        assert_eq!(e.mem.u32(made), ENUM_MEDIA_TYPES_VTABLE);
        assert_eq!(e.mem.u32(made + 8), pin);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x14]]);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_cab0, &args![pin + 0xc, out]).u32(),
            E_OUTOFMEMORY
        );
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(
            e.call(0x009f_cab0, &args![pin + 0xc, 0u32]).u32(),
            E_POINTER
        );
    }

    #[test]
    fn make_pin_enumerator_009fcdc0() {
        let mut e = rig();
        let filter = object(&mut e, OBJ, 0x20);
        let v = e.mem.u32(filter);
        e.mem.set_u32(filter + 0xc, v);
        e.register(OBJ + 4, noop);
        e.register(OBJ + 0x14, |_, _| ret(1));
        e.register(OBJ + 0x18, |_, _| ret(2));
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_cdc0, &args![filter + 0xc, out]).u32(), 0);
        let made = e.mem.u32(out);
        assert_eq!(e.mem.u32(made), ENUM_PINS_VTABLE);
        assert_eq!(e.mem.u32(made + 0xc), filter);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x30]]);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_cdc0, &args![filter + 0xc, out]).u32(),
            E_OUTOFMEMORY
        );
    }

    #[test]
    fn counted_constructor_009fce60() {
        let mut e = rig();
        assert_eq!(e.call(0x009f_ce60, &args![0x40u32, 7u32]).u32(), 0x40);
        assert_eq!(e.global::<u32>(OBJECT_COUNT), 1);
    }

    #[test]
    fn counted_destructor_009fce80_frees_the_module_at_zero() {
        let mut e = rig();
        e.set_global(OBJECT_COUNT, 2u32);
        e.set_global(OBJECT_COUNT_MODULE, 0x0bad_0000u32);
        e.call(0x009f_ce80, &args![]);
        assert!(calls(&e, FREE_LIBRARY).is_empty());
        e.call(0x009f_ce80, &args![]);
        assert_eq!(calls(&e, FREE_LIBRARY), vec![vec![0x0bad_0000]]);
        assert_eq!(e.global::<u32>(OBJECT_COUNT_MODULE), 0);
    }

    #[test]
    fn hand_out_009fceb0() {
        let mut e = rig();
        e.register(OBJ + 4, noop);
        let iface = object(&mut e, OBJ, 8);
        assert_eq!(e.call(0x009f_ceb0, &args![iface, 0u32]).u32(), E_POINTER);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_ceb0, &args![iface, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), iface);
        assert_eq!(calls(&e, OBJ + 4), vec![vec![iface]]);
    }

    #[test]
    fn unknown_constructor_009fcee0() {
        let mut e = rig();
        let this = e.mem.alloc(0xc);
        assert_eq!(
            e.call(0x009f_cee0, &args![this, 0u32, 0x500u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this + 4), 0x500);
        let other = e.mem.alloc(0xc);
        e.mem.set_u32(other + 8, 9);
        e.call(0x009f_cee0, &args![other, 0u32, 0u32]);
        assert_eq!(e.mem.u32(other + 4), other);
        assert_eq!(e.mem.u32(other + 8), 0);
        assert_eq!(e.global::<u32>(OBJECT_COUNT), 2);
    }

    #[test]
    fn base_lookup_009fcf10() {
        let mut e = rig();
        e.register(OBJ + 4, noop);
        let this = object(&mut e, OBJ, 8);
        let out = e.mem.alloc(4);
        let id = id_copy(&mut e, INTERFACE_ID_01093F14);
        assert_eq!(e.call(0x009f_cf10, &args![this, id, 0u32]).u32(), E_POINTER);
        assert_eq!(e.call(0x009f_cf10, &args![this, id, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), this);
        let id = id_copy(&mut e, INTERFACE_ID_01093B6C);
        assert_eq!(
            e.call(0x009f_cf10, &args![this, id, out]).u32(),
            E_NOINTERFACE
        );
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn add_reference_009fcf60_is_at_least_one() {
        let mut e = rig();
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0xffff_ffff);
        assert_eq!(e.call(0x009f_cf60, &args![this]).u32(), 1);
        assert_eq!(e.call(0x009f_cf60, &args![this]).u32(), 1);
        assert_eq!(e.call(0x009f_cf60, &args![this]).u32(), 2);
    }

    #[test]
    fn release_009fcf80_deletes_at_zero() {
        let mut e = rig();
        e.register(OBJ + 0xc, noop);
        let this = object(&mut e, OBJ, 0xc);
        e.mem.set_u32(this + 8, 3);
        assert_eq!(e.call(0x009f_cf80, &args![this]).u32(), 2);
        e.mem.set_u32(this + 8, 1);
        assert_eq!(e.call(0x009f_cf80, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(calls(&e, OBJ + 0xc), vec![vec![this, 1]]);
        e.mem.set_u32(this + 8, 0);
        assert_eq!(e.call(0x009f_cf80, &args![this]).u32(), 0xffff_ffff);
    }

    #[test]
    fn event_constructor_009fcfc0() {
        let mut e = rig();
        e.register(CREATE_EVENT_A, |_, _| ret(0x88));
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_cfc0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x88);
        assert_eq!(calls(&e, CREATE_EVENT_A), vec![vec![0, 1, 0, 0]]);
    }

    #[test]
    fn event_destructor_009fcfe0() {
        let mut e = rig();
        let this = e.mem.alloc(4);
        e.call(0x009f_cfe0, &args![this]);
        assert!(calls(&e, CLOSE_HANDLE).is_empty());
        e.mem.set_u32(this, 0x88);
        e.call(0x009f_cfe0, &args![this]);
        assert_eq!(calls(&e, CLOSE_HANDLE), vec![vec![0x88]]);
    }

    #[test]
    fn bounded_copy_009fcff0() {
        let mut e = rig();
        let src = wide(&mut e, "abcdef");
        let dst = e.mem.alloc(0x20);
        assert_eq!(e.call(0x009f_cff0, &args![dst, src, 4u32]).u32(), dst);
        assert_eq!(read_wide(&e, dst), "abc");
        assert_eq!(e.call(0x009f_cff0, &args![dst, src, 0x10u32]).u32(), dst);
        assert_eq!(read_wide(&e, dst), "abcdef");
        e.mem.set_u16(dst, 0x41);
        e.call(0x009f_cff0, &args![dst, src, 0u32]);
        assert_eq!(e.mem.u16(dst), 0x41);
    }

    #[test]
    fn wide_length_009fd080() {
        let mut e = rig();
        let s = wide(&mut e, "hello");
        assert_eq!(e.call(0x009f_d080, &args![s]).u32(), 5);
        let empty = wide(&mut e, "");
        assert_eq!(e.call(0x009f_d080, &args![empty]).u32(), 0);
    }

    /// A list (0x18 bytes) built by `009fd120`.
    fn list(e: &mut Engine) -> u32 {
        let l = e.mem.alloc(0x18);
        e.call(0x009f_d120, &args![l, 0u32]);
        l
    }

    #[test]
    fn list_constructor_009fd120() {
        let mut e = rig();
        let l = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_u32(l + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x009f_d120, &args![l, 0u32]).u32(), l);
        for i in 0..6 {
            assert_eq!(e.mem.u32(l + 4 * i), if i == 3 { 10 } else { 0 });
        }
    }

    #[test]
    fn list_clear_009fd140() {
        let mut e = rig();
        let l = list(&mut e);
        let a = e.call(0x009f_d270, &args![l, 0x11u32]).u32();
        let b = e.call(0x009f_d270, &args![l, 0x22u32]).u32();
        e.call(0x009f_d140, &args![l]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![a], vec![b]]);
        assert_eq!(e.mem.u32(l), 0);
        assert_eq!(e.mem.u32(l + 4), 0);
        assert_eq!(e.mem.u32(l + 8), 0);
    }

    #[test]
    fn list_find_009fd1c0() {
        let mut e = rig();
        e.register(HEAD_POSITION, |e, a| ret(e.mem.u32(a[0])));
        e.register(LIST_GET_AT, |e, a| {
            ret(if a[0] == 0 { 0 } else { e.mem.u32(a[0] + 8) })
        });
        let l = list(&mut e);
        e.call(0x009f_d270, &args![l, 0x11u32]);
        let b = e.call(0x009f_d270, &args![l, 0x22u32]).u32();
        assert_eq!(e.call(0x009f_d1c0, &args![l, 0x22u32]).u32(), b);
        assert_eq!(e.call(0x009f_d1c0, &args![l, 0x33u32]).u32(), 0);
    }

    #[test]
    fn list_remove_009fd200_keeps_spares_up_to_the_limit() {
        let mut e = rig();
        let l = list(&mut e);
        let a = e.call(0x009f_d270, &args![l, 0x11u32]).u32();
        let b = e.call(0x009f_d270, &args![l, 0x22u32]).u32();
        let c = e.call(0x009f_d270, &args![l, 0x33u32]).u32();
        assert_eq!(e.call(0x009f_d200, &args![l, 0u32]).u32(), 0);
        assert_eq!(e.call(0x009f_d200, &args![l, b]).u32(), 0x22);
        assert_eq!(e.mem.u32(a + 4), c);
        assert_eq!(e.mem.u32(c), a);
        assert_eq!(e.mem.u32(l + 8), 2);
        assert_eq!(e.mem.u32(l + 0x14), b);
        assert_eq!(e.mem.u32(l + 0x10), 1);
        // Spare list full: the node is deleted.
        e.mem.set_u32(l + 0xc, 1);
        assert_eq!(e.call(0x009f_d200, &args![l, a]).u32(), 0x11);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![a]]);
        assert_eq!(e.mem.u32(l), c);
        assert_eq!(e.mem.u32(c), 0);
        assert_eq!(e.call(0x009f_d200, &args![l, c]).u32(), 0x33);
        assert_eq!(e.mem.u32(l), 0);
        assert_eq!(e.mem.u32(l + 4), 0);
    }

    #[test]
    fn list_add_tail_009fd270_reuses_spares() {
        let mut e = rig();
        let l = list(&mut e);
        let a = e.call(0x009f_d270, &args![l, 0x11u32]).u32();
        assert_eq!(e.mem.u32(l), a);
        assert_eq!(e.mem.u32(l + 4), a);
        assert_eq!(e.mem.u32(a + 8), 0x11);
        e.call(0x009f_d200, &args![l, a]);
        let b = e.call(0x009f_d270, &args![l, 0x22u32]).u32();
        assert_eq!(b, a);
        assert_eq!(e.mem.u32(l + 0x10), 0);
        let c = e.call(0x009f_d270, &args![l, 0x33u32]).u32();
        assert_eq!(e.mem.u32(b + 4), c);
        assert_eq!(e.mem.u32(c), b);
        assert_eq!(e.mem.u32(l + 8), 2);
        assert_eq!(calls(&e, OPERATOR_NEW).len(), 2);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x009f_d270, &args![l, 0x44u32]).u32(), 0);
    }

    #[test]
    fn list_destructor_009fd320_deletes_nodes_and_spares() {
        let mut e = rig();
        let l = list(&mut e);
        let a = e.call(0x009f_d270, &args![l, 0x11u32]).u32();
        let b = e.call(0x009f_d270, &args![l, 0x22u32]).u32();
        e.call(0x009f_d200, &args![l, a]);
        e.call(0x009f_d320, &args![l]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![b], vec![a]]);
    }

    #[test]
    fn list_remove_head_009fd390() {
        let mut e = rig();
        let l = list(&mut e);
        e.call(0x009f_d270, &args![l, 0x11u32]);
        e.call(0x009f_d270, &args![l, 0x22u32]);
        assert_eq!(e.call(0x009f_d390, &args![l]).u32(), 0x11);
        assert_eq!(e.call(0x009f_d390, &args![l]).u32(), 0x22);
        assert_eq!(e.call(0x009f_d390, &args![l]).u32(), 0);
    }

    #[test]
    fn alignment_test_009fd3a0() {
        let mut e = rig();
        e.register(OBJ + 0x10, |_, _| ret(8));
        let held = object(&mut e, OBJ, 8);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, held);
        assert_eq!(e.call(0x009f_d3a0, &args![this, 0x40u32]).u32(), 1);
        assert_eq!(e.call(0x009f_d3a0, &args![this, 0x44u32]).u32(), 0);
    }

    #[test]
    fn output_pin_destructor_009fd3c0() {
        let mut e = rig();
        e.set_global(OBJECT_COUNT, 3u32);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 8 + 0x14, 0x2222);
        e.call(0x009f_d3c0, &args![this]);
        assert_eq!(e.mem.u32(this), ASYNC_OUTPUT_PIN_VTABLE);
        assert_eq!(e.mem.u32(this + 8), ASYNC_OUTPUT_PIN_VTABLE_8);
        assert_eq!(e.mem.u32(this + 0x14), ASYNC_OUTPUT_PIN_VTABLE_14);
        assert_eq!(e.mem.u32(this + 0x18), ASYNC_OUTPUT_PIN_VTABLE_18);
        // 009fa1f0 ran with ECX = this + 8: it deleted the name at this + 0x1c.
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x2222]]);
        assert_eq!(e.global::<u32>(OBJECT_COUNT), 2);
    }

    fn delegate_c_test(addr: u32, slot: u32, extra: &[u32]) {
        let mut e = rig();
        e.register(OBJ + slot, |_, _| ret(4));
        let inner = object(&mut e, OBJ, 8);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, inner);
        let mut words = vec![this];
        words.extend_from_slice(extra);
        assert_eq!(e.call(addr, &words).u32(), 4);
        let mut expected = vec![inner];
        expected.extend_from_slice(extra);
        assert_eq!(calls(&e, OBJ + slot), vec![expected]);
    }

    #[test]
    fn delegating_query_009fd3f0() {
        delegate_c_test(0x009f_d3f0, 0, &[0x10, 0x20]);
    }

    #[test]
    fn delegating_add_009fd410() {
        delegate_c_test(0x009f_d410, 4, &[]);
    }

    #[test]
    fn delegating_release_009fd430() {
        delegate_c_test(0x009f_d430, 8, &[]);
    }

    #[test]
    fn clear_flag_then_direction_check_009fd450() {
        let mut e = rig();
        e.register(OBJ + 0x24, |_, _| ret(0));
        let pin = object(&mut e, OBJ, 8);
        let this = e.mem.alloc(0xa8);
        e.mem.set_u32(this + 0xa0, 1);
        e.mem.set_u32(this + 0x1c, 5);
        assert_eq!(e.call(0x009f_d450, &args![this, pin]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0xa0), 0);
    }

    #[test]
    fn flag_gate_009fd460() {
        let mut e = rig();
        e.register(RETURN_ZERO_00A38DE0, |_, _| ret(0));
        let this = e.mem.alloc(0xa8);
        assert_eq!(
            e.call(0x009f_d460, &args![this, 3u32]).u32(),
            STATUS_80040266
        );
        e.mem.set_u32(this + 0xa0, 1);
        assert_eq!(e.call(0x009f_d460, &args![this, 3u32]).u32(), 0);
        assert_eq!(calls(&e, RETURN_ZERO_00A38DE0), vec![vec![this, 3]]);
    }

    #[test]
    fn clear_flag_then_jump_009fd480() {
        let mut e = rig();
        e.register(RETURN_ZERO_00E7CD40, |_, _| ret(0));
        let this = e.mem.alloc(0xa8);
        e.mem.set_u32(this + 0xa0, 1);
        assert_eq!(e.call(0x009f_d480, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0xa0), 0);
        assert_eq!(calls(&e, RETURN_ZERO_00E7CD40), vec![vec![this]]);
    }

    #[test]
    fn lookup_009fd490_sets_the_flag_for_its_id() {
        let mut e = rig();
        e.register(OBJ + 4, noop);
        let base = object(&mut e, OBJ, 0xb0);
        let this = base + 8;
        let v = e.mem.u32(base);
        e.mem.set_u32(this, v);
        e.mem.set_u32(this + 0xc, v);
        let out = e.mem.alloc(4);
        let id = id_copy(&mut e, INTERFACE_ID_01093B4C);
        assert_eq!(e.call(0x009f_d490, &args![this, id, 0u32]).u32(), E_POINTER);
        assert_eq!(e.call(0x009f_d490, &args![this, id, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), base);
        assert_eq!(e.mem.u32(this + 0xa0), 1);
        let id = id_copy(&mut e, INTERFACE_ID_01093C3C);
        assert_eq!(e.call(0x009f_d490, &args![this, id, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), this + 0xc);
    }

    #[test]
    fn media_type_at_009fd4e0() {
        let mut e = rig();
        let this = e.mem.alloc(0xa0);
        let mt = e.mem.alloc(0x48);
        assert_eq!(
            e.call(0x009f_d4e0, &args![this, -1i32, mt]).u32(),
            E_INVALIDARG
        );
        assert_eq!(
            e.call(0x009f_d4e0, &args![this, 1i32, mt]).u32(),
            VFW_S_NO_MORE_ITEMS
        );
        assert_eq!(
            e.call(0x009f_d4e0, &args![this, 0i32, 0u32]).u32(),
            E_POINTER
        );
        assert_eq!(
            e.call(0x009f_d4e0, &args![this, 0i32, mt]).u32(),
            E_UNEXPECTED
        );
        let owner = e.mem.alloc(0x200);
        e.mem.set_u32(owner + 0x1a0, 0x66);
        e.mem.set_u32(this + 0x98, owner);
        assert_eq!(e.call(0x009f_d4e0, &args![this, 0i32, mt]).u32(), 0);
        assert_eq!(e.mem.u32(mt), 0x66);
    }

    #[test]
    fn media_type_check_009fd530() {
        let mut e = rig();
        let owner = e.mem.alloc(0x200);
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 0x20, 0x4444);
        e.mem.set_u32(this + 0x98, owner);
        let mt = e.mem.alloc(0x48);
        e.mem.set_u32(owner + 0x1a0, 0x31);
        e.mem.set_u32(mt, 0x31);
        // The owner's subtype is the null id: accepted.
        for i in 0..4 {
            e.mem
                .set_u32(owner + 0x1b0 + 4 * i, INTERFACE_ID_01093F04 ^ i);
        }
        assert_eq!(e.call(0x009f_d530, &args![this, mt]).u32(), 0);
        // Otherwise the subtypes must match.
        e.mem.set_u32(owner + 0x1b0, 0x77);
        assert_eq!(e.call(0x009f_d530, &args![this, mt]).u32(), 1);
        for i in 0..4 {
            let w = e.mem.u32(owner + 0x1b0 + 4 * i);
            e.mem.set_u32(mt + 0x10 + 4 * i, w);
        }
        assert_eq!(e.call(0x009f_d530, &args![this, mt]).u32(), 0);
        e.mem.set_u32(mt, 0x32);
        assert_eq!(e.call(0x009f_d530, &args![this, mt]).u32(), 1);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 4);
    }

    /// A reader object at `+0xa4` of a fresh 0xb0-byte `this`, whose held
    /// object (+0x18) answers vtable +0x10 with `alignment` (slots at OBJ3).
    fn with_reader(e: &mut Engine, alignment: u32) -> (u32, u32) {
        let held = object(e, OBJ3, 8);
        e.register_double(OBJ3 + 0x10, move |_, _| ret(alignment));
        let reader = e.mem.alloc(0x90);
        e.mem.set_u32(reader + 0x18, held);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0xa4, reader);
        (this, reader)
    }

    #[test]
    fn allocator_009fd6d0_prefers_the_offered_one() {
        let mut e = rig();
        let (this, _) = with_reader(&mut e, 4);
        e.register(OBJ + 4, noop);
        e.register(OBJ + 8, noop);
        e.register(OBJ + 0xc, |e, a| {
            e.mem.set_u32(a[2] + 8, 8);
            ret(0)
        });
        let offered = object(&mut e, OBJ, 8);
        let request = e.mem.alloc(0x10);
        e.mem.set_u32(request + 8, 2);
        let out = e.mem.alloc(4);
        assert_eq!(
            e.call(0x009f_d6d0, &args![this, offered, request, out])
                .u32(),
            0
        );
        // The request's alignment 2 was raised to the reader's 4.
        assert_eq!(e.mem.u32(request + 8), 4);
        assert_eq!(e.mem.u32(out), offered);
        assert_eq!(calls(&e, OBJ + 4), vec![vec![offered]]);
        assert_eq!(
            e.call(0x009f_d6d0, &args![this, 0u32, request, out]).u32(),
            E_POINTER
        );
        // A bad alignment falls back to a new allocator.
        e.register(OBJ + 0xc, |e, a| {
            e.mem.set_u32(a[2] + 8, 3);
            ret(0)
        });
        e.register(OBJ2 + 8, noop);
        e.register(OBJ2 + 0xc, |e, a| {
            e.mem.set_u32(a[2] + 8, 3);
            ret(0)
        });
        e.register_double(ALLOCATOR_MAKE_009FD5A0, |e, a| {
            let made = object(e, OBJ2, 8);
            e.mem.set_u32(a[0], made);
            ret(0)
        });
        assert_eq!(
            e.call(0x009f_d6d0, &args![this, offered, request, out])
                .u32(),
            VFW_E_BADALIGN
        );
        assert_eq!(calls(&e, OBJ2 + 8).len(), 1);
        e.register(OBJ2 + 0xc, |e, a| {
            e.mem.set_u32(a[2] + 8, 4);
            ret(0)
        });
        assert_eq!(
            e.call(0x009f_d6d0, &args![this, offered, request, out])
                .u32(),
            0
        );
        assert_ne!(e.mem.u32(out), offered);
        e.register(ALLOCATOR_MAKE_009FD5A0, |_, _| ret(E_FAIL));
        assert_eq!(
            e.call(0x009f_d6d0, &args![this, offered, request, out])
                .u32(),
            E_FAIL
        );
    }

    #[test]
    fn sample_request_009fd970_clips_the_end_to_the_total() {
        let mut e = rig();
        let (this, reader) = with_reader(&mut e, 4);
        // Reader +0x18 object: vtable +0xc gives the total (10 seconds).
        e.register(OBJ3 + 0xc, |_, _| Ret {
            eax: 10,
            edx: 0,
            ..Ret::default()
        });
        e.register(ALLDIV, |_, a| {
            let v = (u64::from(a[0]) | u64::from(a[1]) << 32) as i64;
            let d = (u64::from(a[2]) | u64::from(a[3]) << 32) as i64;
            let q = (v / d) as u64;
            Ret {
                eax: q as u32,
                edx: (q >> 32) as u32,
                ..Ret::default()
            }
        });
        e.register(ALLMUL, |_, a| {
            let v = u64::from(a[0]) | u64::from(a[1]) << 32;
            let m = u64::from(a[2]) | u64::from(a[3]) << 32;
            let p = v.wrapping_mul(m);
            Ret {
                eax: p as u32,
                edx: (p >> 32) as u32,
                ..Ret::default()
            }
        });
        // The sample: times 8 s .. 16 s, a buffer at 0x100, length set.
        e.register(OBJ + 0x14, |e, a| {
            e.mem.set_u64(a[1], 80_000_000);
            e.mem.set_u64(a[2], 160_000_000);
            ret(0)
        });
        e.register(OBJ + 0x18, noop);
        e.register(OBJ + 0xc, |e, a| {
            e.mem.set_u32(a[1], 0x100);
            ret(0)
        });
        e.register(OBJ + 0x30, noop);
        let sample = object(&mut e, OBJ, 8);
        // 009fdfe0 builds a request whose +4 is the reader's held object;
        // the read reports the length it was given (result 0).
        e.register(OBJ3 + 0x14, noop);
        e.register(OBJ3 + 0x18, noop);
        e.register(OBJ3 + 4, |_, _| ret(0));
        e.register(OBJ3 + 8, |e, a| {
            e.mem.set_u32(a[4], a[2]);
            ret(0)
        });
        assert_eq!(e.call(0x009f_d970, &args![this, sample]).u32(), 0);
        // The end was clipped to 12 s (10 rounded up to 4): length 4 s.
        let set_time = calls(&e, OBJ + 0x18);
        assert_eq!(set_time.len(), 1);
        assert_eq!(
            calls(&e, OBJ3 + 4),
            vec![vec![e.mem.u32(reader + 0x18), 8, 0]]
        );
        assert_eq!(
            calls(&e, OBJ3 + 8),
            vec![vec![
                e.mem.u32(reader + 0x18),
                0x100,
                4,
                1,
                calls(&e, OBJ3 + 8)[0][4]
            ]]
        );
        assert_eq!(calls(&e, OBJ + 0x30), vec![vec![sample, 4]]);
        assert_eq!(e.call(0x009f_d970, &args![this, 0u32]).u32(), E_POINTER);
        // A failed time read is returned.
        e.register(OBJ + 0x14, |_, _| ret(E_FAIL));
        assert_eq!(e.call(0x009f_d970, &args![this, sample]).u32(), E_FAIL);
    }

    #[test]
    fn wait_sample_009fdaf0_sets_the_length() {
        let mut e = rig();
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| ret(0));
        let reader = e.mem.alloc(0x90);
        let l = reader + 0x50;
        e.call(0x009f_d120, &args![l, 0u32]);
        let request = e.mem.alloc(0x28);
        e.mem.set_u32(request + 0x24, 0);
        e.mem.set_u32(request + 0x14, 0x20);
        e.register(OBJ + 0x30, noop);
        let sample = object(&mut e, OBJ, 8);
        e.mem.set_u32(request + 0x1c, sample);
        e.mem.set_u32(request + 0x20, 0x77);
        e.call(0x009f_d270, &args![l, request]);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0xa4, reader);
        let out = e.mem.alloc(4);
        let user = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_daf0, &args![this, 5u32, out, user]).u32(), 0);
        assert_eq!(e.mem.u32(out), sample);
        assert_eq!(e.mem.u32(user), 0x77);
        assert_eq!(calls(&e, OBJ + 0x30), vec![vec![sample, 0x20]]);
        assert_eq!(
            e.call(0x009f_daf0, &args![this, 5u32, 0u32, user]).u32(),
            E_POINTER
        );
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| ret(0x102));
        assert_eq!(
            e.call(0x009f_daf0, &args![this, 5u32, out, user]).u32(),
            VFW_E_TIMEOUT
        );
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn forward_009fdb60_to_the_queued_read() {
        let mut e = rig();
        let (this, reader) = with_reader(&mut e, 1);
        e.register(OBJ3 + 0x14, noop);
        e.register(OBJ3 + 0x18, noop);
        e.register(OBJ3 + 4, |_, _| ret(0));
        e.register(OBJ3 + 8, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_db60, &args![this, 0u32, 0u32, 4u32, 0x100u32])
                .u32(),
            1
        );
        let held = e.mem.u32(reader + 0x18);
        assert_eq!(calls(&e, OBJ3 + 4), vec![vec![held, 0, 0]]);
    }

    #[test]
    fn forward_009fdb90_to_the_total() {
        let mut e = rig();
        let (this, _) = with_reader(&mut e, 1);
        e.register(OBJ3 + 0xc, |_, _| Ret {
            eax: 5,
            edx: 6,
            ..Ret::default()
        });
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x009f_db90, &args![this, out, 0x30u32]).u32(), 0);
        assert_eq!(e.mem.u64(out), 0x6_0000_0005);
    }

    /// A reader made by its constructor, with doubles for the events.
    fn reader(e: &mut Engine) -> u32 {
        e.register(CREATE_EVENT_A, |_, _| ret(0x100));
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| ret(0));
        let r = e.mem.alloc(0x90);
        e.call(0x009f_e1d0, &args![r, 0x900u32]);
        r
    }

    #[test]
    fn forward_009fdbb0_to_the_flush() {
        let mut e = rig();
        let r = reader(&mut e);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0xa4, r);
        assert_eq!(e.call(0x009f_dbb0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(r + 0x34), 1);
    }

    #[test]
    fn forward_009fdbd0_to_the_resume() {
        let mut e = rig();
        let r = reader(&mut e);
        e.mem.set_u32(r + 0x34, 1);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0xa4, r);
        assert_eq!(e.call(0x009f_dbd0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(r + 0x34), 0);
        assert_eq!(calls(&e, RESET_EVENT).len(), 1);
    }

    #[test]
    fn forward_009fdbf0_to_the_held_object() {
        let mut e = rig();
        e.register(OBJ + 0x24, |_, _| ret(9));
        let held = object(&mut e, OBJ, 8);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x8c, held);
        assert_eq!(e.call(0x009f_dbf0, &args![this, 1u32, 2u32]).u32(), 9);
        assert_eq!(calls(&e, OBJ + 0x24), vec![vec![held, 1, 2]]);
    }

    #[test]
    fn reader_filter_destructor_009fdc10() {
        let mut e = rig();
        e.set_global(OBJECT_COUNT, 5u32);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this + 0x3c, 0x3c3c);
        e.call(0x009f_dc10, &args![this]);
        assert_eq!(e.mem.u32(this), ASYNC_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), ASYNC_READER_VTABLE_C);
        assert_eq!(e.mem.u32(this + 0x10), ASYNC_READER_VTABLE_10);
        assert_eq!(e.mem.u32(this + 0xf0), ASYNC_OUTPUT_PIN_VTABLE);
        assert_eq!(
            calls(&e, DELETE_CRITICAL_SECTION),
            vec![
                vec![this + 0x68 + 0x1c],
                vec![this + 0x68],
                vec![this + 0x50]
            ]
        );
        assert!(calls(&e, OPERATOR_DELETE).contains(&vec![0x3c3c]));
        // The pin and the filter each drop the object count once.
        assert_eq!(e.global::<u32>(OBJECT_COUNT), 3);
    }

    #[test]
    fn pin_at_index_009fdcb0() {
        let mut e = rig();
        e.register(OBJ + 0x18, |_, _| ret(1));
        let this = object(&mut e, OBJ, 0x100);
        assert_eq!(e.call(0x009f_dcb0, &args![this, 0u32]).u32(), this + 0xf8);
        assert_eq!(e.call(0x009f_dcb0, &args![this, 1u32]).u32(), 0);
        e.register(OBJ + 0x18, |_, _| ret(0));
        assert_eq!(e.call(0x009f_dcb0, &args![this, 0u32]).u32(), 0);
    }

    fn adjusted_test(addr: u32, slot: u32, extra: &[u32]) {
        let mut e = rig();
        e.register(OBJ + slot, |_, _| ret(6));
        let inner = object(&mut e, OBJ, 8);
        let base = e.mem.alloc(0x30);
        e.mem.set_u32(base + 0xc, inner);
        let mut words = vec![base + 0x14];
        words.extend_from_slice(extra);
        assert_eq!(e.call(addr, &words).u32(), 6);
        let mut expected = vec![inner];
        expected.extend_from_slice(extra);
        assert_eq!(calls(&e, OBJ + slot), vec![expected]);
    }

    #[test]
    fn adjusted_query_009fdce0() {
        adjusted_test(0x009f_dce0, 0, &[0x10, 0x20]);
    }

    #[test]
    fn adjusted_release_009fdd10() {
        adjusted_test(0x009f_dd10, 8, &[]);
    }

    #[test]
    fn adjusted_add_009fdd40() {
        adjusted_test(0x009f_dd40, 4, &[]);
    }

    #[test]
    fn output_pin_constructor_009fdd70() {
        let mut e = rig();
        e.mem.set_u16(NAME_STRING_01093ED4, 0x4f);
        e.mem.set_u16(NAME_STRING_01093ED4 + 2, 0);
        let this = e.mem.alloc(0xb0);
        assert_eq!(
            e.call(
                0x009f_dd70,
                &args![this, 0x11u32, 0x22u32, 0x33u32, 0x44u32]
            )
            .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), ASYNC_OUTPUT_PIN_VTABLE);
        assert_eq!(e.mem.u32(this + 8), ASYNC_OUTPUT_PIN_VTABLE_8);
        assert_eq!(e.mem.u32(this + 0x14), ASYNC_OUTPUT_PIN_VTABLE_14);
        assert_eq!(e.mem.u32(this + 0x18), ASYNC_OUTPUT_PIN_VTABLE_18);
        assert_eq!(e.mem.u32(this + 0xa0), 0x22);
        assert_eq!(e.mem.u32(this + 0xa4), 0x33);
        // The pin base: filter +0x28, lock +0x20, direction +0x1c (from +8).
        assert_eq!(e.mem.u32(this + 8 + 0x28), 0x22);
        assert_eq!(e.mem.u32(this + 8 + 0x20), 0x44);
        assert_eq!(e.mem.u32(this + 8 + 0x1c), 1);
        assert_eq!(read_wide(&e, e.mem.u32(this + 8 + 0x14)), "O");
    }

    #[test]
    fn reader_filter_constructor_009fddf0() {
        let mut e = rig();
        e.register(CREATE_EVENT_A, |_, _| ret(0x100));
        let this = e.mem.alloc(0x200);
        assert_eq!(
            e.call(
                0x009f_ddf0,
                &args![this, 0u32, 0x900u32, 0x700u32, 0x600u32]
            )
            .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), ASYNC_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), ASYNC_READER_VTABLE_C);
        assert_eq!(e.mem.u32(this + 0x10), ASYNC_READER_VTABLE_10);
        assert_eq!(e.mem.u32(this + 4), 0x900);
        assert_eq!(e.mem.u32(this + 0x28), CLASS_ID_01093D70);
        assert_eq!(e.mem.u32(this + 0x38), this + 0x50);
        assert_eq!(e.mem.u32(this + 0x68 + 0x18), 0x700);
        assert_eq!(e.mem.u32(this + 0xf0 + 0xa4), this + 0x68);
        assert_eq!(e.mem.u32(this + 0xf0 + 0xa0), this);
        assert_eq!(e.mem.u32(this + 0x1a0 + 0x20), 1);
        assert_eq!(
            calls(&e, INITIALIZE_CRITICAL_SECTION),
            vec![
                vec![this + 0x50],
                vec![this + 0x68],
                vec![this + 0x68 + 0x1c]
            ]
        );
    }

    #[test]
    fn request_fill_009fded0_swaps_words_five_and_six() {
        let mut e = rig();
        let r = e.mem.alloc(0x28);
        assert_eq!(
            e.call(
                0x009f_ded0,
                &args![r, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32]
            )
            .u32(),
            0
        );
        let words: Vec<u32> = (0..10).map(|i| e.mem.u32(r + 4 * i)).collect();
        assert_eq!(words, vec![1, 2, 3, 4, 6, 5, 7, 8, 9, VFW_E_TIMEOUT]);
    }

    /// A request whose +4 object has its slots at OBJ.
    fn request(e: &mut Engine) -> u32 {
        let target = object(e, OBJ, 8);
        let r = e.mem.alloc(0x28);
        e.call(
            0x009f_ded0,
            &args![r, 0u32, target, 8u32, 0u32, 0x10u32, 1u32, 0x100u32, 0u32, 0u32],
        );
        e.register(OBJ + 0x14, noop);
        e.register(OBJ + 0x18, noop);
        r
    }

    #[test]
    fn request_run_009fdf20() {
        let mut e = rig();
        let r = request(&mut e);
        e.register(OBJ + 4, |_, _| ret(0));
        e.register(OBJ + 8, |e, a| {
            e.mem.set_u32(a[4], 0xc);
            ret(0)
        });
        assert_eq!(e.call(0x009f_df20, &args![r]).u32(), 1);
        assert_eq!(e.mem.u32(r + 0x14), 0xc);
        assert_eq!(calls(&e, OBJ + 0x14).len(), 1);
        assert_eq!(calls(&e, OBJ + 0x18).len(), 1);
        // Same length: result 0.
        assert_eq!(e.call(0x009f_df20, &args![r]).u32(), 0);
        // 0x40000 with a sample: the sample's +0x40 runs, result 0 then 1.
        e.register(OBJ2 + 0x40, noop);
        let sample = object(&mut e, OBJ2, 8);
        e.mem.set_u32(r + 0x1c, sample);
        e.register(OBJ + 8, |e, a| {
            e.mem.set_u32(a[4], 0x20);
            ret(0x4_0000)
        });
        assert_eq!(e.call(0x009f_df20, &args![r]).u32(), 1);
        assert_eq!(calls(&e, OBJ2 + 0x40), vec![vec![sample, 1]]);
        // A failed seek is returned.
        e.register(OBJ + 4, |_, _| ret(E_FAIL));
        assert_eq!(e.call(0x009f_df20, &args![r]).u32(), E_FAIL);
        assert_eq!(calls(&e, OBJ + 0x18).len(), 4);
    }

    #[test]
    fn aligned_read_009fdfe0() {
        let mut e = rig();
        let (_, reader) = with_reader(&mut e, 4);
        e.register(OBJ3 + 0x14, noop);
        e.register(OBJ3 + 0x18, noop);
        e.register(OBJ3 + 4, |_, _| ret(0));
        e.register(OBJ3 + 8, |e, a| {
            e.mem.set_u32(a[4], 3);
            ret(0)
        });
        let out = e.mem.alloc(4);
        assert_eq!(
            e.call(
                0x009f_dfe0,
                &args![reader, 8u32, 0u32, 4u32, 0x100u32, out, 0x55u32]
            )
            .u32(),
            1
        );
        assert_eq!(e.mem.u32(out), 3);
        let held = e.mem.u32(reader + 0x18);
        assert_eq!(calls(&e, OBJ3 + 4), vec![vec![held, 8, 0]]);
        assert_eq!(calls(&e, OBJ3 + 8)[0][..4], [held, 0x100, 4, 1]);
        for case in [[9u32, 4, 0x100], [8, 5, 0x100], [8, 4, 0x101]] {
            assert_eq!(
                e.call(
                    0x009f_dfe0,
                    &args![reader, case[0], 0u32, case[1], case[2], out, 0u32]
                )
                .u32(),
                VFW_E_BADALIGN
            );
        }
        assert_eq!(
            e.call(
                0x009f_dfe0,
                &args![reader, 8u32, 0u32, 4u32, 0x100u32, 0u32, 0u32]
            )
            .u32(),
            E_POINTER
        );
    }

    #[test]
    fn total_009fe090_stores_both_words() {
        let mut e = rig();
        let (_, reader) = with_reader(&mut e, 1);
        e.register(OBJ3 + 0xc, |_, _| Ret {
            eax: 1,
            edx: 2,
            ..Ret::default()
        });
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x009f_e090, &args![reader, out, 0x40u32]).u32(), 0);
        assert_eq!(e.mem.u64(out), 0x2_0000_0001);
        let held = e.mem.u32(reader + 0x18);
        assert_eq!(calls(&e, OBJ3 + 0xc), vec![vec![held, 0x40]]);
        assert_eq!(
            e.call(0x009f_e090, &args![reader, 0u32, 0u32]).u32(),
            E_POINTER
        );
    }

    #[test]
    fn stop_worker_009fe0c0() {
        let mut e = rig();
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| ret(0));
        let r = e.mem.alloc(0x90);
        e.mem.set_u32(r + 0x7c, 0x7c);
        assert_eq!(e.call(0x009f_e0c0, &args![r]).u32(), 0);
        assert!(calls(&e, WAIT_FOR_SINGLE_OBJECT).is_empty());
        e.mem.set_u32(r + 0x80, 0x80);
        assert_eq!(e.call(0x009f_e0c0, &args![r]).u32(), 0);
        assert_eq!(calls(&e, SET_EVENT), vec![vec![0x7c], vec![0x7c]]);
        assert_eq!(
            calls(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x80, 0xffff_ffff]]
        );
        assert_eq!(calls(&e, CLOSE_HANDLE), vec![vec![0x80]]);
        assert_eq!(e.mem.u32(r + 0x80), 0);
    }

    #[test]
    fn queued_read_009fe100_aligned_or_not() {
        let mut e = rig();
        let (_, reader) = with_reader(&mut e, 4);
        e.register(OBJ3 + 0x14, noop);
        e.register(OBJ3 + 0x18, noop);
        e.register(OBJ3 + 4, |_, _| ret(0));
        e.register(OBJ3 + 8, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_e100, &args![reader, 8u32, 0u32, 4u32, 0x100u32])
                .u32(),
            1
        );
        // The aligned path passes mode 1, the other mode 0.
        assert_eq!(calls(&e, OBJ3 + 8)[0][3], 1);
        assert_eq!(
            e.call(0x009f_e100, &args![reader, 9u32, 0u32, 4u32, 0x100u32])
                .u32(),
            1
        );
        assert_eq!(calls(&e, OBJ3 + 8)[1][3], 0);
    }

    #[test]
    fn alignment_009fe1a0() {
        let mut e = rig();
        let (_, reader) = with_reader(&mut e, 0x20);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_e1a0, &args![reader, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 0x20);
        assert_eq!(e.call(0x009f_e1a0, &args![reader, 0u32]).u32(), E_POINTER);
    }

    #[test]
    fn reader_constructor_009fe1d0() {
        let mut e = rig();
        let r = reader(&mut e);
        assert_eq!(e.mem.u32(r + 0x18), 0x900);
        assert_eq!(e.mem.u32(r + 0x38 + 0xc), 10);
        assert_eq!(e.mem.u32(r + 0x50 + 0xc), 10);
        for at in [0x68, 0x6c, 0x78, 0x7c] {
            assert_eq!(e.mem.u32(r + at), 0x100);
        }
        assert_eq!(
            calls(&e, CREATE_EVENT_A)
                .iter()
                .map(|a| a[1])
                .collect::<Vec<_>>(),
            vec![1, 1, 0, 1]
        );
        assert_eq!(
            calls(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![r], vec![r + 0x1c]]
        );
    }

    #[test]
    fn resume_009fe2a0_sets_or_resets_the_event() {
        let mut e = rig();
        let r = reader(&mut e);
        e.mem.set_u32(r + 0x6c, 0x6c);
        e.mem.set_u32(r + 0x58, 1);
        e.mem.set_u32(r + 0x34, 1);
        assert_eq!(e.call(0x009f_e2a0, &args![r]).u32(), 0);
        assert_eq!(calls(&e, SET_EVENT), vec![vec![0x6c]]);
        assert_eq!(e.mem.u32(r + 0x34), 0);
        e.mem.set_u32(r + 0x58, 0);
        e.call(0x009f_e2a0, &args![r]);
        assert_eq!(calls(&e, RESET_EVENT), vec![vec![0x6c]]);
    }

    #[test]
    fn take_queued_009fe2f0() {
        let mut e = rig();
        let r = reader(&mut e);
        e.mem.set_u32(r + 0x68, 0x68);
        e.call(0x009f_d270, &args![r + 0x38, 0x11u32]);
        e.call(0x009f_d270, &args![r + 0x38, 0x22u32]);
        assert_eq!(e.call(0x009f_e2f0, &args![r]).u32(), 0x11);
        assert!(calls(&e, RESET_EVENT).is_empty());
        assert_eq!(e.call(0x009f_e2f0, &args![r]).u32(), 0x22);
        assert_eq!(calls(&e, RESET_EVENT), vec![vec![0x68]]);
    }

    #[test]
    fn take_finished_009fe370() {
        let mut e = rig();
        let r = reader(&mut e);
        e.mem.set_u32(r + 0x6c, 0x6c);
        e.call(0x009f_d270, &args![r + 0x50, 0x11u32]);
        e.mem.set_u32(r + 0x34, 1);
        assert_eq!(e.call(0x009f_e370, &args![r]).u32(), 0x11);
        // Flushing and nobody waiting: the event stays set.
        assert!(calls(&e, RESET_EVENT).is_empty());
        e.mem.set_u32(r + 0x34, 0);
        assert_eq!(e.call(0x009f_e370, &args![r]).u32(), 0);
        assert_eq!(calls(&e, RESET_EVENT), vec![vec![0x6c]]);
    }

    #[test]
    fn wait_finished_009fe570_checks_the_position() {
        let mut e = rig();
        let r = reader(&mut e);
        e.register(OBJ + 0xc, |_, _| Ret {
            eax: 0x18,
            edx: 1,
            ..Ret::default()
        });
        let held = object(&mut e, OBJ, 8);
        e.mem.set_u32(r + 0x18, held);
        let req = e.mem.alloc(0x28);
        e.mem.set_u32(req + 8, 0x10);
        e.mem.set_u32(req + 0xc, 1);
        e.mem.set_u32(req + 0x14, 8);
        e.mem.set_u32(req + 0x1c, 0x51);
        e.mem.set_u32(req + 0x20, 0x52);
        e.mem.set_u32(req + 0x24, 1);
        e.call(0x009f_d270, &args![r + 0x50, req]);
        let outs = [e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(4)];
        assert_eq!(
            e.call(0x009f_e570, &args![r, 9u32, outs[0], outs[1], outs[2]])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(outs[0]), 0x51);
        assert_eq!(e.mem.u32(outs[1]), 0x52);
        assert_eq!(e.mem.u32(outs[2]), 8);
        assert!(calls(&e, OPERATOR_DELETE).contains(&vec![req]));
        assert_eq!(calls(&e, OBJ + 0xc), vec![vec![held, 0]]);
        // A mismatching position is a failure.
        e.mem.set_u32(req + 0x14, 9);
        e.call(0x009f_d270, &args![r + 0x50, req]);
        assert_eq!(
            e.call(0x009f_e570, &args![r, 9u32, outs[0], outs[1], outs[2]])
                .u32(),
            E_FAIL
        );
        // Nothing finished while flushing: wrong state.
        e.mem.set_u32(r + 0x34, 1);
        assert_eq!(
            e.call(0x009f_e570, &args![r, 9u32, outs[0], outs[1], outs[2]])
                .u32(),
            VFW_E_WRONG_STATE
        );
        assert_eq!(e.mem.u32(outs[0]), 0);
        assert_eq!(
            e.call(0x009f_e570, &args![r, 9u32, outs[0], 0u32, outs[2]])
                .u32(),
            E_POINTER
        );
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| ret(0x102));
        assert_eq!(
            e.call(0x009f_e570, &args![r, 9u32, outs[0], outs[1], outs[2]])
                .u32(),
            VFW_E_TIMEOUT
        );
    }

    #[test]
    fn flush_009fe690_completes_queued_and_waits_for_running() {
        let mut e = rig();
        let r = reader(&mut e);
        e.register(REQUEST_FINISH_009FE3F0, |_, _| ret(0));
        e.call(0x009f_d270, &args![r + 0x38, 0x11u32]);
        e.mem.set_u32(r + 0x70, 1);
        e.register_double(WAIT_FOR_SINGLE_OBJECT, move |e, _| {
            e.mem.set_u32(r + 0x70, 0);
            ret(0)
        });
        assert_eq!(e.call(0x009f_e690, &args![r]).u32(), 0);
        assert_eq!(calls(&e, REQUEST_FINISH_009FE3F0), vec![vec![r, 0x11]]);
        assert_eq!(
            calls(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x100, 0xffff_ffff]]
        );
        assert_eq!(e.mem.u32(r + 0x34), 1);
        assert_eq!(e.mem.u32(r + 0x74), 0);
        assert_eq!(calls(&e, SET_EVENT), vec![vec![0x100]]);
        assert_eq!(
            calls(&e, ENTER_CRITICAL_SECTION).len(),
            calls(&e, LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn reader_destructor_009fe870_deletes_what_is_left() {
        let mut e = rig();
        let r = reader(&mut e);
        e.register(LIST_GET_NEXT, |e, a| {
            let node = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], e.mem.u32(node + 4));
            ret(e.mem.u32(node + 8))
        });
        e.call(0x009f_d270, &args![r + 0x50, 0x5150u32]);
        e.call(0x009f_e870, &args![r]);
        let deleted = calls(&e, OPERATOR_DELETE);
        assert_eq!(deleted[0], vec![0x5150]);
        assert_eq!(calls(&e, CLOSE_HANDLE).len(), 4);
        assert_eq!(
            calls(&e, DELETE_CRITICAL_SECTION),
            vec![vec![r + 0x1c], vec![r]]
        );
        assert_eq!(e.mem.u32(r + 0x50), 0);
    }

    #[test]
    fn library_start_009fea2a_fills_the_table() {
        let mut e = rig();
        e.map(0x011a_6000, 0x1000);
        e.set_global(LIBRARY_TABLE, 0x9fea24u32);
        e.set_global(LIBRARY_TABLE + 4, 0x31u32);
        e.set_global(LIBRARY_TABLE + 8, 0x9fea24u32);
        e.set_global(LIBRARY_TABLE + 12, 0x32u32);
        e.register(LOAD_LIBRARY_A, |_, _| ret(0x1000_0000));
        e.register(GET_PROC_ADDRESS, |_, _| ret(OBJ));
        e.register(OBJ, |_, a| {
            ret(match a[0] {
                LIBRARY_FIRST_ID => OBJ2,
                0x31 => 0x5555,
                _ => 0,
            })
        });
        e.register(OBJ2, |_, _| ret(0));
        assert_eq!(e.call(0x009f_ea2a, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(LIBRARY_READY), 1);
        assert_eq!(e.global::<u32>(LIBRARY_FIRST_CELL), 0xffff_fffd);
        assert_eq!(e.global::<u32>(LIBRARY_TABLE), 0x5555);
        assert_eq!(e.global::<u32>(LIBRARY_TABLE + 8), 0x9fea24);
        assert_eq!(calls(&e, LOAD_LIBRARY_A), vec![vec![LIBRARY_NAME]]);
        assert_eq!(
            calls(&e, GET_PROC_ADDRESS),
            vec![vec![0x1000_0000, LIBRARY_PROC_NAME]]
        );
        // Done: nothing runs again.
        assert_eq!(e.call(0x009f_ea2a, &args![]).u32(), 0);
        assert_eq!(calls(&e, LOAD_LIBRARY_A).len(), 1);
        // A negative start-up result gives status -1, a failed load -2.
        e.set_global(LIBRARY_READY, 0u32);
        e.register(OBJ2, |_, _| ret(E_FAIL));
        assert_eq!(e.call(0x009f_ea2a, &args![]).u32(), 0xffff_ffff);
        e.register(LOAD_LIBRARY_A, |_, _| ret(0));
        assert_eq!(e.call(0x009f_ea2a, &args![]).u32(), 0xffff_fffe);
        assert_eq!(e.global::<u32>(LIBRARY_READY), 0);
    }

    #[test]
    fn library_cell_009febd8_forwards_both_words() {
        let mut e = rig();
        e.map(0x011a_6000, 0x1000);
        e.set_global(LIBRARY_CELL_009FEBD8, OBJ);
        e.register(OBJ, |_, _| ret(0x44));
        assert_eq!(e.call(0x009f_ebd8, &args![1u32, 0x2000u32]).u32(), 0x44);
        assert_eq!(calls(&e, OBJ), vec![vec![1, 0x2000]]);
    }
}

#[cfg(test)]
mod n078_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// Doubles over the imports and CRT helpers, the ids mapped (each id's
    /// words are its address xor 0..3), the call log on.
    fn rig() -> Engine {
        let mut e = Engine::new();
        for slot in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            INITIALIZE_CRITICAL_SECTION,
            DELETE_CRITICAL_SECTION,
            CO_TASK_MEM_FREE,
            OPERATOR_DELETE,
            CLOSE_HANDLE,
            SET_EVENT,
            RESET_EVENT,
            RELEASE_SEMAPHORE,
            VIRTUAL_FREE,
            CO_INITIALIZE,
            CO_FREE_UNUSED_LIBRARIES,
            CO_UNINITIALIZE,
            FREE_LIBRARY,
        ] {
            e.register(slot, noop);
        }
        e.register(CO_TASK_MEM_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
        e.register(HEAD_POSITION, |e, a| ret(e.mem.u32(a[0])));
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(VIRTUAL_ALLOC, |e, a| ret(e.mem.alloc(a[1])));
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            ret(a[0])
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            ret(v)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            ret(v)
        });
        e.map(0x0107_c000, 0x1000);
        e.map(0x0109_3000, 0x1000);
        for id in [
            INTERFACE_ID_01093F04,
            INTERFACE_ID_01093F14,
            INTERFACE_ID_01093C2C,
            INTERFACE_ID_01093C1C,
            INTERFACE_ID_01093BDC,
            INTERFACE_ID_01093BCC,
            INTERFACE_ID_01093BBC,
            INTERFACE_ID_01093BAC,
            INTERFACE_ID_01093B8C,
            CLASS_ID_0107CB08,
        ] {
            for i in 0..4 {
                e.mem.set_u32(id + 4 * i, id ^ i);
            }
        }
        e.map(0x011f_3000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A zeroed object of `size` bytes whose vtable slot at byte offset `s`
    /// (up to 0x7c) points at `base + s`.
    fn object(e: &mut Engine, base: u32, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for s in (0..0x80).step_by(4) {
            e.mem.set_u32(vtable + s, base + s);
        }
        let this = e.mem.alloc(size);
        e.mem.set_u32(this, vtable);
        this
    }

    /// A heap copy of the 16-byte id at `id`.
    fn id_copy(e: &mut Engine, id: u32) -> u32 {
        let copy = e.mem.alloc(0x10);
        for i in 0..4 {
            let w = e.mem.u32(id + 4 * i);
            e.mem.set_u32(copy + 4 * i, w);
        }
        copy
    }

    /// A heap wide string.
    fn wide(e: &mut Engine, text: &str) -> u32 {
        let units: Vec<u16> = text.encode_utf16().collect();
        let at = e.mem.alloc(2 * units.len() as u32 + 2);
        for (i, u) in units.iter().enumerate() {
            e.mem.set_u16(at + 2 * i as u32, *u);
        }
        at
    }

    const OBJ: u32 = 0x7100_0000;
    const OBJ2: u32 = 0x7100_1000;
    const OBJ3: u32 = 0x7100_2000;

    #[test]
    fn trespass_init_load_game_009f98e0_resolves_both_words() {
        let mut e = rig();
        e.register(PACKAGE_INIT_LOAD_GAME, noop);
        e.register(FORM_LOOKUP, |_, a| ret(a[0] + 0x100));
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x88, 5);
        e.call(0x009f_98e0, &args![this, 0x77u32]);
        assert_eq!(e.mem.u32(this + 0x88), 0x105);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
        assert_eq!(calls(&e, PACKAGE_INIT_LOAD_GAME), vec![vec![this, 0x77]]);
        assert_eq!(calls(&e, FORM_LOOKUP), vec![vec![5]]);
    }

    #[test]
    fn media_type_test_009f9a50_checks_major_and_format_ids() {
        let mut e = rig();
        let media_type = e.mem.alloc(0x48);
        assert_eq!(e.call(0x009f_9a50, &args![media_type]).u32(), 0);
        for i in 0..4 {
            let w = e.mem.u32(INTERFACE_ID_01093F04 + 4 * i);
            e.mem.set_u32(media_type + 0x2c + 4 * i, w);
        }
        assert_eq!(e.call(0x009f_9a50, &args![media_type]).u32(), 1);
        let other = e.mem.alloc(0x48);
        for i in 0..4 {
            let w = e.mem.u32(INTERFACE_ID_01093F04 + 4 * i);
            e.mem.set_u32(other + 4 * i, w);
        }
        assert_eq!(e.call(0x009f_9a50, &args![other]).u32(), 1);
    }

    /// Two media types with ids 1/2/3 and a `size`-byte format block each.
    fn type_pair(e: &mut Engine, size: u32) -> (u32, u32) {
        let make = |e: &mut Engine| {
            let t = e.mem.alloc(0x48);
            for (at, v) in [(0u32, 1u32), (0x10, 2), (0x2c, 3)] {
                e.mem.set_u32(t + at, v);
            }
            let block = e.mem.alloc(size + 4);
            for i in 0..size {
                e.mem.set_u8(block + i, i as u8);
            }
            e.mem.set_u32(t + 0x40, size);
            e.mem.set_u32(t + 0x44, block);
            t
        };
        (make(e), make(e))
    }

    #[test]
    fn media_type_match_009f9a80_compares_ids_sizes_and_blocks() {
        let mut e = rig();
        let (a, b) = type_pair(&mut e, 6);
        assert_eq!(e.call(0x009f_9a80, &args![a, b]).u32(), 1);
        // The tail byte differs.
        let block = e.mem.u32(b + 0x44);
        e.mem.set_u8(block + 5, 0x55);
        assert_eq!(e.call(0x009f_9a80, &args![a, b]).u32(), 0);
        // A word differs.
        e.mem.set_u8(block + 5, 5);
        e.mem.set_u8(block + 1, 0x55);
        assert_eq!(e.call(0x009f_9a80, &args![a, b]).u32(), 0);
        e.mem.set_u8(block + 1, 1);
        // The other's format id is the wildcard: 1 without the block test.
        for i in 0..4 {
            let w = e.mem.u32(INTERFACE_ID_01093F04 + 4 * i);
            e.mem.set_u32(b + 0x2c + 4 * i, w);
        }
        e.mem.set_u32(b + 0x40, 99);
        assert_eq!(e.call(0x009f_9a80, &args![a, b]).u32(), 1);
        // Sizes differ / both zero.
        let (a, b) = type_pair(&mut e, 4);
        e.mem.set_u32(b + 0x40, 3);
        assert_eq!(e.call(0x009f_9a80, &args![a, b]).u32(), 0);
        e.mem.set_u32(a + 0x40, 0);
        e.mem.set_u32(b + 0x40, 0);
        assert_eq!(e.call(0x009f_9a80, &args![a, b]).u32(), 1);
        // Subtype differs.
        e.mem.set_u32(b + 0x10, 9);
        assert_eq!(e.call(0x009f_9a80, &args![a, b]).u32(), 0);
        // Major differs.
        e.mem.set_u32(b + 0x10, 2);
        e.mem.set_u32(b, 9);
        assert_eq!(e.call(0x009f_9a80, &args![a, b]).u32(), 0);
    }

    #[test]
    fn media_type_duplicate_009f9cd0_frees_a_failed_copy() {
        let mut e = rig();
        let source = e.mem.alloc(0x48);
        e.mem.set_u32(source + 0x10, 0x42);
        let copy = e.call(0x009f_9cd0, &args![source]).u32();
        assert_ne!(copy, 0);
        assert_eq!(e.mem.u32(copy + 0x10), 0x42);
        // The format block's allocation fails: the block is freed.
        e.mem.set_u32(source + 0x40, 8);
        let first = Rc::new(Cell::new(true));
        let f = first.clone();
        e.register_double(CO_TASK_MEM_ALLOC, move |e, a| {
            if f.replace(false) {
                ret(e.mem.alloc(a[0]))
            } else {
                ret(0)
            }
        });
        assert_eq!(e.call(0x009f_9cd0, &args![source]).u32(), 0);
        assert_eq!(calls(&e, CO_TASK_MEM_FREE).len(), 1);
        e.register(CO_TASK_MEM_ALLOC, |_, _| ret(0));
        assert_eq!(e.call(0x009f_9cd0, &args![source]).u32(), 0);
    }

    #[test]
    fn stream_time_009f9db0_subtracts_the_start() {
        let mut e = rig();
        let this = e.mem.alloc(0x28);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x009f_9db0, &args![this, out]).u32(), VFW_E_NO_CLOCK);
        let clock = object(&mut e, OBJ, 8);
        e.mem.set_u32(this + 0x18, clock);
        e.mem.set_u64(this + 0x20, 0x1_0000_0005);
        e.register(OBJ + 0xc, |e, a| {
            e.mem.set_u64(a[1], 0x2_0000_0003);
            ret(0)
        });
        assert_eq!(e.call(0x009f_9db0, &args![this, out]).u32(), 0);
        assert_eq!(e.mem.u64(out), 0x2_0000_0003 - 0x1_0000_0005);
        e.register(OBJ + 0xc, |_, _| ret(E_FAIL));
        assert_eq!(e.call(0x009f_9db0, &args![this, out]).u32(), E_FAIL);
    }

    #[test]
    fn not_implemented_009f9f40_and_009fa510() {
        let mut e = rig();
        assert_eq!(e.call(0x009f_9f40, &args![1u32, 2u32]).u32(), E_NOTIMPL);
        let words = [0u32; 8];
        assert_eq!(e.call(0x009f_a510, &words).u32(), E_NOTIMPL);
    }

    fn lookup_test(addr: u32, own: u32) {
        let mut e = rig();
        let this = object(&mut e, OBJ, 0x20);
        e.register(OBJ + 4, |_, _| ret(2));
        let out = e.mem.alloc(4);
        let id = id_copy(&mut e, own);
        assert_eq!(e.call(addr, &args![this, id, 0u32]).u32(), E_POINTER);
        assert_eq!(e.call(addr, &args![this, id, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), this);
        let id = id_copy(&mut e, INTERFACE_ID_01093F14);
        assert_eq!(e.call(addr, &args![this, id, out]).u32(), 0);
        let id = id_copy(&mut e, INTERFACE_ID_01093B8C);
        assert_eq!(e.call(addr, &args![this, id, out]).u32(), E_NOINTERFACE);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(calls(&e, OBJ + 4).len(), 2);
    }

    #[test]
    fn enum_pins_lookup_009f9fe0() {
        lookup_test(0x009f_9fe0, INTERFACE_ID_01093C2C);
    }

    #[test]
    fn enum_media_types_lookup_009fa120() {
        lookup_test(0x009f_a120, INTERFACE_ID_01093C1C);
    }

    #[test]
    fn reference_adds_009fa040_009fa180_009fa5d0() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0x14, 4);
        e.mem.set_u32(this + 0x10, 7);
        e.mem.set_u32(this + 0x44, 1);
        assert_eq!(e.call(0x009f_a040, &args![this]).u32(), 5);
        assert_eq!(e.call(0x009f_a180, &args![this]).u32(), 8);
        assert_eq!(e.call(0x009f_a5d0, &args![this]).u32(), 2);
    }

    fn enum_pins(e: &mut Engine) -> u32 {
        let filter = object(e, OBJ, 0x10);
        e.register(OBJ + 0x14, |_, _| ret(0x33));
        e.register(OBJ + 0x18, |_, _| ret(6));
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, filter);
        e.mem.set_u32(this + 0x4, 2);
        this
    }

    #[test]
    fn enum_pins_reset_009fa090_also_clears_the_list() {
        let mut e = rig();
        let this = enum_pins(&mut e);
        assert_eq!(e.call(0x009f_a090, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x33);
        assert_eq!(e.mem.u32(this + 0x8), 6);
        assert_eq!(e.mem.u32(this + 0x4), 0);
        assert_eq!(e.mem.u32(this + 0x18 + 8), 0);
    }

    #[test]
    fn enum_pins_refresh_009fa0d0() {
        let mut e = rig();
        let this = enum_pins(&mut e);
        e.mem.set_u32(this + 0x18, 0x1234);
        assert_eq!(e.call(0x009f_a0d0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x33);
        assert_eq!(e.mem.u32(this + 0x8), 6);
        assert_eq!(e.mem.u32(this + 0x4), 0);
        // The list is left alone.
        assert_eq!(e.mem.u32(this + 0x18), 0x1234);
    }

    #[test]
    fn enum_media_types_reset_009fa1d0() {
        let mut e = rig();
        let pin = object(&mut e, OBJ, 0x10);
        e.register(OBJ + 0x10, |_, _| ret(0x44));
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x8, pin);
        e.mem.set_u32(this + 0x4, 3);
        assert_eq!(e.call(0x009f_a1d0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x44);
    }

    #[test]
    fn query_009fa4a0_turns_failures_into_one() {
        let mut e = rig();
        let pin = object(&mut e, OBJ, 0x40);
        let this = pin + 0xc;
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_a4a0, &args![this, 0u32]).u32(), E_POINTER);
        e.register(OBJ + 0x20, |_, _| ret(7));
        assert_eq!(e.call(0x009f_a4a0, &args![this, out]).u32(), 7);
        assert_eq!(calls(&e, OBJ + 0x20), vec![vec![pin, out]]);
        e.register(OBJ + 0x20, |_, _| ret(E_FAIL));
        assert_eq!(e.call(0x009f_a4a0, &args![this, out]).u32(), 1);
    }

    #[test]
    fn locked_store_009fa4e0() {
        let mut e = rig();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x99);
        assert_eq!(e.call(0x009f_a4e0, &args![this, 0x55u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0x55);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![0x99]]);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![0x99]]);
    }

    #[test]
    fn store_009fa520_keeps_the_double_and_quiets_a_signalling_nan() {
        let mut e = rig();
        let this = e.mem.alloc(0x90);
        let rate = 1.5f64;
        assert_eq!(
            e.call(0x009f_a520, &args![this, 1u32, 2u32, 3u32, 4u32, rate])
                .u32(),
            0
        );
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 0x74 + 4 * i)).collect();
        assert_eq!(words, vec![1, 2, 3, 4]);
        assert_eq!(e.mem.u64(this + 0x84), 1.5f64.to_bits());
        let signalling = f64::from_bits(0x7ff0_0000_0000_0001);
        e.call(
            0x009f_a520,
            &args![this, 1u32, 2u32, 3u32, 4u32, signalling],
        );
        assert_eq!(e.mem.u64(this + 0x84), 0x7ff8_0000_0000_0001);
    }

    #[test]
    fn sample_destructor_009fa560_and_deleting_009fc460() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        e.call(0x009f_a560, &args![this]);
        assert_eq!(e.mem.u32(this), MEDIA_SAMPLE_VTABLE);
        assert!(calls(&e, CO_TASK_MEM_FREE).is_empty());
        let media_type = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0x3c, media_type);
        assert_eq!(e.call(0x009f_c460, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, CO_TASK_MEM_FREE), vec![vec![media_type]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.mem.set_u32(this + 0x3c, 0);
        e.call(0x009f_c460, &args![this, 0u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn sample_lookup_009fa580() {
        let mut e = rig();
        let this = object(&mut e, OBJ, 0x48);
        e.register(OBJ + 4, |_, _| ret(2));
        let out = e.mem.alloc(4);
        for own in [
            INTERFACE_ID_01093BDC,
            INTERFACE_ID_01093BCC,
            INTERFACE_ID_01093F14,
        ] {
            e.mem.set_u32(out, 0);
            let id = id_copy(&mut e, own);
            assert_eq!(e.call(0x009f_a580, &args![this, id, out]).u32(), 0);
            assert_eq!(e.mem.u32(out), this);
        }
        e.mem.set_u32(out, 0x77);
        let id = id_copy(&mut e, INTERFACE_ID_01093C1C);
        assert_eq!(
            e.call(0x009f_a580, &args![this, id, out]).u32(),
            E_NOINTERFACE
        );
        assert_eq!(e.mem.u32(out), 0x77);
    }

    #[test]
    fn sample_release_009fa5f0_returns_the_sample_at_zero() {
        let mut e = rig();
        let this = object(&mut e, OBJ, 0x48);
        let allocator = object(&mut e, OBJ2, 0x40);
        e.register(OBJ + 0x38, noop);
        e.register(OBJ2 + 0x20, noop);
        e.mem.set_u32(this + 0x18, allocator - 0xc);
        // Count 3: just decremented.
        e.mem.set_u32(this + 0x44, 3);
        assert_eq!(e.call(0x009f_a5f0, &args![this]).u32(), 2);
        assert!(calls(&e, OBJ2 + 0x20).is_empty());
        // Count 1: cleared without the interlocked call, type dropped, returned.
        e.mem.set_u32(this + 0x44, 1);
        e.mem.set_u32(this + 0x4, 0x18);
        e.mem.set_u32(this + 0x8, 5);
        e.mem.set_u32(this + 0x40, 6);
        assert_eq!(e.call(0x009f_a5f0, &args![this]).u32(), 0);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 1);
        assert_eq!(calls(&e, OBJ + 0x38), vec![vec![this, 0]]);
        assert_eq!(calls(&e, OBJ2 + 0x20), vec![vec![allocator, this]]);
        for at in [0x4u32, 0x8, 0x40] {
            assert_eq!(e.mem.u32(this + at), 0);
        }
        // Count 2: decremented to 1, nothing else.
        e.mem.set_u32(this + 0x44, 2);
        assert_eq!(e.call(0x009f_a5f0, &args![this]).u32(), 1);
        assert_eq!(calls(&e, OBJ2 + 0x20).len(), 1);
    }

    #[test]
    fn sample_buffer_009fa650_and_size_009fa670_and_length_009fa8a0() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x14, 0x200);
        e.mem.set_u32(this + 0x10, 0x80);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_a650, &args![this, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 0x1000);
        assert_eq!(e.call(0x009f_a670, &args![this]).u32(), 0x200);
        assert_eq!(e.call(0x009f_a8a0, &args![this]).u32(), 0x80);
    }

    #[test]
    fn sample_times_009fa680_and_009fa6f0() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        let start = e.mem.alloc(8);
        let end = e.mem.alloc(8);
        assert_eq!(
            e.call(0x009f_a680, &args![this, start, end]).u32(),
            VFW_E_SAMPLE_TIME_NOT_SET
        );
        // Start only.
        e.mem.set_u64(start, 0x1_ffff_ffff);
        assert_eq!(e.call(0x009f_a6f0, &args![this, start, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 0x10);
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        assert_eq!(
            e.call(0x009f_a680, &args![this, a, b]).u32(),
            VFW_S_NO_STOP_TIME
        );
        assert_eq!(e.mem.u64(a), 0x1_ffff_ffff);
        assert_eq!(e.mem.u64(b), 0x2_0000_0000);
        // Both.
        e.mem.set_u64(end, 0x3_0000_0000);
        assert_eq!(e.call(0x009f_a6f0, &args![this, start, end]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 0x110);
        assert_eq!(e.call(0x009f_a680, &args![this, a, b]).u32(), 0);
        assert_eq!(e.mem.u64(b), 0x3_0000_0000);
        // Start only again: 0x100 cleared.
        e.call(0x009f_a6f0, &args![this, start, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0x10);
        // None: both flags cleared.
        e.mem.set_u32(this + 4, 0x111);
        e.call(0x009f_a6f0, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0x1);
    }

    #[test]
    fn sample_media_times_009fa760_and_009fa7a0() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        let start = e.mem.alloc(8);
        let end = e.mem.alloc(8);
        assert_eq!(
            e.call(0x009f_a760, &args![this, start, end]).u32(),
            VFW_E_MEDIA_TIME_NOT_SET
        );
        e.mem.set_u64(start, 0x5_0000_0010);
        e.mem.set_u64(end, 0x5_0000_0004);
        assert_eq!(e.call(0x009f_a7a0, &args![this, start, end]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 0x20);
        // The low-word difference is negative: signed extension on read.
        assert_eq!(e.mem.u32(this + 0x38), 0xffff_fff4);
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        assert_eq!(e.call(0x009f_a760, &args![this, a, b]).u32(), 0);
        assert_eq!(e.mem.u64(a), 0x5_0000_0010);
        assert_eq!(e.mem.u64(b), 0x5_0000_0004);
        e.call(0x009f_a7a0, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn sample_flags_009fa7e0_to_009fa880() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        for (get, set, mask) in [
            (0x009f_a7e0u32, 0x009f_a800u32, 1u32),
            (0x009f_a860, 0x009f_a880, 2),
            (0x009f_a820, 0x009f_a840, 4),
        ] {
            assert_eq!(e.call(get, &args![this]).u32(), 1);
            assert_eq!(e.call(set, &args![this, 7u32]).u32(), 0);
            assert_eq!(e.mem.u32(this + 4) & mask, mask);
            assert_eq!(e.call(get, &args![this]).u32(), 0);
            e.call(set, &args![this, 0u32]);
            assert_eq!(e.mem.u32(this + 4) & mask, 0);
        }
    }

    #[test]
    fn sample_set_length_009fa8b0_is_signed_against_the_size() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0x14, 0x100);
        assert_eq!(
            e.call(0x009f_a8b0, &args![this, 0x101u32]).u32(),
            VFW_E_BUFFER_OVERFLOW
        );
        assert_eq!(e.call(0x009f_a8b0, &args![this, 0x100u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x100);
        assert_eq!(e.call(0x009f_a8b0, &args![this, -1i32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
    }

    #[test]
    fn sample_media_type_009fa8d0_and_009fa910() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 5);
        assert_eq!(e.call(0x009f_a8d0, &args![this, out]).u32(), 1);
        assert_eq!(e.mem.u32(out), 0);
        let media_type = e.mem.alloc(0x48);
        e.mem.set_u32(media_type + 0x10, 0x31);
        assert_eq!(e.call(0x009f_a910, &args![this, media_type]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 8);
        let held = e.mem.u32(this + 0x3c);
        assert_eq!(e.mem.u32(held + 0x10), 0x31);
        assert_eq!(e.call(0x009f_a8d0, &args![this, out]).u32(), 0);
        let copy = e.mem.u32(out);
        assert_ne!(copy, held);
        assert_eq!(e.mem.u32(copy + 0x10), 0x31);
        // Cleared again: the held copy is freed.
        assert_eq!(e.call(0x009f_a910, &args![this, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(calls(&e, CO_TASK_MEM_FREE), vec![vec![held]]);
        // A failed copy.
        e.register(CO_TASK_MEM_ALLOC, |_, _| ret(0));
        e.mem.set_u32(this + 4, 8);
        assert_eq!(
            e.call(0x009f_a910, &args![this, media_type]).u32(),
            E_OUTOFMEMORY
        );
        assert_eq!(e.mem.u32(this + 4), 0);
        e.mem.set_u32(this + 4, 8);
        assert_eq!(e.call(0x009f_a8d0, &args![this, out]).u32(), E_OUTOFMEMORY);
    }

    #[test]
    fn sample_properties_009fa960_copies_at_most_0x30_bytes() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        for (at, v) in [
            (0x4u32, 0x2bu32),
            (0x8, 0x11),
            (0xc, 0x1000),
            (0x10, 0x22),
            (0x14, 0x200),
            (0x20, 0x33),
            (0x24, 0x34),
            (0x28, 0x35),
            (0x2c, 0x36),
            (0x3c, 0x4444),
            (0x40, 0x55),
        ] {
            e.mem.set_u32(this + at, v);
        }
        let out = e.mem.alloc(0x40);
        assert_eq!(e.call(0x009f_a960, &args![this, 0u32, out]).u32(), 0);
        assert!(calls(&e, MEMCPY).is_empty());
        assert_eq!(
            e.call(0x009f_a960, &args![this, 4u32, 0u32]).u32(),
            E_POINTER
        );
        assert_eq!(e.call(0x009f_a960, &args![this, 0x40u32, out]).u32(), 0);
        let words: Vec<u32> = (0..12).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0x30, 0x11, 0x0b, 0x22, 0x33, 0x34, 0x35, 0x36, 0x55, 0x4444, 0x1000, 0x200]
        );
        // Without flag 8 the type is 0; a short size copies only that much.
        e.mem.set_u32(this + 4, 0);
        let short = e.mem.alloc(0x40);
        e.mem.set_u32(short + 4, 0x99);
        assert_eq!(e.call(0x009f_a960, &args![this, 4u32, short]).u32(), 0);
        assert_eq!(e.mem.u32(short), 4);
        assert_eq!(e.mem.u32(short + 4), 0x99);
    }

    /// A 0x30-byte property block.
    fn properties(e: &mut Engine, words: &[(u32, u32)]) -> u32 {
        let p = e.mem.alloc(0x30);
        e.mem.set_u32(p, 0x30);
        for &(at, v) in words {
            e.mem.set_u32(p + at, v);
        }
        p
    }

    #[test]
    fn sample_set_properties_009faa20_checks_then_stores() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x14, 0x200);
        e.mem.set_u32(this + 4, 0x20);
        assert_eq!(e.call(0x009f_aa20, &args![this, 3u32, 0u32]).u32(), 0);
        assert_eq!(
            e.call(0x009f_aa20, &args![this, 4u32, 0u32]).u32(),
            E_POINTER
        );
        // Bad flags, a foreign buffer, a length above the size.
        for words in [
            vec![(0x8u32, 0x200u32)],
            vec![(0x28, 0x2000)],
            vec![(0x2c, 0x100)],
            vec![(0xc, 0x201)],
        ] {
            let p = properties(&mut e, &words);
            assert_eq!(
                e.call(0x009f_aa20, &args![this, 0x30u32, p]).u32(),
                E_INVALIDARG
            );
        }
        // Flag 0x10 on a sample without it, below 0x20 bytes.
        let p = properties(&mut e, &[(0x8, 0x10)]);
        assert_eq!(
            e.call(0x009f_aa20, &args![this, 0x1cu32, p]).u32(),
            E_INVALIDARG
        );
        // A size above 0x30 in the block.
        let p = properties(&mut e, &[(0x0, 0x34)]);
        assert_eq!(
            e.call(0x009f_aa20, &args![this, 0x40u32, p]).u32(),
            E_INVALIDARG
        );
        // Flag 8 without a type.
        let p = properties(&mut e, &[(0x8, 0x8)]);
        assert_eq!(
            e.call(0x009f_aa20, &args![this, 0x30u32, p]).u32(),
            E_POINTER
        );
        // A full block with a type.
        let media_type = e.mem.alloc(0x48);
        let p = properties(
            &mut e,
            &[
                (0x4, 0x11),
                (0x8, 0x18),
                (0xc, 0x80),
                (0x10, 0x21),
                (0x14, 0x22),
                (0x18, 0x23),
                (0x1c, 0x24),
                (0x20, 0x25),
                (0x24, media_type),
                (0x2c, 0x200),
            ],
        );
        assert_eq!(e.call(0x009f_aa20, &args![this, 0x30u32, p]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 0x38);
        for (at, v) in [
            (0x8u32, 0x11u32),
            (0x10, 0x80),
            (0x20, 0x21),
            (0x24, 0x22),
            (0x28, 0x23),
            (0x2c, 0x24),
            (0x40, 0x25),
        ] {
            assert_eq!(e.mem.u32(this + at), v, "+{at:#x}");
        }
        let held = e.mem.u32(this + 0x3c);
        assert_ne!(held, 0);
        // A short block (8 bytes): only +8 changes; the held type keeps flag 8.
        let p = properties(&mut e, &[(0x0, 0x8), (0x4, 0x66)]);
        assert_eq!(e.call(0x009f_aa20, &args![this, 0x30u32, p]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x8), 0x66);
        assert_eq!(e.mem.u32(this + 0x10), 0x80);
        assert_eq!(e.mem.u32(this + 4), 0x38);
        // A failed copy.
        e.register(CO_TASK_MEM_ALLOC, |_, _| ret(0));
        let p = properties(&mut e, &[(0x8, 0x8), (0x24, media_type)]);
        assert_eq!(
            e.call(0x009f_aa20, &args![this, 0x30u32, p]).u32(),
            E_OUTOFMEMORY
        );
    }

    #[test]
    fn allocator_base_destructor_009fabb0() {
        let mut e = rig();
        let this = e.mem.alloc(0x64);
        e.call(0x009f_abb0, &args![this]);
        assert!(calls(&e, CLOSE_HANDLE).is_empty());
        e.register(OBJ + 8, noop);
        let held = object(&mut e, OBJ, 8);
        e.mem.set_u32(this + 0x30, 0x44);
        e.mem.set_u32(this + 0x58, held);
        e.mem.set_u32(OBJECT_COUNT, 3);
        e.call(0x009f_abb0, &args![this]);
        assert_eq!(calls(&e, CLOSE_HANDLE), vec![vec![0x44]]);
        assert_eq!(calls(&e, OBJ + 8), vec![vec![held]]);
        assert_eq!(
            calls(&e, DELETE_CRITICAL_SECTION),
            vec![vec![this + 0x10]; 2]
        );
        assert_eq!(e.mem.u32(OBJECT_COUNT), 2);
    }

    #[test]
    fn allocator_lookup_009fac30() {
        let mut e = rig();
        let this = e.mem.alloc(0x64);
        let interface = object(&mut e, OBJ, 4);
        e.mem.set_u32(this + 0xc, e.mem.u32(interface));
        e.register(OBJ + 4, |_, _| ret(1));
        let out = e.mem.alloc(4);
        let id = id_copy(&mut e, INTERFACE_ID_01093BBC);
        assert_eq!(e.call(0x009f_ac30, &args![this, id, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), this + 0xc);
        // The second id only with +0x5c set.
        let id = id_copy(&mut e, INTERFACE_ID_01093BAC);
        assert_eq!(
            e.call(0x009f_ac30, &args![this, id, out]).u32(),
            E_NOINTERFACE
        );
        e.mem.set_u32(this + 0x5c, 1);
        assert_eq!(e.call(0x009f_ac30, &args![this, id, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), this + 0xc);
    }

    /// A `CMemAllocator`-shaped object (its own vtable at +0 pointing at
    /// `OBJ3`, the interface's at +0xc at `OBJ2`); returns the interface.
    fn allocator(e: &mut Engine) -> u32 {
        let this = object(e, OBJ3, 0x64);
        let vtable = e.mem.alloc(0x80);
        for s in (0..0x80).step_by(4) {
            e.mem.set_u32(vtable + s, OBJ2 + s);
        }
        e.mem.set_u32(this + 0xc, vtable);
        this + 0xc
    }

    #[test]
    fn allocator_properties_009faca0() {
        let mut e = rig();
        let this = allocator(&mut e);
        for (at, v) in [(0x2cu32, 4u32), (0x34, 0x100), (0x38, 8), (0x3c, 2)] {
            e.mem.set_u32(this + at, v);
        }
        assert_eq!(e.call(0x009f_aca0, &args![this, 0u32]).u32(), E_POINTER);
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x009f_aca0, &args![this, out]).u32(), 0);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![4, 0x100, 8, 2]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![this + 4]]);
    }

    #[test]
    fn allocator_get_buffer_009fad00_takes_waits_or_fails() {
        let mut e = rig();
        let this = allocator(&mut e);
        let out = e.mem.alloc(4);
        assert_eq!(
            e.call(0x009f_ad00, &args![this, out, 0u32, 0u32, 0u32])
                .u32(),
            VFW_E_NOT_COMMITTED
        );
        e.mem.set_u32(this + 0x44, 1);
        // No free sample and flag 4: timeout, the waiting count grows.
        assert_eq!(
            e.call(0x009f_ad00, &args![this, out, 0u32, 0u32, 4u32])
                .u32(),
            VFW_E_TIMEOUT
        );
        assert_eq!(e.mem.u32(this + 0x28), 1);
        // A wait that frees a sample.
        let sample = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0x24, 0x88);
        let s = sample;
        let t = this;
        e.register_double(WAIT_FOR_SINGLE_OBJECT, move |e, _| {
            e.mem.set_u32(t + 0x1c, s);
            e.mem.set_u32(t + 0x20, 1);
            ret(0)
        });
        assert_eq!(
            e.call(0x009f_ad00, &args![this, out, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(out), sample);
        assert_eq!(e.mem.u32(sample + 0x44), 1);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x28), 2);
        assert_eq!(
            calls(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x88, 0xffff_ffff]]
        );
    }

    #[test]
    fn allocator_notify_009fada0_swaps_references() {
        let mut e = rig();
        let this = allocator(&mut e);
        let old = object(&mut e, OBJ, 8);
        let new = object(&mut e, OBJ, 8);
        e.register(OBJ + 4, noop);
        e.register(OBJ + 8, noop);
        e.mem.set_u32(this + 0x4c, old);
        assert_eq!(e.call(0x009f_ada0, &args![this, new]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x4c), new);
        assert_eq!(calls(&e, OBJ + 4), vec![vec![new]]);
        assert_eq!(calls(&e, OBJ + 8), vec![vec![old]]);
        assert_eq!(e.call(0x009f_ada0, &args![this, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
    }

    #[test]
    fn allocator_free_count_009fae30() {
        let mut e = rig();
        let this = allocator(&mut e);
        e.mem.set_u32(this + 0x2c, 10);
        e.mem.set_u32(this + 0x30, 4);
        e.mem.set_u32(this + 0x20, 1);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_ae30, &args![this, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 7);
    }

    #[test]
    fn allocator_wake_009fae70() {
        let mut e = rig();
        let this = e.mem.alloc(0x64);
        e.call(0x009f_ae70, &args![this]);
        assert!(calls(&e, RELEASE_SEMAPHORE).is_empty());
        e.mem.set_u32(this + 0x30, 0x77);
        e.mem.set_u32(this + 0x34, 3);
        e.call(0x009f_ae70, &args![this]);
        assert_eq!(calls(&e, RELEASE_SEMAPHORE), vec![vec![0x77, 3, 0]]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
    }

    #[test]
    fn allocator_commit_009fae90() {
        let mut e = rig();
        let this = allocator(&mut e);
        e.register(OBJ3 + 0x14, |_, _| ret(E_OUTOFMEMORY));
        e.register(OBJ2 + 4, noop);
        assert_eq!(e.call(0x009f_ae90, &args![this]).u32(), E_OUTOFMEMORY);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        e.register(OBJ3 + 0x14, |_, _| ret(0));
        assert_eq!(e.call(0x009f_ae90, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x44), 1);
        assert_eq!(calls(&e, OBJ3 + 0x14), vec![vec![this - 0xc]; 2]);
        assert_eq!(calls(&e, OBJ2 + 4), vec![vec![this]]);
        // Already committed: nothing.
        assert_eq!(e.call(0x009f_ae90, &args![this]).u32(), 0);
        assert_eq!(calls(&e, OBJ3 + 0x14).len(), 2);
        // A pending decommit is cancelled instead.
        e.mem.set_u32(this + 0x44, 0);
        e.mem.set_u32(this + 0x48, 1);
        assert_eq!(e.call(0x009f_ae90, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x44), 1);
        assert_eq!(calls(&e, OBJ3 + 0x14).len(), 2);
    }

    #[test]
    fn allocator_decommit_009faf50() {
        let mut e = rig();
        let this = allocator(&mut e);
        e.register(OBJ3 + 0x10, noop);
        e.register(OBJ2 + 8, noop);
        assert_eq!(e.call(0x009f_af50, &args![this]).u32(), 0);
        assert!(calls(&e, OBJ3 + 0x10).is_empty());
        // Buffers out: pending.
        e.mem.set_u32(this + 0x44, 1);
        e.mem.set_u32(this + 0x20, 1);
        e.mem.set_u32(this + 0x30, 2);
        assert_eq!(e.call(0x009f_af50, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x48), 1);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert!(calls(&e, OBJ3 + 0x10).is_empty());
        // All back: freed and released.
        e.mem.set_u32(this + 0x20, 2);
        e.mem.set_u32(this + 0x28, 1);
        e.mem.set_u32(this + 0x24, 5);
        assert_eq!(e.call(0x009f_af50, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(calls(&e, OBJ3 + 0x10), vec![vec![this - 0xc]]);
        assert_eq!(calls(&e, OBJ2 + 8), vec![vec![this]]);
        assert_eq!(calls(&e, RELEASE_SEMAPHORE), vec![vec![5, 1, 0]]);
    }

    #[test]
    fn allocator_size_check_009fb020() {
        let mut e = rig();
        let this = e.mem.alloc(0x64);
        assert_eq!(e.call(0x009f_b020, &args![this]).u32(), VFW_E_SIZENOTSET);
        e.mem.set_u32(this + 0x38, 1);
        e.mem.set_u32(this + 0x40, 1);
        e.mem.set_u32(this + 0x44, 1);
        assert_eq!(e.call(0x009f_b020, &args![this]).u32(), 1);
        e.mem.set_u32(this + 0x4c, 1);
        assert_eq!(e.call(0x009f_b020, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x44, 0xffff_ffff);
        assert_eq!(e.call(0x009f_b020, &args![this]).u32(), VFW_E_SIZENOTSET);
    }

    #[test]
    fn allocator_set_properties_009fb050() {
        let mut e = rig();
        let this = allocator(&mut e);
        e.register(GET_SYSTEM_INFO, |e, a| {
            e.mem.set_u32(a[0] + 0x1c, 0x1_0000);
            ret(0)
        });
        let actual = e.mem.alloc(0x10);
        let request = e.mem.alloc(0x10);
        for (at, v) in [(0x0u32, 3u32), (0x4, 0x101), (0x8, 0x10), (0xc, 4)] {
            e.mem.set_u32(request + at, v);
        }
        assert_eq!(
            e.call(0x009f_b050, &args![this, request, 0u32]).u32(),
            E_POINTER
        );
        assert_eq!(e.call(0x009f_b050, &args![this, request, actual]).u32(), 0);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(actual + 4 * i)).collect();
        // 0x101 + 4 rounded up to 0x110, less the prefix.
        assert_eq!(words, vec![3, 0x10c, 0x10, 4]);
        assert_eq!(e.mem.u32(this + 0x34), 0x10c);
        assert_eq!(e.mem.u32(this + 0x2c), 3);
        assert_eq!(e.mem.u32(this + 0x40), 1);
        // Bad alignments, committed, buffers out.
        e.mem.set_u32(request + 8, 0x2_0000);
        assert_eq!(
            e.call(0x009f_b050, &args![this, request, actual]).u32(),
            VFW_E_BADALIGN
        );
        e.mem.set_u32(request + 8, 0);
        assert_eq!(
            e.call(0x009f_b050, &args![this, request, actual]).u32(),
            VFW_E_BADALIGN
        );
        e.mem.set_u32(request + 8, 0x10);
        e.mem.set_u32(this + 0x44, 1);
        assert_eq!(
            e.call(0x009f_b050, &args![this, request, actual]).u32(),
            VFW_E_ALREADY_COMMITTED
        );
        e.mem.set_u32(this + 0x44, 0);
        e.mem.set_u32(this + 0x30, 2);
        assert_eq!(
            e.call(0x009f_b050, &args![this, request, actual]).u32(),
            VFW_E_BUFFERS_OUTSTANDING
        );
        assert_eq!(
            calls(&e, ENTER_CRITICAL_SECTION).len(),
            calls(&e, LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn allocator_free_all_009fb170() {
        let mut e = rig();
        let this = e.mem.alloc(0x64);
        let a = object(&mut e, OBJ, 0x48);
        let b = object(&mut e, OBJ, 0x48);
        e.register(OBJ + 0x54, noop);
        e.mem.set_u32(this + 0x28, a);
        e.mem.set_u32(a + 0x1c, b);
        e.mem.set_u32(this + 0x2c, 2);
        e.mem.set_u32(this + 0x3c, 2);
        e.mem.set_u32(this + 0x60, 0x5000);
        e.call(0x009f_b170, &args![this]);
        assert_eq!(calls(&e, OBJ + 0x54), vec![vec![a, 1], vec![b, 1]]);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(calls(&e, VIRTUAL_FREE), vec![vec![0x5000, 0, 0x8000]]);
        e.call(0x009f_b170, &args![this]);
        assert_eq!(calls(&e, VIRTUAL_FREE).len(), 1);
    }

    #[test]
    fn allocator_destructor_009fb1c0_and_deleting_009fc580() {
        let mut e = rig();
        let this = e.mem.alloc(0x64);
        assert_eq!(e.call(0x009f_c580, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), MEM_ALLOCATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), MEM_ALLOCATOR_VTABLE_C);
        assert_eq!(calls(&e, DELETE_CRITICAL_SECTION), vec![vec![this + 0x10]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.call(0x009f_b1c0, &args![this]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn outer_thunks_009fb230_009fb250_009fb270() {
        let mut e = rig();
        let outer = object(&mut e, OBJ, 8);
        e.register(OBJ, |_, _| ret(0x10));
        e.register(OBJ + 4, |_, _| ret(0x14));
        e.register(OBJ + 8, |_, _| ret(0x18));
        let this = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(this - 8, outer);
        assert_eq!(e.call(0x009f_b230, &args![this, 1u32, 2u32]).u32(), 0x10);
        assert_eq!(calls(&e, OBJ), vec![vec![outer, 1, 2]]);
        assert_eq!(e.call(0x009f_b250, &args![this]).u32(), 0x14);
        assert_eq!(e.call(0x009f_b270, &args![this]).u32(), 0x18);
        assert_eq!(calls(&e, OBJ + 8), vec![vec![outer]]);
    }

    /// A setup block with one pin holding two type pairs; returns it.
    fn setup_block(e: &mut Engine) -> u32 {
        let class = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(class + 4 * i, 0xc0 + i);
        }
        let connects = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(connects + 4 * i, 0xd0 + i);
        }
        let major = e.mem.alloc(0x10);
        let minor = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(major + 4 * i, 0xe0 + i);
            e.mem.set_u32(minor + 4 * i, 0xf0 + i);
        }
        let types = e.mem.alloc(0x10);
        for i in 0..2 {
            e.mem.set_u32(types + 8 * i, major);
            e.mem.set_u32(types + 8 * i + 4, minor);
        }
        let pin = e.mem.alloc(0x24);
        for (at, v) in [(0x0u32, 0x501u32), (0x4, 1), (0x8, 2), (0xc, 3), (0x10, 4)] {
            e.mem.set_u32(pin + at, v);
        }
        e.mem.set_u32(pin + 0x14, connects);
        e.mem.set_u32(pin + 0x18, 0x502);
        e.mem.set_u32(pin + 0x1c, 2);
        e.mem.set_u32(pin + 0x20, types);
        let setup = e.mem.alloc(0x14);
        for (at, v) in [
            (0x0u32, class),
            (0x4, 0x503),
            (0x8, 0x600),
            (0xc, 1),
            (0x10, pin),
        ] {
            e.mem.set_u32(setup + at, v);
        }
        setup
    }

    #[test]
    fn register_009fb290_walks_pins_and_types() {
        let mut e = rig();
        let mapper = object(&mut e, OBJ, 8);
        for slot in [0xcu32, 0x14, 0x18] {
            e.register(OBJ + slot, |_, _| ret(0));
        }
        e.register(OBJ + 0x1c, |_, _| ret(HRESULT_FILE_NOT_FOUND));
        assert_eq!(e.call(0x009f_b290, &args![0u32, mapper, 1u32]).u32(), 1);
        let setup = setup_block(&mut e);
        // Unregister only: the drop's 0x80070002 becomes 0.
        assert_eq!(e.call(0x009f_b290, &args![setup, mapper, 0u32]).u32(), 0);
        assert_eq!(
            calls(&e, OBJ + 0x1c),
            vec![vec![mapper, 0xc0, 0xc1, 0xc2, 0xc3]]
        );
        assert_eq!(e.call(0x009f_b290, &args![setup, mapper, 1u32]).u32(), 0);
        assert_eq!(
            calls(&e, OBJ + 0xc),
            vec![vec![mapper, 0xc0, 0xc1, 0xc2, 0xc3, 0x503, 0x600]]
        );
        assert_eq!(
            calls(&e, OBJ + 0x14),
            vec![vec![
                mapper, 0xc0, 0xc1, 0xc2, 0xc3, 0x501, 1, 2, 3, 4, 0xd0, 0xd1, 0xd2, 0xd3, 0x502
            ]]
        );
        let types = calls(&e, OBJ + 0x18);
        assert_eq!(types.len(), 2);
        assert_eq!(
            types[0],
            vec![
                mapper, 0xc0, 0xc1, 0xc2, 0xc3, 0x501, 0xe0, 0xe1, 0xe2, 0xe3, 0xf0, 0xf1, 0xf2,
                0xf3
            ]
        );
        // A failing type registration stops and is returned.
        e.register(OBJ + 0x18, |_, _| ret(E_FAIL));
        assert_eq!(
            e.call(0x009f_b290, &args![setup, mapper, 1u32]).u32(),
            E_FAIL
        );
        assert_eq!(calls(&e, OBJ + 0x18).len(), 3);
        // A failing add stops before the pins.
        e.register(OBJ + 0xc, |_, _| ret(E_UNEXPECTED));
        assert_eq!(
            e.call(0x009f_b290, &args![setup, mapper, 1u32]).u32(),
            E_UNEXPECTED
        );
        assert_eq!(calls(&e, OBJ + 0x14).len(), 2);
    }

    #[test]
    fn reference_clock_009fb650() {
        let mut e = rig();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x2c, 0x66);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_b650, &args![this, 0u32]).u32(), E_POINTER);
        e.mem.set_u32(out, 9);
        assert_eq!(e.call(0x009f_b650, &args![this, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 0);
        let clock = object(&mut e, OBJ, 8);
        e.register(OBJ + 4, noop);
        e.mem.set_u32(this + 0xc, clock);
        assert_eq!(e.call(0x009f_b650, &args![this, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), clock);
        assert_eq!(calls(&e, OBJ + 4), vec![vec![clock]]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![0x66]; 2]);
    }

    #[test]
    fn find_pin_009fb960_by_name() {
        let mut e = rig();
        let filter = object(&mut e, OBJ, 0x40);
        let this = filter + 0xc;
        e.mem.set_u32(this + 0x2c, 0x66);
        let pins = [object(&mut e, OBJ2, 0x40), object(&mut e, OBJ2, 0x40)];
        let names = [wide(&mut e, "In"), wide(&mut e, "Out")];
        for i in 0..2 {
            e.mem.set_u32(pins[i] + 0x14, names[i]);
            let vt = e.mem.alloc(8);
            e.mem.set_u32(vt + 4, OBJ3 + 4);
            e.mem.set_u32(pins[i] + 0xc, vt);
        }
        e.register(OBJ3 + 4, noop);
        e.register(OBJ + 0x18, |_, _| ret(2));
        let list = pins;
        e.register_double(OBJ + 0x1c, move |_, a| ret(list[a[1] as usize]));
        let out = e.mem.alloc(4);
        assert_eq!(
            e.call(0x009f_b960, &args![this, 0u32, 0u32]).u32(),
            E_POINTER
        );
        let wanted = wide(&mut e, "Out");
        assert_eq!(e.call(0x009f_b960, &args![this, wanted, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), pins[1] + 0xc);
        assert_eq!(calls(&e, OBJ3 + 4), vec![vec![pins[1] + 0xc]]);
        let missing = wide(&mut e, "Other");
        assert_eq!(
            e.call(0x009f_b960, &args![this, missing, out]).u32(),
            VFW_E_NOT_FOUND
        );
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![0x66]; 2]);
    }

    fn reader_for_registration(e: &mut Engine, setup: u32) -> u32 {
        let reader = object(e, OBJ, 0x40);
        e.register_double(OBJ + 0x20, move |_, _| ret(setup));
        reader + 0x10
    }

    #[test]
    fn register_009fba40_runs_the_mapper_with_one() {
        let mut e = rig();
        let this = reader_for_registration(&mut e, 0);
        assert_eq!(e.call(0x009f_ba40, &args![this]).u32(), 1);
        assert!(calls(&e, CO_INITIALIZE).is_empty());
        let setup = setup_block(&mut e);
        let this = reader_for_registration(&mut e, setup);
        let mapper = object(&mut e, OBJ2, 8);
        for slot in [0x8u32, 0xc, 0x14, 0x18, 0x1c] {
            e.register(OBJ2 + slot, |_, _| ret(0));
        }
        e.register_double(CO_CREATE_INSTANCE, move |e, a| {
            e.mem.set_u32(a[4], mapper);
            ret(0)
        });
        assert_eq!(e.call(0x009f_ba40, &args![this]).u32(), 0);
        let create = calls(&e, CO_CREATE_INSTANCE);
        assert_eq!(
            create[0][..4],
            [CLASS_ID_0107CB08, 0, 1, INTERFACE_ID_01093B8C]
        );
        assert_eq!(calls(&e, OBJ2 + 0xc).len(), 1);
        assert_eq!(calls(&e, OBJ2 + 8), vec![vec![mapper]]);
        assert_eq!(calls(&e, CO_UNINITIALIZE).len(), 1);
        // A failed create: still 0, nothing registered.
        e.register(CO_CREATE_INSTANCE, |_, _| ret(E_FAIL));
        assert_eq!(e.call(0x009f_ba40, &args![this]).u32(), 0);
        assert_eq!(calls(&e, OBJ2 + 0xc).len(), 1);
    }

    #[test]
    fn unregister_009fbab0_returns_the_result() {
        let mut e = rig();
        let this = reader_for_registration(&mut e, 0);
        assert_eq!(e.call(0x009f_bab0, &args![this]).u32(), 1);
        let setup = setup_block(&mut e);
        let this = reader_for_registration(&mut e, setup);
        let mapper = object(&mut e, OBJ2, 8);
        e.register(OBJ2 + 8, noop);
        e.register(OBJ2 + 0x1c, |_, _| ret(E_UNEXPECTED));
        e.register_double(CO_CREATE_INSTANCE, move |e, a| {
            e.mem.set_u32(a[4], mapper);
            ret(0)
        });
        assert_eq!(e.call(0x009f_bab0, &args![this]).u32(), E_UNEXPECTED);
        e.register(OBJ2 + 0x1c, |_, _| ret(HRESULT_FILE_NOT_FOUND));
        assert_eq!(e.call(0x009f_bab0, &args![this]).u32(), 0);
        e.register(CO_CREATE_INSTANCE, |_, _| ret(HRESULT_FILE_NOT_FOUND));
        assert_eq!(e.call(0x009f_bab0, &args![this]).u32(), 0);
        e.register(CO_CREATE_INSTANCE, |_, _| ret(E_FAIL));
        assert_eq!(e.call(0x009f_bab0, &args![this]).u32(), E_FAIL);
    }

    #[test]
    fn enum_pins_skip_009fbc50() {
        let mut e = rig();
        let this = enum_pins(&mut e);
        e.mem.set_u32(this + 0x10, 0x33);
        e.mem.set_u32(this + 0x8, 6);
        assert_eq!(e.call(0x009f_bc50, &args![this, 4u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 6);
        assert_eq!(e.call(0x009f_bc50, &args![this, 1u32]).u32(), 1);
        e.mem.set_u32(this + 0x10, 0x34);
        assert_eq!(
            e.call(0x009f_bc50, &args![this, 0u32]).u32(),
            VFW_E_ENUM_OUT_OF_SYNC
        );
    }

    #[test]
    fn enum_media_types_skip_009fbe40() {
        let mut e = rig();
        let pin = object(&mut e, OBJ, 0x80);
        e.register(OBJ + 0x10, |_, _| ret(0x44));
        e.register(OBJ + 0x34, |_, a| ret(u32::from(a[1] > 2)));
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x8, pin);
        e.mem.set_u32(this + 0xc, 0x44);
        assert_eq!(e.call(0x009f_be40, &args![this, 0u32]).u32(), 0);
        assert!(calls(&e, OBJ + 0x10).is_empty());
        assert_eq!(e.call(0x009f_be40, &args![this, 2u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 2);
        assert_eq!(calls(&e, OBJ + 0x34)[0][1], 1);
        assert_eq!(e.call(0x009f_be40, &args![this, 2u32]).u32(), 1);
        e.mem.set_u32(this + 0xc, 0x45);
        assert_eq!(
            e.call(0x009f_be40, &args![this, 1u32]).u32(),
            VFW_E_ENUM_OUT_OF_SYNC
        );
    }

    /// A pin object (vtable at `OBJ`) with its interface at +0xc, its
    /// filter (state 0) and lock; returns the interface.
    fn receiving_pin(e: &mut Engine) -> u32 {
        let pin = object(e, OBJ, 0xb0);
        let this = pin + 0xc;
        let filter = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x1c, filter);
        e.mem.set_u32(this + 0x14, 0x66);
        for slot in [0x20u32, 0x24, 0x28, 0x2c, 0x30] {
            e.register(OBJ + slot, |_, _| ret(0));
        }
        this
    }

    #[test]
    fn receive_connection_009fc190() {
        let mut e = rig();
        let this = receiving_pin(&mut e);
        let pin = this - 0xc;
        let connector = object(&mut e, OBJ2, 8);
        e.register(OBJ2 + 4, noop);
        e.register(OBJ2 + 8, noop);
        let media_type = e.mem.alloc(0x48);
        assert_eq!(
            e.call(0x009f_c190, &args![this, 0u32, media_type]).u32(),
            E_POINTER
        );
        assert_eq!(
            e.call(0x009f_c190, &args![this, connector, 0u32]).u32(),
            E_POINTER
        );
        assert_eq!(
            e.call(0x009f_c190, &args![this, connector, media_type])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(this + 0xc), connector);
        assert_eq!(calls(&e, OBJ2 + 4), vec![vec![connector]]);
        assert_eq!(calls(&e, OBJ + 0x30), vec![vec![pin, connector]]);
        assert_eq!(
            e.call(0x009f_c190, &args![this, connector, media_type])
                .u32(),
            VFW_E_ALREADY_CONNECTED
        );
        e.mem.set_u32(this + 0xc, 0);
        // A running filter.
        let filter = e.mem.u32(this + 0x1c);
        e.mem.set_u32(filter + 0x14, 1);
        assert_eq!(
            e.call(0x009f_c190, &args![this, connector, media_type])
                .u32(),
            VFW_E_NOT_STOPPED
        );
        e.mem.set_u8(this + 0x19, 1);
        // A refused type (S_FALSE and E_FAIL become the type error).
        for answer in [1u32, E_FAIL, E_OUTOFMEMORY] {
            e.register_double(OBJ + 0x20, move |_, _| ret(answer));
            let expected = if answer == E_OUTOFMEMORY {
                E_OUTOFMEMORY
            } else {
                VFW_E_TYPE_NOT_ACCEPTED
            };
            assert_eq!(
                e.call(0x009f_c190, &args![this, connector, media_type])
                    .u32(),
                expected
            );
        }
        e.register(OBJ + 0x20, |_, _| ret(0));
        // A failed connector check.
        e.register(OBJ + 0x28, |_, _| ret(E_UNEXPECTED));
        assert_eq!(
            e.call(0x009f_c190, &args![this, connector, media_type])
                .u32(),
            E_UNEXPECTED
        );
        e.register(OBJ + 0x28, |_, _| ret(0));
        // A failed completion releases the connector again.
        e.register(OBJ + 0x30, |_, _| ret(E_FAIL));
        assert_eq!(
            e.call(0x009f_c190, &args![this, connector, media_type])
                .u32(),
            E_FAIL
        );
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls(&e, OBJ2 + 8), vec![vec![connector]]);
        assert_eq!(calls(&e, OBJ + 0x2c).len(), 5);
        assert_eq!(
            calls(&e, ENTER_CRITICAL_SECTION).len(),
            calls(&e, LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn connection_media_type_009fc390() {
        let mut e = rig();
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x14, 0x66);
        e.mem.set_u32(this + 0x28 + 0x10, 0x31);
        let out = e.mem.alloc(0x48);
        assert_eq!(e.call(0x009f_c390, &args![this, 0u32]).u32(), E_POINTER);
        e.mem.set_u32(out + 0x10, 0x99);
        assert_eq!(
            e.call(0x009f_c390, &args![this, out]).u32(),
            VFW_E_NOT_CONNECTED
        );
        assert_eq!(e.mem.u32(out + 0x10), 0);
        assert_eq!(e.mem.u32(out + 0x28), 1);
        e.mem.set_u32(this + 0xc, 1);
        assert_eq!(e.call(0x009f_c390, &args![this, out]).u32(), 0);
        assert_eq!(e.mem.u32(out + 0x10), 0x31);
    }

    #[test]
    fn release_buffer_009fc480() {
        let mut e = rig();
        let this = allocator(&mut e);
        e.register(OBJ3 + 0x10, noop);
        e.register(OBJ2 + 8, noop);
        let sample = e.mem.alloc(0x48);
        assert_eq!(e.call(0x009f_c480, &args![this, 0u32]).u32(), E_POINTER);
        e.mem.set_u32(this + 0x1c, 0x1234);
        e.mem.set_u32(this + 0x30, 2);
        assert_eq!(e.call(0x009f_c480, &args![this, sample]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x1c), sample);
        assert_eq!(e.mem.u32(sample + 0x1c), 0x1234);
        assert_eq!(e.mem.u32(this + 0x20), 1);
        assert!(calls(&e, OBJ3 + 0x10).is_empty());
        // The last one back with a decommit pending, waiters and a notify.
        let notify = object(&mut e, OBJ, 8);
        e.register(OBJ + 0xc, noop);
        e.mem.set_u32(this + 0x4c, notify);
        e.mem.set_u32(this + 0x48, 1);
        e.mem.set_u32(this + 0x28, 1);
        e.mem.set_u32(this + 0x24, 0x77);
        e.mem.set_u32(this + 0x28, 2);
        let other = e.mem.alloc(0x48);
        assert_eq!(e.call(0x009f_c480, &args![this, other]).u32(), 0);
        assert_eq!(calls(&e, RELEASE_SEMAPHORE), vec![vec![0x77, 2, 0]]);
        assert_eq!(calls(&e, OBJ3 + 0x10), vec![vec![this - 0xc]]);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(calls(&e, OBJ + 0xc), vec![vec![notify]]);
        assert_eq!(calls(&e, OBJ2 + 8), vec![vec![this]]);
    }

    #[test]
    fn enum_pins_clone_009fc6d0() {
        let mut e = rig();
        let this = enum_pins(&mut e);
        e.register(OBJ + 4, noop);
        e.mem.set_u32(this + 0x10, 0x33);
        e.mem.set_u32(this + 0x8, 6);
        let filter = e.mem.u32(this + 0xc);
        let vt = e.mem.alloc(8);
        e.mem.set_u32(vt + 4, OBJ + 4);
        e.mem.set_u32(filter + 0xc, vt);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_c6d0, &args![this, 0u32]).u32(), E_POINTER);
        assert_eq!(e.call(0x009f_c6d0, &args![this, out]).u32(), 0);
        let copy = e.mem.u32(out);
        assert_eq!(e.mem.u32(copy), ENUM_PINS_VTABLE);
        assert_eq!(e.mem.u32(copy + 4), 2);
        assert_eq!(e.mem.u32(copy + 0xc), filter);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x009f_c6d0, &args![this, out]).u32(), E_OUTOFMEMORY);
        assert_eq!(e.mem.u32(out), 0);
        e.mem.set_u32(this + 0x10, 0x34);
        e.mem.set_u32(out, 5);
        assert_eq!(
            e.call(0x009f_c6d0, &args![this, out]).u32(),
            VFW_E_ENUM_OUT_OF_SYNC
        );
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn enum_media_types_clone_009fc7e0() {
        let mut e = rig();
        let pin = object(&mut e, OBJ, 0x40);
        e.register(OBJ + 0x10, |_, _| ret(0x44));
        let vt = e.mem.alloc(8);
        e.mem.set_u32(vt + 4, OBJ2 + 4);
        e.mem.set_u32(pin + 0xc, vt);
        e.register(OBJ2 + 4, noop);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x8, pin);
        e.mem.set_u32(this + 0xc, 0x44);
        e.mem.set_u32(this + 0x4, 3);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_c7e0, &args![this, 0u32]).u32(), E_POINTER);
        assert_eq!(e.call(0x009f_c7e0, &args![this, out]).u32(), 0);
        let copy = e.mem.u32(out);
        assert_eq!(e.mem.u32(copy), ENUM_MEDIA_TYPES_VTABLE);
        assert_eq!(e.mem.u32(copy + 4), 3);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x009f_c7e0, &args![this, out]).u32(), E_OUTOFMEMORY);
        e.mem.set_u32(this + 0xc, 0x45);
        assert_eq!(
            e.call(0x009f_c7e0, &args![this, out]).u32(),
            VFW_E_ENUM_OUT_OF_SYNC
        );
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn sample_constructor_009fcb50() {
        let mut e = rig();
        let this = e.mem.alloc(0x48);
        for at in (0..0x48).step_by(4) {
            e.mem.set_u32(this + at, 0xeeee_eeee);
        }
        let made = e
            .call(
                0x009f_cb50,
                &args![this, 1u32, 0x2000u32, 3u32, 0x3000u32, 0x80u32],
            )
            .u32();
        assert_eq!(made, this);
        for (at, v) in [
            (0x0u32, MEDIA_SAMPLE_VTABLE),
            (0x4, 0),
            (0x8, 0),
            (0xc, 0x3000),
            (0x10, 0x80),
            (0x14, 0x80),
            (0x18, 0x2000),
            (0x3c, 0),
            (0x40, 0),
            (0x44, 0),
        ] {
            assert_eq!(e.mem.u32(this + at), v, "+{at:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x1c), 0xeeee_eeee);
    }

    #[test]
    fn allocator_constructors_009fcb90_and_009fcc20() {
        let mut e = rig();
        e.register(CREATE_SEMAPHORE_A, |_, _| ret(0x5e));
        let status = e.mem.alloc(4);
        let this = e.mem.alloc(0x64);
        for at in (0..0x64).step_by(4) {
            e.mem.set_u32(this + at, 0xeeee_eeee);
        }
        assert_eq!(
            e.call(0x009f_cc20, &args![this, 0u32, 0u32, status]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), MEM_ALLOCATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), MEM_ALLOCATOR_VTABLE_C);
        assert_eq!(e.mem.u32(this + 0x30), 0x5e);
        assert_eq!(e.mem.u32(this + 0x5c), 1);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert_eq!(
            calls(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![this + 0x10]]
        );
        assert_eq!(
            calls(&e, CREATE_SEMAPHORE_A),
            vec![vec![0, 0, 0x7fff_ffff, 0]]
        );
        assert_eq!(e.mem.u32(status), 0);
        // No semaphore wanted; a failed one sets the status.
        e.call(0x009f_cb90, &args![this, 0u32, 0u32, status, 0u32, 7u32]);
        assert_eq!(calls(&e, CREATE_SEMAPHORE_A).len(), 1);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x5c), 7);
        e.register(CREATE_SEMAPHORE_A, |_, _| ret(0));
        e.call(0x009f_cb90, &args![this, 0u32, 0u32, status, 1u32, 0u32]);
        assert_eq!(e.mem.u32(status), E_OUTOFMEMORY);
    }

    #[test]
    fn allocator_alloc_009fcc60_makes_the_samples() {
        let mut e = rig();
        let this = e.mem.alloc(0x64);
        assert_eq!(e.call(0x009f_cc60, &args![this]).u32(), VFW_E_SIZENOTSET);
        for (at, v) in [(0x38u32, 3u32), (0x40, 0x10), (0x44, 8), (0x48, 4)] {
            e.mem.set_u32(this + at, v);
        }
        e.mem.set_u32(this + 0x4c, 1);
        assert_eq!(e.call(0x009f_cc60, &args![this]).u32(), 0);
        // 0x10 + 4 rounded up to 0x18; three of them.
        assert_eq!(calls(&e, VIRTUAL_ALLOC), vec![vec![0, 0x48, 0x1000, 4]]);
        let block = e.mem.u32(this + 0x60);
        assert_eq!(e.mem.u32(this + 0x3c), 3);
        assert_eq!(e.mem.u32(this + 0x2c), 3);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        let last = e.mem.u32(this + 0x28);
        assert_eq!(e.mem.u32(last + 0xc), block + 0x30 + 4);
        assert_eq!(e.mem.u32(last + 0x10), 0x10);
        assert_eq!(e.mem.u32(last + 0x18), this);
        // Nothing changed since (+0x4c clear): 0 at once.
        assert_eq!(e.call(0x009f_cc60, &args![this]).u32(), 0);
        assert_eq!(calls(&e, VIRTUAL_ALLOC).len(), 1);
        // A failed reservation.
        e.mem.set_u32(this + 0x4c, 1);
        e.mem.set_u32(this + 0x3c, 0);
        e.mem.set_u32(this + 0x28, 0);
        e.register(VIRTUAL_ALLOC, |_, _| ret(0));
        assert_eq!(e.call(0x009f_cc60, &args![this]).u32(), E_OUTOFMEMORY);
        assert_eq!(calls(&e, VIRTUAL_FREE).len(), 1);
        assert_eq!(
            calls(&e, ENTER_CRITICAL_SECTION).len(),
            calls(&e, LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn wide_compare_009fd030() {
        let mut e = rig();
        let a = wide(&mut e, "Pin");
        let b = wide(&mut e, "Pin");
        let c = wide(&mut e, "Pit");
        let d = wide(&mut e, "Pi");
        assert_eq!(e.call(0x009f_d030, &args![a, b]).i32(), 0);
        assert_eq!(
            e.call(0x009f_d030, &args![a, c]).i32(),
            i32::from(b'n') - i32::from(b't')
        );
        assert_eq!(e.call(0x009f_d030, &args![a, d]).i32(), i32::from(b'n'));
    }

    #[test]
    fn list_steps_009fd180_and_009fd1a0() {
        let mut e = rig();
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 4, 0x4444);
        e.mem.set_u32(node + 8, 0x8888);
        let position = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_d180, &args![position]).u32(), 0);
        e.mem.set_u32(position, node);
        assert_eq!(e.call(0x009f_d180, &args![position]).u32(), 0x8888);
        assert_eq!(e.mem.u32(position), 0x4444);
        assert_eq!(e.call(0x009f_d1a0, &args![node]).u32(), 0x8888);
        assert_eq!(e.call(0x009f_d1a0, &args![0u32]).u32(), 0);
    }

    #[test]
    fn list_append_009fd2d0() {
        let mut e = rig();
        let this = e.mem.alloc(0x18);
        let other = e.mem.alloc(0x18);
        assert_eq!(e.call(0x009f_d2d0, &args![this, other]).u32(), 1);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second + 8, 0x22);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(other, first);
        assert_eq!(e.call(0x009f_d2d0, &args![this, other]).u32(), 1);
        assert_eq!(e.mem.u32(this + 8), 2);
        let head = e.mem.u32(this);
        assert_eq!(e.mem.u32(head + 8), 0x11);
        let tail = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(tail + 8), 0x22);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x009f_d2d0, &args![this, other]).u32(), 0);
    }

    #[test]
    fn allocator_factory_009fd5a0() {
        let mut e = rig();
        e.register(CREATE_SEMAPHORE_A, |_, _| ret(0x5e));
        assert_eq!(e.call(0x009f_d5a0, &args![0u32]).u32(), E_POINTER);
        let out = e.mem.alloc(4);
        // The interface query goes through the real vtable's thunk 009fb230;
        // double it.
        e.register(0x009f_b230, |e, a| {
            e.mem.set_u32(a[2], a[0]);
            ret(0)
        });
        e.mem.set_u32(MEM_ALLOCATOR_VTABLE_C, 0x009f_b230);
        e.mem.set_u32(MEM_ALLOCATOR_VTABLE + 0xc, 0x009f_c580);
        assert_eq!(e.call(0x009f_d5a0, &args![out]).u32(), 0);
        let interface = e.mem.u32(out);
        assert_eq!(e.mem.u32(interface - 0xc), MEM_ALLOCATOR_VTABLE);
        let query = calls(&e, 0x009f_b230);
        assert_eq!(query[0][1], INTERFACE_ID_01093BBC);
        // A refused query deletes it.
        e.register(0x009f_b230, |_, _| ret(E_NOINTERFACE));
        assert_eq!(e.call(0x009f_d5a0, &args![out]).u32(), E_NOINTERFACE);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
        // A failed semaphore status deletes it and is returned.
        e.register(CREATE_SEMAPHORE_A, |_, _| ret(0));
        assert_eq!(e.call(0x009f_d5a0, &args![out]).u32(), E_OUTOFMEMORY);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
        // No memory.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x009f_d5a0, &args![out]).u32(), E_OUTOFMEMORY);
        assert_eq!(e.mem.u32(out), 0);
    }

    /// A pin whose reader (+0xa4) has a file (vtable at `OBJ2`: total at
    /// +0xc, alignment at +0x10) and a running thread, and a sample (vtable
    /// at `OBJ`) whose times are 2 s to 5 s.
    fn request_rig(e: &mut Engine) -> (u32, u32, u32) {
        let this = e.mem.alloc(0xb0);
        let reader = reader(e);
        e.mem.set_u32(reader + 0x80, 1);
        e.mem.set_u32(this + 0xa4, reader);
        let file = object(e, OBJ2, 8);
        e.mem.set_u32(reader + 0x18, file);
        let sample = object(e, OBJ, 0x48);
        e.register(OBJ + 0x14, |e, a| {
            e.mem.set_u64(a[1], 20_000_000);
            e.mem.set_u64(a[2], 50_000_000);
            ret(0)
        });
        e.register(OBJ + 0x18, noop);
        e.register(OBJ + 0xc, |e, a| {
            e.mem.set_u32(a[1], 0xb000);
            ret(0)
        });
        e.register(ALLDIV, |_, a| {
            let n = (u64::from(a[1]) << 32 | u64::from(a[0])) as i64;
            let d = (u64::from(a[3]) << 32 | u64::from(a[2])) as i64;
            let q = (n / d) as u64;
            Ret {
                eax: q as u32,
                edx: (q >> 32) as u32,
                ..Ret::default()
            }
        });
        e.register(ALLMUL, |_, a| {
            let n = u64::from(a[1]) << 32 | u64::from(a[0]);
            let m = u64::from(a[3]) << 32 | u64::from(a[2]);
            let p = n.wrapping_mul(m);
            Ret {
                eax: p as u32,
                edx: (p >> 32) as u32,
                ..Ret::default()
            }
        });
        (this, reader, sample)
    }

    /// The request the reader queued last (its +0x38 list's tail).
    fn queued(e: &Engine, reader: u32) -> u32 {
        let node = e.mem.u32(reader + 0x38 + 4);
        e.mem.u32(node + 8)
    }

    #[test]
    fn request_009fd7f0_clips_to_the_total_and_queues() {
        let mut e = rig();
        let (this, reader, sample) = request_rig(&mut e);
        // Total 4 s, alignment 2.
        e.register(OBJ2 + 0xc, |_, _| ret(4));
        e.register(OBJ2 + 0x10, |_, _| ret(2));
        assert_eq!(
            e.call(0x009f_d7f0, &args![this, 0u32, 9u32]).u32(),
            E_POINTER
        );
        assert_eq!(e.call(0x009f_d7f0, &args![this, sample, 9u32]).u32(), 0);
        // Start 2 s, length 3 s: past the total 4, rounded to 4: length 2,
        // end time 4 * 10^7 set on the sample.
        assert_eq!(calls(&e, OBJ + 0x18).len(), 1);
        assert_eq!(calls(&e, ALLMUL)[0][..3], [4, 0, TIME_UNITS]);
        let request = queued(&e, reader);
        let words: Vec<u32> = (0..10).map(|i| e.mem.u32(request + 4 * i)).collect();
        assert_eq!(
            words,
            vec![
                reader,
                e.mem.u32(reader + 0x18),
                2,
                0,
                1,
                2,
                0xb000,
                sample,
                9,
                VFW_E_TIMEOUT
            ]
        );
        // Within a total of 10 (alignment 1): nothing clipped.
        e.register(OBJ2 + 0xc, |_, _| ret(10));
        e.register(OBJ2 + 0x10, |_, _| ret(1));
        assert_eq!(e.call(0x009f_d7f0, &args![this, sample, 9u32]).u32(), 0);
        assert_eq!(calls(&e, OBJ + 0x18).len(), 1);
        assert_eq!(e.mem.u32(queued(&e, reader) + 0x14), 3);
        // A failed buffer read, a failed time read.
        e.register(OBJ + 0xc, |_, _| ret(E_UNEXPECTED));
        assert_eq!(
            e.call(0x009f_d7f0, &args![this, sample, 9u32]).u32(),
            E_UNEXPECTED
        );
        e.register(OBJ + 0x14, |_, _| ret(E_FAIL));
        assert_eq!(
            e.call(0x009f_d7f0, &args![this, sample, 9u32]).u32(),
            E_FAIL
        );
    }

    #[test]
    fn adjustor_thunks_009fdcf0_to_009fdd60() {
        let mut e = rig();
        let pin = e.mem.alloc(0xb0);
        let held = object(&mut e, OBJ, 8);
        e.register(OBJ, |_, _| ret(0x40));
        e.register(OBJ + 4, |_, _| ret(0x44));
        e.register(OBJ + 8, |_, _| ret(0x48));
        e.mem.set_u32(pin + 0xc, held);
        assert_eq!(
            e.call(0x009f_dcf0, &args![pin + 0x18, 1u32, 2u32]).u32(),
            0x40
        );
        assert_eq!(calls(&e, OBJ), vec![vec![held, 1, 2]]);
        assert_eq!(e.call(0x009f_dd60, &args![pin + 0x18]).u32(), 0x44);
        assert_eq!(e.call(0x009f_dd20, &args![pin + 0x18]).u32(), 0x48);
        assert_eq!(calls(&e, OBJ + 8), vec![vec![held]]);
        // The two at -0x14 reach the reader at +0xa4.
        let reader = reader(&mut e);
        e.mem.set_u32(pin + 0xa4, reader);
        e.mem.set_u32(reader + 0x34, 5);
        assert_eq!(e.call(0x009f_dd00, &args![pin + 0x14]).u32(), 0);
        assert_eq!(e.mem.u32(reader + 0x34), 0);
        assert_eq!(calls(&e, RESET_EVENT), vec![vec![0x6c6c]]);
        assert_eq!(e.call(0x009f_dd30, &args![pin + 0x14]).u32(), 0);
        assert_eq!(e.mem.u32(reader + 0x34), 1);
        assert_eq!(calls(&e, SET_EVENT), vec![vec![0x6c6c]]);
    }

    #[test]
    fn deleting_destructors_009fddd0_and_009fdeb0() {
        let mut e = rig();
        let pin = e.mem.alloc(0x100);
        assert_eq!(e.call(0x009f_ddd0, &args![pin, 1u32]).u32(), pin);
        assert_eq!(e.mem.u32(pin), ASYNC_OUTPUT_PIN_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE).last(), Some(&vec![pin]));
        let reader = e.mem.alloc(0x200);
        let deletes = calls(&e, OPERATOR_DELETE).len();
        assert_eq!(e.call(0x009f_deb0, &args![reader, 0u32]).u32(), reader);
        assert_eq!(e.mem.u32(reader), ASYNC_READER_VTABLE);
        assert!(!calls(&e, OPERATOR_DELETE)[deletes..].contains(&vec![reader]));
        assert_eq!(e.call(0x009f_deb0, &args![reader, 3u32]).u32(), reader);
        assert_eq!(calls(&e, OPERATOR_DELETE).last(), Some(&vec![reader]));
    }

    /// A reader object with its lists and events.
    fn reader(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x6c, 0x6c6c);
        e.mem.set_u32(this + 0x68, 0x6868);
        e.mem.set_u32(this + 0x78, 0x7878);
        e.mem.set_u32(this + 0x7c, 0x7c7c);
        this
    }

    #[test]
    fn finished_queue_009fe3f0() {
        let mut e = rig();
        let this = reader(&mut e);
        assert_eq!(e.call(0x009f_e3f0, &args![this, 0x55u32]).u32(), 0);
        let node = e.mem.u32(this + 0x50);
        assert_eq!(e.mem.u32(node + 8), 0x55);
        assert_eq!(calls(&e, SET_EVENT), vec![vec![0x6c6c]]);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_e3f0, &args![this, 0x56u32]).u32(),
            E_OUTOFMEMORY
        );
    }

    #[test]
    fn request_loop_009fe420_runs_each_request() {
        let mut e = rig();
        let this = reader(&mut e);
        e.mem.set_u32(this + 0x74, 1);
        // Two queued requests on a file whose read answers 1 (no second
        // step).
        let file = object(&mut e, OBJ, 8);
        for slot in [0x14u32, 0x18] {
            e.register(OBJ + slot, noop);
        }
        e.register(OBJ + 4, |_, _| ret(1));
        let requests = [e.mem.alloc(0x28), e.mem.alloc(0x28)];
        for (i, &r) in requests.iter().enumerate() {
            e.mem.set_u32(r + 4, file);
            e.mem.set_u32(r + 8, 0x100 + i as u32);
            e.call(0x009f_d270, &args![this + 0x38, r]);
        }
        e.call(0x009f_e420, &args![this]);
        let reads: Vec<u32> = calls(&e, OBJ + 4).iter().map(|c| c[1]).collect();
        assert_eq!(reads, vec![0x100, 0x101]);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x58), 2);
        let finished = e.mem.u32(this + 0x50);
        assert_eq!(e.mem.u32(finished + 8), requests[0]);
        // The finished signal (+0x6c) twice and the idle signal (+0x78) twice.
        assert_eq!(
            calls(&e, SET_EVENT)
                .iter()
                .filter(|c| c[0] == 0x7878)
                .count(),
            2
        );
        assert_eq!(
            calls(&e, ENTER_CRITICAL_SECTION).len(),
            calls(&e, LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn thread_body_009fe500_and_routine_009fe560() {
        let mut e = rig();
        let this = reader(&mut e);
        let round = Rc::new(Cell::new(0u32));
        let r = round.clone();
        e.register_double(WAIT_FOR_MULTIPLE_OBJECTS, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x7c7c);
            assert_eq!(e.mem.u32(a[1] + 4), 0x6868);
            let n = r.get();
            r.set(n + 1);
            ret(if n < 2 { 1 } else { 0 })
        });
        assert_eq!(e.call(0x009f_e560, &args![this]).u32(), 0);
        assert_eq!(round.get(), 3);
        // Each request loop finds the queue empty (its event reset once).
        assert_eq!(calls(&e, RESET_EVENT), vec![vec![0x6868]; 2]);
        assert_eq!(calls(&e, WAIT_FOR_MULTIPLE_OBJECTS)[0][0], 2);
    }

    #[test]
    fn thread_start_009fe780() {
        let mut e = rig();
        let this = reader(&mut e);
        e.register(CREATE_THREAD, |_, _| ret(0x7777));
        assert_eq!(e.call(0x009f_e780, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x80), 0x7777);
        let start = calls(&e, CREATE_THREAD);
        assert_eq!(start[0][..5], [0, 0, THREAD_ROUTINE, this, 0]);
        assert_eq!(calls(&e, RESET_EVENT), vec![vec![0x7c7c]]);
        // Running already: nothing.
        assert_eq!(e.call(0x009f_e780, &args![this]).u32(), 0);
        assert_eq!(calls(&e, CREATE_THREAD).len(), 1);
        // A failed start.
        e.mem.set_u32(this + 0x80, 0);
        e.register(CREATE_THREAD, |_, _| ret(0));
        e.register(GET_LAST_ERROR, |_, _| ret(0x1_0008));
        assert_eq!(e.call(0x009f_e780, &args![this]).u32(), 0x8007_0008);
        e.register(GET_LAST_ERROR, |_, _| ret(0));
        assert_eq!(e.call(0x009f_e780, &args![this]).u32(), 0);
    }

    #[test]
    fn request_queue_009fe7e0() {
        let mut e = rig();
        let this = reader(&mut e);
        e.mem.set_u32(this + 0x80, 1);
        assert_eq!(e.call(0x009f_e7e0, &args![this, 0x44u32]).u32(), 0);
        let node = e.mem.u32(this + 0x38);
        assert_eq!(e.mem.u32(node + 8), 0x44);
        assert_eq!(calls(&e, SET_EVENT), vec![vec![0x6868]]);
        e.mem.set_u32(this + 0x34, 1);
        assert_eq!(
            e.call(0x009f_e7e0, &args![this, 0x45u32]).u32(),
            VFW_E_WRONG_STATE
        );
        e.mem.set_u32(this + 0x34, 0);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(
            e.call(0x009f_e7e0, &args![this, 0x46u32]).u32(),
            E_OUTOFMEMORY
        );
    }

    #[test]
    fn request_009fe960_checks_alignment_and_deletes_on_failure() {
        let mut e = rig();
        let this = reader(&mut e);
        let file = object(&mut e, OBJ, 8);
        e.register(OBJ + 0x10, |_, _| ret(0x10));
        e.mem.set_u32(this + 0x18, file);
        // A thread is running: the queue add answers 0.
        e.mem.set_u32(this + 0x80, 1);
        for (position, length, buffer) in [
            (0x8u32, 0x10u32, 0x20u32),
            (0x10, 0x8, 0x20),
            (0x10, 0x10, 0x28),
        ] {
            assert_eq!(
                e.call(
                    0x009f_e960,
                    &args![this, position, 0u32, length, 1u32, buffer, 0x51u32, 0x52u32]
                )
                .u32(),
                VFW_E_BADALIGN
            );
        }
        assert_eq!(calls(&e, OBJ + 0x10).len(), 6);
        // Not aligned: no test.
        assert_eq!(
            e.call(
                0x009f_e960,
                &args![this, 0x8u32, 1u32, 0x8u32, 0u32, 0x28u32, 0x51u32, 0x52u32]
            )
            .u32(),
            0
        );
        let node = e.mem.u32(this + 0x38);
        let request = e.mem.u32(node + 8);
        let words: Vec<u32> = (0..10).map(|i| e.mem.u32(request + 4 * i)).collect();
        assert_eq!(
            words,
            vec![this, file, 0x8, 1, 0, 0x8, 0x28, 0x51, 0x52, VFW_E_TIMEOUT]
        );
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        // A stopped reader refuses: the request is deleted.
        e.mem.set_u32(this + 0x34, 1);
        assert_eq!(
            e.call(
                0x009f_e960,
                &args![this, 0x10u32, 0u32, 0x10u32, 1u32, 0x20u32, 0x51u32, 0x52u32]
            )
            .u32(),
            VFW_E_WRONG_STATE
        );
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(
            e.call(
                0x009f_e960,
                &args![this, 0x10u32, 0u32, 0x10u32, 0u32, 0x20u32, 0x51u32, 0x52u32]
            )
            .u32(),
            E_OUTOFMEMORY
        );
    }

    #[test]
    fn stub_009fea24_returns_the_first_cell() {
        let mut e = rig();
        e.map(0x011a_6000, 0x1000);
        e.set_global(LIBRARY_FIRST_CELL, 0xffff_fffdu32);
        assert_eq!(e.call(0x009f_ea24, &args![]).u32(), 0xffff_fffd);
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_009fd0a0_duplicates_the_wide_text() {
        let mut e = Engine::new();
        e.register(CO_TASK_MEM_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            a[0].into_ret()
        });
        let text = e.mem.alloc(8);
        for (i, c) in [0x41u16, 0x42, 0x43].iter().enumerate() {
            e.mem.set_u16(text + 2 * i as u32, *c);
        }
        assert_eq!(e.call(0x009f_d0a0, &args![text, 0u32]).u32(), 0x8000_4003);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009f_d0a0, &args![text, out]).u32(), 0);
        let copy = e.mem.u32(out);
        assert_ne!(copy, text);
        let chars: Vec<u16> = (0..4).map(|i| e.mem.u16(copy + 2 * i)).collect();
        assert_eq!(chars, vec![0x41, 0x42, 0x43, 0]);
        // A failed allocation leaves the null block in `out`.
        e.register(CO_TASK_MEM_ALLOC, |_, _| 0u32.into_ret());
        e.mem.set_u32(out, 0x99);
        assert_eq!(e.call(0x009f_d0a0, &args![text, out]).u32(), 0x8007_000e);
        assert_eq!(e.mem.u32(out), 0);
    }
}
