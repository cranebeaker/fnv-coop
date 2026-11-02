use std::fmt;

/// A four-character code identifying a record, group or subrecord type,
/// such as `WEAP`, `NPC_` or `EDID`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FourCC(pub [u8; 4]);

impl FourCC {
    pub const fn new(bytes: &[u8; 4]) -> Self {
        FourCC(*bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }

    /// Parses user input such as `"weap"` or `"NPC_"`. Record types are
    /// upper case in the file, so the input is upper-cased.
    pub fn parse(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 4 || !bytes.iter().all(|b| b.is_ascii_graphic()) {
            return None;
        }
        let mut code = [0u8; 4];
        for (dst, src) in code.iter_mut().zip(bytes) {
            *dst = src.to_ascii_uppercase();
        }
        Some(FourCC(code))
    }
}

impl fmt::Display for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text: String = self
            .0
            .iter()
            .map(|&b| {
                if b.is_ascii_graphic() || b == b' ' {
                    b as char
                } else {
                    '?'
                }
            })
            .collect();
        f.pad(&text)
    }
}

impl fmt::Debug for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FourCC(\"{self}\")")
    }
}

/// A form ID: the 32-bit identifier of a record.
///
/// The top byte is a load-order index into the plugin's master list: in a
/// plugin with masters `[FalloutNV.esm]`, IDs starting `00` belong to
/// FalloutNV.esm and IDs starting `01` are new records defined by the plugin
/// itself. The lower 24 bits identify the object within that file.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct FormId(pub u32);

impl FormId {
    pub fn mod_index(self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub fn object_id(self) -> u32 {
        self.0 & 0x00FF_FFFF
    }

    /// Parses a hexadecimal form ID such as `"0000000F"` or `"0x0000000f"`.
    pub fn parse_hex(s: &str) -> Option<Self> {
        let digits = s
            .strip_prefix("0x")
            .or_else(|| s.strip_prefix("0X"))
            .unwrap_or(s);
        if digits.is_empty() || digits.len() > 8 || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        u32::from_str_radix(digits, 16).ok().map(FormId)
    }
}

impl fmt::Display for FormId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08X}", self.0)
    }
}

impl fmt::Debug for FormId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FormId({:08X})", self.0)
    }
}

/// Well-known type codes.
pub mod sig {
    use super::FourCC;

    // Structure
    pub const TES4: FourCC = FourCC::new(b"TES4");
    pub const GRUP: FourCC = FourCC::new(b"GRUP");
    pub const XXXX: FourCC = FourCC::new(b"XXXX");

    // Header subrecords
    pub const HEDR: FourCC = FourCC::new(b"HEDR");
    pub const CNAM: FourCC = FourCC::new(b"CNAM");
    pub const SNAM: FourCC = FourCC::new(b"SNAM");
    pub const MAST: FourCC = FourCC::new(b"MAST");

    // Common subrecords
    pub const EDID: FourCC = FourCC::new(b"EDID");
    pub const FULL: FourCC = FourCC::new(b"FULL");
    pub const DATA: FourCC = FourCC::new(b"DATA");
    pub const NAME: FourCC = FourCC::new(b"NAME");
    pub const MODL: FourCC = FourCC::new(b"MODL");

    // Record types
    pub const WEAP: FourCC = FourCC::new(b"WEAP");
    pub const ARMO: FourCC = FourCC::new(b"ARMO");
    pub const AMMO: FourCC = FourCC::new(b"AMMO");
    pub const NPC_: FourCC = FourCC::new(b"NPC_");
    pub const CREA: FourCC = FourCC::new(b"CREA");
    pub const CELL: FourCC = FourCC::new(b"CELL");
    pub const WRLD: FourCC = FourCC::new(b"WRLD");
    pub const REFR: FourCC = FourCC::new(b"REFR");
    pub const ACHR: FourCC = FourCC::new(b"ACHR");
    pub const ACRE: FourCC = FourCC::new(b"ACRE");
    pub const PGRE: FourCC = FourCC::new(b"PGRE");
    pub const PMIS: FourCC = FourCC::new(b"PMIS");
    pub const QUST: FourCC = FourCC::new(b"QUST");
    pub const DIAL: FourCC = FourCC::new(b"DIAL");
    pub const INFO: FourCC = FourCC::new(b"INFO");
}
