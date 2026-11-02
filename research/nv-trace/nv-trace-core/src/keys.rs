//! The key script, in the port's `NV_MOVE_KEYS` form: `FRAME:DIK:HOLD,...` (DirectInput key
//! code in hex, held from frame FRAME for HOLD frames; HOLD defaults to 1). The port counts a
//! frame per pass of the game's main message loop; the recorder counts calls of the list's
//! `frame` function (`Main::OnIdle`), which runs once per pass.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Press {
    pub frame: u64,
    pub dik: u8,
    pub hold: u64,
}

pub fn parse(text: &str) -> Result<Vec<Press>, String> {
    let mut out = Vec::new();
    for item in text.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let mut part = item.split(':');
        let frame = part
            .next()
            .and_then(|f| f.trim().parse().ok())
            .ok_or_else(|| format!("bad frame in {item:?}"))?;
        let dik = part
            .next()
            .and_then(|c| u8::from_str_radix(c.trim(), 16).ok())
            .ok_or_else(|| format!("bad key code in {item:?}"))?;
        let hold = match part.next() {
            None => 1,
            Some(h) => h
                .trim()
                .parse()
                .map_err(|_| format!("bad hold in {item:?}"))?,
        };
        out.push(Press { frame, dik, hold });
    }
    Ok(out)
}

/// Key changes due at the start of `frame`: `(dik, down)`, releases first.
pub fn changes_at(script: &[Press], frame: u64) -> Vec<(u8, bool)> {
    let mut out: Vec<(u8, bool)> = script
        .iter()
        .filter(|p| p.frame + p.hold == frame)
        .map(|p| (p.dik, false))
        .collect();
    out.extend(
        script
            .iter()
            .filter(|p| p.frame == frame && p.hold > 0)
            .map(|p| (p.dik, true)),
    );
    out
}

/// The keyboard scan code and whether it is an extended (`E0`) key, for a DirectInput code:
/// DirectInput numbers the extended keys as the scan code plus 0x80 (`DIK_DOWN` 0xD0 is `E0
/// 50`).
pub fn scan_code(dik: u8) -> (u16, bool) {
    if dik >= 0x80 {
        (u16::from(dik & 0x7f), true)
    } else {
        (u16::from(dik), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_new_game_script_parses() {
        let s = parse("1500:d0:1,1504:d0:1,1510:1c:1, 1700:1c").unwrap();
        assert_eq!(s.len(), 4);
        assert_eq!(
            s[0],
            Press {
                frame: 1500,
                dik: 0xd0,
                hold: 1
            }
        );
        assert_eq!(s[3].hold, 1);
        assert!(parse("x:1c").is_err());
        assert!(parse("1:zz").is_err());
    }

    #[test]
    fn presses_and_releases_by_frame() {
        let s = parse("10:d0:2,12:1c:1").unwrap();
        assert_eq!(changes_at(&s, 10), [(0xd0, true)]);
        assert!(changes_at(&s, 11).is_empty());
        assert_eq!(changes_at(&s, 12), [(0xd0, false), (0x1c, true)]);
        assert_eq!(changes_at(&s, 13), [(0x1c, false)]);
        assert_eq!(scan_code(0xd0), (0x50, true));
        assert_eq!(scan_code(0x1c), (0x1c, false));
    }
}
