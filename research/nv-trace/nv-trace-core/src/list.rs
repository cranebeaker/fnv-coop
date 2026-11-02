//! Function lists: which exe functions to record and how to read their words.
//!
//! One function per line, `#` starts a comment:
//!
//! ```text
//! # address  convention  words  return  name
//! 0064aa00   thiscall    ppf    void    BSFaceGenAnimationData::AddPhonemeKeyframe
//! 0064ca60   cdecl       fppb   u8      BSFaceGenAnimationData::UpdateKeyframeList
//! 0086e650   frame       p      void    Main::OnIdle
//! ```
//!
//! `words` has one letter per 32-bit word the call gets, in the order the port's
//! `Engine::call` gets them: for `thiscall` the first letter is `ECX` (`this`), for
//! `fastcall` the first two are `ECX` and `EDX`, then the stack words from the lowest address
//! up. `-` means none. The recorder reads exactly that many stack words, so the count must be
//! the function's (from its `RET n` or, for `cdecl`, its callers; never guessed). Letters:
//!
//! - `p` a pointer: compared after normalisation (see `diff`);
//! - `w` a whole word compared as it is (`i` and `u` are the same);
//! - `b` a byte (only the low 8 bits are compared: the exe pushes a register whose upper bytes
//!   are whatever was there), `h` 16 bits;
//! - `f` a float, compared bit for bit.
//!
//! After the return kind, `max=N` records only the first N calls (a hot function) and
//! `after=ADDR:N` starts recording once the N-th call of `ADDR` (a listed function) has been
//! entered (`after=ADDR` is `ADDR:1`), e.g. `after=007ac730` from `RaceSexMenu::Create` on.
//!
//! `return` is `void` (nothing compared), `u8`/`bool` (`AL`), `u16`, `u32`/`i32`, `ptr`, `i64`
//! (`EDX:EAX`), `f32`/`f64` (`ST0`). The convention `frame` marks the per-frame function the
//! recorder counts frames by (for the key script); it is recorded like `thiscall`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Conv {
    Cdecl,
    Stdcall,
    Thiscall,
    Fastcall,
    /// The per-frame function (recorded like thiscall; the recorder counts its calls).
    Frame,
}

impl Conv {
    /// Words that arrive in registers (`ECX`, then `EDX`).
    pub fn reg_words(self) -> usize {
        match self {
            Conv::Thiscall | Conv::Frame => 1,
            Conv::Fastcall => 2,
            _ => 0,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Conv::Cdecl => "cdecl",
            Conv::Stdcall => "stdcall",
            Conv::Thiscall => "thiscall",
            Conv::Fastcall => "fastcall",
            Conv::Frame => "frame",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WordKind {
    Ptr,
    Word,
    Byte,
    Half,
    Float,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetKind {
    Void,
    U8,
    U16,
    U32,
    Ptr,
    I64,
    F32,
    F64,
}

impl RetKind {
    pub fn x87(self) -> bool {
        matches!(self, RetKind::F32 | RetKind::F64)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnSpec {
    pub addr: u32,
    pub conv: Conv,
    pub words: Vec<WordKind>,
    pub ret: RetKind,
    pub name: String,
    /// Record at most this many calls (`max=N` after the return kind): for hot functions.
    pub max: Option<u64>,
    /// Record only after the N-th call of this function has been entered (`after=ADDR:N`, N
    /// from 1; `after=ADDR` is `ADDR:1`): (address, N). The function must be listed too.
    pub after: Option<(u32, u64)>,
    /// Memory dumped at each recorded entry (`dump=EXPR:LEN`, [`crate::dump`]).
    pub dumps: Vec<crate::dump::DumpSpec>,
    /// Record only during frames S to E-1 (calls of the list's `frame` function) counted from
    /// the first call let through (`frames=S..E`, `frames=N` is `0..N`; with `after=`, from the
    /// event on).
    pub frames: Option<(u64, u64)>,
    /// Record only the calls that meet these (`when=EXPR==N`, [`crate::dump::Cond`]).
    pub when: Vec<crate::dump::Cond>,
}

impl FnSpec {
    /// Stack words the function takes.
    pub fn stack_words(&self) -> usize {
        self.words.len().saturating_sub(self.conv.reg_words())
    }

    /// The line this spec is written as (header `fn=` lines).
    pub fn line(&self) -> String {
        let words: String = if self.words.is_empty() {
            "-".into()
        } else {
            self.words
                .iter()
                .map(|w| match w {
                    WordKind::Ptr => 'p',
                    WordKind::Word => 'w',
                    WordKind::Byte => 'b',
                    WordKind::Half => 'h',
                    WordKind::Float => 'f',
                })
                .collect()
        };
        let ret = match self.ret {
            RetKind::Void => "void",
            RetKind::U8 => "u8",
            RetKind::U16 => "u16",
            RetKind::U32 => "u32",
            RetKind::Ptr => "ptr",
            RetKind::I64 => "i64",
            RetKind::F32 => "f32",
            RetKind::F64 => "f64",
        };
        let max = self.max.map(|m| format!("max={m} ")).unwrap_or_default();
        let after = self
            .after
            .map(|(a, n)| format!("after={a:08x}:{n} "))
            .unwrap_or_default();
        let frames = self
            .frames
            .map(|(s, e)| match s {
                0 => format!("frames={e} "),
                _ => format!("frames={s}..{e} "),
            })
            .unwrap_or_default();
        let dumps: String = self
            .when
            .iter()
            .map(|c| &c.text)
            .chain(self.dumps.iter().map(|d| &d.text))
            .map(|t| format!("{t} "))
            .collect();
        format!(
            "{:08x} {} {} {} {}{}{}{}{}",
            self.addr,
            self.conv.name(),
            words,
            ret,
            max,
            after,
            frames,
            dumps,
            self.name
        )
        .trim_end()
        .to_string()
    }
}

fn parse_line(line: &str) -> Result<Option<FnSpec>, String> {
    let line = line.split('#').next().unwrap_or("").trim();
    if line.is_empty() {
        return Ok(None);
    }
    let mut parts = line.split_whitespace();
    let addr_text = parts.next().unwrap_or("");
    let addr = u32::from_str_radix(
        addr_text.trim_start_matches("0x").trim_start_matches("0X"),
        16,
    )
    .map_err(|_| format!("bad address {addr_text:?}"))?;
    let conv = match parts.next() {
        Some("cdecl") => Conv::Cdecl,
        Some("stdcall") => Conv::Stdcall,
        Some("thiscall") => Conv::Thiscall,
        Some("fastcall") => Conv::Fastcall,
        Some("frame") => Conv::Frame,
        other => return Err(format!("bad convention {other:?}")),
    };
    let words_text = parts.next().ok_or("no word types")?;
    let words = if words_text == "-" {
        Vec::new()
    } else {
        words_text
            .chars()
            .map(|c| match c {
                'p' => Ok(WordKind::Ptr),
                'w' | 'i' | 'u' => Ok(WordKind::Word),
                'b' => Ok(WordKind::Byte),
                'h' => Ok(WordKind::Half),
                'f' => Ok(WordKind::Float),
                _ => Err(format!("bad word type {c:?}")),
            })
            .collect::<Result<Vec<_>, _>>()?
    };
    if words.len() < conv.reg_words() {
        return Err(format!(
            "{} needs at least {} word types (the register words)",
            conv.name(),
            conv.reg_words()
        ));
    }
    if words.len() > 64 {
        return Err("more than 64 words".into());
    }
    let ret = match parts.next() {
        Some("void") => RetKind::Void,
        Some("u8" | "bool") => RetKind::U8,
        Some("u16") => RetKind::U16,
        Some("u32" | "i32") => RetKind::U32,
        Some("ptr") => RetKind::Ptr,
        Some("i64" | "u64") => RetKind::I64,
        Some("f32") => RetKind::F32,
        Some("f64") => RetKind::F64,
        other => return Err(format!("bad return kind {other:?}")),
    };
    let mut max = None;
    let mut after = None;
    let mut dumps = Vec::new();
    let mut frames = None;
    let mut when = Vec::new();
    let mut words_of_name = Vec::new();
    for p in parts {
        if p.starts_with("when=") {
            when.push(
                crate::dump::parse_when(p)
                    .ok_or_else(|| format!("bad {p:?} (when=EXPR==NUMBER or !=)"))?,
            );
        } else if let Some(n) = p.strip_prefix("max=") {
            max = Some(n.parse().map_err(|_| format!("bad {p:?}"))?);
        } else if let Some(n) = p.strip_prefix("frames=") {
            frames = Some(parse_frames(n).ok_or_else(|| format!("bad {p:?} (frames=N or S..E)"))?);
        } else if crate::dump::is_dump_token(p) {
            dumps.push(
                crate::dump::parse_token(p).ok_or_else(|| {
                    format!("bad {p:?} (dump=EXPR:LEN and the others in dump.rs)")
                })?,
            );
        } else if let Some(v) = p.strip_prefix("after=") {
            after = Some(parse_after(v).ok_or_else(|| format!("bad {p:?} (after=ADDR:N)"))?);
        } else {
            words_of_name.push(p);
        }
    }
    let name = words_of_name.join(" ");
    Ok(Some(FnSpec {
        addr,
        conv,
        words,
        ret,
        name,
        max,
        after,
        dumps,
        frames,
        when,
    }))
}

/// `N` (frames 0 to N-1) or `S..E` (S to E-1): (S, E), E above S.
pub fn parse_frames(v: &str) -> Option<(u64, u64)> {
    let (s, e) = match v.split_once("..") {
        Some((s, e)) => (s.parse().ok()?, e.parse().ok()?),
        None => (0, v.parse().ok()?),
    };
    (e > s).then_some((s, e))
}

/// `ADDR` or `ADDR:N` (hex address, decimal count from 1).
pub fn parse_after(v: &str) -> Option<(u32, u64)> {
    let (a, n) = v.split_once(':').unwrap_or((v, "1"));
    let addr = u32::from_str_radix(a.trim_start_matches("0x"), 16).ok()?;
    let n: u64 = n.parse().ok()?;
    (n >= 1).then_some((addr, n))
}

/// Parses a list; errors name the line.
pub fn parse(text: &str) -> Result<Vec<FnSpec>, String> {
    let mut out: Vec<FnSpec> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        match parse_line(line) {
            Ok(Some(spec)) => {
                if out.iter().any(|s| s.addr == spec.addr) {
                    return Err(format!("line {}: {:08x} is listed twice", i + 1, spec.addr));
                }
                out.push(spec);
            }
            Ok(None) => {}
            Err(e) => return Err(format!("line {}: {e}", i + 1)),
        }
    }
    Ok(out)
}

/// The list a trace's header carries (its `fn=` lines).
pub fn from_header(header: &str) -> Result<Vec<FnSpec>, String> {
    let text: Vec<&str> = header
        .lines()
        .filter_map(|l| l.strip_prefix("fn="))
        .collect();
    parse(&text.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_parse_and_print_back() {
        let l = parse("# x\n0064aa00 thiscall ppf void BSFaceGenAnimationData::AddPhonemeKeyframe\n0x0064ca60 cdecl fppb u8\n0086e650 frame p void Main::OnIdle\n").unwrap();
        assert_eq!(l.len(), 3);
        assert_eq!(l[0].conv, Conv::Thiscall);
        assert_eq!(l[0].stack_words(), 2);
        assert_eq!(l[0].words, [WordKind::Ptr, WordKind::Ptr, WordKind::Float]);
        assert_eq!(l[1].ret, RetKind::U8);
        assert_eq!(l[1].stack_words(), 4);
        assert_eq!(l[2].stack_words(), 0);
        assert_eq!(l[2].conv.reg_words(), 1);
        assert_eq!(
            l[0].line(),
            "0064aa00 thiscall ppf void BSFaceGenAnimationData::AddPhonemeKeyframe"
        );
        assert_eq!(parse(&l[1].line()).unwrap()[0], l[1]);
    }

    #[test]
    fn mistakes_name_their_line() {
        assert!(parse("0064aa00 thiscall - void")
            .unwrap_err()
            .starts_with("line 1"));
        assert!(parse("\n0064aa00 pascal p void")
            .unwrap_err()
            .starts_with("line 2"));
        assert!(parse("1 cdecl p void\n1 cdecl p void")
            .unwrap_err()
            .contains("twice"));
        assert!(parse("1 cdecl x void").is_err());
    }

    #[test]
    fn a_start_event_is_read_and_printed() {
        let l = parse(
            "00ba9ee0 thiscall ppwwwpppp void max=9 after=007ac730 Add\n\
             00000010 cdecl - void after=008a20d0:10 X",
        )
        .unwrap();
        assert_eq!(l[0].after, Some((0x007a_c730, 1)));
        assert_eq!(l[1].after, Some((0x008a_20d0, 10)));
        assert_eq!(parse(&l[0].line()).unwrap()[0], l[0]);
        assert!(parse("1 cdecl - void after=zz").is_err());
        assert!(parse("1 cdecl - void after=10:0").is_err());
        let l = parse(
            "00bdf790 thiscall ppwpwpw void dump=this+0x20:8 dump=*(*(this+0x60)+8)+0xec:4 X",
        )
        .unwrap();
        assert_eq!(l[0].dumps.len(), 2);
        assert_eq!(parse(&l[0].line()).unwrap()[0], l[0]);
        assert!(parse("1 cdecl - void dump=this").is_err());
        let l =
            parse("00b9e970 thiscall pp void max=10 when=*(this)==0x010b7eb8 rdump=this+0x110:2 T")
                .unwrap();
        assert_eq!((l[0].when.len(), l[0].dumps.len()), (1, 1));
        assert_eq!(parse(&l[0].line()).unwrap()[0], l[0]);
        assert!(parse("1 cdecl - void when=w0").is_err());
        let l = parse("1 cdecl - void frames=120..124 A\n2 cdecl - void frames=3 B").unwrap();
        assert_eq!((l[0].frames, l[1].frames), (Some((120, 124)), Some((0, 3))));
        assert_eq!(parse(&l[0].line()).unwrap()[0], l[0]);
        assert_eq!(parse(&l[1].line()).unwrap()[0], l[1]);
        assert!(parse("1 cdecl - void frames=4..4").is_err());
    }

    #[test]
    fn a_cap_is_read_and_printed() {
        let l = parse("00000010 cdecl - void max=5 Hot function").unwrap();
        assert_eq!((l[0].max, l[0].name.as_str()), (Some(5), "Hot function"));
        assert_eq!(parse(&l[0].line()).unwrap()[0], l[0]);
        assert!(parse("1 cdecl - void max=x").is_err());
    }

    #[test]
    fn the_header_carries_the_list() {
        let h = "source=real\nfn=00000010 stdcall pw u32 A\nfn=00000020 cdecl - void\n";
        let l = from_header(h).unwrap();
        assert_eq!((l[0].addr, l[0].ret, l[1].addr), (0x10, RetKind::U32, 0x20));
    }
}
