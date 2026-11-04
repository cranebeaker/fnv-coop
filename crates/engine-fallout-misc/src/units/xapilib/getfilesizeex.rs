//! `XAPILIB/getfilesizeex.c` (Xbox PDB source unit), subsystem `XAPILIB`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The import slot of KERNEL32 `GetFileSizeEx` (`stdcall`, two words: handle, the address of
/// the 64-bit size).
const IMPORT_GET_FILE_SIZE_EX: u32 = 0x00fd_f13c;

// Translated from 00aff000 (disassembled, FalloutNV.exe 1.4.0.525)
/// `GetFileSizeEx` (engine map; `thiscall`, one word: the address of a 64-bit size, `RET 4`;
/// called from `DoGetSize` `0065c29e`): asks `GetFileSizeEx` for the size of the handle at +0x10
/// into a local. On success the size is copied out and 0 answered; on failure the size is set to
/// 0 and 4 answered.
pub fn get_file_size_ex(e: &mut Engine, this: u32, size: u32) -> u32 {
    e.with_stack(8, |e, local| {
        let handle = e.mem.u32(this.wrapping_add(0x10));
        if e.call(IMPORT_GET_FILE_SIZE_EX, &args![handle, local.addr()])
            .u32()
            != 0
        {
            let low = e.mem.u32(local.addr());
            let high = e.mem.u32(local.addr() + 4);
            e.mem.set_u32(size, low);
            e.mem.set_u32(size.wrapping_add(4), high);
            0
        } else {
            e.mem.set_u32(size, 0);
            e.mem.set_u32(size.wrapping_add(4), 0);
            4
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00af_f000, get_file_size_ex(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_size_is_copied_out_or_zeroed() {
        let mut e = Engine::new();
        e.register(IMPORT_GET_FILE_SIZE_EX, |e, a| {
            if a[0] != 0x44 {
                return 0u32.into_ret();
            }
            e.mem.set_u32(a[1], 0x1234_5678);
            e.mem.set_u32(a[1] + 4, 2);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x14);
        let size = e.mem.alloc(8);
        e.mem.set_u32(this + 0x10, 0x44);
        assert_eq!(e.call(0x00af_f000, &args![this, size]).u32(), 0);
        assert_eq!((e.mem.u32(size), e.mem.u32(size + 4)), (0x1234_5678, 2));
        e.mem.set_u32(this + 0x10, 0x48);
        assert_eq!(get_file_size_ex(&mut e, this, size), 4);
        assert_eq!((e.mem.u32(size), e.mem.u32(size + 4)), (0, 0));
    }
}
