//! Join compatibility policy: what the host does with a guest whose game is
//! not the host's game, and the player-facing words for it.
//!
//! The handshake ([`crate::proto::Hello`]) carries the protocol version, the
//! game executable version, the build hash, the rules hash, the hash of the
//! fields whose values change by themselves (volatile keys), the ordered
//! plugin list and its hash. [`issues`] compares two of them, [`decide`]
//! applies the policy (refuse, or accept with a warning when the developer
//! override `--coop-allow-mismatch` is on), [`explain`] writes the lines the
//! launcher and the HUD show.
//!
//! Protocol and rules mismatches are never overridable: the two sides cannot
//! understand each other's messages.

use crate::proto::{Hello, RejectReason};

/// Most plugins a hello may list.
pub const MAX_PLUGINS: usize = 255;
/// Longest plugin file name in a hello.
pub const MAX_PLUGIN_NAME: usize = 64;
/// Longest executable version string.
pub const MAX_EXE_LEN: usize = 32;
/// Most plugin names written out in one player-facing line.
const NAMES_SHOWN: usize = 4;

/// Host policy for a joining guest.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompatPolicy {
    /// Developer override (`--coop-allow-mismatch`): let a guest in despite
    /// executable, build, volatile-key and plugin differences, with a warning.
    pub allow_mismatch: bool,
}

/// One difference between the host's game and the guest's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Issue {
    /// Wire protocol versions differ.
    Protocol {
        /// Host's version.
        host: u16,
        /// Guest's version.
        guest: u16,
    },
    /// Different game executable versions.
    Exe {
        /// Host's version text.
        host: String,
        /// Guest's version text.
        guest: String,
    },
    /// Different nv-rs builds.
    Build,
    /// Different sync rule tables.
    Rules,
    /// Different volatile-key sets.
    Volatile,
    /// Plugins the host has that the guest lacks.
    MissingPlugins(Vec<String>),
    /// Plugins the guest has that the host lacks.
    ExtraPlugins(Vec<String>),
    /// Same plugins in a different load order.
    LoadOrder,
    /// Plugin list hashes differ and no list was given to say why.
    ModsHash,
}

impl Issue {
    /// The handshake refusal code for this difference.
    pub fn reason(&self) -> RejectReason {
        match self {
            Issue::Protocol { .. } => RejectReason::ProtocolMismatch,
            Issue::Exe { .. } => RejectReason::ExeMismatch,
            Issue::Build => RejectReason::BuildMismatch,
            Issue::Rules => RejectReason::RulesMismatch,
            Issue::Volatile => RejectReason::VolatileMismatch,
            Issue::MissingPlugins(_) | Issue::ExtraPlugins(_) => RejectReason::PluginsMismatch,
            Issue::LoadOrder => RejectReason::LoadOrderMismatch,
            Issue::ModsHash => RejectReason::ModsMismatch,
        }
    }

    /// True when no override may let the guest in.
    pub fn is_hard(&self) -> bool {
        matches!(self, Issue::Protocol { .. } | Issue::Rules)
    }

    /// What the player should do about it, in plain words.
    pub fn text(&self) -> String {
        match self {
            Issue::Protocol { host, guest } => format!(
                "The co-op versions differ (host {host}, you {guest}). Both players must install the same nv-rs co-op release."
            ),
            Issue::Exe { host, guest } => format!(
                "The game versions differ (host {}, you {}). Both players need the same game version.",
                shown(host),
                shown(guest)
            ),
            Issue::Build => "The nv-rs builds differ. Both players must run the same co-op build.".into(),
            Issue::Rules => "The sync rules differ. Both players must run the same co-op build.".into(),
            Issue::Volatile => "The synced-value tables differ. Both players must run the same co-op build.".into(),
            Issue::MissingPlugins(v) => format!(
                "You are missing {} that the host has: {}. Install {} and match the host's load order.",
                count(v.len(), "plugin"),
                names(v),
                if v.len() == 1 { "it" } else { "them" }
            ),
            Issue::ExtraPlugins(v) => format!(
                "You have {} the host does not: {}. Disable {} to join.",
                count(v.len(), "plugin"),
                names(v),
                if v.len() == 1 { "it" } else { "them" }
            ),
            Issue::LoadOrder => "Your plugins load in a different order than the host's. Match the host's load order.".into(),
            Issue::ModsHash => "Your plugin set differs from the host's. Both players need the same plugins in the same order.".into(),
        }
    }
}

fn shown(v: &str) -> &str {
    if v.is_empty() {
        "unknown"
    } else {
        v
    }
}

fn count(n: usize, noun: &str) -> String {
    if n == 1 {
        format!("1 {noun}")
    } else {
        format!("{n} {noun}s")
    }
}

fn names(v: &[String]) -> String {
    let mut s = v
        .iter()
        .take(NAMES_SHOWN)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    if v.len() > NAMES_SHOWN {
        s.push_str(&format!(" and {} more", v.len() - NAMES_SHOWN));
    }
    s
}

fn same_name(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// Every difference between the host's identity and the guest's, most
/// fundamental first. Empty when the two can play together.
pub fn issues(host: &Hello, guest: &Hello) -> Vec<Issue> {
    let mut v = Vec::new();
    if host.proto != guest.proto {
        v.push(Issue::Protocol {
            host: host.proto,
            guest: guest.proto,
        });
    }
    if host.exe != guest.exe {
        v.push(Issue::Exe {
            host: host.exe.clone(),
            guest: guest.exe.clone(),
        });
    }
    if host.build_hash != guest.build_hash {
        v.push(Issue::Build);
    }
    if host.rules_hash != guest.rules_hash {
        v.push(Issue::Rules);
    }
    if host.volatile_hash != guest.volatile_hash {
        v.push(Issue::Volatile);
    }
    let mut plugin_issue = false;
    if !host.plugins.is_empty() && !guest.plugins.is_empty() {
        let missing: Vec<String> = host
            .plugins
            .iter()
            .filter(|p| !guest.plugins.iter().any(|g| same_name(p, g)))
            .cloned()
            .collect();
        let extra: Vec<String> = guest
            .plugins
            .iter()
            .filter(|p| !host.plugins.iter().any(|h| same_name(p, h)))
            .cloned()
            .collect();
        if !missing.is_empty() {
            v.push(Issue::MissingPlugins(missing));
            plugin_issue = true;
        }
        if !extra.is_empty() {
            v.push(Issue::ExtraPlugins(extra));
            plugin_issue = true;
        }
        if !plugin_issue {
            let in_order = host.plugins.len() == guest.plugins.len()
                && host
                    .plugins
                    .iter()
                    .zip(&guest.plugins)
                    .all(|(a, b)| same_name(a, b));
            if !in_order {
                v.push(Issue::LoadOrder);
                plugin_issue = true;
            }
        }
    }
    if !plugin_issue && host.mods_hash != guest.mods_hash {
        v.push(Issue::ModsHash);
    }
    v
}

/// What the host does with a hello.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Identical games.
    Accept,
    /// Let in despite differences (the override is on).
    Warn(Vec<Issue>),
    /// Refuse with the first blocking difference's code.
    Refuse(RejectReason, Vec<Issue>),
}

/// Applies the policy to a hello.
pub fn decide(host: &Hello, guest: &Hello, policy: CompatPolicy) -> Verdict {
    let found = issues(host, guest);
    if found.is_empty() {
        return Verdict::Accept;
    }
    let blocking = found.iter().find(|i| {
        if policy.allow_mismatch {
            i.is_hard()
        } else {
            true
        }
    });
    match blocking {
        Some(i) => Verdict::Refuse(i.reason(), found.clone()),
        None => Verdict::Warn(found),
    }
}

/// Which side is reading the text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Viewpoint {
    /// The guest that was refused or warned.
    Guest,
    /// The host that turned a guest away or let one in.
    Host,
}

/// The lines for a refusal or a warning: one per difference, worded for the
/// reader. The host's wording names the guest.
pub fn explain(found: &[Issue], who: Viewpoint, guest_name: &str, refused: bool) -> Vec<String> {
    match who {
        Viewpoint::Guest => {
            let head = if refused {
                "You cannot join this game."
            } else {
                "Joined with differences (developer override):"
            };
            let mut v = vec![head.to_string()];
            v.extend(found.iter().map(Issue::text));
            v
        }
        Viewpoint::Host => {
            let head = if refused {
                format!("{guest_name} could not join: their game differs from yours.")
            } else {
                format!("{guest_name} joined with differences (developer override).")
            };
            let mut v = vec![head];
            v.extend(found.iter().map(host_text));
            v
        }
    }
}

fn host_text(i: &Issue) -> String {
    match i {
        Issue::Protocol { .. } | Issue::Build | Issue::Rules | Issue::Volatile => {
            "Different co-op build.".into()
        }
        Issue::Exe { host, guest } => {
            format!("Game version: you {}, they {}.", shown(host), shown(guest))
        }
        Issue::MissingPlugins(v) => format!("They lack: {}.", names(v)),
        Issue::ExtraPlugins(v) => format!("They also have: {}.", names(v)),
        Issue::LoadOrder => "Their load order is different.".into(),
        Issue::ModsHash => "Their plugin set is different.".into(),
    }
}

/// One line for a refusal code alone (when the guest never learned the
/// host's identity, e.g. an older host).
pub fn reason_text(r: RejectReason) -> String {
    match r {
        RejectReason::ProtocolMismatch => Issue::Protocol { host: 0, guest: 0 }
            .text()
            .replace("(host 0, you 0)", ""),
        RejectReason::ExeMismatch => {
            "The game versions differ. Both players need the same game version.".into()
        }
        RejectReason::BuildMismatch => Issue::Build.text(),
        RejectReason::RulesMismatch => Issue::Rules.text(),
        RejectReason::VolatileMismatch => Issue::Volatile.text(),
        RejectReason::PluginsMismatch => {
            "Your plugins differ from the host's. Match the host's plugin list.".into()
        }
        RejectReason::LoadOrderMismatch => Issue::LoadOrder.text(),
        RejectReason::ModsMismatch => Issue::ModsHash.text(),
        RejectReason::Full => "The session is full.".into(),
        RejectReason::SessionMismatch => "The host runs a different campaign.".into(),
        RejectReason::BadMessage => "The host did not understand the join request.".into(),
    }
}

/// Structural bounds on the listing fields of a hello.
pub fn hello_is_sane(h: &Hello) -> bool {
    h.plugins.len() <= MAX_PLUGINS
        && h.plugins.iter().all(|p| p.len() <= MAX_PLUGIN_NAME)
        && h.exe.len() <= MAX_EXE_LEN
}

/// The fields whose values change by themselves (the game clock), which the
/// sync exports on a coarse grid. Peers must agree on the set, or they would
/// disagree on what is shared state and what is presentation.
pub const VOLATILE_KEYS: &[&str] = &["GameHour", "GameDaysPassed"];

/// Hash of a volatile key set (order independent).
pub fn volatile_hash(keys: &[&str]) -> u64 {
    let mut sorted: Vec<String> = keys.iter().map(|k| k.to_ascii_lowercase()).collect();
    sorted.sort();
    plugin_list_hash(&sorted)
}

/// Hash of an ordered plugin list (case-insensitive names), for `mods_hash`.
pub fn plugin_list_hash(plugins: &[String]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for p in plugins {
        for b in p.bytes() {
            h = (h ^ u64::from(b.to_ascii_lowercase())).wrapping_mul(0x0000_0100_0000_01b3);
        }
        h = (h ^ 0xff).wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::PROTOCOL_VERSION;

    fn plugins(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    fn host() -> Hello {
        let p = plugins(&["FalloutNV.esm", "DeadMoney.esm", "Mod.esp"]);
        Hello {
            proto: PROTOCOL_VERSION,
            build_hash: 10,
            rules_hash: 20,
            mods_hash: plugin_list_hash(&p),
            exe: "1.4.0.525".into(),
            volatile_hash: 40,
            plugins: p,
        }
    }

    const STRICT: CompatPolicy = CompatPolicy {
        allow_mismatch: false,
    };
    const LOOSE: CompatPolicy = CompatPolicy {
        allow_mismatch: true,
    };

    #[test]
    fn identical_games_are_accepted() {
        assert_eq!(decide(&host(), &host(), STRICT), Verdict::Accept);
    }

    #[test]
    fn plugin_name_case_does_not_matter() {
        let mut g = host();
        g.plugins = plugins(&["falloutnv.ESM", "deadmoney.esm", "MOD.esp"]);
        assert!(issues(&host(), &g).is_empty());
    }

    #[test]
    fn each_field_is_refused_with_its_own_code() {
        let cases: Vec<(Hello, RejectReason)> = vec![
            (Hello { proto: 1, ..host() }, RejectReason::ProtocolMismatch),
            (
                Hello {
                    exe: "1.4.0.500".into(),
                    ..host()
                },
                RejectReason::ExeMismatch,
            ),
            (
                Hello {
                    build_hash: 0,
                    ..host()
                },
                RejectReason::BuildMismatch,
            ),
            (
                Hello {
                    rules_hash: 0,
                    ..host()
                },
                RejectReason::RulesMismatch,
            ),
            (
                Hello {
                    volatile_hash: 0,
                    ..host()
                },
                RejectReason::VolatileMismatch,
            ),
            (
                Hello {
                    plugins: plugins(&["FalloutNV.esm", "Mod.esp"]),
                    ..host()
                },
                RejectReason::PluginsMismatch,
            ),
            (
                Hello {
                    plugins: plugins(&["FalloutNV.esm", "DeadMoney.esm", "Mod.esp", "X.esp"]),
                    ..host()
                },
                RejectReason::PluginsMismatch,
            ),
            (
                Hello {
                    plugins: plugins(&["FalloutNV.esm", "Mod.esp", "DeadMoney.esm"]),
                    ..host()
                },
                RejectReason::LoadOrderMismatch,
            ),
            (
                Hello {
                    plugins: vec![],
                    mods_hash: 1,
                    ..host()
                },
                RejectReason::ModsMismatch,
            ),
        ];
        for (g, want) in cases {
            match decide(&host(), &g, STRICT) {
                Verdict::Refuse(r, found) => {
                    assert_eq!(r, want);
                    assert!(!found.is_empty());
                }
                other => panic!("{want:?}: {other:?}"),
            }
        }
    }

    #[test]
    fn missing_and_extra_plugins_are_named() {
        let g = Hello {
            plugins: plugins(&["FalloutNV.esm", "Other.esp"]),
            ..host()
        };
        let found = issues(&host(), &g);
        assert!(found.contains(&Issue::MissingPlugins(plugins(&[
            "DeadMoney.esm",
            "Mod.esp"
        ]))));
        assert!(found.contains(&Issue::ExtraPlugins(plugins(&["Other.esp"]))));
        let lines = explain(&found, Viewpoint::Guest, "", true);
        assert!(lines[0].contains("cannot join"));
        assert!(lines
            .iter()
            .any(|l| l.contains("DeadMoney.esm") && l.contains("missing 2 plugins")));
        assert!(lines
            .iter()
            .any(|l| l.contains("Other.esp") && l.contains("Disable it")));
        let host_lines = explain(&found, Viewpoint::Host, "Ann", true);
        assert!(host_lines[0].starts_with("Ann could not join"));
        assert!(host_lines
            .iter()
            .any(|l| l.contains("They lack: DeadMoney.esm, Mod.esp")));
    }

    #[test]
    fn long_plugin_lists_are_cut() {
        let many: Vec<String> = (0..9).map(|i| format!("P{i}.esp")).collect();
        let t = Issue::MissingPlugins(many).text();
        assert!(t.contains("and 5 more"), "{t}");
    }

    #[test]
    fn override_lets_soft_differences_in_with_a_warning() {
        let g = Hello {
            exe: "x".into(),
            plugins: plugins(&["FalloutNV.esm"]),
            build_hash: 3,
            ..host()
        };
        match decide(&host(), &g, LOOSE) {
            Verdict::Warn(found) => {
                assert!(found.iter().any(|i| matches!(i, Issue::Exe { .. })));
                assert!(found.iter().any(|i| matches!(i, Issue::Build)));
                let lines = explain(&found, Viewpoint::Guest, "", false);
                assert!(lines[0].contains("developer override"));
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn override_never_lets_protocol_or_rules_through() {
        for g in [
            Hello { proto: 1, ..host() },
            Hello {
                rules_hash: 0,
                ..host()
            },
        ] {
            assert!(matches!(decide(&host(), &g, LOOSE), Verdict::Refuse(..)));
        }
    }

    #[test]
    fn refusal_names_the_first_blocking_issue_under_the_override() {
        // exe differs (soft) and rules differ (hard): the refusal is the rules one
        let g = Hello {
            exe: "x".into(),
            rules_hash: 0,
            ..host()
        };
        match decide(&host(), &g, LOOSE) {
            Verdict::Refuse(r, _) => assert_eq!(r, RejectReason::RulesMismatch),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn every_reason_has_text() {
        use RejectReason::*;
        for r in [
            ProtocolMismatch,
            ExeMismatch,
            BuildMismatch,
            RulesMismatch,
            VolatileMismatch,
            PluginsMismatch,
            LoadOrderMismatch,
            ModsMismatch,
            Full,
            SessionMismatch,
            BadMessage,
        ] {
            assert!(reason_text(r).len() > 10, "{r:?}");
        }
    }

    #[test]
    fn sanity_bounds() {
        assert!(hello_is_sane(&host()));
        assert!(!hello_is_sane(&Hello {
            plugins: vec!["x".repeat(65)],
            ..host()
        }));
        assert!(!hello_is_sane(&Hello {
            plugins: vec!["a".into(); 256],
            ..host()
        }));
        assert!(!hello_is_sane(&Hello {
            exe: "9".repeat(33),
            ..host()
        }));
    }

    #[test]
    fn volatile_hash_ignores_order_and_case() {
        assert_eq!(
            volatile_hash(&["GameHour", "GameDaysPassed"]),
            volatile_hash(&["gamedayspassed", "GAMEHOUR"])
        );
        assert_ne!(volatile_hash(VOLATILE_KEYS), volatile_hash(&["GameHour"]));
    }

    #[test]
    fn plugin_hash_is_order_sensitive() {
        assert_ne!(
            plugin_list_hash(&plugins(&["a", "b"])),
            plugin_list_hash(&plugins(&["b", "a"]))
        );
        assert_ne!(
            plugin_list_hash(&plugins(&["ab"])),
            plugin_list_hash(&plugins(&["a", "b"]))
        );
        assert_eq!(
            plugin_list_hash(&plugins(&["A"])),
            plugin_list_hash(&plugins(&["a"]))
        );
    }
}
