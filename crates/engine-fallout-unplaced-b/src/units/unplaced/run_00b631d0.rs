//! `(unplaced)/run_00b631d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The per-object call `00c4edf0` (`thiscall (object, &local, flag)`,
/// `RET 8`), the object destructor `00c4ed40` (`thiscall`), and the memory
/// manager (`011f6238`) with its free (`00aa4060`, `thiscall`, one word).
const RELEASE_OBJECT: u32 = 0x00c4_edf0;
const OBJECT_DESTRUCTOR: u32 = 0x00c4_ed40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;

/// One object slot's clean-up: `00c4edf0` on it (with the local and 1),
/// then, when the slot still holds an object, its destructor and its block
/// freed; the slot cleared.
fn clear_slot(e: &mut Engine, slot: u32, local: u32) {
    let object = e.mem.u32(slot);
    e.call(RELEASE_OBJECT, &args![object, local, 1u32]);
    let object = e.mem.u32(slot);
    if object != 0 {
        e.call(OBJECT_DESTRUCTOR, &args![object]);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, object]);
    }
    e.mem.set_u32(slot, 0);
}

// Translated from 00b631d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases every object this holds (the engine map has no name): each
/// non-null entry of the array at +0x8C (16-bit count at +0x8A), the three
/// slots at +0x48 (0x14 bytes apart, each one's byte at +4 cleared too),
/// then, while the word at +0x148 is nonzero, the object at +0x14 of each
/// node's data walking the list from +0x140 (the exe re-reads only that
/// word, so the walk ends when a clean-up clears it). Each object gets
/// `00c4edf0` with the address of a stack word that starts as `this` (the
/// exe's saved `ECX` slot) and 1, then is destroyed and freed. The byte at
/// +0x40 is cleared, both 16-bit counts zeroed and the array freed.
pub fn fn_00b631d0(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, object);
        if e.mem.u16(object + 0x8a) != 0 {
            let mut index = 0u32;
            loop {
                // The array pointer is read again before each access.
                let item = e.mem.u32(e.mem.u32(object + 0x8c).wrapping_add(4 * index));
                if item != 0 {
                    e.call(RELEASE_OBJECT, &args![item, local, 1u32]);
                    let item = e.mem.u32(e.mem.u32(object + 0x8c).wrapping_add(4 * index));
                    if item != 0 {
                        e.call(OBJECT_DESTRUCTOR, &args![item]);
                        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, item]);
                    }
                    let array = e.mem.u32(object + 0x8c);
                    e.mem.set_u32(array.wrapping_add(4 * index), 0);
                }
                index = index.wrapping_add(1);
                if index as i32 >= i32::from(e.mem.u16(object + 0x8a)) {
                    break;
                }
            }
        }
        for i in 0..3 {
            let slot = object + 0x48 + 0x14 * i;
            if e.mem.u32(slot) != 0 {
                clear_slot(e, slot, local);
                e.mem.set_u8(slot + 4, 0);
            }
        }
        e.mem.set_u8(object + 0x40, 0);
        let mut node = e.mem.u32(object + 0x140);
        while e.mem.u32(object + 0x148) != 0 {
            let data = e.mem.u32(node.wrapping_add(8));
            node = e.mem.u32(node);
            if data != 0 && e.mem.u32(data.wrapping_add(0x14)) != 0 {
                clear_slot(e, data.wrapping_add(0x14), local);
            }
        }
    });
    let array = e.mem.u32(object + 0x8c);
    e.mem.set_u16(object + 0x8a, 0);
    e.mem.set_u16(object + 0x88, 0);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, array]);
    e.mem.set_u32(object + 0x8c, 0);
}

/// `NiAVObject::GetProperty` (`00a59d30`, engine map; `thiscall`, one word), the two
/// `cdecl` calls `00bbb840` (no words) and `00b704c0` (one word), and the per-light call
/// `00b71360` (`cdecl`, three words: index, light, float bits).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const CALL_00BBB840: u32 = 0x00bb_b840;
const CALL_00B704C0: u32 = 0x00b7_04c0;
const LIGHT_CALL_00B71360: u32 = 0x00b7_1360;
/// Globals `00b63320` reads and writes: the word passed to the geometry's slot 0xdc,
/// the current pass and pass number, two flag bytes, and the four words copied from
/// `011a9bd0` to `011fa0d0`.
const GEOMETRY_ARGUMENT_011F4748: u32 = 0x011f_4748;
const CURRENT_PASS_011F91E0: u32 = 0x011f_91e0;
const CURRENT_PASS_ENUM_011F91E4: u32 = 0x011f_91e4;
const FLAG_011F94A6: u32 = 0x011f_94a6;
const FLAG_011F91A7: u32 = 0x011f_91a7;
const COPY_SOURCE_011A9BD0: u32 = 0x011a_9bd0;
const COPY_TARGET_011FA0D0: u32 = 0x011f_a0d0;

// Translated from 00b63320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words; its one caller, inside `00b643xx`,
/// passes a `BSShaderProperty::RenderPass` (Xbox PDB layout: `pGeometry` +0,
/// `ucNumLights` +9, `ppSceneLights` +0xc) and that pass's `usPassEnum`). Stores the
/// pass and the number in `011f91e0`/`011f91e4`, then asks the geometry for property 3
/// (`NiAVObject::GetProperty`). With one whose word at +0x1c minus 1 is below 0xc: its
/// float at +0x6c is kept (through the FPU, quieted), `00bbb840` runs when the byte at
/// `011f94a6` is set, `00b704c0(0)` runs unless the number is 0x10..=0x13 or
/// 0x1c8..=0x1cb, `00b71360(index, light, float)` runs for each light, and with the byte
/// at `011f91a7` set the four words at `011a9bd0` are copied to `011fa0d0`. Always ends
/// with the geometry's slot 0xdc called with the word `011f4748` held on entry; the
/// function ends in that call, so its `EAX` is answered.
pub fn fn_00b63320(e: &mut Engine, pass: u32, pass_enum: u32) -> u32 {
    let argument = e.global::<u32>(GEOMETRY_ARGUMENT_011F4748);
    e.set_global(CURRENT_PASS_ENUM_011F91E4, pass_enum);
    e.set_global(CURRENT_PASS_011F91E0, pass);
    let geometry = e.mem.u32(pass);
    let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
    if property != 0 && e.mem.u32(property + 0x1c).wrapping_sub(1) <= 0xb {
        let value = x87_quiet_bits(e.mem.u32(property + 0x6c));
        if e.global::<u8>(FLAG_011F94A6) != 0 {
            e.call(CALL_00BBB840, &[]);
        }
        let keeps = (0x10..=0x13).contains(&pass_enum)
            || pass_enum == 0x1c8
            || pass_enum == 0x1ca
            || pass_enum == 0x1c9
            || pass_enum == 0x1cb;
        if !keeps {
            e.call(CALL_00B704C0, &args![0u32]);
        }
        let mut index = 0u8;
        if e.mem.u8(pass + 9) != 0 {
            loop {
                let lights = e.mem.u32(pass + 0xc);
                let light = e.mem.u32(lights.wrapping_add(4 * u32::from(index)));
                e.call(
                    LIGHT_CALL_00B71360,
                    &args![u32::from(index), light, x87_quiet_bits(value)],
                );
                index = index.wrapping_add(1);
                if index >= e.mem.u8(pass + 9) {
                    break;
                }
            }
        }
        if e.global::<u8>(FLAG_011F91A7) != 0 {
            for word in 0..4 {
                let value = e.global::<u32>(COPY_SOURCE_011A9BD0 + 4 * word);
                e.set_global(COPY_TARGET_011FA0D0 + 4 * word, value);
            }
        }
    }
    let geometry = e.mem.u32(pass);
    e.vcall(geometry, 0xdc, &args![argument]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_31d0, fn_00b631d0(Ptr)),
        entry!(0x00b6_3320, fn_00b63320(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_held_object_is_released_destroyed_and_freed() {
        let mut e = Engine::new();
        e.register(OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x150);
        // The list's clean-up clears the word at +0x148, which ends the walk.
        e.register_double(RELEASE_OBJECT, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), this);
            assert_eq!(a[2], 1);
            if a[0] == 0x6003 {
                e.mem.set_u32(this + 0x148, 0);
            }
            Ret::default()
        });
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, 0x6001);
        e.mem.set_u32(this + 0x8c, array);
        e.mem.set_u16(this + 0x8a, 2);
        e.mem.set_u16(this + 0x88, 4);
        e.mem.set_u32(this + 0x48 + 0x14, 0x6002);
        e.mem.set_u8(this + 0x48 + 0x14 + 4, 1);
        e.mem.set_u8(this + 0x40, 3);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data + 0x14, 0x6003);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, data);
        e.mem.set_u32(this + 0x140, node);
        e.mem.set_u32(this + 0x148, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b6_31d0, &args![this]);
        let log = e.call_log.take().unwrap();
        let released: Vec<u32> = log
            .iter()
            .filter(|call| call.0 == RELEASE_OBJECT)
            .map(|call| call.1[0])
            .collect();
        assert_eq!(released, vec![0x6001, 0x6002, 0x6003]);
        let freed: Vec<u32> = log
            .iter()
            .filter(|call| call.0 == MEMORY_MANAGER_FREE)
            .map(|call| call.1[1])
            .collect();
        assert_eq!(freed, vec![0x6001, 0x6002, 0x6003, array]);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(this + 0x48 + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x48 + 0x14 + 4), 0);
        assert_eq!(e.mem.u32(data + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x40), 0);
        assert_eq!(e.mem.u16(this + 0x8a), 0);
        assert_eq!(e.mem.u16(this + 0x88), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
    }

    fn pass_setup(e: &mut Engine, kind: u32, lights: u8) -> (u32, u32, u32) {
        for page in [0x011f_4000u32, 0x011f_9000, 0x011f_a000, 0x011a_9000] {
            e.map(page, 0x1000);
        }
        e.set_global(GEOMETRY_ARGUMENT_011F4748, 0x4748u32);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xdc, 0x7000_00dc);
        let geometry = e.mem.alloc(0x10);
        e.mem.set_u32(geometry, vtable);
        let property = e.mem.alloc(0x70);
        e.mem.set_u32(property + 0x1c, kind);
        e.mem.set_u32(property + 0x6c, 0x7f80_0001);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, 0x5001);
        e.mem.set_u32(array + 4, 0x5002);
        let pass = e.mem.alloc(0x10);
        e.mem.set_u32(pass, geometry);
        e.mem.set_u8(pass + 9, lights);
        e.mem.set_u32(pass + 0xc, array);
        e.register_double(GET_PROPERTY, move |_, a| {
            assert_eq!(a, &[geometry, 3]);
            Ret {
                eax: property,
                ..Ret::default()
            }
        });
        for addr in [CALL_00BBB840, CALL_00B704C0, LIGHT_CALL_00B71360] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(0x7000_00dc, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        (pass, geometry, property)
    }

    #[test]
    fn a_pass_with_a_lit_property_runs_each_light_then_the_geometry_slot() {
        let mut e = Engine::new();
        let (pass, geometry, _) = pass_setup(&mut e, 4, 2);
        e.set_global(FLAG_011F94A6, 1u8);
        e.set_global(FLAG_011F91A7, 1u8);
        e.set_global(COPY_SOURCE_011A9BD0 + 8, 0x1234u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_3320, &args![pass, 0x20u32]).u32(), 0x55);
        let log = e.call_log.take().unwrap();
        let lights: Vec<_> = log
            .iter()
            .filter(|c| c.0 == LIGHT_CALL_00B71360)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            lights,
            vec![vec![0, 0x5001, 0x7fc0_0001], vec![1, 0x5002, 0x7fc0_0001]]
        );
        assert_eq!(log.iter().filter(|c| c.0 == CALL_00BBB840).count(), 1);
        assert!(log.iter().any(|c| c.0 == CALL_00B704C0 && c.1 == vec![0]));
        assert!(log
            .iter()
            .any(|c| c.0 == 0x7000_00dc && c.1 == vec![geometry, 0x4748]));
        assert_eq!(e.global::<u32>(COPY_TARGET_011FA0D0 + 8), 0x1234);
        assert_eq!(e.global::<u32>(CURRENT_PASS_011F91E0), pass);
        assert_eq!(e.global::<u32>(CURRENT_PASS_ENUM_011F91E4), 0x20);
    }

    #[test]
    fn listed_pass_numbers_and_other_properties_skip_the_calls() {
        let mut e = Engine::new();
        let (pass, _, property) = pass_setup(&mut e, 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b6_3320, &args![pass, 0x1c9u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log
            .iter()
            .any(|c| c.0 == CALL_00B704C0 || c.0 == CALL_00BBB840));
        e.mem.set_u32(property + 0x1c, 0);
        e.set_global(FLAG_011F94A6, 1u8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_3320, &args![pass, 0x20u32]).u32(), 0x55);
        let log = e.call_log.take().unwrap();
        assert!(!log
            .iter()
            .any(|c| c.0 == CALL_00B704C0 || c.0 == CALL_00BBB840));
    }
}
