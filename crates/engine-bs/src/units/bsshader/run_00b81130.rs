//! `BSShader/run_00b81130` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSShaderBloodSplatterProperty`'s methods: its vtable is `010af8e8`, whose `GetRTTI`
//! (`fn_00b811e0`) answers the record `011ff0f4` that the static initializer `00fb9ca0` names
//! `"BSShaderBloodSplatterProperty"` (`010af9bc`). On PC it is 0x70 bytes (the allocations of
//! `fn_00b814a0` and the scalar deleting destructor): the Xbox PDB's fields 8 bytes lower, as
//! the base `BSShaderProperty` is 0x60 bytes on PC.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::bsshaderproperty::BSShaderProperty;

layout! {
    /// `BSShaderBloodSplatterProperty` (Xbox PDB), its own fields at their PC offsets.
    pub struct BSShaderBloodSplatterProperty: 0x70 {
        /// `spTexture[0]` (Xbox PDB +0x68): `NiPointer<NiTexture>`.
        0x60 spTexture0: u32,
        /// `spTexture[1]` (Xbox PDB +0x6c).
        0x64 spTexture1: u32,
        /// `TextureClampMode` (Xbox PDB +0x70).
        0x68 TextureClampMode: u32,
        /// `pfFadeAlpha` (Xbox PDB +0x74): `float *`.
        0x6c pfFadeAlpha: u32,
    }
}

type Splatter = BSShaderBloodSplatterProperty;

/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The vtable (`010af8e8`) and the RTTI record (`011ff0f4`).
const BLOOD_SPLATTER_VTABLE: u32 = 0x010a_f8e8;
const BLOOD_SPLATTER_RTTI: u32 = 0x011f_f0f4;
/// `BSShaderProperty`'s constructor (`00ba8e50`, `thiscall`, no words), destructor body
/// (`00baa0e0`, `thiscall`) and member copy (`00ba8b10`, `thiscall`, two words: the clone and
/// the cloning process).
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00ba_8e50;
const SHADER_PROPERTY_DESTROY: u32 = 0x00ba_a0e0;
const SHADER_PROPERTY_COPY_MEMBERS: u32 = 0x00ba_8b10;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, five words: array, element
/// size, count, constructor, destructor) and `eh vector destructor iterator` (`00ec5fce`,
/// four words: array, element size, count, destructor), with the `NiPointer` constructor
/// `006694e0` and destructor `0045cec0` they get for the two textures.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const POINTER_CONSTRUCT_006694E0: u32 = 0x0066_94e0;
const POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// The allocation `00aa13e0` (`cdecl`, one word) and the sized free `00aa1460` (`cdecl`, two
/// words).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;
/// `BSShaderProperty::CheckCreateRenderPassArray` (`00ba95e0`, `thiscall`, one word) and
/// `BSShaderProperty::RenderPassArray::Add` (`00ba9ee0`, `thiscall`, eight words).
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// A table of up to 100 (geometry, pass) pairs at `011fedd0`, its count at `011ff0f0`, which
/// `fn_00b81330` appends to.
const PASS_LOG_011FEDD0: u32 = 0x011f_edd0;
const PASS_LOG_COUNT_011FF0F0: u32 = 0x011f_f0f0;
/// `NiSourceTexture::ms_RTTI` (`011f444c`) and `NiRenderedTexture::ms_RTTI` (`011f4ac4`).
const SOURCE_TEXTURE_RTTI: u32 = 0x011f_444c;
const RENDERED_TEXTURE_RTTI: u32 = 0x011f_4ac4;
/// `NiAVObject::GetProperty` (engine map, `00a59d30`; `thiscall`, one word: the type).
const GET_PROPERTY: u32 = 0x00a5_9d30;

/// Releases an `NiPointer`'s object: `InterlockedDecrement` on its `+4`, its slot +4
/// (`DeleteThis`) at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// Walks an `NiRTTI` chain (the parent at `+4`) from `rtti`: whether `wanted` is in it.
fn rtti_chain_has(e: &Engine, rtti: u32, wanted: u32) -> bool {
    let mut rtti = rtti;
    while rtti != 0 {
        if rtti == wanted {
            return true;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    false
}

// Translated from 00b81110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: vtable slot +0xc8 (`SetClampD3D` in the Xbox PDB's slot list;
/// `thiscall`, one word). Stores 0 in `TextureClampMode` when the word is 3, otherwise 3.
pub fn fn_00b81110(e: &mut Engine, this: Ptr<Splatter>, mode: u32) {
    let value = if mode.wrapping_sub(3) != 0 { 3 } else { 0 };
    e.set(this, Splatter::TextureClampMode, value);
}

// Translated from 00b81130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor body (`thiscall`). The vtable `010af8e8`; the two
/// textures released (when set) and cleared; the compiler's array destructor (`00ec5fce`) run
/// over them with `0045cec0`; then `BSShaderProperty`'s destructor body (`00baa0e0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b81130 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b81130(e: &mut Engine, this: Ptr<Splatter>) {
    let base = this.addr();
    e.mem.set_u32(base, BLOOD_SPLATTER_VTABLE);
    for index in 0..2u32 {
        let slot = base + 0x60 + 4 * index;
        let texture = e.mem.u32(slot);
        if texture != 0 {
            release(e, texture);
            e.mem.set_u32(slot, 0);
        }
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x60, 4u32, 2u32, POINTER_DESTRUCT_0045CEC0],
    );
    e.call(SHADER_PROPERTY_DESTROY, &args![base]);
}

// Translated from 00b811e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: `GetRTTI` (vtable slot +8): answers the RTTI record
/// `011ff0f4`.
pub fn fn_00b811e0(_e: &mut Engine, _this: Ptr<Splatter>) -> u32 {
    BLOOD_SPLATTER_RTTI
}

// Translated from 00b811f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: vtable slot +0xd0 (`SetClampNI` in the Xbox PDB's slot list;
/// `thiscall`, one word): stores the word in `TextureClampMode`.
pub fn fn_00b811f0(e: &mut Engine, this: Ptr<Splatter>, mode: u32) {
    e.set(this, Splatter::TextureClampMode, mode);
}

// Translated from 00b81200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor (`thiscall`, no words). `BSShaderProperty`'s
/// constructor (`00ba8e50`), the vtable `010af8e8`, the two textures constructed by the
/// compiler's array constructor (`00ec782f` with `006694e0` and `0045cec0`), the clamp mode and
/// `pfFadeAlpha` cleared, `iShader` 7 and `iShaderPropertyType` 0x17. Answers `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b81200 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b81200(e: &mut Engine, this: Ptr<Splatter>) -> Ptr<Splatter> {
    let base = this.addr();
    e.call(SHADER_PROPERTY_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, BLOOD_SPLATTER_VTABLE);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x60,
            4u32,
            2u32,
            POINTER_CONSTRUCT_006694E0,
            POINTER_DESTRUCT_0045CEC0
        ],
    );
    e.set(this, Splatter::TextureClampMode, 0);
    e.set(this, Splatter::pfFadeAlpha, 0);
    let property: Ptr<BSShaderProperty> = this.cast();
    e.set(property, BSShaderProperty::iShader, 7);
    e.set(property, BSShaderProperty::iShaderPropertyType, 0x17);
    this
}

// Translated from 00b81290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor (`thiscall`, one word: the
/// flags): `fn_00b81130`, then with bit 0 of the flags the 0x70 bytes freed (`00aa1460`).
/// Answers `this`.
pub fn fn_00b81290(e: &mut Engine, this: Ptr<Splatter>, flags: u8) -> Ptr<Splatter> {
    fn_00b81130(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x70u32]);
    }
    this
}

// Translated from 00b812b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the member copy into a clone (`thiscall`, two words: the clone
/// and the cloning process). `BSShaderProperty`'s copy (`00ba8b10`) runs with both; the clone's
/// two textures become this one's (an `NiPointer` assignment each) and the clamp mode is
/// copied.
pub fn fn_00b812b0(e: &mut Engine, this: Ptr<Splatter>, clone: u32, process: u32) {
    e.call(SHADER_PROPERTY_COPY_MEMBERS, &args![this, clone, process]);
    for index in 0..2u32 {
        let target = clone.wrapping_add(0x60 + 4 * index);
        let source = this.addr() + 0x60 + 4 * index;
        let old = e.mem.u32(target);
        if old == e.mem.u32(source) {
            continue;
        }
        if old != 0 {
            release(e, old);
        }
        let texture = e.mem.u32(source);
        e.mem.set_u32(target, texture);
        if texture != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![texture.wrapping_add(4)]);
        }
    }
    let mode = e.get(this, Splatter::TextureClampMode);
    e.mem.set_u32(clone.wrapping_add(0x68), mode);
}

// Translated from 00b81330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot +0xa0 (`GetRenderPasses` in the Xbox PDB's slot list;
/// `thiscall`, six words: the geometry, a render-state word, an unread word, a word whose value
/// goes above it in the key, and two unread words). The key is the fourth word shifted left by
/// 8 ORed with the second; when it differs from `iLastRenderPassState` (`+0x38`) the pass list
/// is made (`CheckCreateRenderPassArray` with 1), emptied (its `iSize`, +0x10, cleared), one
/// pass added (0x2f1 with a second texture, else 0x2f0; first-pass flag 1, no lights) and the
/// key kept. Then, while the count at `011ff0f0` is below 100 (signed), the geometry and the
/// list's first pass (the word its element array, +4, points to) are appended to the table at
/// `011fedd0`. Answers the list (`pRenderPassList`, +0x3c).
pub fn fn_00b81330(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    state_word: u32,
    _unused_2: u32,
    state_high: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let key = (state_high << 8) | state_word;
    if e.get(this, BSShaderProperty::iLastRenderPassState) as u32 != key {
        e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![this, 1u32]);
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        e.mem.set_u32(list.wrapping_add(0x10), 0);
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        let second = e.mem.u32(this.addr() + 0x64) != 0;
        let pass = u32::from(second) + 0x2f0;
        e.call(
            RENDER_PASS_ARRAY_ADD,
            &args![list, geometry, pass, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        e.set(this, BSShaderProperty::iLastRenderPassState, key as i32);
    }
    let count = e.mem.u32(PASS_LOG_COUNT_011FF0F0);
    if (count as i32) < 100 {
        let entry = PASS_LOG_011FEDD0.wrapping_add(count.wrapping_mul(8));
        e.mem.set_u32(entry, geometry);
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        let passes = e.mem.u32(list.wrapping_add(4));
        let first = e.mem.u32(passes);
        e.mem.set_u32(entry.wrapping_add(4), first);
        e.mem
            .set_u32(PASS_LOG_COUNT_011FF0F0, count.wrapping_add(1));
    }
    e.get(this, BSShaderProperty::pRenderPassList)
}

// Translated from 00b813b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot +0xbc (`thiscall`, two words: a `cdecl` callback
/// taking three words, and a word passed through to it). For each of the two textures that
/// is set, its RTTI chain (its slot +8, the parents at `+4`) is searched for
/// `NiSourceTexture` and, when not found, searched again (with a second `GetRTTI`) for
/// `NiRenderedTexture` (that result is not used); either way the callback gets the texture,
/// 0 and the word.
pub fn fn_00b813b0(e: &mut Engine, this: Ptr<Splatter>, callback: u32, data: u32) {
    for index in 0..2u32 {
        let slot = this.addr() + 0x60 + 4 * index;
        let texture = e.mem.u32(slot);
        if texture == 0 {
            continue;
        }
        let rtti = e.vcall(texture, 8, &args![]).u32();
        if !rtti_chain_has(e, rtti, SOURCE_TEXTURE_RTTI) {
            let texture = e.mem.u32(slot);
            if texture != 0 {
                let rtti = e.vcall(texture, 8, &args![]).u32();
                rtti_chain_has(e, rtti, RENDERED_TEXTURE_RTTI);
            }
        }
        let texture = e.mem.u32(slot);
        e.call(callback, &args![texture, 0u32, data]);
    }
}

// Translated from 00b81420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words: a scene-graph object and a `float *`): sets
/// the fade-alpha pointer of every blood-splatter property below the object. For a non-null
/// object whose slot +0x18 answers non-zero (`IsGeometry` in the Xbox PDB's slot list), its
/// property 3 (`NiAVObject::GetProperty`) gets the pointer in `pfFadeAlpha` when it is a
/// property of type 0x17 (`iShaderPropertyType`); otherwise, when its slot +0xc (`IsNode`)
/// answers a node, the function runs on each non-null child (the `+0xa6` 16-bit count of the
/// array at `+0xa0`).
pub fn fn_00b81420(e: &mut Engine, object: u32, fade_alpha: u32) {
    if object == 0 {
        return;
    }
    if e.vcall(object, 0x18, &args![]).u32() != 0 {
        let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
        if property != 0 && e.mem.u32(property + 0x1c) == 0x17 {
            e.mem.set_u32(property + 0x6c, fade_alpha);
        }
        return;
    }
    let node = e.vcall(object, 0xc, &args![]).u32();
    if node == 0 {
        return;
    }
    let count = u32::from(e.mem.u16(node + 0xa6));
    let mut index = 0u32;
    while index < count {
        let children = e.mem.u32(node + 0xa0);
        let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
        if child != 0 {
            fn_00b81420(e, child, fade_alpha);
        }
        index += 1;
    }
}

// Translated from 00b814a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot +0x48 (`CreateClone` in the Xbox PDB's slot list;
/// `thiscall`, one word: the cloning process). 0x70 bytes from `00aa13e0` constructed by
/// `fn_00b81200` (null when the allocation gave none) get this one's members (`fn_00b812b0`,
/// also with a null clone). Answers the clone.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b814a0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b814a0(e: &mut Engine, this: Ptr<Splatter>, process: u32) -> u32 {
    let block: Ptr<Splatter> = e.call(ALLOCATE_00AA13E0, &args![0x70u32]).ptr();
    let clone = if block.is_null() {
        0
    } else {
        fn_00b81200(e, block).addr()
    };
    fn_00b812b0(e, this, clone, process);
    clone
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b8_1110, fn_00b81110(Ptr<Splatter>, u32)),
        entry!(0x00b8_1130, fn_00b81130(Ptr<Splatter>)),
        entry!(0x00b8_11e0, fn_00b811e0(Ptr<Splatter>) -> u32),
        entry!(0x00b8_11f0, fn_00b811f0(Ptr<Splatter>, u32)),
        entry!(0x00b8_1200, fn_00b81200(Ptr<Splatter>) -> Ptr<Splatter>),
        entry!(0x00b8_1290, fn_00b81290(Ptr<Splatter>, u8) -> Ptr<Splatter>),
        entry!(0x00b8_12b0, fn_00b812b0(Ptr<Splatter>, u32, u32)),
        entry!(
            0x00b8_1330,
            fn_00b81330(Ptr<BSShaderProperty>, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00b8_13b0, fn_00b813b0(Ptr<Splatter>, u32, u32)),
        entry!(0x00b8_1420, fn_00b81420(u32, u32)),
        entry!(0x00b8_14a0, fn_00b814a0(Ptr<Splatter>, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE: u32 = 0x0f00_0004;
    const GET_RTTI: u32 = 0x0f00_0008;
    const VTABLE: u32 = 0x0f00_1000;
    const CALLBACK: u32 = 0x0f00_0100;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Reference counting on `+4`, objects with [`VTABLE`] (slot +4 deletes, slot +8 answers
    /// the word at +8), quiet doubles for the base calls, and the call log.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.put_vtable(VTABLE, &[0, DELETE, GET_RTTI]);
        e.register(DELETE, |_, _| Ret::default());
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(CALLBACK, |_, _| Ret::default());
        for quiet in [
            SHADER_PROPERTY_CONSTRUCT,
            SHADER_PROPERTY_DESTROY,
            SHADER_PROPERTY_COPY_MEMBERS,
            VECTOR_CONSTRUCT,
            VECTOR_DESTRUCT,
            FREE_SIZED_00AA1460,
            CHECK_CREATE_RENDER_PASS_ARRAY,
            RENDER_PASS_ARRAY_ADD,
        ] {
            e.register(quiet, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let o = e.mem.alloc(0x10);
        e.mem.set_u32(o, VTABLE);
        e.mem.set_u32(o + 4, count);
        o
    }

    #[test]
    fn clamp_setters_and_rtti() {
        let mut e = rig();
        let this = e.new_object::<Splatter>();
        e.call(0x00b8_1110, &args![this, 3u32]);
        assert_eq!(e.get(this, Splatter::TextureClampMode), 0);
        e.call(0x00b8_1110, &args![this, 9u32]);
        assert_eq!(e.get(this, Splatter::TextureClampMode), 3);
        e.call(0x00b8_11f0, &args![this, 5u32]);
        assert_eq!(e.get(this, Splatter::TextureClampMode), 5);
        assert_eq!(e.call(0x00b8_11e0, &args![this]).u32(), 0x011f_f0f4);
    }

    #[test]
    fn destructor_releases_the_textures() {
        let mut e = rig();
        let this = e.new_object::<Splatter>();
        let texture = object(&mut e, 1);
        e.set(this, Splatter::spTexture1, texture);
        e.call(0x00b8_1130, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), BLOOD_SPLATTER_VTABLE);
        assert_eq!(e.get(this, Splatter::spTexture1), 0);
        assert_eq!(calls(&e, DELETE), vec![vec![texture]]);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![vec![this.addr() + 0x60, 4, 2, POINTER_DESTRUCT_0045CEC0]]
        );
        assert_eq!(calls(&e, SHADER_PROPERTY_DESTROY), vec![vec![this.addr()]]);
    }

    #[test]
    fn constructor_sets_the_numbers() {
        let mut e = rig();
        let this = e.new_object::<Splatter>();
        e.mem.set_u32(this.addr() + 0x6c, 9);
        assert_eq!(e.call(0x00b8_1200, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BLOOD_SPLATTER_VTABLE);
        assert_eq!(e.get(this, Splatter::pfFadeAlpha), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x58), 7);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 0x17);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![vec![
                this.addr() + 0x60,
                4,
                2,
                POINTER_CONSTRUCT_006694E0,
                POINTER_DESTRUCT_0045CEC0
            ]]
        );
    }

    #[test]
    fn scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = rig();
        let this = e.new_object::<Splatter>();
        e.call(0x00b8_1290, &args![this, 0u32]);
        assert!(calls(&e, FREE_SIZED_00AA1460).is_empty());
        assert_eq!(e.call(0x00b8_1290, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(
            calls(&e, FREE_SIZED_00AA1460),
            vec![vec![this.addr(), 0x70]]
        );
    }

    #[test]
    fn copy_members_assigns_the_textures() {
        let mut e = rig();
        let this = e.new_object::<Splatter>();
        let clone = e.new_object::<Splatter>();
        let texture = object(&mut e, 1);
        let old = object(&mut e, 1);
        e.set(this, Splatter::spTexture0, texture);
        e.set(clone, Splatter::spTexture1, old);
        e.set(this, Splatter::TextureClampMode, 2);
        e.call(0x00b8_12b0, &args![this, clone, 0x44u32]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x44]]
        );
        assert_eq!(e.get(clone, Splatter::spTexture0), texture);
        assert_eq!(e.mem.u32(texture + 4), 2);
        assert_eq!(e.get(clone, Splatter::spTexture1), 0);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);
        assert_eq!(e.get(clone, Splatter::TextureClampMode), 2);
    }

    #[test]
    fn render_passes_are_made_for_a_new_key_and_logged() {
        let mut e = rig();
        e.map(0x011f_e000, 0x2000);
        let this = e.new_object::<Splatter>();
        let list = e.mem.alloc(0x14);
        let passes = e.mem.alloc(4);
        e.mem.set_u32(passes, 0x7777);
        e.mem.set_u32(list + 4, passes);
        e.mem.set_u32(list + 0x10, 3);
        e.mem.set_u32(this.addr() + 0x3c, list);
        e.mem.set_u32(this.addr() + 0x64, 1);
        let r = e
            .call(
                0x00b8_1330,
                &args![this, 0x900u32, 0x12u32, 0u32, 0x3u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, list);
        assert_eq!(e.mem.u32(list + 0x10), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0x312);
        assert_eq!(
            calls(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![list, 0x900, 0x2f1, 1, 0, 0, 0, 0, 0]]
        );
        assert_eq!(e.global::<u32>(PASS_LOG_COUNT_011FF0F0), 1);
        assert_eq!(e.global::<u32>(PASS_LOG_011FEDD0), 0x900);
        assert_eq!(e.global::<u32>(PASS_LOG_011FEDD0 + 4), 0x7777);
        // The same key: nothing made, still logged; at 100 nothing is logged.
        e.call(
            0x00b8_1330,
            &args![this, 0x900u32, 0x12u32, 0u32, 0x3u32, 0u32, 0u32],
        );
        assert_eq!(calls(&e, RENDER_PASS_ARRAY_ADD).len(), 1);
        assert_eq!(e.global::<u32>(PASS_LOG_COUNT_011FF0F0), 2);
        e.set_global(PASS_LOG_COUNT_011FF0F0, 100u32);
        e.mem.set_u32(this.addr() + 0x64, 0);
        e.call(
            0x00b8_1330,
            &args![this, 0x900u32, 0x13u32, 0u32, 0x3u32, 0u32, 0u32],
        );
        assert_eq!(calls(&e, RENDER_PASS_ARRAY_ADD)[1][2], 0x2f0);
        assert_eq!(e.global::<u32>(PASS_LOG_COUNT_011FF0F0), 100);
    }

    #[test]
    fn fn_00b813b0_hands_out_both_textures() {
        let mut e = rig();
        let own = e.mem.alloc(8);
        e.mem.set_u32(own + 4, SOURCE_TEXTURE_RTTI);
        let first = object(&mut e, 1);
        e.mem.set_u32(first + 8, own);
        let second = object(&mut e, 1);
        let this = e.new_object::<Splatter>();
        e.set(this, Splatter::spTexture0, first);
        e.set(this, Splatter::spTexture1, second);
        e.call(0x00b8_13b0, &args![this, CALLBACK, 0x33u32]);
        assert_eq!(
            calls(&e, CALLBACK),
            vec![vec![first, 0, 0x33], vec![second, 0, 0x33]]
        );
        // The first found at once, the second (no RTTI) asked twice.
        assert_eq!(
            calls(&e, GET_RTTI),
            vec![vec![first], vec![second], vec![second]]
        );
    }

    #[test]
    fn fn_00b81420_sets_the_fade_alpha_below_a_node() {
        const IS_GEOMETRY: u32 = 0x0f00_0018;
        const IS_NODE: u32 = 0x0f00_000c;
        let mut e = rig();
        // Objects: slot +0xc answers the word at +0x10, slot +0x18 the word at +0x14.
        e.put_vtable(0x0f00_2000, &[0, 0, 0, IS_NODE, 0, 0, IS_GEOMETRY]);
        e.register(IS_NODE, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(IS_GEOMETRY, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        // GetProperty answers the word at +0x18.
        e.register(GET_PROPERTY, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        let make = |e: &mut Engine| {
            let o = e.mem.alloc(0xb0);
            e.mem.set_u32(o, 0x0f00_2000);
            o
        };
        let splatter = e.mem.alloc(0x70);
        e.mem.set_u32(splatter + 0x1c, 0x17);
        let other = e.mem.alloc(0x70);
        e.mem.set_u32(other + 0x1c, 0x16);
        let geometry = make(&mut e);
        e.mem.set_u32(geometry + 0x14, 1);
        e.mem.set_u32(geometry + 0x18, splatter);
        let plain = make(&mut e);
        e.mem.set_u32(plain + 0x14, 1);
        e.mem.set_u32(plain + 0x18, other);
        let node = make(&mut e);
        e.mem.set_u32(node + 0x10, node);
        let children = e.mem.alloc(12);
        e.mem.set_u32(children, geometry);
        e.mem.set_u32(children + 8, plain);
        e.mem.set_u32(node + 0xa0, children);
        e.mem.set_u16(node + 0xa6, 3);
        e.call(0x00b8_1420, &args![node, 0x4321u32]);
        assert_eq!(e.mem.u32(splatter + 0x6c), 0x4321);
        assert_eq!(e.mem.u32(other + 0x6c), 0);
        assert_eq!(
            calls(&e, GET_PROPERTY),
            vec![vec![geometry, 3], vec![plain, 3]]
        );
        // A null object or a leaf that is neither does nothing.
        e.call(0x00b8_1420, &args![0u32, 1u32]);
        let leaf = make(&mut e);
        e.call(0x00b8_1420, &args![leaf, 1u32]);
        assert_eq!(calls(&e, GET_PROPERTY).len(), 2);
    }

    #[test]
    fn create_clone_constructs_and_copies() {
        let mut e = rig();
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.new_object::<Splatter>();
        e.set(this, Splatter::TextureClampMode, 1);
        let clone = e.call(0x00b8_14a0, &args![this, 0x55u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), BLOOD_SPLATTER_VTABLE);
        assert_eq!(e.mem.u32(clone + 0x68), 1);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x55]]
        );
        assert_eq!(calls(&e, ALLOCATE_00AA13E0), vec![vec![0x70]]);
    }
}
