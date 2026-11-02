//! `NiMain/nimaterialproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiMaterialProperty::ms_RTTI` (the class's `NiRTTI` record).
const NI_MATERIAL_PROPERTY_RTTI: u32 = 0x011f_4a5c;

// Translated from 00a756c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_material_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_MATERIAL_PROPERTY_RTTI)
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE};

/// `011f4a64`: the pointer `00a759e0` stores the object it builds in.
const MATERIAL_PROPERTY_SLOT: u32 = 0x011f_4a64;
/// `00a75650` (`thiscall`, engine map `NiMaterialProperty::NiMaterialProperty`): constructs
/// the object in the block and answers it.
const MATERIAL_PROPERTY_CONSTRUCTOR: u32 = 0x00a7_5650;

// Translated from 00a759e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimaterialproperty.cpp`; run by the start-up
/// `00a89cc0`, no arguments): allocates 0x4c bytes (`00aa13e0`), constructs a material
/// property in them with `00a75650` and stores the object it answers, counted, in
/// `011f4a64`, releasing the object held there before. A failed allocation stores null.
pub fn fn_00a759e0(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(MATERIAL_PROPERTY_CONSTRUCTOR, &args![block]).u32()
    };
    assign_ref_counted(e, MATERIAL_PROPERTY_SLOT, object);
}

/// `011aa070`: a counted word `00a75650` increments; the count it answers goes to `+0x18`.
const MATERIAL_COUNTER: u32 = 0x011a_a070;
/// `0101e570`: a `float` copied to `+0x38`.
const MATERIAL_FLOAT: u32 = 0x0101_e570;
/// `InterlockedIncrement` import slot.
const INTERLOCKED_INCREMENT_SLOT: u32 = 0x00fd_f1c0;

// Translated from 00a75650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::NiMaterialProperty` (Xbox PDB; `thiscall`, no stack word): runs the
/// base constructor `00a5c200`, sets the vtable `0109d6c4`, zeroes the floats at `+0x1c`
/// to `+0x30` and the word at `+0x34`, copies the float at `0101e570` to `+0x38`, sets
/// the floats at `+0x3c` and `+0x40` to 1.0, counts `011aa070` and stores the count at
/// `+0x18`, sets the word at `+0x44` to 1 and `+0x48` to 0. Returns `this`.
pub fn ni_material_property_ni_material_property(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        crate::units::nimain::nimainsdm::OBJECT_BASE_CONSTRUCTOR,
        &args![this],
    );
    let base = this.addr();
    e.mem.set_u32(base, 0x0109_d6c4);
    // `FLDZ` and `FST`/`FSTP` of 0.0 (stored repeatedly by the exe) into the six floats.
    for offset in [0x1cu32, 0x20, 0x24, 0x28, 0x2c, 0x30] {
        e.mem.set_u32(base + offset, 0.0f32.to_bits());
    }
    e.mem.set_u32(base + 0x34, 0);
    let value = e.mem.u32(MATERIAL_FLOAT);
    e.mem.set_u32(base + 0x38, x87_quiet_bits(value));
    e.mem.set_u32(base + 0x3c, 1.0f32.to_bits());
    e.mem.set_u32(base + 0x40, 1.0f32.to_bits());
    let count = e
        .call(INTERLOCKED_INCREMENT_SLOT, &args![MATERIAL_COUNTER])
        .u32();
    e.mem.set_u32(base + 0x18, count);
    e.mem.set_u32(base + 0x44, 1);
    e.mem.set_u32(base + 0x48, 0);
    this
}

/// `NiMaterialProperty`'s vtable.
const NI_MATERIAL_PROPERTY_VTABLE: u32 = 0x0109_d6c4;
/// `00a5d7a0` (`thiscall`, (clone, cloning process)): the property base's member copy.
const PROPERTY_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `00a5d450` (`thiscall`, (clone, cloning process)): records the clone in the cloning
/// process's clone map.
const OBJECT_RECORD_CLONE: u32 = 0x00a5_d450;
/// `NiProperty::LoadBinary` (`thiscall`, the stream).
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
/// `NiColor::LoadBinary` (`thiscall` on the color, the stream).
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `00a5c240` (unit `niobjectnet.cpp`, `thiscall`): the object base's destructor body.
const OBJECT_NET_DESTRUCT: u32 = 0x00a5_c240;
/// The sized delete (`cdecl`, (block, size)) and the import slot of `InterlockedDecrement`.
const SIZED_DELETE: u32 = 0x00aa_1460;
const INTERLOCKED_DECREMENT_SLOT: u32 = 0x00fd_f1c4;
/// `011f4748`: the object whose virtual slot `+0xf4` the destructor tells about the material
/// (when there is one).
const MATERIAL_LISTENER: u32 = 0x011f_4748;

// Translated from 00a755f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimaterialproperty.cpp`; by its body the member copy):
/// the property base's copy (`00a5d7a0`), the words from +0x1c to +0x34 copied as they are
/// and the floats at +0x38, +0x3c and +0x40 through the x87 stack.
pub fn fn_00a755f0(e: &mut Engine, this: Ptr, clone: Ptr, cloning_process: Ptr) {
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    for offset in (0x1cu32..=0x34).step_by(4) {
        let value = e.mem.u32(this.addr() + offset);
        e.mem.set_u32(clone.addr() + offset, value);
    }
    for offset in [0x38u32, 0x3c, 0x40] {
        let value = x87_store32(x87_load32(e.mem.u32(this.addr() + offset)));
        e.mem.set_u32(clone.addr() + offset, value);
    }
}

// Translated from 00a756d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiMaterialProperty`): allocates 0x4c bytes (`operator new`) and, when that
/// succeeds, runs the constructor (`00a75650`) on them. Answers the object, or null.
pub fn fn_00a756d0(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    ni_material_property_ni_material_property(e, Ptr::new(block))
}

// Translated from 00a756f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable
/// `0109d6c4`, passes `this` to the virtual slot `+0xf4` of the object at `011f4748` when
/// there is one, runs the object base's destructor body (`00a5c240`) and, when bit 0 of
/// `flags` is set, the sized delete of 0x4c bytes. Answers `this`.
pub fn ni_material_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_MATERIAL_PROPERTY_VTABLE);
    let listener = e.mem.u32(MATERIAL_LISTENER);
    if listener != 0 {
        e.vcall(listener, 0xf4, &args![this]);
    }
    e.call(OBJECT_NET_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x4cu32]);
    }
    this
}

// Translated from 00a75730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::CreateClone` (Xbox PDB): a material without controllers
/// (`NiObjectNET` +0xc null) is its own clone (recorded as such by `00a5d450`); otherwise a
/// new material (0x4c bytes, the constructor; null when the allocation fails) given this
/// one's members (`00a755f0`, which runs on null too). Answers the clone.
pub fn ni_material_property_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    if e.mem.u32(this.addr() + 0xc) == 0 {
        e.call(OBJECT_RECORD_CLONE, &args![this, this, cloning_process]);
        return this;
    }
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    let clone = if block != 0 {
        ni_material_property_ni_material_property(e, Ptr::new(block))
    } else {
        Ptr::new(0)
    };
    fn_00a755f0(e, this, clone, cloning_process);
    clone
}

/// Reads one 4-byte word from the stream's input (`NiStream` +0x24c, the read function +8,
/// the position +4) into `destination`, the byte-count word at `count` set to 4 first.
fn read_word(e: &mut Engine, stream: Ptr, destination: u32, count: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
    let function = e.mem.u32(input + 8);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![input, destination, 4u32, count, 1u32])
        .u32();
    let position = e.mem.u32(input + 4).wrapping_add(done);
    e.mem.set_u32(input + 4, position);
}

// Translated from 00a75790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`; streams whose
/// Bethesda version (`NiStream` +4) is below 0x1a then hold two colors that are read into a
/// zeroed local and dropped, two colors at +0x1c and +0x28 (`NiColor::LoadBinary`) and two
/// 4-byte floats at +0x38 and +0x3c; newer ones the two colors and three floats, +0x38,
/// +0x3c and +0x40. The stream argument's stack word is the byte-count word.
pub fn ni_material_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    let old = e.mem.u32(stream.addr() + 4) < 0x1a;
    e.with_stack(0x10, |e, slots| {
        let count = slots.addr();
        let dropped = slots.addr() + 4;
        if old {
            for word in 0..3 {
                e.mem.set_u32(dropped + 4 * word, x87_store32(0.0));
            }
            e.call(COLOR_LOAD_BINARY, &args![dropped, stream]);
            e.call(COLOR_LOAD_BINARY, &args![dropped, stream]);
        }
        e.call(COLOR_LOAD_BINARY, &args![this.addr() + 0x1c, stream]);
        e.call(COLOR_LOAD_BINARY, &args![this.addr() + 0x28, stream]);
        read_word(e, stream, this.addr() + 0x38, count);
        read_word(e, stream, this.addr() + 0x3c, count);
        if !old {
            read_word(e, stream, this.addr() + 0x40, count);
        }
    });
}

// Translated from 00a75a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimaterialproperty.cpp`; the shut-down counterpart of
/// `00a759e0`): releases the material held in `011f4a64` (its slot `+4` when the count
/// reaches 0) and clears the word; nothing when it is null.
pub fn fn_00a75a40(e: &mut Engine) {
    let held = e.mem.u32(MATERIAL_PROPERTY_SLOT);
    if held == 0 {
        return;
    }
    if e.call(INTERLOCKED_DECREMENT_SLOT, &args![held.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(held, 4, &args![]);
    }
    e.mem.set_u32(MATERIAL_PROPERTY_SLOT, 0);
}

/// `00a5d7b0` and `00a62140` (`thiscall`, the stream / the other object): the base's
/// `SaveBinary` and `IsEqual` (the engine map names the shared bodies after
/// `NiSequenceStreamHelper`, identical-code folding).
const BASE_SAVE_BINARY: u32 = 0x00a5_d7b0;
const BASE_IS_EQUAL: u32 = 0x00a6_2140;
/// `NiColor::SaveBinary` (`00a69780`, `thiscall` on the color, the stream) and
/// `NiColor::GetViewerString` (`00a696c0`, `thiscall` on the color, the label).
const COLOR_SAVE_BINARY: u32 = 0x00a6_9780;
const COLOR_VIEWER_STRING: u32 = 0x00a6_96c0;
/// `00439090` (`thiscall` on a color, the other): true when the two colors differ.
const COLOR_DIFFERS: u32 = 0x0043_9090;
/// `NiProperty::GetViewerStrings` (`00a5d850`, `thiscall`, the strings).
const PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a5_d850;
/// Viewer string builders (`cdecl`): `NiGetViewerString` (`00a7d330`, the class name) and
/// `NiGetViewerString_ov9` (`00a7d550`, a label and a float).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// `NiTArray<char *>` resize (`0096ad30`) and `SetAt` (`0096ae90`).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The labels `"m_spec"` (`0109d794`), `"m_external_emit"` (`0109d784`), `"m_emit"`
/// (`0109d77c`), `"m_fShine"` (`0109d770`), `"m_fAlpha"` (`0109d764`) and
/// `"m_fEmitMult"` (`0109d758`).
const TEXT_SPECULAR: u32 = 0x0109_d794;
const TEXT_EXTERNAL_EMIT: u32 = 0x0109_d784;
const TEXT_EMIT: u32 = 0x0109_d77c;
const TEXT_SHINE: u32 = 0x0109_d770;
const TEXT_ALPHA: u32 = 0x0109_d764;
const TEXT_EMIT_MULTIPLIER: u32 = 0x0109_d758;

/// One 4-byte write to the stream's output (`NiStream` +0x250, its write function +0xc and
/// position +4), the byte-count word at `count` set to 4 first.
fn write_word(e: &mut Engine, stream: Ptr, source: u32, count: u32) {
    let output = e.mem.u32(stream.addr() + 0x250);
    let function = e.mem.u32(output + 0xc);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![output, source, 4u32, count, 1u32])
        .u32();
    let position = e.mem.u32(output + 4).wrapping_add(done);
    e.mem.set_u32(output + 4, position);
}

// Translated from 00a758c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::SaveBinary` (Xbox PDB): the base's `SaveBinary` (`00a5d7b0`), the
/// colors at +0x1c and +0x28 (`NiColor::SaveBinary`) and the three 4-byte floats at +0x38,
/// +0x3c and +0x40. The stream argument's stack word is the byte-count word.
pub fn ni_material_property_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.call(COLOR_SAVE_BINARY, &args![this.addr() + 0x1c, stream]);
    e.call(COLOR_SAVE_BINARY, &args![this.addr() + 0x28, stream]);
    e.with_stack(4, |e, count| {
        for offset in [0x38u32, 0x3c, 0x40] {
            write_word(e, stream, this.addr() + offset, count.addr());
        }
    });
}

// Translated from 00a75960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::IsEqual` (Xbox PDB): the base's `IsEqual` (`00a62140`), the colors
/// at +0x1c and +0x28 not differing (`00439090`), the same word at +0x34 and the same floats
/// at +0x38, +0x3c and +0x40 (x87 compares: a NaN is never equal).
pub fn ni_material_property_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (a, b) = (this.addr(), other.addr());
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    for offset in [0x1cu32, 0x28] {
        if e.call(COLOR_DIFFERS, &args![a + offset, b + offset]).bool() {
            return false;
        }
    }
    if e.mem.u32(a + 0x34) != e.mem.u32(b + 0x34) {
        return false;
    }
    [0x38u32, 0x3c, 0x40]
        .iter()
        .all(|offset| x87_load32(e.mem.u32(b + offset)) == x87_load32(e.mem.u32(a + offset)))
}

/// Adds the string in the word at `slot` to the viewer strings: the array grown by its
/// grow-by count when full (`0096ad30`), then `SetAt` (`0096ae90`) at the old size.
fn add_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let array = strings.addr();
    let index = u32::from(e.mem.u16(array + 0xa));
    if index >= u32::from(e.mem.u16(array + 8)) {
        let grow = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a75a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialProperty::GetViewerStrings` (Xbox PDB): after `NiProperty::GetViewerStrings`
/// (`00a5d850`), adds the class name, `"m_spec"` (the color at +0x1c), the emittance and
/// `"m_fShine"`, `"m_fAlpha"`, `"m_fEmitMult"` (the floats at +0x38, +0x3c, +0x40). The
/// emittance is `"m_emit"` with the color at +0x28 when the word at +0x34 is 0; otherwise it
/// is `"m_external_emit"` with that word itself as the color (the code passes the word, not
/// an address in the object).
pub fn ni_material_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let base = this.addr();
    e.call(PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let add = |e: &mut Engine, text: u32| {
            e.mem.set_u32(slot.addr(), text);
            add_viewer_string(e, strings, slot.addr());
        };
        let name = e.mem.u32(NI_MATERIAL_PROPERTY_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
        add(e, text);
        let text = e
            .call(COLOR_VIEWER_STRING, &args![base + 0x1c, TEXT_SPECULAR])
            .u32();
        add(e, text);
        let external = e.mem.u32(base + 0x34);
        let (color, label) = if external != 0 {
            (external, TEXT_EXTERNAL_EMIT)
        } else {
            (base + 0x28, TEXT_EMIT)
        };
        let text = e.call(COLOR_VIEWER_STRING, &args![color, label]).u32();
        add(e, text);
        for (offset, label) in [
            (0x38u32, TEXT_SHINE),
            (0x3c, TEXT_ALPHA),
            (0x40, TEXT_EMIT_MULTIPLIER),
        ] {
            let value = x87_quiet_bits(e.mem.u32(base + offset));
            let text = e.call(VIEWER_STRING_FLOAT, &args![label, value]).u32();
            add(e, text);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_58c0, ni_material_property_save_binary(Ptr, Ptr)),
        entry!(0x00a7_5960, ni_material_property_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a7_5a70,
            ni_material_property_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a7_55f0, fn_00a755f0(Ptr, Ptr, Ptr)),
        entry!(0x00a7_56d0, fn_00a756d0() -> Ptr),
        entry!(
            0x00a7_56f0,
            ni_material_property_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a7_5730, ni_material_property_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a7_5790, ni_material_property_load_binary(Ptr, Ptr)),
        entry!(0x00a7_5a40, fn_00a75a40()),
        entry!(0x00a756c0, ni_material_property_get_rtti() -> Ptr),
        entry!(0x00a7_59e0, fn_00a759e0()),
        entry!(0x00a7_5650, ni_material_property_ni_material_property(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_material_property_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a756c0, &args![]).u32(),
            NI_MATERIAL_PROPERTY_RTTI
        );
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn stores_the_object_the_constructor_answers() {
        let run = run_default_instance(0x00a7_59e0, MATERIAL_PROPERTY_SLOT, |e| {
            e.register(MATERIAL_PROPERTY_CONSTRUCTOR, |e, a| {
                e.mem.set_u32(a[0] + 0x40, 0xabcd);
                a[0].into_ret()
            });
        });
        assert_ne!(run.object, 0);
        assert_eq!(run.engine.mem.u32(run.object + 0x40), 0xabcd);
        assert_eq!(run.engine.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a7_59e0, MATERIAL_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn constructs_the_default_material() {
        let mut e = engine_with_counting();
        e.map(0x011a_a000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.mem.set_u32(MATERIAL_COUNTER, 41);
        e.mem.set_u32(MATERIAL_FLOAT, 0x7fa0_0001);
        let this = e.mem.alloc(0x4c);
        e.mem.write(this, &[0xcc; 0x4c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_5650, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x0109_d6c4);
        for offset in [0x1cu32, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        // The signalling NaN is quieted by the FLD/FSTP copy.
        assert_eq!(e.mem.u32(this + 0x38), 0x7fe0_0001);
        assert_eq!(e.mem.u32(this + 0x3c), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 0x40), 0x3f80_0000);
        assert_eq!(e.mem.u32(MATERIAL_COUNTER), 42);
        assert_eq!(e.mem.u32(this + 0x18), 42);
        assert_eq!(e.mem.u32(this + 0x44), 1);
        assert_eq!(e.mem.u32(this + 0x48), 0);
    }
}

#[cfg(test)]
mod material_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x011a_a000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        for double in [
            PROPERTY_COPY_MEMBERS,
            OBJECT_RECORD_CLONE,
            PROPERTY_LOAD_BINARY,
            OBJECT_NET_DESTRUCT,
            SIZED_DELETE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn copy_members_copies_the_colors_and_floats() {
        let mut e = engine();
        let this = e.mem.alloc(0x4c);
        let clone = e.mem.alloc(0x4c);
        for offset in (0x1cu32..0x44).step_by(4) {
            e.mem.set_u32(this + offset, 0x100 + offset);
        }
        e.mem.set_u32(this + 0x38, 0x7fa0_0001);
        e.call(0x00a7_55f0, &args![this, clone, 0x55u32]);
        assert_eq!(
            calls_to(&e, PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, 0x55]]
        );
        for offset in (0x1cu32..=0x34).step_by(4) {
            assert_eq!(e.mem.u32(clone + offset), 0x100 + offset);
        }
        assert_eq!(e.mem.u32(clone + 0x38), 0x7fe0_0001);
        assert_eq!(e.mem.u32(clone + 0x40), 0x140);
    }

    #[test]
    fn create_object_constructs_a_material() {
        let mut e = engine();
        let object = e.call(0x00a7_56d0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_MATERIAL_PROPERTY_VTABLE);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x4c]]);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a7_56d0, &args![]).u32(), 0);
    }

    #[test]
    fn scalar_deleting_destructor_tells_the_listener() {
        let mut e = engine();
        let this = e.mem.alloc(0x4c);
        assert_eq!(e.call(0x00a7_56f0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_MATERIAL_PROPERTY_VTABLE);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x4c]]);
        let listener = e.mem.alloc(8);
        let mut slots = vec![0u32; 0x3e];
        slots[0xf4 / 4] = 0x0300_00f4;
        e.put_vtable(0x0300_0000, &slots);
        e.register(0x0300_00f4, |_, _| Ret::default());
        e.mem.set_u32(listener, 0x0300_0000);
        e.mem.set_u32(MATERIAL_LISTENER, listener);
        e.call(0x00a7_56f0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, 0x0300_00f4), vec![vec![listener, this]]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
        assert_eq!(calls_to(&e, OBJECT_NET_DESTRUCT).len(), 2);
    }

    #[test]
    fn create_clone_shares_a_material_without_controllers() {
        let mut e = engine();
        let this = e.mem.alloc(0x4c);
        assert_eq!(e.call(0x00a7_5730, &args![this, 0x66u32]).u32(), this);
        assert_eq!(
            calls_to(&e, OBJECT_RECORD_CLONE),
            vec![vec![this, this, 0x66]]
        );
        e.mem.set_u32(this + 0xc, 0x1234);
        e.mem.set_u32(this + 0x20, 0x77);
        let clone = e.call(0x00a7_5730, &args![this, 0x66u32]).u32();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone), NI_MATERIAL_PROPERTY_VTABLE);
        assert_eq!(e.mem.u32(clone + 0x20), 0x77);
    }

    #[test]
    fn load_binary_reads_old_and_new_layouts() {
        let mut e = engine();
        e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0300_0000);
        let mut next = 1u32;
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], next);
            next += 1;
            4u32.into_ret()
        });
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(stream + 4, 0x19);
        e.call(0x00a7_5790, &args![this, stream]);
        let colors: Vec<u32> = calls_to(&e, COLOR_LOAD_BINARY)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(colors.len(), 4);
        assert_eq!(colors[0], colors[1]);
        assert_eq!(colors[2..], [this + 0x1c, this + 0x28]);
        assert_eq!(
            (
                e.mem.u32(this + 0x38),
                e.mem.u32(this + 0x3c),
                e.mem.u32(this + 0x40)
            ),
            (1, 2, 0)
        );
        e.mem.set_u32(stream + 4, 0x1a);
        e.call_log = Some(vec![]);
        e.call(0x00a7_5790, &args![this, stream]);
        assert_eq!(calls_to(&e, COLOR_LOAD_BINARY).len(), 2);
        assert_eq!(
            (
                e.mem.u32(this + 0x38),
                e.mem.u32(this + 0x3c),
                e.mem.u32(this + 0x40)
            ),
            (3, 4, 5)
        );
        assert_eq!(e.mem.u32(input + 4), 20);
    }

    #[test]
    fn release_of_the_default_material() {
        let mut e = engine();
        e.put_vtable(0x0300_1000, &[0, 0x0300_1004]);
        e.register(0x0300_1004, |_, _| Ret::default());
        let held = e.mem.alloc(0x4c);
        e.mem.set_u32(held, 0x0300_1000);
        e.mem.set_u32(held + 4, 2);
        e.mem.set_u32(MATERIAL_PROPERTY_SLOT, held);
        e.call(0x00a7_5a40, &args![]);
        assert_eq!(e.mem.u32(MATERIAL_PROPERTY_SLOT), 0);
        assert_eq!(e.mem.u32(held + 4), 1);
        assert!(calls_to(&e, 0x0300_1004).is_empty());
        e.mem.set_u32(MATERIAL_PROPERTY_SLOT, held);
        e.call(0x00a7_5a40, &args![]);
        assert_eq!(calls_to(&e, 0x0300_1004), vec![vec![held]]);
        e.call(0x00a7_5a40, &args![]);
        assert_eq!(calls_to(&e, 0x0300_1004).len(), 1);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn save_binary_writes_the_base_two_colors_and_three_floats() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x4c);
        for (offset, value) in [(0x38u32, 1.5f32), (0x3c, 0.25), (0x40, 2.0)] {
            e.mem.set_f32(this + offset, value);
        }
        let stream = e.mem.alloc(0x260);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x0340_0000);
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let w = written.clone();
        e.register_double(0x0340_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            w.borrow_mut().push(e.mem.f32(a[1]));
            4u32.into_ret()
        });
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(COLOR_SAVE_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a7_58c0, &args![this, stream]);
        assert_eq!(calls(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        assert_eq!(
            calls(&e, COLOR_SAVE_BINARY),
            vec![vec![this + 0x1c, stream], vec![this + 0x28, stream]]
        );
        assert_eq!(*written.borrow(), vec![1.5, 0.25, 2.0]);
        assert_eq!(e.mem.u32(output + 4), 12);
    }

    #[test]
    fn is_equal_checks_the_base_colors_word_and_floats() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x4c);
        let b = e.mem.alloc(0x4c);
        e.register(BASE_IS_EQUAL, |e, a| {
            u32::from(e.mem.u32(a[0] + 0x48) == 0).into_ret()
        });
        e.register(COLOR_DIFFERS, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let check = |e: &mut Engine| e.call(0x00a7_5960, &args![a, b]).bool();
        assert!(check(&mut e));
        e.mem.set_u32(a + 0x48, 1);
        assert!(!check(&mut e));
        e.mem.set_u32(a + 0x48, 0);
        e.mem.set_u32(b + 0x28, 7);
        assert!(!check(&mut e));
        e.mem.set_u32(b + 0x28, 0);
        e.mem.set_u32(b + 0x34, 9);
        assert!(!check(&mut e));
        e.mem.set_u32(b + 0x34, 0);
        // -0 equals 0; a NaN equals nothing.
        e.mem.set_f32(a + 0x40, -0.0);
        assert!(check(&mut e));
        e.mem.set_f32(a + 0x3c, f32::NAN);
        e.mem.set_f32(b + 0x3c, f32::NAN);
        assert!(!check(&mut e));
    }

    #[test]
    fn viewer_strings_pick_the_emittance_by_the_external_word() {
        let mut e = Engine::new();
        e.map(NI_MATERIAL_PROPERTY_RTTI & !0xfff, 0x1000);
        e.mem.set_u32(NI_MATERIAL_PROPERTY_RTTI, 0x0abc_0000);
        let this = e.mem.alloc(0x4c);
        e.mem.set_f32(this + 0x38, 10.0);
        e.mem.set_f32(this + 0x3c, 0.5);
        e.mem.set_u32(this + 0x40, 0x7f80_0001);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 3);
        for addr in [
            PROPERTY_GET_VIEWER_STRINGS,
            VIEWER_STRING_NAME,
            COLOR_VIEWER_STRING,
            VIEWER_STRING_FLOAT,
        ] {
            e.register(addr, |_, a| (0x0500_0000 + a[0] % 0x1000).into_ret());
        }
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a7_5a70, &args![this, strings]);
        assert_eq!(
            calls(&e, PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls(&e, VIEWER_STRING_NAME), vec![vec![0x0abc_0000]]);
        assert_eq!(
            calls(&e, COLOR_VIEWER_STRING),
            vec![
                vec![this + 0x1c, TEXT_SPECULAR],
                vec![this + 0x28, TEXT_EMIT]
            ]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_FLOAT),
            vec![
                vec![TEXT_SHINE, 10.0f32.to_bits()],
                vec![TEXT_ALPHA, 0.5f32.to_bits()],
                // The float goes through the x87 stack: a signalling NaN is quieted.
                vec![TEXT_EMIT_MULTIPLIER, 0x7fc0_0001],
            ]
        );
        let indices: Vec<u32> = calls(&e, STRING_ARRAY_SET_AT)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(indices, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(
            calls(&e, STRING_ARRAY_SET_SIZE),
            vec![vec![strings, 4], vec![strings, 7]]
        );
        // An external emittance word is passed as the color itself.
        e.mem.set_u32(this + 0x34, 0x0600_0000);
        e.call_log = Some(vec![]);
        e.call(0x00a7_5a70, &args![this, strings]);
        assert_eq!(
            calls(&e, COLOR_VIEWER_STRING)[1],
            vec![0x0600_0000, TEXT_EXTERNAL_EMIT]
        );
    }
}
