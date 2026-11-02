//! `Havok SDK/hkmemorysystem.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global word `00cbbfb0` stores its argument in. The only other code that
/// touches it is the function at `00cbbfc0` (Ghidra name `getInstance`), which
/// reads it.
const MEMORY_SYSTEM_INSTANCE: u32 = 0x0126_8418;

layout! {
    /// `hkMemorySystem::FrameInfo` (Xbox PDB), 8 bytes.
    pub struct HkMemorySystemFrameInfo: 0x8 {
        /// `m_stackSize` (Xbox PDB).
        0x00 m_stackSize: i32,
        /// `m_solverBufferSize` (Xbox PDB).
        0x04 m_solverBufferSize: i32,
    }
}

// Translated from 00cbbfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemorySystem::FrameInfo::FrameInfo` (Xbox PDB): sets the stack size to
/// `0x32000` and the solver buffer size to `0x600000`. Returns `this`.
pub fn hk_memory_system_frame_info_frame_info(
    e: &mut Engine,
    this: Ptr<HkMemorySystemFrameInfo>,
) -> Ptr<HkMemorySystemFrameInfo> {
    e.set(this, HkMemorySystemFrameInfo::m_stackSize, 0x32000);
    e.set(
        this,
        HkMemorySystemFrameInfo::m_solverBufferSize,
        0x0060_0000,
    );
    this
}

// Translated from 00cbbfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one stack word): stores its argument in
/// the global at `01268418`, the word the function at `00cbbfc0` (Ghidra name
/// `getInstance`) reads. The Havok start-up calls it with the memory system it
/// has just built (`00c682f1` and `00c6673e` call it). Returns nothing.
pub fn fn_00cbbfb0(e: &mut Engine, instance: u32) {
    e.set_global(MEMORY_SYSTEM_INSTANCE, instance);
}

/// `hkOstream::printf` (`00cc0410`, cdecl `(stream, format, ...)`): doubles go as two words.
const PRINTF: u32 = 0x00cc_0410;
/// `hkThreadMemory::getCachedSizeUnchecked` (`00d2c350`, in `hkthreadmemory.cpp`): takes the
/// thread memory in `ECX` and answers the bytes it caches (the code passes `EAX` too, which the
/// function overwrites without reading).
const GET_CACHED_SIZE: u32 = 0x00d2_c350;
/// The calling thread's memory router (`00c85750`): its fields `+0x14` (debug) and `+0x18`
/// (solver) are allocators whose slot `0x28` fills a six-word statistics record given by address.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The double `100.0` and the float `2^32` (added to a `FILD` of a word with its top bit set).
const HUNDRED: u32 = 0x0101_7a40;
const TWO_POW_32: u32 = 0x0109_9378;
/// Format strings of the exe: `"\t%s:\n"`, `"            totalSize:   %10i\n"`,
/// `"\tThreadMemory (=CachedHeapMemory)\n"`, `"            Thread %i  size %10i\n"`,
/// `"            = total %i\n\n"`, `"\tStackMemory\n"`, the stack line with a percentage,
/// `"\tDebugMem: Not used.\n\n"` and the detailed statistics lines (reserved, soft limit,
/// overhead, available, used, free, peak).
const FORMAT_NAME: u32 = 0x010c_9078;
const FORMAT_TOTAL_SIZE: u32 = 0x010c_9058;
const FORMAT_THREAD_MEMORY: u32 = 0x010c_913c;
const FORMAT_THREAD_SIZE: u32 = 0x010c_9118;
const FORMAT_TOTAL: u32 = 0x010c_90fc;
const FORMAT_STACK_MEMORY: u32 = 0x010c_90ec;
const FORMAT_STACK_THREAD: u32 = 0x010c_90ac;
const FORMAT_DEBUG_NOT_USED: u32 = 0x010c_908c;
const FORMAT_RESERVED: u32 = 0x010c_902c;
const FORMAT_SOFT_LIMIT: u32 = 0x010c_900c;
const FORMAT_OVERHEAD: u32 = 0x010c_8fe0;
const FORMAT_AVAILABLE: u32 = 0x010c_8fc0;
const FORMAT_USED: u32 = 0x010c_8f94;
const FORMAT_FREE: u32 = 0x010c_8f68;
const FORMAT_PEAK: u32 = 0x010c_8f3c;
/// The names the statistics are printed under: `"Heap"`, `"Solver"` and `"DebugMem"`.
const NAME_HEAP: u32 = 0x010c_9160;
const NAME_SOLVER: u32 = 0x010c_90a4;
const NAME_DEBUG: u32 = 0x010c_9080;

/// `FILD dword` of `value` followed by the exe's correction for a set top bit (`TEST`, `JGE`,
/// `FADD` of the float `2^32`): the word as an unsigned number.
fn fild_unsigned(e: &Engine, value: u32) -> f64 {
    let loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        x87_add(loaded, x87_m32(e.mem.u32(TWO_POW_32)))
    } else {
        loaded
    }
}

/// The low word `FISTP qword` leaves with the truncating rounding mode: the truncated value, or
/// the "integer indefinite" (`0x8000000000000000`, low word 0) for a NaN and out-of-range values.
fn fistp_truncated_low_word(value: f64) -> u32 {
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

// Translated from 00cbc000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `printDetailedStatistics` (engine map name). Takes its inputs in registers: `statistics`
/// (four words `w0..w3` are read) in `ECX`, `soft_limit` in `EAX`, `stream` in `ESI`; then the
/// stack word `label`. All printing is `hkOstream::printf` (`00cc0410`) with the stream first.
/// `total = max(w0, w1)` (unsigned), `free = total - w1 - w3` (as a float, rounded after
/// `FILD`), `ratio = free / (w1 + w3 as unsigned)` (float), the limit is `soft_limit` or `total`
/// when it is 0 or `0x7fffffff`. `truncated = trunc((1 - ratio) * limit as float)` (truncating
/// `FISTP`, low word), `percent = 100.0 / min(limit, truncated)` rounded to a float. It prints
/// `label`, `total`, then when `limit < total` (unsigned) `total - limit` with
/// `(total - limit) * 100.0 / truncated` and `limit`; then `free` with `free * 100.0 / limit`
/// (floats), `truncated`, and `w1`, `w3`, `w2`, each with the word times `percent`.
pub fn print_detailed_statistics(
    e: &mut Engine,
    statistics: Ptr,
    soft_limit: u32,
    stream: u32,
    label: u32,
) {
    let base = statistics.addr();
    let words = [
        e.mem.u32(base),
        e.mem.u32(base + 4),
        e.mem.u32(base + 8),
        e.mem.u32(base + 0xc),
    ];
    let total = words[0].max(words[1]);
    let free = total.wrapping_sub(words[1]).wrapping_sub(words[3]);
    let used = words[1].wrapping_add(words[3]);
    let free_bits = x87_store32(f64::from(free as i32));
    let ratio_bits = x87_store32(x87_div(x87_load32(free_bits), fild_unsigned(e, used)));
    let mut limit = soft_limit;
    if limit == 0 || limit == 0x7fff_ffff {
        limit = total;
    }
    let limit_bits = x87_store32(fild_unsigned(e, limit));
    let remainder = x87_sub(1.0, x87_load32(ratio_bits));
    let scaled = x87_mul(remainder, x87_m32(limit_bits));
    let truncated = fistp_truncated_low_word(scaled);
    let beyond_limit = total.wrapping_sub(limit);
    let smaller = limit.min(truncated);
    let hundred = e.mem.f64(HUNDRED);
    let percent_bits = x87_store32(x87_div(hundred, fild_unsigned(e, smaller)));
    e.call(PRINTF, &args![stream, FORMAT_NAME, label]);
    e.call(PRINTF, &args![stream, FORMAT_TOTAL_SIZE, total]);
    if limit < total {
        let reserved = x87_div(
            x87_mul(fild_unsigned(e, beyond_limit), hundred),
            fild_unsigned(e, truncated),
        );
        e.call(
            PRINTF,
            &args![stream, FORMAT_RESERVED, beyond_limit, reserved],
        );
        e.call(PRINTF, &args![stream, FORMAT_SOFT_LIMIT, limit]);
    }
    let overhead = x87_div(x87_mul(x87_load32(free_bits), hundred), x87_m32(limit_bits));
    e.call(PRINTF, &args![stream, FORMAT_OVERHEAD, free, overhead]);
    e.call(PRINTF, &args![stream, FORMAT_AVAILABLE, truncated]);
    for (word, format) in [
        (words[1], FORMAT_USED),
        (words[3], FORMAT_FREE),
        (words[2], FORMAT_PEAK),
    ] {
        let shown = x87_mul(fild_unsigned(e, word), x87_m32(percent_bits));
        e.call(PRINTF, &args![stream, format, word, shown]);
    }
}

// Translated from 00cbc220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemorySystem::_printStatistics` (Xbox PDB), stdcall `(stream, system)`; the fields of
/// `system` are read at their offsets: the heap allocator (`+0`), the array of thread memories
/// (`+4`, count `+8`), the array of stack memories (`+0x10`, count `+0x14`), a default stack
/// total (`+0x1c`) and the soft limit (`+0x20`). Asks the heap allocator for its statistics (slot
/// `0x28`, a six-word record on the stack, cleared first) and prints them with
/// `printDetailedStatistics` (`00cbc000`) under "Heap". Then prints each thread memory's cached
/// size (`00d2c350`) when it is not 0, with the total; each stack memory with its size
/// (`+8` minus `+4`), used bytes (`+0` minus `+4`) and the percentage `used * 100.0 / size`
/// (signed words), then the total of the sizes (the default total when there are no stack
/// memories). Then the memory router's solver allocator (`+0x18`, `00c85750`) fills the record:
/// it prints "Solver", the record's word 0 as the total size and word 2 with the percentage of
/// word 0 (unsigned). Last, the router's debug allocator (`+0x14`) fills the record; when its
/// word 1 equals the heap's it prints that the debug memory is unused, else it prints the
/// record with `printDetailedStatistics` under "DebugMem" and the limit `0x7fffffff`.
pub fn hk_memory_system_print_statistics(e: &mut Engine, stream: u32, system: Ptr) {
    let system = system.addr();
    let heap = e.mem.u32(system);
    e.with_stack(0x18, |e, statistics| {
        let base = statistics.addr();
        for word in 0..6 {
            e.mem.set_u32(base + 4 * word, 0);
        }
        e.vcall(heap, 0x28, &args![statistics]);
        let heap_in_use = e.mem.u32(base + 4);
        let soft_limit = e.mem.u32(system + 0x20);
        print_detailed_statistics(e, statistics, soft_limit, stream, NAME_HEAP);
        e.call(PRINTF, &args![stream, FORMAT_THREAD_MEMORY]);
        let mut cached_total = 0u32;
        if e.mem.i32(system + 8) > 0 {
            let mut index = 0i32;
            loop {
                let thread_memories = e.mem.u32(system + 4);
                let thread_memory = e.mem.u32(thread_memories.wrapping_add(index as u32 * 4));
                let size = e.call(GET_CACHED_SIZE, &args![thread_memory]).u32();
                if size != 0 {
                    e.call(PRINTF, &args![stream, FORMAT_THREAD_SIZE, index, size]);
                    cached_total = cached_total.wrapping_add(size);
                }
                index += 1;
                if index >= e.mem.i32(system + 8) {
                    break;
                }
            }
        }
        e.call(PRINTF, &args![stream, FORMAT_TOTAL, cached_total]);
        e.call(PRINTF, &args![stream, FORMAT_STACK_MEMORY]);
        let mut stack_total = 0u32;
        if e.mem.i32(system + 0x14) > 0 {
            let hundred = e.mem.f64(HUNDRED);
            let mut index = 0i32;
            loop {
                let stack_memories = e.mem.u32(system + 0x10);
                let stack_memory = e.mem.u32(stack_memories.wrapping_add(index as u32 * 4));
                let end = e.mem.u32(stack_memory + 8);
                let size = end.wrapping_sub(e.mem.u32(stack_memory + 4));
                let used = e.mem.u32(stack_memory).wrapping_sub(end).wrapping_add(size);
                stack_total = stack_total.wrapping_add(size);
                let percent = x87_div(
                    x87_mul(f64::from(used as i32), hundred),
                    f64::from(size as i32),
                );
                e.call(
                    PRINTF,
                    &args![stream, FORMAT_STACK_THREAD, index, size, used, percent],
                );
                index += 1;
                if index >= e.mem.i32(system + 0x14) {
                    break;
                }
            }
        }
        if e.mem.i32(system + 0x14) == 0 {
            stack_total = e.mem.u32(system + 0x1c);
        }
        e.call(PRINTF, &args![stream, FORMAT_TOTAL, stack_total]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let solver = e.mem.u32(router + 0x18);
        e.vcall(solver, 0x28, &args![statistics]);
        e.call(PRINTF, &args![stream, FORMAT_NAME, NAME_SOLVER]);
        let solver_total = e.mem.u32(base);
        e.call(PRINTF, &args![stream, FORMAT_TOTAL_SIZE, solver_total]);
        let solver_peak = e.mem.u32(base + 8);
        let percent = x87_div(
            x87_mul(fild_unsigned(e, solver_peak), e.mem.f64(HUNDRED)),
            fild_unsigned(e, solver_total),
        );
        e.call(PRINTF, &args![stream, FORMAT_PEAK, solver_peak, percent]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let debug = e.mem.u32(router + 0x14);
        e.vcall(debug, 0x28, &args![statistics]);
        if heap_in_use == e.mem.u32(base + 4) {
            e.call(PRINTF, &args![stream, FORMAT_DEBUG_NOT_USED]);
            return;
        }
        print_detailed_statistics(e, statistics, 0x7fff_ffff, stream, NAME_DEBUG);
    });
}

// Translated from 00cbbfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemorySystem::getInstance` (Xbox PDB): the word at `01268418`, the memory system
/// `00cbbfb0` stored.
pub fn hk_memory_system_get_instance(e: &mut Engine) -> u32 {
    e.global::<u32>(MEMORY_SYSTEM_INSTANCE)
}

// Translated from 00cbbfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemorySystem::~hkMemorySystem` (Xbox PDB): sets the object's vtable to `010c8efc`.
pub fn hk_memory_system_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010c_8efc);
}

// Translated from 00cbbff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemorySystem::garbageCollect` (Xbox PDB), a virtual method: does nothing.
pub fn hk_memory_system_garbage_collect(_e: &mut Engine, _this: Ptr) {}

// Translated from 00cbc440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual method of two stack words it never reads (`this` is not
/// read either) that returns 1.
pub fn fn_00cbc440(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    1
}

// Translated from 00cbc450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemorySystem::_scalar_deleting_destructor_` (Xbox PDB): runs `~hkMemorySystem` (`00cbbfe0`)
/// on `this`; the stack word (the flags) is not read and nothing is freed. Returns `this`.
pub fn hk_memory_system_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    hk_memory_system_destructor(e, this);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cbbfc0, hk_memory_system_get_instance() -> u32),
        entry!(0x00cbbfe0, hk_memory_system_destructor(Ptr)),
        entry!(0x00cbbff0, hk_memory_system_garbage_collect(Ptr)),
        entry!(0x00cbc000, print_detailed_statistics(Ptr, u32, u32, u32)),
        entry!(0x00cbc220, hk_memory_system_print_statistics(u32, Ptr)),
        entry!(0x00cbc440, fn_00cbc440(Ptr, u32, u32) -> u32),
        entry!(0x00cbc450, hk_memory_system_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00cbbfa0,
            hk_memory_system_frame_info_frame_info(
                Ptr<HkMemorySystemFrameInfo>,
            ) -> Ptr<HkMemorySystemFrameInfo>
        ),
        entry!(0x00cbbfb0, fn_00cbbfb0(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_info_constructor_sets_the_default_sizes_and_returns_this() {
        let mut e = Engine::new();
        let info: Ptr<HkMemorySystemFrameInfo> = e.new_object();
        e.mem.set_u32(info.addr(), 0xdead_beef);
        e.mem.set_u32(info.addr() + 4, 0xdead_beef);
        assert_eq!(e.call(0x00cb_bfa0, &args![info]).u32(), info.addr());
        assert_eq!(e.mem.u32(info.addr()), 0x32000);
        assert_eq!(e.mem.u32(info.addr() + 4), 0x0060_0000);
    }

    #[test]
    fn the_instance_setter_stores_its_argument_in_the_global() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.call(0x00cb_bfb0, &args![0x0120_4410u32]);
        assert_eq!(e.global::<u32>(MEMORY_SYSTEM_INSTANCE), 0x0120_4410);
        e.call(0x00cb_bfb0, &args![0u32]);
        assert_eq!(e.global::<u32>(MEMORY_SYSTEM_INSTANCE), 0);
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.clone().unwrap();
        log.into_iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1)
            .collect()
    }

    /// The exe's `100.0` and `2^32`, and a `printf` that prints nothing.
    fn printing_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(HUNDRED, 100.0);
        e.mem.set_u32(TWO_POW_32, 0x4f80_0000);
        e.register(PRINTF, |_, _| Ret::default());
        e
    }

    fn statistics(e: &mut Engine, words: [u32; 6]) -> u32 {
        let record = e.mem.alloc(0x18);
        for (i, word) in words.into_iter().enumerate() {
            e.mem.set_u32(record + 4 * i as u32, word);
        }
        record
    }

    #[test]
    fn the_small_methods() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(MEMORY_SYSTEM_INSTANCE, 0x1234u32);
        assert_eq!(e.call(0x00cb_bfc0, &args![]).u32(), 0x1234);
        let this = e.mem.alloc(8);
        e.call(0x00cb_bfe0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x010c_8efc);
        e.mem.set_u32(this, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cb_bff0, &args![this]);
        assert_eq!(e.call_log.clone().unwrap().len(), 1);
        assert_eq!(e.call(0x00cb_c440, &args![this, 1u32, 2u32]).u32(), 1);
        // the deleting destructor runs the destructor, frees nothing and ignores its flag
        assert_eq!(e.call(0x00cb_c450, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_8efc);
    }

    #[test]
    fn detailed_statistics_print_the_figures_with_their_percentages() {
        let mut e = printing_engine();
        let record = statistics(&mut e, [1000, 600, 700, 100, 0, 0]);
        e.call_log = Some(vec![]);
        e.call(0x00cb_c000, &args![record, 800u32, 0x55u32, 0x6000u32]);
        // free = 300; its float ratio to the 700 used; (1 - ratio) * 800 truncates to 457
        let ratio = (300.0f64 / 700.0) as f32;
        let truncated = ((1.0 - f64::from(ratio)) * 800.0) as u32;
        assert_eq!(truncated, 457);
        let percent = f64::from((100.0f64 / 457.0) as f32);
        let expected: Vec<Vec<u32>> = vec![
            args![0x55u32, FORMAT_NAME, 0x6000u32].to_vec(),
            args![0x55u32, FORMAT_TOTAL_SIZE, 1000u32].to_vec(),
            args![0x55u32, FORMAT_RESERVED, 200u32, 20000.0f64 / 457.0].to_vec(),
            args![0x55u32, FORMAT_SOFT_LIMIT, 800u32].to_vec(),
            args![0x55u32, FORMAT_OVERHEAD, 300u32, 30000.0f64 / 800.0].to_vec(),
            args![0x55u32, FORMAT_AVAILABLE, 457u32].to_vec(),
            args![0x55u32, FORMAT_USED, 600u32, 600.0f64 * percent].to_vec(),
            args![0x55u32, FORMAT_FREE, 100u32, 100.0f64 * percent].to_vec(),
            args![0x55u32, FORMAT_PEAK, 700u32, 700.0f64 * percent].to_vec(),
        ];
        assert_eq!(calls_to(&e, PRINTF), expected);
    }

    #[test]
    fn detailed_statistics_use_the_total_when_there_is_no_limit() {
        let mut e = printing_engine();
        let record = statistics(&mut e, [1000, 600, 700, 100, 0, 0]);
        for limit in [0u32, 0x7fff_ffff, 1000, 5000] {
            e.call_log = Some(vec![]);
            e.call(0x00cb_c000, &args![record, limit, 0x55u32, 0x6000u32]);
            let lines = calls_to(&e, PRINTF);
            // no "reserved" and "soft limit" lines: the limit is not below the total
            assert_eq!(lines.len(), 7, "limit {limit}");
            // (1 - 300/700) * 1000 truncates to 571; a larger limit scales it
            let available = lines[3].clone();
            assert_eq!(available[1], FORMAT_AVAILABLE);
            let ratio = (300.0f64 / 700.0) as f32;
            let scaled =
                (1.0 - f64::from(ratio)) * f64::from(if limit == 5000 { 5000 } else { 1000 });
            assert_eq!(available[2], scaled as u32, "limit {limit}");
        }
    }

    #[test]
    fn the_total_is_the_larger_of_the_first_two_words() {
        let mut e = printing_engine();
        let record = statistics(&mut e, [500, 900, 0, 50, 0, 0]);
        e.call_log = Some(vec![]);
        e.call(0x00cb_c000, &args![record, 0u32, 0x55u32, 0x6000u32]);
        let lines = calls_to(&e, PRINTF);
        assert_eq!(lines[1], args![0x55u32, FORMAT_TOTAL_SIZE, 900u32].to_vec());
        // free = 900 - 900 - 50 wraps to a negative word: printed as the word
        assert_eq!(lines[2][2], 0u32.wrapping_sub(50));
    }

    const SYSTEM_STREAM: u32 = 0x55;

    /// A memory system with two thread memories (cached sizes 0 and 5), one stack memory,
    /// and allocators (heap, solver, debug) whose statistics records are set by doubles.
    fn system(e: &mut Engine, debug_in_use: u32) -> u32 {
        let allocator_at = |e: &mut Engine, vtable: u32, slot: u32| {
            let mut slots = vec![0u32; 10];
            slots.push(slot);
            e.put_vtable(vtable, &slots);
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, vtable);
            object
        };
        let heap = allocator_at(e, 0x4400_0000, 0x4500_0000);
        let solver = allocator_at(e, 0x4400_1000, 0x4500_0001);
        let debug = allocator_at(e, 0x4400_2000, 0x4500_0002);
        let fill = |e: &mut Engine, address: u32, words: [u32; 6]| {
            e.register_double(address, move |e, a| {
                for (i, word) in words.into_iter().enumerate() {
                    e.mem.set_u32(a[1] + 4 * i as u32, word);
                }
                Ret::default()
            });
        };
        fill(e, 0x4500_0000, [1000, 600, 700, 100, 0, 0]);
        fill(e, 0x4500_0001, [2000, 0, 500, 0, 0, 0]);
        fill(e, 0x4500_0002, [0, debug_in_use, 0, 0, 0, 0]);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x14, debug);
        e.mem.set_u32(router + 0x18, solver);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(GET_CACHED_SIZE, |_, a| Ret {
            eax: a[0] - 0x1000,
            ..Ret::default()
        });
        let system = e.mem.alloc(0x24);
        e.mem.set_u32(system, heap);
        let threads = e.mem.alloc(8);
        e.mem.set_u32(threads, 0x1000);
        e.mem.set_u32(threads + 4, 0x1005);
        e.mem.set_u32(system + 4, threads);
        e.mem.set_u32(system + 8, 2);
        let stack_memory = e.mem.alloc(0x10);
        e.mem.set_u32(stack_memory, 0x500);
        e.mem.set_u32(stack_memory + 4, 0x100);
        e.mem.set_u32(stack_memory + 8, 0x200);
        let stacks = e.mem.alloc(4);
        e.mem.set_u32(stacks, stack_memory);
        e.mem.set_u32(system + 0x10, stacks);
        e.mem.set_u32(system + 0x14, 1);
        e.mem.set_u32(system + 0x1c, 0x777);
        e.mem.set_u32(system + 0x20, 800);
        system
    }

    #[test]
    fn print_statistics_walks_the_heap_threads_stacks_solver_and_debug_memory() {
        let mut e = printing_engine();
        let system = system(&mut e, 600);
        e.call_log = Some(vec![]);
        e.call(0x00cb_c220, &args![SYSTEM_STREAM, system]);
        let lines = calls_to(&e, PRINTF);
        let with_format = |format: u32| -> Vec<Vec<u32>> {
            lines.iter().filter(|l| l[1] == format).cloned().collect()
        };
        // the heap, then the solver, under their names
        assert_eq!(
            with_format(FORMAT_NAME),
            vec![
                args![SYSTEM_STREAM, FORMAT_NAME, NAME_HEAP].to_vec(),
                args![SYSTEM_STREAM, FORMAT_NAME, NAME_SOLVER].to_vec(),
            ]
        );
        // the thread with a zero cached size is not listed; the total is 5
        assert_eq!(
            with_format(FORMAT_THREAD_SIZE),
            vec![args![SYSTEM_STREAM, FORMAT_THREAD_SIZE, 1i32, 5u32].to_vec()]
        );
        // the stack memory: size 0x100, used (0x500 - 0x200) + 0x100 = 0x400, 400.0 percent
        assert_eq!(
            with_format(FORMAT_STACK_THREAD),
            vec![args![
                SYSTEM_STREAM,
                FORMAT_STACK_THREAD,
                0i32,
                0x100u32,
                0x400u32,
                400.0f64
            ]
            .to_vec()]
        );
        assert_eq!(
            with_format(FORMAT_TOTAL),
            vec![
                args![SYSTEM_STREAM, FORMAT_TOTAL, 5u32].to_vec(),
                args![SYSTEM_STREAM, FORMAT_TOTAL, 0x100u32].to_vec(),
            ]
        );
        // the solver: total size 2000, peak 500 = 25.0 percent; the heap's own peak line is first
        let peaks = with_format(FORMAT_PEAK);
        assert_eq!(peaks.len(), 2);
        assert_eq!(
            peaks[1],
            args![SYSTEM_STREAM, FORMAT_PEAK, 500u32, 25.0f64].to_vec()
        );
        assert_eq!(
            with_format(FORMAT_TOTAL_SIZE),
            vec![
                args![SYSTEM_STREAM, FORMAT_TOTAL_SIZE, 1000u32].to_vec(),
                args![SYSTEM_STREAM, FORMAT_TOTAL_SIZE, 2000u32].to_vec(),
            ]
        );
        // the debug memory's word 1 equals the heap's: unused
        assert_eq!(
            with_format(FORMAT_DEBUG_NOT_USED),
            vec![args![SYSTEM_STREAM, FORMAT_DEBUG_NOT_USED].to_vec()]
        );
        assert_eq!(with_format(FORMAT_THREAD_MEMORY).len(), 1);
        assert_eq!(with_format(FORMAT_STACK_MEMORY).len(), 1);
    }

    #[test]
    fn print_statistics_prints_the_debug_memory_when_it_was_used() {
        let mut e = printing_engine();
        let system = system(&mut e, 601);
        e.call_log = Some(vec![]);
        e.call(0x00cb_c220, &args![SYSTEM_STREAM, system]);
        let lines = calls_to(&e, PRINTF);
        assert!(lines.iter().all(|l| l[1] != FORMAT_DEBUG_NOT_USED));
        let names: Vec<u32> = lines
            .iter()
            .filter(|l| l[1] == FORMAT_NAME)
            .map(|l| l[2])
            .collect();
        assert_eq!(names, vec![NAME_HEAP, NAME_SOLVER, NAME_DEBUG]);
        // the debug record is (0, 601, 0, 0): total 601, its limit 0x7fffffff means the total
        let total_sizes: Vec<u32> = lines
            .iter()
            .filter(|l| l[1] == FORMAT_TOTAL_SIZE)
            .map(|l| l[2])
            .collect();
        assert_eq!(total_sizes, vec![1000, 2000, 601]);
    }

    #[test]
    fn print_statistics_without_stack_memories_prints_the_default_total() {
        let mut e = printing_engine();
        let system = system(&mut e, 600);
        e.mem.set_u32(system + 0x14, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cb_c220, &args![SYSTEM_STREAM, system]);
        let lines = calls_to(&e, PRINTF);
        let totals: Vec<u32> = lines
            .iter()
            .filter(|l| l[1] == FORMAT_TOTAL)
            .map(|l| l[2])
            .collect();
        assert_eq!(totals, vec![5, 0x777]);
        assert!(lines.iter().all(|l| l[1] != FORMAT_STACK_THREAD));
    }
}
