//! Windows command-line quoting, so a program started with `--launch`
//! receives exactly the arguments that were given.

/// Quote one argument the way `CommandLineToArgvW` expects to read it back.
pub fn quote(arg: &str) -> String {
    if !arg.is_empty() && !arg.contains([' ', '\t', '\n', '\u{b}', '"']) {
        return arg.to_string();
    }
    let mut out = String::from("\"");
    let mut backslashes = 0usize;
    for c in arg.chars() {
        match c {
            '\\' => backslashes += 1,
            '"' => {
                out.extend(std::iter::repeat_n('\\', backslashes * 2 + 1));
                out.push('"');
                backslashes = 0;
            }
            c => {
                out.extend(std::iter::repeat_n('\\', backslashes));
                backslashes = 0;
                out.push(c);
            }
        }
    }
    // Backslashes before the closing quote must be doubled.
    out.extend(std::iter::repeat_n('\\', backslashes * 2));
    out.push('"');
    out
}

/// Join a program and its arguments into one command line.
pub fn command_line(program: &str, args: &[String]) -> String {
    let mut line = quote(program);
    for a in args {
        line.push(' ');
        line.push_str(&quote(a));
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_arguments_are_left_alone() {
        assert_eq!(quote("loop"), "loop");
        assert_eq!(quote("C:\\Games\\a.exe"), "C:\\Games\\a.exe");
    }

    #[test]
    fn spaces_and_empty_arguments_are_quoted() {
        assert_eq!(quote("a b"), "\"a b\"");
        assert_eq!(quote(""), "\"\"");
        assert_eq!(quote("C:\\Program Files\\x"), "\"C:\\Program Files\\x\"");
    }

    #[test]
    fn quotes_and_trailing_backslashes() {
        assert_eq!(quote("say \"hi\""), "\"say \\\"hi\\\"\"");
        assert_eq!(quote("dir \\"), "\"dir \\\\\"");
        assert_eq!(quote("a\\\\\"b"), "\"a\\\\\\\\\\\"b\"");
    }

    #[test]
    fn builds_a_command_line() {
        assert_eq!(
            command_line("C:\\a b\\g.exe", &["loop".into(), "5 6".into()]),
            "\"C:\\a b\\g.exe\" loop \"5 6\""
        );
    }
}
