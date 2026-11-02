//! `NiXenonRenderer/nixenonerror.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `"UNKNOWN!"` (`010a094c`): the answer for a code without a name.
const UNKNOWN_ERROR: u32 = 0x010a_094c;

// Translated from 00e89d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonErrorString` (Xbox PDB, engine map), `cdecl`, one word (an `HRESULT`): the address
/// of the code's name in the exe's strings (`"E_FAIL"`, `"D3DERR_DEVICELOST"`, ...), or
/// `"UNKNOWN!"`. The codes above `D3DERR_WRONGTEXTUREFORMAT` (`88760818`) go through the
/// compiler's switch (a byte table at `00e89e78` over the code minus `88760819`, 0x54 cases,
/// into the jump table at `00e89e2c`), written here as `match` arms in the tables' mapping.
pub fn ni_xenon_error_string(_e: &mut Engine, code: u32) -> u32 {
    if code <= 0x8876_0818 {
        if code == 0x8876_0818 {
            // D3DERR_WRONGTEXTUREFORMAT
            return 0x010e_fb1c;
        }
        if code <= 0x8007_0057 {
            return match code {
                // E_INVALIDARG
                0x8007_0057 => 0x010e_fb50,
                // E_FAIL
                0x8000_4005 => 0x010e_fb60,
                // E_OUTOFMEMORY
                0x8007_000e => 0x010e_fb68,
                _ => UNKNOWN_ERROR,
            };
        }
        if code == 0x8876_017c {
            // D3DERR_OUTOFVIDEOMEMORY
            return 0x010e_fb38;
        }
        return UNKNOWN_ERROR;
    }
    match code.wrapping_sub(0x8876_0819) {
        // D3DERR_UNSUPPORTEDCOLOROPERATION
        0 => 0x010e_f974,
        // D3DERR_UNSUPPORTEDCOLORARG
        1 => 0x010e_f998,
        // D3DERR_UNSUPPORTEDALPHAOPERATION
        2 => 0x010e_f9b4,
        // D3DERR_UNSUPPORTEDALPHAARG
        3 => 0x010e_f9d8,
        // D3DERR_TOOMANYOPERATIONS
        4 => 0x010e_f9f4,
        // D3DERR_CONFLICTINGTEXTUREFILTER
        5 => 0x010e_fadc,
        // D3DERR_UNSUPPORTEDFACTORVALUE
        6 => 0x010e_f954,
        // D3DERR_CONFLICTINGRENDERSTATE
        8 => 0x010e_fafc,
        // D3DERR_UNSUPPORTEDTEXTUREFILTER
        9 => 0x010e_f934,
        // D3DERR_CONFLICTINGTEXTUREPALETTE
        13 => 0x010e_fab8,
        // D3DERR_DRIVERINTERNALERROR
        14 => 0x010e_fa70,
        // D3DERR_NOTFOUND
        77 => 0x010e_fa10,
        // D3DERR_MOREDATA
        78 => 0x010e_fa34,
        // D3DERR_DEVICELOST
        79 => 0x010e_faa4,
        // D3DERR_DEVICENOTRESET
        80 => 0x010e_fa8c,
        // D3DERR_NOTAVAILABLE
        81 => 0x010e_fa20,
        // D3DERR_INVALIDDEVICE
        82 => 0x010e_fa44,
        // D3DERR_INVALIDCALL
        83 => 0x010e_fa5c,
        // Cases 7, 10 to 12 and 15 to 76 of the byte table, and anything past 0x53.
        _ => UNKNOWN_ERROR,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e8_9d50, ni_xenon_error_string(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_string_names_each_code() {
        let mut e = Engine::new();
        for (code, name) in [
            (0x8876_0818u32, 0x010e_fb1cu32),
            (0x8007_0057, 0x010e_fb50),
            (0x8000_4005, 0x010e_fb60),
            (0x8007_000e, 0x010e_fb68),
            (0x8876_017c, 0x010e_fb38),
            (0x8876_0819, 0x010e_f974),
            (0x8876_081e, 0x010e_fadc),
            (0x8876_0821, 0x010e_fafc),
            (0x8876_0827, 0x010e_fa70),
            (0x8876_0866, 0x010e_fa10),
            (0x8876_0868, 0x010e_faa4),
            (0x8876_086c, 0x010e_fa5c),
            // Gaps in the switch, and codes outside every range.
            (0x8876_0820, UNKNOWN_ERROR),
            (0x8876_0830, UNKNOWN_ERROR),
            (0x8876_086d, UNKNOWN_ERROR),
            (0x8000_4004, UNKNOWN_ERROR),
            (0x8007_0058, UNKNOWN_ERROR),
            (0, UNKNOWN_ERROR),
        ] {
            assert_eq!(e.call(0x00e8_9d50, &[code]).u32(), name, "{code:08x}");
        }
    }
}
