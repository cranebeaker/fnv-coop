//! What a typed marker is: one definition, used by the ledger (which counts them) and by the
//! repository's hygiene test (`crates/nvinspect/tests/repo_hygiene.rs`, which includes this file and
//! refuses a malformed one), so the two can't disagree about a line.

/// How a marker starts, once its indentation is off. Then come its address, as eight hex digits, and
/// nothing or `, read against fnv_decomp` (ADR-0009's addition to a marker).
pub const PREFIX: &str = "// Typed from ";

/// Whether a line talks about a typed marker: such a line that isn't a well-formed marker counts for
/// nothing in the ledger, and is a finding in engine code.
pub fn mentioned(line: &str) -> bool {
    line.contains("Typed from")
}

/// The address of a well-formed marker line; `None` for any other line.
pub fn address(line: &str) -> Option<u32> {
    let rest = line.trim_start().strip_prefix(PREFIX)?;
    if !rest.is_char_boundary(rest.len().min(8)) {
        return None;
    }
    let (address, tail) = rest.split_at(rest.len().min(8));
    if address.len() != 8 || !address.bytes().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let tail_ok = matches!(
        tail.trim_end(),
        "" | "." | ", read against fnv_decomp" | ", read against fnv_decomp."
    );
    tail_ok
        .then(|| u32::from_str_radix(address, 16).ok())
        .flatten()
}
