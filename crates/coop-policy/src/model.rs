//! Data model of the policy file (`data/policy.json`). Pure data, no logic.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Plain form id as stored in the source plugin (DLC ids keep their original load-index byte).
pub type FormId = u32;

/// Form ids are written as `"0x0012ABCD"` in the JSON so rows stay greppable.
pub mod hex_id {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &u32, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("0x{v:08X}"))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
        let t = String::deserialize(d)?;
        let h = t
            .strip_prefix("0x")
            .ok_or_else(|| serde::de::Error::custom(format!("form id {t:?} must start with 0x")))?;
        u32::from_str_radix(h, 16)
            .map_err(|e| serde::de::Error::custom(format!("form id {t:?}: {e}")))
    }
}

fn is_default<T: Default + PartialEq>(v: &T) -> bool {
    *v == T::default()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyFile {
    pub schema: u32,
    pub policy: GlobalPolicy,
    /// Per-player shadow variables (skills, followers, organs...) versus shared state (karma, rep).
    pub shadow_classes: BTreeMap<ShadowClass, Residence>,
    #[serde(default)]
    pub groups: Vec<ExclusiveGroup>,
    pub quests: Vec<QuestRow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalPolicy {
    pub death: Death,
    pub late_join: LateJoin,
    /// Seconds before an unanswered barrier or ready check is cancelled.
    pub barrier_timeout_s: u32,
    /// Seconds a tether holds a stage before stragglers are warped (or the wait continues).
    pub tether_timeout_s: u32,
    pub follower_cap: FollowerCap,
    pub party_cap: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Death {
    pub mode: DeathMode,
    pub revive_s: u32,
    pub all_down: AllDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeathMode {
    Downed,
    Respawn,
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AllDown {
    ReloadHostSave,
    RespawnAtCheckpoint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LateJoin {
    pub during_scene: LateJoinDuring,
    pub after_scene: LateJoinAfter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LateJoinDuring {
    Wait,
    Refuse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LateJoinAfter {
    WarpToZone,
    StayPut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FollowerCap {
    pub humanoid: u8,
    pub critter: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShadowClass {
    Skills,
    Perks,
    Followers,
    Organs,
    Implants,
    Collar,
    Casino,
    Storage,
    QuestStage,
    QuestVar,
    JournalFocus,
    Achievements,
    Karma,
    Reputation,
    Faction,
    QuestItem,
    World,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Residence {
    Shared,
    PerPlayer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExclusiveGroup {
    pub name: String,
    /// Quest EDIDs; the first one committed wins.
    pub members: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Reviewed by hand in the old table.
    Confirmed,
    /// Derived from risk categories, never reviewed.
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    T0,
    T1,
    T2,
    T3,
    T4,
}

/// Who may advance a stage of the quest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Advance {
    /// Host-run scripts only (endings, timers, cinematics).
    #[default]
    Host,
    /// The player who started the dialogue or activation.
    Initiator,
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// One quest state for the party.
    #[default]
    Shared,
    /// Each player runs and tracks the quest separately.
    PerPlayer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reward {
    /// Every player receives the full reward once.
    #[default]
    Everyone,
    Initiator,
    /// The reward is divided between present players.
    Split,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemRoute {
    #[default]
    All,
    Initiator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostDecision {
    Ending,
    Time,
    Cinematic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gather {
    #[default]
    Wait,
    AutoWarp,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tether {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cells: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<u32>,
    /// Stages held by the tether; empty means every stage.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<u32>,
    #[serde(default)]
    pub gather: Gather,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_s: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Barrier {
    /// One-way stages that need every present player to pass first.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<u32>,
    /// Named gate in front of a host scene (`slideshow`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ready {
    pub infos: Vec<String>,
    pub timeout_s: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exclusive {
    pub stages: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentMode {
    Vote,
    Unanimous,
    Host,
    Initiator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Consent {
    pub mode: ConsentMode,
    pub timeout_s: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub boxes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Absent {
    #[default]
    Hold,
    Warp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Warp {
    pub markers: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub absent: Absent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlScope {
    #[default]
    All,
    Initiator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Controls {
    #[serde(default, skip_serializing_if = "is_default")]
    pub scope: ControlScope,
    pub timeout_s: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub masks: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionOwner {
    /// The recruiting player owns the companion (caps, commands, perks).
    #[default]
    Recruiter,
    /// Plot companion belonging to the whole party.
    Party,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Companion {
    pub owner: CompanionOwner,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub npcs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Perks {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub all: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initiator: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recruiter: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DownMode {
    UntilCheckpoint,
}

/// Per-player copies of quest script state (Dead Money collars, Old World Blues organs...).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shadow {
    pub scope: String,
    pub vars: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub objvars: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scripts: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checkpoint: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub down: Option<DownMode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FriendlyFire {
    pub actors: Vec<String>,
    pub deliberate_hits: u32,
    pub window_s: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Once {
    pub window_s: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<String>,
}

/// One row of the policy table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestRow {
    pub edid: String,
    #[serde(with = "hex_id")]
    pub id: FormId,
    /// The quest's id inside its own plugin (top byte clear). With `plugin` this is the
    /// load-order independent key; `id` is only what the quest has when `plugin` loads
    /// straight after the base game, so a session remaps rows with `PolicyTable::remap`.
    #[serde(with = "hex_id")]
    pub local: FormId,
    /// The id is made up by the importer (no quest record found).
    #[serde(default, skip_serializing_if = "is_default")]
    pub synthetic: bool,
    pub plugin: String,
    pub tier: Tier,
    pub line: String,
    pub status: Status,
    pub advance: Advance,
    #[serde(default, skip_serializing_if = "is_default")]
    pub scope: Scope,
    #[serde(default, skip_serializing_if = "is_default")]
    pub reward: Reward,
    #[serde(default, skip_serializing_if = "is_default")]
    pub quest_items: ItemRoute,
    /// Debug or test quest: no gating at all.
    #[serde(default, skip_serializing_if = "is_default")]
    pub gate_exempt: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_decision: Option<HostDecision>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pooled_items: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tether: Option<Tether>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barrier: Option<Barrier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<Ready>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<Exclusive>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent: Option<Consent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warp: Option<Warp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controls: Option<Controls>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub save_suppress: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub fast_travel_lock: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companion: Option<Companion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perks: Option<Perks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadow: Option<Shadow>,
    /// Scripts a guest may run locally (presentation only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guest_local: Vec<String>,
    /// Containers that hold one inventory per player (gear confiscation, entry strips):
    /// reference editor ids or `plugin:id`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub friendly_fire: Option<FriendlyFire>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub once: Option<Once>,
    /// Condition function -> reducer over the party (initiator, any, min-over-party...).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub party_cond: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
}

impl QuestRow {
    /// A neutral row used for quests the table does not know: host advances, everything shared.
    pub fn fallback(id: FormId) -> QuestRow {
        QuestRow {
            edid: String::new(),
            id,
            local: id & 0x00FF_FFFF,
            synthetic: false,
            plugin: String::new(),
            tier: Tier::T0,
            line: String::new(),
            status: Status::Default,
            advance: Advance::Host,
            scope: Scope::Shared,
            reward: Reward::Everyone,
            quest_items: ItemRoute::All,
            gate_exempt: false,
            host_decision: None,
            pooled_items: vec![],
            tether: None,
            barrier: None,
            ready: None,
            exclusive: None,
            consent: None,
            warp: None,
            controls: None,
            save_suppress: false,
            fast_travel_lock: false,
            companion: None,
            perks: None,
            shadow: None,
            guest_local: vec![],
            storage: vec![],
            friendly_fire: None,
            once: None,
            party_cond: BTreeMap::new(),
            note: String::new(),
        }
    }
}
