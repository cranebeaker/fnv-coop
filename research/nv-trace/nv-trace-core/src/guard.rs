//! The decisions of the recorder's player-folder guard, kept here so they are unit-tested on
//! any host. The DLL hooks `NtCreateFile`, `NtOpenFile` and `NtSetInformationFile` (every
//! file creation, open, rename, link and delete in the process passes through them) and asks:
//! does this open intend to change anything ([`writes`])? does the path lie inside a
//! protected folder ([`Protected::contains`])? Both yes: the call is refused with
//! `STATUS_ACCESS_DENIED` before it reaches the file system, and logged.

/// `ACCESS_MASK` bits that allow a change: `FILE_WRITE_DATA`/`FILE_ADD_FILE`,
/// `FILE_APPEND_DATA`/`FILE_ADD_SUBDIRECTORY`, `FILE_WRITE_EA`, `FILE_DELETE_CHILD`,
/// `FILE_WRITE_ATTRIBUTES`, `DELETE`, `WRITE_DAC`, `WRITE_OWNER`, `MAXIMUM_ALLOWED`,
/// `GENERIC_ALL`, `GENERIC_WRITE`.
pub const WRITE_ACCESS: u32 = 0x0000_0002
    | 0x0000_0004
    | 0x0000_0010
    | 0x0000_0040
    | 0x0000_0100
    | 0x0001_0000
    | 0x0004_0000
    | 0x0008_0000
    | 0x0200_0000
    | 0x1000_0000
    | 0x4000_0000;

/// `FILE_OPEN`: the only `NtCreateFile` disposition that never creates or truncates.
pub const FILE_OPEN: u32 = 1;
/// `FILE_DELETE_ON_CLOSE` in the create options.
pub const FILE_DELETE_ON_CLOSE: u32 = 0x1000;

/// Whether an open with these values can change the file system: write-class access, any
/// disposition but `FILE_OPEN` (`None` for `NtOpenFile`, which only opens), or delete on
/// close.
pub fn writes(access: u32, disposition: Option<u32>, options: u32) -> bool {
    access & WRITE_ACCESS != 0
        || disposition.is_some_and(|d| d != FILE_OPEN)
        || options & FILE_DELETE_ON_CLOSE != 0
}

/// `FILE_INFORMATION_CLASS` values of `NtSetInformationFile` that name a second path: rename
/// and hard link (and their `Ex` forms). Their target must not be protected either.
pub fn names_target(class: u32) -> bool {
    matches!(class, 10 | 11 | 65 | 72)
}

/// A path in one comparable form: Win32 or NT prefixes removed (`\??\`, `\\?\`, `\\.\`,
/// `\DosDevices\`, `\GLOBAL??\`), separators as `\`, `.` and `..` resolved, no trailing
/// separator, ASCII lowercase (NTFS compares names case-insensitively; non-ASCII letters are
/// left alone, which only makes the check stricter for names that differ in them).
pub fn normalize(path: &str) -> String {
    let mut p = path.replace('/', "\\");
    for prefix in [
        "\\??\\",
        "\\\\?\\",
        "\\\\.\\",
        "\\DosDevices\\",
        "\\GLOBAL??\\",
    ] {
        if p.len() >= prefix.len() && p[..prefix.len()].eq_ignore_ascii_case(prefix) {
            p = p[prefix.len()..].to_string();
            break;
        }
    }
    if p.len() >= 4 && p[..4].eq_ignore_ascii_case("UNC\\") {
        p = format!("\\\\{}", &p[4..]);
    }
    let unc = p.starts_with("\\\\");
    let mut parts: Vec<&str> = Vec::new();
    for part in p.split('\\') {
        match part {
            "" | "." => {}
            ".." => {
                if parts.len() > 1 {
                    parts.pop();
                }
            }
            other => parts.push(other),
        }
    }
    let joined = parts.join("\\");
    let joined = if unc { format!("\\\\{joined}") } else { joined };
    joined.to_ascii_lowercase()
}

/// `name` (relative) appended to the folder `root`, then normalised.
pub fn join(root: &str, name: &str) -> String {
    normalize(&format!("{root}\\{name}"))
}

/// The protected folders, normalised.
#[derive(Clone, Debug, Default)]
pub struct Protected {
    roots: Vec<String>,
}

impl Protected {
    pub fn new<'a>(roots: impl IntoIterator<Item = &'a str>) -> Protected {
        Protected {
            roots: roots
                .into_iter()
                .map(normalize)
                .filter(|r| !r.is_empty())
                .collect(),
        }
    }

    pub fn roots(&self) -> &[String] {
        &self.roots
    }

    /// Whether the normalised `path` is a protected folder or lies inside one.
    pub fn contains(&self, normalized: &str) -> bool {
        self.roots.iter().any(|r| {
            normalized == r
                || (normalized.len() > r.len()
                    && normalized.starts_with(r.as_str())
                    && normalized.as_bytes()[r.len()] == b'\\')
        })
    }

    /// Whether the normalised `path` is an ancestor of a protected folder (a rename or
    /// delete of it would move or remove the protected folder too).
    pub fn is_ancestor(&self, normalized: &str) -> bool {
        self.roots.iter().any(|r| {
            r.len() > normalized.len()
                && r.starts_with(normalized)
                && r.as_bytes()[normalized.len()] == b'\\'
        })
    }

    /// The check for a changing open: inside a protected folder, or an ancestor of one.
    pub fn refuses(&self, normalized: &str) -> bool {
        self.contains(normalized) || self.is_ancestor(normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAME: &str = "C:\\Games\\Steam\\steamapps\\common\\Fallout New Vegas";

    #[test]
    fn nt_and_win32_forms_normalise_alike() {
        let want = "c:\\games\\steam\\steamapps\\common\\fallout new vegas\\data\\x.esp";
        for p in [
            "\\??\\C:\\Games\\Steam\\steamapps\\common\\Fallout New Vegas\\Data\\x.esp",
            "\\\\?\\C:\\Games\\Steam\\steamapps\\common\\Fallout New Vegas\\Data\\x.esp",
            "C:/Games/Steam/steamapps/common/Fallout New Vegas/Data/x.esp",
            "C:\\Games\\Steam\\steamapps\\common\\Fallout New Vegas\\Data\\..\\Data\\.\\x.esp",
            "c:\\games\\steam\\STEAMAPPS\\common\\fallout new vegas\\\\data\\x.esp",
        ] {
            assert_eq!(normalize(p), want, "{p}");
        }
        assert_eq!(normalize("\\??\\UNC\\srv\\share\\a"), "\\\\srv\\share\\a");
    }

    #[test]
    fn inside_and_beside_a_protected_folder() {
        let p = Protected::new([GAME, "C:\\Users\\<name>\\AppData\\Local\\FalloutNV"]);
        assert!(p.contains(&normalize(GAME)));
        assert!(p.contains(&normalize(&format!("{GAME}\\Data\\x.esp"))));
        assert!(p.contains(&join(GAME, "..\\Fallout New Vegas\\FalloutNV.log")));
        // A sibling whose name starts the same is not inside.
        assert!(!p.contains(&normalize(
            "C:\\Games\\Steam\\steamapps\\common\\Fallout New Vegas2\\a"
        )));
        assert!(!p.contains(&normalize("C:\\Users\\<name>\\nv-re\\work\\x")));
        // Renaming or deleting a parent would take the folder with it.
        assert!(p.refuses(&normalize("C:\\Games\\Steam\\steamapps\\common")));
        assert!(!p.refuses(&normalize("C:\\Games\\Steam\\steamapps\\commo")));
    }

    #[test]
    fn what_counts_as_a_change() {
        // GENERIC_READ | SYNCHRONIZE, FILE_OPEN: a read.
        assert!(!writes(0x8010_0000, Some(1), 0x20));
        // NtOpenFile with read access.
        assert!(!writes(0x0012_0089, None, 0));
        // GENERIC_WRITE.
        assert!(writes(0x4000_0000, Some(1), 0));
        // FILE_OVERWRITE_IF with only read access still truncates.
        assert!(writes(0x8000_0000, Some(5), 0));
        // DELETE, FILE_WRITE_ATTRIBUTES, delete on close, MAXIMUM_ALLOWED.
        assert!(writes(0x0001_0000, None, 0));
        assert!(writes(0x0000_0100, None, 0));
        assert!(writes(0x8000_0000, None, FILE_DELETE_ON_CLOSE));
        assert!(writes(0x0200_0000, None, 0));
        assert!(names_target(10) && names_target(11) && !names_target(13));
    }
}
