//! Bot verbs for the campaign layer (host side only; a guest ignores them).
//! The parser is shared with the viewer's bot:
//! [`world::coop::campaign::bot::bot_line`].
//!
//! ```text
//! coop recruit SLOT NPC          the player in SLOT recruits NPC (caps apply)
//! coop dismiss SLOT NPC
//! coop command SLOT NPC OP       OP: wait | follow | trade (owner only)
//! coop down SLOT [QUEST]         the player in SLOT is downed
//! coop downrevive SLOT         the downed player in SLOT stands up (campaign layer only;
//!                                `coop revive SLOT` is the live revive of the viewer's bot)
//! coop stash SLOT HOLDER ITEM N  put N of ITEM into HOLDER as that player
//! coop give SLOT ITEM N          N of ITEM into the player's inventory
//! coop level SLOT N              set the player's level
//! coop objvar SLOT OWNER SCRIPT VAR VALUE   a script object variable as that player
//! coop vote QUEST SLOT yes|no    a vote on an open fork (`coop vote [yes|no]` answers the
//!                                party prompt in `bot.rs`, as the viewer's bot does)
//! coop fork QUEST CHOICE         a script asks to take the fork
//! coop pcond QUEST FUNC [ARG]    party condition probe, result in campaign.probes.QUEST.FUNC
//! coop ffhit SLOT QUEST IDX AMT  SLOT hits the IDX-th protected actor of QUEST's row
//! coop hardcore 0|1              the host turns hardcore mode off or on
//! ```

use world::coop::apply::Ctx;
use world::scripting::GameState;

use crate::data::Data;

pub use world::coop::campaign::bot::{claims, is_campaign_verb};

/// Runs one campaign verb. Returns false when the line is not one of them.
pub fn run(verb: &str, words: &[&str], d: &Data, gs: &mut GameState, cx: Ctx) -> bool {
    let _ = verb;
    world::coop::campaign::bot::bot_line(cx, gs, words, &|w| d.form(w))
}

#[cfg(test)]
mod tests {
    use super::claims;

    fn line(s: &str) -> (String, Vec<&str>) {
        let w: Vec<&str> = s.split_whitespace().collect();
        (w[1].to_string(), w)
    }

    #[test]
    fn the_viewers_verbs_stay_with_the_live_bot() {
        for l in [
            "coop vote yes",
            "coop vote no",
            "coop vote",
            "coop revive 1",
            "coop give TestVendorRef Caps001 5",
        ] {
            let (v, w) = line(l);
            assert!(!claims(&v, &w), "{l} must reach bot.rs");
        }
        for l in [
            "coop vote NVDLC01ElijahDialogue 1 yes",
            "coop downrevive 1",
            "coop down 1",
            "coop give 1 Caps001 5",
        ] {
            let (v, w) = line(l);
            assert!(claims(&v, &w), "{l} is a campaign line");
        }
    }
}
