//! Reputation (`REPU`) and karma, read from the game's code
//! (`TESReputation`, Xbox PDB; `docs/FACTIONS_CRIME.md`).
//!
//! A reputation's `DATA` f32 is its most (NCR 80, Legion 100, Goodsprings
//! 15); the game keeps fame and infamy for each, both from 0. Scripts:
//! `AddReputation rep type n` / `RemoveReputation` move fame (type 1) or
//! infamy (type 0) by `fReputationBump…` (n 1–5: 1, 2, 4, 7, 12; others
//! nothing); `…Exact` by the amount given. Adding stops at the most (no
//! floor), removing at 0 (no ceiling) (`00615730`, `00615a00`);
//! `SetReputation` sets it (no clamp, no notice); `GetReputation` the
//! value, `GetReputationPct` value / most (a fraction). Each axis's level
//! from value / most: 1 from 0.15, 2 from 0.5, 3 at the most
//! (`fReputationThreshold…`, `00616950`); the title is by infamy level × 4
//! plus fame level (`sRepTitlePos<F>Neg<I>`), and `GetReputationThreshold`
//! answers per axis as `00616a90` does. Every change shows the HUD notice
//! "<name>\n<sRepPositiveGain…>" (the data's "Fame Gained!" and so on);
//! when the changed axis's level changes, a box with the reputation's
//! name, "<title>\n<description>", the title's icon and a good or bad sound
//! ([`title_box`], `006155f0`). The engine itself changes reputations
//! only through crimes' infamy (`world::crime`).
//!
//! Karma (actor value 23): `RewardKarma` (always the player) clamped to
//! ±1000 (`iKarmaMin`/`Max`), with the game's notice; bands very evil ≤
//! −750, evil ≤ −250, good ≥ 250, very good ≥ 750. The player's kills of
//! anyone in a faction that tracks crime (`FACT` `DATA` flag 0x100) change
//! it by the victim's own karma: good −50, very good −100, evil +100 (the
//! data's), very evil +2 (`fKarmaMod…`, `0089d900`); taking what someone
//! not evil owns −5 (`world::crime::stealing_karma`).

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::dialogue::PLAYER_REF;
use crate::scripting::{game_setting, game_setting_text, Event, GameState};

const REPU: FourCC = FourCC::new(b"REPU");

/// The karma actor value.
pub const KARMA: u16 = 23;

/// The axes: type 1 fame, 0 infamy.
pub const FAME: u8 = 1;
pub const INFAMY: u8 = 0;

/// A reputation's most and name.
pub fn reputation_max(order: &LoadOrder, rep: FormId) -> Option<f32> {
    let record = order
        .get(rep)
        .filter(|r| r.entry.header.kind == REPU)?
        .record()
        .ok()?;
    record
        .get(esm::sig::DATA)
        .filter(|s| s.data.len() >= 4)
        .map(|s| le_f32(&s.data, 0))
}

fn reputation_name(order: &LoadOrder, rep: FormId) -> String {
    order
        .get(rep)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name())
        .unwrap_or_else(|| rep.to_string())
}

/// Fame (type 1) or infamy (type 0) now.
pub fn get(state: &GameState, rep: FormId, kind: u8) -> f32 {
    let (fame, infamy) = state.reputations.get(&rep).copied().unwrap_or((0.0, 0.0));
    if kind == FAME {
        fame
    } else {
        infamy
    }
}

fn set_raw(state: &mut GameState, rep: FormId, kind: u8, value: f32) {
    state.emit(crate::coop::Domain::Reputation, rep);
    let e = state.reputations.entry(rep).or_insert((0.0, 0.0));
    if kind == FAME {
        e.0 = value;
    } else {
        e.1 = value;
    }
}

/// An axis's level (0–3) for a value (`00616950`).
pub fn level(order: &LoadOrder, value: f32, max: f32) -> u8 {
    let x = if max > 0.0 { value / max } else { 0.0 };
    let t = |n: &str, d: f32| game_setting(order, n).unwrap_or(d);
    if x >= t("fReputationThresholdThree", 1.0) {
        3
    } else if x >= t("fReputationThresholdTwo", 0.5) {
        2
    } else if x >= t("fReputationThresholdOne", 0.15) {
        1
    } else {
        0
    }
}

/// Both levels of a reputation now: (fame, infamy).
pub fn levels(order: &LoadOrder, state: &GameState, rep: FormId) -> (u8, u8) {
    let max = reputation_max(order, rep).unwrap_or(1.0);
    (
        level(order, get(state, rep, FAME), max),
        level(order, get(state, rep, INFAMY), max),
    )
}

/// The title for fame and infamy levels (`sRepTitlePos<F>Neg<I>`; the
/// exe's own names where the data has none).
pub fn title(order: &LoadOrder, fame: u8, infamy: u8) -> String {
    const WORDS: [&str; 4] = ["None", "One", "Two", "Three"];
    // The exe's defaults, by infamy × 4 + fame.
    const DEFAULTS: [&str; 16] = [
        "Neutral",
        "Accepted",
        "Liked",
        "Idolized",
        "Shunned",
        "Mixed",
        "Smiling Troublemaker",
        "Good Natured Rascal",
        "Hated",
        "Sneering Punk",
        "Unpredictable",
        "Dark Hero",
        "Vilified",
        "Merciful Thug",
        "Soft-Hearted Devil",
        "Wild Child",
    ];
    let (f, i) = (fame.min(3), infamy.min(3));
    game_setting_text(
        order,
        &format!(
            "sRepTitlePos{}Neg{}",
            WORDS[usize::from(f)],
            WORDS[usize::from(i)]
        ),
    )
    .unwrap_or_else(|| DEFAULTS[usize::from(i) * 4 + usize::from(f)].to_string())
}

/// `GetReputationThreshold rep axis` (`00616a90`): 1 when neither is
/// known; else by axis 0 (mixed), 1 (good), 2 (bad) as the game's table
/// has it, 0 off the axis.
pub fn threshold(order: &LoadOrder, state: &GameState, rep: FormId, axis: u8) -> f32 {
    let (f, i) = levels(order, state, rep);
    if (f, i) == (0, 0) {
        return 1.0;
    }
    let v = match axis {
        0 => match (f, i) {
            (1, 1) => 3,
            (2, 2) => 4,
            (3, 3) => 5,
            (3, 2) | (2, 3) => 2,
            _ => 0,
        },
        1 => match (f, i) {
            (1, 0) => 4,
            (2, 0) => 5,
            (3, 0) => 6,
            (2, 1) => 2,
            (3, 1) => 3,
            _ => 0,
        },
        2 => match (f, i) {
            (0, 1) => 4,
            (0, 2) => 5,
            (0, 3) => 6,
            (1, 2) => 2,
            (1, 3) => 3,
            _ => 0,
        },
        _ => 0,
    };
    v as f32
}

/// Fame or infamy raised by `amount` (`AddReputation` with its bump,
/// `AddReputationExact`, a faction's crimes' infamy), at most to the
/// reputation's most (no floor: the amount isn't checked).
// Translated from 00615730 (decompiled, FalloutNV.exe 1.4.0.525;
// `TESReputation::AddReputationValue`, Xbox PDB), 00615c90 the same with
// the bump.
pub fn add(order: &LoadOrder, state: &mut GameState, rep: FormId, kind: u8, amount: f32) {
    let Some(max) = reputation_max(order, rep) else {
        return;
    };
    let before = axis_level(order, state, rep, kind, max);
    let now = (get(state, rep, kind) + amount).min(max);
    set_raw(state, rep, kind, now);
    let words = if kind == FAME {
        ("sRepPositiveGain", "Reputation Gain")
    } else {
        ("sRepNegativeGain", "Reputation Loss")
    };
    changed(order, state, rep, kind, max, before, words, kind == FAME);
}

/// Fame or infamy lowered by `amount` (`RemoveReputation` with its bump,
/// `RemoveReputationExact`), at least to 0 (no ceiling).
// Translated from 00615a00 (decompiled, FalloutNV.exe 1.4.0.525;
// `TESReputation::RemoveReputationValue`, Xbox PDB), 00615fa0 the same
// with the bump.
pub fn remove(order: &LoadOrder, state: &mut GameState, rep: FormId, kind: u8, amount: f32) {
    let Some(max) = reputation_max(order, rep) else {
        return;
    };
    let before = axis_level(order, state, rep, kind, max);
    let now = (get(state, rep, kind) - amount).max(0.0);
    set_raw(state, rep, kind, now);
    let words = if kind == FAME {
        ("sRepPositiveLoss", "Reputation Loss")
    } else {
        ("sRepNegativeLoss", "Reputation Gain")
    };
    changed(order, state, rep, kind, max, before, words, kind != FAME);
}

fn axis_level(order: &LoadOrder, state: &GameState, rep: FormId, kind: u8, max: f32) -> u8 {
    level(order, get(state, rep, kind), max)
}

/// After a change: the HUD notice "<name>\n<words>" (the words' setting,
/// else the exe's default; its icon `sRep…Icon`, 2 s), and when the
/// axis's level changed the title box ([`title_box`]). `good`: fame
/// raised or infamy lowered (the box's sound, the reputation's `+0x4c`).
#[allow(clippy::too_many_arguments)]
fn changed(
    order: &LoadOrder,
    state: &mut GameState,
    rep: FormId,
    kind: u8,
    max: f32,
    before: u8,
    words: (&str, &str),
    good: bool,
) {
    // With the picture its `…Icon` setting names (the exe's defaults:
    // fame gained and infamy lost very happy, the others sad).
    use crate::message_icon::{SAD, VERY_HAPPY};
    let default_icon = match words.0 {
        "sRepPositiveGain" | "sRepNegativeLoss" => VERY_HAPPY,
        _ => SAD,
    };
    let icon = crate::message_icon::from_setting(order, &format!("{}Icon", words.0), default_icon);
    let name = reputation_name(order, rep);
    let text = game_setting_text(order, words.0).unwrap_or_else(|| words.1.into());
    state.events.push(Event::Message {
        title: None,
        text: format!("{name}\n{text}"),
        buttons: Vec::new(),
        icon: Some(icon),
    });
    if axis_level(order, state, rep, kind, max) != before {
        title_box(order, state, rep, good);
    }
}

/// The exe's descriptions of the titles (`sRepTitle…Desc`, by infamy × 4 +
/// fame; the data sets only `sRepTitlePosTwoNegThreeDesc`).
const DESCRIPTIONS: [&str; 16] = [
    "People don't know enough about you to form an opinion.",
    "Folks have come to accept you for your helpful nature.",
    "Enough news of your good works has been passed around that people like you.",
    "Renowned for your extensive support and goodwill, you are idolized by the community.",
    "You've left a poor impression on the community and may be shunned as a result.",
    "A little bit good mixed with a little bit bad, people haven't figured you out yet.",
    "People know you're good at heart even though you're occasionally a troublemaker.",
    "Your reputation as a good-natured friend of the community manages to outshine your dark side.",
    "Now that folks know you're bad, most people outright hate you.",
    "Even though you've done some good for the community, people still think you're a punk.",
    "No one's sure what to make of your unpredictable nature, but you've left a strong impression.",
    "Folks still think you're some kind of hero, but you sure can be nasty sometimes.",
    "For your overwhelmingly monstrous behavior, you have become vilified by the community.",
    "Despite your reputation as a thug, you are known to occasionally show a charitable side.",
    "Most people say you're the devil himself, but most admit you've also done a world of good.",
    "Your wild, seemingly capricious behavior leaves people scratching their heads in confusion and avoiding close contact.",
];

/// The exe's icons of the titles (`sRepTitle…Icon`), by infamy × 4 + fame.
const ICONS: [&str; 16] = [
    "neutral",
    "neutral",
    "very_happy",
    "very_happy",
    "in_pain",
    "neutral",
    "very_happy",
    "very_happy",
    "sad",
    "sad",
    "in_pain",
    "sad",
    "sad",
    "sad",
    "in_pain",
    "in_pain",
];

/// A title's description (`sRepTitle<…>Desc`, else the exe's).
pub fn title_description(order: &LoadOrder, fame: u8, infamy: u8) -> String {
    let (f, i) = (fame.min(3), infamy.min(3));
    game_setting_text(order, &format!("{}Desc", title_setting(f, i)))
        .unwrap_or_else(|| DESCRIPTIONS[usize::from(i) * 4 + usize::from(f)].to_string())
}

fn title_setting(fame: u8, infamy: u8) -> String {
    const WORDS: [&str; 4] = ["None", "One", "Two", "Three"];
    format!(
        "sRepTitlePos{}Neg{}",
        WORDS[usize::from(fame)],
        WORDS[usize::from(infamy)]
    )
}

/// The box a changed title brings up (`006155f0`,
/// `TESReputation::DisplayReputationTitleChange`, Xbox PDB): titled with the
/// reputation's name, "<title>\n<description>", the title's icon, the sound
/// `sRepChangePosSound` (`UIRepGood`) when the change was good else
/// `sRepChangeNegSound` (`UIRepBad`), one button `sOk`. (With a menu open
/// the game keeps the reputation and shows it later, `006159e0`; the
/// viewer's queue of boxes does the waiting here.)
// Translated from 006155f0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn title_box(order: &LoadOrder, state: &mut GameState, rep: FormId, good: bool) {
    // The title's box (`006155f0`) first asks for the reputation help,
    // over any menu (the box), half a second on.
    crate::tutorial::ask(
        order,
        &mut state.tutorials,
        crate::tutorial::id::REPUTATION,
        0,
        500,
    );
    let (f, i) = levels(order, state, rep);
    let text = format!("{}\n{}", title(order, f, i), title_description(order, f, i));
    let icon =
        game_setting_text(order, &format!("{}Icon", title_setting(f, i))).unwrap_or_else(|| {
            format!(
                "Interface\\Icons\\Message Icons\\glow_message_vaultboy_{}.dds",
                ICONS[usize::from(i) * 4 + usize::from(f)]
            )
        });
    let sound = if good {
        game_setting_text(order, "sRepChangePosSound").unwrap_or_else(|| "UIRepGood".into())
    } else {
        game_setting_text(order, "sRepChangeNegSound").unwrap_or_else(|| "UIRepBad".into())
    };
    state.events.push(Event::Popup {
        title: Some(reputation_name(order, rep)),
        text,
        icon: Some(icon),
        sound: Some(sound),
    });
}

/// `AddReputation` / `RemoveReputation`'s size (1–5) as points
/// (`fReputationBump…`: 1, 2, 4, 7, 12).
pub fn bump(order: &LoadOrder, size: i32) -> Option<f32> {
    let (name, default) = match size {
        1 => ("fReputationBumpVeryMinor", 1.0),
        2 => ("fReputationBumpMinor", 2.0),
        3 => ("fReputationBumpAverage", 4.0),
        4 => ("fReputationBumpMajor", 7.0),
        5 => ("fReputationBumpVeryMajor", 12.0),
        _ => return None,
    };
    Some(game_setting(order, name).unwrap_or(default))
}

/// `SetReputation`: the value as given, no clamp, no notice.
pub fn set(state: &mut GameState, rep: FormId, kind: u8, value: f32) {
    set_raw(state, rep, kind, value);
}

// ---------------------------------------------------------------------------
// Karma

/// Someone's karma band (`0047e040`): 4 very evil (≤ −750), 2 evil (≤
/// −250), 1 neutral, 0 good (≥ 250), 3 very good (≥ 750).
pub fn alignment(order: &LoadOrder, karma: f32) -> u8 {
    let s = |n: &str, d: f32| game_setting(order, n).unwrap_or(d);
    if karma <= s("fAlignVeryEvilMaxKarma", -750.0) {
        4
    } else if karma <= s("fAlignEvilMaxKarma", -250.0) {
        2
    } else if karma < s("fAlignGoodMinKarma", 250.0) {
        1
    } else if karma < s("fAlignVeryGoodMinKarma", 750.0) {
        0
    } else {
        3
    }
}

/// The player's karma.
pub fn karma(order: &LoadOrder, state: &GameState) -> f32 {
    crate::scripting::Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(PLAYER_REF, KARMA)
    .unwrap_or(0.0) as f32
}

/// The player's karma changes (`0094fd30`): clamped to `iKarmaMin` …
/// `iKarmaMax` (±1000), with the game's notice (`sKarmaMajor/Minor
/// Gained/Lost`, major past ±`iKarmaChangeThreshold` 250).
pub fn reward_karma(order: &LoadOrder, state: &mut GameState, amount: i32) {
    state.emit(crate::coop::Domain::Karma, PLAYER_REF);
    let s = |n: &str, d: f32| game_setting(order, n).unwrap_or(d) as i32;
    let k = karma(order, state) as i32;
    let (min, max) = (s("iKarmaMin", -1000.0), s("iKarmaMax", 1000.0));
    let mut a = amount;
    if a < 0 && k + a < min {
        a = min - k;
    }
    if a >= 0 && k + a > max {
        a = max - k;
    }
    let major = s("iKarmaChangeThreshold", 250.0);
    // With the picture its `…Image` setting names (`0094fd30`; the exe's
    // defaults).
    use crate::message_icon::{IN_PAIN, NEUTRAL, SAD, VERY_HAPPY};
    let (name, fallback, icon) = if a < -major {
        ("sKarmaMajorLost", "You've lost Karma!", IN_PAIN)
    } else if a < 0 {
        ("sKarmaMinorLost", "You've lost Karma!", SAD)
    } else if a < major {
        ("sKarmaMinorGained", "You've gained Karma!", NEUTRAL)
    } else {
        ("sKarmaMajorGained", "You've gained Karma!", VERY_HAPPY)
    };
    state.events.push(Event::Message {
        title: None,
        text: game_setting_text(order, name).unwrap_or_else(|| fallback.into()),
        buttons: Vec::new(),
        icon: Some(crate::message_icon::from_setting(
            order,
            &format!("{name}Image"),
            icon,
        )),
    });
    if (a > 0 && k >= max) || (a < 0 && k <= min) {
        return;
    }
    *state
        .actor_values
        .entry((PLAYER_REF, KARMA))
        .or_insert(k as f64) += f64::from(a);
}

/// The karma for the player killing someone (`0089e242`): only when they
/// are in a faction that tracks crime (`FACT` `DATA` flag 0x100), by the
/// victim's own record karma's band.
pub fn kill_karma(order: &LoadOrder, state: &GameState, victim: FormId) -> i32 {
    let tracks_crime = crate::factions::factions_of(order, state, victim)
        .into_iter()
        .any(|f| {
            order
                .get(f)
                .and_then(|r| r.record().ok())
                .and_then(|r| r.get(esm::sig::DATA).map(|s| s.data.clone()))
                .is_some_and(|d| d.len() >= 4 && le_u32(&d, 0) & 0x100 != 0)
        });
    if !tracks_crime {
        return 0;
    }
    let base = crate::scripting::Facts {
        order,
        state,
        speaker: None,
    }
    .base_actor_value(victim, KARMA)
    .unwrap_or(0.0) as f32;
    let s = |n: &str, d: f32| game_setting(order, n).unwrap_or(d);
    let creature = crate::combat::is_creature(order, victim);
    let amount = match alignment(order, base) {
        0 => s("fKarmaModMurderingGoodNPC", -50.0),
        3 => s("fKarmaModMurderingVeryGoodNPC", -100.0),
        2 => s("fKarmaModKillingEvilActor", 1.0),
        4 => s("fKarmaModKillingVeryEvilActor", 2.0),
        _ if creature => s("fKarmaModMurderingNonEvilCreature", 0.0),
        _ => s("fKarmaModMurderingNonEvilNPC", 0.0),
    };
    amount.trunc() as i32
}

/// The player's karmic title at their level (`0047e0e0`): the band's row
/// (very good uses good's, very evil evil's), `sKarmicTitle<Good|Neutral|
/// Evil><NN>` by level (30 and past the same); only the titles the data
/// sets are known here (the rest are the exe's own).
pub fn karmic_title(order: &LoadOrder, state: &GameState) -> Option<String> {
    let row = match alignment(order, karma(order, state)) {
        0 | 3 => "Good",
        2 | 4 => "Evil",
        _ => "Neutral",
    };
    let level = state.player_level.clamp(1, 30);
    game_setting_text(order, &format!("sKarmicTitle{row}{level:02}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tutorial::{self, id, Screen};

    /// A message box on top, shown.
    struct MessageBoxUp;
    impl Screen for MessageBoxUp {
        fn is_menu_open(&self, class: i32) -> bool {
            class == tutorial::menu::MESSAGE
        }
        fn top_menu_shown(&self) -> bool {
            true
        }
    }

    /// A master with a reputation (most 20) and the reputation help
    /// (`HelpReputation`, Auto Display).
    fn order() -> LoadOrder {
        use testdata::{group, record, sub, zstr};
        let mut header = 1.34f32.to_le_bytes().to_vec();
        header.extend([0; 8]);
        let mut bytes = record(b"TES4", 0, &sub(b"HEDR", &header));
        let mut rep = sub(b"EDID", &zstr("RepTest"));
        rep.extend(sub(b"FULL", &zstr("Testville")));
        rep.extend(sub(b"DATA", &20f32.to_le_bytes()));
        bytes.extend(group(*b"REPU", 0, &record(b"REPU", 0x900, &rep)));
        let mut help = sub(b"EDID", &zstr("HelpReputation"));
        help.extend(sub(b"DESC", &zstr("Text")));
        help.extend(sub(b"FULL", &zstr("Reputation")));
        help.extend(sub(b"DNAM", &3u32.to_le_bytes()));
        let form = tutorial::message_form(id::REPUTATION).0;
        bytes.extend(group(*b"MESG", 0, &record(b"MESG", form, &help)));
        let plugin = esm::Plugin::from_bytes(bytes).unwrap();
        LoadOrder::single("FalloutNV.esm", None, plugin).unwrap()
    }

    /// A new title asks for the reputation help (`006155f0`:
    /// `ShowMessage(0x27, 0, 500)`), which comes up over a message box
    /// half a second later; a change within the level doesn't.
    #[test]
    fn a_new_title_asks_for_the_reputation_help() {
        let order = order();
        let mut state = GameState::new(&order);
        let rep = FormId(0x900);
        add(&order, &mut state, rep, FAME, 1.0);
        assert_eq!(
            state.tutorials.update(1000, true, false, &MessageBoxUp),
            None
        );
        assert_eq!(
            state.tutorials.update(1600, true, false, &MessageBoxUp),
            None
        );
        add(&order, &mut state, rep, FAME, 3.0);
        assert_eq!(
            state.tutorials.update(2000, true, false, &MessageBoxUp),
            None
        );
        assert_eq!(
            state.tutorials.update(2500, true, false, &MessageBoxUp),
            Some(id::REPUTATION)
        );
    }
}
