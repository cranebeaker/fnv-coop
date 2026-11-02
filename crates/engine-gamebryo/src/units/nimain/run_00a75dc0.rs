//! `NiMain/run_00a75dc0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiTriBasedGeomData` constructor taking eight words (`00a8a920`,
/// thiscall, `RET 0x20`).
const NI_TRI_BASED_GEOM_DATA_CONSTRUCT_00A8A920: u32 = 0x00a8_a920;
/// The vtable of `NiTriStripsData` (RTTI `.?AVNiTriStripsData@@`).
const NI_TRI_STRIPS_DATA_VTABLE: u32 = 0x0109_d8ac;
/// `NiTriStripsData` (Xbox PDB, PC offsets as the code writes them):
/// `m_usStrips` +0x44 (u16), `m_pusStripLengths` +0x48,
/// `m_pusStripLists` +0x4c.
const STRIPS: u32 = 0x44;
const STRIP_LENGTHS: u32 = 0x48;
const STRIP_LISTS: u32 = 0x4c;

// Translated from 00a75dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData`'s constructor (by the vtable it stores; thiscall,
/// `RET 0x2c`): the eight geometry words go to the `NiTriBasedGeomData`
/// constructor (`00a8a920`), then the strip lengths, the vtable, the strip
/// count (the low half of its word) and the strip lists. Returns `this`.
pub fn ni_tri_strips_data_construct(
    e: &mut Engine,
    this: Ptr,
    geometry: [u32; 8],
    strips: u16,
    strip_lengths: u32,
    strip_lists: u32,
) -> Ptr {
    let base = this.addr();
    let mut words = vec![base];
    words.extend_from_slice(&geometry);
    e.call(NI_TRI_BASED_GEOM_DATA_CONSTRUCT_00A8A920, &words);
    e.mem.set_u32(base + STRIP_LENGTHS, strip_lengths);
    e.mem.set_u32(base, NI_TRI_STRIPS_DATA_VTABLE);
    e.mem.set_u16(base + STRIPS, strips);
    e.mem.set_u32(base + STRIP_LISTS, strip_lists);
    this
}

/// The registered form: `this` and the eleven stack words.
fn ni_tri_strips_data_construct_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let mut geometry = [0u32; 8];
    geometry.copy_from_slice(&words[1..9]);
    ni_tri_strips_data_construct(
        e,
        Ptr::new(words[0]),
        geometry,
        words[9] as u16,
        words[10],
        words[11],
    )
    .addr()
    .into_ret()
}

/// `NiTriBasedGeomData::NiTriBasedGeomData` (`00a8a970`, engine map; `thiscall`, no stack
/// words): the default constructor of the base.
const NI_TRI_BASED_GEOM_DATA_DEFAULT_CONSTRUCT: u32 = 0x00a8_a970;

// Translated from 00a75e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData`'s default constructor (by the vtable it stores; `thiscall`, no stack
/// words): the `NiTriBasedGeomData` default constructor (`00a8a970`), then the strip count
/// (+0x44, 16 bits), the strip lengths (+0x48) and the strip lists (+0x4c) cleared and the
/// vtable. Returns `this`.
pub fn fn_00a75e20(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_TRI_BASED_GEOM_DATA_DEFAULT_CONSTRUCT, &args![base]);
    e.mem.set_u16(base + STRIPS, 0);
    e.mem.set_u32(base + STRIP_LENGTHS, 0);
    e.mem.set_u32(base + STRIP_LISTS, 0);
    e.mem.set_u32(base, NI_TRI_STRIPS_DATA_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00a7_5dc0, ni_tri_strips_data_construct_entry as AbiFn),
        entry!(0x00a7_5e20, fn_00a75e20(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_base_then_the_strip_fields() {
        let mut e = Engine::new();
        e.register(NI_TRI_BASED_GEOM_DATA_CONSTRUCT_00A8A920, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 0x40, 0x0007_0007);
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + STRIPS, 0xdddd_dddd);
        e.call_log = Some(vec![]);
        let words = [
            this,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            0xabcd_0003,
            0x0600_0000,
            0x0700_0000,
        ];
        assert_eq!(e.call(0x00a7_5dc0, &words).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TRI_STRIPS_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 0x40), 0x0007_0007);
        assert_eq!(e.mem.u32(this + STRIPS), 0xdddd_0003);
        assert_eq!(e.mem.u32(this + STRIP_LENGTHS), 0x0600_0000);
        assert_eq!(e.mem.u32(this + STRIP_LISTS), 0x0700_0000);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == NI_TRI_BASED_GEOM_DATA_CONSTRUCT_00A8A920)
            .collect();
        assert_eq!(base[0].1, vec![this, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
}

#[cfg(test)]
mod default_constructor_tests {
    use super::*;

    #[test]
    fn constructs_the_base_then_clears_the_strip_fields() {
        let mut e = Engine::new();
        e.register(NI_TRI_BASED_GEOM_DATA_DEFAULT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x40, 0x0007_0007);
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        e.mem.write(this + 0x44, &[0xdd; 0xc]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_5e20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TRI_STRIPS_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 0x40), 0x0007_0007);
        assert_eq!(e.mem.u32(this + STRIPS), 0xdddd_0000);
        assert_eq!(e.mem.u32(this + STRIP_LENGTHS), 0);
        assert_eq!(e.mem.u32(this + STRIP_LISTS), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(NI_TRI_BASED_GEOM_DATA_DEFAULT_CONSTRUCT, vec![this])));
    }
}
