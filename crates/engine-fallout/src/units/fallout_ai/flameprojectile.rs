//! `fallout/ai/combat/flameprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `0.3` (`double`, the `float` widened): the age after which the flame's
/// particle add-on nodes go.
const ADDON_AGE: u32 = 0x0102_90d0;
/// `1.0` (`double`): the age after which the flame may end.
const END_AGE: u32 = 0x0101_2070;
/// `00874480` (thiscall): the projectile's shooter.
const GET_SHOOTER: u32 = 0x0087_4480;
/// `00461130` (thiscall): a node's transform (the matrix at its start).
const TRANSFORM: u32 = 0x0046_1130;
/// `NiMatrix3::operator*` (`0043f8d0`, thiscall(out, other)).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `0045bb80` (thiscall): a node's world point.
const NODE_POINT: u32 = 0x0045_bb80;
/// `00450f90` (thiscall(byte) on the projectile's 3D).
const SET_3D_BYTE: u32 = 0x0045_0f90;

// Translated from 009b34c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FlameProjectile::UpdateProjectile` (Xbox PDB; thiscall(`time`, a
/// `float`), `RET 4`). Nothing when `00440d80` answers true. With the
/// shooter's node (virtual +0x20C) and the projectile's 3D (virtual
/// +0x1D0): the 3D gets `rotation (+0x94) * node transform` (`0043fa80`),
/// the projectile's orientation the same product
/// (`Projectile::SetProjectileOrientation`), the 3D and the reference the
/// node's point (`00440460`, `SetLocationOnReference`). While the age
/// (+0x150) is exactly 0: virtual +0x328 and `CheckExplosion(time)`, then
/// the 3D's virtual +0xC gets its particle add-on nodes
/// (`AddMasterParticleAddonNodes`) and `00450f90(0)`; past 0.3 instead
/// `00450f90(1)` and `RemoveMasterParticleAddonNodes`. The age grows by
/// `time`; past 1.0 the flame ends (`009bc8f0`) when `008256d0` on +0x88
/// answers true or the byte at +0x90 is set.
pub fn flame_projectile_update_projectile(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    if e.call(0x0044_0d80, &args![this]).bool() {
        return;
    }
    e.with_stack(0x64, |e, frame| {
        let ebp = frame.addr() + 0x64;
        let node = if e.call(GET_SHOOTER, &args![this]).u32() != 0 {
            let shooter = e.call(GET_SHOOTER, &args![this]).u32();
            e.vcall(shooter, 0x20c, &args![]).u32()
        } else {
            0
        };
        let flame = e.vcall(base, 0x1d0, &args![]).u32();
        if node != 0 && flame != 0 {
            let transform = e.call(TRANSFORM, &args![node]).u32();
            let product = e
                .call(MATRIX_MULTIPLY, &args![base + 0x94, ebp - 0x2c, transform])
                .u32();
            e.call(0x0043_fa80, &args![flame, product]);
            let transform = e.call(TRANSFORM, &args![node]).u32();
            let product = e
                .call(MATRIX_MULTIPLY, &args![base + 0x94, ebp - 0x50, transform])
                .u32();
            e.call(0x009c_6330, &args![this, product]);
            let point = e.call(NODE_POINT, &args![node]).u32();
            e.call(0x0044_0460, &args![flame, point]);
            let point = e.call(NODE_POINT, &args![node]).u32();
            e.call(0x0057_5830, &args![this, point]);
        }
        // `FCOMP` with the `double` 0, `TEST AH,0x44`/`JP`: only an equal
        // (ordered) age passes.
        let zero = e.global::<f64>(ZERO);
        if x87_m32(e.mem.u32(base + 0x150)) == zero {
            e.vcall(base, 0x328, &args![]);
            e.call(0x009c_3190, &args![this, x87_quiet_bits(time.to_bits())]);
        }
        if x87_m32(e.mem.u32(base + 0x150)) == zero {
            let flame = e.vcall(base, 0x1d0, &args![]).u32();
            let nodes = e.vcall(flame, 0xc, &args![]).u32();
            e.call(0x0057_8060, &args![nodes]);
            let flame = e.vcall(base, 0x1d0, &args![]).u32();
            e.call(SET_3D_BYTE, &args![flame, 0u32]);
        } else if x87_m32(e.mem.u32(base + 0x150)) > e.global::<f64>(ADDON_AGE) {
            // `TEST AH,0x41`/`JNZ`: only an ordered "above" passes.
            let flame = e.vcall(base, 0x1d0, &args![]).u32();
            e.call(SET_3D_BYTE, &args![flame, 1u32]);
            let flame = e.vcall(base, 0x1d0, &args![]).u32();
            let nodes = e.vcall(flame, 0xc, &args![]).u32();
            e.call(0x0057_8170, &args![nodes]);
        }
        let age = x87_add(x87_load32(e.mem.u32(base + 0x150)), x87_m32(time.to_bits()));
        e.mem.set_u32(base + 0x150, x87_store32(age));
        if x87_load32(e.mem.u32(base + 0x150)) > e.global::<f64>(END_AGE)
            && (e.call(0x0082_56d0, &args![base + 0x88]).bool() || e.mem.u8(base + 0x90) != 0)
        {
            e.call(0x009b_c8f0, &args![this]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x009b_34c0,
        flame_projectile_update_projectile(Ptr, f32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];
    type Logged = Vec<(u32, Vec<u32>)>;

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const FLAME_VTABLE: u32 = 0x0a00_0000;
    const SHOOTER_VTABLE: u32 = 0x0a01_0000;
    const NODES_VTABLE: u32 = 0x0a02_0000;

    fn slots(e: &mut Engine, address: u32, entries: &[(u32, u32)]) {
        let size = entries.iter().map(|(s, _)| *s).max().unwrap() / 4 + 1;
        let mut words = vec![0u32; size as usize];
        for (slot, target) in entries {
            words[*slot as usize / 4] = *target;
        }
        e.put_vtable(address, &words);
    }

    fn update(dead: bool, shooter: bool, age: f32, ends: bool, time: f32) -> (u32, Engine, Logged) {
        let mut e = Engine::new();
        for (address, value) in [
            (ZERO, 0.0f64),
            (ADDON_AGE, f64::from(0.3f32)),
            (END_AGE, 1.0),
        ] {
            e.map(address & !0xfff, 0x1000);
            e.set_global(address, value);
        }
        let nodes = e.mem.alloc(8);
        e.mem.set_u32(nodes, NODES_VTABLE);
        slots(&mut e, NODES_VTABLE, &[(0xc, 0x0b00_000c)]);
        e.register(0x0b00_000c, |_, _| 0x0c00_0000u32.into_ret());
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, FLAME_VTABLE);
        e.mem.set_f32(this + 0x150, age);
        slots(
            &mut e,
            FLAME_VTABLE,
            &[(0x1d0, 0x0b00_01d0), (0x328, 0x0b00_0328)],
        );
        e.register_double(0x0b00_01d0, move |_, _| nodes.into_ret());
        let actor = if shooter { e.mem.alloc(8) } else { 0 };
        if shooter {
            e.mem.set_u32(actor, SHOOTER_VTABLE);
        }
        slots(&mut e, SHOOTER_VTABLE, &[(0x20c, 0x0b00_020c)]);
        e.register(0x0b00_020c, |_, _| 0x0d00_0000u32.into_ret());
        e.register_double(GET_SHOOTER, move |_, _| actor.into_ret());
        e.register_double(0x0044_0d80, move |_, _| u32::from(dead).into_ret());
        e.register(TRANSFORM, |_, a| (a[0] + 0x10).into_ret());
        e.register(MATRIX_MULTIPLY, |_, a| a[1].into_ret());
        e.register(NODE_POINT, |_, a| (a[0] + 0x20).into_ret());
        e.register_double(0x0082_56d0, move |_, _| u32::from(ends).into_ret());
        for address in [
            0x0b00_0328,
            0x0043_fa80,
            0x009c_6330,
            0x0044_0460,
            0x0057_5830,
            0x009c_3190,
            0x0057_8060,
            0x0057_8170,
            SET_3D_BYTE,
            0x009b_c8f0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x009b_34c0, &args![this, time]);
        let log = e.call_log.take().unwrap();
        (this, e, log)
    }

    #[test]
    fn a_dead_flame_does_nothing() {
        let (_, _, log) = update(true, true, 0.0, true, 0.5);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn a_new_flame_follows_the_node_and_gets_its_addons() {
        let (this, e, log) = update(false, true, 0.0, false, 0.5);
        let multiplied = calls(&log, MATRIX_MULTIPLY);
        assert_eq!(multiplied.len(), 2);
        assert_eq!(multiplied[0][0], this + 0x94);
        assert_eq!(multiplied[0][2], 0x0d00_0010);
        assert_eq!(calls(&log, 0x0043_fa80)[0][1], multiplied[0][1]);
        assert_eq!(calls(&log, 0x009c_6330), vec![vec![this, multiplied[1][1]]]);
        assert_eq!(calls(&log, 0x0057_5830), vec![vec![this, 0x0d00_0020]]);
        assert_eq!(calls(&log, 0x0b00_0328).len(), 1);
        assert_eq!(calls(&log, 0x009c_3190), vec![vec![this, 0.5f32.to_bits()]]);
        assert_eq!(calls(&log, 0x0057_8060), vec![vec![0x0c00_0000]]);
        assert_eq!(calls(&log, SET_3D_BYTE)[0][1], 0);
        assert!(calls(&log, 0x0057_8170).is_empty());
        assert_eq!(e.mem.f32(this + 0x150), 0.5);
        assert!(calls(&log, 0x009b_c8f0).is_empty());
    }

    #[test]
    fn an_older_flame_drops_its_addons_and_ends() {
        let (this, e, log) = update(false, false, 0.75, true, 0.5);
        assert!(calls(&log, MATRIX_MULTIPLY).is_empty());
        assert!(calls(&log, 0x009c_3190).is_empty());
        assert_eq!(calls(&log, SET_3D_BYTE)[0][1], 1);
        assert_eq!(calls(&log, 0x0057_8170), vec![vec![0x0c00_0000]]);
        assert_eq!(e.mem.f32(this + 0x150), 1.25);
        assert_eq!(calls(&log, 0x0082_56d0), vec![vec![this + 0x88]]);
        assert_eq!(calls(&log, 0x009b_c8f0), vec![vec![this]]);

        // Past 1.0 but neither test: it stays; the byte at +0x90 ends it.
        let (_, _, log) = update(false, false, 0.75, false, 0.5);
        assert!(calls(&log, 0x009b_c8f0).is_empty());
        // Between 0 and 0.3: no add-on change.
        let (_, _, log) = update(false, false, 0.25, false, 0.0);
        assert!(calls(&log, SET_3D_BYTE).is_empty());
    }

    #[test]
    fn the_byte_at_0x90_also_ends_an_old_flame() {
        let mut e = Engine::new();
        for (address, value) in [
            (ZERO, 0.0f64),
            (ADDON_AGE, f64::from(0.3f32)),
            (END_AGE, 1.0),
        ] {
            e.map(address & !0xfff, 0x1000);
            e.set_global(address, value);
        }
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, FLAME_VTABLE);
        e.mem.set_f32(this + 0x150, 2.0);
        e.mem.set_u8(this + 0x90, 1);
        let nodes = e.mem.alloc(8);
        e.mem.set_u32(nodes, NODES_VTABLE);
        slots(&mut e, NODES_VTABLE, &[(0xc, 0x0b00_000c)]);
        slots(
            &mut e,
            FLAME_VTABLE,
            &[(0x1d0, 0x0b00_01d0), (0x328, 0x0b00_0328)],
        );
        e.register_double(0x0b00_01d0, move |_, _| nodes.into_ret());
        for address in [
            0x0044_0d80,
            GET_SHOOTER,
            0x0b00_000c,
            0x0082_56d0,
            SET_3D_BYTE,
            0x0057_8170,
            0x009b_c8f0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x009b_34c0, &args![this, 0.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x009b_c8f0), vec![vec![this]]);
    }
}
