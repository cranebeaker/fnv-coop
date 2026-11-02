//! `Havok SDK/hksystemclock.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkWindowsSystemClock` counts ticks with the processor's time-stamp counter and finds its
//! rate by comparing it with the performance counter of the operating system. The
//! functions call `QueryPerformanceCounter` and `QueryPerformanceFrequency` through their import
//! slots; the time-stamp counter is read with the host's own `RDTSC`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The cached ticks per second (`01269cc8`, 64 bits; also where `QueryPerformanceFrequency`
/// is told to write).
const CACHED_TICKS_PER_SECOND: u32 = 0x0126_9cc8;
/// `1000000.0` (`double`), the starting value of the smallest ratio found.
const INITIAL_RATIO: u32 = 0x0106_b798;
/// `QueryPerformanceCounter` and `QueryPerformanceFrequency` (`kernel32` import slots, `stdcall`,
/// one pointer to a 64-bit result).
const QUERY_PERFORMANCE_COUNTER: u32 = 0x00fd_f0a0;
const QUERY_PERFORMANCE_FREQUENCY: u32 = 0x00fd_f0a4;
/// The CRT's conversion of an `ST0` value to a 64-bit integer (`00ec62f6`, no map name; the value
/// is passed as an `f64` argument, the result in `EDX:EAX`).
const FLOAT_TO_INT64: u32 = 0x00ec_62f6;
/// How many times the calibration measures, and how long its busy loop runs.
const MEASUREMENTS: u32 = 10;
const BUSY_LOOP_ITERATIONS: u32 = 5000;

/// The processor's time-stamp counter (`RDTSC`).
#[cfg(target_arch = "x86_64")]
fn read_time_stamp_counter() -> u64 {
    // SAFETY: `RDTSC` reads a register and has no preconditions on this host.
    unsafe { core::arch::x86_64::_rdtsc() }
}

/// The processor's time-stamp counter (`RDTSC`): only an x86-64 host has it.
#[cfg(not(target_arch = "x86_64"))]
fn read_time_stamp_counter() -> u64 {
    panic!("hkWindowsSystemClock::getTickCounter (00dd8e06) reads RDTSC, which this host lacks")
}

// Translated from 00dd8e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkWindowsSystemClock::getTickCounter` (Xbox PDB), virtual (slot `0xc`), no stack words
/// (`ECX` not read): the processor's time-stamp counter, 64 bits in `EDX:EAX`.
pub fn hk_windows_system_clock_get_tick_counter(_e: &mut Engine, _this: Ptr) -> u64 {
    read_time_stamp_counter()
}

/// A 64-bit count as the double the code makes of it with two `FILD`s: the low 63 bits as a
/// signed integer plus the negated top bit read as a signed integer (`-2^63` negated: `2^63`, or
/// `-0`), the sum taken on the x87 stack.
fn unsigned_as_double(value: u64) -> f64 {
    let low = (value & 0x7fff_ffff_ffff_ffff) as i64 as f64;
    let top = (value & 0x8000_0000_0000_0000) as i64 as f64;
    x87_add(low, -top)
}

// Translated from 00dd8e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkWindowsSystemClock::getTicksPerSecond` (Xbox PDB), `ECX` = `this`, no stack words (the
/// slot-0x10 function of the clock; its result is the cached 64-bit word at `01269cc8`, which this
/// function fills). Does nothing when that word is not 0. Otherwise measures 10 times: the clock's
/// tick counter (slot `0xc`), `QueryPerformanceCounter`, a busy loop of 5000 steps of `x = x * x +
/// x`, the tick counter and `QueryPerformanceCounter` again, and `QueryPerformanceFrequency`
/// (written straight into the cached word). Each ratio of the tick difference to the
/// performance-counter difference (both converted by `unsigned_as_double`, divided on the x87
/// stack) replaces the smallest so far, starting from 1000000.0, only when it is smaller (a
/// NaN never is). The result is that smallest ratio times the performance frequency now in the
/// cached word, converted to 64 bits by `00ec62f6` and stored there.
///
/// Answers the cached word in `EDX:EAX` on both paths: the exe loads it into `EAX`/`EDX` first
/// and leaves them on the early return, and the measured rate is the `00ec62f6` result it stores
/// (`getTicksPerSecond` answers an `hkUint64`).
pub fn hk_windows_system_clock_get_ticks_per_second(e: &mut Engine, this: Ptr) -> u64 {
    let low = e.mem.u32(CACHED_TICKS_PER_SECOND);
    let high = e.mem.u32(CACHED_TICKS_PER_SECOND + 4);
    if low | high != 0 {
        return u64::from(high) << 32 | u64::from(low);
    }
    let mut smallest = e.mem.f64(INITIAL_RATIO);
    // The exe's frame (`AND ESP,0xfffffff8`, `SUB ESP,0x48`, four pushes): 0x58 bytes from an
    // 8-byte boundary; the busy loop's word at +0x10 (00dd8e74), the end counter at +0x28
    // (00dd8ea2) and the start counter at +0x30 (00dd8e63).
    e.with_stack(0x58 + 8, |e, raw| {
        let frame = (raw.addr() + 7) & !7;
        let (spin_at, end_at, start_at) = (frame + 0x10, frame + 0x28, frame + 0x30);
        for _ in 0..MEASUREMENTS {
            let first = e.vcall(this.addr(), 0xc, &args![]).u64();
            e.call(QUERY_PERFORMANCE_COUNTER, &args![start_at]);
            let start = e.mem.u64(start_at);
            let mut spin: u32 = 1;
            for _ in 0..BUSY_LOOP_ITERATIONS {
                spin = spin.wrapping_mul(spin).wrapping_add(spin);
            }
            e.mem.set_u32(spin_at, spin);
            let second = e.vcall(this.addr(), 0xc, &args![]).u64();
            e.call(QUERY_PERFORMANCE_COUNTER, &args![end_at]);
            let end = e.mem.u64(end_at);
            e.call(QUERY_PERFORMANCE_FREQUENCY, &args![CACHED_TICKS_PER_SECOND]);
            let ticks = unsigned_as_double(second.wrapping_sub(first));
            let performance = unsigned_as_double(end.wrapping_sub(start));
            let ratio = x87_div(ticks, performance);
            if smallest > ratio {
                smallest = ratio;
            }
        }
    });
    let frequency = u64::from(e.mem.u32(CACHED_TICKS_PER_SECOND + 4)) << 32
        | u64::from(e.mem.u32(CACHED_TICKS_PER_SECOND));
    let product = x87_mul(smallest, unsigned_as_double(frequency));
    let rate = e.call(FLOAT_TO_INT64, &args![product]);
    e.mem.set_u32(CACHED_TICKS_PER_SECOND, rate.eax);
    e.mem.set_u32(CACHED_TICKS_PER_SECOND + 4, rate.edx);
    u64::from(rate.edx) << 32 | u64::from(rate.eax)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd8e00, hk_windows_system_clock_get_tick_counter(Ptr) -> u64),
        entry!(
            0x00dd8e20,
            hk_windows_system_clock_get_ticks_per_second(Ptr) -> u64
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn the_tick_counter_is_the_time_stamp_counter() {
        let mut e = Engine::new();
        let first = e.call(0x00dd_8e00, &args![0u32]).u64();
        let second = e.call(0x00dd_8e00, &args![0u32]).u64();
        assert!(first > 0);
        assert!(second >= first);
    }

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A clock whose tick counter answers `counters` one after another (slot `0xc`), over
    /// performance counter reads that answer `performance` one after another and a performance
    /// frequency of `frequency`; the 64-bit conversion `00ec62f6` truncates its `f64` argument.
    fn world(counters: Vec<u64>, performance: Vec<u64>, frequency: u64) -> (Engine, u32, Calls) {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        e.map(0x0106_b000, 0x1000);
        e.mem.set_f64(INITIAL_RATIO, 1_000_000.0);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let mut next_counter = 0;
        e.register_double(0x0b20_000c, move |_, _| {
            let value = counters[next_counter];
            next_counter += 1;
            Ret {
                eax: value as u32,
                edx: (value >> 32) as u32,
                ..Ret::default()
            }
        });
        let mut next_performance = 0;
        e.register_double(QUERY_PERFORMANCE_COUNTER, move |e, a| {
            e.mem.set_u64(a[0], performance[next_performance]);
            next_performance += 1;
            Ret::default()
        });
        let log = Rc::clone(&calls);
        e.register_double(QUERY_PERFORMANCE_FREQUENCY, move |e, a| {
            log.borrow_mut()
                .push((QUERY_PERFORMANCE_FREQUENCY, a.to_vec()));
            e.mem.set_u64(a[0], frequency);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(FLOAT_TO_INT64, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let rate = value as i64;
            Ret {
                eax: rate as u32,
                edx: (rate >> 32) as u32,
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0b20_000c);
        let clock = e.mem.alloc(0x10);
        e.mem.set_u32(clock, vtable);
        (e, clock, calls)
    }

    fn cached(e: &Engine) -> u64 {
        e.mem.u64(CACHED_TICKS_PER_SECOND)
    }

    /// Ten measurements: tick differences `ticks[i]` over performance differences `performance[i]`.
    fn measurements(ticks: [u64; 10], performance: [u64; 10]) -> (Vec<u64>, Vec<u64>) {
        let mut counters = Vec::new();
        let mut counts = Vec::new();
        for i in 0..10u64 {
            counters.push(1000 * i);
            counters.push(1000 * i + ticks[i as usize]);
            counts.push(500 * i);
            counts.push(500 * i + performance[i as usize]);
        }
        (counters, counts)
    }

    #[test]
    fn a_cached_rate_is_left_alone() {
        let (mut e, clock, calls) = world(vec![], vec![], 5);
        e.mem.set_u64(CACHED_TICKS_PER_SECOND, 123);
        e.call(0x00dd_8e20, &args![clock]);
        assert_eq!(cached(&e), 123);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn the_rate_is_the_smallest_measured_ratio_times_the_performance_frequency() {
        // Ratios 10, 8, 4, 6, ...: the smallest is 4 (400 ticks over 100).
        let (counters, counts) = measurements(
            [1000, 800, 400, 600, 900, 700, 500, 450, 1000, 1000],
            [100; 10],
        );
        let (mut e, clock, calls) = world(counters, counts, 2_000_000);
        // The start counter at +0x30, the end counter at +0x28 of the exe's 8-aligned frame.
        let block = e.next_stack_local(0x58 + 8);
        let frame = (block + 7) & !7;
        e.call_log = Some(vec![]);
        e.call(0x00dd_8e20, &args![clock]);
        let counted: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == QUERY_PERFORMANCE_COUNTER)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(counted[..2], [frame + 0x30, frame + 0x28]);
        assert_eq!(cached(&e), 8_000_000);
        // `QueryPerformanceFrequency` was asked ten times, writing into the cached word.
        let asked = calls.borrow();
        assert_eq!(asked.len(), 10);
        assert!(asked.iter().all(|c| c.1 == vec![CACHED_TICKS_PER_SECOND]));
    }

    #[test]
    fn ratios_above_the_starting_value_and_not_a_number_ratios_do_not_count() {
        // Every ratio is above 1000000 (10000000 ticks over 1) or not a number (0 over 0): the
        // starting value stays and the rate is 1000000 times the frequency.
        let (counters, counts) = measurements(
            [10_000_000, 0, 10_000_000, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        );
        let (mut e, clock, _) = world(counters, counts, 3);
        e.call(0x00dd_8e20, &args![clock]);
        assert_eq!(cached(&e), 3_000_000);
    }

    #[test]
    fn the_rate_is_answered_measured_or_cached() {
        let (counters, counts) = measurements(
            [10_000_000, 0, 10_000_000, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        );
        let (mut e, clock, _) = world(counters, counts, 3);
        assert_eq!(e.call(0x00dd_8e20, &args![clock]).u64(), 3_000_000);
        // The second call finds the cached word and answers it unchanged.
        e.mem.set_u32(CACHED_TICKS_PER_SECOND, 0x89ab_cdef);
        e.mem.set_u32(CACHED_TICKS_PER_SECOND + 4, 0x0123_4567);
        assert_eq!(
            e.call(0x00dd_8e20, &args![clock]).u64(),
            0x0123_4567_89ab_cdef
        );
    }

    #[test]
    fn the_count_conversion_adds_the_top_bit_as_two_to_the_63() {
        assert_eq!(unsigned_as_double(5), 5.0);
        assert_eq!(
            unsigned_as_double(0x8000_0000_0000_0000),
            9.223_372_036_854_776e18
        );
        assert_eq!(unsigned_as_double(u64::MAX), 1.8446744073709552e19);
        // A zero top bit adds a negative zero: still zero.
        assert_eq!(unsigned_as_double(0), 0.0);
    }
}
