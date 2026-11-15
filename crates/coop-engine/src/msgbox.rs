//! msgbox: co-op's own pages in the game's message box (docs/coop/SPECS.md, voice list).
//!
//! The players list is shown the way the game shows its own questions: a modal entry of the
//! message box (`00703e80`, [`addr::MESSAGE_BOX_SHOW`]), with no result callback. A click on a
//! button stores the entry's first button's result plus the button's index in the interface
//! manager's result byte (`MessageMenu::DoClick`, `007aa070`); co-op's entries number their
//! buttons from [`FIRST_RESULT`], which no game question uses (the game's own count from 0 or 1,
//! the button counts it passes), so a value in co-op's range is co-op's click, read with the
//! game's own `GetMessageMenuResult` (which sets the byte back). Escape clicks the last button
//! (`MessageMenu::DoKey`, `007aa110`, for a warning context other than 0x17).

use engine_core::prelude::*;

use crate::addr;

/// The first button's result of co-op's entries.
pub const FIRST_RESULT: u8 = 0xa0;
/// The most buttons an entry of co-op's has.
pub const MAX_BUTTONS: usize = 12;
/// The longest text kept (the game's HUD message limit, as a safe length for a text tile).
pub const MAX_TEXT: usize = 0x400;

fn latin1(s: &str, max: usize) -> Vec<u8> {
    let mut b: Vec<u8> = s
        .chars()
        .map(|c| if (c as u32) < 0x100 { c as u8 } else { b'?' })
        .filter(|b| *b != 0)
        .collect();
    b.truncate(max);
    b
}

/// Queues a modal message box with `text` and `buttons` (at most [`MAX_BUTTONS`]); false when
/// the game has no message box now (no function, no memory, or the interface is not ready).
pub fn show(e: &mut Engine, text: &str, buttons: &[String]) -> bool {
    if !e.is_registered(addr::MESSAGE_BOX_SHOW) || buttons.is_empty() {
        return false;
    }
    let mut texts = vec![latin1(text, MAX_TEXT)];
    texts.extend(buttons.iter().take(MAX_BUTTONS).map(|b| latin1(b, 0x100)));
    let mut blocks = Vec::new();
    for bytes in &texts {
        let b = e
            .call(addr::OPERATOR_NEW, &args![bytes.len() as u32 + 1])
            .u32();
        if b == 0 {
            break;
        }
        e.mem.set_cstr(b, bytes);
        blocks.push(b);
    }
    let mut shown = false;
    if blocks.len() == texts.len() {
        let mut words = args![blocks[0], 0u32, 0u32, 0u32, u32::from(FIRST_RESULT), 0u32];
        words.extend(args![0.0f32, 0.0f32]);
        words.extend_from_slice(&blocks[1..]);
        words.push(0);
        shown = e.call(addr::MESSAGE_BOX_SHOW, &words).u32() & 0xff != 0;
    }
    for b in blocks {
        e.call(addr::OPERATOR_DELETE, &args![b]);
    }
    shown
}

/// The index of the button of co-op's entry clicked since the last call (`count` buttons), if
/// one was; the result byte is left alone when it holds another entry's answer or none.
pub fn answer(e: &mut Engine, count: usize) -> Option<usize> {
    if !e.is_registered(addr::INTERFACE_MANAGER) {
        return None;
    }
    let manager = e.call(addr::INTERFACE_MANAGER, &[]).u32();
    if manager == 0 {
        return None;
    }
    let v = e
        .mem
        .u8(manager.wrapping_add(addr::MESSAGE_MENU_RESULT_OFFSET));
    let index = usize::from(v.checked_sub(FIRST_RESULT)?);
    if index >= count.min(MAX_BUTTONS) {
        return None;
    }
    if e.is_registered(addr::MESSAGE_MENU_RESULT) {
        e.call(addr::MESSAGE_MENU_RESULT, &[]);
    } else {
        e.mem
            .set_u8(manager.wrapping_add(addr::MESSAGE_MENU_RESULT_OFFSET), 0xff);
    }
    Some(index)
}

/// Whether a message box is on the screen now.
pub fn open(e: &mut Engine) -> bool {
    e.mem.u32(addr::MESSAGE_MENU) != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAP: u32 = 0x0500_0000;
    const MANAGER: u32 = 0x0510_0000;
    const SEEN: u32 = 0x0520_0000;

    fn eax(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }

    /// The game's side, cut down: memory, the manager, and a message box that keeps what it was
    /// given (the text and the buttons' texts, copied as the game copies them).
    fn game() -> Engine {
        let mut e = Engine::new();
        e.mem.map(HEAP, 0x10_0000);
        e.mem.map(MANAGER, 0x1000);
        e.mem.map(SEEN, 0x1_0000);
        e.mem.map(addr::MESSAGE_MENU & !0xfff, 0x1000);
        e.mem.set_u32(HEAP, HEAP + 0x10);
        e.register(addr::OPERATOR_NEW, |e, a| {
            let at = e.mem.u32(HEAP);
            e.mem.set_u32(HEAP, at + ((a[0] + 15) & !15));
            eax(at)
        });
        e.register(addr::OPERATOR_DELETE, |e, a| {
            // Freed blocks are cleared: the box must have copied its texts.
            e.mem.set_u8(a[0], 0);
            Ret::default()
        });
        e.register(addr::INTERFACE_MANAGER, |_, _| eax(MANAGER));
        e.mem
            .set_u8(MANAGER + addr::MESSAGE_MENU_RESULT_OFFSET, 0xff);
        e.register(addr::MESSAGE_MENU_RESULT, |e, _| {
            let v = e.mem.u8(MANAGER + addr::MESSAGE_MENU_RESULT_OFFSET);
            e.mem
                .set_u8(MANAGER + addr::MESSAGE_MENU_RESULT_OFFSET, 0xff);
            eax(u32::from(v))
        });
        e.register(addr::MESSAGE_BOX_SHOW, |e, a| {
            // Seen: the fixed words, then the texts joined by '|'.
            for (i, w) in a.iter().take(8).enumerate() {
                e.mem.set_u32(SEEN + 4 * i as u32, *w);
            }
            let mut joined = e.mem.cstr(a[0]).to_vec();
            for &t in a[8..].iter().take_while(|t| **t != 0) {
                joined.push(b'|');
                joined.extend(e.mem.cstr(t));
            }
            e.mem.set_cstr(SEEN + 0x100, &joined);
            eax(1)
        });
        e
    }

    #[test]
    fn a_page_is_a_modal_box_whose_buttons_count_from_co_op_s_first_result() {
        let mut e = game();
        assert!(show(
            &mut e,
            "Co-op players\nBo",
            &["Bo".into(), "Zoë".into(), "Close".into()]
        ));
        assert_eq!(e.mem.u32(SEEN), HEAP + 0x10, "the text first");
        assert_eq!(e.mem.u32(SEEN + 12), 0, "no callback");
        assert_eq!(e.mem.u32(SEEN + 16), u32::from(FIRST_RESULT));
        assert_eq!(e.mem.u32(SEEN + 20), 0, "Escape clicks the last button");
        assert_eq!(
            e.mem.cstr(SEEN + 0x100),
            b"Co-op players\nBo|Bo|Zo\xeb|Close".as_slice()
        );
        // Nothing clicked yet; then the second button.
        assert_eq!(answer(&mut e, 3), None);
        e.mem
            .set_u8(MANAGER + addr::MESSAGE_MENU_RESULT_OFFSET, FIRST_RESULT + 1);
        assert_eq!(answer(&mut e, 3), Some(1));
        assert_eq!(
            e.mem.u8(MANAGER + addr::MESSAGE_MENU_RESULT_OFFSET),
            0xff,
            "read through the game's own function"
        );
        assert_eq!(answer(&mut e, 3), None);
    }

    #[test]
    fn another_entry_s_answer_is_left_for_it() {
        let mut e = game();
        for v in [0u8, 2, FIRST_RESULT + 3] {
            e.mem.set_u8(MANAGER + addr::MESSAGE_MENU_RESULT_OFFSET, v);
            assert_eq!(answer(&mut e, 3), None, "{v}");
            assert_eq!(e.mem.u8(MANAGER + addr::MESSAGE_MENU_RESULT_OFFSET), v);
        }
    }

    #[test]
    fn without_the_game_s_box_nothing_is_shown() {
        let mut e = Engine::new();
        assert!(!show(&mut e, "x", &["Close".into()]));
        assert_eq!(answer(&mut e, 1), None);
        let mut e = game();
        assert!(!show(&mut e, "x", &[]));
        assert!(!open(&mut e));
        e.mem.set_u32(addr::MESSAGE_MENU, 0x1234);
        assert!(open(&mut e));
    }
}
