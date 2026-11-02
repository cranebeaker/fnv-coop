//! Reading a menu file the way the game does (`00a01b00` and what it
//! calls in FalloutNV.exe): the text cleaned (line breaks, extra spaces and
//! comments gone), includes pasted in as text, then a small
//! tokenizer, then the tokens merged into "start a tile", "trait = value",
//! "add this operator" and so on. Not a general XML reader: it does what
//! the game's does, including its quirks (a tag name runs to the next `>`
//! or whitespace, so `<systemcolor</systemcolor>` is one tag; a numeric
//! `<string>` loses its text).

use crate::names::{attr, kind, op, t, Names, NOT_FOUND};

/// Where prefabs come from (`Data\Menus\Prefabs\<name>`, `00a02d40`).
pub const PREFAB_FOLDER: &str = "menus\\prefabs\\";

/// What a token does once merged (`00a0a410`).
pub mod token {
    /// An opening tag nothing else claimed (an unknown tag).
    pub const OPEN: i32 = 0;
    /// A closing tag nothing else claimed.
    pub const CLOSE: i32 = 1;
    /// An operator tag with operators inside: a group begins...
    pub const ACTION_BEGIN: i32 = 2;
    /// ...and ends.
    pub const ACTION_END: i32 = 3;
    /// A trait tag with operators inside: they apply to this trait...
    pub const TRAIT_OPEN: i32 = 4;
    /// ...until it closes.
    pub const TRAIT_CLOSE: i32 = 5;
    /// A tile starts (its type in `value`, its name in `text`).
    pub const TILE_START: i32 = 6;
    /// A tile ends.
    pub const TILE_END: i32 = 7;
    /// A trait set to a value (`id` the trait, `value`/`text` the value).
    pub const TRAIT_VALUE: i32 = 8;
    /// An operator with a constant (`id` the operator, `value` the
    /// constant; text is dropped).
    pub const ACTION_CONST: i32 = 9;
    /// An operator reading another tile's trait (`id` the operator,
    /// `value` the trait, `text` the `src`).
    pub const ACTION_LINK: i32 = 10;
}

/// One token: `kind` (see [`token`]; before merging also an attribute's
/// number or [`attr::VALUE`]), the number the text looked up to (or
/// parsed as), its whole-number form, and the text.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: i32,
    pub value: f32,
    pub id: i32,
    pub text: String,
}

/// A file's tokens, and the templates it defines (each its own tokens).
#[derive(Debug, Clone, Default)]
pub struct Parsed {
    pub tokens: Vec<Token>,
    pub templates: Vec<(String, Vec<Token>)>,
    /// What the game would print as "MENUS: ..." and carry on past.
    pub warnings: Vec<String>,
}

/// What the game's menu file reader (`00a1ce70`, behind `00a1c9b0`, used
/// for menu files and prefabs alike) does to a file's text before anything
/// else reads it:
/// - tabs, line feeds and carriage returns are dropped (not turned into
///   spaces: "a\nb" becomes "ab");
/// - a space after a space or after `>` is dropped, a space before `<` or
///   `>` is dropped, and " />" becomes "/>";
/// - comments (`<!--` to the next `-->`) are cut out whole, so a commented
///   `<include>` is never pasted;
/// - once more than 100,000 characters have gone by since the last break,
///   a line break goes in after the next closing tag (`</...>`).
///
/// The text stops at a zero byte.
pub fn clean(text: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(text.len());
    // The last three characters kept (or, in a comment, read).
    let (mut p3, mut p2, mut p1) = (0u8, 0u8, 0u8);
    let mut in_comment = false;
    let mut since_break = 0u32;
    let mut closing = false;
    for &c in text {
        if c == 0 {
            break;
        }
        if matches!(c, b'\t' | b'\n' | b'\r') {
            continue;
        }
        if in_comment {
            if p2 == b'-' && p1 == b'-' && c == b'>' {
                in_comment = false;
            }
            (p3, p2, p1) = (p2, p1, c);
            continue;
        }
        if c == b' ' {
            if p1 == b' ' || p1 == b'>' {
                continue;
            }
        } else if p1 == b' ' && (c == b'<' || c == b'>') {
            out.pop();
        } else if p2 == b' ' && p1 == b'/' && c == b'>' {
            out.pop();
            if let Some(last) = out.last_mut() {
                *last = b'/';
            }
        } else if p3 == b'<' && p2 == b'!' && p1 == b'-' && c == b'-' {
            (p3, p2, p1) = (0, 0, 0);
            in_comment = true;
            out.truncate(out.len().saturating_sub(3));
            continue;
        }
        let mut line_break = false;
        if since_break > 100_000 {
            if closing && c == b'>' {
                line_break = true;
            } else if p1 == b'<' && c == b'/' {
                closing = true;
            }
        }
        out.push(c);
        (p3, p2, p1) = (p2, p1, c);
        since_break += 1;
        if line_break {
            closing = false;
            out.push(b'\n');
            since_break = 0;
        }
    }
    out
}

/// Pastes every `<include src="X"/>` in: the text from `<include` to the
/// first `>` after it is replaced by the whole prefab file named by the
/// first quoted string (read and [`clean`]ed as the game reads every menu
/// file), again and again until none is left (`00a02d40`). A prefab that
/// can't be read pastes nothing (the game would crash).
pub fn expand_includes(text: &[u8], read: &mut dyn FnMut(&str) -> Option<Vec<u8>>) -> Vec<u8> {
    let mut text = text.to_vec();
    // The game recurses after each pass; a loop does the same. A prefab
    // including itself would never end there either; stop after a while.
    for _ in 0..64 {
        let Some(mut at) = find(&text, b"<include", 0) else {
            return text;
        };
        let mut out = text[..at].to_vec();
        loop {
            let end = text[at..]
                .iter()
                .position(|&b| b == b'>')
                .map_or(text.len(), |p| at + p + 1);
            let tag = &text[at..end];
            let name = tag
                .split(|&b| b == b'"')
                .nth(1)
                .map(|n| String::from_utf8_lossy(n).into_owned())
                .unwrap_or_default();
            if let Some(prefab) = read(&format!("{PREFAB_FOLDER}{name}")) {
                out.extend_from_slice(&clean(&prefab));
            }
            match find(&text, b"<include", end) {
                Some(next) => {
                    out.extend_from_slice(&text[end..next]);
                    at = next;
                }
                None => {
                    out.extend_from_slice(&text[end..]);
                    break;
                }
            }
        }
        text = out;
    }
    text
}

fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if from >= haystack.len() {
        return None;
    }
    haystack[from..]
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|p| p + from)
}

/// The tokenizer's states (`00a01e20`).
#[derive(Clone, Copy, PartialEq)]
enum State {
    BeforeFirstTag,
    TagName,
    AfterSlash,
    Content,
    Comment,
    AttributeName,
    AttributeValue,
}

/// Reads a menu file's text (includes already pasted in) into merged
/// tokens. `setting` answers `&-sName;` text with the game setting's
/// string (`007073d0`); text it can't answer is dropped, as the game
/// drops it. An error the game stops on ("Empty tag name", "Attribute
/// with no value", ...) is returned as `Err`.
pub fn tokenize(
    text: &[u8],
    names: &mut Names,
    setting: &dyn Fn(&str) -> Option<String>,
) -> Result<Parsed, String> {
    let mut merger = Merger {
        names,
        out: Parsed::default(),
        template: None,
    };
    let mut state = State::BeforeFirstTag;
    let mut name = String::new();
    let mut close = false;
    let mut started = false;
    let mut quoted = false;
    let mut buf = String::new();
    let mut attribute = 0;
    let n = text.len();
    let mut i = 0usize;
    let byte_at = |i: usize| text.get(i).copied().unwrap_or(0);
    let tag_end = |i: usize| {
        byte_at(i) == b'>' || (i + 1 < n && byte_at(i) == b'/' && byte_at(i + 1) == b'>')
    };
    while i < n {
        let c = text[i];
        match state {
            State::BeforeFirstTag => {
                if c == b'<' {
                    if i + 3 < n && &text[i + 1..i + 4] == b"!--" {
                        started = false;
                        state = State::Comment;
                    } else {
                        state = State::TagName;
                        name.clear();
                        close = false;
                    }
                }
            }
            State::TagName => {
                if name.is_empty() && c == b'/' {
                    close = true;
                } else if tag_end(i) {
                    if name.is_empty() {
                        if !started {
                            return Err(at_line(text, i, "Empty tag name"));
                        }
                    } else {
                        merger.emit(if close { 1 } else { 0 }, &name, false);
                    }
                    state = if c == b'/' {
                        State::AfterSlash
                    } else {
                        State::Content
                    };
                    started = false;
                } else if c < 0x21 {
                    if !name.is_empty() {
                        merger.emit(if close { 1 } else { 0 }, &name, false);
                        started = true;
                    }
                } else if !started {
                    name.push(char::from(c));
                } else {
                    // A word after the tag's name: an attribute; read this
                    // character again as its first.
                    state = State::AttributeName;
                    started = false;
                    continue;
                }
            }
            State::Comment => {
                if i + 2 < n && &text[i..i + 3] == b"-->" {
                    i += 2;
                    state = State::Content;
                    started = false;
                }
            }
            State::AttributeName => {
                if tag_end(i) {
                    return Err(at_line(text, i, "Attribute with no value"));
                }
                if c == b'=' {
                    if buf.is_empty() {
                        return Err(at_line(text, i, "Missing attribute name"));
                    }
                    attribute = merger.names.lookup(&buf).unwrap_or(NOT_FOUND as i32);
                    buf.clear();
                    state = State::AttributeValue;
                    started = false;
                } else if c < 0x21 {
                    if !buf.is_empty() {
                        attribute = merger.names.lookup(&buf).unwrap_or(NOT_FOUND as i32);
                        buf.clear();
                        started = true;
                    }
                } else {
                    if started {
                        return Err(at_line(text, i, "Unexpected word after attribute name"));
                    }
                    buf.push(char::from(c));
                }
            }
            State::AttributeValue => {
                if c == b'"' {
                    quoted = !quoted;
                } else if !quoted {
                    if tag_end(i) {
                        if buf.is_empty() {
                            if !started {
                                return Err(at_line(text, i, "Missing attribute's value"));
                            }
                        } else {
                            merger.emit(attribute, &buf, false);
                        }
                        buf.clear();
                        state = if c == b'/' {
                            State::AfterSlash
                        } else {
                            State::Content
                        };
                        started = false;
                    } else if c < 0x21 {
                        if !buf.is_empty() {
                            merger.emit(attribute, &buf, false);
                            buf.clear();
                            started = true;
                        }
                    } else if !started {
                        buf.push(char::from(c));
                    } else {
                        state = State::AttributeName;
                        started = false;
                        continue;
                    }
                } else {
                    buf.push(char::from(c));
                }
            }
            State::AfterSlash => {
                if c != b'>' {
                    return Err(at_line(
                        text,
                        i,
                        "Close-tag marker '/' not followed by end-of-tag marker '>'",
                    ));
                }
                merger.emit(1, &name, false);
                state = State::Content;
                name.clear();
            }
            State::Content => {
                if c == b'<' {
                    if i + 3 < n && &text[i + 1..i + 4] == b"!--" {
                        state = State::Comment;
                    } else {
                        let value = buf.trim_end_matches(|ch: char| (ch as u32) < 0x21);
                        if !value.is_empty() {
                            if let Some(rest) = value.strip_prefix('&') {
                                if rest.starts_with('-') {
                                    if let Some(s) = setting(value) {
                                        merger.emit(attr::VALUE, &s, false);
                                    }
                                } else {
                                    merger.emit(attr::VALUE, value, false);
                                }
                            } else {
                                merger.emit(attr::VALUE, value, true);
                            }
                        }
                        buf.clear();
                        close = false;
                        state = State::TagName;
                        name.clear();
                    }
                    started = false;
                } else {
                    if i + 1 < n && c == b'/' && text[i + 1] == b'>' {
                        return Err(at_line(
                            text,
                            i,
                            "Unbalanced close-tag marker pair '/>' found",
                        ));
                    }
                    if c == b'>' {
                        return Err(at_line(text, i, "Unbalanced end-of-tag marker '>' found"));
                    }
                    if c > 0x20 || started {
                        buf.push(char::from(c));
                        started = true;
                    }
                }
            }
        }
        i += 1;
    }
    Ok(merger.out)
}

/// An error message with the text just before it (the file's line breaks
/// are gone by now, see [`clean`]).
fn at_line(text: &[u8], at: usize, message: &str) -> String {
    let end = (at + 1).min(text.len());
    let start = end.saturating_sub(60);
    format!(
        "{message}, at \"...{}\"",
        String::from_utf8_lossy(&text[start..end])
    )
}

/// Builds the token list as `00a0a410` does: looks the text up, then
/// merges it with the tokens before it.
struct Merger<'a> {
    names: &'a mut Names,
    out: Parsed,
    /// A template being defined: its name and tokens.
    template: Option<(String, Vec<Token>)>,
}

impl Merger<'_> {
    fn list(&mut self) -> &mut Vec<Token> {
        match &mut self.template {
            Some((_, tokens)) => tokens,
            None => &mut self.out.tokens,
        }
    }

    /// `kind`: 0 open tag, 1 close tag, an attribute's number, or
    /// [`attr::VALUE`]; `literal`: text between tags not starting with `&`
    /// (not looked up as a name).
    fn emit(&mut self, kind: i32, text: &str, literal: bool) {
        let mut value = if literal {
            NOT_FOUND
        } else {
            self.names.lookup(text).map_or(NOT_FOUND, |v| v as f32)
        };
        if value == NOT_FOUND && (text.starts_with('_') || text.starts_with("&_")) {
            value = self
                .names
                .lookup_or_add(text)
                .map_or(NOT_FOUND, |v| v as f32);
        }
        let mut token = Token {
            kind,
            value,
            id: value as i32,
            text: text.to_string(),
        };
        // Text made only of digits, '-' and '.' is a number (`%f`); its
        // text is dropped either way.
        if text
            .bytes()
            .all(|b| b.is_ascii_digit() || b == b'-' || b == b'.')
        {
            if token.value == NOT_FOUND || token.value == 0.0 {
                if let Some(v) = parse_float(text) {
                    token.value = v;
                }
            }
            token.text.clear();
            token.id = token.value as i32;
        }
        let len = self.list().len();
        let last = len.checked_sub(1);
        let kind_at = |list: &Vec<Token>, i: Option<usize>| i.map(|i| list[i].kind);

        // A template's name: start capturing its tokens (the opening tag
        // is dropped either way).
        if kind == attr::NAME {
            let opens_template = last.is_some_and(|l| {
                let list = self.list();
                list[l].kind == token::OPEN && list[l].value == kind::TEMPLATE as f32
            });
            if opens_template {
                self.list().pop();
                if self.template.is_some() {
                    self.out
                        .warnings
                        .push("Can't have nested template definitions in an XML file.".into());
                } else {
                    self.template = Some((token.text.clone(), Vec::new()));
                }
                return;
            }
        }
        if kind == token::CLOSE && token.value == kind::TEMPLATE as f32 && self.template.is_some() {
            if let Some(done) = self.template.take() {
                self.out.templates.push(done);
            }
            return;
        }
        let list = self.list();
        // A `name` attribute right after a tile type's opening tag: the
        // tile starts, named.
        if kind == attr::NAME {
            if let Some(l) = last {
                if list[l].kind == token::OPEN && kind::is_tile(list[l].value) {
                    list[l].kind = token::TILE_START;
                    list[l].text = token.text;
                    return;
                }
            }
            token.kind = token::TRAIT_VALUE;
            token.id = attr::NAME;
            list.push(token);
            return;
        }
        let second = last.and_then(|l| l.checked_sub(1));
        let third = second.and_then(|s| s.checked_sub(1));
        // open, value, close: "trait = value" or "operator with a constant".
        if kind == token::CLOSE
            && kind_at(list, last) == Some(attr::VALUE)
            && matches!(
                kind_at(list, second),
                Some(token::TRAIT_OPEN | token::ACTION_BEGIN)
            )
            && second.is_some_and(|s| list[s].value == token.value)
        {
            let (l, s) = (last.unwrap_or(0), second.unwrap_or(0));
            let v = token.value;
            let bad = !((4001.0..4125.0).contains(&v) || token.id > 9999)
                && !(2000.0..=2025.0).contains(&v);
            list[s].kind = if (4001.0..4125.0).contains(&v) || token.id > 9999 {
                token::TRAIT_VALUE
            } else if bad {
                -1
            } else {
                token::ACTION_CONST
            };
            let value_token = list[l].clone();
            list[s].id = token.id;
            list[s].value = value_token.value;
            list[s].text = value_token.text;
            list.truncate(l);
            if bad {
                self.out
                    .warnings
                    .push("Bad trait/action type in XML".into());
            }
            return;
        }
        // open operator, src, trait, close: "operator reading a trait".
        if kind == token::CLOSE
            && kind_at(list, last) == Some(attr::TRAIT)
            && kind_at(list, second) == Some(attr::SRC)
            && kind_at(list, third) == Some(token::ACTION_BEGIN)
            && third.is_some_and(|th| list[th].value == token.value)
        {
            let (l, s, th) = (last.unwrap_or(0), second.unwrap_or(0), third.unwrap_or(0));
            let trait_value = list[l].value;
            let src = list[s].text.clone();
            list[th].kind = token::ACTION_LINK;
            list[th].id = token.id;
            list[th].value = trait_value;
            list[th].text = src;
            list.truncate(s);
            return;
        }
        let id = token.id;
        if t::is_trait(id) {
            if token.kind == token::OPEN {
                token.kind = token::TRAIT_OPEN;
            } else if token.kind == token::CLOSE {
                token.kind = token::TRAIT_CLOSE;
            }
        } else if op::is_op(id) {
            if token.kind == token::OPEN {
                token.kind = token::ACTION_BEGIN;
            } else if token.kind == token::CLOSE && last.is_some() {
                token.kind = token::ACTION_END;
            }
        } else if token.kind == token::CLOSE && kind::is_tile(token.value) {
            token.kind = token::TILE_END;
        }
        list.push(token);
    }
}

/// `sscanf("%f")`: the longest number at the start of the text.
fn parse_float(text: &str) -> Option<f32> {
    let bytes = text.as_bytes();
    let mut end = 0;
    let mut seen_digit = false;
    let mut seen_dot = false;
    if end < bytes.len() && (bytes[end] == b'-' || bytes[end] == b'+') {
        end += 1;
    }
    while end < bytes.len() {
        match bytes[end] {
            b'0'..=b'9' => seen_digit = true,
            b'.' if !seen_dot => seen_dot = true,
            _ => break,
        }
        end += 1;
    }
    if !seen_digit {
        return None;
    }
    text[..end].parse::<f32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(xml: &str) -> Parsed {
        let mut names = Names::new(true);
        tokenize(xml.as_bytes(), &mut names, &|s| {
            (s == "&-sStatsXP;").then(|| "XP".to_string())
        })
        .unwrap()
    }

    #[test]
    fn traits_with_values_merge_into_one_token() {
        let p = tokens("<rect name=\"a\"><width> 386 </width><visible>&true;</visible></rect>");
        let kinds: Vec<i32> = p.tokens.iter().map(|t| t.kind).collect();
        assert_eq!(kinds, [token::TILE_START, 8, 8, token::TILE_END]);
        assert_eq!(p.tokens[0].text, "a");
        assert_eq!(p.tokens[0].value, kind::RECT as f32);
        assert_eq!((p.tokens[1].id, p.tokens[1].value), (t::WIDTH, 386.0));
        assert_eq!((p.tokens[2].id, p.tokens[2].value), (t::VISIBLE, 1.0));
    }

    #[test]
    fn operators_merge_with_constants_and_links() {
        let p = tokens(
            "<rect name=\"a\"><x><copy src=\"screen()\" trait=\"width\"/><div> 2 </div></x></rect>",
        );
        let kinds: Vec<i32> = p.tokens.iter().map(|t| t.kind).collect();
        assert_eq!(
            kinds,
            [
                token::TILE_START,
                token::TRAIT_OPEN,
                token::ACTION_LINK,
                token::ACTION_CONST,
                token::TRAIT_CLOSE,
                token::TILE_END
            ]
        );
        let link = &p.tokens[2];
        assert_eq!(
            (link.id, link.value, link.text.as_str()),
            (op::COPY, 4017.0, "screen()")
        );
        assert_eq!((p.tokens[3].id, p.tokens[3].value), (op::DIV, 2.0));
    }

    #[test]
    fn settings_text_and_strings() {
        let p = tokens(
            "<text name=\"a\"><string>&-sStatsXP;</string><filename> a\\b c.dds </filename></text>",
        );
        assert_eq!(p.tokens[1].text, "XP");
        assert_eq!(p.tokens[1].value, NOT_FOUND);
        assert_eq!(p.tokens[2].text, "a\\b c.dds");
    }

    #[test]
    fn numeric_strings_lose_their_text() {
        // The game's `00a0a410` clears the text of anything that reads as
        // a number, strings included.
        let p = tokens("<text name=\"a\"><string> 42 </string></text>");
        assert_eq!((p.tokens[1].value, p.tokens[1].text.as_str()), (42.0, ""));
    }

    #[test]
    fn a_tag_name_runs_to_the_next_bracket() {
        // hud_main_menu.xml's `<systemcolor</systemcolor>` is one unknown
        // tag, left as an unmatched opening tag.
        let p = tokens("<image name=\"a\"><systemcolor</systemcolor><depth>10</depth></image>");
        assert_eq!(p.tokens[1].kind, token::OPEN);
        assert_eq!(p.tokens[1].text, "systemcolor</systemcolor");
        assert_eq!((p.tokens[2].kind, p.tokens[2].id), (8, t::DEPTH));
    }

    #[test]
    fn comments_and_templates() {
        let p = tokens(
            "<!-- x --><menu name=\"m\"><template name=\"t\"><image name=\"i\"><x>3</x></image></template><!-- y --></menu>",
        );
        assert_eq!(p.tokens.len(), 2);
        assert_eq!(p.templates.len(), 1);
        let (name, body) = &p.templates[0];
        assert_eq!(name, "t");
        assert_eq!(body.len(), 3);
        assert_eq!(body[0].kind, token::TILE_START);
    }

    #[test]
    fn custom_traits_are_registered() {
        let p = tokens("<rect name=\"a\"><_Gap> 15 </_Gap></rect>");
        assert!(p.tokens[1].id > 9999);
        assert_eq!(p.tokens[1].value, 15.0);
    }

    #[test]
    fn errors_the_game_stops_on() {
        let mut names = Names::new(true);
        let none = &|_: &str| None;
        assert!(tokenize(b"<rect name=\"a\">x > y</rect>", &mut names, none).is_err());
        assert!(tokenize(b"<>", &mut names, none).is_err());
    }

    #[test]
    fn includes_are_pasted_in() {
        let mut read = |path: &str| -> Option<Vec<u8>> {
            match path {
                "menus\\prefabs\\a.xml" => Some(b"<x>1</x><include src=\"b.xml\"/>".to_vec()),
                "menus\\prefabs\\b.xml" => Some(b"<y>2</y>".to_vec()),
                _ => None,
            }
        };
        let out = expand_includes(b"<r><include src=\"a.xml\"/> <z/></r>", &mut read);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "<r><x>1</x><y>2</y> <z/></r>"
        );
    }

    #[test]
    fn files_are_cleaned_before_reading() {
        let cleaned = |s: &str| String::from_utf8(clean(s.as_bytes())).unwrap();
        // Line breaks and tabs go, runs of spaces shrink, spaces next to
        // brackets go.
        assert_eq!(
            cleaned("<text name=\"a\" >\n\t<string>  two\nwords  three </string>\r\n</text>"),
            "<text name=\"a\"><string>twowords three</string></text>"
        );
        assert_eq!(cleaned("<x />"), "<x/>");
        // Comments are cut out whole, with the space before them.
        assert_eq!(cleaned("<a>1 <!-- <b> --> 2</a>"), "<a>12</a>");
        // A file stops at a zero byte.
        assert_eq!(cleaned("<a/>\0<b/>"), "<a/>");
    }

    #[test]
    fn a_commented_include_is_not_pasted() {
        // hotkeys.xml comments out `<include src="HotKey.xml"/>`, whose
        // own comments would otherwise leave a stray "-->" behind.
        let mut read = |path: &str| -> Option<Vec<u8>> {
            (path == "menus\\prefabs\\b.xml").then(|| b"<!-- b -->\n<y>2</y>\n<!-- /b -->".to_vec())
        };
        let text = clean(b"<r>\n<!-- <include src=\"b.xml\"/> -->\n<include src=\"b.xml\"/>\n</r>");
        let out = expand_includes(&text, &mut read);
        assert_eq!(String::from_utf8(out).unwrap(), "<r><y>2</y></r>");
    }
}
