//! The game's form type numbers (the byte at form +4), by record type: the number
//! `TESDataHandler::CreateFormOfType` takes for a record ([`crate::player_move`]'s forms).

use esm::FourCC;

/// Every form type, by its number.
pub const FORM_TYPES: [&str; 121] = [
    "NONE", "TES4", "GRUP", "GMST", "TXST", "MICN", "GLOB", "CLAS", "FACT", "HDPT", "HAIR", "EYES",
    "RACE", "SOUN", "ASPC", "SKIL", "MGEF", "SCPT", "LTEX", "ENCH", "SPEL", "ACTI", "TACT", "TERM",
    "ARMO", "BOOK", "CLOT", "CONT", "DOOR", "INGR", "LIGH", "MISC", "STAT", "SCOL", "MSTT", "PWAT",
    "GRAS", "TREE", "FLOR", "FURN", "WEAP", "AMMO", "NPC_", "CREA", "LVLC", "LVLN", "KEYM", "ALCH",
    "IDLM", "NOTE", "COBJ", "PROJ", "LVLI", "WTHR", "CLMT", "REGN", "NAVI", "CELL", "REFR", "ACHR",
    "ACRE", "PMIS", "PGRE", "PBEA", "PFLA", "WRLD", "LAND", "NAVM", "TLOD", "DIAL", "INFO", "QUST",
    "IDLE", "PACK", "CSTY", "LSCR", "LVSP", "ANIO", "WATR", "EFSH", "TOFT", "EXPL", "DEBR", "IMGS",
    "IMAD", "FLST", "PERK", "BPTD", "ADDN", "AVIF", "RADS", "CAMS", "CPTH", "VTYP", "IPCT", "IPDS",
    "ARMA", "ECZN", "MESG", "RGDL", "DOBJ", "LGTM", "MUSC", "IMOD", "REPU", "PCBE", "RCPE", "RCCT",
    "CHIP", "CSNO", "LSCT", "MSET", "ALOC", "CHAL", "AMEF", "CCRD", "CMNY", "CDCK", "DEHY", "HUNG",
    "SLPD",
];

/// A record type's form type number.
pub fn form_type_number(kind: FourCC) -> Option<u8> {
    FORM_TYPES
        .iter()
        .position(|t| t.as_bytes() == kind.as_bytes())
        .map(|i| i as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_type_numbers_are_the_exes() {
        assert_eq!(form_type_number(FourCC(*b"WEAP")), Some(0x28));
        assert_eq!(form_type_number(FourCC(*b"NPC_")), Some(0x2A));
        assert_eq!(form_type_number(FourCC(*b"SLPD")), Some(120));
        assert_eq!(form_type_number(FourCC(*b"ZZZZ")), None);
    }
}
