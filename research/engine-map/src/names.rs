//! Name rules shared by `match` and `frame`.

/// PowerPC prologue/epilogue helpers (`__savegprlr_14` and friends) have no
/// x86 counterpart; they are left out of Xbox call lists.
pub fn ppc_helper(name: &str) -> bool {
    let n = name.trim_start_matches('_');
    [
        "savegpr", "restgpr", "savefpr", "restfpr", "savevmx", "restvmx",
    ]
    .iter()
    .any(|p| n.starts_with(p))
}

/// An Xbox name as Ghidra accepts it (`*`→`P`, `&`→`R`, operators spelled out).
pub fn sanitize(name: &str) -> String {
    // Operators whose symbols contain < or > would read as template brackets.
    let mut n = name.to_string();
    for (op, word) in [
        ("operator<<=", "operator_shl_assign"),
        ("operator>>=", "operator_shr_assign"),
        ("operator<<", "operator_shl"),
        ("operator>>", "operator_shr"),
        ("operator<=", "operator_le"),
        ("operator>=", "operator_ge"),
        ("operator->", "operator_arrow"),
        ("operator<", "operator_lt"),
        ("operator>", "operator_gt"),
    ] {
        n = n.replace(op, word);
    }
    let n = n.replace('*', "P").replace('&', "R");
    let mut out = String::new();
    let mut last_us = false;
    for c in n.chars() {
        let ok = c.is_ascii_alphanumeric() || matches!(c, '_' | ':' | '~' | '<' | '>');
        if ok {
            out.push(c);
            last_us = c == '_';
        } else if !last_us {
            out.push('_');
            last_us = true;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers_and_sanitize() {
        assert!(ppc_helper("__savegprlr_14"));
        assert!(!ppc_helper("Main::OnIdle"));
        assert_eq!(
            sanitize("NiPointer<A *>::operator->"),
            "NiPointer<A_P>::operator_arrow"
        );
    }
}
