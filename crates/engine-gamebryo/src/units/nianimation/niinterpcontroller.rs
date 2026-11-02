//! `NiAnimation/niinterpcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiInterpController::ms_RTTI` (the class's `NiRTTI` record).
const NI_INTERP_CONTROLLER_RTTI: u32 = 0x011f_36b4;

// Translated from 00a2f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_interp_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_INTERP_CONTROLLER_RTTI)
}

/// `NiTimeController::LoadBinary` and `NiTimeController::LinkObject` (`thiscall`, the stream).
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;

// Translated from 00a2f910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpController::LoadBinary` (Xbox PDB): `NiTimeController::LoadBinary`; files of
/// versions `0x0a010068` up to (not including) `0x0a01006d` (`NiStream` +0xd8) follow with one
/// byte (read into the stream argument's stack word, the byte-count word a local set to 1):
/// not 0 sets flag bit 5 (`0x20`) of the controller's flags (+8), 0 clears it.
pub fn ni_interp_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    let version = e.get(stream, NiStream::m_uiNifFileVersion);
    if !(0x0a01_0068..0x0a01_006d).contains(&version) {
        return;
    }
    let set = e.with_stack(8, |e, slots| {
        let argument = slots.addr();
        let count = slots.addr() + 4;
        let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        e.mem.set_u32(count, 1);
        let done = e
            .call(function, &args![input, argument, 1u32, count, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
        e.mem.u8(argument) != 0
    });
    // NiTimeController::m_uFlags (Xbox PDB) +0x08
    let flags = e.mem.u16(this.addr() + 8);
    let flags = if set { flags | 0x20 } else { flags & 0xffdf };
    e.mem.set_u16(this.addr() + 8, flags);
}

// Translated from 00c9a6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpController::LinkObject` (Xbox PDB): a jump to `NiTimeController::LinkObject`
/// (`00a6d500`), passing on `this` and the stream.
pub fn ni_interp_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
}

/// `NiTimeController::m_fLoKeyTime` / `m_fHiKeyTime` (Xbox PDB) +0x14 / +0x18.
const LO_KEY_TIME: u32 = 0x14;
const HI_KEY_TIME: u32 = 0x18;

// Translated from 00a2f8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpController::ResetTimeExtrema` (Xbox PDB): unless flag bit 5 of the controller's
/// flags byte (+8) is set (the controller is managed), both key times become 0.0.
pub fn ni_interp_controller_reset_time_extrema(e: &mut Engine, this: Ptr) {
    if (e.mem.u8(this.addr() + 8) >> 5) & 1 != 0 {
        return;
    }
    e.mem.set_f32(this.addr() + LO_KEY_TIME, 0.0);
    e.mem.set_f32(this.addr() + HI_KEY_TIME, 0.0);
}

/// `NiTimeController::GetViewerStrings` (`00a6d830`, thiscall, the array).
const TIME_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a6_d830;
/// `NiGetViewerString` (`00a7d330`, cdecl, the text) and its bool form (`00a7d370`, cdecl,
/// label and a word whose low byte is the value).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
/// The viewer-string array's resize (`0096ad30`, one word) and `SetAt` (`0096ae90`: index,
/// address of the value), both thiscall on the array.
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The label `"IsManagerControlled"`.
const TEXT_IS_MANAGER_CONTROLLED: u32 = 0x0109_6414;

/// Appends the string in the stack word `slot` to the viewer-string array `strings` (16-bit
/// size +0xA, capacity +8, grow-by +0xE): grows it to size + grow-by when full, then `SetAt`
/// at the old size with the slot's address.
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + size]);
    }
    e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, slot]);
}

// Translated from 00a2f980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiInterpController::GetViewerStrings` (Xbox PDB): `NiTimeController::GetViewerStrings`,
/// then the class name (the first word of the `NiRTTI` record at `011f36b4` through
/// `NiGetViewerString`) and `"IsManagerControlled"` with flag bit 5 of the flags byte (+8).
/// Each string is held in the argument's own stack word; the bool is stored into that word's
/// low byte only, so the word passed to `00a7d370` keeps the upper bytes of the class-name
/// string's address.
pub fn ni_interp_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(TIME_CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, strings);
        let class_name = e.mem.u32(NI_INTERP_CONTROLLER_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let managed = (e.mem.u8(this.addr() + 8) >> 5) & 1;
        e.mem.set_u8(slot, managed);
        let word = e.mem.u32(slot);
        let text = e
            .call(
                NI_GET_VIEWER_STRING_BOOL,
                &args![TEXT_IS_MANAGER_CONTROLLED, word],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_f8f0, ni_interp_controller_reset_time_extrema(Ptr)),
        entry!(
            0x00a2_f980,
            ni_interp_controller_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a2f8e0, ni_interp_controller_get_rtti() -> Ptr),
        entry!(
            0x00a2_f910,
            ni_interp_controller_load_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00c9_a6b0,
            ni_interp_controller_link_object(Ptr, Ptr<NiStream>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_interp_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a2f8e0, &args![]).u32(),
            NI_INTERP_CONTROLLER_RTTI
        );
    }

    #[test]
    fn load_binary_reads_the_flag_byte_of_middle_versions() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, 0x0300_0000);
        let mut answers = vec![0u8, 1u8];
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (1, 1, 1));
            e.mem.set_u8(a[1], answers.pop().unwrap());
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 8, 0x0101);
        e.set(stream, NiStream::m_uiNifFileVersion, 0x0a01_0068);
        e.call(0x00a2_f910, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 8), 0x0121);
        e.set(stream, NiStream::m_uiNifFileVersion, 0x0a01_006c);
        e.call(0x00a2_f910, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 8), 0x0101);
        // Other versions read nothing more.
        for version in [0x0a01_0067u32, 0x0a01_006d] {
            e.set(stream, NiStream::m_uiNifFileVersion, version);
            e.call(0x00a2_f910, &args![this, stream]);
        }
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 2);
    }

    #[test]
    fn link_object_jumps_to_the_time_controller() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_LINK_OBJECT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c9_a6b0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|c| c.0 == TIME_CONTROLLER_LINK_OBJECT)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000, 0x2000]);
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn reset_time_extrema_clears_the_key_times_unless_managed() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.set_f32(this + LO_KEY_TIME, 2.0);
        e.mem.set_f32(this + HI_KEY_TIME, 3.0);
        e.mem.set_u8(this + 8, 0x20);
        e.call(0x00a2_f8f0, &args![this]);
        assert_eq!(e.mem.f32(this + LO_KEY_TIME), 2.0);
        e.mem.set_u8(this + 8, 0xdf);
        e.call(0x00a2_f8f0, &args![this]);
        assert_eq!(e.mem.u32(this + LO_KEY_TIME), 0);
        assert_eq!(e.mem.u32(this + HI_KEY_TIME), 0);
    }

    #[test]
    fn get_viewer_strings_adds_the_name_and_the_manager_flag() {
        let mut e = Engine::new();
        e.map(NI_INTERP_CONTROLLER_RTTI, 4);
        e.mem.set_u32(NI_INTERP_CONTROLLER_RTTI, 0x0155_0000);
        e.register(TIME_CONTROLLER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0x1234_5678));
        e.register(NI_GET_VIEWER_STRING_BOOL, |_, a| ret(a[1]));
        e.register(VIEWER_STRINGS_GROW, |_, _| Ret::default());
        e.register(VIEWER_STRINGS_SET_AT, |e, a| ret(e.mem.u32(a[2])));
        let this = e.mem.alloc(0x40);
        e.mem.set_u8(this + 8, 0x20);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 4);
        e.call_log = Some(vec![]);
        e.call(0x00a2_f980, &args![this, strings]);
        let log = e.call_log.take().unwrap();
        let of = |address: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == address)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(
            of(TIME_CONTROLLER_GET_VIEWER_STRINGS),
            [vec![this, strings]]
        );
        assert_eq!(of(NI_GET_VIEWER_STRING), [vec![0x0155_0000]]);
        // The flag goes into the low byte of the word that held the name string.
        assert_eq!(
            of(NI_GET_VIEWER_STRING_BOOL),
            [vec![TEXT_IS_MANAGER_CONTROLLED, 0x1234_5601]]
        );
        assert_eq!(
            of(VIEWER_STRINGS_GROW),
            [vec![strings, 5], vec![strings, 5]]
        );
        let set_at = of(VIEWER_STRINGS_SET_AT);
        assert_eq!(set_at.len(), 2);
        assert_eq!(&set_at[0][..2], [strings, 1]);
    }
}
