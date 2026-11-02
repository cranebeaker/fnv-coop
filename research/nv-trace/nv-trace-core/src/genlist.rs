//! Function-list lines generated from the port's own translations: for an address, the unit
//! file that says `// Translated from ADDR` holds the function that registers for it, and its
//! Rust signature gives the words (`this` first) and the return kind; its doc comment usually
//! names the convention. What a translation's signature can't say (a word the translation
//! drops, a custom register convention) is the translator's to check: generated lines carry
//! where they came from.

/// The list line for `addr` from one unit file's text, if the file translates it.
pub fn line_for(source: &str, addr: u32) -> Option<String> {
    let marker = format!("// Translated from {addr:08x}");
    let at = source.find(&marker)?;
    let rest = &source[at..];
    // The doc comment and attributes up to the function's signature.
    let fn_at = rest.find("pub fn ")?;
    let docs = &rest[..fn_at];
    let sig_end = rest[fn_at..].find('{')? + fn_at;
    let sig = &rest[fn_at..sig_end];
    let (params, ret) = split_signature(sig)?;
    let mut words = String::new();
    let mut first_this = false;
    for (i, (name, ty)) in params.iter().enumerate() {
        if ty.contains("Engine") {
            continue;
        }
        if words.is_empty() && (name == "this" || name.starts_with("this_")) && i <= 1 {
            first_this = true;
        }
        words.push_str(&letters(ty));
    }
    let lower = docs.to_ascii_lowercase();
    let conv = if lower.contains("fastcall") {
        "fastcall"
    } else if lower.contains("thiscall") || first_this {
        "thiscall"
    } else if lower.contains("stdcall") {
        "stdcall"
    } else {
        "cdecl"
    };
    if conv == "fastcall" && words.len() < 2 || conv == "thiscall" && words.is_empty() {
        return None;
    }
    let ret = ret_kind(ret.as_deref());
    let name = docs
        .lines()
        .filter(|l| l.trim_start().starts_with("///"))
        .find_map(|l| {
            let s = l.split('`').nth(1)?;
            let plain = s.chars().all(|c| c.is_alphanumeric() || c == '_');
            let upper = s.chars().next().is_some_and(char::is_uppercase);
            let lower = s.chars().any(char::is_lowercase);
            (s.contains("::") || (plain && lower && (upper || s.contains('_'))))
                .then(|| s.to_string())
        })
        .unwrap_or_else(|| format!("{addr:08x} (no label)"));
    Some(format!(
        "{addr:08x} {conv:<8} {:<15} {ret:<4} {name}",
        if words.is_empty() {
            "-".to_string()
        } else {
            words
        }
    ))
}

/// A signature's parameters (name, type) and return type.
type Signature = (Vec<(String, String)>, Option<String>);

/// The parameters (name, type) and the return type of `pub fn name(...) -> R`.
fn split_signature(sig: &str) -> Option<Signature> {
    let open = sig.find('(')?;
    // The matching close parenthesis.
    let mut depth = 0;
    let mut close = None;
    for (i, c) in sig[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    close = Some(open + i);
                    break;
                }
            }
            _ => {}
        }
    }
    let close = close?;
    let inner = &sig[open + 1..close];
    let mut params = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    let bytes: Vec<char> = inner.chars().collect();
    let mut parts = Vec::new();
    for (i, &c) in bytes.iter().enumerate() {
        match c {
            '<' | '(' | '[' => depth += 1,
            '>' | ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(bytes[start..i].iter().collect::<String>());
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(bytes[start..].iter().collect::<String>());
    for p in parts {
        let p = p.trim();
        if p.is_empty() {
            continue;
        }
        let (name, ty) = p.split_once(':')?;
        params.push((
            name.trim().trim_start_matches("mut ").to_string(),
            ty.trim().to_string(),
        ));
    }
    let ret = sig[close + 1..]
        .split_once("->")
        .map(|(_, r)| r.trim().to_string())
        .filter(|r| !r.is_empty());
    Some((params, ret))
}

/// The word letters a parameter type takes.
fn letters(ty: &str) -> String {
    let t = ty.trim();
    match t {
        "f32" => "f".into(),
        "u8" | "i8" | "bool" => "b".into(),
        "u16" | "i16" => "h".into(),
        "f64" | "u64" | "i64" => "ww".into(),
        "u32" | "i32" | "usize" | "isize" => "w".into(),
        _ if t.starts_with("Ptr") || t.starts_with('&') || t.ends_with("Ptr") => "p".into(),
        // A typed handle or a named alias of a pointer (FaceData, ...): one word.
        _ => "p".into(),
    }
}

fn ret_kind(ret: Option<&str>) -> &'static str {
    match ret.map(str::trim) {
        None | Some("()") => "void",
        Some("bool" | "u8" | "i8") => "u8",
        Some("u16" | "i16") => "u16",
        Some("u32" | "i32") => "u32",
        Some("f32") => "f32",
        Some("f64") => "f64",
        Some("u64" | "i64") => "i64",
        Some(t) if t.starts_with("Ptr") => "ptr",
        Some("Ret") => "u32",
        Some(_) => "u32",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    const SRC: &str = r#"
// Translated from 00d41a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPredGskAgent3::process` (engine map, cdecl `(pair, entry, data, input, collector)`).
pub fn hkp_pred_gsk_agent3_process(
    e: &mut Engine,
    pair: Ptr,
    entry: Ptr,
    data: Ptr,
    input: Ptr,
    collector: Ptr,
) -> u32 {
    0
}

// Translated from 0064ad20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ad20 - the exception frame is not translated
/// `BSFaceGenAnimationData::AddSpeechEmptyKeyframe` (Xbox PDB, vtable
/// +0xF4): unless dead, adds an empty keyframe.
pub fn add_speech_empty_keyframe(
    e: &mut Engine,
    this: FaceData,
    morph_type: u32,
    time: f32,
    _unused_3: u32,
) {
}

// Translated from 00ba3aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: an index and the address of a value).
pub fn fn_00ba3aa0(e: &mut Engine, this: Ptr, index: u32, value: Ptr<NiTArray<u32>>) -> bool {
    true
}
"#;

    #[test]
    fn lines_come_from_the_signatures() {
        let l = line_for(SRC, 0x00d4_1a30).unwrap();
        assert_eq!(
            list::parse(&l).unwrap()[0].line(),
            "00d41a30 cdecl ppppp u32 hkPredGskAgent3::process"
        );
        let l = line_for(SRC, 0x0064_ad20).unwrap();
        assert_eq!(
            list::parse(&l).unwrap()[0].line(),
            "0064ad20 thiscall pwfw void BSFaceGenAnimationData::AddSpeechEmptyKeyframe"
        );
        let l = line_for(SRC, 0x00ba_3aa0).unwrap();
        assert_eq!(
            list::parse(&l).unwrap()[0].line(),
            "00ba3aa0 thiscall pwp u8 00ba3aa0 (no label)"
        );
        assert!(line_for(SRC, 0x0040_1000).is_none());
    }
}
