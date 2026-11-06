//! Repository hygiene: things that must never be committed.
//!
//! Checks every file in the repository and fails with a list of
//! `path:line: rule` entries when it finds:
//!
//! * a control character (other than tab, line feed and carriage return) in a
//!   text file, a hidden control character (a C1 control, U+0080 to U+009F, or
//!   a bidirectional control such as U+202E, which can make code read
//!   differently from what it does), text that is not valid UTF-8, or a binary
//!   file (a NUL byte in a file whose extension is not a known text type);
//! * an absolute Windows or macOS user-profile path in a text file (write
//!   `%USERPROFILE%` instead). A Windows drive path matches `Users` or the older
//!   `Documents and Settings` in any case; the macOS form `/Users/<name>` is
//!   case-sensitive, because git hosts and web APIs use a lowercase `users`. The
//!   shared profile folders `Public`, `Default`, `Default User`, `All Users` and
//!   `Shared` name nobody and are allowed;
//! * a game, recording or Ghidra project file type, as a file name or as a
//!   directory name (a project database is a `<name>.rep` directory). The name
//!   rule applies to a tracked file even when it was deleted from the work tree;
//! * under `crates/`, `viewer/` and `docs/`: a decompiler's automatic names
//!   (function, label, data, pointer and string names made from an address,
//!   numbered parameters, register inputs, stack and numbered temporaries,
//!   helper calls), or a fenced C or C++ block in Markdown (any of the tags
//!   `c`, `h`, `cc`, `cpp`, `cxx`, `c++`, `hpp` and `hxx`, however the fence
//!   writes it). Write a function as its address (`00b0d7b0`), a global as
//!   `[011d8a84]`, and anything else in plain words;
//! * in typed code (the folders in [`TYPED_DIRS`] and every function under a typed marker,
//!   docs/ENGINE_CRATE.md): an `as` from a float to an integer, `core::arch` or another x86
//!   assumption, decompiled text (C++ words, paths into a decompilation, placeholder names
//!   made from an address), a malformed marker, or a typed function with no marker;
//! * a direct Rust call to an address in `crates/engine-core/data/overridable.tsv`: none in
//!   typed code, and the translations' own stay at the count the table holds, so no new one
//!   appears (`overridable_addresses_are_called_by_address`).
//!
//! Which files: the list comes from `git ls-files`, the tracked files plus the
//! untracked ones git does not ignore, so every ignore source git honours
//! (nested `.gitignore` files, `.git/info/exclude`, the user's global excludes
//! file) applies, and a file added with `git add -f` is checked even under an
//! ignored directory. Files git ignores are not part of the repository and are
//! not read. One exception: an untracked file inside a `.claude` directory is
//! local agent state (settings that name the user's folders, scratch worktrees)
//! and is skipped, as the walk below skips it; once such a file is tracked it
//! is checked like any other. Better still, ignore `.claude/settings.local.json`
//! and `.claude/worktrees/` in the root `.gitignore`. When the checkout has no
//! `.git`, or git cannot be run, the test walks the directory tree instead and
//! follows the simple entries of every `.gitignore` it meets (and always skips
//! `target`, `.git`, `node_modules`, `.build` and `.claude`).
//!
//! Every listed file is read except a few known image and font types. The
//! content rules read the copy in the work tree, not the staged one: run the
//! test before staging, or after the last edit. Entries that cannot be read,
//! and files too large to scan, are reported rather than skipped. Symbolic links
//! are not followed; only their names are checked.
//!
//! Matching is done by hand (the core crates take no external libraries), and
//! the matchers have their own tests below on synthetic strings. Banned tokens
//! are built from pieces in those tests so this file passes its own rules.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

#[path = "../../../scripts/ledger/src/typed_marker.rs"]
mod typed_marker;

/// Directories the fallback walk never enters, wherever they are.
const SKIP_DIRS: &[&str] = &["target", ".git", "node_modules", ".build", ".claude"];

/// Extensions known to be text (lowercase). A dotfile counts by its name, so
/// `.gitignore` has the extension `gitignore`. A NUL byte in one of these is a
/// control character; in a file with any other extension (or none) it means the
/// file is binary.
const TEXT_EXTENSIONS: &[&str] = &[
    "rs",
    "md",
    "toml",
    "lock",
    "wgsl",
    "ps1",
    "cmd",
    "bat",
    "yml",
    "yaml",
    "json",
    "jsonl",
    "txt",
    "csv",
    "tsv",
    "hex",
    "sha256",
    "java",
    "py",
    "sh",
    "c",
    "h",
    "cpp",
    "hpp",
    "s",
    "asm",
    "xml",
    "html",
    "css",
    "js",
    "svg",
    "ini",
    "cfg",
    "patch",
    "diff",
    "gitignore",
    "gitattributes",
];

/// Image and font types that may be committed and are not read.
const BINARY_MEDIA: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "ico", "webp", "ttf", "otf", "woff", "woff2",
];

/// Game data, recordings, decompiler databases and executables: none of
/// these belongs in the repository (lowercase; matched without regard to case).
/// `gbf` and `prp` are the files inside a Ghidra `.rep` directory.
const BANNED_EXTENSIONS: &[&str] = &[
    "esm", "esp", "esl", "bsa", "ba2", "nif", "kf", "egm", "egt", "tri", "dds", "ogg", "wav",
    "mp3", "lip", "fuz", "bik", "fos", "trace", "exe", "dll", "pdb", "sdp", "gpr", "rep", "gzf",
    "idb", "i64", "bndb", "xex", "spt", "gbf", "prp",
];

/// Top-level directories where decompiler names are not allowed.
const PROVENANCE_DIRS: &[&str] = &["crates", "viewer", "docs"];

/// Files above this size are not scanned (and are reported as such).
const MAX_SCAN_BYTES: u64 = 16 * 1024 * 1024;

// ---------------------------------------------------------------------------
// Matchers. Each works on one file or one line and has no file access.
// ---------------------------------------------------------------------------

/// Lowercase extension of a file name, or `None` when it has none.
fn extension(name: &str) -> Option<String> {
    let dot = name.rfind('.')?;
    let ext = &name[dot + 1..];
    if ext.is_empty() {
        None
    } else {
        Some(ext.to_ascii_lowercase())
    }
}

/// The banned extension of a file or directory name, if it has one.
fn banned_extension(name: &str) -> Option<String> {
    extension(name).filter(|ext| BANNED_EXTENSIONS.contains(&ext.as_str()))
}

/// The first component of `rel` (a `/`-separated path) with a banned extension:
/// the path up to and including it, and the extension.
fn banned_component(rel: &str) -> Option<(&str, String)> {
    let mut end = 0;
    for component in rel.split('/') {
        end += component.len();
        if let Some(ext) = banned_extension(component) {
            return Some((&rel[..end], ext));
        }
        end += 1;
    }
    None
}

/// Line number (1-based) and byte of the first control character on each line
/// of `bytes`. Tab, line feed and carriage return are fine; every other byte
/// below 0x20, and DEL, is not.
fn control_characters(bytes: &[u8]) -> Vec<(usize, u8)> {
    let mut found = Vec::new();
    let mut line = 1;
    let mut last_reported = 0;
    for &b in bytes {
        if b == b'\n' {
            line += 1;
            continue;
        }
        let control = (b < 0x20 && b != b'\t' && b != b'\r') || b == 0x7f;
        if control && last_reported != line {
            found.push((line, b));
            last_reported = line;
        }
    }
    found
}

/// A control character that is encoded as more than one byte and does not show:
/// the C1 controls and the bidirectional controls (the Unicode `Bidi_Control`
/// set: the Arabic letter mark, the left-to-right and right-to-left marks, the
/// embeddings and overrides, and the isolates).
fn is_hidden_control(c: char) -> bool {
    matches!(
        c,
        '\u{80}'..='\u{9f}'
            | '\u{61c}'
            | '\u{200e}'
            | '\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2066}'..='\u{2069}'
    )
}

/// Line number (1-based) and character of the first hidden control character
/// on each line of `text`.
fn hidden_controls(text: &str) -> Vec<(usize, char)> {
    text.lines()
        .enumerate()
        .filter_map(|(i, line)| {
            line.chars()
                .find(|&c| is_hidden_control(c))
                .map(|c| (i + 1, c))
        })
        .collect()
}

/// The line (1-based) of the first byte sequence in `bytes` that is not valid
/// UTF-8, or `None` when the whole input is.
fn invalid_utf8_line(bytes: &[u8]) -> Option<usize> {
    let error = std::str::from_utf8(bytes).err()?;
    let before = &bytes[..error.valid_up_to()];
    Some(1 + before.iter().filter(|&&b| b == b'\n').count())
}

fn is_separator(c: u8) -> bool {
    c == b'\\' || c == b'/'
}

fn is_ident(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

/// Profile folders shared by every user. They name no person. Longer names come
/// before the shorter names they start with.
const SHARED_PROFILES: &[&str] = &["default user", "default", "public", "all users", "shared"];

/// Whether `rest` (the bytes after `Users` and its separator) starts with the
/// name of a person: a placeholder (`<name>`, `%USERNAME%`, `{user}`, `*`) does
/// not count, and neither does a shared profile folder.
fn names_a_person(rest: &[u8]) -> bool {
    if !rest.first().is_some_and(|&c| is_ident(c)) {
        return false;
    }
    !SHARED_PROFILES.iter().any(|shared| {
        let n = shared.len();
        rest.len() >= n
            && rest[..n].eq_ignore_ascii_case(shared.as_bytes())
            && !rest
                .get(n)
                .is_some_and(|&c| is_ident(c) || c == b'-' || c == b'.')
    })
}

/// The bytes after one or more leading path separators (`None` with none).
/// Several separators are allowed because paths in string literals and JSON
/// have their backslashes doubled.
fn skip_separators(s: &[u8]) -> Option<&[u8]> {
    let n = s.iter().take_while(|&&c| is_separator(c)).count();
    if n == 0 {
        None
    } else {
        Some(&s[n..])
    }
}

/// The folders that hold the user profiles: `Users`, and `Documents and
/// Settings` on Windows XP and older (lowercase; matched without regard to
/// case).
const PROFILE_ROOTS: &[&str] = &["users", "documents and settings"];

/// Whether `rest` starts with separator(s), a profile root in any case,
/// separator(s) and the name of a person.
fn users_then_name(rest: &[u8]) -> bool {
    let Some(rest) = skip_separators(rest) else {
        return false;
    };
    let Some(root) = PROFILE_ROOTS.iter().find(|root| {
        rest.len() >= root.len() && rest[..root.len()].eq_ignore_ascii_case(root.as_bytes())
    }) else {
        return false;
    };
    let Some(rest) = skip_separators(&rest[root.len()..]) else {
        return false;
    };
    names_a_person(rest)
}

/// A drive letter, a colon, and a profile root and a user name below it.
fn windows_user_path(b: &[u8]) -> bool {
    let mut i = 0;
    while i + 1 < b.len() {
        let drive = b[i].is_ascii_alphabetic()
            && b[i + 1] == b':'
            && (i == 0 || !b[i - 1].is_ascii_alphanumeric());
        if drive && users_then_name(&b[i + 2..]) {
            return true;
        }
        i += 1;
    }
    false
}

/// Whether the `/Users/` at `b[i..]` begins an absolute path rather than
/// ending a longer one (a `Users` folder inside a project path is not a profile
/// path). A drive letter component is accepted only as the first component
/// (`/c/Users`) or right after `/mnt` or `/cygdrive`.
fn path_starts_at(b: &[u8], i: usize) -> bool {
    let path_char =
        |c: u8| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'.' | b'-' | b'~' | b'$' | b'%');
    // Whether the `/` at `pos` opens a path: it is first, or follows a
    // character that cannot be part of a path.
    let opens_path = |pos: usize| pos == 0 || !path_char(b[pos - 1]);
    if opens_path(i) {
        return true;
    }
    if i >= 2 && b[i - 1].is_ascii_alphabetic() && b[i - 2] == b'/' {
        let slash = i - 2;
        if opens_path(slash) {
            return true;
        }
        return ["/mnt", "/cygdrive"].iter().any(|mount| {
            slash >= mount.len()
                && &b[slash - mount.len()..slash] == mount.as_bytes()
                && opens_path(slash - mount.len())
        });
    }
    false
}

/// `/Users/` and a user name at the start of an absolute path (macOS home).
fn unix_user_path(b: &[u8]) -> bool {
    const USERS: &[u8] = b"/Users/";
    let mut i = 0;
    while i + USERS.len() < b.len() {
        if b[i..].starts_with(USERS)
            && names_a_person(&b[i + USERS.len()..])
            && path_starts_at(b, i)
        {
            return true;
        }
        i += 1;
    }
    false
}

/// Whether `line` holds an absolute Windows or macOS user-profile path.
/// `%USERPROFILE%` and placeholders such as `<name>` are not paths of a person.
fn has_user_profile_path(line: &str) -> bool {
    let b = line.as_bytes();
    windows_user_path(b) || unix_user_path(b)
}

/// An address written in hex: 6 to 8 digits and nothing else.
fn is_address(s: &str) -> bool {
    (6..=8).contains(&s.len()) && s.bytes().all(|c| c.is_ascii_hexdigit())
}

/// A stack offset or case value written in lowercase hex: 1 to 8 digits.
fn is_hex_offset(s: &str) -> bool {
    (1..=8).contains(&s.len())
        && s.bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

/// Kind prefixes of names made from an address. `thunk_` and the kinds may wrap
/// each other (a pointer to a function, a thunk of a function). Switch labels,
/// string labels and the fixed-width forms are matched on their own below.
const ADDRESS_KINDS: &[&str] = &[
    "FUN_",
    "DAT_",
    "LAB_",
    "PTR_",
    "SUB_",
    "EXT_",
    "OFF_",
    "UNK_",
    "BYTE_",
    "WORD_",
    "DWORD_",
    "QWORD_",
    "FLOAT_",
    "DOUBLE_",
    "joined_r0x",
    "code_r0x",
];

/// Whether `s` is exactly eight hex digits (a stack or register offset is
/// always printed in full).
fn is_full_width_hex(s: &str) -> bool {
    s.len() == 8 && s.bytes().all(|c| c.is_ascii_hexdigit())
}

/// A string label: `s_` (or `u_` for a Unicode string), the start of the text
/// with every character that cannot be in a name replaced by `_`, and the
/// address, as in `s_<text>_<address>`.
fn string_label(s: &str) -> bool {
    let Some(body) = s.strip_prefix("s_").or_else(|| s.strip_prefix("u_")) else {
        return false;
    };
    body.rsplit_once('_')
        .is_some_and(|(_, address)| is_address(address))
}

/// The label of a pointer: `PTR_`, the name of what it points to (itself made
/// from an address, or a string, or an ordinary symbol), `_` and the address of
/// the pointer.
fn pointer_label(s: &str) -> bool {
    s.strip_prefix("PTR_").is_some_and(|body| {
        body.rsplit_once('_')
            .is_some_and(|(target, address)| !target.is_empty() && is_address(address))
    })
}

/// A name the decompiler or the listing makes from an address: a kind prefix and
/// an address, a switch label (the address of the switch, then a case label or
/// another suffix), a switch case label (a prefix and a case value), a string or
/// pointer label, a stack or register offset, or an import by ordinal. The whole
/// token has to match. Leading underscores do not matter: the decompiler prints
/// one when an access does not match the type of the data it reads.
fn address_name(token: &str) -> bool {
    let mut rest = token.trim_start_matches('_');
    loop {
        if let Some(inner) = rest.strip_prefix("thunk_") {
            rest = inner;
            continue;
        }
        if let Some(value) = rest.strip_prefix("caseD_") {
            return is_hex_offset(value);
        }
        if let Some(number) = rest.strip_prefix("Ordinal_") {
            return all_digits(number) && number.len() <= 5;
        }
        if let Some(hex) = ["stack0x", "register0x"]
            .iter()
            .find_map(|kind| rest.strip_prefix(kind))
        {
            return is_full_width_hex(hex);
        }
        if let Some(tail) = ["switchD_", "switchdataD_"]
            .iter()
            .find_map(|kind| rest.strip_prefix(kind))
        {
            return is_address(tail.split('_').next().unwrap_or(""));
        }
        if pointer_label(rest) || string_label(rest) {
            return true;
        }
        let Some(kind) = ADDRESS_KINDS.iter().find(|k| rest.starts_with(**k)) else {
            return false;
        };
        let tail = &rest[kind.len()..];
        if is_address(tail) {
            return true;
        }
        rest = tail;
    }
}

fn all_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit())
}

/// A numbered parameter.
fn numbered_parameter(token: &str) -> bool {
    token.strip_prefix("param_").is_some_and(all_digits)
}

/// General registers, segment registers and flags, `ST0`..`ST7`, `MM0`..`MM7`,
/// `XMM0`..`XMM15` and `YMM0`..`YMM15`: what the decompiler shows as inputs.
fn register_name(s: &str) -> bool {
    const FIXED: &[&str] = &[
        "EAX", "EBX", "ECX", "EDX", "ESI", "EDI", "EBP", "ESP", "EIP", "AX", "BX", "CX", "DX",
        "AL", "BL", "CL", "DL", "AH", "BH", "CH", "DH", "CS", "DS", "ES", "FS", "GS", "SS", "CF",
        "PF", "AF", "ZF", "SF", "OF", "DF", "EFLAGS",
    ];
    let numbered = |prefix: &str, max: u32| {
        s.strip_prefix(prefix).is_some_and(|n| {
            all_digits(n) && n.len() <= 2 && n.parse::<u32>().is_ok_and(|v| v <= max)
        })
    };
    FIXED.contains(&s)
        || numbered("ST", 7)
        || numbered("MM", 7)
        || numbered("XMM", 15)
        || numbered("YMM", 15)
}

/// What can follow a register in a name: `OFFSET` (a segment base), a capital
/// and a lowercase letter marking a piece of a wider register (`Da`, `Qa`), or a
/// two-digit counter that tells duplicates apart.
fn register_suffix(s: &str) -> bool {
    let b = s.as_bytes();
    s == "OFFSET"
        || (b.len() == 2 && b[0].is_ascii_uppercase() && b[1].is_ascii_lowercase())
        || (b.len() == 2 && all_digits(s))
}

/// A register used without being set in the function: the input, the
/// unaffected and the extra-output forms, with their piece and counter suffixes.
fn register_input(token: &str) -> bool {
    for prefix in ["in_", "unaff_", "extraout_"] {
        let Some(rest) = token.strip_prefix(prefix) else {
            continue;
        };
        let mut parts = rest.split('_');
        let first = parts.next().unwrap_or("");
        let named = register_name(first)
            || (prefix == "unaff_" && first == "retaddr")
            || (prefix == "extraout_" && first == "var");
        if named && parts.all(register_suffix) {
            return true;
        }
    }
    false
}

/// A stack slot read before it is written: the prefix and eight hex digits.
fn stack_input(token: &str) -> bool {
    token
        .strip_prefix("in_stack_")
        .is_some_and(|hex| hex.len() == 8 && hex.bytes().all(|c| c.is_ascii_hexdigit()))
}

/// Letters the decompiler puts before `Var` and `Stack_` to say what a
/// temporary holds (unsigned, int, long, short, char, byte, float, double,
/// pointer, array, void).
const TYPE_LETTERS: &[u8] = b"abcdfilpsuv";

/// The type prefix of a temporary: any run of `p` (pointers) and then up to
/// three of the type letters (`u`, `pc`, `ppu`, `ppuc`), or one capital letter
/// for a named type (`pF` points to a `FILE`, `U` holds a `UINT`).
fn temporary_prefix(prefix: &str) -> bool {
    let b = prefix.as_bytes();
    if b.is_empty() || b.len() > 8 {
        return false;
    }
    let pointers = b.iter().take_while(|&&c| c == b'p').count();
    match &b[pointers..] {
        [named] if named.is_ascii_uppercase() => true,
        rest => rest.len() <= 3 && rest.iter().all(|c| TYPE_LETTERS.contains(c)),
    }
}

/// A numbered temporary: a type prefix, `Var` and a decimal number.
fn numbered_temporary(token: &str) -> bool {
    let Some(at) = token.find("Var") else {
        return false;
    };
    temporary_prefix(&token[..at]) && all_digits(&token[at + 3..])
}

/// A temporary on the stack: a type prefix, `Stack`, an optional capital letter
/// (a `Y` marks some slots), `_` and the offset in hex.
fn stack_temporary(token: &str) -> bool {
    let Some(at) = token.find("Stack") else {
        return false;
    };
    let after = &token[at + 5..];
    let after = after
        .strip_prefix(|c: char| c.is_ascii_uppercase())
        .unwrap_or(after);
    after.strip_prefix('_').is_some_and(is_hex_offset) && temporary_prefix(&token[..at])
}

/// Short words that are made only of hex letters, so a stack local of that name
/// is a normal identifier.
const LOCAL_WORDS: &[&str] = &[
    "ace", "ada", "add", "bad", "bed", "bee", "cab", "cad", "dab", "dad", "dec", "eff", "fab",
    "fad", "fed", "fee",
];

/// A stack local named after its offset: lowercase hex. Up to three letters
/// count even without a decimal digit (the first local of a frame is often
/// `c`), except a few real words; longer names made only of letters (`dead`)
/// are ordinary identifiers. Locals in the parameter area end in `res` and the
/// offset.
fn stack_local(token: &str) -> bool {
    let Some(hex) = token.strip_prefix("local_") else {
        return false;
    };
    if let Some(offset) = hex.strip_prefix("res") {
        return is_hex_offset(offset);
    }
    if !is_hex_offset(hex) {
        return false;
    }
    hex.bytes().any(|c| c.is_ascii_digit()) || (hex.len() <= 3 && !LOCAL_WORDS.contains(&hex))
}

/// A decompiler's helper call (joining, truncating or extending values, carry
/// and borrow tests): the name, digits, and an opening parenthesis straight
/// after.
fn helper_call(token: &str, next: Option<u8>) -> bool {
    const HELPERS: [&str; 7] = [
        "CONCAT", "SUB", "ZEXT", "SEXT", "CARRY", "SCARRY", "SBORROW",
    ];
    next == Some(b'(')
        && HELPERS
            .iter()
            .any(|h| token.strip_prefix(h).is_some_and(all_digits))
}

/// The decompiler's sized types: `undefined` and a size for an unknown value,
/// and `unk`, a kind and a size for a value of an odd size.
fn undefined_type(token: &str) -> bool {
    ["undefined", "unkbyte", "unkint", "unkuint", "unkfloat"]
        .iter()
        .any(|kind| {
            token
                .strip_prefix(kind)
                .is_some_and(|n| all_digits(n) && n.len() <= 2)
        })
}

fn is_auto_name(token: &str, next: Option<u8>) -> bool {
    address_name(token)
        || numbered_parameter(token)
        || register_input(token)
        || stack_input(token)
        || numbered_temporary(token)
        || stack_temporary(token)
        || stack_local(token)
        || helper_call(token, next)
        || undefined_type(token)
}

/// The decompiler-generated names on `line`, as written, each once, in order.
/// Tokens are runs of letters, digits and underscores, so a name inside a longer
/// identifier (`my_` before it, a letter after it) is not matched.
fn ghidra_auto_names(line: &str) -> Vec<&str> {
    let b = line.as_bytes();
    let mut found: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if !is_ident(b[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < b.len() && is_ident(b[i]) {
            i += 1;
        }
        // The run is ASCII, so both ends are character boundaries.
        let token = &line[start..i];
        if is_auto_name(token, b.get(i).copied()) && !found.contains(&token) {
            found.push(token);
        }
    }
    found
}

/// The first decompiler-generated name on `line`.
fn ghidra_auto_name(line: &str) -> Option<&str> {
    ghidra_auto_names(line).into_iter().next()
}

/// Fence languages that mean C or C++ source or headers (lowercase).
const C_LANGUAGES: &[&str] = &["c", "h", "cc", "cpp", "cxx", "c++", "hpp", "hxx"];

/// The language a fence names, lowercase, from the text after the fence
/// characters. Besides a bare word (`c`), this reads a word followed by more
/// (`c title="x"`, `c,ignore`, `c{1-3}`, `c:file.h`), a class in attribute
/// braces (`{.c}`, `{#id .c .numberLines}`) and `language-c`.
fn fence_language(info: &str) -> String {
    let word = match info.strip_prefix('{') {
        Some(attributes) => attributes
            .split(|c: char| c.is_whitespace() || c == ',')
            .find_map(|part| part.strip_prefix('.'))
            .unwrap_or(""),
        None => info,
    };
    let word = word.strip_prefix("language-").unwrap_or(word);
    word.chars()
        .take_while(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '#'))
        .collect::<String>()
        .to_ascii_lowercase()
}

/// Opening lines (1-based) of fenced code blocks tagged as C or C++, with the
/// language. A fence inside another block (a longer or different fence showing
/// Markdown, say) is content, not an opener.
fn c_fences(text: &str) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    let mut open: Option<(u8, usize)> = None;
    for (n, line) in text.lines().enumerate() {
        let t = line.trim_start();
        let Some(&c) = t.as_bytes().first() else {
            continue;
        };
        if c != b'`' && c != b'~' {
            continue;
        }
        let len = t.bytes().take_while(|&x| x == c).count();
        if len < 3 {
            continue;
        }
        let info = t[len..].trim();
        match open {
            Some((open_char, open_len)) => {
                if c == open_char && len >= open_len && info.is_empty() {
                    open = None;
                }
            }
            None => {
                // A backtick fence's info string cannot hold a backtick, so
                // three backticks around inline code are not a fence.
                if c == b'`' && info.contains('`') {
                    continue;
                }
                open = Some((c, len));
                let language = fence_language(info);
                if C_LANGUAGES.contains(&language.as_str()) {
                    found.push((n + 1, language));
                }
            }
        }
    }
    found
}

// ---------------------------------------------------------------------------
// `.gitignore` files, as far as the fallback walk needs them.
// ---------------------------------------------------------------------------

/// Whether `text` matches `pattern`, where `*` stands for any run of
/// characters and `?` for one, neither crossing a `/`.
fn glob(pattern: &[u8], text: &[u8]) -> bool {
    let (mut p, mut t) = (0, 0);
    let mut star: Option<(usize, usize)> = None;
    while t < text.len() {
        if p < pattern.len() && pattern[p] == b'*' {
            star = Some((p, t));
            p += 1;
        } else if p < pattern.len()
            && ((pattern[p] == b'?' && text[t] != b'/') || pattern[p] == text[t])
        {
            p += 1;
            t += 1;
        } else if let Some((star_p, star_t)) = star {
            if text[star_t] == b'/' {
                return false;
            }
            star = Some((star_p, star_t + 1));
            p = star_p + 1;
            t = star_t + 1;
        } else {
            return false;
        }
    }
    while p < pattern.len() && pattern[p] == b'*' {
        p += 1;
    }
    p == pattern.len()
}

struct IgnoreRule {
    /// The directory of the `.gitignore` that holds the entry, as a path from
    /// the root with a trailing `/` (empty for the root). The entry applies
    /// below it.
    base: String,
    pattern: String,
    negated: bool,
    dir_only: bool,
    /// The pattern has a `/` in it, so it is matched against the path from its
    /// `.gitignore`'s directory rather than against the last name.
    anchored: bool,
}

/// The entries of the `.gitignore` files this test follows: names and `*`/`?`
/// globs, optionally anchored with a `/` or limited to directories with a
/// trailing `/`, and `!` negations. Entries with other special characters are
/// left out, which only means more files are checked.
struct GitIgnore {
    rules: Vec<IgnoreRule>,
}

impl GitIgnore {
    fn new() -> Self {
        GitIgnore { rules: Vec::new() }
    }

    fn parse(text: &str) -> Self {
        let mut ignore = GitIgnore::new();
        ignore.add("", text);
        ignore
    }

    /// Adds the entries of the `.gitignore` found in directory `dir` (a path
    /// from the root without a trailing `/`; empty for the root itself).
    fn add(&mut self, dir: &str, text: &str) {
        let base = if dir.is_empty() {
            String::new()
        } else {
            format!("{dir}/")
        };
        for line in text.lines() {
            let line = line.trim_end();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (negated, body) = match line.strip_prefix('!') {
                Some(rest) => (true, rest),
                None => (false, line),
            };
            let (dir_only, body) = match body.strip_suffix('/') {
                Some(rest) => (true, rest),
                None => (false, body),
            };
            if body.is_empty() || body.contains(['[', '\\']) || body.contains("**") {
                continue;
            }
            let anchored = body.contains('/');
            self.rules.push(IgnoreRule {
                base: base.clone(),
                pattern: body.trim_start_matches('/').to_string(),
                negated,
                dir_only,
                anchored,
            });
        }
    }

    /// Whether git ignores `rel` (a `/`-separated path from the root). The last
    /// matching entry decides, as in git.
    fn is_ignored(&self, rel: &str, is_dir: bool) -> bool {
        let mut ignored = false;
        for rule in &self.rules {
            if rule.dir_only && !is_dir {
                continue;
            }
            let Some(below) = rel.strip_prefix(rule.base.as_str()) else {
                continue;
            };
            let name = below.rsplit('/').next().unwrap_or(below);
            let subject = if rule.anchored { below } else { name };
            if glob(rule.pattern.as_bytes(), subject.as_bytes()) {
                ignored = !rule.negated;
            }
        }
        ignored
    }
}

// ---------------------------------------------------------------------------
// Listing files, checking them and reporting.
// ---------------------------------------------------------------------------

struct RepoFile {
    /// Path from the root with `/` separators.
    rel: String,
    path: PathBuf,
}

struct Listing {
    files: Vec<RepoFile>,
    /// Entries that could not be read, as findings.
    problems: Vec<String>,
    /// How the files were found, for the failure message.
    source: &'static str,
}

/// A git command run in `root`. The variables git sets for hooks are removed so
/// a test run from inside a hook still looks at `root` and nothing else.
fn git(root: &Path) -> Command {
    let mut command = Command::new("git");
    command.current_dir(root);
    for var in [
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_INDEX_FILE",
        "GIT_COMMON_DIR",
        "GIT_PREFIX",
    ] {
        command.env_remove(var);
    }
    command
}

/// The names `git ls-files -z <args>` prints in `root`, as raw bytes, or `None`
/// when git cannot be run or fails.
fn git_ls_files(root: &Path, args: &[&str]) -> Option<Vec<Vec<u8>>> {
    let output = git(root)
        .args(["ls-files", "-z"])
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(
        output
            .stdout
            .split(|&b| b == 0)
            .filter(|name| !name.is_empty())
            .map(<[u8]>::to_vec)
            .collect(),
    )
}

/// Whether `rel` is inside a `.claude` directory: local agent settings and
/// scratch worktrees, which belong to one person's checkout.
fn in_agent_state(rel: &str) -> bool {
    rel.split('/').any(|component| component == ".claude")
}

/// The files git tracks or would add (everything not ignored), or `None` when
/// `root` is not the top of a git checkout or git cannot be run. An untracked
/// file inside a `.claude` directory is left out (see the module header).
fn files_from_git(root: &Path) -> Option<Listing> {
    // `root` must be the top of the checkout. Without this, a directory that is
    // not a repository (a copy made with `git archive`, a fixture) would be
    // taken for part of one in a parent directory, and listed from it.
    let prefix = git(root)
        .args(["rev-parse", "--show-prefix"])
        .output()
        .ok()?;
    if !prefix.status.success() || !prefix.stdout.iter().all(u8::is_ascii_whitespace) {
        return None;
    }
    let tracked = git_ls_files(root, &["--cached"])?;
    let untracked = git_ls_files(root, &["--others", "--exclude-standard"])?;
    let mut rels = Vec::new();
    let mut problems = Vec::new();
    let named = tracked
        .iter()
        .map(|name| (name, true))
        .chain(untracked.iter().map(|name| (name, false)));
    for (raw, is_tracked) in named {
        match std::str::from_utf8(raw) {
            Ok(rel) if is_tracked || !in_agent_state(rel) => rels.push(rel.to_string()),
            Ok(_) => {}
            Err(_) => problems.push(format!(
                "{}:1: file name is not valid UTF-8",
                String::from_utf8_lossy(raw)
            )),
        }
    }
    // A path is listed more than once while a merge is unresolved.
    rels.sort();
    rels.dedup();
    let files = rels
        .into_iter()
        .map(|rel| RepoFile {
            path: root.join(&rel),
            rel,
        })
        .collect();
    Some(Listing {
        files,
        problems,
        source: "git ls-files",
    })
}

fn walk(dir: &Path, rel_dir: &str, ignore: &mut GitIgnore, out: &mut Listing) {
    let here = if rel_dir.is_empty() { "." } else { rel_dir };
    if let Ok(text) = fs::read_to_string(dir.join(".gitignore")) {
        ignore.add(rel_dir, &text);
    }
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            out.problems
                .push(format!("{here}:1: cannot read directory: {e}"));
            return;
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                out.problems
                    .push(format!("{here}:1: cannot read directory entry: {e}"));
                continue;
            }
        };
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            out.problems.push(format!(
                "{here}:1: file name is not valid UTF-8: {:?}",
                entry.file_name()
            ));
            continue;
        };
        let rel = if rel_dir.is_empty() {
            name.clone()
        } else {
            format!("{rel_dir}/{name}")
        };
        let kind = match entry.file_type() {
            Ok(kind) => kind,
            Err(e) => {
                out.problems
                    .push(format!("{rel}:1: cannot read entry type: {e}"));
                continue;
            }
        };
        // The type of a symbolic link is the link's own: it is listed as a
        // file and not followed.
        let is_dir = kind.is_dir();
        if ignore.is_ignored(&rel, is_dir) {
            continue;
        }
        if is_dir {
            if SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            // A directory with a banned extension is reported itself and not
            // entered.
            if banned_extension(&name).is_some() {
                out.files.push(RepoFile {
                    rel,
                    path: entry.path(),
                });
                continue;
            }
            walk(&entry.path(), &rel, ignore, out);
        } else {
            out.files.push(RepoFile {
                rel,
                path: entry.path(),
            });
        }
    }
}

/// The files under `root` found by walking the tree, for a checkout git cannot
/// list.
fn files_from_walk(root: &Path) -> Listing {
    let mut listing = Listing {
        files: Vec::new(),
        problems: Vec::new(),
        source: "a directory walk (no git)",
    };
    walk(root, "", &mut GitIgnore::new(), &mut listing);
    listing.files.sort_by(|a, b| a.rel.cmp(&b.rel));
    listing
}

fn list_files(root: &Path) -> Listing {
    files_from_git(root).unwrap_or_else(|| files_from_walk(root))
}

/// Findings for one text file, as (line, message).
fn check_text(rel: &str, bytes: &[u8]) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    for (line, byte) in control_characters(bytes) {
        found.push((
            line,
            format!("control character 0x{byte:02x} (only tab, LF and CR are allowed)"),
        ));
    }
    if let Some(line) = invalid_utf8_line(bytes) {
        found.push((line, "not valid UTF-8 (text files are UTF-8)".to_string()));
    }
    let text = String::from_utf8_lossy(bytes);
    for (line, c) in hidden_controls(&text) {
        found.push((
            line,
            format!(
                "hidden control character U+{:04X} (C1 control or bidirectional control)",
                c as u32
            ),
        ));
    }
    let top = rel.split('/').next().unwrap_or("");
    let provenance = rel.contains('/') && PROVENANCE_DIRS.contains(&top);
    for (i, line) in text.lines().enumerate() {
        if has_user_profile_path(line) {
            found.push((
                i + 1,
                "absolute user-profile path (write %USERPROFILE%)".to_string(),
            ));
        }
        if provenance {
            let names = ghidra_auto_names(line);
            if !names.is_empty() {
                found.push((
                    i + 1,
                    format!(
                        "decompiler auto-name `{}` (write the address, or plain words)",
                        names.join("`, `")
                    ),
                ));
            }
        }
    }
    if provenance && rel.ends_with(".md") {
        for (line, tag) in c_fences(&text) {
            found.push((
                line,
                format!("fenced `{tag}` block (no decompiler output in docs)"),
            ));
        }
    }
    if is_engine_source(rel) {
        found.extend(typed_findings(rel, &text));
    }
    found.sort();
    found
}

/// At most `limit` bytes from the start of the file.
fn read_prefix(path: &Path, limit: u64) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    fs::File::open(path)?.take(limit).read_to_end(&mut bytes)?;
    Ok(bytes)
}

/// Every problem in the listed files, as `path:line: rule`, in path order.
fn check_listing(listing: Listing) -> Vec<String> {
    let mut findings = listing.problems;
    let mut banned_dirs: BTreeSet<String> = BTreeSet::new();
    for file in &listing.files {
        let meta = fs::symlink_metadata(&file.path);
        // The name rule needs no file, so it also applies to a tracked file that
        // was deleted from the work tree.
        if let Some((prefix, ext)) = banned_component(&file.rel) {
            let is_dir = meta.as_ref().is_ok_and(|meta| meta.is_dir());
            if prefix.len() == file.rel.len() && !is_dir {
                findings.push(format!(
                    "{}:1: game, recording or binary file type `.{ext}` is never committed",
                    file.rel
                ));
            } else if banned_dirs.insert(prefix.to_string()) {
                findings.push(format!(
                    "{prefix}/:1: directory type `.{ext}` (decompiler project or game data) \
                     is never committed"
                ));
            }
            continue;
        }
        let meta = match meta {
            Ok(meta) => meta,
            // Tracked, but removed from the work tree: nothing to read.
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => {
                findings.push(format!("{}:1: cannot read file: {e}", file.rel));
                continue;
            }
        };
        // A directory here is a submodule or a nested repository; a link is
        // not followed.
        if meta.is_dir() || meta.file_type().is_symlink() {
            continue;
        }
        let name = file.rel.rsplit('/').next().unwrap_or(&file.rel);
        let ext = extension(name);
        if ext.as_deref().is_some_and(|e| BINARY_MEDIA.contains(&e)) {
            continue;
        }
        let known_text = ext.as_deref().is_some_and(|e| TEXT_EXTENSIONS.contains(&e));
        let bytes = match read_prefix(&file.path, MAX_SCAN_BYTES + 1) {
            Ok(bytes) => bytes,
            Err(e) => {
                findings.push(format!("{}:1: cannot read file: {e}", file.rel));
                continue;
            }
        };
        let looks_binary = !known_text && bytes[..bytes.len().min(65536)].contains(&0);
        if looks_binary {
            findings.push(format!(
                "{}:1: binary file (never committed; test inputs are built from scratch)",
                file.rel
            ));
        } else if bytes.len() as u64 > MAX_SCAN_BYTES {
            findings.push(format!(
                "{}:1: larger than {} MiB, not scanned",
                file.rel,
                MAX_SCAN_BYTES >> 20
            ));
        } else {
            for (line, message) in check_text(&file.rel, &bytes) {
                findings.push(format!("{}:{line}: {message}", file.rel));
            }
        }
    }
    findings
}

// ---------------------------------------------------------------------------
// Typed code (docs/ENGINE_CRATE.md, "Typed code"). Lexical checks: nothing here knows types.
// ---------------------------------------------------------------------------

/// Folders that hold only typed code.
const TYPED_DIRS: &[&str] = &["crates/engine-core/src/typed/", "crates/engine-typed/src/"];

/// The files at the top of a typed folder, which deny truncating casts for all of it.
const TYPED_ROOTS: &[&str] = &[
    "crates/engine-core/src/typed/mod.rs",
    "crates/engine-typed/src/lib.rs",
];

/// Clippy's type-aware form of the float rule: the text rule here reads words, this reads types.
const DENY_CASTS: &str = "#![deny(clippy::cast_possible_truncation)]";

/// The lint and the group that hold it: an `allow` of either lifts it.
const CAST_LINTS: &[&str] = &["clippy::cast_possible_truncation", "clippy::pedantic"];

/// Text that only comes from decompiled C++ or a path into a decompilation. Nothing in typed code
/// needs it: a typed function cites its address and says what it does in plain words.
const DECOMP_TEXT: &[&str] = &[
    "#include",
    "@fnv",
    "reinterpret_cast",
    "static_cast",
    "const_cast",
    "dynamic_cast",
    "this->",
    "nullptr",
    "__thiscall",
    "__fastcall",
    "__cdecl",
    "src/staging",
    "fnv_decomp/",
    "fnv_decomp\\",
    "src\\staging",
    ".cpp",
    ".hpp",
];

/// Integer types an `as` may go to.
const INT_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
];

/// Calls that answer a float, seen inside an `as` operand.
const FLOAT_CALLS: &[&str] = &[
    "x87_",
    ".sqrt(",
    ".floor(",
    ".ceil(",
    ".trunc(",
    ".round(",
    ".fract(",
    ".sin(",
    ".cos(",
    ".f32(",
    ".f64(",
    "to_radians",
    "to_degrees",
];

/// Whether `rel` is Rust the game runs: an engine crate's source, or the host's or the viewer's, which
/// can call a translation directly too.
fn is_engine_source(rel: &str) -> bool {
    (rel.starts_with("crates/engine/")
        || rel.starts_with("crates/engine-")
        || rel.starts_with("crates/host/")
        || rel.starts_with("viewer/"))
        && rel.contains("/src/")
        && rel.ends_with(".rs")
}

/// `text` with comments and the contents of strings and character literals blanked (same length
/// and line breaks), so a scan sees only code.
fn code_only(text: &str) -> String {
    let b = text.as_bytes();
    let mut out = b.to_vec();
    let blank = |out: &mut Vec<u8>, from: usize, to: usize| {
        for c in &mut out[from..to] {
            if *c != b'\n' {
                *c = b' ';
            }
        }
    };
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'/') => {
                let end = b[i..]
                    .iter()
                    .position(|c| *c == b'\n')
                    .map_or(b.len(), |n| i + n);
                blank(&mut out, i, end);
                i = end;
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let (mut depth, mut j) = (1, i + 2);
                while j < b.len() && depth > 0 {
                    if b[j] == b'/' && b.get(j + 1) == Some(&b'*') {
                        depth += 1;
                        j += 2;
                    } else if b[j] == b'*' && b.get(j + 1) == Some(&b'/') {
                        depth -= 1;
                        j += 2;
                    } else {
                        j += 1;
                    }
                }
                let j = j.min(b.len());
                blank(&mut out, i, j);
                i = j;
            }
            b'"' => {
                let mut j = i + 1;
                match raw_string_hashes(b, i) {
                    Some(n) => {
                        let close: Vec<u8> = std::iter::once(b'"')
                            .chain(std::iter::repeat(b'#').take(n))
                            .collect();
                        while j < b.len() && !b[j..].starts_with(&close) {
                            j += 1;
                        }
                    }
                    None => {
                        while j < b.len() && b[j] != b'"' {
                            j += if b[j] == b'\\' { 2 } else { 1 };
                        }
                    }
                }
                let j = j.min(b.len());
                blank(&mut out, i + 1, j);
                i = j + 1;
            }
            b'\'' => {
                // A character literal, not a lifetime: an escape, or one character then a quote.
                let len = if b.get(i + 1) == Some(&b'\\') {
                    b[i + 2..]
                        .iter()
                        .position(|c| *c == b'\'')
                        .map(|n| n + 2 + 1)
                } else {
                    text.get(i + 1..)
                        .and_then(|s| s.chars().next())
                        .filter(|c| b.get(i + 1 + c.len_utf8()) == Some(&b'\''))
                        .map(|c| c.len_utf8() + 1)
                };
                if let Some(len) = len {
                    blank(&mut out, i + 1, i + len);
                    i += len + 1;
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// The number of `#` of the raw string whose opening quote is at `quote`, or `None` when it is an
/// ordinary string.
fn raw_string_hashes(b: &[u8], quote: usize) -> Option<usize> {
    let mut k = quote;
    let mut n = 0;
    while k > 0 && b[k - 1] == b'#' {
        k -= 1;
        n += 1;
    }
    let r = k.checked_sub(1)?;
    if b[r] != b'r' {
        return None;
    }
    match r.checked_sub(1).map(|p| b[p]) {
        Some(b'b') => {
            let before = r.checked_sub(2).map(|p| b[p]);
            (!before.is_some_and(is_ident)).then_some(n)
        }
        Some(c) if is_ident(c) => None,
        _ => Some(n),
    }
}

/// The line where the block that opens at or after line `start` of `code` (from [`code_only`])
/// closes.
fn block_end(code: &[&str], start: usize) -> usize {
    let mut depth = 0i32;
    let mut opened = false;
    for (i, line) in code.iter().enumerate().skip(start) {
        for c in line.bytes() {
            if c == b'{' {
                depth += 1;
                opened = true;
            } else if c == b'}' {
                depth -= 1;
            }
        }
        if opened && depth <= 0 {
            return i;
        }
    }
    code.len().saturating_sub(1)
}

/// Marks the lines of `#[cfg(test)]` modules.
fn test_module_lines(code: &[&str]) -> Vec<bool> {
    let mut marks = vec![false; code.len()];
    let mut i = 0;
    while i < code.len() {
        if code[i].trim() == "#[cfg(test)]" {
            let next = (i + 1..code.len()).find(|k| !code[*k].trim().is_empty());
            if let Some(next) = next {
                let line = code[next].trim_start();
                let is_mod = line.starts_with("mod ")
                    || line.starts_with("pub mod ")
                    || line.starts_with("pub(crate) mod ");
                // `mod tests;` has no body here.
                if is_mod && !line.trim_end().ends_with(';') {
                    let end = block_end(code, next);
                    marks[i..=end].fill(true);
                    i = end;
                }
            }
        }
        i += 1;
    }
    marks
}

/// Whether a line of code starts a function.
fn starts_fn(line: &str) -> bool {
    let line = line.trim_start();
    let line = line
        .strip_prefix("pub(crate) ")
        .or_else(|| line.strip_prefix("pub(super) "))
        .or_else(|| line.strip_prefix("pub "))
        .unwrap_or(line);
    line.starts_with("fn ") || line.starts_with("const fn ") || line.starts_with("unsafe fn ")
}

/// The line of the function a marker at line `marker` is for: past doc comments (blank in `code`)
/// and attributes. `None` when something else comes first.
fn marked_fn_line(code: &[&str], marker: usize) -> Option<usize> {
    for (i, line) in code.iter().enumerate().skip(marker + 1) {
        let line = line.trim_start();
        if line.is_empty() || line.starts_with("#[") {
            continue;
        }
        return starts_fn(line).then_some(i);
    }
    None
}

/// Marks the lines of typed code in a file: all of it in [`TYPED_DIRS`], else each function under a
/// typed marker.
fn typed_lines(rel: &str, raw: &[&str], code: &[&str]) -> Vec<bool> {
    if TYPED_DIRS.iter().any(|d| rel.starts_with(d)) {
        return vec![true; raw.len()];
    }
    let mut marks = vec![false; raw.len()];
    for (i, line) in raw.iter().enumerate() {
        if line.trim_start().starts_with(typed_marker::PREFIX) {
            if let Some(f) = marked_fn_line(code, i) {
                let end = block_end(code, f);
                marks[i..=end.min(raw.len() - 1)].fill(true);
            }
        }
    }
    marks
}

/// Identifiers `code` declares as floats: a parameter or `let` with the type written, or a `let` of
/// a float literal or a float call.
fn float_names(code: &[&str]) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for line in code {
        for ty in [": f32", ": f64"] {
            let mut from = 0;
            while let Some(k) = line[from..].find(ty) {
                let at = from + k;
                let (_, name) = ident_before(line, at);
                if !name.is_empty() {
                    names.insert(name.to_string());
                }
                from = at + ty.len();
            }
        }
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed
            .strip_prefix("let mut ")
            .or_else(|| trimmed.strip_prefix("let "))
        {
            if let Some((name, value)) = rest.split_once('=') {
                if !value.starts_with('=') && expr_is_float(value, &names) {
                    let name = name.trim();
                    if name.bytes().all(is_ident) {
                        names.insert(name.to_string());
                    }
                }
            }
        }
    }
    names
}

/// The identifier that ends just before byte `end` of `line`, as (its start, itself).
fn ident_before(line: &str, end: usize) -> (usize, &str) {
    let b = line.as_bytes();
    let mut start = end;
    while start > 0 && is_ident(b[start - 1]) {
        start -= 1;
    }
    (start, &line[start..end])
}

/// Whether an expression shows that it is a float.
fn expr_is_float(expr: &str, floats: &BTreeSet<String>) -> bool {
    if FLOAT_CALLS.iter().any(|c| expr.contains(c)) {
        return true;
    }
    let b = expr.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if !is_ident(b[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < b.len() && is_ident(b[i]) {
            i += 1;
        }
        let token = &expr[start..i];
        if token == "f32" || token == "f64" || floats.contains(token) {
            return true;
        }
        // A number with a fractional part or a float suffix: not a tuple field (`t.0`).
        let is_number = token.as_bytes()[0].is_ascii_digit();
        let after_dot = start > 0 && b[start - 1] == b'.';
        if is_number && !after_dot {
            if token.ends_with("f32") || token.ends_with("f64") {
                return true;
            }
            if b.get(i) == Some(&b'.') && b.get(i + 1).is_some_and(u8::is_ascii_digit) {
                return true;
            }
        }
    }
    false
}

/// The operand of the `as` at byte `at` of `line`: back over a call or parenthesis group and a
/// path or field chain.
fn cast_operand(line: &str, at: usize) -> &str {
    let b = line.as_bytes();
    let mut i = at;
    while i > 0 && b[i - 1] == b' ' {
        i -= 1;
    }
    let end = i;
    loop {
        if i > 0 && b[i - 1] == b')' {
            let mut depth = 0;
            while i > 0 {
                i -= 1;
                if b[i] == b')' {
                    depth += 1;
                } else if b[i] == b'(' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
        }
        let (start, _) = ident_before(line, i);
        i = start;
        if i >= 1 && b[i - 1] == b'.' {
            i -= 1;
        } else if i >= 2 && &b[i - 2..i] == b"::" {
            i -= 2;
        } else {
            break;
        }
    }
    &line[i..end]
}

/// Whether an operand holding floats ends as something else: their bits, a test of one, a compare.
fn answers_no_float(operand: &str) -> bool {
    let operand = operand.trim();
    let ends_as = |suffix: &str| operand.ends_with(suffix);
    let named_test = operand
        .rfind("is_")
        .is_some_and(|at| operand[at..].trim_end_matches(')').find('(').is_some());
    ends_as(".to_bits()")
        || named_test
        || [" < ", " > ", " <= ", " >= ", " == ", " != "]
            .iter()
            .any(|compare| operand.contains(compare))
}

/// The `as` casts from a float to an integer on a line of code: the operands, as written.
fn float_casts<'a>(line: &'a str, floats: &BTreeSet<String>) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(k) = line[from..].find(" as ") {
        let at = from + k;
        from = at + 4;
        let target: String = line[from..]
            .bytes()
            .take_while(|c| is_ident(*c))
            .map(char::from)
            .collect();
        if INT_TYPES.contains(&target.as_str()) {
            let operand = cast_operand(line, at);
            if expr_is_float(operand, floats) && !answers_no_float(operand) {
                out.push(operand);
            }
        }
    }
    out
}

/// A name made from an address that a typed function may not adopt: an identifier holding an exe
/// address (`00` or `01` first, as a whole run of hex digits), however it goes on (`_struct`,
/// `_class`), other than exactly the project's own `fn_<addr>` and `run_<addr>` for functions and run
/// units with no name yet.
fn placeholder_names(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let b = line.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if !is_ident(b[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < b.len() && is_ident(b[i]) {
            i += 1;
        }
        let token = &line[start..i];
        if token.as_bytes()[0].is_ascii_digit() || token.len() <= 8 {
            continue;
        }
        let own = token
            .strip_prefix("fn_")
            .or_else(|| token.strip_prefix("run_"))
            .is_some_and(|tail| tail.len() == 8 && tail.bytes().all(|c| c.is_ascii_hexdigit()));
        if !own && holds_address(token.as_bytes()) {
            out.push(token);
        }
    }
    out
}

/// Whether an identifier holds eight hex digits, `00` or `01` first, not inside a longer run.
fn holds_address(token: &[u8]) -> bool {
    token.windows(8).enumerate().any(|(at, w)| {
        let hex =
            w.iter().all(u8::is_ascii_hexdigit) && w[0] == b'0' && matches!(w[1], b'0' | b'1');
        let before = at == 0 || !token[at - 1].is_ascii_hexdigit();
        let after = token.get(at + 8).map_or(true, |c| !c.is_ascii_hexdigit());
        hex && before && after
    })
}

/// A path into a decompilation in a comment: a header or inline file (`actor.h`, `actor.inl`), or
/// words joined by backslashes. In a comment only: in code, `rect.h` is a field and `\n` an escape.
fn decomp_path(comment: &str) -> Option<&str> {
    let b = comment.as_bytes();
    for (i, _) in comment.match_indices('.') {
        let name_before = i > 0 && is_ident(b[i - 1]);
        let tail = &comment[i + 1..];
        for ext in ["h", "inl"] {
            let after = tail.as_bytes().get(ext.len());
            if name_before && tail.starts_with(ext) && !after.is_some_and(|c| is_ident(*c)) {
                return Some(&comment[..i + 1 + ext.len()]);
            }
        }
    }
    for (i, _) in comment.match_indices('\\') {
        let (before, _) = ident_before(comment, i);
        let after = b.get(i + 1).is_some_and(|c| is_ident(*c));
        if i - before >= 2 && after {
            return Some(comment);
        }
    }
    None
}

/// Marks the layouts and constants in a file's code, with the comments directly above them.
fn data_item_lines(raw: &[&str], code: &[&str]) -> Vec<bool> {
    let mut marks = vec![false; raw.len()];
    let mut i = 0;
    while i < code.len() {
        let line = code[i].trim_start();
        let line = line
            .strip_prefix("pub(crate) ")
            .or_else(|| line.strip_prefix("pub(super) "))
            .or_else(|| line.strip_prefix("pub "))
            .unwrap_or(line);
        let item = line.starts_with("layout!")
            || line.starts_with("const ")
            || line.starts_with("static ");
        if !item || line.starts_with("const fn ") {
            i += 1;
            continue;
        }
        let end = if line.starts_with("layout!") {
            block_end(code, i)
        } else {
            (i..code.len())
                .find(|k| code[*k].contains(';'))
                .unwrap_or(i)
        };
        marks[i..=end].fill(true);
        let mut above = i;
        while above > 0 && code[above - 1].trim().is_empty() && !raw[above - 1].trim().is_empty() {
            above -= 1;
            marks[above] = true;
        }
        i = end + 1;
    }
    marks
}

/// The value of the `0x...` literal `text` starts with.
fn hex_literal(text: &str) -> Option<u32> {
    let rest = text.strip_prefix("0x")?;
    let digits: String = rest
        .bytes()
        .take_while(|c| c.is_ascii_hexdigit() || *c == b'_')
        .filter(|c| *c != b'_')
        .map(char::from)
        .collect();
    u32::from_str_radix(&digits, 16).ok()
}

/// The typed-code rules for one engine source that need only that file.
fn typed_findings(rel: &str, text: &str) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    let in_typed_dir = TYPED_DIRS.iter().any(|d| rel.starts_with(d));
    if !in_typed_dir && !typed_marker::mentioned(text) {
        return found;
    }
    let stripped = code_only(text);
    let raw: Vec<&str> = text.lines().collect();
    let code: Vec<&str> = stripped.lines().collect();
    let tests = test_module_lines(&code);
    let typed = typed_lines(rel, &raw, &code);
    // In a file with typed functions, the layouts and constants they read are under the taint rules
    // too, with the comments on them.
    let data = if in_typed_dir {
        Vec::new()
    } else {
        data_item_lines(&raw, &code)
    };
    let mut floats = BTreeSet::new();
    for (i, line) in raw.iter().enumerate() {
        // Names mean what their own function says they mean.
        if starts_fn(code[i]) {
            floats = float_names(&code[i..=block_end(&code, i)]);
        }
        let comment = line.find("//").map(|at| &line[at..]);
        if comment.is_some_and(typed_marker::mentioned) {
            if typed_marker::address(line).is_none() {
                found.push((
                    i + 1,
                    "malformed typed marker (its address, then at most `, read against \
                     fnv_decomp`)"
                        .to_string(),
                ));
            }
            if marked_fn_line(&code, i).is_none() {
                found.push((
                    i + 1,
                    "typed marker not directly above a function".to_string(),
                ));
            }
        }
        if !typed[i] && !data.get(i).copied().unwrap_or(false) {
            continue;
        }
        for banned in DECOMP_TEXT {
            if line.contains(banned) {
                found.push((
                    i + 1,
                    format!("`{banned}` in typed code (decompiled text or a path into one)"),
                ));
            }
        }
        if let Some(path) = comment.and_then(decomp_path) {
            found.push((
                i + 1,
                format!("`{path}` in typed code (a path into a decompilation)"),
            ));
        }
        for name in placeholder_names(line) {
            found.push((
                i + 1,
                format!("placeholder name `{name}` in typed code (write a real name)"),
            ));
        }
        if !typed[i] {
            continue;
        }
        let c = code[i];
        if ["core::arch", "std::arch", "target_arch"]
            .iter()
            .any(|a| c.contains(a))
        {
            found.push((
                i + 1,
                "an x86 assumption in typed code (it stays portable, docs/ENGINE_CRATE.md)"
                    .to_string(),
            ));
        }
        if !tests[i] {
            for operand in float_casts(c, &floats) {
                found.push((
                    i + 1,
                    format!(
                        "`{operand} as` an integer in typed code: `as` saturates where the exe \
                         gets the integer indefinite (use x87::fistp_chop)"
                    ),
                ));
            }
        }
    }
    // Each typed folder's root denies truncating casts, type-aware; nothing lifts that for a whole file
    // or module but the differential test itself.
    if in_typed_dir && !rel.ends_with("typed/diff.rs") {
        for line in casts_lifted(&code, &tests) {
            found.push((
                line + 1,
                "this lifts the type-aware ban on truncating casts for a whole file or module (put \
                 a local `allow` on one named helper)"
                    .to_string(),
            ));
        }
    }
    if TYPED_ROOTS.contains(&rel) && !text.contains(DENY_CASTS) {
        found.push((
            1,
            format!("a typed folder's root needs `{DENY_CASTS}` (clippy's type-aware float rule)"),
        ));
    }
    found
}

/// The lines outside test modules that lift the truncating-cast lint for a whole file or module: an
/// inner `allow` in any form, or an outer one on a `mod`.
fn casts_lifted(code: &[&str], tests: &[bool]) -> Vec<usize> {
    let mut out = Vec::new();
    for (i, line) in code.iter().enumerate() {
        if tests[i] {
            continue;
        }
        let line = line.trim_start();
        let inner = line.starts_with("#![allow");
        let outer = line.starts_with("#[allow");
        if !inner && !outer {
            continue;
        }
        let close = (i..code.len())
            .find(|k| code[*k].contains(']'))
            .unwrap_or(i);
        let attribute = code[i..=close].join(" ");
        if !CAST_LINTS.iter().any(|lint| attribute.contains(lint)) {
            continue;
        }
        let on_a_module = (close + 1..code.len())
            .find(|k| !code[*k].trim().is_empty())
            .is_some_and(|k| {
                let next = code[k].trim_start();
                next.starts_with("mod ") || next.starts_with("pub mod ")
            });
        if inner || on_a_module {
            out.push(i);
        }
    }
    out
}

/// One row of `crates/engine-core/data/overridable.tsv`.
#[derive(Debug, PartialEq)]
struct Overridable {
    line: usize,
    address: u32,
    direct_calls: usize,
}

/// The rows of the table, or what is wrong with it.
fn parse_overridable(text: &str) -> Result<Vec<Overridable>, String> {
    let mut rows: Vec<Overridable> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        let [address, direct_calls, _source] = f[..] else {
            return Err(format!("line {}: not three columns: {line:?}", i + 1));
        };
        if address.len() != 8 {
            return Err(format!("line {}: bad address {address:?}", i + 1));
        }
        let address = u32::from_str_radix(address, 16)
            .map_err(|e| format!("line {}: bad address: {e}", i + 1))?;
        let direct_calls = direct_calls
            .parse()
            .map_err(|e| format!("line {}: bad count: {e}", i + 1))?;
        if rows.last().is_some_and(|r| r.address >= address) {
            return Err(format!("line {}: addresses must rise", i + 1));
        }
        rows.push(Overridable {
            line: i + 1,
            address,
            direct_calls,
        });
    }
    Ok(rows)
}

/// A function registered at an address: what `entry!(0x.., path::name(..)` says, and where.
#[derive(Clone, Debug)]
struct Registration {
    address: u32,
    name: String,
    /// The module it is in as far as text can say: the path's last module, else the file's.
    module: String,
    rel: String,
}

/// The module a file is, by its name.
fn module_of_file(rel: &str) -> &str {
    let mut parts = rel.rsplit('/');
    let file = parts.next().unwrap_or(rel);
    let stem = file.strip_suffix(".rs").unwrap_or(file);
    if matches!(stem, "mod" | "lib" | "main") {
        parts.next().unwrap_or(stem)
    } else {
        stem
    }
}

/// The module a file's `super` is, by its folders.
fn parent_module(rel: &str) -> &str {
    let mut parts = rel.rsplit('/');
    let file = parts.next().unwrap_or(rel);
    if matches!(file, "mod.rs" | "lib.rs" | "main.rs") {
        parts.next();
    }
    parts.next().unwrap_or("")
}

/// (address, path as written, line index) of each registration in `code`: `entry!(0x.., path::name(..)`
/// and `(0x.., name as AbiFn)`.
fn registered_paths(code: &str) -> Vec<(u32, String, usize)> {
    let line_of = |at: usize| code[..at].bytes().filter(|c| *c == b'\n').count();
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(k) = code[from..].find("entry!(") {
        let at = from + k;
        from = at + 7;
        let rest = code[from..].trim_start();
        let Some(address) = hex_literal(rest) else {
            continue;
        };
        let Some((_, after)) = rest.split_once(',') else {
            continue;
        };
        let path: String = after
            .trim_start()
            .bytes()
            .take_while(|c| is_ident(*c) || *c == b':')
            .map(char::from)
            .collect();
        if !path.is_empty() {
            out.push((address, path, line_of(at)));
        }
    }
    let mut from = 0;
    while let Some(k) = code[from..].find(" as AbiFn") {
        let at = from + k;
        from = at + 9;
        let (start, name) = ident_before(code, at);
        if let Some(head) = code[..start].trim_end().strip_suffix(',') {
            let open = head.rfind('(').map_or(0, |o| o + 1);
            if let Some(address) = hex_literal(head[open..].trim()) {
                out.push((address, name.to_string(), line_of(at)));
            }
        }
    }
    out
}

/// A file's registrations.
fn registrations(rel: &str, code: &str) -> Vec<Registration> {
    registered_paths(code)
        .into_iter()
        .map(|(address, path, _)| {
            let mut segments = path.rsplit("::");
            let name = segments.next().unwrap_or("").to_string();
            let module = match segments.next() {
                None | Some("self") => module_of_file(rel).to_string(),
                Some("super") => parent_module(rel).to_string(),
                Some(module) => module.to_string(),
            };
            Registration {
                address,
                name,
                module,
                rel: rel.to_string(),
            }
        })
        .collect()
}

/// What a file's `use` lines bring in: each name as the file uses it, with the path it stands for,
/// and the modules it takes everything from.
#[derive(Default, Debug)]
struct Imports {
    names: BTreeMap<String, Vec<String>>,
    globs: Vec<String>,
}

fn file_imports(code: &str) -> Imports {
    let mut imports = Imports::default();
    let mut from = 0;
    while from < code.len() {
        let end_of_line = code[from..].find('\n').map_or(code.len(), |n| from + n);
        let line = code[from..end_of_line].trim_start();
        let statement = ["use ", "pub use ", "pub(crate) use ", "pub(super) use "]
            .iter()
            .find_map(|p| line.strip_prefix(p));
        if statement.is_some() {
            let start = code[from..].find("use ").map_or(from, |n| from + n + 4);
            let end = code[start..].find(';').map_or(code.len(), |n| start + n);
            flatten_use(&mut Vec::new(), &code[start..end], &mut imports);
            from = end + 1;
        } else {
            from = end_of_line + 1;
        }
    }
    imports
}

/// One `use` tree under `prefix`: `a::{b, c as d, e::*}`.
fn flatten_use(prefix: &mut Vec<String>, tree: &str, imports: &mut Imports) {
    let mut depth = 0;
    let mut start = 0;
    let mut items = Vec::new();
    for (i, c) in tree.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            ',' if depth == 0 => {
                items.push(&tree[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    items.push(&tree[start..]);
    for item in items {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }
        if let Some(open) = item.find('{') {
            let head: Vec<String> = item[..open]
                .split("::")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect();
            let kept = prefix.len();
            prefix.extend(head);
            let close = item.rfind('}').unwrap_or(item.len());
            flatten_use(prefix, &item[open + 1..close], imports);
            prefix.truncate(kept);
            continue;
        }
        let (path, alias) = match item.split_once(" as ") {
            Some((path, alias)) => (path.trim(), Some(alias.trim())),
            None => (item, None),
        };
        let mut segments: Vec<String> = prefix.clone();
        segments.extend(path.split("::").map(|s| s.trim().to_string()));
        match segments.last().map(String::as_str) {
            Some("*") => {
                if segments.len() >= 2 {
                    imports.globs.push(segments[segments.len() - 2].clone());
                }
            }
            Some("self") => {
                segments.pop();
                if let Some(last) = segments.last().cloned() {
                    imports
                        .names
                        .insert(alias.unwrap_or(&last).to_string(), segments);
                }
            }
            Some(last) => {
                let name = alias.unwrap_or(last).to_string();
                if name != "_" {
                    imports.names.insert(name, segments);
                }
            }
            None => {}
        }
    }
}

/// A function call as far as text can read it.
#[derive(Debug, PartialEq)]
struct CalledName {
    /// Its own name, with any `use .. as ..` undone.
    name: String,
    /// The last module on its path, or that its import names.
    module: Option<String>,
    /// Written bare and not imported by name: its own module's, or from a glob.
    bare: bool,
}

/// What `path` (segments, the function last) names, with `imports`.
fn called_name(path: &[&str], imports: &Imports) -> Option<CalledName> {
    let last = *path.last()?;
    if path.len() == 1 {
        if let Some(full) = imports.names.get(last) {
            return Some(CalledName {
                name: full.last()?.clone(),
                module: full.len().checked_sub(2).map(|k| full[k].clone()),
                bare: false,
            });
        }
        return Some(CalledName {
            name: last.to_string(),
            module: None,
            bare: true,
        });
    }
    let module = path[path.len() - 2];
    let module = if path.len() == 2 {
        imports
            .names
            .get(module)
            .and_then(|full| full.last())
            .map_or(module, String::as_str)
    } else {
        module
    };
    Some(CalledName {
        name: last.to_string(),
        module: Some(module.to_string()),
        bare: false,
    })
}

/// Whether a call in `rel` of `called` can be `registration`, as far as text can say.
fn may_be(called: &CalledName, rel: &str, imports: &Imports, registration: &Registration) -> bool {
    if called.name != registration.name {
        return false;
    }
    match called.module.as_deref() {
        None => {
            registration.rel == rel
                || registration.module == module_of_file(rel)
                || imports.globs.iter().any(|g| {
                    *g == registration.module
                        || (g == "super" && registration.module == parent_module(rel))
                        || g == "self"
                })
        }
        Some("self") => registration.module == module_of_file(rel),
        Some("super") => registration.module == parent_module(rel),
        Some("crate") => true,
        Some(module) => registration.module == module,
    }
}

/// A call found by [`direct_calls`].
struct DirectCall {
    /// The line, from 0.
    line: usize,
    called: CalledName,
}

/// The calls in `code` of a name in `names`: an identifier followed by `(`, not a method, not an
/// associated function (`Thing::name(`, `f32::sqrt(`), not the `fn` itself, not the standard
/// library's, not one on a path into a module that doesn't register that name (`helpers::read(`), not
/// the function an `entry!` or a `typed_call!` names.
fn direct_calls(
    code: &str,
    names: &BTreeMap<String, Vec<Registration>>,
    imports: &Imports,
) -> Vec<DirectCall> {
    let mut out = Vec::new();
    let b = code.as_bytes();
    let (mut i, mut line) = (0, 0);
    while i < b.len() {
        if b[i] == b'\n' {
            line += 1;
            i += 1;
            continue;
        }
        if !is_ident(b[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < b.len() && is_ident(b[i]) {
            i += 1;
        }
        let mut next = i;
        // A turbofish, then the argument list.
        if code[next..].starts_with("::<") {
            let mut depth = 0;
            while next < b.len() {
                match b[next] {
                    b'<' => depth += 1,
                    b'>' => {
                        depth -= 1;
                        if depth == 0 {
                            next += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                next += 1;
            }
        }
        while next < b.len() && matches!(b[next], b' ' | b'\n' | b'\r') {
            next += 1;
        }
        if b.get(next) != Some(&b'(') {
            continue;
        }
        let mut chain = start;
        let mut path: Vec<&str> = vec![&code[start..i]];
        while chain >= 2 && &b[chain - 2..chain] == b"::" {
            let (s, segment) = ident_before(code, chain - 2);
            path.insert(0, segment);
            chain = s;
        }
        let Some(called) = called_name(&path, imports) else {
            continue;
        };
        let Some(registered) = names.get(&called.name) else {
            continue;
        };
        let before = code[..chain].trim_end();
        if before.ends_with('.') || before.ends_with("fn") || names_a_registration(before) {
            continue;
        }
        if matches!(path[0], "std" | "core" | "alloc") {
            continue;
        }
        if let Some(module) = called.module.as_deref() {
            let associated = module.starts_with(|c: char| c.is_ascii_uppercase());
            let known = matches!(module, "self" | "super" | "crate")
                || registered.iter().any(|r| r.module == module);
            if associated || !known {
                continue;
            }
        }
        out.push(DirectCall { line, called });
    }
    out
}

/// Whether text ending just before a function's path is an `entry!` or a `typed_call!` up to that
/// path: the macro, then an argument list ending in a literal address and a comma.
fn names_a_registration(before: &str) -> bool {
    let Some(head) = before.strip_suffix(',') else {
        return false;
    };
    let last_comma = head.rfind(',');
    let open = head.rfind(['(', '[', '{']);
    let start = match (last_comma, open) {
        (Some(c), Some(o)) => c.max(o) + 1,
        (None, Some(o)) => o + 1,
        _ => return false,
    };
    if hex_literal(head[start..].trim()).is_none() {
        return false;
    }
    let Some(open) = open else { return false };
    let macro_name = head[..open].trim_end();
    macro_name.ends_with("entry!") || macro_name.ends_with("typed_call!")
}

/// A `typed_call!` in code, in any bracket form.
#[derive(Debug, PartialEq)]
struct TypedCallSite {
    /// The line, from 0.
    line: usize,
    /// Its address: `Err` with the text when that isn't a literal.
    address: Result<u32, String>,
    /// The path of the function it calls directly.
    path: Vec<String>,
}

fn typed_call_sites(code: &str) -> Vec<TypedCallSite> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(k) = code[from..].find("typed_call!") {
        let at = from + k;
        from = at + 11;
        let rest = code[from..].trim_start();
        if !rest.starts_with(['(', '[', '{']) {
            continue;
        }
        // The arguments at the top level: the family, the address, the call.
        let mut parts = Vec::new();
        let (mut depth, mut part) = (0, 1);
        for (i, c) in rest.char_indices() {
            match c {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => {
                    depth -= 1;
                    if depth == 0 {
                        parts.push(&rest[part..i]);
                        break;
                    }
                }
                ',' if depth == 1 && parts.len() < 2 => {
                    parts.push(&rest[part..i]);
                    part = i + 1;
                }
                _ => {}
            }
        }
        let [family, address, call] = parts[..] else {
            continue;
        };
        // The macro's own `@bind` arms pass `$addr` on.
        if family.trim_start().starts_with('@') || address.trim_start().starts_with('$') {
            continue;
        }
        let address = address.trim();
        let path: String = call
            .trim_start()
            .bytes()
            .take_while(|c| is_ident(*c) || *c == b':')
            .map(char::from)
            .collect();
        out.push(TypedCallSite {
            line: code[..at].bytes().filter(|c| *c == b'\n').count(),
            address: hex_literal(address).ok_or_else(|| address.to_string()),
            path: path
                .split("::")
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect(),
        });
    }
    out
}

/// `code` with its test modules blanked, lines kept.
fn without_test_modules(code: &str) -> String {
    if !code.contains("#[cfg(test)]") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let tests = test_module_lines(&lines);
    let mut out = String::with_capacity(code.len());
    for (line, test) in lines.iter().zip(tests) {
        if !test {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

/// Whether `code` has its own `fn name` or a local of that name, so a bare `name(` means that.
fn defines_locally(code: &str, name: &str) -> bool {
    [
        format!("fn {name}("),
        format!("fn {name}<"),
        format!("let {name} "),
        format!("let {name}:"),
        format!("let mut {name} "),
    ]
    .iter()
    .any(|p| code.contains(p.as_str()))
}

/// The functions under typed markers in a file, as (address, name, line index).
fn marked_functions(text: &str, code: &str) -> Vec<(u32, String, usize)> {
    if !typed_marker::mentioned(text) {
        return Vec::new();
    }
    let raw: Vec<&str> = text.lines().collect();
    let lines: Vec<&str> = code.lines().collect();
    let mut out = Vec::new();
    for (i, line) in raw.iter().enumerate() {
        let Some(address) = typed_marker::address(line) else {
            continue;
        };
        let Some(f) = marked_fn_line(&lines, i) else {
            continue;
        };
        let after = lines[f].split("fn ").nth(1).unwrap_or("");
        let name: String = after
            .bytes()
            .take_while(|c| is_ident(*c))
            .map(char::from)
            .collect();
        if !name.is_empty() {
            out.push((address, name, i));
        }
    }
    out
}

/// What the table and the sources `files` (relative path, text) say across files: every finding as
/// (path, line, message).
fn typed_cross_file_findings(
    table: &str,
    files: &[(String, String)],
) -> Vec<(String, usize, String)> {
    const TABLE: &str = "crates/engine-core/data/overridable.tsv";
    let mut found = Vec::new();
    let rows = match parse_overridable(table) {
        Ok(rows) => rows,
        Err(why) => return vec![(TABLE.to_string(), 1, why)],
    };
    let overridable: BTreeSet<u32> = rows.iter().map(|r| r.address).collect();
    let codes: Vec<String> = files
        .iter()
        .map(|(_, text)| without_test_modules(&code_only(text)))
        .collect();

    // Who is registered, by name and by address; which functions carry a marker for which address.
    let mut by_name: BTreeMap<String, Vec<Registration>> = BTreeMap::new();
    let mut by_address: BTreeMap<u32, Vec<Registration>> = BTreeMap::new();
    let mut marked: BTreeSet<(u32, String)> = BTreeSet::new();
    for ((rel, text), code) in files.iter().zip(&codes) {
        for registration in registrations(rel, code) {
            by_name
                .entry(registration.name.clone())
                .or_default()
                .push(registration.clone());
            by_address
                .entry(registration.address)
                .or_default()
                .push(registration);
        }
        for (address, name, _) in marked_functions(text, code) {
            marked.insert((address, name));
        }
    }
    let overridable_names: BTreeMap<String, Vec<Registration>> = by_name
        .iter()
        .filter_map(|(name, list)| {
            let listed: Vec<Registration> = list
                .iter()
                .filter(|r| overridable.contains(&r.address))
                .cloned()
                .collect();
            (!listed.is_empty()).then(|| (name.clone(), listed))
        })
        .collect();

    let mut counts: BTreeMap<u32, usize> = BTreeMap::new();
    let mut where_called: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for ((rel, text), code) in files.iter().zip(&codes) {
        let in_typed_dir = TYPED_DIRS.iter().any(|d| rel.starts_with(d));
        let lines: Vec<&str> = code.lines().collect();
        let imports = file_imports(code);
        if in_typed_dir {
            // A function registered in a typed folder has the marker for that address above it.
            for (address, path, line) in registered_paths(code) {
                let name = path.rsplit("::").next().unwrap_or("").to_string();
                if !marked.contains(&(address, name.clone())) {
                    found.push((
                        rel.clone(),
                        line + 1,
                        format!(
                            "typed function {address:08x} (`{name}`) registered without its marker"
                        ),
                    ));
                }
            }
        }
        for site in typed_call_sites(code) {
            let segments: Vec<&str> = site.path.iter().map(String::as_str).collect();
            let called = called_name(&segments, &imports);
            let message = match (&site.address, called) {
                (Ok(address), _) if overridable.contains(address) => Some(format!(
                    "typed_call! of overridable {address:08x}: call it with e.call (a fix or \
                     built-in may replace it, overridable.tsv)"
                )),
                (Ok(address), Some(called)) => {
                    let registered = by_address.get(address);
                    let resolves = registered
                        .is_some_and(|list| list.iter().any(|r| may_be(&called, rel, &imports, r)));
                    if !resolves {
                        Some(format!(
                            "typed_call! at {address:08x} calls `{}` directly, but that address is \
                             registered as {}: the wrong function would run in play",
                            site.path.join("::"),
                            registered.map_or("nothing".to_string(), |list| {
                                list.iter()
                                    .map(|r| format!("{}::{}", r.module, r.name))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            })
                        ))
                    } else if !marked.contains(&(*address, called.name.clone())) {
                        Some(format!(
                            "typed_call! at {address:08x} calls `{}` directly, which has no typed \
                             marker for that address: it would skip the typed version",
                            called.name
                        ))
                    } else {
                        None
                    }
                }
                (Ok(address), None) => {
                    Some(format!("typed_call! at {address:08x} names no function"))
                }
                (Err(word), _) => Some(format!(
                    "typed_call! of `{word}`: write the address as a literal, so this test can \
                     read it"
                )),
            };
            if let Some(message) = message {
                found.push((rel.clone(), site.line + 1, message));
            }
        }
        let raw: Vec<&str> = text.lines().collect();
        let typed = typed_lines(rel, &raw, &lines);
        let has_typed = typed.iter().any(|t| *t);
        // A call of an overridable function, in any file.
        for call in direct_calls(code, &overridable_names, &imports) {
            let list = &overridable_names[&call.called.name];
            let same_file = list.iter().any(|r| r.rel == *rel);
            if call.called.bare && !same_file && defines_locally(code, &call.called.name) {
                continue;
            }
            let addresses: BTreeSet<u32> = list
                .iter()
                .filter(|r| call.called.bare || may_be(&call.called, rel, &imports, r))
                .map(|r| r.address)
                .collect();
            for address in addresses {
                if typed.get(call.line).copied().unwrap_or(false) {
                    found.push((
                        rel.clone(),
                        call.line + 1,
                        format!(
                            "direct call of overridable {address:08x} (`{}`) in typed code: call \
                             it with e.call (a fix or built-in may replace it, overridable.tsv)",
                            call.called.name
                        ),
                    ));
                } else {
                    *counts.entry(address).or_default() += 1;
                    where_called
                        .entry(address)
                        .or_default()
                        .push(format!("{rel}:{}", call.line + 1));
                }
            }
        }
        // In typed code, a plain call of any registered function skips the direct flag, doubles and
        // diagnostics: it is a typed_call! or an e.call.
        if has_typed {
            for call in direct_calls(code, &by_name, &imports) {
                if !typed.get(call.line).copied().unwrap_or(false)
                    || overridable_names.contains_key(&call.called.name)
                {
                    continue;
                }
                let registered_here = by_name[&call.called.name].iter().any(|r| r.rel == *rel);
                if call.called.bare && !registered_here && defines_locally(code, &call.called.name)
                {
                    continue;
                }
                found.push((
                    rel.clone(),
                    call.line + 1,
                    format!(
                        "plain call of registered `{}` in typed code: use typed_call! or e.call, \
                         or doubles, diagnostics and the switch won't see it",
                        call.called.name
                    ),
                ));
            }
        }
    }
    for row in &rows {
        let n = counts.get(&row.address).copied().unwrap_or(0);
        if n > row.direct_calls {
            found.push((
                TABLE.to_string(),
                row.line,
                format!(
                    "{:08x} has {n} direct call(s), the table allows {} (no new one: a fix or \
                     built-in replacing the address would miss it; call it with e.call): {}",
                    row.address,
                    row.direct_calls,
                    where_called[&row.address].join(", ")
                ),
            ));
        } else if n < row.direct_calls {
            found.push((
                TABLE.to_string(),
                row.line,
                format!(
                    "{:08x} has {n} direct call(s), the table allows {}: lower the table's number",
                    row.address, row.direct_calls
                ),
            ));
        }
    }
    found
}

/// The addresses `Diff::new` names in `code`, test modules included.
fn diff_test_addresses(code: &str) -> BTreeSet<u32> {
    let mut out = BTreeSet::new();
    let mut from = 0;
    while let Some(k) = code[from..].find("Diff::new(") {
        from += k + 10;
        let rest = code[from..].trim_start();
        let rest = rest.strip_prefix("crate::").unwrap_or(rest);
        let rest = rest.strip_prefix("engine_core::").unwrap_or(rest);
        let rest = rest.strip_prefix("entry!(").unwrap_or(rest);
        let rest = rest.strip_prefix('(').unwrap_or(rest);
        out.extend(hex_literal(rest.trim_start()));
    }
    out
}

/// A `Diff::new(entry!(.., f(..)))..translation(entry!(.., f(..)).1)` in `code`: the same function on
/// both sides, each through its own `entry!`, which the harness can't see at run time.
fn compared_with_itself(code: &str) -> Vec<(usize, String)> {
    let name_after = |text: &str| -> Option<String> {
        let rest = text.trim_start();
        let rest = rest.strip_prefix("crate::").unwrap_or(rest);
        let rest = rest.strip_prefix("engine_core::").unwrap_or(rest);
        let rest = rest.strip_prefix("entry!(")?;
        let (_, after) = rest.split_once(',')?;
        let path: String = after
            .trim_start()
            .bytes()
            .take_while(|c| is_ident(*c) || *c == b':')
            .map(char::from)
            .collect();
        path.rsplit("::").next().map(str::to_string)
    };
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(k) = code[from..].find("Diff::new(") {
        let at = from + k;
        from = at + 10;
        let statement = code[from..]
            .find(';')
            .map_or(&code[from..], |n| &code[from..from + n]);
        let Some(typed) = name_after(statement) else {
            continue;
        };
        if let Some(t) = statement.find(".translation(") {
            if name_after(&statement[t + 13..]).as_deref() == Some(typed.as_str()) {
                let line = code[..at].bytes().filter(|c| *c == b'\n').count();
                out.push((line, typed));
            }
        }
    }
    out
}

/// What `sources` (relative path, text: the Rust that makes the game) and `tests` (every Rust file
/// that may hold a differential test) say about each typed function: it is registered, and a
/// differential test names its address. As (path, line, message).
fn typed_function_findings(
    sources: &[(String, String)],
    tests: &[(String, String)],
) -> Vec<(String, usize, String)> {
    let mut found = Vec::new();
    let mut diffed = BTreeSet::new();
    for (rel, text) in tests {
        let code = code_only(text);
        diffed.extend(diff_test_addresses(&code));
        if !rel.ends_with("typed/diff.rs") {
            for (line, name) in compared_with_itself(&code) {
                found.push((
                    rel.clone(),
                    line + 1,
                    format!("a differential test compares `{name}` with itself"),
                ));
            }
        }
    }
    let codes: Vec<String> = sources
        .iter()
        .map(|(_, text)| {
            if typed_marker::mentioned(text) || text.contains("entry!(") {
                without_test_modules(&code_only(text))
            } else {
                String::new()
            }
        })
        .collect();
    let mut registered: BTreeSet<(u32, String)> = BTreeSet::new();
    for ((rel, _), code) in sources.iter().zip(&codes) {
        for registration in registrations(rel, code) {
            registered.insert((registration.address, registration.name));
        }
    }
    for ((rel, text), code) in sources.iter().zip(&codes) {
        for (address, name, line) in marked_functions(text, code) {
            if !registered.contains(&(address, name.clone())) {
                found.push((
                    rel.clone(),
                    line + 1,
                    format!(
                        "`{name}` has a typed marker for {address:08x} and isn't registered there"
                    ),
                ));
            }
            if !diffed.contains(&address) {
                found.push((
                    rel.clone(),
                    line + 1,
                    format!(
                        "typed function {address:08x} (`{name}`) has no differential test: no \
                         `Diff::new` names its address"
                    ),
                ));
            }
        }
    }
    found
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

// ---------------------------------------------------------------------------
// Tests.
// ---------------------------------------------------------------------------

/// The check itself: the repository as checked out has no problems.
#[test]
fn repository_is_clean() {
    let listing = list_files(&repo_root());
    let source = listing.source;
    let count = listing.files.len();
    let findings = check_listing(listing);
    assert!(
        findings.is_empty(),
        "{} repository hygiene problem(s) in {count} files (from {source}):\n{}",
        findings.len(),
        findings.join("\n")
    );
}

mod matchers {
    use super::*;

    /// A decompiler-style name such as `kind_rest`, assembled here so the
    /// token never appears in this file's source.
    fn name(kind: &str, rest: &str) -> String {
        format!("{kind}_{rest}")
    }

    #[test]
    fn extensions_are_lowercase_and_cover_dotfiles() {
        assert_eq!(extension("Foo.RS").as_deref(), Some("rs"));
        assert_eq!(extension("a.b.Esm").as_deref(), Some("esm"));
        assert_eq!(extension(".gitignore").as_deref(), Some("gitignore"));
        assert_eq!(extension("LICENSE-MIT"), None);
        assert_eq!(extension("trailing."), None);
        assert_eq!(extension("license-apache-2.0").as_deref(), Some("0"));
    }

    #[test]
    fn banned_extensions_are_found_on_any_path_component() {
        assert_eq!(
            banned_component("crates/a/Data.ESM"),
            Some(("crates/a/Data.ESM", "esm".to_string()))
        );
        // A Ghidra project database is a directory.
        assert_eq!(
            banned_component("tools/proj.rep/idata/00/x.gbf"),
            Some(("tools/proj.rep", "rep".to_string()))
        );
        assert_eq!(
            banned_component("a/b/c.gbf"),
            Some(("a/b/c.gbf", "gbf".to_string()))
        );
        assert_eq!(
            banned_component("a/b/c.prp").map(|p| p.1).as_deref(),
            Some("prp")
        );
        assert_eq!(banned_component("crates/esm/src/lib.rs"), None);
        assert_eq!(banned_component("esm/x"), None);
        assert_eq!(banned_component("LICENSE-MIT"), None);
    }

    #[test]
    fn control_characters_ignore_tab_lf_cr() {
        assert!(control_characters(b"a\tb\r\nc\n").is_empty());
        assert_eq!(control_characters(b"ok\nbad\0here\n"), vec![(2, 0)]);
        // One report per line, however many bad bytes it holds.
        assert_eq!(
            control_characters(b"\x01\x02\x03\nx\x1b"),
            vec![(1, 1), (2, 0x1b)]
        );
        assert_eq!(control_characters(b"del\x7f"), vec![(1, 0x7f)]);
        // A lone carriage return is allowed and does not start a new line.
        assert!(control_characters(b"a\rb\rc").is_empty());
    }

    #[test]
    fn hidden_controls_are_found_once_per_line() {
        assert_eq!(
            hidden_controls("ok\nbad\u{85}\u{202e}\nfine\n"),
            vec![(2, '\u{85}')]
        );
        // Every C1 control and every bidirectional control.
        let hidden = (0x80..=0x9f)
            .chain([0x61c, 0x200e, 0x200f])
            .chain(0x202a..=0x202e)
            .chain(0x2066..=0x2069);
        for code in hidden {
            let c = char::from_u32(code).unwrap();
            assert_eq!(hidden_controls(&format!("a{c}b")), vec![(1, c)], "{code:x}");
        }
        // The characters next to those ranges, and ordinary non-ASCII text.
        for c in [
            '\u{7e}', '\u{a0}', '\u{2029}', '\u{202f}', '\u{2065}', '\u{206a}', '\u{200d}',
            '\u{e9}', '\u{2014}', '\u{65e5}',
        ] {
            assert!(hidden_controls(&format!("a{c}b")).is_empty(), "{c:?}");
        }
        // Lines are counted the way the byte check counts them.
        assert_eq!(hidden_controls("a\r\nb\r\nc\u{85}"), vec![(3, '\u{85}')]);
    }

    #[test]
    fn text_that_is_not_utf8_is_reported_with_its_line() {
        assert_eq!(invalid_utf8_line("fine \u{e9}\n".as_bytes()), None);
        assert_eq!(invalid_utf8_line(b""), None);
        assert_eq!(invalid_utf8_line(b"ok\nsecond \xff here\n"), Some(2));
        assert_eq!(invalid_utf8_line(b"caf\xe9\n"), Some(1));
        // A sequence cut short at the end.
        assert_eq!(invalid_utf8_line(b"a\nb\xc3"), Some(2));
    }

    #[test]
    fn text_findings_include_hidden_controls_and_bad_utf8() {
        let found = check_text("docs/a.md", "ok\nx\u{202e}y\n".as_bytes());
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].0, 2);
        assert!(found[0].1.contains("U+202E"), "{found:?}");
        let found = check_text("crates/a.rs", b"ok\ncaf\xe9\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].0, 2);
        assert!(found[0].1.contains("not valid UTF-8"), "{found:?}");
        // The other rules still run on text that is not valid UTF-8.
        let profile = ["C:", r"\Users\alice\x"].concat();
        let mut bytes = format!("{profile}\n").into_bytes();
        bytes.extend_from_slice(b"\xff\n");
        let found = check_text("docs/b.md", &bytes);
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(check_text("docs/c.md", "plain \u{e9}\u{2014} text\n".as_bytes()).is_empty());
    }

    #[test]
    fn windows_profile_paths_are_found() {
        let backslash = ["C:", r"\Users\alice\nv-re\findings"].concat();
        let slash = ["d:", "/users/", "Bob/Desktop"].concat();
        let doubled = ["C:", r"\\Users\\carol\\x"].concat();
        let in_text = format!("see `{backslash}` for more");
        assert!(has_user_profile_path(&backslash));
        assert!(has_user_profile_path(&slash));
        assert!(has_user_profile_path(&doubled));
        assert!(has_user_profile_path(&in_text));
    }

    #[test]
    fn windows_xp_profile_paths_are_found() {
        let xp = ["C:", r"\Documents and Settings\erin\Desktop"].concat();
        let lower = ["d:", "/documents and settings/", "Fay/x"].concat();
        let upper = ["E:", r"\DOCUMENTS AND SETTINGS\Gus"].concat();
        let doubled = ["C:", r"\\Documents and Settings\\hal\\x"].concat();
        let quoted = format!("\"{xp}\"");
        for path in [&xp, &lower, &upper, &doubled, &quoted] {
            assert!(has_user_profile_path(path), "{path}");
        }
        // Placeholders, shared folders and the bare root name nobody.
        for path in [
            ["C:", r"\Documents and Settings\<name>\x"].concat(),
            ["C:", r"\Documents and Settings\%USERNAME%\x"].concat(),
            ["C:", r"\Documents and Settings\All Users\x"].concat(),
            ["C:", r"\Documents and Settings\Default User\x"].concat(),
            ["C:", r"\Documents and Settings"].concat(),
            ["C:", r"\Documents and Settingsx\y"].concat(),
            r"in Documents and Settings\alice".to_string(),
        ] {
            assert!(!has_user_profile_path(&path), "{path}");
        }
    }

    #[test]
    fn windows_placeholders_and_environment_names_are_allowed() {
        assert!(!has_user_profile_path(r"%USERPROFILE%\nv-re\findings"));
        assert!(!has_user_profile_path(
            &["C:", r"\Users\<name>\nv-re"].concat()
        ));
        assert!(!has_user_profile_path(
            &["C:", r"\Users\%USERNAME%\x"].concat()
        ));
        assert!(!has_user_profile_path(&["C:", r"\Users"].concat()));
        assert!(!has_user_profile_path(
            &["C:", r"\Program Files\x"].concat()
        ));
        // `Users` as a directory name that is not under a drive.
        assert!(!has_user_profile_path(r"the Users\alice folder"));
        // The letter before the colon has to stand alone.
        assert!(!has_user_profile_path(&["abc:", r"\Users\x"].concat()));
    }

    #[test]
    fn shared_profile_folders_name_nobody() {
        for shared in ["Public", "public", "Default", "Default User", "All Users"] {
            let path = ["C:", r"\Users\"].concat() + shared + r"\Documents";
            assert!(!has_user_profile_path(&path), "{path}");
            let at_end = ["C:", r"\Users\"].concat() + shared;
            assert!(!has_user_profile_path(&at_end), "{at_end}");
            let quoted = format!("\"{at_end}\"");
            assert!(!has_user_profile_path(&quoted), "{quoted}");
        }
        assert!(!has_user_profile_path(&["/", "Users/Shared/x"].concat()));
        // Only the whole folder name is shared: these are people.
        for person in ["Publication", "Defaults", "Public2", "Default-x", "Allison"] {
            let path = ["C:", r"\Users\"].concat() + person + r"\x";
            assert!(has_user_profile_path(&path), "{path}");
        }
        assert!(has_user_profile_path(&["/", "Users/Sharedrive/x"].concat()));
    }

    #[test]
    fn macos_profile_paths_are_found() {
        let home = ["/", "Users/alice/Library/x"].concat();
        let quoted = format!("path = \"{home}\"");
        let msys = ["/c/", "Users/dave/x"].concat();
        let msys_double = ["//c/", "Users/dave/x"].concat();
        let wsl = ["/mnt/c/", "Users/dave/x"].concat();
        let cygwin = ["/cygdrive/c/", "Users/dave/x"].concat();
        let in_text = format!("Read({msys_double})");
        for path in [&home, &quoted, &msys, &msys_double, &wsl, &cygwin, &in_text] {
            assert!(has_user_profile_path(path), "{path}");
        }
    }

    #[test]
    fn macos_lookalikes_are_allowed() {
        assert!(!has_user_profile_path(&["/", "Users/<name>/x"].concat()));
        assert!(!has_user_profile_path(&["/", "Users/ and more"].concat()));
        assert!(!has_user_profile_path(&["src/", "Users/alice/"].concat()));
        assert!(!has_user_profile_path(
            &["https://example.com/", "Users/alice"].concat()
        ));
        // Case matters here: git hosts and web APIs use lowercase `users`.
        assert!(!has_user_profile_path(
            &["https://api.example.com/", "users/alice"].concat()
        ));
    }

    #[test]
    fn a_drive_letter_component_must_start_the_path() {
        // One letter in the middle of a path is a directory name, not a drive.
        for text in [
            ["docs/a/", "Users/bob/x"].concat(),
            ["https://h.io/a/", "Users/bob"].concat(),
            ["a/", "Users/bob"].concat(),
            ["x/mnt/c/", "Users/dave/x"].concat(),
            ["/srv/c/", "Users/dave/x"].concat(),
        ] {
            assert!(!has_user_profile_path(&text), "{text}");
        }
        // As the first component, after a mount point, or after quoting.
        for text in [
            ["/a/", "Users/bob"].concat(),
            ["(/a/", "Users/bob)"].concat(),
            ["\"/mnt/d/", "Users/bob\""].concat(),
        ] {
            assert!(has_user_profile_path(&text), "{text}");
        }
    }

    #[test]
    fn address_names_need_six_to_eight_hex_digits() {
        for kind in ["FUN", "DAT", "LAB", "PTR"] {
            assert_eq!(
                ghidra_auto_name(&format!("x {}(1) y", name(kind, "00b0d7b0"))),
                Some(name(kind, "00b0d7b0").as_str()),
                "{kind}"
            );
        }
        assert!(ghidra_auto_name(&name("FUN", "00B0D7B0")).is_some());
        assert!(ghidra_auto_name(&name("FUN", "0b0d7b")).is_some());
        // Too short, too long, not hex, or followed by more name.
        assert!(ghidra_auto_name(&name("FUN", "0b0d7")).is_none());
        assert!(ghidra_auto_name(&name("FUN", "00b0d7b01")).is_none());
        assert!(ghidra_auto_name(&name("FUN", "00b0d7bz")).is_none());
        assert!(ghidra_auto_name(&format!("{}x", name("FUN", "00b0d7b0"))).is_none());
        // A prefix with no address is just text.
        assert!(ghidra_auto_name(&["`FUN", "_`, `DAT", "_` and so on"].concat()).is_none());
    }

    #[test]
    fn other_address_kinds_are_found() {
        for kind in [
            "SUB",
            "EXT",
            "OFF",
            "UNK",
            "BYTE",
            "WORD",
            "DWORD",
            "QWORD",
            "FLOAT",
            "DOUBLE",
            "switchD",
            "switchdataD",
        ] {
            let token = name(kind, "004a6d52");
            assert_eq!(
                ghidra_auto_name(&format!("({token})")),
                Some(token.as_str()),
                "{kind}"
            );
            assert!(ghidra_auto_name(&name(kind, "4a6d5")).is_none(), "{kind}");
        }
        // A joined register read carries its address after `r0x`.
        let joined = name("joined", "r0x004a6d52");
        assert_eq!(ghidra_auto_name(&joined), Some(joined.as_str()));
        // Case labels carry the case value, which can be short.
        for value in ["0", "1b", "ffffffff"] {
            let token = name("caseD", value);
            assert_eq!(ghidra_auto_name(&token), Some(token.as_str()), "{value}");
        }
        assert!(ghidra_auto_name(&name("caseD", "")).is_none());
        assert!(ghidra_auto_name(&name("caseD", "xyz")).is_none());
        // Constants and ordinary words that merely start alike are not names.
        assert!(ghidra_auto_name("BYTES_004a6d52").is_none());
        assert!(ghidra_auto_name("WORD_COUNT").is_none());
        assert!(ghidra_auto_name("switch_00401000").is_none());
    }

    #[test]
    fn a_leading_underscore_does_not_hide_a_name() {
        // The decompiler prints one when an access does not match the type of
        // the data it reads.
        let data = format!("_{}", name("DAT", "011d8a84"));
        let two = format!("__{}", name("DAT", "011d8a84"));
        let function = format!("_{}", name("FUN", "00401000"));
        let pointer = format!("_{}", name("PTR", &name("DAT", "011d8a84")));
        for token in [&data, &two, &function, &pointer] {
            assert_eq!(
                ghidra_auto_name(&format!("writes {token};")),
                Some(token.as_str()),
                "{token}"
            );
        }
        // Underscores alone, or around ordinary words, are not names.
        assert!(ghidra_auto_name("_").is_none());
        assert!(ghidra_auto_name("__init__").is_none());
        assert!(ghidra_auto_name(&format!("_{}", name("DAT", "0"))).is_none());
        assert!(ghidra_auto_name(&format!("_my_{}", name("DAT", "011d8a84"))).is_none());
    }

    #[test]
    fn string_labels_are_found() {
        let ascii = name("s", "Hello_world_0040907c");
        let wide = name("u", "Wide_text_00409068");
        let empty = name("s", "_00409068");
        let wrapped = name("PTR", &name("s", "Argument_domain_error__DOMAIN__004091e0"));
        let thunk = name("thunk", &name("u", "Text_00409068"));
        for token in [&ascii, &wide, &empty, &wrapped, &thunk] {
            assert_eq!(
                ghidra_auto_name(&format!("puts(&{token});")),
                Some(token.as_str()),
                "{token}"
            );
        }
        // The label has to start with the prefix, end in an address, and have
        // something between them or at least the separator.
        for token in [
            name("my_s", "x_00401000"),
            name("us", "x_00401000"),
            name("s", "x_4010"),
            name("s", "x_00401000_y"),
            name("s", "x_0040100g"),
            name("s", "00401000"),
            name("s", "x_004010000"),
            name("u", "name"),
            name("x", "00401000"),
        ] {
            assert!(ghidra_auto_name(&token).is_none(), "{token}");
        }
    }

    #[test]
    fn pointer_labels_name_their_target() {
        // A pointer to a function the program names, to a thunk, and to a string.
        let token = |parts: &[&str]| parts.concat();
        for found in [
            token(&["PTR", "_tls_callback_0", "_004090a8"]),
            token(&["PTR", "_thunk", "_FUN", "_00401410", "_00407f64"]),
            token(&["PTR", "_PTR", "_DAT", "_00401410"]),
            token(&["PTR", "_Sleep", "_00408050"]),
        ] {
            assert_eq!(
                ghidra_auto_name(&format!("(*{found})();")),
                Some(found.as_str()),
                "{found}"
            );
        }
        for other in [
            token(&["PTR", "_tls_callback_0"]),
            token(&["PTR", "_x", "_4090a"]),
            token(&["PTR", "_x", "_00408050", "_y"]),
            token(&["my", "_PTR", "_x", "_00408050"]),
            token(&["PTR", "_"]),
            token(&["PTR_", "SIZE"]),
            token(&["ptr", "_x", "_00408050"]),
        ] {
            assert!(ghidra_auto_name(&other).is_none(), "{other}");
        }
    }

    #[test]
    fn stack_and_register_offsets_have_eight_digits() {
        for found in [
            ["stack0x", "00000004"].concat(),
            ["stack0x", "ffffffb4"].concat(),
            ["register0x", "00000010"].concat(),
            ["code_r0x", "00405706"].concat(),
        ] {
            assert_eq!(
                ghidra_auto_name(&format!("f(&{found});")),
                Some(found.as_str()),
                "{found}"
            );
        }
        for other in [
            ["stack0x", "4"].concat(),
            ["stack0x", "000000004"].concat(),
            ["stack0x", "0000000g"].concat(),
            ["register0x", "10"].concat(),
            ["code_r0x", "4057"].concat(),
            ["my_stack0x", "00000004"].concat(),
            "stack0x".to_string(),
        ] {
            assert!(ghidra_auto_name(&other).is_none(), "{other}");
        }
    }

    #[test]
    fn switch_labels_carry_the_switch_address() {
        for found in [
            ["switchD_", "00404443", "_caseD_", "21"].concat(),
            ["switchD_", "00404443", "_default"].concat(),
            ["switchdataD_", "00404450"].concat(),
            ["switchD_", "004044"].concat(),
        ] {
            assert_eq!(
                ghidra_auto_name(&format!("goto {found};")),
                Some(found.as_str()),
                "{found}"
            );
        }
        for other in [
            ["switchD_", "4044", "_caseD_", "21"].concat(),
            ["switchD_", "00404443x"].concat(),
            ["switchD_", "xyz"].concat(),
            ["switchD_"].concat(),
            ["my_switchD_", "00404443"].concat(),
        ] {
            assert!(ghidra_auto_name(&other).is_none(), "{other}");
        }
    }

    #[test]
    fn imports_by_ordinal_are_found() {
        for number in ["1", "12", "65535"] {
            let token = name("Ordinal", number);
            assert_eq!(ghidra_auto_name(&token), Some(token.as_str()), "{number}");
        }
        let thunk = name("thunk", &name("Ordinal", "12"));
        assert_eq!(ghidra_auto_name(&thunk), Some(thunk.as_str()));
        for other in [
            name("Ordinal", ""),
            name("Ordinal", "x"),
            name("Ordinal", "123456"),
            name("ordinal", "12"),
            name("Ordinals", "12"),
            name("my_Ordinal", "12"),
        ] {
            assert!(ghidra_auto_name(&other).is_none(), "{other}");
        }
    }

    /// Shapes that Ghidra 12.1.4 printed for a small synthetic 32-bit
    /// executable built from a C file written for this project (named
    /// functions, strings, a table of strings, floating-point constants, a
    /// switch and the C runtime start-up code). Each is a name this check
    /// has to flag.
    #[test]
    fn shapes_seen_in_ghidra_output_are_found() {
        let seen: &[&[&str]] = &[
            &["_", "DAT", "_004094f8"],
            &["DAT", "_7ffe0010"],
            &["_", "DAT", "_7ffe0100"],
            &["PTR", "_DAT", "_0040803c"],
            &["PTR", "_FUN", "_00408050"],
            &["PTR", "_tls_callback_0", "_004090a8"],
            &["PTR", "_thunk", "_FUN", "_00401410", "_00407f64"],
            &["PTR", "_s", "_Argument_domain_error__DOMAIN__", "004091e0"],
            &["s", "_Hello_world_", "0040907c"],
            &["u", "_Wide_text_", "00409068"],
            &["s", "_a_literal_in_main_", "00409056"],
            &["stack0x", "00000004"],
            &["stack0x", "ffffffb4"],
            &["switchD", "_00404443", "_caseD", "_21"],
            &["code_r0x", "00405706"],
            &["joined_r0x", "0040131d"],
            &["thunk", "_FUN", "_00407e90"],
            &["extraout", "_EAX", "_00"],
            &["extraout", "_ECX", "_10"],
            &["in", "_EAX"],
            &["unaff", "_ESI"],
            &["u", "StackY", "_50"],
            &["u", "Stack", "_44"],
            &["ac", "Stack", "_3c"],
            &["ai", "Stack", "_60"],
            &["pb", "Stack", "_88"],
            &["p", "F", "Var1"],
            &["U", "Var1"],
            &["B", "Var2"],
            &["puVar", "2"],
            &["pcVar", "5"],
            &["local", "_5c"],
            &["param", "_3"],
            &["unkuint", "10"],
            &["undefined", "4"],
        ];
        for parts in seen {
            let token = parts.concat();
            assert_eq!(
                ghidra_auto_name(&format!("  x = {token} + 1;")),
                Some(token.as_str()),
                "{token}"
            );
        }
    }

    #[test]
    fn address_names_look_through_wrappers() {
        let thunk = name("thunk", &name("FUN", "00401000"));
        let pointer = name("PTR", &name("DAT", "01234567"));
        let nested = name("PTR", &name("PTR", &name("FUN", "00401000")));
        assert_eq!(ghidra_auto_name(&thunk), Some(thunk.as_str()));
        assert_eq!(ghidra_auto_name(&pointer), Some(pointer.as_str()));
        assert_eq!(ghidra_auto_name(&nested), Some(nested.as_str()));
        assert!(ghidra_auto_name(&name("thunk", "x")).is_none());
    }

    #[test]
    fn names_inside_longer_identifiers_are_not_matched() {
        assert!(ghidra_auto_name(&format!("my_{}", name("FUN", "00401000"))).is_none());
        assert!(ghidra_auto_name(&format!("{}_ext", name("FUN", "00401000"))).is_none());
        assert!(ghidra_auto_name(&format!("my_{}", name("param", "1"))).is_none());
        assert!(ghidra_auto_name(&format!("{}x", name("param", "1"))).is_none());
        assert!(ghidra_auto_name(&format!("my{}1", "uVar")).is_none());
        assert!(ghidra_auto_name(&format!("my_{}", name("unaff", "ESI"))).is_none());
        assert!(ghidra_auto_name(&format!("{}_x", name("in_stack", "00000008"))).is_none());
    }

    #[test]
    fn numbered_parameters_need_digits() {
        let one = name("param", "1");
        let twelve = name("param", "12");
        assert_eq!(ghidra_auto_name(&one), Some(one.as_str()));
        assert_eq!(
            ghidra_auto_name(&format!("({twelve})")),
            Some(twelve.as_str())
        );
        assert!(ghidra_auto_name(&["param", "_"].concat()).is_none());
        assert!(ghidra_auto_name(&name("param", "x")).is_none());
        assert!(ghidra_auto_name("params_1").is_none());
        assert!(ghidra_auto_name("with its second argument 1").is_none());
    }

    #[test]
    fn register_inputs_are_found() {
        for reg in [
            "ECX", "EAX", "EDX", "ST0", "ST7", "XMM0", "XMM15", "CF", "FS", "MM3", "YMM15",
        ] {
            assert_eq!(
                ghidra_auto_name(&name("in", reg)),
                Some(name("in", reg).as_str()),
                "{reg}"
            );
        }
        assert!(ghidra_auto_name(&name("in", "ST8")).is_none());
        assert!(ghidra_auto_name(&name("in", "XMM16")).is_none());
        // Ordinary snake_case words are not registers.
        assert!(ghidra_auto_name(&name("in", "ecx")).is_none());
        assert!(ghidra_auto_name(&name("in", "range")).is_none());
        assert!(ghidra_auto_name(&name("in", "EAX_flag")).is_none());
    }

    #[test]
    fn unaffected_and_extra_output_registers_are_found() {
        for token in [
            name("unaff", "ESI"),
            name("unaff", "EBX"),
            name("unaff", "retaddr"),
            name("unaff", "FS_OFFSET"),
            name("extraout", "ECX"),
            name("extraout", "ST0"),
            name("extraout", "XMM0_Da"),
            name("extraout", "XMM0_Qa_00"),
            name("extraout", "var"),
            name("extraout", "var_00"),
            name("in", "FS_OFFSET"),
            name("in", "XMM0_Da"),
            name("in", "XMM1_Qb"),
        ] {
            assert_eq!(
                ghidra_auto_name(&format!("{token} = 0;")),
                Some(token.as_str()),
                "{token}"
            );
        }
        for token in [
            name("unaff", "esi"),
            name("unaff", "var"),
            name("unaff", "ESI_flag"),
            name("extraout", "retaddr"),
            name("extraout", "value"),
            name("unaff", "ST9"),
            name("in", "OFFSET"),
        ] {
            assert!(ghidra_auto_name(&token).is_none(), "{token}");
        }
    }

    #[test]
    fn stack_inputs_have_eight_hex_digits() {
        for hex in ["00000008", "ffffffe8", "FFFFFFF0"] {
            let token = name("in_stack", hex);
            assert_eq!(ghidra_auto_name(&token), Some(token.as_str()), "{hex}");
        }
        assert!(ghidra_auto_name(&name("in_stack", "8")).is_none());
        assert!(ghidra_auto_name(&name("in_stack", "0000000g")).is_none());
        assert!(ghidra_auto_name(&name("in_stack", "000000008")).is_none());
        assert!(ghidra_auto_name("in_stack").is_none());
    }

    #[test]
    fn numbered_temporaries_are_found() {
        for prefix in [
            "uVar", "iVar", "fVar", "bVar", "pcVar", "puVar", "cVar", "sVar", "lVar", "dVar",
            "pvVar", "piVar", "pfVar", "ppVar", "pbVar", "psVar", "ppuVar", "auVar", "ulVar",
            "pCVar", "ppCVar",
        ] {
            let token = format!("{prefix}7");
            assert_eq!(
                ghidra_auto_name(&format!("{token} = 1;")),
                Some(token.as_str()),
                "{prefix}"
            );
        }
        // Any depth of pointers, and a capital letter for a named type.
        for prefix in [
            "pppcVar", "ppppuVar", "pppVar", "UVar", "BVar", "DVar", "SVar", "pFVar", "pIVar",
            "ppDVar",
        ] {
            let token = format!("{prefix}3");
            assert_eq!(
                ghidra_auto_name(&format!("{token} = 1;")),
                Some(token.as_str()),
                "{prefix}"
            );
        }
        for prefix in ["XYZVar", "UuVar", "ppppppppuVar", "pxVar", "Up", "pUUVar"] {
            assert!(
                ghidra_auto_name(&format!("{prefix}3")).is_none(),
                "{prefix}"
            );
        }
        assert!(ghidra_auto_name(&["uVar", "x"].concat()).is_none());
        assert!(ghidra_auto_name("uVar").is_none());
        assert!(ghidra_auto_name(&["uVar", "1a"].concat()).is_none());
        assert!(ghidra_auto_name("var1").is_none());
        // Ordinary names that end in `Var` and a number are not temporaries.
        assert!(ghidra_auto_name(&["myVar", "1"].concat()).is_none());
        assert!(ghidra_auto_name(&["newVar", "2"].concat()).is_none());
        assert!(ghidra_auto_name(&["Var", "1"].concat()).is_none());
        assert!(ghidra_auto_name(&["xVar", "1"].concat()).is_none());
    }

    #[test]
    fn stack_temporaries_are_found() {
        for prefix in [
            "uStack", "auStack", "iStack", "fStack", "pcStack", "puStack", "cStack", "ppuStack",
            "pCStack",
        ] {
            for offset in ["8", "c", "28", "14c"] {
                let token = name(prefix, offset);
                assert_eq!(
                    ghidra_auto_name(&format!("{token} = 1;")),
                    Some(token.as_str()),
                    "{token}"
                );
            }
        }
        // A capital letter may follow `Stack`.
        for token in [
            ["uStack", "Y_50"].concat(),
            ["auStack", "X_8"].concat(),
            ["pcStack", "Y_c"].concat(),
        ] {
            assert_eq!(ghidra_auto_name(&token), Some(token.as_str()), "{token}");
        }
        for token in [
            ["uStack", "YZ_50"].concat(),
            ["uStack", "Y50"].concat(),
            ["uStack", "Y_"].concat(),
            ["uStack", "y_50"].concat(),
        ] {
            assert!(ghidra_auto_name(&token).is_none(), "{token}");
        }
        assert!(ghidra_auto_name(&name("uStack", "")).is_none());
        assert!(ghidra_auto_name(&name("uStack", "xyz")).is_none());
        assert!(ghidra_auto_name(&name("uStack", "123456789")).is_none());
        assert!(ghidra_auto_name(&name("callStack", "1")).is_none());
        assert!(ghidra_auto_name(&name("mStack", "1")).is_none());
        assert!(ghidra_auto_name("Stack_8").is_none());
    }

    #[test]
    fn stack_locals_are_lowercase_hex() {
        for hex in ["8", "c8", "10", "14c", "2c", "1a4"] {
            assert!(ghidra_auto_name(&name("local", hex)).is_some(), "{hex}");
        }
        // The first local of a frame is often `c`; offsets with no decimal digit
        // are names too.
        for hex in ["a", "c", "e", "f", "ac", "cc", "fec"] {
            assert!(ghidra_auto_name(&name("local", hex)).is_some(), "{hex}");
        }
        // Words made of hex letters are normal identifiers.
        for word in ["add", "bad", "bed", "fee", "dead", "face", "beef", "cafe"] {
            assert!(ghidra_auto_name(&name("local", word)).is_none(), "{word}");
        }
        assert!(ghidra_auto_name(&name("local", "123456789")).is_none());
        assert!(ghidra_auto_name(&name("local", "8x")).is_none());
        assert!(ghidra_auto_name(&name("local", "")).is_none());
        assert!(ghidra_auto_name("local_time").is_none());
        assert!(ghidra_auto_name("local_x").is_none());
        assert!(ghidra_auto_name(&name("local", "C8")).is_none());
        // Parameter-area locals.
        for offset in ["0", "4", "8", "c"] {
            let token = name("local", &format!("res{offset}"));
            assert_eq!(ghidra_auto_name(&token), Some(token.as_str()), "{offset}");
        }
        assert!(ghidra_auto_name(&name("local", "res")).is_none());
        assert!(ghidra_auto_name(&name("local", "resume")).is_none());
        assert!(ghidra_auto_name(&name("local", "result")).is_none());
    }

    #[test]
    fn helper_calls_need_digits_and_a_parenthesis() {
        for helper in [
            "CONCAT", "SUB", "ZEXT", "SEXT", "CARRY", "SCARRY", "SBORROW",
        ] {
            let call = |digits: &str, after: &str| format!("x = {helper}{digits}{after}");
            let token = format!("{helper}41");
            assert_eq!(
                ghidra_auto_name(&call("41", "(a, b)")),
                Some(token.as_str()),
                "{helper}"
            );
            assert_eq!(
                ghidra_auto_name(&call("416", "(")),
                Some(format!("{helper}416").as_str())
            );
            assert!(ghidra_auto_name(&call("41", " (a, b)")).is_none());
            assert!(ghidra_auto_name(&call("", "(a)")).is_none());
            assert!(ghidra_auto_name(&call("41", "")).is_none());
            assert!(ghidra_auto_name(&call("4x", "(a)")).is_none());
        }
    }

    #[test]
    fn undefined_types_carry_a_size() {
        for size in ["1", "2", "4", "8", "16"] {
            let token = format!("undefined{size}");
            assert_eq!(
                ghidra_auto_name(&format!("{token} *")),
                Some(token.as_str())
            );
        }
        for kind in ["unkbyte", "unkint", "unkuint", "unkfloat"] {
            let token = format!("{kind}10");
            assert_eq!(ghidra_auto_name(&token), Some(token.as_str()), "{kind}");
            assert!(ghidra_auto_name(kind).is_none(), "{kind}");
            assert!(ghidra_auto_name(&format!("{kind}123")).is_none(), "{kind}");
        }
        assert!(ghidra_auto_name("unknown10").is_none());
        assert!(ghidra_auto_name("undefined").is_none());
        assert!(ghidra_auto_name("undefined behaviour").is_none());
        assert!(ghidra_auto_name("undefined123").is_none());
    }

    #[test]
    fn every_name_on_a_line_is_listed_once() {
        let line = format!(
            "{} = {}({}, {});",
            name("local", "c"),
            name("FUN", "00401000"),
            name("local", "c"),
            name("param", "1")
        );
        assert_eq!(
            ghidra_auto_names(&line),
            vec![
                name("local", "c").as_str(),
                name("FUN", "00401000").as_str(),
                name("param", "1").as_str(),
            ]
        );
        assert_eq!(ghidra_auto_name(&line), Some(name("local", "c").as_str()));
        assert!(ghidra_auto_names("nothing to see").is_empty());
    }

    #[test]
    fn repo_style_text_is_not_flagged() {
        for line in [
            "/// `00b0d7b0`: the constructor, reads `[011d8a84]` (a global).",
            "let local_x = 3; let param = 2; let in_range = true;",
            "// the second argument is 1; `008a7570() == -1`",
            "const MAX: u16 = 15; // `[011ae75c]`",
            "fn fun_00401000_is_not_a_name() {}",
            "let sub_total = a - b; let ext = 1; let local_add = 2;",
            "const DWORD_SIZE: usize = 4; const BYTE_COUNT: usize = 8;",
            "let var1 = 2; let my_var2 = 3; let new_stack_4 = 4;",
            "let s_total = 1; let u_len = 2; let s_a_b_c = 3; let PTR_SIZE = 4;",
            "let ptr_00401000 = 5; let ordinal_3 = 6; let Ordinals_1 = 7;",
            "// the `stack0x` prefix, `register0x` and `switchD_` are only prefixes",
        ] {
            assert!(ghidra_auto_name(line).is_none(), "{line}");
        }
    }

    #[test]
    fn fenced_c_blocks_are_found_in_markdown() {
        let md = "text\n```c\nint x;\n```\n\n```cpp\nint y;\n```\n```text\nno\n```\n";
        assert_eq!(
            c_fences(md),
            vec![(2, "c".to_string()), (6, "cpp".to_string())]
        );
        // Indented fences (inside a list), tildes, tag case, extra words.
        let md = "- item\n  ```C\n  x\n  ```\n~~~c++ extra\nx\n~~~\n";
        assert_eq!(
            c_fences(md),
            vec![(2, "c".to_string()), (5, "c++".to_string())]
        );
    }

    #[test]
    fn every_way_of_naming_a_c_fence_is_found() {
        // Header and source tags, other spellings of C++, attribute braces
        // (pandoc), a word followed by more, and `language-` classes.
        let tags = [
            ("c", "c"),
            ("C", "c"),
            ("h", "h"),
            ("cc", "cc"),
            ("cxx", "cxx"),
            ("c++", "c++"),
            ("cpp", "cpp"),
            ("CPP", "cpp"),
            ("hpp", "hpp"),
            ("hxx", "hxx"),
            ("{.c}", "c"),
            ("{ .cpp }", "cpp"),
            ("{.c .numberLines}", "c"),
            ("{#listing .h startFrom=\"10\"}", "h"),
            ("c,ignore", "c"),
            ("c{1-3}", "c"),
            ("c:file.h", "c"),
            ("c title=\"x\"", "c"),
            ("language-c", "c"),
            ("language-cpp", "cpp"),
        ];
        for (tag, language) in tags {
            for fence in ["```", "~~~", "````"] {
                let md = format!("text\n{fence}{tag}\nint x;\n{fence}\n");
                assert_eq!(
                    c_fences(&md),
                    vec![(2, language.to_string())],
                    "{fence}{tag}"
                );
            }
        }
    }

    #[test]
    fn other_languages_are_not_c_fences() {
        for tag in [
            "rust",
            "text",
            "cs",
            "csharp",
            "c#",
            "ch",
            "cmake",
            "cfg",
            "console",
            "clojure",
            "{.rust}",
            "{.text .c_lines}",
            "{#c}",
            "{}",
            "{",
            "language-rust",
            "language-",
            "toml,c",
        ] {
            let md = format!("```{tag}\nx\n```\n");
            assert!(c_fences(&md).is_empty(), "{tag}");
        }
    }

    #[test]
    fn other_fences_and_inline_code_are_not_c_blocks() {
        assert!(c_fences("```rust\nlet x = 1;\n```\n").is_empty());
        assert!(c_fences("```\nplain\n```\n").is_empty());
        assert!(c_fences("```powershell\ncargo test\n```\n").is_empty());
        // Three backticks around inline code are not a fence.
        assert!(c_fences("use ```c``` inline\n```c```\n").is_empty());
        // A fence shown inside a longer fence is content.
        assert!(c_fences("````markdown\n```c\nx\n```\n````\n").is_empty());
        // After a block closes the next opener counts again.
        assert_eq!(c_fences("```text\nx\n```\n```c\ny\n```\n").len(), 1);
    }

    #[test]
    fn gitignore_entries_follow_git_for_simple_cases() {
        let ignore = GitIgnore::parse(
            "# comment\n\ntarget/\n/dist/\nreports/\n*.exe\nnv-rs-*.txt\nviewer/nv-rs-*.txt\n\
             .env\n.env.*\n!/.env.example\n/00447330/\n",
        );
        assert!(ignore.is_ignored("target", true));
        assert!(ignore.is_ignored("crates/x/target", true));
        assert!(!ignore.is_ignored("target", false));
        assert!(ignore.is_ignored("dist", true));
        assert!(!ignore.is_ignored("crates/dist", true));
        assert!(ignore.is_ignored("reports", true));
        assert!(ignore.is_ignored("crates/x/reports", true));
        assert!(ignore.is_ignored("a/b.exe", false));
        assert!(ignore.is_ignored("nv-rs-log.txt", false));
        assert!(ignore.is_ignored("sub/nv-rs-log.txt", false));
        assert!(ignore.is_ignored("viewer/nv-rs-log.txt", false));
        assert!(ignore.is_ignored(".env.local", false));
        assert!(!ignore.is_ignored(".env.example", false));
        assert!(ignore.is_ignored("00447330", true));
        assert!(!ignore.is_ignored("docs/notes.txt", false));
        assert!(!ignore.is_ignored("src/main.rs", false));
    }

    #[test]
    fn nested_gitignore_entries_apply_below_their_directory() {
        let mut ignore = GitIgnore::parse("*.log\n");
        ignore.add(
            "research/tool",
            "out/\n/build/\n*.tmp\nsub/cache\n!keep.tmp\n",
        );
        assert!(ignore.is_ignored("research/tool/out", true));
        assert!(ignore.is_ignored("research/tool/deep/out", true));
        assert!(ignore.is_ignored("research/tool/build", true));
        assert!(!ignore.is_ignored("research/tool/deep/build", true));
        assert!(ignore.is_ignored("research/tool/a.tmp", false));
        assert!(!ignore.is_ignored("research/tool/keep.tmp", false));
        assert!(ignore.is_ignored("research/tool/sub/cache", false));
        assert!(!ignore.is_ignored("research/tool/other/sub/cache", false));
        // Outside that directory the nested entries do not apply.
        assert!(!ignore.is_ignored("research/out", true));
        assert!(!ignore.is_ignored("out", true));
        assert!(!ignore.is_ignored("research/tool2/out", true));
        assert!(!ignore.is_ignored("research/other/a.tmp", false));
        // Root entries still apply everywhere.
        assert!(ignore.is_ignored("research/tool/x.log", false));
    }

    #[test]
    fn globs_do_not_cross_directories() {
        assert!(glob(b"*.rs", b"main.rs"));
        assert!(glob(b"a*c", b"abbbc"));
        assert!(glob(b"a?c", b"abc"));
        assert!(glob(b"*", b"anything"));
        assert!(!glob(b"*.rs", b"src/main.rs"));
        assert!(!glob(b"a?c", b"a/c"));
        assert!(!glob(b"a*c", b"abd"));
        assert!(glob(b"viewer/nv-rs-*.txt", b"viewer/nv-rs-1.txt"));
        assert!(!glob(b"viewer/nv-rs-*.txt", b"viewer/a/nv-rs-1.txt"));
    }
}

// ---------------------------------------------------------------------------
// Fixture trees: the whole check on small repositories made here, so every
// rule, the file listing and the ignore handling are exercised without
// touching the real repository.
// ---------------------------------------------------------------------------

mod fixtures {
    use super::*;

    /// An empty directory under the test target directory.
    fn fresh_root(name: &str) -> PathBuf {
        let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn put(root: &Path, rel: &str, bytes: &[u8]) {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    fn auto(kind: &str, rest: &str) -> String {
        format!("{kind}_{rest}")
    }

    /// The files every fixture holds. `.gitignore` is given by the test.
    /// Returns the expected `path:line` of each problem, in order.
    fn plant(root: &Path, gitignore: &str) -> Vec<&'static str> {
        let user_path = ["C:", r"\Users\alice\nv-re"].concat();
        put(root, ".gitignore", gitignore.as_bytes());
        put(root, "research/tool/.gitignore", b"out/\n");
        // Rule a: a control character, in a text file under any directory, and
        // in a text file with an unusual extension.
        put(root, "crates/a/src/nul.rs", b"fn f() {}\n// bad \0 byte\n");
        put(root, "research/tool.py", b"x = 1\n\x1b[0m\n");
        put(root, "Cargo.lock", b"# lock\nname = \"a\0\"\n");
        // Binary files: a NUL in a file that is not known text, and no
        // extension needed. Known image types are left alone.
        put(root, "crates/x/blob", b"MZ\0\0data");
        put(root, "crates/x/lib.o", b"\x7fELF\0\0");
        put(root, "assets/logo.png", b"\x89PNG\r\n\x1a\n\0\0");
        // Rule b: a user-profile path; `%USERPROFILE%` is fine. Any file type
        // is read: a CSV, a file with no extension, a numbered extension.
        put(
            root,
            "docs/paths.md",
            format!("fine: %USERPROFILE%\\x\nbad: {user_path}\n").as_bytes(),
        );
        put(
            root,
            "data/notes.csv",
            format!("a,{user_path}\n").as_bytes(),
        );
        put(
            root,
            "crates/x/NOTES",
            format!("see {user_path}\n").as_bytes(),
        );
        put(
            root,
            "license-apache-2.0",
            format!("ok\n{user_path}\n").as_bytes(),
        );
        // Rule c: banned types anywhere, in any case; a Ghidra project is a
        // directory.
        put(root, "crates/a/data.ESM", b"x");
        put(root, "research/sub/tool.exe", b"x");
        put(root, "tools/proj.rep/idata/00/00000001.gbf", b"x");
        put(root, "tools/proj.rep/project.prp", b"x");
        // Rule d: only under crates/, viewer/ and docs/. Each line has one
        // finding; the later names on a line are not reported separately.
        put(
            root,
            "docs/decomp.md",
            format!(
                "ok\n```c\nint {} = {}(1);\n```\nuses {}\n{} {} {}\n{}\n",
                auto("local", "c"),
                auto("FUN", "00401000"),
                auto("DAT", "011d8a84"),
                ["c", "Var1"].concat(),
                auto("uStack", "8"),
                auto("unaff", "ESI"),
                ["SUB", "41", "(x)"].concat(),
            )
            .as_bytes(),
        );
        put(
            root,
            "research/notes.md",
            format!("```c\n{}\n```\n", auto("FUN", "00401000")).as_bytes(),
        );
        put(
            root,
            "viewer/src/x.rs",
            format!("// {}\n", auto("param", "2")).as_bytes(),
        );
        put(
            root,
            "viewer/src/clean.rs",
            b"// `00401000` reads `[011d8a84]`.\n",
        );
        // Hidden control characters, text that is not UTF-8, an older Windows
        // profile root, a header fence, and names in the shapes Ghidra prints
        // for strings, pointers and untyped data (one finding per line).
        let xp_path = ["C:", r"\Documents and Settings\erin\x"].concat();
        put(root, "docs/hidden.md", "ok\nx\u{202e}y\n".as_bytes());
        put(root, "crates/a/src/latin1.rs", b"// caf\xe9\n");
        put(root, "docs/xp.md", format!("{xp_path}\n").as_bytes());
        put(root, "docs/fence.md", b"```h\nint x;\n```\n");
        put(
            root,
            "docs/shapes.md",
            format!(
                "{}\n{}\n{}\nfine\n",
                ["_", "DAT", "_011d8a84"].concat(),
                ["s", "_Hello_", "00401000"].concat(),
                ["PTR", "_tls_callback_0", "_004090a8"].concat(),
            )
            .as_bytes(),
        );
        // Ignored by the nested .gitignore.
        put(
            root,
            "research/tool/out/run.json",
            format!("{{\"p\": \"{user_path}\"}}\n").as_bytes(),
        );
        vec![
            "Cargo.lock:2",
            "crates/a/data.ESM:1",
            "crates/a/src/latin1.rs:1",
            "crates/a/src/nul.rs:2",
            "crates/x/NOTES:1",
            "crates/x/blob:1",
            "crates/x/lib.o:1",
            "data/notes.csv:1",
            "docs/decomp.md:2",
            "docs/decomp.md:3",
            "docs/decomp.md:5",
            "docs/decomp.md:6",
            "docs/decomp.md:7",
            "docs/fence.md:1",
            "docs/hidden.md:2",
            "docs/paths.md:2",
            "docs/shapes.md:1",
            "docs/shapes.md:2",
            "docs/shapes.md:3",
            "docs/xp.md:1",
            "license-apache-2.0:2",
            "research/sub/tool.exe:1",
            "research/tool.py:2",
            "tools/proj.rep/:1",
            "viewer/src/x.rs:1",
        ]
    }

    fn places(findings: &[String]) -> Vec<&str> {
        findings
            .iter()
            .map(|f| f.split_once(": ").expect("path:line: rule").0)
            .collect()
    }

    /// Checks the messages of the planted problems, by place.
    fn assert_messages(findings: &[String]) {
        let message = |place: &str| -> &str {
            findings
                .iter()
                .find_map(|f| f.strip_prefix(&format!("{place}: ")))
                .unwrap_or_else(|| panic!("no finding at {place}: {findings:#?}"))
        };
        assert!(message("crates/a/src/nul.rs:2").contains("control character 0x00"));
        assert!(message("Cargo.lock:2").contains("control character 0x00"));
        assert!(message("crates/x/blob:1").contains("binary file"));
        assert!(message("crates/x/lib.o:1").contains("binary file"));
        assert!(message("docs/decomp.md:2").contains("fenced `c` block"));
        assert!(
            message("docs/decomp.md:3").contains(&format!("auto-name `{}`", auto("local", "c")))
        );
        assert!(message("docs/decomp.md:5")
            .contains(&format!("auto-name `{}`", auto("DAT", "011d8a84"))));
        assert!(message("docs/decomp.md:6").contains(&format!(
            "auto-name `{}`, `{}`, `{}`",
            ["c", "Var1"].concat(),
            auto("uStack", "8"),
            auto("unaff", "ESI")
        )));
        assert!(message("docs/decomp.md:7")
            .contains(&format!("auto-name `{}`", ["SUB", "41"].concat())));
        assert!(message("docs/hidden.md:2").contains("hidden control character U+202E"));
        assert!(message("crates/a/src/latin1.rs:1").contains("not valid UTF-8"));
        assert!(message("docs/xp.md:1").contains("user-profile path"));
        assert!(message("docs/fence.md:1").contains("fenced `h` block"));
        assert!(message("docs/shapes.md:1").contains(&format!(
            "auto-name `{}`",
            ["_", "DAT", "_011d8a84"].concat()
        )));
        assert!(message("docs/shapes.md:2").contains(&format!(
            "auto-name `{}`",
            ["s", "_Hello_", "00401000"].concat()
        )));
        assert!(message("docs/shapes.md:3").contains(&format!(
            "auto-name `{}`",
            ["PTR", "_tls_callback_0", "_004090a8"].concat()
        )));
        assert!(message("docs/paths.md:2").contains("user-profile path"));
        assert!(message("data/notes.csv:1").contains("user-profile path"));
        assert!(message("crates/a/data.ESM:1").contains("`.esm`"));
        assert!(message("tools/proj.rep/:1").contains("directory type `.rep`"));
    }

    /// The walk used when git is not available: skips the named directories and
    /// follows the `.gitignore` files it meets.
    #[test]
    fn walk_reports_exactly_the_planted_problems() {
        let root = fresh_root("repo_hygiene_walk");
        let expected = plant(&root, "dist/\nreports/\nnv-*.txt\ntarget/\n*.dll\n");
        let user_path = ["C:", r"\Users\alice\nv-re"].concat();
        // Skipped: the named directories, directories and files the root
        // .gitignore lists, and `.claude` (local settings and worktrees).
        for dir in [
            "target",
            "node_modules",
            ".git",
            ".build",
            ".claude",
            ".claude/worktrees/old",
            "crates/a/target",
            "dist",
            "reports",
        ] {
            put(&root, &format!("{dir}/skip.esm"), b"x");
            put(
                &root,
                &format!("{dir}/skip.txt"),
                format!("{user_path}\n").as_bytes(),
            );
        }
        put(&root, "nv-play.txt", format!("{user_path}\n").as_bytes());
        put(
            &root,
            "viewer/nv-play.txt",
            format!("{user_path}\n").as_bytes(),
        );
        // An ignored file is not part of the repository, whatever its type.
        put(&root, "stray.dll", b"x");
        // Read in this fixture only: it is not a git checkout.
        assert!(files_from_git(&root).is_none());

        let listing = list_files(&root);
        assert_eq!(listing.source, "a directory walk (no git)");
        let findings = check_listing(listing);
        assert_eq!(places(&findings), expected, "{findings:#?}");
        assert_messages(&findings);
        let _ = fs::remove_dir_all(&root);
    }

    #[cfg(unix)]
    #[test]
    fn symbolic_links_are_not_followed_but_their_names_are_checked() {
        use std::os::unix::fs::symlink;
        let root = fresh_root("repo_hygiene_links");
        let user_path = ["C:", r"\Users\alice\nv-re"].concat();
        put(&root, "real/notes.txt", format!("{user_path}\n").as_bytes());
        put(&root, "real/a.txt", b"fine\n");
        // A link to a directory with a text extension, a link to nothing, a
        // link to a file whose own content is bad, and a link named like game
        // data.
        symlink("real", root.join("linked.md")).unwrap();
        symlink("missing", root.join("dangling.txt")).unwrap();
        symlink("real/a.txt", root.join("alias.txt")).unwrap();
        symlink("real/a.txt", root.join("Game.ESM")).unwrap();

        let findings = check_listing(files_from_walk(&root));
        assert_eq!(
            places(&findings),
            vec!["Game.ESM:1", "real/notes.txt:1"],
            "{findings:#?}"
        );
        let _ = fs::remove_dir_all(&root);
    }

    /// A name that is not valid UTF-8 cannot be matched against the rules, so it
    /// is reported instead of being skipped, by the walk and by the git listing.
    // APFS rejects invalid UTF-8 path components before the checker can see
    // them; other Unix filesystems permit this fixture.
    #[cfg(all(unix, not(target_os = "macos")))]
    #[test]
    fn names_that_are_not_utf8_are_reported() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;
        let root = fresh_root("repo_hygiene_utf8");
        put(&root, "fine.txt", b"ok\n");
        fs::write(root.join(OsStr::from_bytes(b"bad\xff.txt")), b"x").unwrap();

        let findings = check_listing(files_from_walk(&root));
        assert_eq!(findings.len(), 1, "{findings:#?}");
        assert!(findings[0].starts_with(".:1: file name is not valid UTF-8"));

        if git_available() {
            run_git(&root, &["init", "-q"]);
            let listing = files_from_git(&root).expect("a git checkout");
            let findings = check_listing(listing);
            assert_eq!(findings.len(), 1, "{findings:#?}");
            assert!(findings[0].ends_with(":1: file name is not valid UTF-8"));
        }
        let _ = fs::remove_dir_all(&root);
    }

    fn git_available() -> bool {
        Command::new("git")
            .arg("--version")
            .output()
            .is_ok_and(|o| o.status.success())
    }

    fn run_git(root: &Path, args: &[&str]) {
        let output = git(root).args(args).output().expect("run git");
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    /// The listing used on a real checkout: git decides what is ignored, from
    /// nested `.gitignore` files and `.git/info/exclude` as well as the root
    /// one, and a file added with `git add -f` is seen under an ignored
    /// directory. An untracked file in a `.claude` directory is local agent
    /// state and is left out; a tracked one is checked.
    #[test]
    fn git_listing_honours_every_ignore_source() {
        if !git_available() {
            eprintln!("git is not available: the git listing is not tested here");
            return;
        }
        let root = fresh_root("repo_hygiene_git");
        run_git(&root, &["init", "-q"]);
        // No entry for `.claude`: the real repository has none either.
        let mut expected = plant(&root, "dist/\nreports/\nnv-*.txt\ntarget/\n*.dll\n");
        let user_path = ["C:", r"\Users\alice\nv-re"].concat();
        // Local agent state that holds a profile path, untracked and not
        // ignored: at the top, nested, and a whole scratch worktree.
        for rel in [
            ".claude/settings.local.json",
            ".claude/worktrees/old/docs/a.md",
            "research/tool/.claude/notes.txt",
        ] {
            put(
                &root,
                rel,
                format!("{{\"allow\": \"Read(//c/{}/x)\"}}\n", "Users/alice").as_bytes(),
            );
        }
        // Ignored by the root .gitignore, and by .git/info/exclude.
        put(&root, "dist/skip.esm", b"x");
        put(&root, "nv-play.txt", format!("{user_path}\n").as_bytes());
        put(&root, "stray.dll", b"x");
        put(
            &root,
            "scratch/notes.txt",
            format!("{user_path}\n").as_bytes(),
        );
        put(&root, ".git/info/exclude", b"scratch/\n");
        // And by an excludes file named in the git configuration, which is how
        // a contributor's global ignore list reaches the repository.
        let excludes = root.with_file_name("repo_hygiene_git_excludes");
        fs::write(&excludes, b"personal/\n").unwrap();
        run_git(
            &root,
            &["config", "core.excludesFile", excludes.to_str().unwrap()],
        );
        put(
            &root,
            "personal/notes.txt",
            format!("{user_path}\n").as_bytes(),
        );
        // Force-added under an ignored directory: it will be committed, so it
        // is checked.
        put(&root, "dist/forced.esm", b"x");
        run_git(&root, &["add", "-f", "dist/forced.esm"]);
        // Tracked, then removed from the work tree: nothing to read, so a
        // text file is not reported ...
        put(&root, "gone.md", format!("{user_path}\n").as_bytes());
        run_git(&root, &["add", "gone.md"]);
        fs::remove_file(root.join("gone.md")).unwrap();
        // ... but a name that is never committed still is.
        put(&root, "gone.esm", b"x");
        run_git(&root, &["add", "gone.esm"]);
        fs::remove_file(root.join("gone.esm")).unwrap();
        // Tracked and bad: found without being named in any ignore file,
        // including a settings file under `.claude` once it is tracked.
        put(
            &root,
            "viewer/tracked.rs",
            format!("// {user_path}\n").as_bytes(),
        );
        run_git(&root, &["add", "viewer/tracked.rs"]);
        put(
            &root,
            ".claude/settings.json",
            format!("{{\"allow\": \"Read(//c/{}/x)\"}}\n", "Users/alice").as_bytes(),
        );
        run_git(&root, &["add", ".claude/settings.json"]);

        let listing = files_from_git(&root).expect("a git checkout");
        assert_eq!(listing.source, "git ls-files");
        let findings = check_listing(listing);
        expected.extend([
            ".claude/settings.json:1",
            "dist/forced.esm:1",
            "gone.esm:1",
            "viewer/tracked.rs:1",
        ]);
        expected.sort_by_key(|place| place.split_once(':').unwrap().0.to_string());
        assert_eq!(places(&findings), expected, "{findings:#?}");
        assert_messages(&findings);
        let _ = fs::remove_dir_all(&root);
        let _ = fs::remove_file(&excludes);
    }

    #[test]
    fn agent_state_is_any_path_through_a_claude_directory() {
        assert!(in_agent_state(".claude/settings.local.json"));
        assert!(in_agent_state(".claude/worktrees/a/b.md"));
        assert!(in_agent_state("research/tool/.claude/x"));
        assert!(!in_agent_state("claude/x"));
        assert!(!in_agent_state("docs/.claudex/x"));
        assert!(!in_agent_state("docs/a.claude"));
        assert!(!in_agent_state("CLAUDE.md"));
    }

    /// A directory that is not the top of a git checkout (a scratch copy made
    /// with `git archive`, say) is walked, even when a parent directory is a
    /// repository. The parent repository is made here, so the guard is
    /// exercised wherever the test target directory is.
    #[test]
    fn a_directory_inside_another_repository_is_walked() {
        let parent = fresh_root("repo_hygiene_nested");
        let root = parent.join("inner");
        put(&root, "docs/ok.md", b"fine\n");
        put(&parent, "outer.md", b"fine\n");
        if git_available() {
            run_git(&parent, &["init", "-q"]);
            // Git itself sees `root` as part of the parent repository: it is
            // one level down from the top, so the show-prefix guard is what
            // keeps it from being listed through the parent.
            let prefix = git(&root)
                .args(["rev-parse", "--show-prefix"])
                .output()
                .expect("run git");
            assert_eq!(String::from_utf8_lossy(&prefix.stdout).trim(), "inner/");
            let top = files_from_git(&parent).expect("the parent is a checkout");
            let names: Vec<&str> = top.files.iter().map(|f| f.rel.as_str()).collect();
            assert_eq!(names, vec!["inner/docs/ok.md", "outer.md"]);
        }
        assert!(files_from_git(&root).is_none());
        let listing = list_files(&root);
        assert_eq!(listing.source, "a directory walk (no git)");
        assert_eq!(
            listing
                .files
                .iter()
                .map(|f| f.rel.as_str())
                .collect::<Vec<_>>(),
            vec!["docs/ok.md"]
        );
        let _ = fs::remove_dir_all(&parent);
    }

    #[test]
    fn large_files_are_reported_not_skipped() {
        let root = fresh_root("repo_hygiene_large");
        let mut big = vec![b'a'; MAX_SCAN_BYTES as usize + 1];
        big[10] = b'\n';
        put(&root, "data.txt", &big);
        put(&root, "small.txt", b"fine\n");
        let findings = check_listing(files_from_walk(&root));
        assert_eq!(places(&findings), vec!["data.txt:1"], "{findings:#?}");
        assert!(findings[0].contains("not scanned"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn unreadable_directories_are_findings_and_removed_files_are_skipped() {
        let root = fresh_root("repo_hygiene_missing");
        let mut listing = files_from_walk(&root);
        // A listed file that cannot be examined is reported; one that no
        // longer exists is not, unless its name is one that is never committed.
        listing.files.push(RepoFile {
            rel: "gone.rs".to_string(),
            path: root.join("gone.rs"),
        });
        assert!(check_listing(listing).is_empty());
        let mut listing = files_from_walk(&root);
        for rel in ["gone.rs", "gone.ESM", "dir.rep/inner/x.gbf"] {
            listing.files.push(RepoFile {
                rel: rel.to_string(),
                path: root.join(rel),
            });
        }
        let findings = check_listing(listing);
        assert_eq!(
            places(&findings),
            vec!["gone.ESM:1", "dir.rep/:1"],
            "{findings:#?}"
        );
        let mut out = Listing {
            files: Vec::new(),
            problems: Vec::new(),
            source: "test",
        };
        walk(&root.join("nope"), "nope", &mut GitIgnore::new(), &mut out);
        assert_eq!(out.problems.len(), 1);
        assert!(out.problems[0].starts_with("nope:1: cannot read directory"));
        let _ = fs::remove_dir_all(&root);
    }
}

/// Each typed function is registered and has its differential test.
#[test]
fn typed_functions_are_registered_and_have_a_differential_test() {
    let root = repo_root();
    let (mut sources, mut tests) = (Vec::new(), Vec::new());
    for file in list_files(&root).files {
        if !file.rel.ends_with(".rs")
            || !(file.rel.starts_with("crates/") || file.rel.starts_with("viewer/"))
        {
            continue;
        }
        let text = fs::read(&file.path).map(|b| String::from_utf8_lossy(&b).into_owned());
        let text = text.unwrap_or_default();
        if is_engine_source(&file.rel) {
            sources.push((file.rel.clone(), text.clone()));
        }
        // Only what could hold one: every file names it.
        if text.contains("Diff::new(") {
            tests.push((file.rel, text));
        }
    }
    assert!(sources.len() > 1000, "only {} sources", sources.len());
    assert!(!tests.is_empty());
    let found = typed_function_findings(&sources, &tests);
    assert!(
        found.is_empty(),
        "{} problem(s):\n{}",
        found.len(),
        found
            .iter()
            .map(|(f, l, m)| format!("{f}:{l}: {m}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// The overridable table against the sources.
#[test]
fn overridable_addresses_are_called_by_address() {
    let root = repo_root();
    let table = fs::read_to_string(root.join("crates/engine-core/data/overridable.tsv"))
        .expect("the overridable table");
    let mut files = Vec::new();
    for file in list_files(&root).files {
        if is_engine_source(&file.rel) {
            let text = fs::read(&file.path).map(|b| String::from_utf8_lossy(&b).into_owned());
            files.push((file.rel, text.unwrap_or_default()));
        }
    }
    assert!(files.len() > 1000, "only {} engine sources", files.len());
    let found = typed_cross_file_findings(&table, &files);
    assert!(
        found.is_empty(),
        "{} problem(s):\n{}",
        found.len(),
        found
            .iter()
            .map(|(f, l, m)| format!("{f}:{l}: {m}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

mod typed_rules {
    use super::*;

    const UNIT: &str = "crates/engine-bs/src/units/x/y.rs";
    const TYPED: &str = "crates/engine-core/src/typed/leaf.rs";

    fn marker(address: &str) -> String {
        format!("{}{address}", typed_marker::PREFIX)
    }

    fn messages(rel: &str, text: &str) -> Vec<String> {
        typed_findings(rel, text)
            .into_iter()
            .map(|(line, m)| format!("{line}: {m}"))
            .collect()
    }

    #[test]
    fn code_only_blanks_comments_and_strings_and_keeps_lines() {
        let text = "let a = \"x as i32 // no\"; // y as u8\n/* z\n as u8 */ let b = 'c';\nlet l: &'a str = r#\"q\" as u32\"#;\n'\\n';";
        let code = code_only(text);
        assert_eq!(code.len(), text.len());
        assert_eq!(code.lines().count(), text.lines().count());
        assert!(!code.contains("as i32"));
        assert!(!code.contains("as u8"));
        assert!(!code.contains("as u32"));
        assert!(code.contains("let b = ' ';"));
        assert!(code.contains("&'a str"));
    }

    /// A test module with no body in this file spans one line, not the rest of the file.
    #[test]
    fn a_bodiless_test_module_is_one_line() {
        let code = [
            "#[cfg(test)]",
            "mod tests;",
            "fn real(x: f32) -> i32 {",
            "    x as i32",
            "}",
        ];
        assert_eq!(
            test_module_lines(&code),
            [false, false, false, false, false]
        );
        let found = messages(TYPED, &format!("{}\n", code.join("\n")));
        assert_eq!(
            found.len(),
            1,
            "the rule still reads what follows: {found:?}"
        );
        assert!(found[0].starts_with("4: "), "{found:?}");
        // With a body it spans the body.
        let code = [
            "#[cfg(test)]",
            "mod tests {",
            "    fn t() {}",
            "}",
            "fn real() {}",
        ];
        assert_eq!(test_module_lines(&code), [true, true, true, true, false]);
    }

    #[test]
    fn the_float_cast_rule_fires_on_each_form() {
        let floats: BTreeSet<String> = ["x".to_string(), "self_speed".to_string()].into();
        for line in [
            "let i = x as i32;",
            "let i = (x + 1.0) as i32;",
            "let i = 1.5 as u8;",
            "let i = 2f32 as i64;",
            "let i = e.call(a, &w).f32() as u32;",
            "let i = ret.f64() as i64;",
            "let i = x87_add(a, b) as i32;",
            "let i = y.sqrt() as u16;",
            "let i = f64::from(y) as i32;",
            "let i = self_speed as usize;",
            "let i = (y as f64) as i32;",
        ] {
            assert_eq!(float_casts(line, &floats).len(), 1, "{line}");
        }
        for line in [
            "let i = n as u32;",
            "let i = t.0 as u32;",
            "let i = (n + 1) as u8;",
            "let i = n.len() as u32;",
            "let f = n as f32;",
            "let f = x as f64;",
            "let i = bits(n) as f32;",
        ] {
            assert!(float_casts(line, &floats).is_empty(), "{line}");
        }
    }

    #[test]
    fn float_names_come_from_parameters_and_lets() {
        let code = [
            "fn f(e: &mut Engine, speed: f32, n: u32) -> f64 {",
            "    let half = speed * 0.5;",
            "    let mut sum: f64 = 0.0;",
            "    let count = n + 1;",
            "}",
        ];
        let names = float_names(&code);
        for name in ["speed", "half", "sum"] {
            assert!(names.contains(name), "{name}");
        }
        assert!(!names.contains("n"));
        assert!(!names.contains("count"));
    }

    /// In a typed folder and under a marker the rule fires; in a translation next to it it doesn't.
    #[test]
    fn a_float_cast_is_banned_in_typed_code_only() {
        let body = "fn get(e: &mut Engine, x: f32) -> i32 {\n    x as i32\n}\n";
        assert!(messages(UNIT, body).is_empty(), "a translation may");
        let found = messages(TYPED, body);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].starts_with("2: `x as` an integer in typed code"));

        let marked = format!(
            "fn plain(x: f32) -> i32 {{\n    x as i32\n}}\n\n{}\n/// Doc.\n#[inline]\npub fn typed(x: f32) -> i32 {{\n    x as i32\n}}\nfn after(x: f32) -> i32 {{\n    x as i32\n}}\n",
            marker("00123456")
        );
        let found = messages(UNIT, &marked);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].starts_with("9: "), "{found:?}");
    }

    /// A deliberately wrong stand-in in a test module is how the harness is shown to catch it.
    #[test]
    fn test_modules_may_cast() {
        let body =
            "#[cfg(test)]\nmod tests {\n    fn wrong(x: f32) -> i32 {\n        x as i32\n    }\n}\n";
        assert!(messages(TYPED, body).is_empty());
    }

    /// The type-aware half of the float rule: each typed folder's root denies truncating casts, and
    /// no file lifts that for itself but the harness.
    #[test]
    fn typed_roots_deny_truncating_casts() {
        const ROOT: &str = "crates/engine-core/src/typed/mod.rs";
        let found = messages(ROOT, "pub fn f() {}\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("needs `#![deny(clippy::cast_possible_truncation)]`"));
        assert!(messages(ROOT, &format!("{DENY_CASTS}\n")).is_empty());
        let lifted = "#![allow(clippy::cast_possible_truncation)]\n";
        let found = messages(TYPED, lifted);
        assert!(found[0].contains("lifts the type-aware ban"), "{found:?}");
        assert!(messages("crates/engine-core/src/typed/diff.rs", lifted).is_empty());
        // A local one on a helper is how a meant cut is written.
        assert!(messages(
            TYPED,
            "#[allow(clippy::cast_possible_truncation)]\nfn low(x: u32) -> u8 {\n    x as u8\n}\n"
        )
        .is_empty());
    }

    #[test]
    fn x86_assumptions_are_banned_in_typed_code() {
        for line in [
            "use core::arch::x86_64::_mm_add_ss;",
            "use std::arch::x86_64::*;",
            "#[cfg(target_arch = \"x86_64\")]",
        ] {
            let found = messages(TYPED, &format!("{line}\n"));
            assert!(
                found.len() == 1 && found[0].contains("x86 assumption"),
                "{line}: {found:?}"
            );
            assert!(messages(UNIT, &format!("{line}\n")).is_empty());
        }
    }

    #[test]
    fn decompiled_text_is_banned_in_typed_code_comments_included() {
        for text in [
            "// a #include of something",
            "// reinterpret_cast<NiNode*>(x)",
            "// this->m_field",
            "// see src/staging/run.cpp",
            "// fnv_decomp/src/fallout",
            "// nullptr",
            "// __thiscall",
            "// @fnv 00559450",
            "// NiNode.h",
            "// fallout/ai/actor.h line 3",
            "// fallout/ai/actor.inl",
            "// src\\fallout\\ai\\actor",
            "// fnv_decomp\\src",
        ] {
            let found = messages(TYPED, &format!("{text}\n"));
            assert!(
                !found.is_empty() && found.iter().all(|m| m.contains("in typed code")),
                "{text}: {found:?}"
            );
        }
        for clean in [
            "// read against fnv_decomp\n",
            "// the height is rect.height, 1/3 of it\n",
            "// C:\n",
            "// a\\n in a string\n",
            "let h = rect.h;\n",
        ] {
            assert!(messages(TYPED, clean).is_empty(), "{clean}");
        }
        // A translation's comment is not under this rule (ADR-0009 has its own).
        assert!(messages(UNIT, "// this->m_field\n").is_empty());
    }

    #[test]
    fn placeholder_names_are_banned_in_typed_code() {
        for (line, name) in [
            ("let a = fnv_00559450(e);", "fnv_00559450"),
            ("static g_011dea3c: u32 = 0;", "g_011dea3c"),
            ("struct GlobalObject_01234567;", "GlobalObject_01234567"),
            ("struct Holder00abcdef;", "Holder00abcdef"),
            ("struct Thing_00abcdef_struct;", "Thing_00abcdef_struct"),
            ("struct Thing_00abcdef_class;", "Thing_00abcdef_class"),
            ("struct Thing_00abcdef_shell;", "Thing_00abcdef_shell"),
        ] {
            assert_eq!(placeholder_names(line), [name]);
            let found = messages(TYPED, &format!("{line}\n"));
            assert!(found[0].contains("placeholder name"), "{found:?}");
        }
        // The project's own, literals and bare addresses are not names from elsewhere.
        for line in [
            "pub fn fn_00aa4ee0(e: &mut Engine) {}",
            "use crate::units::x::run_00a7c9f0::thing;",
            "e.call(0x00559450, &[]);",
            "e.call(0x0055_9450, &[]);",
            "// 00559450 is the getter",
        ] {
            assert!(placeholder_names(line).is_empty(), "{line}");
        }
    }

    /// A file with typed functions has its layouts and constants read too, with their comments:
    /// that is where a pasted C++ name or path sits. The rest of a translation's file isn't.
    #[test]
    fn layouts_and_constants_beside_a_typed_function_are_read() {
        let file = |item: &str| {
            format!(
                "{item}\nfn untyped() {{\n    // this->other\n}}\n{}\nfn typed() {{}}\n",
                marker("00123456")
            )
        };
        for item in [
            "const G_011dea3c: u32 = 0;",
            "pub const VALUE: u32 = 1; // from actor.h",
            "// this->m_x\nconst VALUE: u32 = 1;",
            "pub static TABLE: u32 = 1; // @fnv",
            "layout! {\n    Thing_00abcdef_struct, 0x10,\n    field: u32 = 4,\n}",
            "layout! {\n    Thing, 0x10,\n    // this->m_field\n    field: u32 = 4,\n}",
        ] {
            let found = messages(UNIT, &file(item));
            assert!(!found.is_empty(), "{item}");
            assert!(
                found.iter().all(|m| m.contains("in typed code")),
                "{found:?}"
            );
        }
        // A clean item, and a dirty line in a function that isn't typed.
        assert!(messages(UNIT, &file("const VALUE: u32 = 1;")).is_empty());
    }

    /// One definition of a marker: what this test refuses, the ledger doesn't count (the ledger's
    /// own test holds the other half).
    #[test]
    fn markers_must_be_well_formed_and_above_a_function() {
        let good = marker("00123456");
        for ok in [
            good.clone(),
            format!("    {good}"),
            format!("{good}."),
            format!("{good}, read against fnv_decomp"),
            format!("{good}, read against fnv_decomp."),
        ] {
            assert_eq!(typed_marker::address(&ok), Some(0x0012_3456), "{ok}");
        }
        for bad in [
            marker("0012345"),
            marker("00123456 (decompiled)"),
            marker("zzzzzzzz"),
            format!("{good}, from fnv_decomp"),
            format!("/{good}"),
            format!("    /{good}"),
            good.replacen("// ", "//", 1),
            format!("fn x() {{}} {good}"),
        ] {
            assert_eq!(typed_marker::address(&bad), None, "{bad}");
            let found = messages(UNIT, &format!("{bad}\nfn f() {{}}\n"));
            assert!(
                found[0].contains("malformed typed marker"),
                "{bad}: {found:?}"
            );
        }
        let found = messages(UNIT, &format!("{good}\nconst X: u32 = 1;\n"));
        assert!(
            found[0].contains("not directly above a function"),
            "{found:?}"
        );
        assert!(messages(UNIT, &format!("{good}\nfn f() {{}}\n")).is_empty());
    }

    fn findings(rows: &str, list: &[(&str, &str)]) -> Vec<String> {
        let files: Vec<(String, String)> = list
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        typed_cross_file_findings(&format!("# header\n{rows}"), &files)
            .into_iter()
            .map(|(f, l, m)| format!("{f}:{l}: {m}"))
            .collect()
    }

    const NONE: Vec<String> = Vec::new();

    /// In a typed folder every registered function carries its marker, so the ledger counts it. The
    /// marker may be in another of the family's files.
    #[test]
    fn a_typed_folder_registers_only_marked_functions() {
        const TABLE_FILE: &str = "crates/engine-core/src/typed/leaf.rs";
        const OTHER_FILE: &str = "crates/engine-core/src/typed/leaf_more.rs";
        let table = "pub fn funcs() -> Vec<(u32, AbiFn)> {\n    vec![\n        entry!(0x0012_3456, a() -> u32),\n        entry!(\n            0x0012_3460,\n            b() -> u32\n        ),\n    ]\n}\n";
        let functions = format!(
            "{}\npub fn a(e: &mut Engine) -> u32 {{\n    0\n}}\n",
            marker("00123456")
        );
        let found = findings("", &[(TABLE_FILE, table), (OTHER_FILE, &functions)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].contains("typed function 00123460 (`b`) registered without its marker"),
            "{found:?}"
        );
        let both = format!(
            "{functions}{}\npub fn b(e: &mut Engine) -> u32 {{\n    0\n}}\n",
            marker("00123460")
        );
        assert_eq!(
            findings("", &[(TABLE_FILE, table), (OTHER_FILE, &both)]),
            NONE
        );
        // A translation's unit isn't under the rule.
        assert_eq!(findings("", &[(UNIT, table)]), NONE);
    }

    const OWNER: &str = "pub fn hooked(e: &mut Engine) {}\npub fn caller(e: &mut Engine) {\n    hooked(e);\n}\npub fn funcs() -> Vec<(u32, AbiFn)> {\n    vec![entry!(0x0012_3450, hooked()), entry!(0x0012_3460, caller())]\n}\n#[cfg(test)]\nmod tests {\n    fn t(e: &mut Engine) { super::hooked(e); }\n}\n";

    #[test]
    fn the_translations_direct_calls_stay_at_the_tables_count() {
        const ROW_1: &str = "00123450\t1\tJIP\n";
        const ROW_2: &str = "00123450\t2\tJIP\n";
        // One call outside tests, in the same file: the count matches.
        assert_eq!(findings(ROW_1, &[("a.rs", OWNER)]), NONE);
        // A new direct call fails and says where.
        let found = findings("00123450\t0\tJIP\n", &[("a.rs", OWNER)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].contains("has 1 direct call(s), the table allows 0"),
            "{found:?}"
        );
        assert!(found[0].contains("a.rs:3"), "{found:?}");
        // Removing one fails until the table is lowered.
        let found = findings(ROW_2, &[("a.rs", OWNER)]);
        assert!(found[0].contains("lower the table's number"), "{found:?}");
        // Another file: a path counts, a bare name it imports, one through a glob or an alias.
        for other in [
            "fn f(e: &mut Engine) {\n    crate::units::a::hooked(e);\n}\n",
            "use super::a::hooked;\nfn f(e: &mut Engine) {\n    hooked(e);\n}\n",
            "use super::a::*;\nfn f(e: &mut Engine) {\n    hooked(e);\n}\n",
            "use super::a::hooked as h;\nfn f(e: &mut Engine) {\n    h(e);\n}\n",
            "use super::a::{caller, hooked as h};\nfn f(e: &mut Engine) {\n    h(e);\n}\n",
        ] {
            assert_eq!(
                findings(ROW_2, &[("a.rs", OWNER), ("b.rs", other)]),
                NONE,
                "{other}"
            );
        }
        // A method, an associated function, the library's, its own function or local of that name,
        // a comment and a string don't.
        for other in [
            "fn hooked() {}\nfn f(e: &mut Engine) {\n    hooked();\n    e.hooked();\n}\n",
            "fn f(e: &mut Engine) {\n    e.hooked();\n    Thing::hooked(e);\n    std::mem::hooked(e);\n}\n",
            "fn f() {\n    let hooked = || 1;\n    hooked();\n}\n",
            "// hooked(e)\nfn f() { let s = \"hooked(e)\"; }\n",
        ] {
            assert_eq!(findings(ROW_1, &[("a.rs", OWNER), ("b.rs", other)]), NONE, "{other}");
        }
    }

    #[test]
    fn typed_code_has_no_direct_call_of_an_overridable_function() {
        let typed = format!(
            "{}\npub fn t(e: &mut Engine) {{\n    hooked(e);\n}}\n",
            marker("00123470")
        );
        let owner = format!("{typed}{OWNER}");
        // The count allows the translation's one; the typed caller fails all the same.
        let found = findings("00123450\t1\tJIP\n", &[("a.rs", &owner)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].contains("direct call of overridable 00123450 (`hooked`) in typed code"),
            "{found:?}"
        );
        for leaf in [
            "use super::a::hooked;\nfn t(e: &mut Engine) {\n    hooked(e);\n}\n",
            "use super::a::*;\nfn t(e: &mut Engine) {\n    hooked(e);\n}\n",
            "use super::a::hooked as h;\nfn t(e: &mut Engine) {\n    h(e);\n}\n",
            "fn t(e: &mut Engine) {\n    super::a::hooked(e);\n}\n",
        ] {
            let found = findings(
                "00123450\t1\tJIP\n",
                &[
                    ("a.rs", OWNER),
                    ("crates/engine-core/src/typed/leaf.rs", leaf),
                ],
            );
            assert_eq!(found.len(), 1, "{leaf}: {found:?}");
            assert!(found[0].contains("in typed code"), "{found:?}");
        }
    }

    /// A plain Rust call of a registered function from typed code skips the flag, doubles and
    /// diagnostics. A helper that nothing registers is ordinary code.
    #[test]
    fn typed_code_has_no_plain_call_of_a_registered_function() {
        const LEAF: &str = "crates/engine-core/src/typed/leaf.rs";
        let leaf = format!(
            "{}\npub fn outer(e: &mut Engine) -> u32 {{\n    helper(e) + inner(e)\n}}\n{}\npub fn inner(e: &mut Engine) -> u32 {{\n    0\n}}\nfn helper(e: &mut Engine) -> u32 {{\n    1\n}}\npub fn funcs() -> Vec<(u32, AbiFn)> {{\n    vec![entry!(0x0012_3450, outer() -> u32), entry!(0x0012_3460, inner() -> u32)]\n}}\n",
            marker("00123450"),
            marker("00123460")
        );
        let found = findings("", &[(LEAF, &leaf)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].starts_with(&format!("{LEAF}:3: plain call of registered `inner`")),
            "{found:?}"
        );
        // Through typed_call! it is fine.
        let fixed = leaf.replace(
            "helper(e) + inner(e)",
            "helper(e) + typed_call!(LEAF, 0x0012_3460, inner(e))",
        );
        assert_eq!(findings("", &[(LEAF, &fixed)]), NONE);
        // A translation's unit may: only typed lines are read. A marked function there may not.
        let unit = "pub fn get(e: &mut Engine) -> u32 {\n    0\n}\npub fn outer(e: &mut Engine) -> u32 {\n    get(e)\n}\npub fn funcs() -> Vec<(u32, AbiFn)> {\n    vec![entry!(0x0012_3470, get() -> u32)]\n}\n";
        assert_eq!(findings("", &[(UNIT, unit)]), NONE);
        let marked = unit.replace(
            "pub fn outer",
            &format!("{}\npub fn outer", marker("00123480")),
        );
        let found = findings("", &[(UNIT, &marked)]);
        assert!(
            found[0].contains("plain call of registered `get`"),
            "{found:?}"
        );
        // A global that is also a local helper's name elsewhere, defined here, is this file's.
        let helper = "fn get(e: &mut Engine) -> u32 {\n    0\n}\nfn t(e: &mut Engine) -> u32 {\n    get(e)\n}\n";
        assert_eq!(findings("", &[(UNIT, unit), (LEAF, helper)]), NONE);
    }

    #[test]
    fn typed_call_is_tied_to_the_function_registered_there() {
        let table = format!(
            "{}\npub fn other(e: &mut Engine) {{}}\npub fn funcs() -> Vec<(u32, AbiFn)> {{\n    vec![entry!(0x0012_3460, other())]\n}}\n",
            marker("00123460")
        );
        let table = table.as_str();
        // Right.
        let right = "fn f(e: &mut Engine) {\n    let y: u32 = typed_call!(LEAF, 0x0012_3460, leaf::other(e));\n}\n";
        assert_eq!(findings("", &[("leaf.rs", table), ("a.rs", right)]), NONE);
        // The wrong function for the address: it would run in play.
        let wrong = "fn f(e: &mut Engine) {\n    let y: u32 = typed_call!(LEAF, 0x0012_3460, leaf::another(e));\n}\n";
        let found = findings("", &[("leaf.rs", table), ("a.rs", wrong)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("another` directly"), "{found:?}");
        assert!(found[0].contains("registered as leaf::other"), "{found:?}");
        // An address nothing is registered at.
        let found = findings("", &[("a.rs", right)]);
        assert!(found[0].contains("registered as nothing"), "{found:?}");
        // Every bracket form is read.
        for form in ["typed_call![", "typed_call!{", "typed_call! ("] {
            let close = if form.ends_with('[') {
                "]"
            } else if form.ends_with('{') {
                "}"
            } else {
                ")"
            };
            let text = format!(
                "fn f(e: &mut Engine) {{\n    {form}LEAF, 0x0012_3460, leaf::another(e){close};\n}}\n"
            );
            let found = findings("", &[("leaf.rs", table), ("a.rs", &text)]);
            assert_eq!(found.len(), 1, "{form}: {found:?}");
            assert!(found[0].contains("another` directly"), "{found:?}");
        }
    }

    #[test]
    fn typed_call_of_an_overridable_address_is_banned() {
        let row = "00123450\t0\tJIP\n";
        let table = format!(
            "{}\npub fn other(e: &mut Engine) {{}}\npub fn funcs() -> Vec<(u32, AbiFn)> {{\n    vec![entry!(0x0012_3450, get()), entry!(0x0012_3460, other())]\n}}\n",
            marker("00123460")
        );
        let table = table.as_str();
        let site = "fn f(e: &mut Engine) {\n    let x: u32 = typed_call!(LEAF, 0x0012_3450, leaf::get(e, 1));\n    let y: u32 = typed_call!(LEAF, 0x0012_3460, leaf::other(e));\n}\n";
        let found = findings(row, &[("leaf.rs", table), ("a.rs", site)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].starts_with("a.rs:2: typed_call! of overridable 00123450"),
            "{found:?}"
        );
        for form in [
            "typed_call![LEAF, 0x0012_3450, leaf::get(e)]",
            "typed_call!{LEAF, 0x0012_3450, leaf::get(e)}",
        ] {
            let text = format!("fn f(e: &mut Engine) {{\n    {form};\n}}\n");
            let found = findings(row, &[("leaf.rs", table), ("a.rs", &text)]);
            assert!(found[0].contains("of overridable 00123450"), "{found:?}");
        }
        // A name for the address hides it from this test.
        let named =
            "fn f(e: &mut Engine) {\n    let x: u32 = typed_call!(LEAF, HOOKED, leaf::get(e));\n}\n";
        let found = findings(row, &[("leaf.rs", table), ("a.rs", named)]);
        assert!(
            found[0].contains("write the address as a literal"),
            "{found:?}"
        );
        // A test module and the macro's own arms are left alone.
        let test = "#[cfg(test)]\nmod tests {\n    fn f(e: &mut Engine) {\n        typed_call!(F, A, g(e));\n    }\n}\n";
        assert_eq!(findings(row, &[("leaf.rs", table), ("a.rs", test)]), NONE);
        let arm = "macro_rules! typed_call {\n    ($f:expr, $addr:expr) => { $crate::typed_call!(@bind $f, $addr) };\n}\n";
        assert_eq!(findings(row, &[("leaf.rs", table), ("a.rs", arm)]), NONE);
    }

    #[test]
    fn the_table_is_checked() {
        assert_eq!(
            parse_overridable("# c\n\n00123450\t3\tJIP\n00123460\t0\tJG,kNVSE\n"),
            Ok(vec![
                Overridable {
                    line: 3,
                    address: 0x0012_3450,
                    direct_calls: 3
                },
                Overridable {
                    line: 4,
                    address: 0x0012_3460,
                    direct_calls: 0
                },
            ])
        );
        for bad in [
            "123450\t0\tJIP",
            "00123450\tx\tJIP",
            "00123450\t0",
            "0012345z\t0\tJIP",
            "00123460\t0\tJIP\n00123450\t0\tJIP",
            "00123450\t0\tJIP\n00123450\t0\tJIP",
        ] {
            assert!(parse_overridable(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn raw_registrations_are_read_too() {
        let code =
            "vec![(0x0012_3450, parse_entry as AbiFn), entry!(0x0012_3460, x::y::name(u32) -> u32)]";
        assert_eq!(
            registered_paths(code),
            [
                (0x0012_3460, "x::y::name".to_string(), 0),
                (0x0012_3450, "parse_entry".to_string(), 0)
            ]
        );
    }

    // ---- The second review -----------------------------------------------------------------

    /// What is cast and isn't a float any more isn't a finding.
    #[test]
    fn the_float_cast_rule_reads_what_the_operand_ends_as() {
        let floats: BTreeSet<String> = ["x".to_string(), "a".to_string(), "b".to_string()].into();
        for line in [
            "let i = x.to_bits() as i32;",
            "let i = (a < b) as u32;",
            "let i = (a == b) as u8;",
            "let i = (a >= b) as u32;",
            "let i = x.is_nan() as u32;",
            "let i = f32::is_nan(x) as u32;",
        ] {
            assert!(float_casts(line, &floats).is_empty(), "{line}");
        }
        for line in ["let i = (x + a) as u32;", "let i = (x - 1.0) as i32;"] {
            assert_eq!(float_casts(line, &floats).len(), 1, "{line}");
        }
    }

    /// However the truncating-cast lint is lifted for a whole file or module: indented, combined with
    /// others, across lines, through its group, or on a `mod`.
    #[test]
    fn lifting_the_cast_lint_is_seen_in_each_form() {
        for text in [
            "    #![allow(clippy::cast_possible_truncation)]\n",
            "#![allow(dead_code, clippy::cast_possible_truncation)]\n",
            "#![allow(\n    dead_code,\n    clippy::cast_possible_truncation,\n)]\n",
            "#![allow(clippy::pedantic)]\n",
            "#[allow(clippy::cast_possible_truncation)]\nmod inner {\n}\n",
            "#[allow(clippy::cast_possible_truncation)]\npub mod inner {\n}\n",
        ] {
            let found = messages(TYPED, text);
            assert!(
                found.iter().any(|m| m.contains("lifts the type-aware ban")),
                "{text}: {found:?}"
            );
        }
        // A test module's own, and a local one on a helper.
        for text in [
            "#[cfg(test)]\nmod tests {\n    #![allow(clippy::cast_possible_truncation)]\n}\n",
            "#[allow(clippy::cast_possible_truncation)]\nfn low(x: u32) -> u8 {\n    x as u8\n}\n",
        ] {
            assert!(messages(TYPED, text).is_empty(), "{text}");
        }
    }

    #[test]
    fn a_use_line_is_read_in_every_form() {
        let imports = file_imports(
            "use a::b::plain;\npub use c::{d, e as f, g::{h, i::*}};\nuse super::*;\nuse j::k::{self, l as m};\nuse n::o as p;\nuse q::r as _;\nfn not_use() {}\n",
        );
        assert_eq!(imports.names["plain"], ["a", "b", "plain"]);
        assert_eq!(imports.names["d"], ["c", "d"]);
        assert_eq!(imports.names["f"], ["c", "e"]);
        assert_eq!(imports.names["h"], ["c", "g", "h"]);
        assert_eq!(imports.names["k"], ["j", "k"]);
        assert_eq!(imports.names["m"], ["j", "k", "l"]);
        assert_eq!(imports.names["p"], ["n", "o"]);
        assert!(!imports.names.contains_key("_"));
        assert_eq!(imports.globs, ["i", "super"]);
    }

    /// The tie reads the path: another module's function of the same name, or a rename, isn't what
    /// is registered there.
    #[test]
    fn typed_call_is_tied_through_paths_and_renames() {
        let registered = format!(
            "{}\npub fn construct(e: &mut Engine) {{}}\n{}\npub fn other(e: &mut Engine) {{}}\npub fn funcs() -> Vec<(u32, AbiFn)> {{\n    vec![entry!(0x0012_3460, construct()), entry!(0x0012_3470, other())]\n}}\n",
            marker("00123460"),
            marker("00123470")
        );
        const FILE: &str = "crates/engine-core/src/typed/leaf.rs";
        let site = |text: &str| findings("", &[(FILE, &registered), ("a.rs", text)]);
        for right in [
            "fn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, leaf::construct(e));\n}\n",
            "fn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, typed::leaf::construct(e));\n}\n",
            "use typed::leaf::construct;\nfn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, construct(e));\n}\n",
            "use typed::leaf::*;\nfn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, construct(e));\n}\n",
            "use typed::leaf as l;\nfn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, l::construct(e));\n}\n",
            "use typed::leaf::construct as make;\nfn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, make(e));\n}\n",
        ] {
            assert_eq!(site(right), NONE, "{right}");
        }
        for wrong in [
            // Another module's function of the same name: 75 names are registered more than once.
            "fn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, other_module::construct(e));\n}\n",
            // A rename hiding another function.
            "use typed::leaf::other as construct;\nfn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, construct(e));\n}\n",
            // A bare name this file can't have from there.
            "fn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, construct(e));\n}\n",
            // The right module, the wrong function for the address.
            "fn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, leaf::other(e));\n}\n",
        ] {
            let found = site(wrong);
            assert_eq!(found.len(), 1, "{wrong}: {found:?}");
            assert!(found[0].contains("the wrong function would run"), "{found:?}");
        }
    }

    /// The translation registered at an address, called directly: its name and address agree, and it
    /// has no typed marker, so the call would silently skip the typed version.
    #[test]
    fn typed_call_of_a_translation_is_refused() {
        let unit = "pub fn fn_00123460(e: &mut Engine) {}\npub fn funcs() -> Vec<(u32, AbiFn)> {\n    vec![entry!(0x0012_3460, fn_00123460())]\n}\n";
        let caller = "use unit::fn_00123460;\nfn f(e: &mut Engine) {\n    typed_call!(LEAF, 0x0012_3460, fn_00123460(e));\n}\n";
        let found = findings("", &[("unit.rs", unit), ("a.rs", caller)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("has no typed marker"), "{found:?}");
        // With its marker for another address, the same.
        let marked = unit.replace(
            "pub fn fn_00123460",
            &format!("{}\npub fn fn_00123460", marker("00123999")),
        );
        let found = findings("", &[("unit.rs", &marked), ("a.rs", caller)]);
        assert!(found[0].contains("has no typed marker"), "{found:?}");
    }

    /// A marker anywhere in the folder isn't enough: it has to be for that function.
    #[test]
    fn a_registered_typed_function_needs_its_own_marker() {
        const LEAF: &str = "crates/engine-core/src/typed/leaf.rs";
        let text = format!(
            "{}\npub fn a(e: &mut Engine) {{}}\npub fn b(e: &mut Engine) {{}}\npub fn funcs() -> Vec<(u32, AbiFn)> {{\n    vec![entry!(0x0012_3450, b())]\n}}\n",
            marker("00123450")
        );
        let found = findings("", &[(LEAF, &text)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].contains("(`b`) registered without its marker"),
            "{found:?}"
        );
    }

    /// Helpers named like a registered function aren't it.
    #[test]
    fn a_helper_named_like_a_registered_function_is_no_finding() {
        const LEAF: &str = "crates/engine-core/src/typed/leaf.rs";
        let elsewhere = "pub fn read(e: &mut Engine) {}\npub fn sqrt(e: &mut Engine) {}\npub fn funcs() -> Vec<(u32, AbiFn)> {\n    vec![entry!(0x0012_3450, read()), entry!(0x0012_3460, sqrt())]\n}\n";
        for clean in [
            "fn t(e: &mut Engine) {\n    helpers::read(e);\n}\n",
            "fn t(x: f32) -> f32 {\n    f32::sqrt(x)\n}\n",
            "use helpers::read;\nfn t(e: &mut Engine) {\n    read(e);\n}\n",
            "use helpers::read as get;\nfn t(e: &mut Engine) {\n    get(e);\n}\n",
        ] {
            assert_eq!(
                findings("", &[("unit.rs", elsewhere), (LEAF, clean)]),
                NONE,
                "{clean}"
            );
        }
        // From its own module it is the function.
        let plain = "fn t(e: &mut Engine) {\n    unit::read(e);\n}\n";
        let found = findings("", &[("unit.rs", elsewhere), (LEAF, plain)]);
        assert!(
            found[0].contains("plain call of registered `read`"),
            "{found:?}"
        );
    }

    /// The program around the engine counts too: `crates/host` and `viewer` can make direct calls.
    #[test]
    fn the_host_and_the_viewer_are_read() {
        assert!(is_engine_source("crates/host/src/uncap.rs"));
        assert!(is_engine_source("viewer/src/main.rs"));
        assert!(is_engine_source("crates/engine-bs/src/lib.rs"));
        assert!(!is_engine_source("crates/host/tests/live.rs"));
        assert!(!is_engine_source("crates/esm/src/lib.rs"));
    }

    fn typed_function_messages(sources: &[(&str, &str)], tests: &[(&str, &str)]) -> Vec<String> {
        let list = |l: &[(&str, &str)]| -> Vec<(String, String)> {
            l.iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect()
        };
        typed_function_findings(&list(sources), &list(tests))
            .into_iter()
            .map(|(f, l, m)| format!("{f}:{l}: {m}"))
            .collect()
    }

    #[test]
    fn every_typed_function_is_registered_and_has_a_differential_test() {
        let source = format!(
            "{}\npub fn get(e: &mut Engine) -> u32 {{\n    0\n}}\npub fn funcs() -> Vec<(u32, AbiFn)> {{\n    vec![entry!(0x0012_3450, get() -> u32)]\n}}\n",
            marker("00123450")
        );
        let source = source.as_str();
        // No test.
        let found = typed_function_messages(&[("a.rs", source)], &[]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].contains("00123450 (`get`) has no differential test"),
            "{found:?}"
        );
        // A test in any of the forms the harness is written in.
        for test in [
            "Diff::new(entry!(0x0012_3450, get() -> u32)).run().assert_same();",
            "Diff::new(crate::entry!(0x0012_3450, get() -> u32)).run().assert_same();",
            "Diff::new(engine_core::entry!(\n    0x00123450,\n    get() -> u32,\n)).run().assert_same();",
            "Diff::new((0x0012_3450, f)).run().assert_same();",
        ] {
            assert_eq!(
                typed_function_messages(&[("a.rs", source)], &[("t.rs", test)]),
                NONE,
                "{test}"
            );
        }
        // A test of something else, or a comment.
        for test in [
            "Diff::new(entry!(0x0012_3460, get() -> u32)).run().assert_same();",
            "// Diff::new(entry!(0x0012_3450, get() -> u32))",
        ] {
            let found = typed_function_messages(&[("a.rs", source)], &[("t.rs", test)]);
            assert_eq!(found.len(), 1, "{test}: {found:?}");
        }
        // A marker with no registration credits nothing.
        let unregistered = format!("{}\npub fn get(e: &mut Engine) {{}}\n", marker("00123450"));
        let test = "Diff::new((0x0012_3450, f)).run().assert_same();";
        let found = typed_function_messages(&[("a.rs", &unregistered)], &[("t.rs", test)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("isn't registered there"), "{found:?}");
    }

    /// The same function on both sides, each through its own `entry!`: the harness sees two functions.
    #[test]
    fn a_test_comparing_a_function_with_itself_is_refused() {
        let test = "fn t() {\n    Diff::new(entry!(0x0012_3450, get() -> u32))\n        .translation(entry!(0x0012_3450, get() -> u32).1)\n        .run();\n}\n";
        let found = typed_function_messages(&[], &[("t.rs", test)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("compares `get` with itself"), "{found:?}");
        let other = test.replace(
            ".translation(entry!(0x0012_3450, get()",
            ".translation(entry!(0x0012_3450, get_translated()",
        );
        assert_eq!(typed_function_messages(&[], &[("t.rs", &other)]), NONE);
        // Two tests in a row are two statements.
        let two = "fn t() {\n    let _ = Diff::new(entry!(0x0012_3450, a()));\n    let _ = Diff::new(entry!(0x0012_3450, b())).translation(entry!(0x0012_3450, a()).1);\n}\n";
        assert_eq!(typed_function_messages(&[], &[("t.rs", two)]), NONE);
        // The harness's own tests do it on purpose.
        assert_eq!(
            typed_function_messages(&[], &[("crates/engine-core/src/typed/diff.rs", test)]),
            NONE
        );
    }
}
