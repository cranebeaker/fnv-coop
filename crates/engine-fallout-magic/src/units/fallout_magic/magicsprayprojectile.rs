//! `fallout/magic/magicsprayprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `MagicProjectile::MagicProjectile` (engine map, `0081dd00`, `thiscall`,
/// ten words) and `MobileObject::InitHavok` (`00930c70`, `thiscall`).
const MAGIC_PROJECTILE_CONSTRUCT: u32 = 0x0081_dd00;
const INIT_HAVOK: u32 = 0x0093_0c70;
/// `MagicSprayProjectile`'s two vtables (`0107b8c4` at +0, `0107b8b8` at
/// +0x18).
const MAGIC_SPRAY_PROJECTILE_VTABLE: u32 = 0x0107_b8c4;
const MAGIC_SPRAY_PROJECTILE_VTABLE_18: u32 = 0x0107_b8b8;

// Translated from 00822920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicSprayProjectile::MagicSprayProjectile` (engine map; `thiscall`,
/// `RET 0x28`: caster, magic item, dominant effect, effect setting, the
/// position and the rotation as three words each): the `MagicProjectile`
/// base with the same words, both vtables, then
/// `MobileObject::InitHavok`. Returns `this`.
// Translation incomplete: 00822920 - the FS:[0] exception frame (unwinding the base on a throw) is not translated
#[allow(clippy::too_many_arguments)]
pub fn magic_spray_projectile_magic_spray_projectile(
    e: &mut Engine,
    this: Ptr,
    caster: u32,
    spell: u32,
    dominant_effect: u32,
    setting: u32,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    rotation_x: u32,
    rotation_y: u32,
    rotation_z: u32,
) -> Ptr {
    let base = this.addr();
    e.call(
        MAGIC_PROJECTILE_CONSTRUCT,
        &args![
            base,
            caster,
            spell,
            dominant_effect,
            setting,
            position_x,
            position_y,
            position_z,
            rotation_x,
            rotation_y,
            rotation_z
        ],
    );
    e.mem.set_u32(base, MAGIC_SPRAY_PROJECTILE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x18), MAGIC_SPRAY_PROJECTILE_VTABLE_18);
    e.call(INIT_HAVOK, &args![base]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0082_2920,
        magic_spray_projectile_magic_spray_projectile(
            Ptr,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        ) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_builds_the_base_then_havok() {
        let mut e = Engine::new();
        e.register(MAGIC_PROJECTILE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(INIT_HAVOK, |e, a| {
            // The vtables are in place before the Havok set-up.
            assert_eq!(e.mem.u32(a[0]), MAGIC_SPRAY_PROJECTILE_VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(0xa8);
        let words: Vec<u32> = (1..=10).collect();
        let mut call = vec![this];
        call.extend_from_slice(&words);
        e.call_log = Some(vec![]);
        let r = e.call(0x0082_2920, &call);
        let log = e.call_log.take().unwrap();
        assert_eq!(r.u32(), this);
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == MAGIC_PROJECTILE_CONSTRUCT)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(base, vec![call.clone()]);
        assert_eq!(e.mem.u32(this + 0x18), MAGIC_SPRAY_PROJECTILE_VTABLE_18);
        assert_eq!(log.iter().filter(|(a, _)| *a == INIT_HAVOK).count(), 1);
    }
}
