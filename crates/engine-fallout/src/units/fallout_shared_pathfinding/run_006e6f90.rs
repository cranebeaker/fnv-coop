//! `fallout shared/pathfinding/run_006e6f90` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The point `006e6f90` scales (`011a9484`, a `float` triple in `.data`).
const POINT_011A9484: u32 = 0x011a_9484;
/// `0045bb20` (`thiscall`, `RET 8`: out, a float): `this * scale` into
/// `out`; returns `out`.
const POINT_SCALED: u32 = 0x0045_bb20;
/// `00439e90` and `00439ef0` (`thiscall`, `RET 8`: out, other): `this +
/// other` and `this - other` into `out`; return `out`.
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `0062a2b0` (`thiscall`, on the object at `+0x80` plus 0x30) and
/// `0062a290` (`thiscall`, on the object at `+0x84`): run before each test.
const RESET_0062A2B0: u32 = 0x0062_a2b0;
const RESET_0062A290: u32 = 0x0062_a290;
/// `004a3da0` and `004a3eb0` (`thiscall` on the object at `+0x80`,
/// `RET 4`): given the first and the second point of each test.
const SET_FIRST_004A3DA0: u32 = 0x004a_3da0;
const SET_SECOND_004A3EB0: u32 = 0x004a_3eb0;
/// The virtual slot (`+0xc8`) of the object at `+0x7c` that runs a test
/// with the object at `+0x80`.
const SLOT_TEST: u32 = 0xc8;
/// `00460140` (`thiscall`): the object at `+0x84` plus 0x10.
const RESULT_00460140: u32 = 0x0046_0140;

// Translated from 006e6f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`; two points and an out point; `RET 0xc`): two
/// tests through the virtual slot `+0xc8` of the object at `+0x7c` with
/// the object at `+0x80`, each after `0062a2b0` (on `+0x80`'s `+0x30`),
/// `0062a290` (on `+0x84`) and with the counter at `+0x88` raised. The
/// first goes from `from` to `to`, each moved by the point at `011a9484`
/// times the float at `+0x90`; when it answers true the result is false.
/// The second goes from that moved `to` down by the point at `011a9484`
/// times (`+0x8c` + `+0x90`); when it answers false the result is false,
/// else `out` is `to' + (end - to') * t`, `t` the float at `+0x10` of
/// `00460140` of `+0x84`, and the result is true.
pub fn fn_006e6f90(e: &mut Engine, this: Ptr, from: Ptr, to: Ptr, out: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(0x78, |e, frame| {
        let l = |x: u32| frame.addr() + 0x78 - x;
        let pushed = |e: &Engine, address: u32| x87_quiet_bits(e.mem.u32(address));
        let value = pushed(e, base + 0x90);
        e.mem.set_u32(l(0x10), value);
        let radius = pushed(e, l(0x10));
        let offset = e
            .call(POINT_SCALED, &args![POINT_011A9484, l(0x40), radius])
            .u32();
        e.call(POINT_ADD, &args![from, l(0xc), offset]);
        let radius = pushed(e, l(0x10));
        let offset = e
            .call(POINT_SCALED, &args![POINT_011A9484, l(0x4c), radius])
            .u32();
        e.call(POINT_ADD, &args![to, l(0x1c), offset]);
        let (start, end) = (l(0xc), l(0x1c));

        let caster = e.mem.u32(base + 0x80);
        e.call(RESET_0062A2B0, &args![caster.wrapping_add(0x30)]);
        let result = e.mem.u32(base + 0x84);
        e.call(RESET_0062A290, &args![result]);
        let counter = e.mem.u32(base + 0x88).wrapping_add(1);
        e.mem.set_u32(base + 0x88, counter);
        let caster = e.mem.u32(base + 0x80);
        e.call(SET_FIRST_004A3DA0, &args![caster, start]);
        let caster = e.mem.u32(base + 0x80);
        e.call(SET_SECOND_004A3EB0, &args![caster, end]);
        let caster = e.mem.u32(base + 0x80);
        let world = e.mem.u32(base + 0x7c);
        if e.vcall(world, SLOT_TEST, &args![caster]).bool() {
            return false;
        }
        let counter = e.mem.u32(base + 0x88).wrapping_add(1);
        e.mem.set_u32(base + 0x88, counter);
        let reach = x87_store32(x87_add(
            x87_load32(e.mem.u32(base + 0x8c)),
            x87_m32(e.mem.u32(base + 0x90)),
        ));
        e.mem.set_u32(l(0x78), reach);
        let reach = pushed(e, l(0x78));
        let offset = e
            .call(POINT_SCALED, &args![POINT_011A9484, l(0x58), reach])
            .u32();
        e.call(POINT_SUBTRACT, &args![end, l(0x28), offset]);
        let below = l(0x28);
        let caster = e.mem.u32(base + 0x80);
        e.call(RESET_0062A2B0, &args![caster.wrapping_add(0x30)]);
        let result = e.mem.u32(base + 0x84);
        e.call(RESET_0062A290, &args![result]);
        let caster = e.mem.u32(base + 0x80);
        e.call(SET_FIRST_004A3DA0, &args![caster, end]);
        let caster = e.mem.u32(base + 0x80);
        e.call(SET_SECOND_004A3EB0, &args![caster, below]);
        let caster = e.mem.u32(base + 0x80);
        let world = e.mem.u32(base + 0x7c);
        if !e.vcall(world, SLOT_TEST, &args![caster]).bool() {
            return false;
        }
        let result = e.mem.u32(base + 0x84);
        let hit = e.call(RESULT_00460140, &args![result]).u32();
        let fraction = pushed(e, hit + 0x10);
        let span = e.call(POINT_SUBTRACT, &args![below, l(0x64), end]).u32();
        let step = e.call(POINT_SCALED, &args![span, l(0x70), fraction]).u32();
        e.call(POINT_ADD, &args![end, l(0x34), step]);
        for offset in [0, 4, 8] {
            let word = e.mem.u32(l(0x34) + offset);
            e.mem.set_u32(out.addr() + offset, word);
        }
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006e_6f90, fn_006e6f90(Ptr, Ptr, Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }

    fn write(e: &mut Engine, p: u32, v: [f32; 3]) {
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *x);
        }
    }

    /// The object (`+0x7c` a world whose slot `0xc8` answers `answers` in
    /// turn, `+0x80` and `+0x84` two objects, the counter at `+0x88`, the
    /// floats at `+0x8c` and `+0x90`), point doubles and the up point at
    /// `011a9484` set to (0, 0, 1).
    fn probe_case(answers: [bool; 2]) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(POINT_011A9484 & !0xfff, 0x1000);
        write(&mut e, POINT_011A9484, [0.0, 0.0, 1.0]);
        e.register(POINT_SCALED, |e, a| {
            let (p, s) = (read(e, a[0]), f32::from_bits(a[2]));
            write(e, a[1], [p[0] * s, p[1] * s, p[2] * s]);
            a[1].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            let (p, q) = (read(e, a[0]), read(e, a[2]));
            write(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            let (p, q) = (read(e, a[0]), read(e, a[2]));
            write(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            a[1].into_ret()
        });
        for addr in [RESET_0062A2B0, RESET_0062A290] {
            e.register(addr, |_, _| Ret::default());
        }
        // The points given to the caster are recorded at its +0 and +0xc.
        e.register(SET_FIRST_004A3DA0, |e, a| {
            let p = read(e, a[1]);
            write(e, a[0], p);
            Ret::default()
        });
        e.register(SET_SECOND_004A3EB0, |e, a| {
            let p = read(e, a[1]);
            write(e, a[0] + 0xc, p);
            Ret::default()
        });
        e.register(RESULT_00460140, |_, a| (a[0] + 0x10).into_ret());
        let mut turn = 0usize;
        e.register_double(0x0f00_00c8, move |_, _| {
            let answer = answers[turn];
            turn += 1;
            u32::from(answer).into_ret()
        });
        let vtable = 0x0f00_0000;
        e.map(vtable, 0x100);
        e.mem.set_u32(vtable + SLOT_TEST, 0x0f00_00c8);
        let world = e.mem.alloc(4);
        e.mem.set_u32(world, vtable);
        let caster = e.mem.alloc(0x40);
        let result = e.mem.alloc(0x24);
        // `00460140` gives the result plus 0x10; the fraction is at its +0x10.
        e.mem.set_f32(result + 0x20, 0.25);
        let this = e.mem.alloc(0x94);
        e.mem.set_u32(this + 0x7c, world);
        e.mem.set_u32(this + 0x80, caster);
        e.mem.set_u32(this + 0x84, result);
        e.mem.set_f32(this + 0x8c, 3.0);
        e.mem.set_f32(this + 0x90, 1.0);
        (e, this)
    }

    #[test]
    fn fn_006e6f90_probes_down_from_the_raised_end() {
        let (mut e, this) = probe_case([false, true]);
        let from = e.mem.alloc(0xc);
        write(&mut e, from, [1.0, 2.0, 3.0]);
        let to = e.mem.alloc(0xc);
        write(&mut e, to, [5.0, 6.0, 7.0]);
        let out = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006e_6f90, &args![this, from, to, out]).bool());
        let log = e.call_log.take().unwrap();
        let caster = e.mem.u32(this + 0x80);
        // The second test runs from `to` raised by 1 down by 4.
        assert_eq!(read(&e, caster), [5.0, 6.0, 8.0]);
        assert_eq!(read(&e, caster + 0xc), [5.0, 6.0, 4.0]);
        // A quarter of the way: z 8 - 1.
        assert_eq!(read(&e, out), [5.0, 6.0, 7.0]);
        assert_eq!(e.mem.u32(this + 0x88), 2);
        let resets = log
            .iter()
            .filter(|(a, w)| *a == RESET_0062A2B0 && w[0] == caster + 0x30)
            .count();
        assert_eq!(resets, 2);
    }

    #[test]
    fn fn_006e6f90_fails_when_the_first_hits_or_the_second_misses() {
        // The first test answers true: false, after one test.
        let (mut e, this) = probe_case([true, true]);
        let point = e.mem.alloc(0xc);
        let out = e.mem.alloc(0xc);
        write(&mut e, out, [9.0; 3]);
        assert!(!e.call(0x006e_6f90, &args![this, point, point, out]).bool());
        assert_eq!(e.mem.u32(this + 0x88), 1);
        assert_eq!(read(&e, out), [9.0; 3]);
        let caster = e.mem.u32(this + 0x80);
        // The first test went from the start to the end, both raised by 1.
        assert_eq!(read(&e, caster), [0.0, 0.0, 1.0]);
        assert_eq!(read(&e, caster + 0xc), [0.0, 0.0, 1.0]);
        // The second misses: false.
        let (mut e, this) = probe_case([false, false]);
        let point = e.mem.alloc(0xc);
        let out = e.mem.alloc(0xc);
        write(&mut e, out, [9.0; 3]);
        assert!(!e.call(0x006e_6f90, &args![this, point, point, out]).bool());
        assert_eq!(e.mem.u32(this + 0x88), 2);
        assert_eq!(read(&e, out), [9.0; 3]);
    }
}
