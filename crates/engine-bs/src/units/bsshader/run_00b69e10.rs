//! `BSShader/run_00b69e10` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's allocation context (`+0x2b4` of the `_tls_index`
/// block), 0x1d while the render passes are built.
const TLS_CONTEXT: u32 = 0x2b4;
/// The byte that enables the water passes (`011ad86e`).
const WATER_PASSES_ENABLED: u32 = 0x011a_d86e;
/// The two global flag bytes the pass choice reads (`011ff104`, `011ff101`).
const GLOBAL_FLAG_011FF104: u32 = 0x011f_f104;
const GLOBAL_FLAG_011FF101: u32 = 0x011f_f101;
/// `BSShaderProperty::CheckCreateRenderPassArray` (engine map), thiscall (property, -1).
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
/// `BSShaderProperty::RenderPassArray::Add` (engine map), thiscall
/// (array, geometry, pass, 1, 0, 0, 0, 0, 0).
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// thiscall (property, geometry): the second pass's geometry (word +0) and pass (16 bits at +4).
const SECOND_PASS_B69F30: u32 = 0x00b6_9f30;
/// `WaterShaderProperty`'s vtable (the one its constructor sets).
const WATER_SHADER_PROPERTY_VTABLE: u32 = 0x010a_e438;
/// The list destructor run on `+0x128` (`NiTPointerListBase<..>::RemoveAll` in the engine map; folded code).
const LIST_REMOVE_ALL: u32 = 0x004e_d900;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The `BSShaderProperty` destructor (`00baa0e0`, thiscall).
const SHADER_PROPERTY_DESTROY: u32 = 0x00ba_a0e0;

/// The pass number the code picks (see [`fn_00b69ff0`]): `w` is the byte at
/// `+0x7e`, `f` whether the geometry data has a word at `+0x28`, `d` and `l`
/// the bytes at `+0x60` and `+0x61`, `g` the global byte at `011ff101`, `a`
/// and `b` the bytes at `+0x80` and `+0x81` and `c` the byte at `+0x63`.
#[allow(clippy::too_many_arguments)]
fn pass_for(w: u8, f: bool, d: u8, l: u8, g: u8, a: u8, b: u8, c: u8) -> u32 {
    let z = u32::from(c == 0);
    let (a, b) = (a != 0, b != 0);
    if w != 0 {
        return 0x2bd;
    }
    if d != 0 {
        // Four passes apart per `c`, chosen by the global flag, `a` and `b`.
        let start = match (f, g != 0, a, b) {
            (true, true, false, false) => 0x2a6,
            (true, true, false, true) => 0x2a4,
            (true, true, true, false) => 0x2a5,
            (true, true, true, true) => 0x2a3,
            (true, false, false, false) => 0x29e,
            (true, false, false, true) => 0x29c,
            (true, false, true, false) => 0x29d,
            (true, false, true, true) => 0x29b,
            (false, true, false, false) => 0x293,
            (false, true, false, true) => 0x295,
            (false, true, true, false) => 0x294,
            (false, true, true, true) => 0x292,
            (false, false, false, false) => 0x28b,
            (false, false, false, true) => 0x28d,
            (false, false, true, false) => 0x28c,
            (false, false, true, true) => 0x28a,
        };
        return z * 4 + start;
    }
    if l != 0 {
        return u32::from(!a) + 0x23f;
    }
    // Eight passes apart per `c`; without the global flag and `a`, the code
    // computes the pair as `x - 10` or `x - 6` when `c` is set, else `x`.
    let minus_ten = if c != 0 { 0xffff_fff6u32 } else { 0 };
    let minus_six = if c != 0 { 0xffff_fffau32 } else { 0 };
    match (f, g != 0, a, b) {
        (true, true, false, false) => z * 8 + 0x2c3,
        (true, true, false, true) => z * 8 + 0x2c5,
        (true, true, true, false) => z * 8 + 0x2c4,
        (true, true, true, true) => z * 8 + 0x2c2,
        (true, false, false, false) => minus_ten.wrapping_add(0x2c9),
        (true, false, false, true) => minus_six.wrapping_add(0x2c7),
        (true, false, true, false) => z * 8 + 0x2c0,
        (true, false, true, true) => z * 8 + 0x2be,
        (false, true, false, false) => z * 8 + 0x2b1,
        (false, true, false, true) => z * 8 + 0x2b3,
        (false, true, true, false) => z * 8 + 0x2b2,
        (false, true, true, true) => z * 8 + 0x2b0,
        (false, false, false, false) => minus_ten.wrapping_add(0x2b7),
        (false, false, false, true) => minus_six.wrapping_add(0x2b5),
        (false, false, true, false) => z * 8 + 0x2ae,
        (false, false, true, true) => z * 8 + 0x2ac,
    }
}

fn add_pass(e: &mut Engine, list: u32, geometry: u32, pass: u32) {
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
}

// Translated from 00b69ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b69ff0 - the C++ exception-unwinding frame (FS:[0]) that restores the allocation context when a call throws is not translated
/// The water shader property's render passes (unnamed in the engine map;
/// the fields are those its destructor `00b6a520` releases), thiscall
/// `(geometry, five more words never read)`: with the allocation context
/// at 0x1d (restored at the end), answers 0 unless the byte at `011ad86e`
/// is set. When the global byte at `011ff104` differs from the one kept at
/// `+0x7e`, clears the pass list's count (`+0x10` of the list at `+0x3c`)
/// and keeps the new byte. Makes sure the pass list exists; when it is
/// empty, adds the pass [`pass_for`] picks, the second pass from
/// `00b69f30` when the bytes at `+0x60` and `+0x61` are 0, and, when the
/// byte at `+0x83` and the word at `+0x130` are set, a third pass (0x2ab,
/// 0x29a, 0x2ce or 0x2bc by `+0x60` and the geometry flag). Answers the
/// pass list.
#[allow(clippy::too_many_arguments)]
pub fn fn_00b69ff0(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let base = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, 0x1d);
    if e.global::<u8>(WATER_PASSES_ENABLED) == 0 {
        e.mem.set_u32(tls + TLS_CONTEXT, saved);
        return 0;
    }
    let data = e.mem.u32(geometry.wrapping_add(0xb8));
    let flag = e.mem.u32(data.wrapping_add(0x28)) != 0;
    if e.global::<u8>(GLOBAL_FLAG_011FF104) != e.mem.u8(base + 0x7e) {
        let list = e.mem.u32(base + 0x3c);
        if list != 0 {
            e.mem.set_u32(list + 0x10, 0);
        }
        let value: u8 = e.global(GLOBAL_FLAG_011FF104);
        e.mem.set_u8(base + 0x7e, value);
    }
    e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![base, 0xffff_ffffu32]);
    let list = e.mem.u32(base + 0x3c);
    if e.mem.u32(list.wrapping_add(0x10)) == 0 {
        let pass = pass_for(
            e.mem.u8(base + 0x7e),
            flag,
            e.mem.u8(base + 0x60),
            e.mem.u8(base + 0x61),
            e.global(GLOBAL_FLAG_011FF101),
            e.mem.u8(base + 0x80),
            e.mem.u8(base + 0x81),
            e.mem.u8(base + 0x63),
        );
        add_pass(e, list, geometry, pass);
        if e.mem.u8(base + 0x60) == 0 && e.mem.u8(base + 0x61) == 0 {
            let second = e.call(SECOND_PASS_B69F30, &args![base, geometry]).u32();
            let pass = u32::from(e.mem.u16(second + 4));
            let second_geometry = e.mem.u32(second);
            let list = e.mem.u32(base + 0x3c);
            add_pass(e, list, second_geometry, pass);
        }
        if e.mem.u8(base + 0x83) != 0 && e.mem.u32(base + 0x130) != 0 {
            let list = e.mem.u32(base + 0x3c);
            let pass = match (e.mem.u8(base + 0x60) != 0, flag) {
                (true, true) => 0x2ab,
                (true, false) => 0x29a,
                (false, true) => 0x2ce,
                (false, false) => 0x2bc,
            };
            add_pass(e, list, geometry, pass);
        }
    }
    let list = e.mem.u32(base + 0x3c);
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
    list
}

/// Releases an `NiPointer`'s object: decrements its reference count (+4)
/// and deletes it (vtable +4) when the count reaches 0.
fn release(e: &mut Engine, object: u32) {
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00b6a520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b6a520 - the C++ exception-unwinding frame (FS:[0]) that destroys the remaining members when a release throws is not translated
/// `WaterShaderProperty::~WaterShaderProperty` (by the vtable it sets;
/// unnamed in the engine map): sets the vtable, empties the list at
/// `+0x128`, releases the six `NiPointer`s from `+0x148` down to `+0x134`,
/// empties the list again (its destructor) and destroys the
/// `BSShaderProperty` base.
pub fn fn_00b6a520(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, WATER_SHADER_PROPERTY_VTABLE);
    e.call(LIST_REMOVE_ALL, &args![base + 0x128]);
    for offset in [0x148u32, 0x144, 0x140, 0x13c, 0x138, 0x134] {
        let object = e.mem.u32(base + offset);
        release(e, object);
    }
    e.call(LIST_REMOVE_ALL, &args![base + 0x128]);
    e.call(SHADER_PROPERTY_DESTROY, &args![base]);
}

use crate::types::NiTArray;

/// `InterlockedIncrement` (import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `NiSourceTexture::ms_RTTI` (`011f444c`) and `NiRenderedTexture::ms_RTTI` (`011f4ac4`; their
/// static initializers `00fb5790` and `00fb5d30` name them), which `fn_00b69e10` looks for in
/// the RTTI chain of its first texture.
const SOURCE_TEXTURE_RTTI: u32 = 0x011f_444c;
const RENDERED_TEXTURE_RTTI: u32 = 0x011f_4ac4;
/// `BSRenderedTexture::GetTexture` (engine map, `004bc320`; `thiscall`, one word).
const RENDERED_TEXTURE_GET_TEXTURE: u32 = 0x004b_c320;
/// The memory manager (`011f6238`) and its allocation (`00aa3e40`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `BSShaderProperty::RenderPass::RenderPass` (engine map, `00ba8ec0`; `cdecl`, six words:
/// the pass, the geometry, the pass number and three more).
const RENDER_PASS_CONSTRUCT: u32 = 0x00ba_8ec0;
/// `WaterShaderProperty::ms_RTTI` (`011fa018`; the static initializer `00fb9980` names it
/// `"WaterShaderProperty"`).
const WATER_SHADER_PROPERTY_RTTI: u32 = 0x011f_a018;
/// `BSShaderProperty::GetViewerStrings` (engine map, `00ba9890`; `thiscall`, one word: the
/// string array), `NiGetViewerString` (`00a7d330`; `cdecl`, one word: the name) and the
/// string array's grow (`0096ad30`, `thiscall`, one word: the new size) and set
/// (`0096ae90`, `thiscall`, two words: the index and the address of the element).
const SHADER_PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00ba_9890;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const ARRAY_SET_SIZE_0096AD30: u32 = 0x0096_ad30;
const ARRAY_SET_AT_0096AE90: u32 = 0x0096_ae90;
/// The base member copy (`00ba8b10`, unit `bsshaderproperty.cpp`; `thiscall`, two words: the
/// clone and the cloning process).
const SHADER_PROPERTY_COPY_MEMBERS: u32 = 0x00ba_8b10;
/// The list-item pool (lock `011c5f80` and its count `011c5f84`, entered through `0040fbf0`
/// with 0; free list `011c5f58`, refilled by `0043a080`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;
const NODE_POOL_REFILL: u32 = 0x0043_a080;
/// The cloning process `fn_00b6ab20` keeps on its stack (0x10 bytes): its constructor
/// (`004ad0c0`, unit `bipedanim.cpp`; `thiscall`, one word).
const CLONING_PROCESS_CONSTRUCT: u32 = 0x004a_d0c0;

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

// Translated from 00b69e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: hands the water property's textures to a callback (`thiscall`,
/// two words: a `cdecl` callback taking three words, and a word passed through to it). When
/// the noise height map (`+0x134`, `spNoiseHeightMap` in the Xbox PDB) is set, its RTTI chain
/// (its slot +8, the parents at `+4`) is searched for `NiSourceTexture` and, when not found,
/// searched again (with a second `GetRTTI`) for `NiRenderedTexture` (that result is not used);
/// either way the callback gets the texture, 0 and the word. Then for each of the rendered
/// textures at `+0x138`, `+0x13c`, `+0x140` and `+0x144` that is set and whose
/// `BSRenderedTexture::GetTexture(0)` is not null, the callback gets that texture (asked
/// again), 0 and the word.
pub fn fn_00b69e10(e: &mut Engine, this: Ptr, callback: u32, data: u32) {
    let base = this.addr();
    let noise = e.mem.u32(base + 0x134);
    if noise != 0 {
        let rtti = e.vcall(noise, 8, &args![]).u32();
        if !rtti_chain_has(e, rtti, SOURCE_TEXTURE_RTTI) {
            let noise = e.mem.u32(base + 0x134);
            if noise != 0 {
                let rtti = e.vcall(noise, 8, &args![]).u32();
                rtti_chain_has(e, rtti, RENDERED_TEXTURE_RTTI);
            }
        }
        let noise = e.mem.u32(base + 0x134);
        e.call(callback, &args![noise, 0u32, data]);
    }
    for offset in [0x138u32, 0x13c, 0x140, 0x144] {
        let rendered = e.mem.u32(base + offset);
        if rendered == 0 {
            continue;
        }
        if e.call(RENDERED_TEXTURE_GET_TEXTURE, &args![rendered, 0u32])
            .u32()
            == 0
        {
            continue;
        }
        let rendered = e.mem.u32(base + offset);
        let texture = e
            .call(RENDERED_TEXTURE_GET_TEXTURE, &args![rendered, 0u32])
            .u32();
        e.call(callback, &args![texture, 0u32, data]);
    }
}

// Translated from 00b69f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the water property's second render pass (`thiscall`, one word:
/// the geometry), made on first use. When the pass at `+0x14c` (`pWaterPass` in the Xbox PDB)
/// is null, 0x10 bytes from the memory manager (`00aa3e40` on `011f6238`) become
/// `BSShaderProperty::RenderPass` (`00ba8ec0` with the block, the geometry, pass 0x289 when the
/// geometry's data (`+0xb8`) has a word at `+0x28`, else 0x288, then 1, 0, 0), or null when the
/// allocation gave none, and are kept there. Answers the pass at `+0x14c`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b69f30 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b69f30(e: &mut Engine, this: Ptr, geometry: u32) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + 0x14c) == 0 {
        let data = e.mem.u32(geometry.wrapping_add(0xb8));
        let pass_number: u32 = if e.mem.u32(data.wrapping_add(0x28)) != 0 {
            0x289
        } else {
            0x288
        };
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
            .u32();
        let pass = if block != 0 {
            e.call(
                RENDER_PASS_CONSTRUCT,
                &args![block, geometry, pass_number, 1u32, 0u32, 0u32],
            )
            .u32()
        } else {
            0
        };
        e.mem.set_u32(base + 0x14c, pass);
    }
    e.mem.u32(base + 0x14c)
}

// Translated from 00b6a660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: `WaterShaderProperty::GetRTTI` (vtable slot +8): answers the
/// address of its RTTI record, `011fa018`.
pub fn fn_00b6a660(_e: &mut Engine, _this: Ptr) -> u32 {
    WATER_SHADER_PROPERTY_RTTI
}

// Translated from 00b6a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the water property's `GetViewerStrings` (`thiscall`, one word:
/// the string array, an `NiTArray` of `char *`). `BSShaderProperty::GetViewerStrings` runs with
/// it; then the viewer string of the class name (`NiGetViewerString` with the name word of the
/// RTTI record `011fa018`) is added at the array's end: when `m_usSize` is not below
/// `m_usMaxSize` the array grows to `m_usSize + m_usGrowBy` (`0096ad30`), then the string is set
/// at index `m_usSize` (`0096ae90` with the address of a stack word holding it).
pub fn fn_00b6a670(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(SHADER_PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(WATER_SHADER_PROPERTY_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    let array: Ptr<NiTArray> = Ptr::new(strings);
    let size = u32::from(e.get(array, NiTArray::m_usSize));
    let capacity = u32::from(e.get(array, NiTArray::m_usMaxSize));
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), text);
        if size >= capacity {
            let grow = u32::from(e.get(array, NiTArray::m_usGrowBy));
            e.call(
                ARRAY_SET_SIZE_0096AD30,
                &args![strings, grow.wrapping_add(size)],
            );
        }
        e.call(ARRAY_SET_AT_0096AE90, &args![strings, size, element]);
    });
}

/// An `NiPointer` assignment from `source`'s slot to `target`'s, as the copy inlines it.
fn copy_pointer(e: &mut Engine, source: u32, target: u32) {
    let old = e.mem.u32(target);
    let new = e.mem.u32(source);
    if old == new {
        return;
    }
    release(e, old);
    let new = e.mem.u32(source);
    e.mem.set_u32(target, new);
    if new != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![new + 4]);
    }
}

// Translated from 00b6a6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the water property's member copy into a clone (`thiscall`, two
/// words: the clone and the cloning process). The base copy (`00ba8b10`) runs; then, in the
/// exe's order: the `NiPointer` at `+0x148` assigned; the bytes `+0x61` to `+0x63`, the words
/// `+0x64`, `+0x68`, the floats `+0x6c`, `+0x70` (`FLD`/`FSTP`), the bytes `+0x7d`, `+0x7c`,
/// `+0x7f`, `+0x80`, `+0x81`, the words `+0x88` to `+0x114`, the floats `+0x118`, `+0x124`,
/// `+0x11c`, `+0x120`, `+0x74`, `+0x78` and the byte `+0x82` copied; the `NiPointer`s at
/// `+0x13c`, `+0x140`, `+0x138` and `+0x144` assigned; the word `+0x84` and the byte `+0x83`
/// copied. Last the clone's light list (`+0x128`) is emptied (`004ed900`) and each non-null
/// element of this one's list is added at its tail, each item taken from the item pool inline
/// (the lock entered through `0040fbf0` on `011c5f80` with 0, the free list at `011c5f58`
/// refilled by `0043a080` when empty, the item cleared, the lock's count at `011c5f84` down by
/// one and its owner cleared at zero).
pub fn fn_00b6a6c0(e: &mut Engine, this: Ptr, clone: u32, process: u32) {
    let source = this.addr();
    e.call(SHADER_PROPERTY_COPY_MEMBERS, &args![source, clone, process]);
    copy_pointer(e, source + 0x148, clone.wrapping_add(0x148));
    let copy_byte = |e: &mut Engine, offset: u32| {
        let value = e.mem.u8(source + offset);
        e.mem.set_u8(clone.wrapping_add(offset), value);
    };
    let copy_word = |e: &mut Engine, offset: u32| {
        let value = e.mem.u32(source + offset);
        e.mem.set_u32(clone.wrapping_add(offset), value);
    };
    let copy_float = |e: &mut Engine, offset: u32| {
        let value = x87_quiet_bits(e.mem.u32(source + offset));
        e.mem.set_u32(clone.wrapping_add(offset), value);
    };
    for offset in [0x61u32, 0x62, 0x63] {
        copy_byte(e, offset);
    }
    copy_word(e, 0x64);
    copy_word(e, 0x68);
    copy_float(e, 0x6c);
    copy_float(e, 0x70);
    for offset in [0x7du32, 0x7c, 0x7f, 0x80, 0x81] {
        copy_byte(e, offset);
    }
    for offset in (0x88u32..=0x114).step_by(4) {
        copy_word(e, offset);
    }
    for offset in [0x118u32, 0x124, 0x11c, 0x120, 0x74, 0x78] {
        copy_float(e, offset);
    }
    copy_byte(e, 0x82);
    for offset in [0x13cu32, 0x140, 0x138, 0x144] {
        copy_pointer(e, source + offset, clone.wrapping_add(offset));
    }
    copy_word(e, 0x84);
    copy_byte(e, 0x83);

    let list = clone.wrapping_add(0x128);
    e.call(LIST_REMOVE_ALL, &args![list]);
    let mut item = e.mem.u32(source + 0x128);
    while item != 0 {
        let element = e.mem.u32(item + 8);
        item = e.mem.u32(item);
        if element == 0 {
            continue;
        }
        e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
        let mut node = e.mem.u32(NODE_POOL_FREE_LIST);
        if node == 0 {
            e.call(NODE_POOL_REFILL, &args![]);
            node = e.mem.u32(NODE_POOL_FREE_LIST);
        }
        let next = e.mem.u32(node);
        e.mem.set_u32(NODE_POOL_FREE_LIST, next);
        e.mem.set_u32(node + 8, 0);
        e.mem.set_u32(node, 0);
        e.mem.set_u32(node + 4, 0);
        let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
        if count == 0 {
            e.mem.set_u32(NODE_POOL_LOCK, 0);
        }
        e.mem.set_u32(node + 8, element);
        e.mem.set_u32(node, 0);
        let tail = e.mem.u32(list.wrapping_add(4));
        e.mem.set_u32(node + 4, tail);
        let tail = e.mem.u32(list.wrapping_add(4));
        if tail != 0 {
            e.mem.set_u32(tail, node);
        } else {
            e.mem.set_u32(list, node);
        }
        let items = e.mem.u32(list.wrapping_add(8)).wrapping_add(1);
        e.mem.set_u32(list.wrapping_add(8), items);
        e.mem.set_u32(list.wrapping_add(4), node);
    }
}

// Translated from 00b6ab20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the water property's members into a clone with a
/// temporary cloning process (`thiscall`, one word: the clone). A 0x10-byte cloning process on
/// the stack is constructed (`004ad0c0` with 0x101), `fn_00b6a6c0` runs with the clone and it,
/// and the process's two objects (its words `+0` and `+4`) are deleted when set (slot 0 with
/// 1).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b6ab20 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b6ab20(e: &mut Engine, this: Ptr, clone: u32) {
    e.with_stack(0x10, |e, process| {
        let process = process.addr();
        e.call(CLONING_PROCESS_CONSTRUCT, &args![process, 0x101u32]);
        fn_00b6a6c0(e, this, clone, process);
        for offset in [0u32, 4] {
            let object = e.mem.u32(process + offset);
            if object != 0 {
                e.vcall(object, 0, &args![1u32]);
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_9e10, fn_00b69e10(Ptr, u32, u32)),
        entry!(0x00b6_9f30, fn_00b69f30(Ptr, u32) -> u32),
        entry!(0x00b6_a660, fn_00b6a660(Ptr) -> u32),
        entry!(0x00b6_a670, fn_00b6a670(Ptr, u32)),
        entry!(0x00b6_a6c0, fn_00b6a6c0(Ptr, u32, u32)),
        entry!(0x00b6_ab20, fn_00b6ab20(Ptr, u32)),
        entry!(
            0x00b69ff0,
            fn_00b69ff0(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00b6a520, fn_00b6a520(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE_DOUBLE: u32 = 0xdead_8000;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn pass_choice_follows_the_flags() {
        assert_eq!(pass_for(1, true, 1, 1, 1, 1, 1, 1), 0x2bd);
        assert_eq!(pass_for(0, true, 1, 0, 1, 0, 0, 0), 0x2aa);
        assert_eq!(pass_for(0, true, 1, 0, 0, 1, 1, 1), 0x29b);
        assert_eq!(pass_for(0, false, 1, 0, 1, 1, 0, 0), 0x298);
        assert_eq!(pass_for(0, false, 1, 0, 0, 0, 1, 1), 0x28d);
        assert_eq!(pass_for(0, true, 0, 1, 0, 0, 0, 0), 0x240);
        assert_eq!(pass_for(0, false, 0, 1, 0, 1, 0, 0), 0x23f);
        assert_eq!(pass_for(0, true, 0, 0, 1, 0, 1, 0), 0x2cd);
        assert_eq!(pass_for(0, true, 0, 0, 0, 0, 0, 1), 0x2bf);
        assert_eq!(pass_for(0, true, 0, 0, 0, 0, 0, 0), 0x2c9);
        assert_eq!(pass_for(0, true, 0, 0, 0, 0, 1, 1), 0x2c1);
        assert_eq!(pass_for(0, true, 0, 0, 0, 1, 1, 0), 0x2c6);
        assert_eq!(pass_for(0, false, 0, 0, 1, 1, 1, 1), 0x2b0);
        assert_eq!(pass_for(0, false, 0, 0, 0, 0, 0, 1), 0x2ad);
        assert_eq!(pass_for(0, false, 0, 0, 0, 0, 1, 0), 0x2b5);
        assert_eq!(pass_for(0, false, 0, 0, 0, 1, 0, 0), 0x2b6);
    }

    #[test]
    fn builds_the_passes_with_the_allocation_context() {
        let mut e = Engine::new();
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 7);
        e.register(CHECK_CREATE_RENDER_PASS_ARRAY, |_, _| Ret::default());
        e.register_double(RENDER_PASS_ARRAY_ADD, move |e, _| {
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 0x1d);
            Ret::default()
        });
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x4444);
        e.mem.set_u16(second + 4, 0x123);
        e.register_double(SECOND_PASS_B69F30, move |_, _| Ret {
            eax: second,
            ..Ret::default()
        });
        let list = e.mem.alloc(0x14);
        let data = e.mem.alloc(0x2c);
        e.mem.set_u32(data + 0x28, 1);
        let geometry = e.mem.alloc(0xbc);
        e.mem.set_u32(geometry + 0xb8, data);
        let this = e.mem.alloc(0x158);
        e.mem.set_u32(this + 0x3c, list);
        e.mem.set_u8(this + 0x7e, 1);
        e.mem.set_u8(this + 0x83, 1);
        e.mem.set_u32(this + 0x130, 1);
        let words = args![this, geometry, 0u32, 0u32, 0u32, 0u32, 0u32];
        // Disabled: answers 0 and restores the context.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_9ff0, &words).u32(), 0);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 7);
        assert!(calls(&e, CHECK_CREATE_RENDER_PASS_ARRAY).is_empty());
        e.set_global(WATER_PASSES_ENABLED, 1u8);
        e.mem.set_u32(list + 0x10, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_9ff0, &words).u32(), list);
        // The global byte differs: the list is cleared and the byte kept.
        assert_eq!(e.mem.u8(this + 0x7e), 0);
        assert_eq!(
            calls(&e, CHECK_CREATE_RENDER_PASS_ARRAY),
            vec![vec![this, 0xffff_ffff]]
        );
        assert_eq!(
            calls(&e, RENDER_PASS_ARRAY_ADD),
            vec![
                vec![list, geometry, 0x2c9, 1, 0, 0, 0, 0, 0],
                vec![list, 0x4444, 0x123, 1, 0, 0, 0, 0, 0],
                vec![list, geometry, 0x2ce, 1, 0, 0, 0, 0, 0],
            ]
        );
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 7);
        // A list with passes is left alone.
        e.mem.set_u32(list + 0x10, 2);
        e.call_log = Some(vec![]);
        e.call(0x00b6_9ff0, &words);
        assert!(calls(&e, RENDER_PASS_ARRAY_ADD).is_empty());
    }

    #[test]
    fn destructor_releases_the_pointers() {
        let mut e = Engine::new();
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(SHADER_PROPERTY_DESTROY, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_DOUBLE);
        let last = e.mem.alloc(8);
        e.mem.set_u32(last, vtable);
        e.mem.set_u32(last + 4, 1);
        let shared = e.mem.alloc(8);
        e.mem.set_u32(shared, vtable);
        e.mem.set_u32(shared + 4, 2);
        let this = e.mem.alloc(0x158);
        e.mem.set_u32(this + 0x148, last);
        e.mem.set_u32(this + 0x134, shared);
        e.call_log = Some(vec![]);
        e.call(0x00b6_a520, &args![this]);
        assert_eq!(e.mem.u32(this), WATER_SHADER_PROPERTY_VTABLE);
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![last]]);
        assert_eq!(e.mem.u32(shared + 4), 1);
        assert_eq!(
            calls(&e, LIST_REMOVE_ALL),
            vec![vec![this + 0x128], vec![this + 0x128]]
        );
        assert_eq!(calls(&e, SHADER_PROPERTY_DESTROY), vec![vec![this]]);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    const CALLBACK: u32 = 0x0f00_0100;
    const GET_RTTI: u32 = 0x0f00_0108;
    const DELETE: u32 = 0x0f00_0104;
    const VTABLE: u32 = 0x0f00_1000;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Reference counting on `+4`, objects with the vtable [`VTABLE`] (slot 0 and +4 delete,
    /// slot +8 answers the RTTI in the object's word +8), a callback and the call log.
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
        e.put_vtable(VTABLE, &[DELETE, DELETE, GET_RTTI]);
        e.register(DELETE, |_, _| Ret::default());
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(CALLBACK, |_, _| Ret::default());
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
    fn fn_00b69e10_hands_out_the_textures() {
        let mut e = rig();
        // An RTTI chain: own record, then NiSourceTexture.
        let own = e.mem.alloc(8);
        e.mem.set_u32(own + 4, SOURCE_TEXTURE_RTTI);
        let noise = object(&mut e, 1);
        e.mem.set_u32(noise + 8, own);
        // A rendered texture whose GetTexture answers its word +8.
        e.register(RENDERED_TEXTURE_GET_TEXTURE, |e, a| {
            e.mem.u32(a[0] + 8).into_ret()
        });
        let with_texture = e.mem.alloc(0x10);
        e.mem.set_u32(with_texture + 8, 0x7000);
        let without_texture = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x158);
        e.mem.set_u32(this + 0x134, noise);
        e.mem.set_u32(this + 0x13c, with_texture);
        e.mem.set_u32(this + 0x144, without_texture);
        e.call(0x00b6_9e10, &args![this, CALLBACK, 0x55u32]);
        assert_eq!(
            calls(&e, CALLBACK),
            vec![vec![noise, 0, 0x55], vec![0x7000, 0, 0x55]]
        );
        // Found at once: one GetRTTI; the texture asked twice, the empty one once.
        assert_eq!(calls(&e, GET_RTTI).len(), 1);
        assert_eq!(
            calls(&e, RENDERED_TEXTURE_GET_TEXTURE),
            vec![
                vec![with_texture, 0],
                vec![with_texture, 0],
                vec![without_texture, 0]
            ]
        );
        // Not a source texture: the chain is searched a second time.
        e.mem.set_u32(own + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b6_9e10, &args![this, CALLBACK, 0x55u32]);
        assert_eq!(calls(&e, GET_RTTI).len(), 2);
        assert_eq!(calls(&e, CALLBACK)[0], vec![noise, 0, 0x55]);
        // Nothing set: no callback.
        let empty = e.mem.alloc(0x158);
        e.call_log = Some(vec![]);
        e.call(0x00b6_9e10, &args![empty, CALLBACK, 0x55u32]);
        assert!(calls(&e, CALLBACK).is_empty());
    }

    #[test]
    fn fn_00b69f30_makes_the_pass_once() {
        let mut e = rig();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(RENDER_PASS_CONSTRUCT, |_, a| a[0].into_ret());
        let data = e.mem.alloc(0x30);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xb8, data);
        let this = e.mem.alloc(0x158);
        let pass = e.call(0x00b6_9f30, &args![this, geometry]).u32();
        assert_ne!(pass, 0);
        assert_eq!(e.mem.u32(this + 0x14c), pass);
        assert_eq!(
            calls(&e, RENDER_PASS_CONSTRUCT),
            vec![vec![pass, geometry, 0x288, 1, 0, 0]]
        );
        assert_eq!(e.call(0x00b6_9f30, &args![this, geometry]).u32(), pass);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
        // With the data word set the pass is 0x289.
        e.mem.set_u32(data + 0x28, 1);
        let other = e.mem.alloc(0x158);
        e.call(0x00b6_9f30, &args![other, geometry]);
        assert_eq!(calls(&e, RENDER_PASS_CONSTRUCT)[1][2], 0x289);
        // No block: the pass stays null.
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        let third = e.mem.alloc(0x158);
        assert_eq!(e.call(0x00b6_9f30, &args![third, geometry]).u32(), 0);
        assert_eq!(calls(&e, RENDER_PASS_CONSTRUCT).len(), 2);
    }

    #[test]
    fn fn_00b6a660_answers_the_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b6_a660, &args![0u32]).u32(), 0x011f_a018);
    }

    #[test]
    fn fn_00b6a670_adds_the_class_name() {
        let mut e = rig();
        e.map(0x011f_a000, 0x1000);
        e.set_global(WATER_SHADER_PROPERTY_RTTI, 0x0123_4567u32);
        e.register(SHADER_PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(ARRAY_SET_SIZE_0096AD30, |_, _| Ret::default());
        e.register(ARRAY_SET_AT_0096AE90, |e, a| {
            // Record the element the address points to.
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(0x011f_aff0, value);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 3);
        e.call(0x00b6_a670, &args![0x900u32, strings]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![0x900, strings]]
        );
        assert!(calls(&e, ARRAY_SET_SIZE_0096AD30).is_empty());
        assert_eq!(calls(&e, ARRAY_SET_AT_0096AE90)[0][..2], [strings, 2]);
        assert_eq!(e.global::<u32>(0x011f_aff0), 0x0123_4568);
        // Full: it grows by the grow-by count first.
        e.mem.set_u16(strings + 0xa, 4);
        e.call(0x00b6_a670, &args![0x900u32, strings]);
        assert_eq!(calls(&e, ARRAY_SET_SIZE_0096AD30), vec![vec![strings, 7]]);
        assert_eq!(calls(&e, ARRAY_SET_AT_0096AE90)[1][..2], [strings, 4]);
    }

    #[test]
    fn fn_00b6a6c0_copies_members_and_lights() {
        let mut e = rig();
        e.map(0x011c_5000, 0x1000);
        e.register(SHADER_PROPERTY_COPY_MEMBERS, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, 0);
            }
            Ret::default()
        });
        e.register(LOCK_ENTER, |e, _| {
            let n = e.mem.u32(NODE_POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, n);
            e.mem.set_u32(NODE_POOL_LOCK, 9);
            Ret::default()
        });
        e.register(NODE_POOL_REFILL, |e, _| {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(NODE_POOL_FREE_LIST, node);
            Ret::default()
        });
        let this = e.mem.alloc(0x158);
        let clone = e.mem.alloc(0x158);
        for offset in (0x64..0x128).step_by(4) {
            e.mem.set_u32(this + offset, 0x1000 + offset);
        }
        e.mem.set_u32(this + 0x60, 0x0101_0101);
        // A signalling NaN copied through FLD/FSTP comes out quiet.
        e.mem.set_u32(this + 0x6c, 0x7f80_0001);
        let texture = object(&mut e, 1);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x13c, texture);
        e.mem.set_u32(clone + 0x13c, old);
        // Lights: 0x11, null, 0x22.
        let mut next = 0;
        for element in [0x22u32, 0, 0x11] {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item, next);
            e.mem.set_u32(item + 8, element);
            next = item;
        }
        e.mem.set_u32(this + 0x128, next);
        e.call(0x00b6_a6c0, &args![this, clone, 0x77u32]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, 0x77]]
        );
        assert_eq!(e.mem.u32(clone + 0x64), 0x1064);
        assert_eq!(e.mem.u32(clone + 0x100), 0x1100);
        assert_eq!(e.mem.u32(clone + 0x84), 0x1084);
        assert_eq!(e.mem.u32(clone + 0x60), 0x0101_0100);
        assert_eq!(e.mem.u32(clone + 0x6c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(clone + 0x13c), texture);
        assert_eq!(e.mem.u32(texture + 4), 2);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);
        // Two items at the clone's list, in order; the pool lock is left.
        let head = e.mem.u32(clone + 0x128);
        let tail = e.mem.u32(clone + 0x12c);
        assert_eq!(e.mem.u32(clone + 0x130), 2);
        assert_eq!(e.mem.u32(head + 8), 0x11);
        assert_eq!(e.mem.u32(head), tail);
        assert_eq!(e.mem.u32(tail + 8), 0x22);
        assert_eq!(e.mem.u32(tail + 4), head);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        assert_eq!(calls(&e, NODE_POOL_REFILL).len(), 2);
    }

    #[test]
    fn fn_00b6ab20_copies_through_a_temporary_process() {
        let mut e = rig();
        let first = object(&mut e, 1);
        e.register_double(CLONING_PROCESS_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], first);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(SHADER_PROPERTY_COPY_MEMBERS, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        let this = e.mem.alloc(0x158);
        let clone = e.mem.alloc(0x158);
        e.call(0x00b6_ab20, &args![this, clone]);
        let construct = calls(&e, CLONING_PROCESS_CONSTRUCT);
        assert_eq!(construct[0][1], 0x101);
        let process = construct[0][0];
        assert_eq!(
            calls(&e, SHADER_PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, process]]
        );
        assert_eq!(calls(&e, DELETE), vec![vec![first, 1]]);
    }
}
