//! Sandbox packages (type 12): someone left to keep themselves busy in an
//! area, as the game runs them. Read from `FalloutNV.exe` with Ghidra
//! (the addresses are given with each rule; "inferred" marks what was
//! deduced rather than seen in the code).
//!
//! Every so often the area is scanned for things to do ([`Sandbox::scan`]):
//! chairs and other furniture, beds, food, people to talk to, idle
//! markers, and wandering. One is chosen at random, weighted so the same
//! kind of thing is unlikely twice running ([`Sandbox::choose`]), for a
//! time that depends on the activity and the person's energy
//! ([`duration`]), counted in game minutes ([`elapsed_minutes`]). Four
//! times of day (sleeping and three meals) take over when they come round
//! ([`Window`]). Carrying the activities out (walking, sitting, playing a
//! marker's idles) is the caller's.

use esm::{FormId, LoadOrder};

use crate::scripting::{game_setting, GameState};

/// The activities, as the game numbers them (`SANDBOX_PROCEDURE_*` in its
/// strings; the numbers index the counters).
pub mod activities {
    /// Sit in a piece of furniture that isn't a bed.
    pub const SIT: u8 = 0;
    pub const SLEEP: u8 = 1;
    pub const EAT: u8 = 2;
    pub const WANDER: u8 = 3;
    pub const IDLE_MARKER: u8 = 4;
    pub const DIALOGUE: u8 = 5;

    pub fn name(activity: u8) -> &'static str {
        ["sit", "sleep", "eat", "wander", "use idle marker", "talk"]
            .get(usize::from(activity))
            .copied()
            .unwrap_or("?")
    }
}

/// A sandbox package's own flags (`PKDT` u16 at 8, the package's +0x24;
/// read where the scan and the choice test them: `009f45e0` 0x01,
/// `009f4600` 0x02, `00901c50` 0x04, `009f54b0` 0x08, `009f4620` 0x10,
/// `009f5690` 0x20).
pub mod flags {
    pub const NO_EATING: u16 = 0x01;
    pub const NO_SLEEPING: u16 = 0x02;
    pub const NO_CONVERSATION: u16 = 0x04;
    pub const NO_IDLE_MARKERS: u16 = 0x08;
    pub const NO_FURNITURE: u16 = 0x10;
    pub const NO_WANDERING: u16 = 0x20;
}

/// The game's sandbox settings: `FalloutNV.esm`'s values where it sets
/// them (`fSandboxDurationMultFurniture` 3, `…IdleMarker` 1.5), else the
/// exe's defaults.
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// `fSandBoxSearchRadius` (6000): the area when the package gives no
    /// radius.
    pub search_radius: f32,
    /// `fSandBoxExtraDialogueRange` (384): people a little farther count.
    pub extra_dialogue_range: f32,
    /// `iSandBoxPreventRepeatedActionTime` (30 s): the last target isn't
    /// chosen again for this long.
    pub prevent_repeat: f32,
    /// `iMinSandboxRescanSeconds`, `iMaxSandboxRescanSeconds` (3, 6).
    pub rescan: (f32, f32),
    /// `fSandboxDurationBase` (10 game minutes).
    pub duration_base: f32,
    /// Per activity: `fSandboxDurationMultFurniture` (sit, sleep),
    /// `…Eating` 0.75, `…Wandering` 0.5 (wander, talk), `…IdleMarker`.
    pub duration_mult: [f32; 6],
    /// `fSandboxEnergyMult` (0.05).
    pub energy_mult: f32,
    /// Per activity: `fSandboxEnergyMultFurniture` -0.1, `…Eating`
    /// -0.05, `…Wandering` 0.2, `…IdleMarker` 0.05.
    pub energy_mult_by: [f32; 6],
    /// `fSandboxDurationRangeMult` (0.25): durations vary by this either
    /// way.
    pub range_mult: f32,
    /// The four times of day: sleep (`fSandboxSleepStartMin`/`Max` 19 and
    /// 1, `…DurationMin`/`Max` 6 and 10 hours), breakfast (6 to 9), lunch
    /// (11 to 14), dinner (17 to 20), meals `fSandboxMealDurationMin`/`Max`
    /// 0.5 to 2 hours (`00f5cb40`): (start min, start max, length min,
    /// length max).
    pub windows: [(f32, f32, f32, f32); 4],
    /// `fAIMaxWanderTime` (100), `fAIEngergyLevelMult` (1),
    /// `fAIEnergyLevelBase` (0): the pause between wanders.
    pub max_wander_time: f32,
    pub energy_level_mult: f32,
    pub energy_level_base: f32,
}

impl Settings {
    pub fn read(order: &LoadOrder) -> Settings {
        let f = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
        let furniture = f("fSandboxDurationMultFurniture", 2.0);
        let eating = f("fSandboxDurationMultEating", 0.75);
        let wandering = f("fSandboxDurationMultWandering", 0.5);
        let marker = f("fSandboxDurationMultIdleMarker", 1.0);
        let e_furniture = f("fSandboxEnergyMultFurniture", -0.1);
        let e_eating = f("fSandboxEnergyMultEating", -0.05);
        let e_wandering = f("fSandboxEnergyMultWandering", 0.2);
        let e_marker = f("fSandboxEnergyMultIdleMarker", 0.05);
        let meal = (
            f("fSandboxMealDurationMin", 0.5),
            f("fSandboxMealDurationMax", 2.0),
        );
        Settings {
            search_radius: f("fSandBoxSearchRadius", 6000.0),
            extra_dialogue_range: f("fSandBoxExtraDialogueRange", 384.0),
            prevent_repeat: f("iSandBoxPreventRepeatedActionTime", 30.0),
            rescan: (
                f("iMinSandboxRescanSeconds", 3.0),
                f("iMaxSandboxRescanSeconds", 6.0),
            ),
            duration_base: f("fSandboxDurationBase", 10.0),
            // The exe's table (`0119b194`…): sit and sleep share the
            // furniture setting, talking takes wandering's.
            duration_mult: [furniture, furniture, eating, wandering, marker, wandering],
            energy_mult: f("fSandboxEnergyMult", 0.05),
            energy_mult_by: [
                e_furniture,
                e_furniture,
                e_eating,
                e_wandering,
                e_marker,
                e_wandering,
            ],
            range_mult: f("fSandboxDurationRangeMult", 0.25),
            windows: [
                (
                    f("fSandboxSleepStartMin", 19.0),
                    f("fSandboxSleepStartMax", 1.0),
                    f("fSandboxSleepDurationMin", 6.0),
                    f("fSandboxSleepDurationMax", 10.0),
                ),
                (
                    f("fSandboxBreakfastMin", 6.0),
                    f("fSandboxBreakfastMax", 9.0),
                    meal.0,
                    meal.1,
                ),
                (
                    f("fSandboxLunchMin", 11.0),
                    f("fSandboxLunchMax", 14.0),
                    meal.0,
                    meal.1,
                ),
                (
                    f("fSandboxDinnerMin", 17.0),
                    f("fSandboxDinnerMax", 20.0),
                    meal.0,
                    meal.1,
                ),
            ],
            max_wander_time: f("fAIMaxWanderTime", 100.0),
            energy_level_mult: f("fAIEngergyLevelMult", 1.0),
            energy_level_base: f("fAIEnergyLevelBase", 0.0),
        }
    }
}

/// The package flags as read (`PKDT` u16 at 8).
pub fn package_flags(order: &LoadOrder, package: FormId) -> u16 {
    order
        .get(package)
        .and_then(|rr| rr.record_shared().ok())
        .and_then(|r| {
            r.get(esm::FourCC::new(b"PKDT"))
                .filter(|s| s.data.len() >= 10)
                .map(|s| u16::from_le_bytes([s.data[8], s.data[9]]))
        })
        .unwrap_or(0)
}

/// Someone's energy (`AIDT` byte 2 of their base: the duration and wander
/// pause read a byte of the AI data, `0087f990`; that it's the energy
/// level is inferred from the settings' names).
pub fn energy(order: &LoadOrder, actor: FormId) -> u8 {
    crate::scripting::base_of(order, actor)
        .and_then(|b| order.get(b))
        .and_then(|rr| rr.record().ok())
        .and_then(|r| {
            r.get(esm::FourCC::new(b"AIDT"))
                .and_then(|s| s.data.get(2).copied())
        })
        .unwrap_or(50)
}

/// How long one activity lasts, in game minutes (`006435c0`): the base
/// × the activity's multiplier × (1 + energy × `fSandboxEnergyMult` × the
/// activity's energy multiplier), anywhere within ± `range_mult` of that
/// (`unit` a random number in 0..1). Energy 50: sitting about 22.5
/// minutes, eating 6.6, wandering 7.5, an idle marker 16.9.
pub fn duration(settings: &Settings, activity: u8, energy: u8, unit: f32) -> f32 {
    let a = usize::from(activity.min(5));
    let d = settings.duration_base
        * settings.duration_mult[a]
        * (1.0 + f32::from(energy) * settings.energy_mult * settings.energy_mult_by[a]);
    let (lo, hi) = (
        (1.0 - settings.range_mult) * d,
        (1.0 + settings.range_mult) * d,
    );
    lo + (hi - lo) * unit
}

/// The pause between two wanders, seconds (`006434f0`): (`fAIMaxWanderTime`
/// − (energy × `fAIEngergyLevelMult` + `fAIEnergyLevelBase`)) ÷ 2. That
/// it's in seconds is inferred.
pub fn wander_pause(settings: &Settings, energy: u8) -> f32 {
    ((settings.max_wander_time
        - (f32::from(energy) * settings.energy_level_mult + settings.energy_level_base))
        / 2.0)
        .max(0.0)
}

/// The wander procedure's timer (`008ed420`, process +0x2d0), counted in
/// frame seconds while the person stands (the mover idle): within 5 of the
/// area's middle it's let go at once (that the 5 is measured to the middle
/// is inferred); at 10 or less a new spot is chosen; after choosing one at
/// least 50 away it's set to the wander pause ([`wander_pause`]), else 0.
/// So the pause at each spot is the wander pause less 10 s (15 s at energy
/// 50; none from energy 80).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WanderTimer(pub f32);

impl WanderTimer {
    /// One frame standing: whether to choose a new spot now.
    pub fn idle(&mut self, dt: f32, distance_to_middle: f32) -> bool {
        if distance_to_middle <= 5.0 {
            self.0 = 0.0;
        }
        if self.0 <= 10.0 {
            return true;
        }
        self.0 -= dt;
        false
    }

    /// A spot was chosen `distance` away.
    pub fn chose(&mut self, distance: f32, pause: f32) {
        self.0 = if distance >= 50.0 { pause } else { 0.0 };
    }
}

/// How often someone eating seated asks for an eating idle (`008e3140`:
/// `00476b70(1.0, 6.0)`, process +0x334): every random 1 to 6 s.
pub const EATING_IDLE_EVERY: (f32, f32) = (1.0, 6.0);

/// Whether a form is food (`INGR`, `ALCH`: form types 0x1D, 0x2F, the
/// eat procedure's).
pub fn is_food(order: &LoadOrder, item: FormId) -> bool {
    order
        .get(item)
        .is_some_and(|r| matches!(r.entry.header.kind.as_bytes(), b"INGR" | b"ALCH"))
}

/// The first food someone carries (`00891d30`), if any.
pub fn carried_food(order: &LoadOrder, state: &GameState, who: FormId) -> Option<FormId> {
    state
        .inventory(order, who)
        .into_iter()
        .find(|(item, n)| *n > 0 && is_food(order, *item))
        .map(|(item, _)| item)
}

/// Someone takes a piece of food lying in the world (the activate
/// procedure picking it up, `0088c650`): it goes into their inventory and
/// the placed one is gone. The item taken.
pub fn take_food(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    reference: FormId,
) -> Option<FormId> {
    let item = crate::scripting::base_of(order, reference).filter(|b| is_food(order, *b))?;
    if !crate::enabled_now(order, reference, &state.disabled) {
        return None;
    }
    state.stock(order, who);
    *state.items.entry((who, item)).or_insert(0) += 1;
    state.added(order, who, item, 1);
    state.disabled.insert(reference, true);
    state
        .events
        .push(crate::scripting::Event::Enable(reference, false));
    Some(item)
}

/// Someone eats one of an item they carry (`008c1de0`: the item's effects
/// cast on them, one taken away): [`crate::items::use_item`], without its
/// sound (that's played where they are, not for the player). Whether they
/// had one.
pub fn eat(order: &LoadOrder, state: &mut GameState, who: FormId, item: FormId) -> bool {
    let before = state.events.len();
    let done = crate::items::use_item(order, state, who, item).is_some();
    let mut i = before;
    while i < state.events.len() {
        if matches!(state.events[i], crate::scripting::Event::Sound(_)) {
            state.events.remove(i);
        } else {
            i += 1;
        }
    }
    done
}

/// How many game minutes the game counts from one clock time to another
/// (hours, 0–24), the way it compares an activity's duration (`009f41f0`):
/// the whole hours between them, plus the minutes from the start's whole
/// hour to now, less the start's minutes, the minutes kept in a signed
/// byte. So each hour boundary crossed adds a minute, and past about two
/// hours the count wraps round; read as the code does it.
pub fn elapsed_minutes(start: f32, now: f32) -> f32 {
    let start_hour = start as i32 as i8;
    let start_minutes = ((start - f32::from(start_hour)) * 60.0) as i32 as i8;
    let now_hour = now as i32 as i8;
    let now_minutes = ((now - f32::from(start_hour)) * 60.0) as i32 as i8;
    let hours = (i32::from(now_hour) - i32::from(start_hour)).abs() as f32;
    hours + f32::from(i16::from(now_minutes) - i16::from(start_minutes))
}

/// A time of day that takes over: 0 sleep, 1 breakfast, 2 lunch, 3
/// dinner; on which day (days passed), from which hour, for how long.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Window {
    pub which: u8,
    pub day: i32,
    pub start: f32,
    pub length: f32,
}

impl Window {
    /// The activity it's for (`011a5f3c`: sleep, then eating).
    pub fn activity(&self) -> u8 {
        if self.which == 0 {
            activities::SLEEP
        } else {
            activities::EAT
        }
    }

    /// Where it ends: its day and hour.
    fn end(&self) -> (i32, f32) {
        let mut day = self.day;
        let mut hour = self.start + self.length;
        while hour >= 24.0 {
            hour -= 24.0;
            day += 1;
        }
        (day, hour)
    }

    /// Whether it covers a moment (`009f3d50`).
    pub fn covers(&self, day: i32, hour: f32) -> bool {
        let (end_day, end) = self.end();
        if day < self.day || day > end_day {
            return false;
        }
        !(day == self.day && hour < self.start) && !(day == end_day && hour >= end)
    }

    /// Whether it's over (`009f3c90`).
    fn over(&self, day: i32, hour: f32) -> bool {
        let (end_day, end) = self.end();
        day > end_day || (day == end_day && hour >= end)
    }
}

/// Rolls window `which`: a start between its earliest and latest (past
/// midnight when the latest is earlier: sleep starts 19:00 to 01:00) and a
/// length between its least and most, each rolled afresh when the next
/// window is worked out (`00643540`, `006429e0`). `unit` gives random
/// numbers in 0..1.
fn roll_window(settings: &Settings, which: usize, unit: &mut dyn FnMut() -> f32) -> (f32, f32) {
    let (lo, hi, len_lo, len_hi) = settings.windows[which];
    let hi = if hi < lo { hi + 24.0 } else { hi };
    let mut start = lo + (hi - lo) * unit();
    if start - 24.0 >= 0.0 {
        start -= 24.0;
    }
    (start, len_lo + (len_hi - len_lo) * unit())
}

/// The next window from a moment (`009f3e80`): each of the four is rolled;
/// one already begun (and not over today) is taken at once, else the one
/// starting soonest (tomorrow's if today's is over). `just_ended`: the
/// window that ended today, which isn't taken again today however its new
/// roll falls.
pub fn next_window(
    settings: &Settings,
    day: i32,
    hour: f32,
    just_ended: Option<u8>,
    unit: &mut dyn FnMut() -> f32,
) -> Window {
    let mut best: Option<(f32, Window)> = None;
    for which in 0..4 {
        let (start, length) = roll_window(settings, which, unit);
        let done_today = start + length <= hour;
        if just_ended == Some(which as u8) && !done_today {
            continue;
        }
        let wait = if done_today { start + 24.0 } else { start } - hour;
        let w = Window {
            which: which as u8,
            day: if done_today { day + 1 } else { day },
            start,
            length,
        };
        if wait <= 0.0 {
            return w;
        }
        if best.map_or(true, |(b, _)| wait < b) {
            best = Some((wait, w));
        }
    }
    best.map(|(_, w)| w).unwrap_or(Window {
        which: 0,
        day,
        start: 0.0,
        length: 0.0,
    })
}

/// What kind of thing a reference is, for a sandbox (`009f5070`, by the
/// base's record type).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// An ingredient or drink (`INGR`, `ALCH`).
    Food,
    /// Furniture (`FURN`) with its `MNAM`.
    Furniture(u32),
    /// Someone (`NPC_`, `CREA`).
    Actor,
    IdleMarker,
}

/// A reference's [`Kind`], from its base's record type.
pub fn kind_of(order: &LoadOrder, reference: FormId) -> Option<Kind> {
    let base = crate::scripting::base_of(order, reference)?;
    let rr = order.get(base)?;
    Some(match rr.entry.header.kind.as_bytes() {
        b"INGR" | b"ALCH" => Kind::Food,
        b"FURN" => Kind::Furniture(crate::furniture::marker_flags(order, base)),
        b"NPC_" | b"CREA" => Kind::Actor,
        b"IDLM" => Kind::IdleMarker,
        _ => return None,
    })
}

/// Whether a reference is kept out of sandboxes: the scan skips references
/// carrying extra data 0x80 (`0057b500`), which is taken to be what the
/// reference's "ignored by sandbox" mark (`XIBS`) loads as (inferred from
/// the name; Goodsprings' store goods have it, the chairs don't).
pub fn ignored(order: &LoadOrder, reference: FormId) -> bool {
    order
        .get(reference)
        .and_then(|rr| rr.record().ok())
        .is_some_and(|r| r.get(esm::FourCC::new(b"XIBS")).is_some())
}

/// Whether a child may use a reference in a sandbox: activators,
/// furniture and idle markers need their base's record flag 0x20000000
/// ("child can use"; read as the flag `008859e0` tests, the name
/// inferred).
pub fn child_can_use(order: &LoadOrder, reference: FormId) -> bool {
    let Some(base) = crate::scripting::base_of(order, reference).and_then(|b| order.get(b)) else {
        return false;
    };
    let kind = base.entry.header.kind;
    let needs = [b"ACTI", b"FURN", b"IDLM"].contains(&kind.as_bytes());
    !needs || base.entry.header.flags & 0x2000_0000 != 0
}

/// Whether someone may use an object as its owner allows (`005785e0`,
/// with the owner from `00567790`): no owner (the object's `XOWN`, else
/// its cell's), the owner is them (their base), or a faction they're in.
/// Faction ranks (`XRNK`) aren't checked (the code compares them only on
/// a path not taken here).
pub fn may_use(order: &LoadOrder, state: &GameState, actor: FormId, reference: FormId) -> bool {
    let xown = esm::FourCC::new(b"XOWN");
    let owner_of = |id: FormId| {
        let rr = order.get(id)?;
        let record = rr.record_shared().ok()?;
        let s = record.get(xown).filter(|s| s.data.len() >= 4)?;
        Some(rr.plugin.to_global(FormId(crate::cell::le_u32(&s.data, 0))))
    };
    let owner = owner_of(reference).or_else(|| {
        let cell = crate::scripting::whereabouts(order, reference)?.cell;
        owner_of(cell)
    });
    let Some(owner) = owner.filter(|o| o.0 != 0) else {
        return true;
    };
    if crate::scripting::base_of(order, actor) == Some(owner) {
        return true;
    }
    let is_faction = order
        .get(owner)
        .is_some_and(|r| r.entry.header.kind.as_bytes() == b"FACT");
    is_faction && crate::factions::factions_of(order, state, actor).contains(&owner)
}

/// Something near the area's centre the scan looks at: a reference, what
/// it is, where it is, whether the person may use it, and for furniture
/// whether its model has markers (`00c54400`, `00509490`), for an idle
/// marker whether one of its idles passes for the person (`00479fb0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Nearby {
    pub reference: FormId,
    pub kind: Kind,
    pub position: [f32; 3],
    pub allowed: bool,
    pub usable: bool,
}

/// Something to do: an activity and its target (none for wandering, or
/// eating carried food), weighted 1 (`009f3800(…, 1)`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Candidate {
    pub activity: u8,
    pub target: Option<FormId>,
    pub weight: i32,
}

/// The current activity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Choice {
    pub activity: u8,
    pub target: Option<FormId>,
    /// For furniture: the marker reserved (the first free one, `005682c0`).
    pub marker: Option<u8>,
    /// Game minutes ([`duration`]).
    pub duration: f32,
    /// The game hour it began.
    pub started: f32,
}

/// Where someone sandboxing is in carrying out the activity (the
/// procedure's states, `00929fc0`): just begun, getting up (from a chair
/// or bed, waiting for idles to end), on the way, doing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Phase {
    #[default]
    Init,
    GettingUp,
    Moving,
    Doing,
}

/// One person's sandbox: the area, what's around, the counters and the
/// current activity (the procedure's data).
#[derive(Debug, Clone, PartialEq)]
pub struct Sandbox {
    pub package: FormId,
    pub center: [f32; 3],
    /// The package's radius, else `fSandBoxSearchRadius`.
    pub radius: f32,
    pub flags: u16,
    pub energy: u8,
    /// One per activity: start at 128, all go up by one (to 255 at most)
    /// with every choice and the chosen one drops to 1 (`009f4700`).
    pub counters: [u8; 6],
    pub candidates: Vec<Candidate>,
    /// What the last scan found (`+0x35`, `+0x36`, `+0x37`).
    pub food_found: bool,
    pub chair_found: bool,
    pub bed_found: bool,
    /// When (seconds of play) the area may be scanned again.
    pub next_scan: f32,
    pub choice: Option<Choice>,
    /// The last activity's target, not chosen again until a moment
    /// (seconds of play).
    pub previous: Option<(FormId, f32)>,
    pub window: Option<Window>,
    pub phase: Phase,
    /// The person's conversation timer is still running (process vfunc
    /// +0xb8, `008d9320`: +0x2c8, [`crate::social::Social::conversation`]):
    /// talking then weighs nothing (`009f4700`). The caller's to keep.
    pub conversation_wait: bool,
}

impl Sandbox {
    /// A sandbox for a package (`flags` from [`package_flags`]) around
    /// `center`, its radius the package's (`PLDT`) if any, else the search
    /// radius.
    pub fn new(
        package: FormId,
        center: [f32; 3],
        package_radius: i32,
        flags: u16,
        energy: u8,
        settings: &Settings,
    ) -> Sandbox {
        Sandbox {
            package,
            center,
            radius: if package_radius >= 1 {
                package_radius as f32
            } else {
                settings.search_radius
            },
            flags,
            energy,
            counters: [128; 6],
            candidates: Vec::new(),
            food_found: false,
            chair_found: false,
            bed_found: false,
            next_scan: 0.0,
            choice: None,
            previous: None,
            window: None,
            phase: Phase::Init,
            conversation_wait: false,
        }
    }

    fn allows(&self, flag: u16) -> bool {
        self.flags & flag == 0
    }

    /// Lists what there is to do (`009f54d0` with `009f5070` per
    /// reference): references within the radius of the centre — food
    /// (eating allowed, the owner allows), furniture with markers (a bed
    /// when sleeping is allowed, else sitting furniture or any other
    /// non-bed furniture when furniture is allowed; the owner allows),
    /// people other than the player and themselves (conversation allowed),
    /// usable idle markers (idle markers allowed, the owner allows); then
    /// food they carry (`carries_food`), and wandering (allowed).
    pub fn scan(&mut self, actor: FormId, nearby: &[Nearby], carries_food: bool) {
        self.candidates.clear();
        self.food_found = false;
        self.chair_found = false;
        self.bed_found = false;
        let add = |activity: u8, target: Option<FormId>| Candidate {
            activity,
            target,
            weight: 1,
        };
        let mut found = Vec::new();
        for n in nearby {
            if distance(n.position, self.center) > self.radius {
                continue;
            }
            match n.kind {
                Kind::Food => {
                    if self.allows(flags::NO_EATING) && n.allowed {
                        found.push(add(activities::EAT, Some(n.reference)));
                        self.food_found = true;
                    }
                }
                Kind::Furniture(mnam) => {
                    if !n.usable {
                        continue;
                    }
                    let bed = mnam & crate::furniture::BED != 0;
                    let sit = mnam & crate::furniture::SIT_FURNITURE != 0;
                    if self.allows(flags::NO_SLEEPING) && bed && n.allowed {
                        found.push(add(activities::SLEEP, Some(n.reference)));
                        self.bed_found = true;
                    } else if self.allows(flags::NO_FURNITURE) && (sit || !bed) && n.allowed {
                        found.push(add(activities::SIT, Some(n.reference)));
                        self.chair_found = true;
                    }
                }
                Kind::Actor => {
                    let other = n.reference != actor && n.reference != crate::dialogue::PLAYER_REF;
                    if self.allows(flags::NO_CONVERSATION) && other {
                        found.push(add(activities::DIALOGUE, Some(n.reference)));
                    }
                }
                Kind::IdleMarker => {
                    if self.allows(flags::NO_IDLE_MARKERS) && n.allowed && n.usable {
                        found.push(add(activities::IDLE_MARKER, Some(n.reference)));
                    }
                }
            }
        }
        if self.allows(flags::NO_EATING) && carries_food {
            found.push(add(activities::EAT, None));
            self.food_found = true;
        }
        if self.allows(flags::NO_WANDERING) {
            found.push(add(activities::WANDER, None));
        }
        self.candidates = found;
    }

    /// Moves the time-of-day window on when the current one is over (or
    /// none was worked out yet), `009f3e80`.
    pub fn update_window(
        &mut self,
        settings: &Settings,
        day: i32,
        hour: f32,
        unit: &mut dyn FnMut() -> f32,
    ) {
        if self.window.map_or(true, |w| w.over(day, hour)) {
            let just_ended = self.window.filter(|w| w.end().0 == day).map(|w| w.which);
            self.window = Some(next_window(settings, day, hour, just_ended, unit));
        }
    }

    /// The window now, if it's on and its activity is possible here (a
    /// bed found for sleeping, food for meals; `009f3d50`, `009f3e20`).
    pub fn window_now(&self, day: i32, hour: f32) -> Option<Window> {
        let w = self.window?;
        let possible = if w.which == 0 {
            self.bed_found
        } else {
            self.food_found
        };
        (w.covers(day, hour) && possible).then_some(w)
    }

    /// Whether the area is due to be scanned again (pass [`Self::choose`]
    /// the list then).
    pub fn scan_due(&self, now: f32) -> bool {
        now >= self.next_scan
    }

    /// Chooses what to do next (`009f4700`): scans first when it's time
    /// (`nearby` lists the area then, and what they carry: see
    /// [`Self::scan_due`]; the next scan 3–6 s later), then
    /// weights each candidate by its activity's counter — in a time-of-day
    /// window only that window's activity counts (10 each); otherwise
    /// sleeping counts 0, eating 20 more when food was found and 15 less
    /// when no chair was — skipping the last target for its 30 s, and
    /// targets `current` says are gone, elsewhere or unusable (it gives
    /// where the target is now), or out of the radius (people within the
    /// radius + 384). One is picked by `roll() % total`; its duration is
    /// rolled, every counter goes up by one and the chosen one's drops
    /// to 1. `None` when nothing is left.
    #[allow(clippy::too_many_arguments)]
    pub fn choose(
        &mut self,
        actor: FormId,
        settings: &Settings,
        now: f32,
        (day, hour): (i32, f32),
        roll: &mut dyn FnMut() -> u64,
        nearby: Option<(&[Nearby], bool)>,
        current: &dyn Fn(&Candidate) -> Option<[f32; 3]>,
    ) -> Option<Choice> {
        self.update_window(settings, day, hour, &mut || unit(roll));
        // The last activity's target rests for a while.
        if let Some(t) = self.choice.and_then(|c| c.target) {
            if self.previous.map(|p| p.0) != Some(t) {
                self.previous = Some((t, now + settings.prevent_repeat));
            }
        }
        if self.previous.is_some_and(|(_, until)| now > until) {
            self.previous = None;
        }
        self.choice = None;
        if now >= self.next_scan {
            if let Some((list, carries_food)) = nearby {
                self.scan(actor, list, carries_food);
            }
            let (lo, hi) = settings.rescan;
            self.next_scan = now + lo + (hi - lo) * unit(roll);
        }
        let mut weights = [0i32; 6];
        match self.window_now(day, hour) {
            Some(w) => weights[usize::from(w.activity())] = 10,
            None => {
                for (w, c) in weights.iter_mut().zip(self.counters) {
                    *w = i32::from(c);
                }
                weights[usize::from(activities::SLEEP)] = 0;
                if self.food_found {
                    weights[usize::from(activities::EAT)] += 20;
                }
                if !self.chair_found {
                    weights[usize::from(activities::EAT)] -= 15;
                }
                if self.conversation_wait {
                    weights[usize::from(activities::DIALOGUE)] = 0;
                }
            }
        }
        let mut total = 0i32;
        let mut picks: Vec<i32> = Vec::with_capacity(self.candidates.len());
        for c in &self.candidates {
            let mut w = (c.weight * weights[usize::from(c.activity.min(5))]).max(0);
            if let Some(t) = c.target {
                let rests = self.previous.is_some_and(|(p, _)| p == t);
                let reach = if c.activity == activities::DIALOGUE {
                    self.radius + settings.extra_dialogue_range
                } else {
                    self.radius
                };
                let there = current(c).filter(|p| distance(*p, self.center) <= reach);
                if rests || there.is_none() {
                    w = 0;
                }
            }
            picks.push(w);
            total += w;
        }
        if total < 1 {
            return None;
        }
        let mut pick = (roll() % total as u64) as i32;
        let mut index = picks.len() - 1;
        for (i, w) in picks.iter().enumerate() {
            pick -= w;
            if pick < 0 {
                index = i;
                break;
            }
        }
        let c = self.candidates[index];
        let choice = Choice {
            activity: c.activity,
            target: c.target,
            marker: None,
            duration: duration(settings, c.activity, self.energy, unit(roll)),
            started: hour,
        };
        for counter in &mut self.counters {
            *counter = counter.saturating_add(1);
        }
        self.counters[usize::from(c.activity.min(5))] = 1;
        self.choice = Some(choice);
        self.phase = Phase::GettingUp;
        Some(choice)
    }

    /// The candidate list after an activity failed (`009f4680`): its
    /// target weighs nothing from now on (till the next scan). One without
    /// a target (eating carried food that's gone) weighs nothing too, which
    /// the game doesn't do (it leaves those): so it isn't retried every
    /// frame. (A wander never fails: with no spot they stand.)
    pub fn failed(&mut self) {
        let Some(choice) = self.choice else {
            return;
        };
        for c in &mut self.candidates {
            let same = match choice.target {
                Some(t) => c.target == Some(t),
                None => c.target.is_none() && c.activity == choice.activity,
            };
            if same {
                c.weight = 0;
            }
        }
    }

    /// Whether it's time for something else (`009f43c0`): nothing chosen
    /// or no duration; in a time-of-day window, when the activity isn't
    /// the window's; else once the activity's game minutes have passed
    /// ([`elapsed_minutes`]). A target gone or unusable is the caller's
    /// to report ([`Self::failed`]).
    pub fn time_for_something_else(&self, (day, hour): (i32, f32)) -> bool {
        let Some(c) = self.choice else {
            return true;
        };
        if c.duration <= 0.0 {
            return true;
        }
        let Some(w) = self.window else {
            return true;
        };
        if w.over(day, hour) {
            return true;
        }
        if self.window_now(day, hour).is_some() {
            return c.activity != w.activity();
        }
        c.duration < elapsed_minutes(c.started, hour)
    }

    /// Whether someone has strayed too far from the area: farther than its
    /// radius + 150 (the float at `010231d0`) from the centre, when they
    /// go back to it (`00929fc0`).
    pub fn strayed(&self, at: [f32; 3]) -> bool {
        distance(at, self.center) > self.radius + 150.0
    }
}

/// A random number in 0..1 from the dice.
fn unit(roll: &mut dyn FnMut() -> u64) -> f32 {
    (roll() % 1_000_000) as f32 / 1_000_000.0
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|k| (a[k] - b[k]).powi(2)).sum::<f32>().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> Settings {
        Settings {
            search_radius: 6000.0,
            extra_dialogue_range: 384.0,
            prevent_repeat: 30.0,
            rescan: (3.0, 6.0),
            duration_base: 10.0,
            duration_mult: [3.0, 3.0, 0.75, 0.5, 1.5, 0.5],
            energy_mult: 0.05,
            energy_mult_by: [-0.1, -0.1, -0.05, 0.2, 0.05, 0.2],
            range_mult: 0.25,
            windows: [
                (19.0, 1.0, 6.0, 10.0),
                (6.0, 9.0, 0.5, 2.0),
                (11.0, 14.0, 0.5, 2.0),
                (17.0, 20.0, 0.5, 2.0),
            ],
            max_wander_time: 100.0,
            energy_level_mult: 1.0,
            energy_level_base: 0.0,
        }
    }

    #[test]
    fn durations_follow_the_settings_and_energy() {
        let s = settings();
        // Energy 50, the middle of the range: 10 × 3 × (1 - 0.25) = 22.5.
        assert!((duration(&s, activities::SIT, 50, 0.5) - 22.5).abs() < 1e-4);
        assert!((duration(&s, activities::WANDER, 50, 0.5) - 7.5).abs() < 1e-4);
        assert!((duration(&s, activities::IDLE_MARKER, 50, 0.5) - 16.875).abs() < 1e-4);
        assert!((duration(&s, activities::EAT, 50, 0.5) - 6.5625).abs() < 1e-4);
        // ± 25 %.
        assert!((duration(&s, activities::SIT, 50, 0.0) - 22.5 * 0.75).abs() < 1e-4);
        assert!((duration(&s, activities::SIT, 50, 1.0) - 22.5 * 1.25).abs() < 1e-4);
        assert!((wander_pause(&s, 50) - 25.0).abs() < 1e-5);
    }

    #[test]
    fn elapsed_minutes_count_as_the_game_does() {
        assert!((elapsed_minutes(10.0, 10.25) - 15.0).abs() < 1e-4);
        // Across an hour: one extra.
        assert!((elapsed_minutes(10.5, 11.25) - 46.0).abs() < 1e-4);
        // Past about two hours the minutes wrap round in a byte.
        assert!(elapsed_minutes(10.0, 12.2) < 0.0);
    }

    #[test]
    fn windows_take_over_at_their_times() {
        let s = settings();
        let mut half = || 0.5;
        // At 15:00 the next is dinner (18:30 for 1.25 h with the middle
        // rolls); sleep would start at 22:00.
        let w = next_window(&s, 3, 15.0, None, &mut half);
        assert_eq!(w.which, 3);
        assert!((w.start - 18.5).abs() < 1e-4 && (w.length - 1.25).abs() < 1e-4);
        assert!(w.covers(3, 19.0) && !w.covers(3, 18.0) && !w.covers(3, 20.0));
        // Late at night: sleep, begun.
        let w = next_window(&s, 3, 23.0, None, &mut half);
        assert_eq!(w.which, 0);
        assert!(w.covers(3, 23.5) && w.covers(4, 5.0) && !w.covers(4, 9.0));
        assert_eq!(w.activity(), activities::SLEEP);
    }

    fn nearby(reference: u32, kind: Kind, x: f32) -> Nearby {
        Nearby {
            reference: FormId(reference),
            kind,
            position: [x, 0.0, 0.0],
            allowed: true,
            usable: true,
        }
    }

    #[test]
    fn the_scan_follows_the_package_flags() {
        let s = settings();
        let me = FormId(1);
        let all = [
            nearby(
                10,
                Kind::Furniture(crate::furniture::SIT_FURNITURE | 1),
                100.0,
            ),
            nearby(11, Kind::Furniture(crate::furniture::BED | 1), 100.0),
            nearby(12, Kind::Food, 100.0),
            nearby(13, Kind::Actor, 100.0),
            nearby(1, Kind::Actor, 0.0),
            nearby(14, Kind::IdleMarker, 100.0),
            // Out of reach.
            nearby(
                15,
                Kind::Furniture(crate::furniture::SIT_FURNITURE | 1),
                900.0,
            ),
        ];
        let mut b = Sandbox::new(FormId(2), [0.0; 3], 512, 0, 50, &s);
        b.scan(me, &all, false);
        let kinds: Vec<u8> = b.candidates.iter().map(|c| c.activity).collect();
        use activities::*;
        assert_eq!(kinds, [SIT, SLEEP, EAT, DIALOGUE, IDLE_MARKER, WANDER]);
        assert!(b.chair_found && b.bed_found && b.food_found);
        // Chet's store: no sleeping. The bed is then other furniture? No:
        // beds are never sat in.
        let mut chet = Sandbox::new(FormId(2), [0.0; 3], 512, flags::NO_SLEEPING, 50, &s);
        chet.scan(me, &all, true);
        let kinds: Vec<u8> = chet.candidates.iter().map(|c| c.activity).collect();
        assert_eq!(kinds, [SIT, EAT, DIALOGUE, IDLE_MARKER, EAT, WANDER]);
        let none = flags::NO_EATING
            | flags::NO_SLEEPING
            | flags::NO_CONVERSATION
            | flags::NO_IDLE_MARKERS
            | flags::NO_FURNITURE
            | flags::NO_WANDERING;
        let mut idle = Sandbox::new(FormId(2), [0.0; 3], 0, none, 50, &s);
        assert_eq!(idle.radius, 6000.0);
        idle.scan(me, &all, true);
        assert!(idle.candidates.is_empty());
    }

    #[test]
    fn choices_are_weighted_by_counters_that_reset_when_chosen() {
        let s = settings();
        let me = FormId(1);
        let all = vec![
            nearby(
                10,
                Kind::Furniture(crate::furniture::SIT_FURNITURE | 1),
                100.0,
            ),
            nearby(13, Kind::Actor, 100.0),
        ];
        let mut b = Sandbox::new(FormId(2), [0.0; 3], 512, 0, 50, &s);
        let mut dice = 0x9E37_79B9_7F4A_7C15u64;
        let mut roll = || {
            dice ^= dice << 13;
            dice ^= dice >> 7;
            dice ^= dice << 17;
            dice
        };
        let here = |c: &Candidate| {
            Some(if c.target.is_some() {
                [100.0, 0.0, 0.0]
            } else {
                [0.0; 3]
            })
        };
        let first = b
            .choose(
                me,
                &s,
                0.0,
                (1, 12.0),
                &mut roll,
                Some((all.as_slice(), false)),
                &here,
            )
            .unwrap();
        // Three candidates: sit, talk, wander; the choice's counter is 1,
        // the others 129.
        assert_eq!(b.candidates.len(), 3);
        let i = usize::from(first.activity);
        assert_eq!(b.counters[i], 1);
        assert!(b
            .counters
            .iter()
            .enumerate()
            .all(|(j, c)| j == i || *c == 129));
        assert_eq!(b.phase, Phase::GettingUp);
        assert!(first.duration > 0.0);
        // Over many choices the same activity is seldom chosen twice
        // running: its weight drops to 1 while the others' grow (a third
        // of the time if chosen evenly).
        let mut repeats = 0;
        let mut last = first.activity;
        for k in 0..200 {
            let c = b
                .choose(
                    me,
                    &s,
                    100.0 + k as f32 * 40.0,
                    (1, 12.0),
                    &mut roll,
                    Some((all.as_slice(), false)),
                    &here,
                )
                .unwrap();
            if c.activity == last {
                repeats += 1;
            }
            last = c.activity;
        }
        assert!(repeats < 40, "{repeats}");
        // Nothing usable: no choice.
        let gone = |c: &Candidate| c.target.is_none().then_some([0.0; 3]);
        let mut walls = Sandbox::new(FormId(2), [0.0; 3], 512, flags::NO_WANDERING, 50, &s);
        assert!(walls
            .choose(
                me,
                &s,
                0.0,
                (1, 12.0),
                &mut roll,
                Some((all.as_slice(), false)),
                &gone
            )
            .is_none());
    }

    #[test]
    fn talking_waits_for_the_conversation_timer() {
        let s = settings();
        let me = FormId(1);
        let all = vec![nearby(13, Kind::Actor, 100.0)];
        let here = |_: &Candidate| Some([100.0, 0.0, 0.0]);
        let mut dice = 99u64;
        let mut roll = || {
            dice ^= dice << 13;
            dice ^= dice >> 7;
            dice ^= dice << 17;
            dice
        };
        let talks = |wait: bool, roll: &mut dyn FnMut() -> u64| {
            let mut b = Sandbox::new(FormId(2), [0.0; 3], 512, 0, 50, &s);
            b.conversation_wait = wait;
            (0..50)
                .filter(|k| {
                    b.choose(
                        me,
                        &s,
                        *k as f32 * 100.0,
                        (1, 12.0),
                        roll,
                        Some((all.as_slice(), false)),
                        &here,
                    )
                    .is_some_and(|c| c.activity == activities::DIALOGUE)
                })
                .count()
        };
        assert!(talks(false, &mut roll) > 0);
        assert_eq!(talks(true, &mut roll), 0);
    }

    #[test]
    fn the_wander_pause_is_the_timer_less_ten_seconds() {
        let mut t = WanderTimer::default();
        // A new spot at once.
        assert!(t.idle(0.1, 100.0));
        t.chose(200.0, 25.0);
        let mut waited = 0.0f32;
        while !t.idle(0.1, 100.0) {
            waited += 0.1;
        }
        assert!((waited - 15.0).abs() < 0.2, "{waited}");
        // A spot under 50 away: no pause.
        t.chose(40.0, 25.0);
        assert!(t.idle(0.1, 100.0));
        // At the middle the timer is let go.
        t.chose(200.0, 25.0);
        assert!(t.idle(0.1, 3.0));
    }

    #[test]
    fn a_failed_activity_is_not_chosen_again_till_the_next_scan() {
        let s = settings();
        let me = FormId(1);
        let all = vec![nearby(
            10,
            Kind::Furniture(crate::furniture::SIT_FURNITURE | 1),
            100.0,
        )];
        let mut b = Sandbox::new(FormId(2), [0.0; 3], 512, 0, 50, &s);
        let mut dice = 5u64;
        let mut roll = || {
            dice ^= dice << 13;
            dice ^= dice >> 7;
            dice ^= dice << 17;
            dice
        };
        let here = |_: &Candidate| Some([100.0, 0.0, 0.0]);
        let mut chosen = Vec::new();
        for k in 0..2 {
            let c = b
                .choose(
                    me,
                    &s,
                    k as f32,
                    (1, 12.0),
                    &mut roll,
                    Some((all.as_slice(), false)),
                    &here,
                )
                .unwrap();
            chosen.push(c.activity);
            b.failed();
        }
        // Both the chair and the wander failed: nothing left till a scan.
        chosen.sort();
        assert_eq!(chosen, [activities::SIT, activities::WANDER]);
        assert!(b
            .choose(me, &s, 2.0, (1, 12.0), &mut roll, None, &here)
            .is_none());
    }

    #[test]
    fn the_last_target_rests_and_the_sleep_window_wants_beds() {
        let s = settings();
        let me = FormId(1);
        let all = vec![
            nearby(
                10,
                Kind::Furniture(crate::furniture::SIT_FURNITURE | 1),
                100.0,
            ),
            nearby(11, Kind::Furniture(crate::furniture::BED | 1), 100.0),
        ];
        let mut b = Sandbox::new(FormId(2), [0.0; 3], 512, flags::NO_WANDERING, 50, &s);
        let mut dice = 12345u64;
        let mut roll = || {
            dice ^= dice << 13;
            dice ^= dice >> 7;
            dice ^= dice << 17;
            dice
        };
        let here = |_: &Candidate| Some([100.0, 0.0, 0.0]);
        // By day only the chair counts (sleeping weighs 0 outside the
        // window).
        let c = b
            .choose(
                me,
                &s,
                0.0,
                (1, 12.0),
                &mut roll,
                Some((all.as_slice(), false)),
                &here,
            )
            .unwrap();
        assert_eq!((c.activity, c.target), (activities::SIT, Some(FormId(10))));
        // The chair rests 30 s: nothing else to choose.
        assert!(b
            .choose(
                me,
                &s,
                10.0,
                (1, 12.1),
                &mut roll,
                Some((all.as_slice(), false)),
                &here
            )
            .is_none());
        assert!(b
            .choose(
                me,
                &s,
                45.0,
                (1, 12.2),
                &mut roll,
                Some((all.as_slice(), false)),
                &here
            )
            .is_some());
        assert!(!b.time_for_something_else((1, 12.2)));
        // At midnight the sleep window takes over: the bed.
        let c = b
            .choose(
                me,
                &s,
                100.0,
                (1, 23.9),
                &mut roll,
                Some((all.as_slice(), false)),
                &here,
            )
            .unwrap();
        assert_eq!(c.activity, activities::SLEEP);
        assert!(!b.time_for_something_else((2, 2.0)));
        assert!(b.time_for_something_else((2, 9.0)));
        assert!(b.strayed([700.0, 0.0, 0.0]) && !b.strayed([600.0, 0.0, 0.0]));
    }
}
