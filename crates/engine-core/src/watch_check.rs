//! `NV_WATCH` (diagnostic only, not exe behaviour): names the code that writes a range of game
//! memory.
//!
//! `NV_WATCH=ADDR,LEN` watches `LEN` bytes at `ADDR`; `NV_WATCH=*PTR+OFF,LEN` watches `LEN` bytes
//! at `OFF` past the word at `PTR`, resolved once that word is non-zero (all numbers hex, `0x`
//! optional; e.g. `NV_WATCH=*011f91c8+1e4,12`). Through [`Mem::watch`](crate::mem::Mem::watch)
//! every store that changes the watched bytes (also zeroing, and re-allocation or freeing of the
//! block) is reported at the next call or return: its address, the bytes before and after (as
//! words and floats), and the calls it happened in (the function returning, or the one calling,
//! then its callers from [`crate::Engine::call_stack`]). Lines start `watch check:`; after 20
//! changes (`NV_WATCH_REPORTS=N`: N) it stays quiet. The run goes on.
//!
//! Off by default: without the variable no hook is installed and nothing is watched.

use crate::mem::{WatchHit, WatchKind};
use crate::Engine;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock, PoisonError};

/// `Engine::call_hook`'s type.
type CallHook = fn(&mut Engine, u32, bool);

/// The changes reported before it stays quiet.
const REPORTS: u32 = 20;

/// The changes reported before it stays quiet: [`REPORTS`], or `NV_WATCH_REPORTS` (decimal).
fn reports() -> u32 {
    static N: OnceLock<u32> = OnceLock::new();
    *N.get_or_init(|| {
        std::env::var("NV_WATCH_REPORTS")
            .ok()
            .and_then(|v| v.trim().parse().ok())
            .unwrap_or(REPORTS)
    })
}

/// Callers named per report.
const PATH_DEPTH: usize = 12;

/// The parsed variable: a fixed address or a pointer word and an offset, and the length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spec {
    /// `Some(word)`: the address is the word at `word` plus `offset`.
    pub pointer: Option<u32>,
    pub offset: u32,
    pub len: u32,
}

/// Parses `ADDR,LEN` or `*PTR+OFF,LEN` (hex).
pub fn parse(text: &str) -> Option<Spec> {
    let (place, len) = text.trim().split_once(',')?;
    let hex = |s: &str| u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok();
    let len = hex(len)?;
    if len == 0 {
        return None;
    }
    let place = place.trim();
    if let Some(rest) = place.strip_prefix('*') {
        let (pointer, offset) = match rest.split_once('+') {
            Some((p, o)) => (hex(p)?, hex(o)?),
            None => (hex(rest)?, 0),
        };
        Some(Spec {
            pointer: Some(pointer),
            offset,
            len,
        })
    } else {
        Some(Spec {
            pointer: None,
            offset: hex(place)?,
            len,
        })
    }
}

fn spec() -> Option<Spec> {
    static SPEC: OnceLock<Option<Spec>> = OnceLock::new();
    *SPEC.get_or_init(|| {
        let text = std::env::var("NV_WATCH").ok()?;
        let spec = parse(&text);
        if spec.is_none() {
            eprintln!("watch check: NV_WATCH={text}: expected ADDR,LEN or *PTR+OFF,LEN (hex)");
        }
        spec
    })
}

/// The resolved address (0 until resolved).
static TARGET: AtomicU32 = AtomicU32::new(0);
static REPORTED: AtomicU32 = AtomicU32::new(0);
static IN_HOOK: AtomicBool = AtomicBool::new(false);
static PREVIOUS: Mutex<Option<CallHook>> = Mutex::new(None);

/// Installs the call hook on a new engine when `NV_WATCH` is set (`Engine::new`).
pub fn install(e: &mut Engine) {
    if spec().is_none() {
        return;
    }
    let previous = e
        .call_hook
        .filter(|h| *h as usize != on_call as CallHook as usize);
    *PREVIOUS.lock().unwrap_or_else(PoisonError::into_inner) = previous;
    e.call_hook = Some(on_call);
}

/// Watches the range once its address is known; answers whether it is watched.
fn resolve(e: &mut Engine, spec: Spec) -> bool {
    if TARGET.load(Ordering::Relaxed) != 0 {
        return true;
    }
    let address = match spec.pointer {
        None => spec.offset,
        Some(pointer) => {
            if !e.mem.is_mapped(pointer) {
                return false;
            }
            let word = e.mem.u32(pointer);
            if word == 0 {
                return false;
            }
            word.wrapping_add(spec.offset)
        }
    };
    if !e.mem.is_mapped(address) {
        return false;
    }
    e.mem.watch(address, spec.len, false);
    TARGET.store(address, Ordering::Relaxed);
    let bytes = e.mem.bytes(address, spec.len.min(16));
    println!(
        "watch check: watching {address:08x}+{:x} (now {})",
        spec.len,
        words_text(&bytes)
    );
    true
}

/// Little-endian words, each also as a float.
fn words_text(bytes: &[u8]) -> String {
    bytes
        .chunks(4)
        .map(|c| {
            let mut w = [0u8; 4];
            w[..c.len()].copy_from_slice(c);
            let v = u32::from_le_bytes(w);
            format!("{v:08x} ({})", f32::from_bits(v))
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// One report line.
pub fn report_text(hit: &WatchHit, path: &str) -> String {
    let n = hit.len.min(8) as usize;
    let old = &hit.old.to_le_bytes()[..n];
    let new = &hit.new.to_le_bytes()[..n];
    format!(
        "{:?} at {:08x} (+{:x}, {} bytes): {} -> {} in {path}",
        hit.kind,
        hit.addr,
        hit.addr.wrapping_sub(hit.start),
        hit.len,
        words_text(old),
        if matches!(hit.kind, WatchKind::Write) {
            words_text(new)
        } else {
            "-".into()
        }
    )
}

/// `Engine::call_hook`: resolves the range, reports its changes, calls on to an earlier hook.
fn on_call(e: &mut Engine, addr: u32, exit: bool) {
    if let Some(spec) = spec() {
        if !IN_HOOK.swap(true, Ordering::Acquire) {
            if resolve(e, spec) && e.mem.has_watch_hits() {
                let hits = e.mem.take_watch_hits();
                let mut frames: Vec<u32> = Vec::with_capacity(PATH_DEPTH);
                if exit {
                    frames.push(addr);
                }
                frames.extend(e.call_stack.iter().rev().take(PATH_DEPTH));
                frames.truncate(PATH_DEPTH);
                let path: Vec<String> = frames.iter().map(|a| format!("{a:08x}")).collect();
                let path = format!(
                    "{} ({})",
                    path.join(" < "),
                    if exit {
                        "seen as it returned"
                    } else {
                        "seen as it called on"
                    }
                );
                for hit in &hits {
                    if REPORTED.fetch_add(1, Ordering::Relaxed) < reports() {
                        println!("watch check: {}", report_text(hit, &path));
                    }
                }
            }
            IN_HOOK.store(false, Ordering::Release);
        }
    }
    let previous = *PREVIOUS.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(hook) = previous {
        hook(e, addr, exit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::Mem;

    #[test]
    fn parses_fixed_and_pointer_forms() {
        assert_eq!(
            parse("011f474c,c"),
            Some(Spec {
                pointer: None,
                offset: 0x011f_474c,
                len: 12
            })
        );
        assert_eq!(
            parse("*011f91c8+1e4,12"),
            Some(Spec {
                pointer: Some(0x011f_91c8),
                offset: 0x1e4,
                len: 0x12
            })
        );
        assert_eq!(
            parse("*0x011f91c8,4"),
            Some(Spec {
                pointer: Some(0x011f_91c8),
                offset: 0,
                len: 4
            })
        );
        assert_eq!(parse("nonsense"), None);
        assert_eq!(parse("10,0"), None);
    }

    #[test]
    fn report_names_the_change_and_the_calls() {
        let mut m = Mem::new();
        let a = m.alloc(0x10);
        m.watch(a, 12, false);
        m.set_f32(a, -80.0);
        let hit = m.take_watch_hits()[0];
        let text = report_text(&hit, "00b6c0d0 < 0086e650");
        assert!(text.contains("c2a00000 (-80)"), "{text}");
        assert!(text.contains("00000000 (0) -> "), "{text}");
        assert!(text.ends_with("in 00b6c0d0 < 0086e650"), "{text}");
    }
}
