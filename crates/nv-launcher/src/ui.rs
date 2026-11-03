//! Plain stdin/stdout menus with ANSI colour. No emojis.

use std::io::{self, BufRead, Write};

pub struct Ui {
    pub color: bool,
}

/// Turns on ANSI escape handling in the Windows console. Returns false when it cannot.
#[cfg(windows)]
pub fn enable_ansi() -> bool {
    extern "system" {
        fn GetStdHandle(n: u32) -> isize;
        fn GetConsoleMode(h: isize, mode: *mut u32) -> i32;
        fn SetConsoleMode(h: isize, mode: u32) -> i32;
    }
    unsafe {
        let h = GetStdHandle(-11i32 as u32);
        let mut m = 0u32;
        if h == 0 || h == -1 || GetConsoleMode(h, &mut m) == 0 {
            return false;
        }
        SetConsoleMode(h, m | 0x0004) != 0
    }
}

#[cfg(not(windows))]
pub fn enable_ansi() -> bool {
    true
}

impl Ui {
    pub fn new(no_color_env: bool) -> Ui {
        Ui {
            color: !no_color_env && enable_ansi(),
        }
    }
    fn paint(&self, code: &str, s: &str) -> String {
        if self.color {
            format!("\x1b[{code}m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
    pub fn bold(&self, s: &str) -> String {
        self.paint("1", s)
    }
    pub fn dim(&self, s: &str) -> String {
        self.paint("2", s)
    }
    pub fn good(&self, s: &str) -> String {
        self.paint("32", s)
    }
    pub fn bad(&self, s: &str) -> String {
        self.paint("31", s)
    }
    pub fn warn(&self, s: &str) -> String {
        self.paint("33", s)
    }
    pub fn accent(&self, s: &str) -> String {
        self.paint("36", s)
    }

    pub fn title(&self, s: &str) {
        println!(
            "\n{}\n{}",
            self.bold(&self.accent(s)),
            self.dim(&"-".repeat(s.chars().count()))
        );
    }
    pub fn ok(&self, s: &str) {
        println!("  {} {s}", self.good("[ok]"));
    }
    pub fn fail(&self, s: &str) {
        println!("  {} {s}", self.bad("[!!]"));
    }
    pub fn note(&self, s: &str) {
        println!("  {} {s}", self.warn("[..]"));
    }
    pub fn line(&self, s: &str) {
        println!("  {s}");
    }

    /// Asks a question; None at end of input.
    pub fn ask(&self, q: &str) -> Option<String> {
        print!("{} ", self.bold(q));
        let _ = io::stdout().flush();
        let mut s = String::new();
        match io::stdin().lock().read_line(&mut s) {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(s.trim().to_string()),
        }
    }

    /// Asks with a default shown in brackets; empty input keeps it.
    pub fn ask_default(&self, q: &str, default: &str) -> Option<String> {
        let a = self.ask(&if default.is_empty() {
            q.to_string()
        } else {
            format!("{q} [{default}]")
        })?;
        Some(if a.is_empty() { default.to_string() } else { a })
    }

    pub fn confirm(&self, q: &str) -> bool {
        matches!(
            self.ask(&format!("{q} (y/n)"))
                .as_deref()
                .map(str::to_ascii_lowercase)
                .as_deref(),
            Some("y") | Some("yes")
        )
    }

    /// Shows numbered `items` and returns the 0-based choice; "0" or end of input is None (back).
    pub fn menu(&self, title: &str, items: &[String]) -> Option<usize> {
        loop {
            self.title(title);
            for (i, it) in items.iter().enumerate() {
                println!("  {}  {it}", self.accent(&format!("{}", i + 1)));
            }
            println!("  {}  Back", self.dim("0"));
            let a = self.ask(">")?;
            match parse_choice(&a, items.len()) {
                Choice::Back => return None,
                Choice::Item(i) => return Some(i),
                Choice::Invalid => self.fail("Type one of the numbers above."),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Choice {
    Back,
    Item(usize),
    Invalid,
}

pub fn parse_choice(input: &str, n: usize) -> Choice {
    match input.trim().parse::<usize>() {
        Ok(0) => Choice::Back,
        Ok(k) if k <= n => Choice::Item(k - 1),
        _ => Choice::Invalid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choices() {
        assert_eq!(parse_choice("0", 3), Choice::Back);
        assert_eq!(parse_choice(" 2 ", 3), Choice::Item(1));
        assert_eq!(parse_choice("4", 3), Choice::Invalid);
        assert_eq!(parse_choice("x", 3), Choice::Invalid);
        assert_eq!(parse_choice("-1", 3), Choice::Invalid);
    }

    #[test]
    fn colour_off_is_plain() {
        let u = Ui { color: false };
        assert_eq!(u.bold("a"), "a");
        let u = Ui { color: true };
        assert!(u.good("a").contains("\x1b[32m"));
    }
}
