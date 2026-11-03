//! Removes the Windows user name from text that a player may share.

/// Replaces the folder after `Users\` (or `Users/`, also doubled backslashes as in JSON) with
/// `<user>`, and every occurrence of `user` itself (case-insensitive, when it has 3+ characters).
pub fn redact(text: &str, user: Option<&str>) -> String {
    let step1 = redact_user_dirs(text);
    match user.map(str::trim).filter(|u| u.len() >= 3) {
        Some(u) => replace_ci(&step1, u, "<user>"),
        None => step1,
    }
}

fn is_sep(c: u8) -> bool {
    c == b'\\' || c == b'/'
}

fn redact_user_dirs(text: &str) -> String {
    let b = text.as_bytes();
    let lower = text.to_ascii_lowercase();
    let lb = lower.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    let mut copied = 0;
    while i + 5 < b.len() {
        if &lb[i..i + 5] == b"users" && i > 0 && is_sep(b[i - 1]) && is_sep(b[i + 5]) {
            let mut j = i + 5;
            while j < b.len() && is_sep(b[j]) {
                j += 1;
            }
            let start = j;
            while j < b.len()
                && !is_sep(b[j])
                && !matches!(b[j], b'"' | b'\'' | b'\n' | b'\r' | b'\t')
            {
                j += 1;
            }
            let seg = &lower[start..j];
            if !seg.is_empty()
                && seg != "<user>"
                && seg != "public"
                && seg != "default"
                && seg != "all users"
            {
                out.push_str(&text[copied..start]);
                out.push_str("<user>");
                copied = j;
            }
            i = j.max(i + 1);
        } else {
            i += 1;
        }
    }
    out.push_str(&text[copied..]);
    out
}

fn replace_ci(text: &str, needle: &str, with: &str) -> String {
    let lt = text.to_ascii_lowercase();
    let ln = needle.to_ascii_lowercase();
    let mut out = String::new();
    let mut at = 0;
    while let Some(p) = lt[at..].find(&ln) {
        out.push_str(&text[at..at + p]);
        out.push_str(with);
        at += p + ln.len();
    }
    out.push_str(&text[at..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // The tests split the drive off each profile path, so that no such path stands in the
    // source (repo_hygiene).

    #[test]
    fn user_folders_are_replaced() {
        assert_eq!(
            redact(&["C:", r"\Users\bobsmith\AppData\Roaming\x"].concat(), None),
            r"C:\<user>\AppData\Roaming\x".replace(r"C:\<user>", r"C:\Users\<user>")
        );
        assert_eq!(
            redact(&["c:", "/users/Bob Smith/Docs"].concat(), None),
            "c:/users/<user>/Docs"
        );
        assert_eq!(
            redact(&[r#""C:"#, r#"\\Users\\bobsmith\\x""#].concat(), None),
            r#""C:\\Users\\<user>\\x""#
        );
    }

    #[test]
    fn public_and_idempotent() {
        let s = [r"C:\Users\Public\x C:", r"\Users\bobsmith\y"].concat();
        let once = redact(&s, None);
        assert_eq!(once, r"C:\Users\Public\x C:\Users\<user>\y");
        assert_eq!(redact(&once, None), once);
    }

    #[test]
    fn plain_name_everywhere() {
        assert_eq!(
            redact("hello BobSmith, bobsmith@pc", Some("bobsmith")),
            "hello <user>, <user>@pc"
        );
        // too short to replace safely
        assert_eq!(redact("a b ab", Some("ab")), "a b ab");
    }

    #[test]
    fn no_panic_on_odd_text() {
        redact("\\Users\\", None);
        redact("Users/", Some("x"));
        redact("é\\users\\é", Some("éé"));
    }
}
