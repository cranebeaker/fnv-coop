//! Party saves: the on-disk unit of a co-op campaign.
//!
//! A party save is a folder (`<save>.partysave`) holding
//!
//! ```text
//! party.txt              manifest: version, campaign id, roster, sizes and hashes
//! world.txt              the host's world save (`save::save`: shared state + the host's character)
//! guest-<key>.char       one sidecar per guest: that guest's character (a shadow blob)
//! ```
//!
//! The folder is written as one unit: everything goes into a sibling temporary folder and the
//! folder is swapped in with renames, so a failed write leaves the previous party save untouched
//! and a reader never sees half of a new one. The manifest records the length and hash of every
//! file, so a truncated or edited file is found on load.
//!
//! Loading is forgiving about guests and strict about the world: a missing or corrupted sidecar
//! costs that guest its saved character (it gets a fresh one, reported as a [`Problem`]); a bad
//! world file or a newer format refuses the whole save.
//!
//! Synthetic forms never reach disk. Puppets (`0xFE000000..`) stand for remote players and
//! shadow forms (`0xFC000000..`) carry their state on the wire; both are removed from the world
//! text and from every sidecar by [`PartySave::scrub`], which [`write_party`] always runs.

use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use esm::FormId;

use super::facts::PUPPET_BASE;
use super::shadow::{decode, encode, PlayerShadow, SHADOW_FORM_BASE};

/// Newest format this build writes and reads.
pub const FORMAT_VERSION: u32 = 1;
/// First line of the manifest (without the version).
pub const MANIFEST_MAGIC: &str = "nv-rs party save";
/// Manifest file name.
pub const MANIFEST: &str = "party.txt";
/// World save file name.
pub const WORLD: &str = "world.txt";
/// Folder suffix appended to a save's path.
pub const FOLDER_SUFFIX: &str = "partysave";

/// 64-bit FNV-1a, the hash of every file in the folder.
pub fn fnv64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// A campaign id as 16 lower-case hex digits.
pub fn campaign_hex(id: u64) -> String {
    format!("{id:016x}")
}

/// Parses 16 hex digits; zero is not a campaign.
pub fn parse_campaign(s: &str) -> Option<u64> {
    let t = s.trim();
    if t.len() != 16 || !t.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    u64::from_str_radix(t, 16).ok().filter(|v| *v != 0)
}

/// True for a form that stands for a remote player: a puppet or a shadow carrier.
pub fn is_synthetic(f: FormId) -> bool {
    (PUPPET_BASE..PUPPET_BASE + 256).contains(&f.0)
        || (SHADOW_FORM_BASE..SHADOW_FORM_BASE + 256).contains(&f.0)
}

/// The folder a save at `save_path` keeps its party data in.
pub fn party_dir_for(save_path: &Path) -> PathBuf {
    let mut n = save_path
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_default();
    n.push(format!(".{FOLDER_SUFFIX}"));
    save_path.with_file_name(n)
}

/// File name of a guest's sidecar.
pub fn sidecar_name(key: u64) -> String {
    format!("guest-{}.char", campaign_hex(key))
}

/// One guest's character in a party save.
#[derive(Clone, Debug, PartialEq)]
pub struct GuestSidecar {
    /// The guest's persistent player id.
    pub key: u64,
    /// Display name at save time.
    pub name: String,
    /// Slot it had in the session (informational: slots are not stable).
    pub slot: u8,
    /// Level at save time (for lists).
    pub level: u16,
    /// True when the guest itself answered the save order; false when the host wrote its own
    /// copy of the guest's shadow because the guest did not answer in time.
    pub from_guest: bool,
    /// The character: a [`shadow::encode`](super::shadow::encode) blob.
    pub blob: Vec<u8>,
}

impl GuestSidecar {
    /// Builds a sidecar from a shadow.
    pub fn from_shadow(
        key: u64,
        name: &str,
        slot: u8,
        from_guest: bool,
        s: &PlayerShadow,
    ) -> GuestSidecar {
        GuestSidecar {
            key,
            name: name.to_string(),
            slot,
            level: s.level,
            from_guest,
            blob: encode(s),
        }
    }

    /// The character, `None` when the blob does not parse.
    pub fn shadow(&self) -> Option<PlayerShadow> {
        decode(&self.blob)
    }
}

/// The host's campaign rules: the difficulty mode and friendly fire. The
/// launcher and viewer flags choose them, and the party save keeps them
/// (manifest line `rules=MODE on|off`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PartyRules {
    /// Normal or Co-op balanced.
    pub mode: coop_core::rules::Mode,
    /// Players' hits on other players count.
    pub friendly_fire: bool,
}

impl Default for PartyRules {
    /// Co-op balanced, friendly fire off (a session without a choice).
    fn default() -> Self {
        PartyRules {
            mode: coop_core::rules::Mode::CoopBalanced,
            friendly_fire: false,
        }
    }
}

impl PartyRules {
    /// `normal` or `balanced` (the flag's word).
    pub fn mode_name(&self) -> &'static str {
        match self.mode {
            coop_core::rules::Mode::Normal => "normal",
            coop_core::rules::Mode::CoopBalanced => "balanced",
        }
    }

    /// `on` or `off` (the flag's word).
    pub fn friendly_fire_name(&self) -> &'static str {
        if self.friendly_fire {
            "on"
        } else {
            "off"
        }
    }

    /// A mode word: `normal` or `balanced`.
    pub fn parse_mode(s: &str) -> Option<coop_core::rules::Mode> {
        match s.trim().to_ascii_lowercase().as_str() {
            "normal" => Some(coop_core::rules::Mode::Normal),
            "balanced" | "coop-balanced" | "coop_balanced" => {
                Some(coop_core::rules::Mode::CoopBalanced)
            }
            _ => None,
        }
    }

    /// A switch word: `on` or `off`.
    pub fn parse_switch(s: &str) -> Option<bool> {
        match s.trim().to_ascii_lowercase().as_str() {
            "on" => Some(true),
            "off" => Some(false),
            _ => None,
        }
    }

    /// The manifest value: `MODE on|off`.
    fn text(&self) -> String {
        format!("{} {}", self.mode_name(), self.friendly_fire_name())
    }

    /// Reads the manifest value.
    fn parse(s: &str) -> Option<PartyRules> {
        let mut w = s.split_whitespace();
        Some(PartyRules {
            mode: Self::parse_mode(w.next()?)?,
            friendly_fire: Self::parse_switch(w.next()?)?,
        })
    }
}

/// A whole party save.
#[derive(Clone, Debug, PartialEq)]
pub struct PartySave {
    /// Campaign id (never zero).
    pub campaign: u64,
    /// The host's display name.
    pub host_name: String,
    /// Increases with every save of a campaign in one run.
    pub seq: u32,
    /// Seconds since the Unix epoch.
    pub saved_unix: u64,
    /// The host's world save text.
    pub world_text: String,
    /// One per guest, ordered by key.
    pub guests: Vec<GuestSidecar>,
    /// The host's campaign rules (a save without the line: the default).
    pub rules: PartyRules,
}

/// What can go wrong with a party save.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PartyError {
    /// A file operation failed.
    Io(String),
    /// The save is from a newer format.
    Version { found: u32, supported: u32 },
    /// The manifest cannot be understood.
    BadManifest(String),
    /// The world file is missing, truncated or edited.
    WorldBad(String),
    /// The save has no campaign.
    NoCampaign,
}

impl std::fmt::Display for PartyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartyError::Io(e) => write!(f, "party save: {e}"),
            PartyError::Version { found, supported } => {
                write!(
                    f,
                    "party save is format {found}, this build reads up to {supported}"
                )
            }
            PartyError::BadManifest(e) => write!(f, "party save manifest: {e}"),
            PartyError::WorldBad(e) => write!(f, "party save world file: {e}"),
            PartyError::NoCampaign => write!(f, "party save has no campaign id"),
        }
    }
}

impl std::error::Error for PartyError {}

impl From<io::Error> for PartyError {
    fn from(e: io::Error) -> Self {
        PartyError::Io(e.to_string())
    }
}

/// A guest sidecar that could not be used on load.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    /// The manifest lists the guest but the file is gone.
    SidecarMissing(u64),
    /// The file is truncated, edited or from an unknown character format.
    SidecarCorrupt(u64),
    /// Synthetic forms were found in the save and dropped.
    SyntheticDropped(usize),
}

/// A loaded party save and what was wrong with it.
#[derive(Clone, Debug, PartialEq)]
pub struct LoadedParty {
    /// The save with only the usable guests.
    pub save: PartySave,
    /// Guests that were listed but could not be loaded, and other repairs.
    pub problems: Vec<Problem>,
}

impl LoadedParty {
    /// The sidecar of a guest by its player id.
    pub fn guest(&self, key: u64) -> Option<&GuestSidecar> {
        self.save.guests.iter().find(|g| g.key == key)
    }
}

/// The steps of a write, for failure injection in tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// A file of the new folder is about to be written (index in write order).
    File(usize),
    /// The folders are about to be swapped.
    Swap,
}

// ---- scrubbing ------------------------------------------------------------

fn token_is_synthetic(tok: &str) -> bool {
    tok.len() == 8
        && tok.bytes().all(|b| b.is_ascii_hexdigit())
        && u32::from_str_radix(tok, 16).is_ok_and(|v| is_synthetic(FormId(v)))
}

/// Removes every line of a world save that names a synthetic form. Returns the text and how
/// many lines were dropped. A `var` line's name (third word) is not a form and is not looked at.
pub fn scrub_world_text(text: &str) -> (String, usize) {
    let mut out = String::with_capacity(text.len());
    let mut dropped = 0;
    for line in text.lines() {
        let mut words = line.split_whitespace();
        let first = words.next().unwrap_or("");
        let hit = words
            .enumerate()
            .any(|(i, w)| !(first == "var" && i == 1) && token_is_synthetic(w));
        if hit {
            dropped += 1;
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    (out, dropped)
}

/// Removes synthetic forms from a character. Returns how many rows were dropped.
pub fn scrub_shadow(s: &mut PlayerShadow) -> usize {
    let mut n = 0;
    let bad = |f: &FormId| is_synthetic(*f);
    macro_rules! map {
        ($m:expr) => {{
            let before = $m.len();
            $m.retain(|k, _| !bad(k));
            n += before - $m.len();
        }};
    }
    macro_rules! set {
        ($m:expr) => {{
            let before = $m.len();
            $m.retain(|k| !bad(k));
            n += before - $m.len();
        }};
    }
    map!(s.items);
    map!(s.weapon_health);
    map!(s.weapon_mods);
    set!(s.dropped);
    set!(s.equip_locked);
    set!(s.perks);
    map!(s.perk_ranks);
    set!(s.teammates);
    map!(s.quest_stage);
    map!(s.quest_flags);
    let b = s.stages_done.len();
    s.stages_done.retain(|(q, _)| !bad(q));
    n += b - s.stages_done.len();
    let b = s.objectives.len();
    s.objectives.retain(|(q, _), _| !bad(q));
    n += b - s.objectives.len();
    let b = s.quest_vars.len();
    s.quest_vars.retain(|(q, _), _| !bad(q));
    n += b - s.quest_vars.len();
    if let Some(e) = &mut s.equipped {
        let b = e.len();
        e.retain(|f| !bad(f));
        n += b - e.len();
    }
    if s.ammo_loaded.is_some_and(|f| bad(&f)) {
        s.ammo_loaded = None;
        n += 1;
    }
    for h in s.hotkeys.iter_mut() {
        if h.is_some_and(|f| bad(&f)) {
            *h = None;
            n += 1;
        }
    }
    for f in [&mut s.cell, &mut s.world] {
        if f.is_some_and(|x| bad(&x)) {
            *f = None;
            n += 1;
        }
    }
    n
}

impl PartySave {
    /// Drops synthetic forms from the world text and from every sidecar. Returns the count.
    pub fn scrub(&mut self) -> usize {
        let (text, mut n) = scrub_world_text(&self.world_text);
        self.world_text = text;
        for g in &mut self.guests {
            if let Some(mut s) = g.shadow() {
                let d = scrub_shadow(&mut s);
                if d > 0 {
                    g.blob = encode(&s);
                    n += d;
                }
            }
        }
        n
    }
}

// ---- file formats ---------------------------------------------------------

const SIDE_MAGIC: &[u8; 4] = b"NVPC";
const SIDE_VERSION: u8 = 1;

fn side_bytes(g: &GuestSidecar) -> Vec<u8> {
    let mut b = Vec::with_capacity(g.blob.len() + 32);
    b.extend_from_slice(SIDE_MAGIC);
    b.push(SIDE_VERSION);
    b.extend_from_slice(&g.key.to_le_bytes());
    b.extend_from_slice(&(g.blob.len() as u32).to_le_bytes());
    b.extend_from_slice(&g.blob);
    let h = fnv64(&b);
    b.extend_from_slice(&h.to_le_bytes());
    b
}

fn side_parse(b: &[u8]) -> Option<(u64, Vec<u8>)> {
    if b.len() < 4 + 1 + 8 + 4 + 8 || &b[..4] != SIDE_MAGIC || b[4] != SIDE_VERSION {
        return None;
    }
    let (body, tail) = b.split_at(b.len() - 8);
    if fnv64(body) != u64::from_le_bytes(tail.try_into().ok()?) {
        return None;
    }
    let key = u64::from_le_bytes(body[5..13].try_into().ok()?);
    let n = u32::from_le_bytes(body[13..17].try_into().ok()?) as usize;
    (body.len() == 17 + n).then(|| (key, body[17..].to_vec()))
}

fn clean_line(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect()
}

fn manifest_text(save: &PartySave, world: &[u8], sides: &[(u64, Vec<u8>)]) -> String {
    let mut t = format!("{MANIFEST_MAGIC} {FORMAT_VERSION}\n");
    t.push_str(&format!("campaign={}\n", campaign_hex(save.campaign)));
    t.push_str(&format!("host={}\n", clean_line(&save.host_name)));
    t.push_str(&format!("seq={}\n", save.seq));
    t.push_str(&format!("saved={}\n", save.saved_unix));
    t.push_str(&format!("world={} {:016x}\n", world.len(), fnv64(world)));
    for (g, (_, bytes)) in save.guests.iter().zip(sides) {
        t.push_str(&format!(
            "guest={} {} {} {} {} {:016x} {}\n",
            campaign_hex(g.key),
            g.slot,
            g.level,
            u8::from(g.from_guest),
            bytes.len(),
            fnv64(bytes),
            clean_line(&g.name)
        ));
    }
    // after the pinned lines the launcher reads (it skips keys it does not know)
    t.push_str(&format!("rules={}\n", save.rules.text()));
    t
}

// ---- writing --------------------------------------------------------------

fn write_file(path: &Path, bytes: &[u8]) -> io::Result<()> {
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    f.write_all(bytes)?;
    f.sync_all()
}

fn sibling(dir: &Path, tag: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let mut n = dir
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_default();
    n.push(format!(
        ".{}.{}.{tag}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    dir.with_file_name(n)
}

/// Writes `save` to `dir` as one unit (see the module docs). Synthetic forms are scrubbed first.
pub fn write_party(dir: &Path, save: &PartySave) -> Result<(), PartyError> {
    write_party_with(dir, save, &mut |_| Ok(()))
}

/// [`write_party`] with a hook called before every step; an error from it aborts the write like
/// an I/O failure would (tests use this to prove the old save survives).
pub fn write_party_with(
    dir: &Path,
    save: &PartySave,
    hook: &mut dyn FnMut(Step) -> io::Result<()>,
) -> Result<(), PartyError> {
    if save.campaign == 0 {
        return Err(PartyError::NoCampaign);
    }
    let mut save = save.clone();
    save.guests.sort_by_key(|g| g.key);
    save.guests.dedup_by_key(|g| g.key);
    save.scrub();
    let tmp = sibling(dir, "tmp");
    let result = (|| -> Result<(), PartyError> {
        if let Some(parent) = dir.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        fs::create_dir(&tmp)?;
        let world = save.world_text.as_bytes();
        let sides: Vec<(u64, Vec<u8>)> =
            save.guests.iter().map(|g| (g.key, side_bytes(g))).collect();
        let mut i = 0;
        hook(Step::File(i))?;
        write_file(&tmp.join(WORLD), world)?;
        for (key, bytes) in &sides {
            i += 1;
            hook(Step::File(i))?;
            write_file(&tmp.join(sidecar_name(*key)), bytes)?;
        }
        hook(Step::File(i + 1))?;
        write_file(
            &tmp.join(MANIFEST),
            manifest_text(&save, world, &sides).as_bytes(),
        )?;
        hook(Step::Swap)?;
        let old = sibling(dir, "old");
        let had = dir.exists();
        if had {
            fs::rename(dir, &old)?;
        }
        if let Err(e) = fs::rename(&tmp, dir) {
            if had {
                let _ = fs::rename(&old, dir);
            }
            return Err(e.into());
        }
        if had {
            let _ = fs::remove_dir_all(&old);
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_dir_all(&tmp);
    }
    result
}

// ---- reading --------------------------------------------------------------

/// What the manifest says, without reading the other files (for lists).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Info {
    /// Format version.
    pub version: u32,
    /// Campaign id.
    pub campaign: u64,
    /// Host name.
    pub host_name: String,
    /// Save sequence.
    pub seq: u32,
    /// Seconds since the Unix epoch.
    pub saved_unix: u64,
    /// Roster: (key, name, level).
    pub roster: Vec<(u64, String, u16)>,
}

struct Listed {
    key: u64,
    slot: u8,
    level: u16,
    from_guest: bool,
    len: usize,
    hash: u64,
    name: String,
}

struct Manifest {
    info: Info,
    world_len: usize,
    world_hash: u64,
    guests: Vec<Listed>,
    rules: PartyRules,
}

fn parse_manifest(text: &str) -> Result<Manifest, PartyError> {
    let mut lines = text.lines();
    let first = lines.next().unwrap_or("");
    let bad = |m: &str| PartyError::BadManifest(m.to_string());
    let v = first
        .strip_prefix(MANIFEST_MAGIC)
        .ok_or_else(|| bad("not a party save"))?;
    let version: u32 = v.trim().parse().map_err(|_| bad("no version"))?;
    if version > FORMAT_VERSION || version == 0 {
        return Err(PartyError::Version {
            found: version,
            supported: FORMAT_VERSION,
        });
    }
    let (mut campaign, mut host, mut seq, mut saved) = (None, String::new(), 0u32, 0u64);
    let mut world = None;
    let mut guests = Vec::new();
    let mut rules = PartyRules::default();
    for l in lines {
        let Some((k, v)) = l.split_once('=') else {
            continue;
        };
        match k {
            "campaign" => campaign = parse_campaign(v),
            "host" => host = v.to_string(),
            "seq" => seq = v.parse().map_err(|_| bad("seq"))?,
            "saved" => saved = v.parse().map_err(|_| bad("saved"))?,
            "rules" => rules = PartyRules::parse(v).ok_or_else(|| bad("rules"))?,
            "world" => {
                let mut w = v.split_whitespace();
                let len = w
                    .next()
                    .and_then(|x| x.parse().ok())
                    .ok_or_else(|| bad("world length"))?;
                let hash = w
                    .next()
                    .and_then(|x| u64::from_str_radix(x, 16).ok())
                    .ok_or_else(|| bad("world hash"))?;
                world = Some((len, hash));
            }
            "guest" => {
                let mut w = v.splitn(7, ' ');
                let mut next = || w.next().ok_or_else(|| bad("guest line"));
                let key = parse_campaign(next()?).ok_or_else(|| bad("guest key"))?;
                let slot = next()?.parse().map_err(|_| bad("guest slot"))?;
                let level = next()?.parse().map_err(|_| bad("guest level"))?;
                let from_guest = next()? == "1";
                let len = next()?.parse().map_err(|_| bad("guest length"))?;
                let hash = u64::from_str_radix(next()?, 16).map_err(|_| bad("guest hash"))?;
                let name = next().map(str::to_string).unwrap_or_default();
                guests.push(Listed {
                    key,
                    slot,
                    level,
                    from_guest,
                    len,
                    hash,
                    name,
                });
            }
            _ => {}
        }
    }
    let campaign = campaign.ok_or(PartyError::NoCampaign)?;
    let (world_len, world_hash) = world.ok_or_else(|| bad("no world line"))?;
    let roster = guests
        .iter()
        .map(|g| (g.key, g.name.clone(), g.level))
        .collect();
    Ok(Manifest {
        info: Info {
            version,
            campaign,
            host_name: host,
            seq,
            saved_unix: saved,
            roster,
        },
        world_len,
        world_hash,
        guests,
        rules,
    })
}

/// If `dir` is missing but a crash left `<dir>.<pid>.<n>.old` behind, that is the last good save.
fn recover(dir: &Path) -> Option<PathBuf> {
    if dir.is_dir() {
        return Some(dir.to_path_buf());
    }
    let name = dir.file_name()?.to_string_lossy().into_owned();
    let parent = dir
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut best: Option<(std::time::SystemTime, PathBuf)> = None;
    for e in fs::read_dir(parent).ok()?.flatten() {
        let n = e.file_name().to_string_lossy().into_owned();
        if n.starts_with(&format!("{name}.")) && n.ends_with(".old") && e.path().is_dir() {
            let t = e
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::UNIX_EPOCH);
            if best.as_ref().is_none_or(|(bt, _)| t > *bt) {
                best = Some((t, e.path()));
            }
        }
    }
    best.map(|b| b.1)
}

/// Reads only the manifest (roster and time for lists).
pub fn read_info(dir: &Path) -> Result<Info, PartyError> {
    let d =
        recover(dir).ok_or_else(|| PartyError::Io(format!("{} is not there", dir.display())))?;
    let text = fs::read_to_string(d.join(MANIFEST))?;
    Ok(parse_manifest(&text)?.info)
}

/// Reads and checks a party save.
pub fn load_party(dir: &Path) -> Result<LoadedParty, PartyError> {
    let d =
        recover(dir).ok_or_else(|| PartyError::Io(format!("{} is not there", dir.display())))?;
    let m = parse_manifest(&fs::read_to_string(d.join(MANIFEST))?)?;
    let world = fs::read(d.join(WORLD)).map_err(|e| PartyError::WorldBad(e.to_string()))?;
    if world.len() != m.world_len || fnv64(&world) != m.world_hash {
        return Err(PartyError::WorldBad(
            "size or hash differs from the manifest".into(),
        ));
    }
    let world_text =
        String::from_utf8(world).map_err(|_| PartyError::WorldBad("not text".into()))?;
    let mut problems = Vec::new();
    let mut guests = Vec::new();
    let mut seen = BTreeSet::new();
    for g in &m.guests {
        if !seen.insert(g.key) {
            continue;
        }
        let Ok(bytes) = fs::read(d.join(sidecar_name(g.key))) else {
            problems.push(Problem::SidecarMissing(g.key));
            continue;
        };
        let ok = bytes.len() == g.len && fnv64(&bytes) == g.hash;
        match ok.then(|| side_parse(&bytes)).flatten() {
            Some((key, blob)) if key == g.key && decode(&blob).is_some() => {
                guests.push(GuestSidecar {
                    key,
                    name: g.name.clone(),
                    slot: g.slot,
                    level: g.level,
                    from_guest: g.from_guest,
                    blob,
                })
            }
            _ => problems.push(Problem::SidecarCorrupt(g.key)),
        }
    }
    let mut save = PartySave {
        campaign: m.info.campaign,
        host_name: m.info.host_name,
        seq: m.info.seq,
        saved_unix: m.info.saved_unix,
        world_text,
        guests,
        rules: m.rules,
    };
    let dropped = save.scrub();
    if dropped > 0 {
        problems.push(Problem::SyntheticDropped(dropped));
    }
    Ok(LoadedParty { save, problems })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coop::shadow::PlayerShadow;

    fn tmp(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("nvrs-party-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn shadow(level: u16, caps: i32) -> PlayerShadow {
        let mut s = PlayerShadow::fresh();
        s.level = level;
        s.items.insert(FormId(0xF), caps);
        s.name = Some("Ann".into());
        s
    }

    fn sample() -> PartySave {
        PartySave {
            campaign: 0x00ab_12cd_34ef_5678,
            host_name: "Host".into(),
            seq: 3,
            saved_unix: 1_700_000_000,
            world_text: "nv-rs save 1\nlevel 4\nstage 00104C1C 55\n".into(),
            guests: vec![
                GuestSidecar::from_shadow(2, "Bo", 2, true, &shadow(7, 40)),
                GuestSidecar::from_shadow(1, "Ann", 1, false, &shadow(3, 9)),
            ],
            rules: PartyRules {
                mode: coop_core::rules::Mode::Normal,
                friendly_fire: true,
            },
        }
    }

    #[test]
    fn round_trips_sorted_by_key() {
        let d = tmp("rt");
        let p = d.join("Save 1.txt.partysave");
        let mut s = sample();
        write_party(&p, &s).unwrap();
        let l = load_party(&p).unwrap();
        assert!(l.problems.is_empty());
        s.guests.sort_by_key(|g| g.key);
        assert_eq!(l.save, s);
        assert_eq!(l.guest(2).unwrap().shadow().unwrap().level, 7);
        let info = read_info(&p).unwrap();
        assert_eq!(info.roster.len(), 2);
        assert_eq!(info.campaign, s.campaign);
        assert_eq!(info.seq, 3);
        // writing over an existing save replaces it whole
        let mut s2 = sample();
        s2.seq = 4;
        s2.guests.truncate(1);
        write_party(&p, &s2).unwrap();
        let l2 = load_party(&p).unwrap();
        assert_eq!(l2.save.seq, 4);
        assert_eq!(l2.save.guests.len(), 1);
        assert!(!p.join(sidecar_name(1)).exists());
        let leftovers: Vec<_> = fs::read_dir(&d).unwrap().flatten().collect();
        assert_eq!(leftovers.len(), 1, "no temporary or old folder is left");
    }

    #[test]
    fn a_failed_write_leaves_the_previous_save_untouched() {
        let d = tmp("atomic");
        let p = d.join("a.txt.partysave");
        write_party(&p, &sample()).unwrap();
        let before = load_party(&p).unwrap();
        for fail_at in [
            Step::File(0),
            Step::File(1),
            Step::File(2),
            Step::File(3),
            Step::Swap,
        ] {
            let mut next = sample();
            next.seq = 99;
            let r = write_party_with(&p, &next, &mut |s| {
                if s == fail_at {
                    Err(io::Error::other("disk full"))
                } else {
                    Ok(())
                }
            });
            assert!(matches!(r, Err(PartyError::Io(_))), "{fail_at:?}");
            assert_eq!(load_party(&p).unwrap(), before, "{fail_at:?}");
            assert_eq!(
                fs::read_dir(&d).unwrap().count(),
                1,
                "temporary folder removed after {fail_at:?}"
            );
        }
        // a first write that fails leaves nothing
        let q = d.join("b.txt.partysave");
        let r = write_party_with(&q, &sample(), &mut |s| {
            if s == Step::Swap {
                Err(io::Error::other("x"))
            } else {
                Ok(())
            }
        });
        assert!(r.is_err());
        assert!(!q.exists());
    }

    #[test]
    fn a_crash_between_the_swap_renames_recovers_the_old_save() {
        let d = tmp("crash");
        let p = d.join("c.txt.partysave");
        write_party(&p, &sample()).unwrap();
        let old = sibling(&p, "old");
        fs::rename(&p, &old).unwrap();
        assert!(
            load_party(&p).is_ok(),
            "the .old folder is used when the folder is missing"
        );
    }

    #[test]
    fn newer_version_is_refused() {
        let d = tmp("ver");
        let p = d.join("v.txt.partysave");
        write_party(&p, &sample()).unwrap();
        let m = fs::read_to_string(p.join(MANIFEST))
            .unwrap()
            .replacen(" 1\n", " 2\n", 1);
        fs::write(p.join(MANIFEST), m).unwrap();
        assert_eq!(
            load_party(&p),
            Err(PartyError::Version {
                found: 2,
                supported: 1
            })
        );
        assert!(matches!(read_info(&p), Err(PartyError::Version { .. })));
        fs::write(p.join(MANIFEST), "garbage\n").unwrap();
        assert!(matches!(load_party(&p), Err(PartyError::BadManifest(_))));
    }

    #[test]
    fn a_missing_sidecar_costs_only_that_guest() {
        let d = tmp("miss");
        let p = d.join("m.txt.partysave");
        write_party(&p, &sample()).unwrap();
        fs::remove_file(p.join(sidecar_name(2))).unwrap();
        let l = load_party(&p).unwrap();
        assert_eq!(l.problems, vec![Problem::SidecarMissing(2)]);
        assert_eq!(l.save.guests.len(), 1);
        assert_eq!(l.save.guests[0].key, 1);
    }

    #[test]
    fn a_corrupted_sidecar_costs_only_that_guest() {
        let d = tmp("corrupt");
        let p = d.join("k.txt.partysave");
        write_party(&p, &sample()).unwrap();
        let f = p.join(sidecar_name(1));
        let mut b = fs::read(&f).unwrap();
        let mid = b.len() / 2;
        b[mid] ^= 0x55;
        fs::write(&f, &b).unwrap();
        let l = load_party(&p).unwrap();
        assert_eq!(l.problems, vec![Problem::SidecarCorrupt(1)]);
        assert_eq!(l.save.guests.len(), 1);
        // truncated too
        fs::write(&f, &b[..5]).unwrap();
        assert_eq!(
            load_party(&p).unwrap().problems,
            vec![Problem::SidecarCorrupt(1)]
        );
    }

    #[test]
    fn a_damaged_world_refuses_the_save() {
        let d = tmp("world");
        let p = d.join("w.txt.partysave");
        write_party(&p, &sample()).unwrap();
        fs::write(p.join(WORLD), "nv-rs save 1\nlevel 99\n").unwrap();
        assert!(matches!(load_party(&p), Err(PartyError::WorldBad(_))));
        fs::remove_file(p.join(WORLD)).unwrap();
        assert!(matches!(load_party(&p), Err(PartyError::WorldBad(_))));
    }

    #[test]
    fn the_manifest_is_what_the_launcher_reads() {
        // crates/nv-launcher/src/party.rs (`read_party_manifest`) parses these lines
        let d = tmp("manifest");
        let p = d.join("m.txt.partysave");
        write_party(&p, &sample()).unwrap();
        let t = fs::read_to_string(p.join(MANIFEST)).unwrap();
        let lines: Vec<&str> = t.lines().collect();
        assert_eq!(lines[0], "nv-rs party save 1");
        assert_eq!(lines[1], "campaign=00ab12cd34ef5678");
        assert_eq!(lines[2], "host=Host");
        assert_eq!(lines[3], "seq=3");
        assert_eq!(lines[4], "saved=1700000000");
        assert!(lines[5].starts_with("world="));
        // guest=<key> <slot> <level> <from_guest> <len> <hash> <name>, sorted by key
        let g: Vec<&str> = lines[6]
            .splitn(2, '=')
            .nth(1)
            .unwrap()
            .splitn(7, ' ')
            .collect();
        assert_eq!(
            (g[0], g[1], g[2], g[3], g[6]),
            ("0000000000000001", "1", "3", "0", "Ann")
        );
    }

    #[test]
    fn the_rules_line_round_trips_and_an_older_save_gets_the_default() {
        let d = tmp("rules");
        let p = d.join("Save 1.txt.partysave");
        write_party(&p, &sample()).unwrap();
        let t = fs::read_to_string(p.join(MANIFEST)).unwrap();
        assert!(t.lines().any(|l| l == "rules=normal on"), "{t}");
        assert_eq!(load_party(&p).unwrap().save.rules, sample().rules);
        // a save written before the rules line
        let old: String = t
            .lines()
            .filter(|l| !l.starts_with("rules="))
            .map(|l| format!("{l}\n"))
            .collect();
        fs::write(p.join(MANIFEST), old).unwrap();
        assert_eq!(load_party(&p).unwrap().save.rules, PartyRules::default());
    }

    #[test]
    fn campaign_ids() {
        assert_eq!(parse_campaign("00ab12cd34ef5678"), Some(0x00ab12cd34ef5678));
        assert_eq!(parse_campaign("0000000000000000"), None);
        assert_eq!(parse_campaign("abc"), None);
        assert_eq!(campaign_hex(0xab), "00000000000000ab");
        let mut s = sample();
        s.campaign = 0;
        assert_eq!(
            write_party(&tmp("zero").join("z"), &s),
            Err(PartyError::NoCampaign)
        );
    }

    #[test]
    fn synthetic_forms_are_scrubbed_from_world_and_sidecars() {
        let mut s = sample();
        s.world_text = "nv-rs save 1\nitem FE000001 0000000F 5\nitem 00000014 0000000F 5\nposition FE000002 1 2 3 0\n\
disabled FC000001 1\nvar 00000014 FE000001 f 1\nitem FF000001 0000000F 1\n"
            .into();
        let mut sh = shadow(5, 1);
        sh.items.insert(FormId(0xFE00_0003), 1);
        sh.teammates.insert(FormId(0xFE00_0001));
        sh.hotkeys[0] = Some(FormId(0xFC00_0000));
        s.guests[0] = GuestSidecar::from_shadow(2, "Bo", 2, true, &sh);
        let d = tmp("scrub");
        let p = d.join("s.txt.partysave");
        write_party(&p, &s).unwrap();
        let l = load_party(&p).unwrap();
        assert!(
            !l.save.world_text.contains("item FE0000")
                && !l.save.world_text.contains("position FE"),
            "{}",
            l.save.world_text
        );
        assert!(!l.save.world_text.contains("FC0000"));
        assert!(l.save.world_text.contains("item 00000014 0000000F 5"));
        assert!(
            l.save.world_text.contains("item FF000001"),
            "dynamic references are real state"
        );
        assert!(
            l.save.world_text.contains("var 00000014 FE000001 f 1"),
            "a variable name is not a form"
        );
        let back = l.guest(2).unwrap().shadow().unwrap();
        assert!(!back.items.keys().any(|f| is_synthetic(*f)));
        assert!(back.teammates.is_empty());
        assert_eq!(back.hotkeys[0], None);
        assert_eq!(back.items.get(&FormId(0xF)), Some(&1));
    }
}
