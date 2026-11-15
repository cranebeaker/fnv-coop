//! Runs a line as the program runs a line typed in its console.
//!
//! The program compiles a line with form ids. It does not resolve each editor id (a quest's
//! editor id fails), so a caller gives form ids.

use crate::addr;
use crate::program::Program;

/// The program runs `line`, with `reference` as the calling reference (0 for none). False
/// when the program has no console manager yet.
pub fn run_line(p: &mut dyn Program, line: &str, reference: u32) -> bool {
    let manager = p.call(addr::CONSOLE_MANAGER, &[1]).eax;
    if manager == 0 {
        return false;
    }
    let context = p.u32(manager);
    let bytes = line.as_bytes();
    let text = p.alloc(bytes.len() as u32 + 1);
    p.set_cstr(text, bytes);
    let script = p.alloc(addr::SCRIPT_BYTES);
    p.write(script, &[0; addr::SCRIPT_BYTES as usize]);
    p.call(addr::SCRIPT_NEW, &[script]);
    p.call(addr::FORM_SET_TEMPORARY, &[script]);
    p.call(addr::SCRIPT_SET_TEXT, &[script, text]);
    p.call(addr::SCRIPT_RUN, &[script, context, 1, reference]);
    p.call(addr::SCRIPT_DROP, &[script]);
    p.free(script);
    p.free(text);
    true
}

/// The player's reference and position, or `None` before the program made the player.
pub fn player_place(p: &dyn Program) -> Option<(u32, [f32; 3])> {
    let player = p.u32(addr::PLAYER);
    (player != 0).then(|| {
        (
            player,
            [0, 4, 8].map(|o| p.f32(player + addr::REF_POSITION + o)),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    /// The line runs in the program's own order of calls, and its memory is given back.
    #[test]
    fn a_line_runs_in_the_programs_order_and_frees_its_memory() {
        let mut p = FakeProgram::new();
        p.console_manager = 0x3000_0000;
        p.set_u32(0x3000_0000, 0x1234);
        assert!(run_line(&mut p, "SetStage 0015D912 10", 0));
        let order: Vec<u32> = p.calls.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            [
                addr::CONSOLE_MANAGER,
                addr::SCRIPT_NEW,
                addr::FORM_SET_TEMPORARY,
                addr::SCRIPT_SET_TEXT,
                addr::SCRIPT_RUN,
                addr::SCRIPT_DROP
            ]
        );
        let run = &p.calls[4].1;
        assert_eq!(
            (run[1], run[2], run[3]),
            (0x1234, 1, 0),
            "context, 1, no reference"
        );
        let text = p.calls[3].1[1];
        let mut got = [0u8; 20];
        p.read(text, &mut got);
        assert_eq!(&got, b"SetStage 0015D912 10");
        assert!(p.live.is_empty(), "{:?}", p.live);
    }

    #[test]
    fn no_console_manager_runs_nothing() {
        let mut p = FakeProgram::new();
        assert!(!run_line(&mut p, "coc X", 0));
        assert_eq!(p.calls.len(), 1);
    }

    #[test]
    fn the_players_place_reads_three_floats() {
        let mut p = FakeProgram::new();
        assert_eq!(player_place(&p), None);
        p.set_u32(addr::PLAYER, 0x2100_0000);
        p.set_f32(0x2100_0000 + addr::REF_POSITION + 4, 2.5);
        assert_eq!(player_place(&p), Some((0x2100_0000, [0.0, 2.5, 0.0])));
    }
}
