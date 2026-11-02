//! `fallout/ai/run_009a4690` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00435dd0` (unnamed, `extradataobjects.cpp` per the engine map): no words; returns the
/// float at `0x011f1bf0` in `ST0`.
const CURRENT_TIME: u32 = 0x0043_5dd0;
/// `00435de0` (unnamed, `extradataobjects.cpp` per the engine map): `thiscall`, one float
/// (`RET 4`); stores it at `this`.
const SET_TIME_STAMP: u32 = 0x0043_5de0;

/// `00435dd0` plus `delay` (`FADD dword`), stored as a float, loaded again and passed to
/// `00435de0` on a stack local; the local's word.
fn time_stamp(e: &mut Engine, delay: f32) -> u32 {
    let now = e.call(CURRENT_TIME, &[]).f64();
    let sum = x87_store32(x87_add(now, x87_m32(delay.to_bits())));
    e.with_stack(4, |e, local| {
        e.call(SET_TIME_STAMP, &args![local, x87_quiet_bits(sum)]);
        e.mem.u32(local.addr())
    })
}

// Translated from 009a4690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x18`): stores `first` at `+0` and the float
/// `second` at `+4`; unless `keep_start` is set, `+8` becomes the time stamp of now plus
/// `start_delay` (`00435dd0`, `00435de0`); `+0xc` always becomes the time stamp of now
/// plus `end_delay`, and `+0x10` the float `last`.
pub fn fn_009a4690(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: f32,
    start_delay: f32,
    end_delay: f32,
    last: f32,
    keep_start: u8,
) {
    let p = this.addr();
    e.mem.set_u32(p, first);
    e.mem.set_u32(p + 0x4, x87_quiet_bits(second.to_bits()));
    if keep_start == 0 {
        let stamp = time_stamp(e, start_delay);
        e.mem.set_u32(p + 0x8, stamp);
    }
    let stamp = time_stamp(e, end_delay);
    e.mem.set_u32(p + 0xc, stamp);
    e.mem.set_u32(p + 0x10, x87_quiet_bits(last.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x009a_4690,
        fn_009a4690(Ptr, u32, f32, f32, f32, f32, u8)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doubles(e: &mut Engine) {
        e.register(CURRENT_TIME, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.register(SET_TIME_STAMP, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    #[test]
    fn fn_009a4690_stores_values_and_time_stamps() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x14);
        e.call(
            0x009a_4690,
            &args![
                this,
                7u32,
                1.5f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                4.5f32.to_bits(),
                0x100u32
            ],
        );
        assert_eq!(e.mem.u32(this), 7);
        assert_eq!(e.mem.f32(this + 4), 1.5);
        assert_eq!(e.mem.f32(this + 8), 12.0);
        assert_eq!(e.mem.f32(this + 0xc), 13.0);
        assert_eq!(e.mem.f32(this + 0x10), 4.5);
    }

    #[test]
    fn fn_009a4690_keeps_the_start_stamp() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, 0x1234_5678);
        e.call_log = Some(vec![]);
        // A signalling NaN copied through FLD/FSTP comes out quiet.
        e.call(
            0x009a_4690,
            &args![
                this,
                7u32,
                0x7f80_0001u32,
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                4.5f32.to_bits(),
                1u32
            ],
        );
        assert_eq!(e.mem.u32(this + 4), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 8), 0x1234_5678);
        assert_eq!(e.mem.f32(this + 0xc), 13.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == SET_TIME_STAMP).count(), 1);
    }
}
