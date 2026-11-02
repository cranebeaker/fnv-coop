//! `BSShader/run_00bb7d70` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `double` the depth is multiplied by (`010b9e30`, 0.83 in the exe).
const DEPTH_FACTOR: u32 = 0x010b_9e30;
/// The `double` the result is scaled by (`0103a480`, 163840.0 in the exe).
const DISTANCE_SCALE: u32 = 0x0103_a480;
/// The `float` copied to `+0x30` (`010317c4`, 163840.0 in the exe).
const END_DISTANCE: u32 = 0x0103_17c4;

// Translated from 00bb8140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSShader`). `thiscall`, one `float` (`RET 4`): stores
/// `(1 - value * [010b9e30]) * [0103a480]` (x87, the two `double`s) as the `float` at `+0x2c`
/// and copies the `float` at `010317c4` to `+0x30` (`FLD`/`FSTP`: a signalling NaN comes out
/// quiet). On a `BSFogProperty` these are `fStartDistance` and `fEndDistance` (Xbox PDB);
/// `BSFogProperty`'s constructor inlines the same code on its depth.
pub fn fn_00bb8140(e: &mut Engine, this: Ptr, value: f32) {
    let factor: f64 = e.global(DEPTH_FACTOR);
    let scale: f64 = e.global(DISTANCE_SCALE);
    let product = x87_mul(x87_load32(value.to_bits()), factor);
    let start = x87_mul(x87_sub(1.0, product), scale);
    e.mem
        .set_u32(this.addr().wrapping_add(0x2c), x87_store32(start));
    let end = x87_quiet_bits(e.global::<u32>(END_DISTANCE));
    e.mem.set_u32(this.addr().wrapping_add(0x30), end);
}

/// `InterlockedDecrement` (KERNEL32 import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The memory manager object and its deallocation (`00aa4060`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The vtable at `010b9d28` (`TileShaderProperty`'s, installed by `00bb7c30`).
const TILE_SHADER_PROPERTY_VTABLE: u32 = 0x010b_9d28;
/// The shader property's destructor `00baa0e0` (`thiscall`).
const SHADER_PROPERTY_DESTRUCTOR_00BAA0E0: u32 = 0x00ba_a0e0;
/// The allocation `00aa13e0` (`cdecl`, one word: the size).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `TileShaderProperty::TileShaderProperty` (`00bb7c30`, `thiscall`, one word).
const TILE_SHADER_PROPERTY_CONSTRUCTOR: u32 = 0x00bb_7c30;
/// The sized free `00aa1460` (`cdecl`, two words: the block and its size).
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

/// Releases a reference (`InterlockedDecrement` on `+4`; at zero the object's slot +4,
/// `DeleteThis`).
fn release_reference(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00bb7d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tile shader property's destructor (unnamed in the engine map; [`fn_00bb7f60`], slot 0
/// of the vtable at `010b9d28`, runs it). Sets that vtable, releases and clears the
/// texture at `+0x60`, frees the file name's text at `+0x94` (when set) and clears the
/// string (`+0x94`, `+0x98`, `+0x9a`), releases the alpha map at `+0x64`, releases the
/// `+0x60` texture again when set (the members' own destructors), then runs the shader
/// property's destructor (`00baa0e0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bb7d70 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00bb7d70(e: &mut Engine, this: Ptr) {
    let at = |off: u32| this.addr().wrapping_add(off);
    e.mem.set_u32(this.addr(), TILE_SHADER_PROPERTY_VTABLE);
    let texture = e.mem.u32(at(0x60));
    if texture != 0 {
        release_reference(e, texture);
        e.mem.set_u32(at(0x60), 0);
    }
    let text = e.mem.u32(at(0x94));
    if text != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, text]);
    }
    e.mem.set_u32(at(0x94), 0);
    e.mem.set_u16(at(0x9a), 0);
    e.mem.set_u16(at(0x98), 0);
    let alpha_map = e.mem.u32(at(0x64));
    if alpha_map != 0 {
        release_reference(e, alpha_map);
    }
    let texture = e.mem.u32(at(0x60));
    if texture != 0 {
        release_reference(e, texture);
    }
    e.call(SHADER_PROPERTY_DESTRUCTOR_00BAA0E0, &args![this]);
}

// Translated from 00bb7ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The stream's creation function for `"TileShaderProperty"` (the shader manager's
/// registration table pairs the name at `010a922c` with this address): allocates 0xb0 bytes
/// (`00aa13e0`) and constructs a tile shader property in them (`00bb7c30` with 0), answering
/// what the constructor answers; 0 when the allocation fails.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bb7ef0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn tile_shader_property_create_object(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE_00AA13E0, &args![0xb0u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(TILE_SHADER_PROPERTY_CONSTRUCTOR, &args![block, 0u32])
        .u32()
}

// Translated from 00bb7f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tile shader property's scalar deleting destructor (slot 0 of the vtable at
/// `010b9d28`): [`fn_00bb7d70`], then frees the 0xb0-byte object (`00aa1460(this, 0xb0)`)
/// when bit 0 of the flags is set. Answers `this`.
pub fn fn_00bb7f60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bb7d70(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0xb0u32]);
    }
    this
}

/// `TileShaderProperty::ms_RTTI` (`012007d8`; its first word is the class name) and the
/// record `00bb8170` answers (`012007e0`).
const TILE_SHADER_PROPERTY_RTTI: u32 = 0x0120_07d8;
const RTTI_012007E0: u32 = 0x0120_07e0;
/// The tile shader property's member copy (`00bb7ac0`, thiscall on the source: clone,
/// cloning process).
const TILE_SHADER_PROPERTY_COPY_MEMBERS: u32 = 0x00bb_7ac0;
/// `BSShaderProperty::GetViewerStrings` (`00ba9890`, thiscall: string array), the string
/// array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`), `NiGetViewerString` (`00a7d330`),
/// `_ov9` (`00a7d550`: label, `float`) and `_ov11` (`00a7d5e0`: label, text).
const SHADER_PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00ba_9890;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
/// The checked cast `00653290` (thiscall on the object: an `NiRTTI` record; answers the
/// object or null), the record it checks the texture against (`011f444c`) and the texture's
/// virtual slot +0x9c (answers the address of its file name's text pointer).
const CAST_00653290: u32 = 0x0065_3290;
const TEXTURE_RTTI_011F444C: u32 = 0x011f_444c;
const TEXTURE_SLOT_9C: u32 = 0x9c;
/// `"Tint Color R"`, `"Tint Color G"`, `"Tint Color B"`, `"Alpha"`, `"Texture"`.
const TEXT_TINT_R: u32 = 0x010b_9e1c;
const TEXT_TINT_G: u32 = 0x010b_9e0c;
const TEXT_TINT_B: u32 = 0x010b_9dfc;
const TEXT_ALPHA: u32 = 0x010b_9df4;
const TEXT_TEXTURE: u32 = 0x010b_9dec;

// Translated from 00bb7d60 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `TileShaderProperty::GetRTTI` (slot +8 of the vtable at `010b9d28`): the address of the
/// class's `NiRTTI` record (`012007d8`).
pub fn tile_shader_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(TILE_SHADER_PROPERTY_RTTI)
}

// Translated from 00bb7e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bb7e70 - the FS:[0] exception frame (unwind state 0 around the constructor) is not translated
/// `TileShaderProperty::CreateClone` (slot +0x48 of the vtable at `010b9d28`; thiscall, `RET 4`:
/// the cloning process): 0xb0 bytes (`00aa13e0`) constructed as a tile shader property
/// (`00bb7c30` with 0; null when the allocation fails), then the members copied into it
/// (`00bb7ac0` with the cloning process; into null too). Answers the clone.
pub fn tile_shader_property_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    let block = e.call(ALLOCATE_00AA13E0, &args![0xb0u32]).u32();
    let clone = if block != 0 {
        e.call(TILE_SHADER_PROPERTY_CONSTRUCTOR, &args![block, 0u32])
            .u32()
    } else {
        0
    };
    e.call(
        TILE_SHADER_PROPERTY_COPY_MEMBERS,
        &args![this, clone, cloning],
    );
    clone
}

// Translated from 00bb7f90 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `TileShaderProperty::GetViewerStrings` (slot +0x60 of the vtable at `010b9d28`; thiscall,
/// `RET 4`): `BSShaderProperty::GetViewerStrings` (`00ba9890`), the class name, the tint
/// colour's four `float`s (+0x68 to +0x74, through `FLD`/`FSTP`) as `"Tint Color R"`, `"G"`,
/// `"B"` and `"Alpha"`, and for a tile texture (+0x60) that passes the cast to the record at
/// `011f444c`, `"Texture"` with the text its slot +0x9c points at. Each string is held in the
/// argument word and added through `SetSize`/`SetAt`.
pub fn tile_shader_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(SHADER_PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let add = |e: &mut Engine, text: u32| {
            e.mem.set_u32(slot, text);
            let index = u32::from(e.mem.u16(strings + 0xa));
            if index >= u32::from(e.mem.u16(strings + 8)) {
                let grow = u32::from(e.mem.u16(strings + 0xe));
                e.call(
                    STRING_ARRAY_SET_SIZE,
                    &args![strings, grow.wrapping_add(index)],
                );
            }
            e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
        };
        let class_name = e.global::<u32>(TILE_SHADER_PROPERTY_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        add(e, text);
        for (label, offset) in [
            (TEXT_TINT_R, 0x68u32),
            (TEXT_TINT_G, 0x6c),
            (TEXT_TINT_B, 0x70),
            (TEXT_ALPHA, 0x74),
        ] {
            let value = x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(offset)));
            let text = e.call(VIEWER_STRING_FLOAT, &args![label, value]).u32();
            add(e, text);
        }
        let texture = e.mem.u32(this.addr().wrapping_add(0x60));
        if texture == 0 {
            return;
        }
        let cast = e
            .call(CAST_00653290, &args![texture, TEXTURE_RTTI_011F444C])
            .u32();
        if cast == 0 {
            return;
        }
        let name = e.vcall(cast, TEXTURE_SLOT_9C, &args![]).u32();
        let name = e.mem.u32(name);
        let text = e.call(VIEWER_STRING_TEXT, &args![TEXT_TEXTURE, name]).u32();
        add(e, text);
    });
}

// Translated from 00bb8170 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// The `GetRTTI` (slot +8) of the vtable at `010b9e38` (no RTTI locator precedes it, so its
/// class is not confirmed): the address of the record at `012007e0`.
pub fn fn_00bb8170(_e: &mut Engine) -> Ptr {
    Ptr::new(RTTI_012007E0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_7d60, tile_shader_property_get_rtti() -> Ptr),
        entry!(0x00bb_7e70, tile_shader_property_create_clone(Ptr, u32) -> u32),
        entry!(
            0x00bb_7f90,
            tile_shader_property_get_viewer_strings(Ptr, u32)
        ),
        entry!(0x00bb_8170, fn_00bb8170() -> Ptr),
        entry!(0x00bb_8140, fn_00bb8140(Ptr, f32)),
        entry!(0x00bb_7d70, fn_00bb7d70(Ptr)),
        entry!(0x00bb_7ef0, tile_shader_property_create_object() -> u32),
        entry!(0x00bb_7f60, fn_00bb7f60(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [0x0103_1000, 0x0103_a000, 0x010b_9000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEPTH_FACTOR, 0.83f64);
        e.set_global(DISTANCE_SCALE, 163_840.0f64);
        e.set_global(END_DISTANCE, 163_840.0f32);
        e
    }

    #[test]
    fn fn_00bb8140_sets_the_two_distances() {
        let mut e = rig();
        let this = e.mem.alloc(0x40);
        e.call(0x00bb_8140, &args![this, 0.5f32]);
        let expected = ((1.0 - 0.5 * 0.83f64) * 163_840.0) as f32;
        assert_eq!(e.mem.f32(this + 0x2c), expected);
        assert_eq!(e.mem.f32(this + 0x30), 163_840.0);
    }

    #[test]
    fn fn_00bb8140_quiets_nans() {
        let mut e = rig();
        e.set_global(END_DISTANCE, 0x7f80_0001u32);
        let this = e.mem.alloc(0x40);
        e.call(0x00bb_8140, &args![this, 0x7f80_0002u32]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x7fc0_0002);
        assert_eq!(e.mem.u32(this + 0x30), 0x7fc0_0001);
    }
}

#[cfg(test)]
mod tile_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004]);
        for addr in [
            0x0f00_0004,
            MEMORY_DEALLOCATE,
            SHADER_PROPERTY_DESTRUCTOR_00BAA0E0,
            FREE_SIZED_00AA1460,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, references: u32) -> u32 {
        let o = e.mem.alloc(8);
        e.mem.set_u32(o, 0x0f00_1000);
        e.mem.set_u32(o + 4, references);
        o
    }

    #[test]
    fn destructor_releases_textures_and_the_name() {
        let mut e = rig();
        let this = e.mem.alloc(0xb0);
        let texture = object(&mut e, 1);
        let alpha_map = object(&mut e, 2);
        e.mem.set_u32(this + 0x60, texture);
        e.mem.set_u32(this + 0x64, alpha_map);
        e.mem.set_u32(this + 0x94, 0x9000);
        e.mem.set_u32(this + 0x98, 0x0010_0004);
        e.call(0x00bb_7d70, &args![this]);
        assert_eq!(e.mem.u32(this), TILE_SHADER_PROPERTY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), alpha_map);
        assert_eq!(e.mem.u32(alpha_map + 4), 1);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![texture]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x9000]]
        );
        assert_eq!((e.mem.u32(this + 0x94), e.mem.u32(this + 0x98)), (0, 0));
        assert_eq!(
            calls(&e, SHADER_PROPERTY_DESTRUCTOR_00BAA0E0),
            vec![vec![this]]
        );
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 2);
    }

    #[test]
    fn create_object_constructs_a_tile_property() {
        let mut e = rig();
        e.map(0x0f00_2000, 0x1000);
        e.register(ALLOCATE_00AA13E0, |e, _| e.mem.u32(0x0f00_2000).into_ret());
        e.register(TILE_SHADER_PROPERTY_CONSTRUCTOR, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x00bb_7ef0, &[]).u32(), 0);
        e.mem.set_u32(0x0f00_2000, 0x4000);
        assert_eq!(e.call(0x00bb_7ef0, &[]).u32(), 0x4000);
        assert_eq!(calls(&e, ALLOCATE_00AA13E0), vec![vec![0xb0], vec![0xb0]]);
        assert_eq!(
            calls(&e, TILE_SHADER_PROPERTY_CONSTRUCTOR),
            vec![vec![0x4000, 0]]
        );
    }

    #[test]
    fn deleting_destructor_frees_with_bit_0() {
        let mut e = rig();
        let this = e.mem.alloc(0xb0);
        assert_eq!(e.call(0x00bb_7f60, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, FREE_SIZED_00AA1460).is_empty());
        e.call(0x00bb_7f60, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE_SIZED_00AA1460), vec![vec![this, 0xb0]]);
        assert_eq!(calls(&e, SHADER_PROPERTY_DESTRUCTOR_00BAA0E0).len(), 2);
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn get_rtti_functions_answer_their_records() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00bb_7d60, &args![]).u32(),
            TILE_SHADER_PROPERTY_RTTI
        );
        assert_eq!(e.call(0x00bb_8170, &args![]).u32(), RTTI_012007E0);
    }

    #[test]
    fn create_clone_constructs_and_copies_even_into_null() {
        let mut e = Engine::new();
        e.register(TILE_SHADER_PROPERTY_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(TILE_SHADER_PROPERTY_COPY_MEMBERS, |_, _| Ret::default());
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00bb_7e70, &args![0x1000u32, 0x2000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(calls_to(&e, ALLOCATE_00AA13E0), vec![vec![0xb0]]);
        assert_eq!(
            calls_to(&e, TILE_SHADER_PROPERTY_CONSTRUCTOR),
            vec![vec![clone, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SHADER_PROPERTY_COPY_MEMBERS),
            vec![vec![0x1000, clone, 0x2000]]
        );
        e.register(ALLOCATE_00AA13E0, |_, _| Ret::default());
        assert_eq!(e.call(0x00bb_7e70, &args![0x1000u32, 0x2000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, TILE_SHADER_PROPERTY_COPY_MEMBERS)[1],
            vec![0x1000, 0, 0x2000]
        );
        assert_eq!(calls_to(&e, TILE_SHADER_PROPERTY_CONSTRUCTOR).len(), 1);
    }

    #[test]
    fn get_viewer_strings_lists_the_tint_and_the_texture() {
        const NAME_SLOT: u32 = 0x03e0_0000;
        let mut e = Engine::new();
        e.mem.map(TILE_SHADER_PROPERTY_RTTI, 4);
        e.mem.set_u32(TILE_SHADER_PROPERTY_RTTI, 0x0109_0000);
        e.register(SHADER_PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_FLOAT, |_, a| (a[0] ^ a[1]).into_ret());
        e.register(VIEWER_STRING_TEXT, |_, a| (a[0] + a[1]).into_ret());
        e.register(CAST_00653290, |e, a| {
            assert_eq!(a[1], TEXTURE_RTTI_011F444C);
            u32::from(e.mem.u32(a[0] + 8) != 0)
                .wrapping_mul(a[0])
                .into_ret()
        });
        e.register(NAME_SLOT, |_, a| (a[0] + 0xc).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0] + 4);
            for i in 0..u32::from(e.mem.u16(a[0] + 0xa)) {
                let value = e.mem.u32(old + 4 * i);
                e.mem.set_u32(base + 4 * i, value);
            }
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(vtable + TEXTURE_SLOT_9C, NAME_SLOT);
        let texture = e.mem.alloc(0x10);
        e.mem.set_u32(texture, vtable);
        e.mem.set_u32(texture + 8, 1);
        e.mem.set_u32(texture + 0xc, 0x6000);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0x60, texture);
        for (offset, value) in [(0x68u32, 1.0f32), (0x6c, 2.0), (0x70, 3.0)] {
            e.mem.set_f32(this + offset, value);
        }
        e.mem.set_u32(this + 0x74, 0x7f80_0001);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 4);
        e.call_log = Some(vec![]);
        e.call(0x00bb_7f90, &args![this, strings]);
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        let floats = calls_to(&e, VIEWER_STRING_FLOAT);
        assert_eq!(
            floats,
            vec![
                vec![TEXT_TINT_R, 1.0f32.to_bits()],
                vec![TEXT_TINT_G, 2.0f32.to_bits()],
                vec![TEXT_TINT_B, 3.0f32.to_bits()],
                vec![TEXT_ALPHA, 0x7fc0_0001]
            ]
        );
        let base = e.mem.u32(strings + 4);
        assert_eq!(e.mem.u16(strings + 0xa), 6);
        assert_eq!(e.mem.u32(base), 0x0109_0001);
        assert_eq!(e.mem.u32(base + 20), TEXT_TEXTURE + 0x6000);
        // A texture failing the cast adds no texture string.
        e.mem.set_u32(texture + 8, 0);
        e.call(0x00bb_7f90, &args![this, strings]);
        assert_eq!(e.mem.u16(strings + 0xa), 11);
    }
}
