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
//!   `[011d8a84]`, and anything else in plain words.
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

use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

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
