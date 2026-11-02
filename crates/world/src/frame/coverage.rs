//! Phase 1's gate (docs/FRAME_SKELETON.md, "PR 7 result"): every call of
//! `Main::OnIdle` (`0086e650`) to depth 2 is either wired to an nv-rs system
//! or listed as open. Depth 1 is [`super::STEPS`], each step with its
//! [`Wiring`]. Depth 2 is the direct calls of those functions: where a model
//! splits a function ([`super::player`], [`super::world_time`],
//! [`super::ai_stage`], [`super::interface`]), its sub-steps carry their own
//! wiring; every other depth-2 call (a query the models leave out, or a
//! function no model splits yet) is open.
//!
//! [`cover`] matches a function's depth-2 calls, in call order, against its
//! model's direct calls in the order of their call sites (the models' tests
//! check that order against `research/engine-map/frame.tsv`). A test writes
//! the result for every depth-1 and depth-2 row of `frame.tsv` to
//! `research/engine-map/frame_wiring.tsv` (and checks the committed file is
//! current), which `scripts/ledger` counts in docs/LEDGER.md ("Frame").

use super::{ai_stage, interface, player, world_time, Wiring};

/// Which model holds a call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    /// `world::frame` itself (a call of `Main::OnIdle`).
    Frame,
    Player,
    WorldTime,
    AiStage,
    Interface,
}

impl Model {
    /// The module's name.
    pub fn name(self) -> &'static str {
        match self {
            Model::Frame => "world::frame",
            Model::Player => "world::frame::player",
            Model::WorldTime => "world::frame::world_time",
            Model::AiStage => "world::frame::ai_stage",
            Model::Interface => "world::frame::interface",
        }
    }
}

/// The direct calls a model gives for a function `Main::OnIdle` calls, as
/// (call site, callee, wiring) in site order; `None` when no model splits
/// it.
pub type ModelledCalls = (Model, Vec<(u32, u32, Wiring)>);

pub fn modelled_calls(address: u32) -> Option<ModelledCalls> {
    let mut out: Vec<(u32, u32, Wiring)> = Vec::new();
    let model;
    if address == player::UPDATE_PLAYER_ADDRESS {
        model = Model::Player;
        for c in &player::UPDATE_PLAYER {
            if let player::Callee::Direct(a) = c.callee {
                out.push((c.site, a, c.wiring));
            }
        }
    } else if let Some(f) = world_time::FUNCTIONS
        .iter()
        .find(|f| f.address == address && f.caller == world_time::ON_IDLE)
    {
        model = Model::WorldTime;
        for st in f.steps {
            if let world_time::Callee::Direct(a) = st.callee {
                out.extend(st.sites.iter().map(|&s| (s, a, st.wiring)));
            }
        }
    } else if let Some(f) = ai_stage::FUNCTIONS
        .iter()
        .find(|f| f.address == address && f.callers.iter().any(|c| c.function == ai_stage::ON_IDLE))
    {
        model = Model::AiStage;
        for st in f.steps {
            if let ai_stage::Callee::Direct(a) = st.callee {
                out.extend(st.sites.iter().map(|&s| (s, a, st.wiring)));
            }
        }
    } else {
        let f = interface::FUNCTIONS.iter().find(|f| {
            f.address == address && f.callers.iter().any(|c| c.function == interface::ON_IDLE)
        })?;
        model = Model::Interface;
        for st in f.steps {
            if let interface::Callee::Direct(a) = st.callee {
                out.extend(st.sites.iter().map(|&s| (s, a, st.wiring)));
            }
        }
    }
    out.sort_by_key(|c| c.0);
    Some((model, out))
}

/// The wiring of each of a depth-1 function's direct calls (`children`, in
/// call order, as `frame.tsv` lists them): its model's sub-step's, matched
/// in order, or `None` (open: not in a model).
pub fn cover(address: u32, children: &[u32]) -> Vec<Option<(Model, Wiring)>> {
    let Some((model, calls)) = modelled_calls(address) else {
        return vec![None; children.len()];
    };
    let mut at = 0;
    children
        .iter()
        .map(|&c| {
            if at < calls.len() && calls[at].1 == c {
                at += 1;
                Some((model, calls[at - 1].2))
            } else {
                None
            }
        })
        .collect()
}

/// A wiring's word in `frame_wiring.tsv`.
pub fn word(w: Wiring) -> &'static str {
    match w {
        Wiring::Open => "open",
        Wiring::Partial(_) => "partial",
        Wiring::System(_) => "system",
    }
}

/// A wiring's text (the system, or what of it exists).
pub fn text(w: Wiring) -> &'static str {
    match w {
        Wiring::Open => "",
        Wiring::Partial(t) | Wiring::System(t) => t,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::STEPS;

    /// `research/engine-map/frame.tsv`, the committed call tree.
    const FRAME_TSV: &str = include_str!("../../../../research/engine-map/frame.tsv");

    struct Row<'a> {
        seq: &'a str,
        depth: u32,
        call: &'a str,
        address: u32,
        name: &'a str,
    }

    fn rows() -> Vec<Row<'static>> {
        FRAME_TSV
            .lines()
            .filter(|l| !l.starts_with('#') && !l.starts_with("seq\t"))
            .map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                Row {
                    seq: f[0],
                    depth: f[1].parse().unwrap(),
                    call: f[2],
                    address: u32::from_str_radix(f[3], 16).unwrap(),
                    name: f[4],
                }
            })
            .collect()
    }

    /// `frame_wiring.tsv`: each depth-1 and depth-2 row of `frame.tsv` with
    /// its wiring and the model that holds it.
    fn table() -> String {
        let rows = rows();
        let mut out = String::from(
            "# Wiring of Main::OnIdle's calls (Xbox PDB, PC 0086e650) to depth 2, from \
             world::frame (Phase 1's gate, docs/FRAME_SKELETON.md \"PR 7 result\"); generated by \
             `cargo test -p world frame::coverage` with NVRS_WRITE_FRAME_WIRING=1\n\
             seq\tdepth\tcall\taddress\tname\twiring\tmodel\tsystem\n",
        );
        let mut step = 0;
        let mut i = 0;
        while i < rows.len() {
            let r = &rows[i];
            if r.depth != 1 {
                i += 1;
                continue;
            }
            let s = &STEPS[step];
            assert_eq!(s.address, r.address, "step {}", step + 1);
            out += &format!(
                "{}\t1\t{}\t{:08x}\t{}\t{}\t{}\t{}\n",
                r.seq,
                r.call,
                r.address,
                r.name,
                word(s.wiring),
                Model::Frame.name(),
                text(s.wiring)
            );
            let kids: Vec<&Row> = rows[i + 1..]
                .iter()
                .take_while(|k| k.depth > 1)
                .filter(|k| k.depth == 2)
                .collect();
            let addresses: Vec<u32> = kids.iter().map(|k| k.address).collect();
            for (k, c) in kids.iter().zip(cover(r.address, &addresses)) {
                let (wiring, model) = match c {
                    Some((m, w)) => (w, m.name()),
                    None => (Wiring::Open, "-"),
                };
                out += &format!(
                    "{}\t2\t{}\t{:08x}\t{}\t{}\t{}\t{}\n",
                    k.seq,
                    k.call,
                    k.address,
                    k.name,
                    word(wiring),
                    model,
                    text(wiring)
                );
            }
            step += 1;
            i += 1;
        }
        assert_eq!(step, STEPS.len());
        out
    }

    /// Every model's direct calls are found, in order, under each frame
    /// step that calls the function (none is left unmatched).
    #[test]
    fn every_modelled_call_is_matched() {
        let rows = rows();
        for (i, r) in rows.iter().enumerate() {
            if r.depth != 1 {
                continue;
            }
            let Some((_, calls)) = modelled_calls(r.address) else {
                continue;
            };
            let kids: Vec<u32> = rows[i + 1..]
                .iter()
                .take_while(|k| k.depth > 1)
                .filter(|k| k.depth == 2)
                .map(|k| k.address)
                .collect();
            let matched = cover(r.address, &kids).iter().flatten().count();
            assert_eq!(matched, calls.len(), "{:08x}", r.address);
        }
    }

    /// The committed `research/engine-map/frame_wiring.tsv` is this table
    /// (set `NVRS_WRITE_FRAME_WIRING=1` to write it).
    #[test]
    fn frame_wiring_tsv_is_current() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../research/engine-map/frame_wiring.tsv");
        let want = table();
        if std::env::var_os("NVRS_WRITE_FRAME_WIRING").is_some() {
            std::fs::write(&path, &want).unwrap();
        }
        let have = std::fs::read_to_string(&path)
            .unwrap_or_default()
            .replace("\r\n", "\n");
        assert!(
            have == want,
            "research/engine-map/frame_wiring.tsv is stale; run the test with \
             NVRS_WRITE_FRAME_WIRING=1"
        );
    }

    /// Phase 1's gate: every depth-1 and depth-2 call has a row, wired or
    /// open; the systems and partly wired calls are named.
    #[test]
    fn phase_1_gate() {
        let t = table();
        let lines: Vec<&str> = t.lines().filter(|l| !l.starts_with('#')).skip(1).collect();
        let want = rows().iter().filter(|r| (1..=2).contains(&r.depth)).count();
        assert_eq!(lines.len(), want);
        for l in lines {
            let f: Vec<&str> = l.split('\t').collect();
            assert!(["system", "partial", "open"].contains(&f[5]), "{l}");
            assert_eq!(f[5] == "open", f[7].is_empty(), "{l}");
        }
    }
}
