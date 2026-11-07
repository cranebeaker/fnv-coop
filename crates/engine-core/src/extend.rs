//! Where the built-in script extenders join the engine (ADR-0016): a slot per engine for their
//! state ([`Engine::ext`]), the wrap chain that puts a built-in's function in front of the one
//! registered at an exe address ([`Engine::wrap`], [`Engine::call_next`]), and the seams, named
//! extension points inside translated functions ([`Engine::site`], [`Engine::on_site`]).
//!
//! xNVSE and JIP LN NVSE change the exe in two ways that the port cannot copy as byte writes:
//! they detour a function's entry to their own code, which then runs the original (xNVSE
//! `Hooks_Other.cpp`: the script context pushed around `ScriptRunner::Run` `005e0d20`), and they
//! patch an instruction inside a function (`Hooks_Gameplay.cpp`: the main-loop call at
//! `0086B386`). The first is a wrap, the second a seam.
//!
//! Both are fixed at build time: every address that may be wrapped and every seam is a row of a
//! static table here ([`WRAPS`], [`SITES`]) that lists its owners in the order they run (fix
//! rows of the settings layer first, then `nvse`, then `jip`). A built-in names its place with a
//! [`WrapSlot`] or [`SiteSlot`] made in a `const`, so a place or owner the table does not list is
//! a build error, not a run-time surprise. With nothing installed, a wrapped address calls its
//! translation as before, and a seam costs one test of an empty table.

use crate::abi::{AbiFn, Ret};
use crate::Engine;

/// The code address of built-in command `index`'s handler (`CommandInfo +0x18`): one word
/// apart, from here, in the extenders' range of the reserved block
/// ([`crate::mem::EXTENDER_CODE`]; ADR-0016 §3).
pub const EXECUTE_BASE: u32 = 0x7e80_0000;
/// The code address of built-in command `index`'s condition eval (`CommandInfo +0x20`).
pub const EVAL_BASE: u32 = 0x7ec0_0000;
/// Named code addresses from here: the built-ins' parse routines (`CommandInfo +0x1c`) and
/// xNVSE's padding handler, up to the end of the extenders' range.
pub const ROUTINE_BASE: u32 = 0x7eff_0000;

const _: () = assert!(
    crate::mem::EXTENDER_CODE.start == EXECUTE_BASE
        && EXECUTE_BASE < EVAL_BASE
        && EVAL_BASE < ROUTINE_BASE
        && ROUTINE_BASE < crate::mem::EXTENDER_CODE.end
);

/// Who may own a wrap or a seam handler, in the order they run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Owner {
    /// A fix row of the settings layer (ADR-0010), by its id. Rows run before the extenders.
    Row(&'static str),
    /// xNVSE built in (`crates/nvse`).
    Nvse,
    /// JIP LN NVSE built in (`crates/jip`); it runs after xNVSE, as JIP is xNVSE's plugin.
    Jip,
}

impl Owner {
    /// Its place in the order: rows, then `nvse`, then `jip`.
    const fn rank(self) -> u8 {
        match self {
            Owner::Row(_) => 0,
            Owner::Nvse => 1,
            Owner::Jip => 2,
        }
    }

    const fn same(self, other: Owner) -> bool {
        match (self, other) {
            (Owner::Row(a), Owner::Row(b)) => same_str(a, b),
            (Owner::Nvse, Owner::Nvse) | (Owner::Jip, Owner::Jip) => true,
            _ => false,
        }
    }
}

const fn same_str(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// One row of [`WRAPS`] or [`SITES`]: an exe address, its owners in the order they run, and the
/// build step of ADR-0016 that puts it to use.
#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub addr: u32,
    pub owners: &'static [Owner],
    pub step: &'static str,
}

const NVSE: &[Owner] = &[Owner::Nvse];

/// Every exe address a built-in may wrap (ADR-0016 §3, "Wrappers"), by address. Each is a
/// function entry registered by a translation (or a fix row's copy of it) and reached by
/// address; xNVSE detours its entry (`Hooks_Other.cpp`, `Hooks_Script.cpp` (xNVSE)).
pub const WRAPS: &[Entry] = &[
    // ScriptEventList destructor: the string and array variables an event list holds are freed.
    Entry {
        addr: 0x0041_af70,
        owners: NVSE,
        step: "I-5",
    },
    // TESObjectCELL::SetState: cell attach and detach events.
    Entry {
        addr: 0x0045_12a0,
        owners: NVSE,
        step: "I-9",
    },
    // ScriptEventList::SetActionFlag: the game events of the script event masks.
    Entry {
        addr: 0x005a_c750,
        owners: NVSE,
        step: "I-9",
    },
    // Script::ParseParameters: xNVSE's argument reader for extender opcodes.
    Entry {
        addr: 0x005a_ccb0,
        owners: NVSE,
        step: "I-4",
    },
    // ScriptRunner::Run: the script context pushed and popped.
    Entry {
        addr: 0x005e_0d20,
        owners: NVSE,
        step: "I-3",
    },
    // ScriptRunner InitActionList: the script context pushed and popped.
    Entry {
        addr: 0x005e_1160,
        owners: NVSE,
        step: "I-3",
    },
    // ScriptRunner::ParseLine: the data cursor of the line.
    Entry {
        addr: 0x005e_13a0,
        owners: NVSE,
        step: "I-3",
    },
    // ScriptRunner::EvaluateLine: the current line recorded.
    Entry {
        addr: 0x005e_1550,
        owners: NVSE,
        step: "I-3",
    },
];

/// Every seam: an instruction inside a translated function where a built-in (or a fix row whose
/// patch is inside a large function) takes part (ADR-0016 §12), by address. The translation
/// calls [`Engine::site`] there once the step named on the row has made it. xNVSE's own patch
/// sites (xNVSE) and JIP LN NVSE's (JIP LN NVSE) are added by the step that needs them.
pub const SITES: &[Entry] = &[
    // TESForm::~TESForm 00483630: the form extra data detour (FormExtraData).
    Entry {
        addr: 0x0048_3669,
        owners: NVSE,
        step: "I-12",
    },
    // CExpression::Eval: a command inside a vanilla expression called with the script's data.
    Entry {
        addr: 0x0059_49d4,
        owners: NVSE,
        step: "I-6",
    },
    // The result zeroed before the handler call in the operand reader 005ac7a0.
    Entry {
        addr: 0x005a_cb9d,
        owners: NVSE,
        step: "I-4",
    },
    // ScriptCompiler::GetFunction: the hashed name lookup over the extended table.
    Entry {
        addr: 0x005b_19ba,
        owners: NVSE,
        step: "I-14",
    },
    // An exit of the game in 005b6c90 (xNVSE's exit message).
    Entry {
        addr: 0x005b_6ca6,
        owners: NVSE,
        step: "I-8",
    },
    // HelpFunction: xNVSE's listing for the filter text.
    Entry {
        addr: 0x005b_cbc2,
        owners: NVSE,
        step: "I-14",
    },
    // ScriptRunner::EvaluateLine: the result zeroed before its first and second handler calls.
    Entry {
        addr: 0x005e_16c3,
        owners: NVSE,
        step: "I-4",
    },
    Entry {
        addr: 0x005e_2329,
        owners: NVSE,
        step: "I-4",
    },
    // The loading screen's present in 00713fb0: the present notification.
    Entry {
        addr: 0x0071_47c4,
        owners: NVSE,
        step: "I-8",
    },
    // Exits of the game in ChooseMainMenu 007d0a70 and 007d0bf0 (xNVSE's exit messages).
    Entry {
        addr: 0x007d_0b17,
        owners: NVSE,
        step: "I-8",
    },
    Entry {
        addr: 0x007d_0c3e,
        owners: NVSE,
        step: "I-8",
    },
    // New game in ConfirmNewGame 007d32b0: the co-save cleared.
    Entry {
        addr: 0x007d_33ce,
        owners: NVSE,
        step: "I-10",
    },
    // BGSSaveLoadGame::SaveGame: the co-save written.
    Entry {
        addr: 0x0084_7c45,
        owners: NVSE,
        step: "I-10",
    },
    // BGSSaveLoadGame::LoadGame: the co-save read before the .fos, after it, and post-load.
    Entry {
        addr: 0x0084_7fd9,
        owners: NVSE,
        step: "I-10",
    },
    Entry {
        addr: 0x0084_8c3c,
        owners: NVSE,
        step: "I-10",
    },
    Entry {
        addr: 0x0084_8c91,
        owners: NVSE,
        step: "I-10",
    },
    // A save deleted (in 00850360) and one renamed (in 00857370): the co-save goes with it.
    Entry {
        addr: 0x0085_0398,
        owners: NVSE,
        step: "I-10",
    },
    Entry {
        addr: 0x0085_762b,
        owners: NVSE,
        step: "I-10",
    },
    // The form extra data detour in 00867f90 (FormExtraData).
    Entry {
        addr: 0x0086_80a4,
        owners: NVSE,
        step: "I-12",
    },
    // main: the extender's tick, in place of the call to 0043c4b0, before Main::OnIdle.
    Entry {
        addr: 0x0086_b386,
        owners: NVSE,
        step: "I-5",
    },
    // The present in 0086ff70 (Main::Swap in ADR-0016): the present notification.
    Entry {
        addr: 0x0087_055e,
        owners: NVSE,
        step: "I-8",
    },
    // New game through CenterOnCell 0093db60: the co-save cleared.
    Entry {
        addr: 0x0093_db69,
        owners: NVSE,
        step: "I-10",
    },
];

/// Checks a table at build time: addresses in increasing order (each once), at least one owner,
/// owners in the order rows, `nvse`, `jip`, none twice, and at most 255 owners.
const fn check_table(table: &[Entry]) {
    let mut i = 0;
    while i < table.len() {
        let e = table[i];
        assert!(
            i == 0 || table[i - 1].addr < e.addr,
            "addresses out of order or listed twice"
        );
        assert!(
            !e.owners.is_empty() && e.owners.len() < 256,
            "an entry needs owners"
        );
        let mut j = 1;
        while j < e.owners.len() {
            let (a, b) = (e.owners[j - 1], e.owners[j]);
            assert!(
                a.rank() <= b.rank(),
                "owners out of order: rows, then nvse, then jip"
            );
            let mut k = 0;
            while k < j {
                assert!(!e.owners[k].same(b), "an owner listed twice");
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}

const _: () = check_table(WRAPS);
const _: () = check_table(SITES);

/// A rows-only owner list is not a wrap: a row is a base registration (ADR-0010), never a wrap.
const fn no_row_wraps(table: &[Entry]) {
    let mut i = 0;
    while i < table.len() {
        let mut j = 0;
        while j < table[i].owners.len() {
            assert!(
                table[i].owners[j].rank() > 0,
                "a fix row is a base, not a wrap"
            );
            j += 1;
        }
        i += 1;
    }
}

const _: () = no_row_wraps(WRAPS);

/// The index of `addr`'s row in `table` and of `owner` in it.
const fn find(table: &'static [Entry], owner: Owner, addr: u32) -> Option<(usize, u8)> {
    let mut i = 0;
    while i < table.len() {
        if table[i].addr == addr {
            let mut j = 0;
            while j < table[i].owners.len() {
                if table[i].owners[j].same(owner) {
                    return Some((i, j as u8));
                }
                j += 1;
            }
            return None;
        }
        i += 1;
    }
    None
}

/// A place in a table: the address, its owner, and which table.
#[derive(Debug, Clone, Copy)]
struct Slot {
    table: &'static [Entry],
    row: usize,
    index: u8,
}

impl Slot {
    const fn new(table: &'static [Entry], owner: Owner, addr: u32) -> Slot {
        match find(table, owner, addr) {
            Some((row, index)) => Slot { table, row, index },
            None => panic!("this owner and address are not listed in the table"),
        }
    }
    const fn addr(self) -> u32 {
        self.table[self.row].addr
    }
    const fn owner(self) -> Owner {
        self.table[self.row].owners[self.index as usize]
    }
}

/// An owner's wrap at an address of [`WRAPS`]. Made in a `const`, an address or owner the table
/// does not list is a build error:
///
/// ```compile_fail
/// use engine_core::extend::{Owner, WrapSlot};
/// engine_core::assemble!();
/// // 00401000 is not in WRAPS.
/// const NOT_LISTED: WrapSlot = WrapSlot::new(Owner::Nvse, 0x0040_1000);
/// fn main() {
///     let _ = NOT_LISTED;
/// }
/// ```
///
/// ```
/// use engine_core::extend::{Owner, WrapSlot};
/// engine_core::assemble!();
/// const RUN: WrapSlot = WrapSlot::new(Owner::Nvse, 0x005e_0d20);
/// fn main() {
///     assert_eq!(RUN.addr(), 0x005e_0d20);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct WrapSlot(Slot);

impl WrapSlot {
    pub const fn new(owner: Owner, addr: u32) -> WrapSlot {
        WrapSlot(Slot::new(WRAPS, owner, addr))
    }
    /// A slot of another table (the unit tests' own).
    #[cfg(test)]
    const fn of(table: &'static [Entry], owner: Owner, addr: u32) -> WrapSlot {
        WrapSlot(Slot::new(table, owner, addr))
    }
    pub const fn addr(self) -> u32 {
        self.0.addr()
    }
    pub const fn owner(self) -> Owner {
        self.0.owner()
    }
}

/// An owner's handler at a seam of [`SITES`]. Made in a `const`, a seam or owner the table does
/// not list is a build error:
///
/// ```compile_fail
/// use engine_core::extend::{Owner, SiteSlot};
/// engine_core::assemble!();
/// // JIP is not an owner of the tick.
/// const NOT_LISTED: SiteSlot = SiteSlot::new(Owner::Jip, 0x0086_b386);
/// fn main() {
///     let _ = NOT_LISTED;
/// }
/// ```
///
/// ```
/// use engine_core::extend::{Owner, SiteSlot};
/// engine_core::assemble!();
/// const TICK: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0086_b386);
/// fn main() {
///     assert_eq!(TICK.addr(), 0x0086_b386);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SiteSlot(Slot);

impl SiteSlot {
    pub const fn new(owner: Owner, addr: u32) -> SiteSlot {
        SiteSlot(Slot::new(SITES, owner, addr))
    }
    #[cfg(test)]
    const fn of(table: &'static [Entry], owner: Owner, addr: u32) -> SiteSlot {
        SiteSlot(Slot::new(table, owner, addr))
    }
    pub const fn addr(self) -> u32 {
        self.0.addr()
    }
    pub const fn owner(self) -> Owner {
        self.0.owner()
    }
}

/// A seam's handler: given the values the hook reads in the exe (registers, the function's
/// locals and parameters; each seam documents its words), `Some(answer)` takes the call (the
/// translation goes on as the seam's answer says: a skip, an early return) and ends it; `None`
/// passes it on to the next owner, and after the last the translation goes on as it is.
pub type SiteFn = fn(&mut Engine, &[u32]) -> Option<u32>;

/// What is registered at a wrapped address: the base (the translation, or a fix row's copy)
/// first, then each wrap in table order, with its index among the row's owners.
#[derive(Debug, Clone)]
pub(crate) struct Chain {
    pub(crate) table: &'static [Entry],
    pub(crate) links: Vec<(Option<u8>, AbiFn)>,
}

/// A link of a chain running through [`Engine::call_next`]: the address, the link's level in
/// its chain and the call-stack depth of the call it belongs to.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Active {
    addr: u32,
    level: usize,
    depth: usize,
}

/// The script command table's place as FalloutNV.exe's code reads it (ADR-0016 §2, option A).
///
/// xNVSE copies the exe's script command table into its own memory, appends its commands and
/// its plugins' and writes the new numbers over 19 immediates in 13 exe functions
/// (`CommandTable.cpp` `kPatch_ScriptCommands_Start`, `_End`, `_MaxIdx` (xNVSE)). The port's
/// translations read those immediates from here instead: the code is unchanged and only the
/// numbers move. With no extender installed every read gives the exe's immediate
/// ([`ScriptTables::EXE`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScriptTables {
    /// The first `CommandInfo` record (opcode `0x1000`); the exe's immediate `0x01190910`.
    /// Field addresses are this plus the field's offset (`+0x10` needs a reference, `+0x12`
    /// the parameter count, `+0x14` the parameters, `+0x20` the eval), as the exe's
    /// `0x01190920`, `0x01190922`, `0x01190924`, `0x01190930`.
    pub start: u32,
    /// One past the last record; the exe reads the opcode word of the record there
    /// (`end + 8`, the immediate `0x01196d18` at `005AEA59`).
    pub end: u32,
    /// One past the highest script opcode: `0x1280` in the exe (`0059390A`, `005AEA5D`,
    /// `005B115E`; that minus `0x1000` at `005B19A3` and `005BCBE3`).
    pub max_opcode: u32,
}

impl ScriptTables {
    /// FalloutNV.exe 1.4.0.525's own table: 0x280 records of 0x28 bytes from `0x01190910`.
    pub const EXE: ScriptTables = ScriptTables {
        start: 0x0119_0910,
        end: 0x0119_6d10,
        max_opcode: 0x1280,
    };
    /// The size of one `CommandInfo` record.
    pub const RECORD: u32 = 0x28;
    /// The first script opcode.
    pub const FIRST_OPCODE: u32 = 0x1000;

    /// The address of `opcode`'s record (`0x1000` and up; not checked).
    pub fn record(self, opcode: u32) -> u32 {
        self.start.wrapping_add(
            opcode
                .wrapping_sub(Self::FIRST_OPCODE)
                .wrapping_mul(Self::RECORD),
        )
    }
}

impl Default for ScriptTables {
    fn default() -> ScriptTables {
        ScriptTables::EXE
    }
}

/// The extenders' state on an engine (fields of [`Engine`]).
#[derive(Default)]
pub(crate) struct ExtendState {
    pub(crate) slots: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>,
    pub(crate) chains: crate::engine::AddrMap<u32, Chain>,
    /// The links running through [`Engine::call_next`], per game thread (`thread_host::current_id`):
    /// each thread has its own call stack (`thread_host` swaps them), so a link's depth means
    /// something only on its own thread.
    pub(crate) active: std::collections::HashMap<u32, Vec<Active>>,
    pub(crate) sites: crate::engine::AddrMap<u32, SiteHandlers>,
    pub(crate) tables: ScriptTables,
}

/// The handlers at one seam: its table, and a handler or none per owner, in the table's order.
pub(crate) type SiteHandlers = (&'static [Entry], Box<[Option<SiteFn>]>);

impl Engine {
    /// Where the script command table is ([`ScriptTables`]): the exe's own unless an extender
    /// moved it. The translations of the 13 functions whose immediates xNVSE patches read it.
    #[inline]
    pub fn script_tables(&self) -> ScriptTables {
        self.extend.tables
    }

    /// Moves the script command table (an extender's install, as xNVSE's `ApplyPatch`).
    pub fn set_script_tables(&mut self, tables: ScriptTables) {
        self.extend.tables = tables;
    }

    /// The extension slot of type `T` on this engine, made with `T::default()` at first use: a
    /// built-in's state per engine instance (tests run several engines at once; no
    /// thread-locals). Reached only while no other borrow of the engine is held, so no reference
    /// to it lives across an [`Engine::call`].
    pub fn ext<T: Default + 'static>(&mut self) -> &mut T {
        self.extend
            .slots
            .entry(std::any::TypeId::of::<T>())
            .or_insert_with(|| Box::new(T::default()))
            .downcast_mut::<T>()
            .unwrap_or_else(|| unreachable!("a slot holds the type it is keyed by"))
    }

    /// The extension slot of type `T` when it was made.
    pub fn ext_get<T: 'static>(&self) -> Option<&T> {
        self.extend
            .slots
            .get(&std::any::TypeId::of::<T>())
            .and_then(|b| b.downcast_ref::<T>())
    }

    /// Puts the plain function `f` in front of what is registered at the slot's address: `f`
    /// runs when the address is called, and [`Engine::call_next`] from inside it runs what was
    /// there before (the next link). Wraps install in the table's order (an owner listed later
    /// never wraps before one listed earlier, and none twice), after the fix rows (ADR-0016 §1).
    ///
    /// Refused (a stop): an address with nothing registered, or with a test double (a closure
    /// cannot be a link: it misses the call cache).
    pub fn wrap(&mut self, slot: WrapSlot, f: AbiFn) {
        let s = slot.0;
        let addr = s.addr();
        let base = match self.callable_at(addr) {
            Some(Ok(f)) => f,
            Some(Err(())) => {
                panic!("{addr:08x}: a test double is registered there; it cannot be wrapped")
            }
            None => panic!("{addr:08x}: nothing is registered there to wrap"),
        };
        let last = self
            .extend
            .chains
            .get(&addr)
            .and_then(|c| c.links.last().and_then(|l| l.0));
        if let Some(last) = last {
            assert!(
                s.index > last,
                "{addr:08x}: {:?} wraps out of the table's order (after {:?})",
                s.owner(),
                s.table[s.row].owners[usize::from(last)]
            );
        }
        let chain = self.extend.chains.entry(addr).or_insert_with(|| Chain {
            table: s.table,
            links: vec![(None, base)],
        });
        chain.links.push((Some(s.index), f));
        self.put(addr, f);
    }

    /// Calls the next link of the chain at `addr` from inside a link of it (the wrap that is
    /// running): the wrap installed before it, or the base. The next link runs as part of the
    /// same call of `addr`, which is one call in the exe: the call stack keeps `addr` once on top
    /// (so the callees of every link see the same caller), and the call log, the call counts,
    /// the time slice, the nv-trace record and the profile count the one call [`Engine::call`]
    /// made, not one per link. A recursive call of `addr` from inside any link starts again at
    /// the outermost link.
    ///
    /// Refused (a stop): `addr` is not the call running now, it is not wrapped, or the running
    /// link is the base.
    pub fn call_next(&mut self, addr: u32, args: &[u32]) -> Ret {
        assert_eq!(
            self.call_stack.last(),
            Some(&addr),
            "call_next({addr:08x}) outside a call of {addr:08x}"
        );
        let depth = self.call_stack.len();
        let Some(chain) = self.extend.chains.get(&addr) else {
            panic!("call_next({addr:08x}): nothing is wrapped there");
        };
        let top = chain.links.len() - 1;
        let thread = crate::thread_host::current_id(self);
        let level = self
            .extend
            .active
            .get(&thread)
            .into_iter()
            .flatten()
            .rev()
            .find(|a| a.addr == addr && a.depth == depth)
            .map_or(top, |a| a.level);
        assert!(
            level > 0,
            "call_next({addr:08x}) from its base: there is no next link"
        );
        let next = chain.links[level - 1].1;
        self.extend.active.entry(thread).or_default().push(Active {
            addr,
            level: level - 1,
            depth,
        });
        // A stop (a panic) unwinding through the link must not leave its entry behind: a later
        // call at the same depth would take it for its own.
        let ret = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| next(self, args)));
        if let Some(entries) = self.extend.active.get_mut(&thread) {
            entries.pop();
        }
        match ret {
            Ok(ret) => ret,
            Err(stop) => std::panic::resume_unwind(stop),
        }
    }

    /// The owners of the wraps at `addr`, innermost first (empty when it is not wrapped).
    pub fn wraps_at(&self, addr: u32) -> Vec<Owner> {
        self.extend.chains.get(&addr).map_or_else(Vec::new, |c| {
            let row = c.table.iter().find(|e| e.addr == addr);
            c.links
                .iter()
                .filter_map(|(i, _)| i.and_then(|i| row.map(|r| r.owners[usize::from(i)])))
                .collect()
        })
    }

    /// Every wrapped address with its wraps' owners, innermost first, by address: for the facade
    /// test that every registration besides an address's base is a wrap of [`WRAPS`].
    pub fn wrapped(&self) -> Vec<(u32, Vec<Owner>)> {
        let mut out: Vec<_> = self
            .extend
            .chains
            .keys()
            .map(|&a| (a, self.wraps_at(a)))
            .collect();
        out.sort_by_key(|(a, _)| *a);
        out
    }

    /// The dispatch check (ADR-0016 §16): every registration at an address besides its one base
    /// must be a wrap listed for that address in [`WRAPS`], in the table's order. Answers the
    /// chains that break it (empty when all is well): their address and their wraps' owners.
    pub fn unlisted_wraps(&self) -> Vec<(u32, Vec<Owner>)> {
        self.wrapped()
            .into_iter()
            .filter(|(addr, owners)| {
                let Some(row) = WRAPS.iter().find(|e| e.addr == *addr) else {
                    return true;
                };
                let places: Vec<Option<usize>> = owners
                    .iter()
                    .map(|o| row.owners.iter().position(|r| r.same(*o)))
                    .collect();
                places.iter().any(Option::is_none) || places.windows(2).any(|w| w[0] >= w[1])
            })
            .collect()
    }

    /// Registers `f` as the slot's handler at its seam. Refused (a stop): the slot has a handler.
    pub fn on_site(&mut self, slot: SiteSlot, f: SiteFn) {
        let s = slot.0;
        let addr = s.addr();
        let owners = s.table[s.row].owners.len();
        let (_, handlers) = self
            .extend
            .sites
            .entry(addr)
            .or_insert_with(|| (s.table, vec![None; owners].into_boxed_slice()));
        let at = &mut handlers[usize::from(s.index)];
        assert!(
            at.is_none(),
            "{addr:08x}: {:?} has a handler at this seam already",
            s.owner()
        );
        *at = Some(f);
    }

    /// The seam at `addr`, called by the translation at the instruction xNVSE or JIP hook: runs
    /// the handlers in the table's order until one takes the call, and answers its answer.
    /// `None` (nobody registered, or none took it): the translation goes on as the exe's code.
    /// With no handler anywhere this is one test.
    #[inline]
    pub fn site(&mut self, addr: u32, args: &[u32]) -> Option<u32> {
        if self.extend.sites.is_empty() {
            return None;
        }
        self.site_handlers(addr, args)
    }

    #[cold]
    #[inline(never)]
    fn site_handlers(&mut self, addr: u32, args: &[u32]) -> Option<u32> {
        let count = self.extend.sites.get(&addr)?.1.len();
        for i in 0..count {
            // Read again each time: a handler may register another (never its own slot again).
            let handler = self.extend.sites.get(&addr).and_then(|(_, h)| h[i]);
            if let Some(f) = handler {
                if let Some(answer) = f(self, args) {
                    return Some(answer);
                }
            }
        }
        None
    }

    /// The owners with a handler at the seam `addr`, in the order they run.
    pub fn site_owners(&self, addr: u32) -> Vec<Owner> {
        self.extend
            .sites
            .get(&addr)
            .map_or_else(Vec::new, |(table, h)| {
                let row = table.iter().find(|e| e.addr == addr);
                h.iter()
                    .enumerate()
                    .filter(|(_, f)| f.is_some())
                    .filter_map(|(i, _)| row.map(|r| r.owners[i]))
                    .collect()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::Ret;

    /// A table of the tests' own, with an address both extenders wrap.
    const BOTH: &[Owner] = &[Owner::Nvse, Owner::Jip];
    const TEST_WRAPS: &[Entry] = &[Entry {
        addr: 0x0000_1100,
        owners: BOTH,
        step: "test",
    }];
    const ROWS_FIRST: &[Owner] = &[
        Owner::Row("a.row"),
        Owner::Row("b.row"),
        Owner::Nvse,
        Owner::Jip,
    ];
    const TEST_SITES: &[Entry] = &[Entry {
        addr: 0x0000_1200,
        owners: ROWS_FIRST,
        step: "test",
    }];
    const _: () = check_table(TEST_WRAPS);
    const _: () = check_table(TEST_SITES);

    const NVSE_AT: WrapSlot = WrapSlot::of(TEST_WRAPS, Owner::Nvse, 0x0000_1100);
    const JIP_AT: WrapSlot = WrapSlot::of(TEST_WRAPS, Owner::Jip, 0x0000_1100);

    /// A word of memory the test functions append their marks to, one byte each.
    const LOG: u32 = 0x0100_0000;

    fn mark(e: &mut Engine, b: u8) {
        let n = e.mem.u32(LOG);
        e.mem.set_u8(LOG + 4 + n, b);
        e.mem.set_u32(LOG, n + 1);
    }

    fn marks(e: &Engine) -> Vec<u8> {
        let n = e.mem.u32(LOG);
        e.mem.bytes(LOG + 4, n)
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(LOG, 0x1000);
        e
    }

    fn base(e: &mut Engine, a: &[u32]) -> Ret {
        mark(e, b'b');
        // The base's callees see the wrapped address as their caller.
        e.call(0x0000_1300, &[]);
        Ret {
            eax: a[0] + 1,
            ..Ret::default()
        }
    }

    fn callee(e: &mut Engine, _: &[u32]) -> Ret {
        let caller = e.call_stack[e.call_stack.len() - 2];
        e.mem.set_u32(LOG + 0x800, caller);
        e.mem.set_u32(LOG + 0x804, e.call_stack.len() as u32);
        Ret::default()
    }

    fn nvse_wrap(e: &mut Engine, a: &[u32]) -> Ret {
        mark(e, b'n');
        let r = e.call_next(0x0000_1100, &[a[0] * 10]);
        mark(e, b'N');
        r
    }

    fn jip_wrap(e: &mut Engine, a: &[u32]) -> Ret {
        mark(e, b'j');
        let r = e.call_next(0x0000_1100, &[a[0] + 100]);
        mark(e, b'J');
        Ret {
            eax: r.eax * 2,
            ..r
        }
    }

    /// The links run outermost first, each through `call_next` to the one before, the base last;
    /// a cached base is not reached directly once wrapped (the cache is emptied).
    #[test]
    fn a_wrap_runs_in_front_of_its_next_link_in_table_order() {
        let mut e = engine();
        e.register(0x0000_1100, base);
        e.register(0x0000_1300, callee);
        // Prime the call cache with the base.
        assert_eq!(e.call(0x0000_1100, &[1]).eax, 2);
        e.mem.set_u32(LOG, 0);
        e.wrap(NVSE_AT, nvse_wrap);
        e.wrap(JIP_AT, jip_wrap);
        // jip: 3 + 100 = 103; nvse: 1030; base: 1031; jip doubles it.
        assert_eq!(e.call(0x0000_1100, &[3]).eax, 2062);
        assert_eq!(marks(&e), b"jnbNJ");
        assert_eq!(e.wraps_at(0x0000_1100), [Owner::Nvse, Owner::Jip]);
        assert_eq!(e.wrapped(), [(0x0000_1100, vec![Owner::Nvse, Owner::Jip])]);
    }

    /// The links are one call of the address: the call log, the call counts and the call stack
    /// see it once, and the base's callees see the address as their caller, one level down.
    #[test]
    fn a_wrap_calls_its_next_link_with_calls_bookkeeping() {
        let mut e = engine();
        e.register(0x0000_1100, base);
        e.register(0x0000_1300, callee);
        e.call(0x0000_1100, &[0]);
        let plain_depth = e.mem.u32(LOG + 0x804);
        e.wrap(NVSE_AT, nvse_wrap);
        e.call_log = Some(Vec::new());
        e.call_counts = Some(Default::default());
        e.call(0x0000_1100, &[0]);
        let log = e.call_log.take().unwrap_or_default();
        assert_eq!(log, [(0x0000_1100, vec![0]), (0x0000_1300, vec![])]);
        let counts = e.call_counts.take().unwrap_or_default();
        assert_eq!(counts.get(&0x0000_1100), Some(&1));
        assert_eq!(counts.get(&(0x0000_1100u64 << 32 | 0x0000_1300)), Some(&1));
        assert_eq!(e.mem.u32(LOG + 0x800), 0x0000_1100);
        assert_eq!(e.mem.u32(LOG + 0x804), plain_depth);
        assert!(e.call_stack.is_empty());
    }

    /// A call of the wrapped address from inside a link starts at the outermost link again;
    /// the outer call's links go on where they were.
    #[test]
    fn a_recursive_call_starts_at_the_outermost_link() {
        fn recursing_base(e: &mut Engine, a: &[u32]) -> Ret {
            mark(e, b'b');
            if a[0] > 0 {
                e.call(0x0000_1100, &[a[0] - 1]);
            }
            Ret::default()
        }
        fn wrap(e: &mut Engine, a: &[u32]) -> Ret {
            mark(e, b'n');
            e.call_next(0x0000_1100, a)
        }
        let mut e = engine();
        e.register(0x0000_1100, recursing_base);
        e.wrap(NVSE_AT, wrap);
        e.call(0x0000_1100, &[2]);
        assert_eq!(marks(&e), b"nbnbnb");
        assert!(e.extend.active.values().all(Vec::is_empty));
    }

    /// A stop unwinding through a link leaves no link marked running: the next call at the same
    /// depth starts at the outermost link.
    #[test]
    fn a_stop_through_a_link_leaves_the_chain_as_it_was() {
        fn stopping_base(e: &mut Engine, a: &[u32]) -> Ret {
            mark(e, b'b');
            assert!(a[0] != 0, "stop");
            Ret::default()
        }
        fn wrap(e: &mut Engine, a: &[u32]) -> Ret {
            mark(e, b'n');
            e.call_next(0x0000_1100, a)
        }
        let mut e = engine();
        e.register(0x0000_1100, stopping_base);
        e.wrap(NVSE_AT, wrap);
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x0000_1100, &[0]);
        }));
        assert!(stopped.is_err());
        e.call_stack.clear();
        assert!(e.extend.active.values().all(Vec::is_empty));
        e.call(0x0000_1100, &[1]);
        assert_eq!(marks(&e), b"nbnb");
    }

    /// A link running on another game thread at the same address and depth is not this thread's:
    /// each thread has its own call stack, so a call here starts at the outermost link.
    #[test]
    fn running_links_are_kept_per_game_thread() {
        fn wrap(e: &mut Engine, a: &[u32]) -> Ret {
            mark(e, b'n');
            e.call_next(0x0000_1100, a)
        }
        let mut e = engine();
        e.register(0x0000_1100, base);
        e.register(0x0000_1300, callee);
        e.wrap(NVSE_AT, wrap);
        let other = crate::thread_host::current_id(&e).wrapping_add(1);
        e.extend.active.entry(other).or_default().push(Active {
            addr: 0x0000_1100,
            level: 0,
            depth: 1,
        });
        e.call(0x0000_1100, &[1]);
        assert_eq!(marks(&e), b"nb");
        assert_eq!(e.extend.active[&other].len(), 1);
    }

    #[test]
    #[should_panic(expected = "nothing is registered there to wrap")]
    fn a_wrap_needs_something_registered() {
        engine().wrap(NVSE_AT, nvse_wrap);
    }

    #[test]
    #[should_panic(expected = "a test double is registered there")]
    fn a_double_cannot_be_wrapped() {
        let mut e = engine();
        e.register_double(0x0000_1100, |_, _| Ret::default());
        e.wrap(NVSE_AT, nvse_wrap);
    }

    #[test]
    #[should_panic(expected = "wraps out of the table's order")]
    fn wraps_install_in_table_order() {
        let mut e = engine();
        e.register(0x0000_1100, base);
        e.wrap(JIP_AT, jip_wrap);
        e.wrap(NVSE_AT, nvse_wrap);
    }

    #[test]
    #[should_panic(expected = "outside a call of 00001100")]
    fn call_next_only_inside_a_call_of_its_address() {
        let mut e = engine();
        e.register(0x0000_1100, base);
        e.wrap(NVSE_AT, nvse_wrap);
        e.call_next(0x0000_1100, &[0]);
    }

    /// A registration over a wrapped address replaces the whole chain, as `register` replaces
    /// what was there.
    #[test]
    fn register_over_a_wrapped_address_replaces_the_chain() {
        fn other(_: &mut Engine, _: &[u32]) -> Ret {
            Ret {
                eax: 7,
                ..Ret::default()
            }
        }
        let mut e = engine();
        e.register(0x0000_1100, base);
        e.register(0x0000_1300, callee);
        e.wrap(NVSE_AT, nvse_wrap);
        e.register(0x0000_1100, other);
        assert!(e.wraps_at(0x0000_1100).is_empty());
        assert_eq!(e.call(0x0000_1100, &[0]).eax, 7);
    }

    /// The dispatch check passes wraps listed in `WRAPS` and names a chain that is not (here one
    /// made through the tests' own table, the only way to make one).
    #[test]
    fn the_dispatch_check_allows_only_listed_wraps() {
        fn run_wrap(e: &mut Engine, a: &[u32]) -> Ret {
            e.call_next(0x005e_0d20, a)
        }
        const RUN: WrapSlot = WrapSlot::new(Owner::Nvse, 0x005e_0d20);
        let mut e = engine();
        e.register(0x005e_0d20, base);
        e.register(0x0000_1300, callee);
        e.wrap(RUN, run_wrap);
        assert_eq!(e.call(0x005e_0d20, &[1]).eax, 2);
        assert!(e.unlisted_wraps().is_empty());
        e.register(0x0000_1100, base);
        e.wrap(NVSE_AT, nvse_wrap);
        assert_eq!(e.unlisted_wraps(), [(0x0000_1100, vec![Owner::Nvse])]);
    }

    /// Every wrap address of the real table is listed for `nvse`, and the slots name them.
    #[test]
    fn the_wrap_table_lists_the_interpreter_entries() {
        const RUN: WrapSlot = WrapSlot::new(Owner::Nvse, 0x005e_0d20);
        assert_eq!((RUN.addr(), RUN.owner()), (0x005e_0d20, Owner::Nvse));
        for a in [
            0x005e_0d20,
            0x005e_1160,
            0x005e_13a0,
            0x005e_1550,
            0x005a_ccb0,
        ] {
            assert!(find(WRAPS, Owner::Nvse, a).is_some(), "{a:08x}");
        }
        assert!(find(WRAPS, Owner::Jip, 0x005e_0d20).is_none());
    }

    const AT: u32 = 0x0000_1200;

    fn row_a(e: &mut Engine, _: &[u32]) -> Option<u32> {
        mark(e, b'a');
        None
    }
    fn row_b(e: &mut Engine, a: &[u32]) -> Option<u32> {
        mark(e, b'b');
        (a[0] == 1).then_some(11)
    }
    fn nvse_site(e: &mut Engine, a: &[u32]) -> Option<u32> {
        mark(e, b'n');
        (a[0] == 2).then_some(22)
    }
    fn jip_site(e: &mut Engine, _: &[u32]) -> Option<u32> {
        mark(e, b'j');
        Some(33)
    }

    /// A seam with no handler answers `None` (the translation goes on as the exe's code).
    #[test]
    fn a_seam_with_no_handler_changes_nothing() {
        let mut e = engine();
        assert_eq!(e.site(0x0086_b386, &[1, 2]), None);
        assert_eq!(e.site(AT, &[1]), None);
        assert!(e.site_owners(0x0086_b386).is_empty());
    }

    /// Handlers run rows first, then nvse, then jip, whatever order they were registered in; the
    /// first that takes the call ends it.
    #[test]
    fn seam_handlers_run_in_table_order_until_one_takes_the_call() {
        let mut e = engine();
        e.on_site(SiteSlot::of(TEST_SITES, Owner::Jip, AT), jip_site);
        e.on_site(SiteSlot::of(TEST_SITES, Owner::Nvse, AT), nvse_site);
        e.on_site(SiteSlot::of(TEST_SITES, Owner::Row("b.row"), AT), row_b);
        e.on_site(SiteSlot::of(TEST_SITES, Owner::Row("a.row"), AT), row_a);
        assert_eq!(
            e.site_owners(AT),
            [
                Owner::Row("a.row"),
                Owner::Row("b.row"),
                Owner::Nvse,
                Owner::Jip
            ]
        );
        assert_eq!(e.site(AT, &[1]), Some(11));
        assert_eq!(marks(&e), b"ab");
        e.mem.set_u32(LOG, 0);
        assert_eq!(e.site(AT, &[2]), Some(22));
        assert_eq!(marks(&e), b"abn");
        e.mem.set_u32(LOG, 0);
        assert_eq!(e.site(AT, &[0]), Some(33));
        assert_eq!(marks(&e), b"abnj");
        // Another seam is untouched.
        assert_eq!(e.site(0x0086_b386, &[0]), None);
    }

    #[test]
    #[should_panic(expected = "has a handler at this seam already")]
    fn a_seam_slot_takes_one_handler() {
        let mut e = engine();
        const TICK: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0086_b386);
        e.on_site(TICK, nvse_site);
        e.on_site(TICK, nvse_site);
    }

    /// The real seam table names the sites of ADR-0016 §12 for `nvse`.
    #[test]
    fn the_seam_table_lists_the_extender_core_sites() {
        for a in [
            0x0086_b386,
            0x0059_49d4,
            0x005a_cb9d,
            0x005e_16c3,
            0x005e_2329,
            0x0084_7c45,
        ] {
            assert!(find(SITES, Owner::Nvse, a).is_some(), "{a:08x}");
        }
        assert_eq!(SITES.len(), 22);
    }

    /// The tables' order rules, checked at build time, also reject a bad table here.
    #[test]
    fn the_table_checks_reject_out_of_order_owners() {
        const JIP_FIRST: &[Owner] = &[Owner::Jip, Owner::Nvse];
        static BAD: [Entry; 1] = [Entry {
            addr: 1,
            owners: JIP_FIRST,
            step: "test",
        }];
        assert!(std::panic::catch_unwind(|| check_table(&BAD)).is_err());
        const TWICE: &[Owner] = &[Owner::Row("x"), Owner::Row("x")];
        static DUP: [Entry; 1] = [Entry {
            addr: 1,
            owners: TWICE,
            step: "test",
        }];
        assert!(std::panic::catch_unwind(|| check_table(&DUP)).is_err());
        static UNSORTED: [Entry; 2] = [
            Entry {
                addr: 2,
                owners: NVSE,
                step: "test",
            },
            Entry {
                addr: 1,
                owners: NVSE,
                step: "test",
            },
        ];
        assert!(std::panic::catch_unwind(|| check_table(&UNSORTED)).is_err());
    }

    /// With nothing installed, every read of the descriptor gives the exe's immediates: the table
    /// start `0x01190910` (`005B1172` and the field addresses `0x01190920`, `0x01190922`,
    /// `0x01190924`, `0x01190930`), the end marker `0x01196d18` (`005AEA59`), the highest opcode
    /// `0x1280` (`0059390A`, `005AEA5D`, `005B115E`).
    #[test]
    fn the_script_tables_are_the_exes_until_moved() {
        let mut e = engine();
        let t = e.script_tables();
        assert_eq!(t, ScriptTables::EXE);
        assert_eq!(t.start, 0x0119_0910);
        assert_eq!(
            [0x10, 0x12, 0x14, 0x20].map(|f| t.start + f),
            [0x0119_0920, 0x0119_0922, 0x0119_0924, 0x0119_0930]
        );
        assert_eq!(t.end + 8, 0x0119_6d18);
        assert_eq!(t.max_opcode, 0x1280);
        assert_eq!(t.record(0x127f), 0x0119_0910 + 0x27f * 0x28);
        let moved = ScriptTables {
            start: 0x7f00_0000,
            end: 0x7f00_0000 + 0x28,
            max_opcode: 0x1001,
        };
        e.set_script_tables(moved);
        assert_eq!(e.script_tables(), moved);
        assert_eq!(Engine::new().script_tables(), ScriptTables::EXE);
    }

    /// Two engines keep their own extension state; slots are keyed by type.
    #[test]
    fn the_extension_slot_is_per_engine_and_per_type() {
        #[derive(Default)]
        struct Counter(u32);
        #[derive(Default)]
        struct Other(u32);
        let mut a = engine();
        let mut b = engine();
        assert!(a.ext_get::<Counter>().is_none());
        a.ext::<Counter>().0 += 2;
        a.ext::<Counter>().0 += 1;
        b.ext::<Counter>().0 += 5;
        a.ext::<Other>().0 = 9;
        assert_eq!(a.ext::<Counter>().0, 3);
        assert_eq!(b.ext_get::<Counter>().map(|c| c.0), Some(5));
        assert_eq!(a.ext_get::<Other>().map(|o| o.0), Some(9));
        assert!(b.ext_get::<Other>().is_none());
    }
}
