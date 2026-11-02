//! `fallout/interface/run_00709f00` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte `Setting` base constructor (`0045cfb0`), the vtable of the
/// interface settings and their collection (`004de490`).
const SETTING_CONSTRUCT_BYTE: u32 = 0x0045_cfb0;
const SETTING_VTABLE: u32 = 0x0102_2738;
const COLLECTION: u32 = 0x004d_e490;
/// Slot of a settings collection's vtable that adds a setting.
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 00709f00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a boolean setting of the `004de490` collection
/// (`thiscall`, `RET 8`; the value a byte, `MOVZX`): the byte base
/// constructor (`0045cfb0`), the vtable `01022738`, then adds itself to the
/// collection (vtable slot +4). Returns `this`. The exception-unwinding
/// frame is not translated.
pub fn fn_00709f00(e: &mut Engine, this: Ptr, name: u32, value: u8) -> Ptr {
    e.call(SETTING_CONSTRUCT_BYTE, &args![this, name, value as u32]);
    e.mem.set_u32(this.addr(), SETTING_VTABLE);
    let collection = e.call(COLLECTION, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

// Translated from 00709f80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A six-word record's constructor (`thiscall`): five words cleared and the
/// sixth set to 1. Returns `this`.
pub fn fn_00709f80(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    for i in 0..5 {
        e.mem.set_u32(t + 4 * i, 0);
    }
    e.mem.set_u32(t + 0x14, 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00709f00, fn_00709f00(Ptr, u32, u8) -> Ptr),
        entry!(0x00709f80, fn_00709f80(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_boolean_setting_joins_its_collection() {
        let mut e = Engine::new();
        let c = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(c, 0x0090_0000);
        e.register_double(COLLECTION, move |_, _| c.into_ret());
        e.register(0x0090_1000, |_, _| Ret::default());
        let s = e.mem.alloc(0xc);
        e.call(0x00709f00, &[s, 0x30, 1]);
        assert_eq!(e.mem.u32(s), SETTING_VTABLE);
        assert_eq!(e.mem.u8(s + 4), 1);
        assert_eq!(e.mem.u32(s + 8), 0x30);
    }

    #[test]
    fn the_record_is_cleared_with_its_last_word_one() {
        let mut e = Engine::new();
        let r = e.mem.alloc(0x18);
        e.mem.write(r, &[0xff; 0x18]);
        e.call(0x00709f80, &[r]);
        let got: Vec<u32> = (0..6).map(|i| e.mem.u32(r + 4 * i)).collect();
        assert_eq!(got, [0, 0, 0, 0, 0, 1]);
    }
}
