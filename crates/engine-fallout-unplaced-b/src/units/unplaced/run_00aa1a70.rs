//! `(unplaced)/run_00aa1a70` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function-pointer global `00aa1a70` sets.
const CALLBACK_SLOT: u32 = 0x011a_c228;
/// Its default: `00aa1a40` (placed in `NiSystem/nifile.cpp` by the engine
/// map, 45 bytes; not named there).
const DEFAULT_CALLBACK: u32 = 0x00aa_1a40;

// Translated from 00aa1a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): stores `callback` in the
/// function-pointer global `011ac228`, or the default `00aa1a40` when
/// `callback` is null (the exe writes the default first, then overwrites it).
pub fn fn_00aa1a70(e: &mut Engine, callback: u32) {
    e.set_global(CALLBACK_SLOT, DEFAULT_CALLBACK);
    if callback != 0 {
        e.set_global(CALLBACK_SLOT, callback);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_1a70, fn_00aa1a70(u32)),
        entry!(0x00aa_1a90, fn_00aa1a90(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_callback_or_the_default() {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.call(0x00aa_1a70, &args![0x0055_0000u32]);
        assert_eq!(e.global::<u32>(CALLBACK_SLOT), 0x0055_0000);
        e.call(0x00aa_1a70, &args![0u32]);
        assert_eq!(e.global::<u32>(CALLBACK_SLOT), DEFAULT_CALLBACK);
    }
}

/// The seek origin `00aa1a90` passes (the word at `010a2484`, 1 in the exe).
const SEEK_ORIGIN_010A2484: u32 = 0x010a_2484;

// Translated from 00aa1a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A stream's one-argument seek (slot +8 of the vtables of `NiFile`,
/// `SaveGameFile`, `Archive`, `ArchiveFile`, `CompressedArchiveFile` and
/// `BSFile`, by their RTTI; the Xbox PDB's `NiBinaryStream` names slot +8
/// `Seek`): calls the object's slot +0x14 with `offset` and the word at
/// `010a2484`, and passes its EAX on.
pub fn fn_00aa1a90(e: &mut Engine, this: u32, offset: u32) -> u32 {
    let origin = e.mem.u32(SEEK_ORIGIN_010A2484);
    e.vcall(this, 0x14, &[offset, origin]).u32()
}

#[cfg(test)]
mod n079_tests {
    use super::*;

    #[test]
    fn seek_passes_the_origin() {
        const SEEK2: u32 = 0x00de_ad14;
        let mut e = Engine::new();
        e.map(0x010a_2000, 0x1000);
        e.mem.set_u32(SEEK_ORIGIN_010A2484, 1);
        e.put_vtable(0x0070_0000, &[0, 0, 0, 0, 0, SEEK2]);
        e.register(SEEK2, |_, a| (a[1] + a[2]).into_ret());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0070_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_1a90, &[this, 40]).u32(), 41);
        assert_eq!(e.call_log.take().unwrap()[1], (SEEK2, vec![this, 40, 1]));
    }
}
