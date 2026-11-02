//! Formatting helpers shared by the commands.

use esm::text::{decode_cp1252, looks_like_text};
use esm::{FourCC, Subrecord};

/// A subrecord's contents in one line: quoted text if it looks like text,
/// otherwise the first bytes in hex.
pub fn preview(sub: &Subrecord) -> String {
    let data = &sub.data;
    if data.is_empty() {
        return String::new();
    }
    if looks_like_text(data) {
        let text = decode_cp1252(data.strip_suffix(&[0]).unwrap_or(data));
        return format!("\"{}\"", one_line(&text, 70));
    }
    let max = if FULL.load(std::sync::atomic::Ordering::Relaxed) {
        usize::MAX
    } else {
        24
    };
    let hex: Vec<String> = data.iter().take(max).map(|b| format!("{b:02X}")).collect();
    let mut s = hex.join(" ");
    if data.len() > max {
        s.push_str(&format!(" ... (+{} bytes)", data.len() - max));
    }
    s
}

/// `--full`: show every byte (see [`preview`]).
pub static FULL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn one_line(text: &str, max_chars: usize) -> String {
    let flat: String = text
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    truncate(&flat, max_chars)
}

pub fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        text.to_string()
    } else {
        let mut s: String = text.chars().take(max_chars.saturating_sub(1)).collect();
        s.push('…');
        s
    }
}

pub fn thousands(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

pub fn human_bytes(n: u64) -> String {
    const KIB: f64 = 1024.0;
    let n = n as f64;
    if n >= KIB * KIB * KIB {
        format!("{:.1} GiB", n / (KIB * KIB * KIB))
    } else if n >= KIB * KIB {
        format!("{:.1} MiB", n / (KIB * KIB))
    } else if n >= KIB {
        format!("{:.1} KiB", n / KIB)
    } else {
        format!("{n} B")
    }
}

/// Short descriptions of common Fallout 3 / New Vegas record types.
pub fn describe_type(kind: FourCC) -> Option<&'static str> {
    let desc = match kind.as_bytes() {
        b"ACHR" => "placed NPCs",
        b"ACRE" => "placed creatures",
        b"ACTI" => "activators",
        b"ALCH" => "consumables (food, drink, chems)",
        b"AMMO" => "ammunition",
        b"ARMO" => "armor and clothing",
        b"BOOK" => "books and magazines",
        b"CCRD" => "Caravan cards",
        b"CELL" => "cells (interiors and exterior grid squares)",
        b"CHAL" => "challenges",
        b"CONT" => "containers",
        b"CREA" => "creatures",
        b"CSNO" => "casinos",
        b"DIAL" => "dialogue topics",
        b"DOOR" => "doors",
        b"FACT" => "factions",
        b"FLST" => "form lists",
        b"FURN" => "furniture",
        b"GLOB" => "global variables",
        b"GMST" => "game settings",
        b"IMOD" => "weapon mods",
        b"INFO" => "dialogue responses",
        b"KEYM" => "keys",
        b"LAND" => "terrain",
        b"LIGH" => "lights",
        b"LVLC" => "leveled creature lists",
        b"LVLI" => "leveled item lists",
        b"LVLN" => "leveled NPC lists",
        b"MGEF" => "magic/base effects",
        b"MISC" => "miscellaneous items",
        b"NAVM" => "navigation meshes",
        b"NOTE" => "notes and holotapes",
        b"NPC_" => "non-player characters",
        b"PACK" => "AI packages",
        b"PERK" => "perks",
        b"PGRE" => "placed grenades/projectiles",
        b"QUST" => "quests",
        b"RACE" => "races",
        b"RCPE" => "crafting recipes",
        b"REFR" => "placed objects",
        b"REPU" => "reputations",
        b"SCPT" => "scripts",
        b"SOUN" => "sounds",
        b"SPEL" => "actor effects",
        b"STAT" => "static objects",
        b"TERM" => "terminals",
        b"TXST" => "texture sets",
        b"WEAP" => "weapons",
        b"WRLD" => "worldspaces",
        _ => return None,
    };
    Some(desc)
}
