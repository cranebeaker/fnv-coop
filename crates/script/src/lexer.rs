//! Splitting script source into tokens, line by line.

/// One token.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A word: keyword, variable, function or editor ID (which may start
    /// with a digit, like `1stReconBeret`).
    Word(String),
    Number(f64),
    Str(String),
    /// An operator or punctuation: `== != <= >= < > && || + - * / % ! ( ) , . =`.
    Sym(&'static str),
}

/// The tokens of each source line (comments removed), with line numbers
/// (from 1). Empty lines are left out.
pub fn lex(source: &str) -> Vec<(usize, Vec<Token>)> {
    let mut out = Vec::new();
    for (n, line) in source.lines().enumerate() {
        let tokens = lex_line(line);
        if !tokens.is_empty() {
            out.push((n + 1, tokens));
        }
    }
    out
}

const SYMBOLS: [&str; 19] = [
    "==", "!=", "<=", ">=", "&&", "||", "<", ">", "+", "-", "*", "/", "%", "!", "(", ")", ",", ".",
    "=",
];

fn lex_line(line: &str) -> Vec<Token> {
    let chars: Vec<char> = line.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == ';' {
            break;
        }
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c == '"' {
            let start = i + 1;
            let mut j = start;
            while j < chars.len() && chars[j] != '"' {
                j += 1;
            }
            tokens.push(Token::Str(chars[start..j].iter().collect()));
            i = j + 1;
            continue;
        }
        if c.is_ascii_alphanumeric() || c == '_' {
            let start = i;
            let mut j = i;
            while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            // A number: digits, then perhaps a decimal part.
            let word: String = chars[start..j].iter().collect();
            if word.chars().all(|c| c.is_ascii_digit()) {
                let mut k = j;
                if k < chars.len()
                    && chars[k] == '.'
                    && k + 1 < chars.len()
                    && chars[k + 1].is_ascii_digit()
                {
                    k += 1;
                    while k < chars.len() && chars[k].is_ascii_digit() {
                        k += 1;
                    }
                }
                let text: String = chars[start..k].iter().collect();
                if let Ok(v) = text.parse::<f64>() {
                    tokens.push(Token::Number(v));
                    i = k;
                    continue;
                }
            }
            tokens.push(Token::Word(word));
            i = j;
            continue;
        }
        // A number written `.5`, but not a dot before a name that starts
        // with a digit (`RepconHQFreeform.1stFloorAlarm`).
        let digits_only = |from: usize| {
            chars[from..]
                .iter()
                .take_while(|c| c.is_ascii_alphanumeric() || **c == '_')
                .all(|c| c.is_ascii_digit())
        };
        if c == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit() && digits_only(i + 1) {
            let mut k = i + 1;
            while k < chars.len() && chars[k].is_ascii_digit() {
                k += 1;
            }
            let text: String = std::iter::once('0')
                .chain(chars[i..k].iter().copied())
                .collect();
            tokens.push(Token::Number(text.parse().unwrap_or(0.0)));
            i = k;
            continue;
        }
        let rest: String = chars[i..chars.len().min(i + 2)].iter().collect();
        match SYMBOLS.iter().find(|s| rest.starts_with(*s)) {
            Some(s) => {
                tokens.push(Token::Sym(s));
                i += s.len();
            }
            // Anything else (a stray character) is skipped.
            None => i += 1,
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_words_numbers_strings_and_operators() {
        let t = lex_line("set fTimer to fTimer - 1.5 ; a comment");
        assert_eq!(
            t,
            vec![
                Token::Word("set".into()),
                Token::Word("fTimer".into()),
                Token::Word("to".into()),
                Token::Word("fTimer".into()),
                Token::Sym("-"),
                Token::Number(1.5),
            ]
        );
        let t = lex_line("if Player.GetAV Luck >= 5 && x != .5");
        assert!(t.contains(&Token::Sym(">=")) && t.contains(&Token::Sym("&&")));
        assert!(t.contains(&Token::Number(0.5)));
        // Editor IDs can start with digits.
        assert_eq!(
            lex_line("1stReconBeret"),
            vec![Token::Word("1stReconBeret".into())]
        );
        // A variable whose name starts with a digit, after a dot.
        assert_eq!(
            lex_line("set Repcon.1stFloorAlarm to .5"),
            vec![
                Token::Word("set".into()),
                Token::Word("Repcon".into()),
                Token::Sym("."),
                Token::Word("1stFloorAlarm".into()),
                Token::Word("to".into()),
                Token::Number(0.5),
            ]
        );
        assert_eq!(
            lex_line("ShowMessage \"Hi\""),
            vec![Token::Word("ShowMessage".into()), Token::Str("Hi".into())]
        );
    }
}
