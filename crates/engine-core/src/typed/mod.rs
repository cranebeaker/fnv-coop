//! The seam of the typed 64-bit migration (ADR-0012, ADR-0014; docs/ENGINE_CRATE.md, "Typed
//! code"): the switch, and what typed code plugs into. The first typed functions are in [`leaf`].
//!
//! A *family* is a group of typed functions that goes on and off together. Each typed function
//! stays registered at its exe address in the uniform form (`entry!`), so anything that calls it
//! by address or through a vtable reaches it unchanged, and the translation stays linked as the
//! fallback.
//!
//! - **The switch** is one table bound before the first frame: `NV_TYPED`, read once per process
//!   ([`parse_switch`] has the grammar). A family that is off stays the translation.
//! - **[`apply`]** runs at the end of [`Engine::new`], after every registration: it registers each
//!   family that is on over the translation, with [`Engine::register`].
//! - **[`typed_call!`](crate::typed_call)** is a call between typed functions: a Rust call when
//!   the family's direct flag is set ([`Engine::typed_direct`]), else a call by address. The two
//!   agree but for one thing: an `f32` result by address goes through the uniform form's `f64`,
//!   which quiets a signalling NaN (as the exe's `FLD` does), and a direct one keeps its bits.
//! - **The direct flag** is per engine (two engines in one test process can differ, and doubles
//!   and diagnostics belong to one engine). It is clear while [`Engine`] has any per-call
//!   diagnostic on, so a trace, a hook or a profile sees every call, and for good on a family once
//!   anything is registered over one of its addresses (a test double, a built-in fix): that keeps
//!   the double in the path whichever crate's code calls it, also from a crate below the one under
//!   test, which is not built with `cfg(test)`.
//!
//! [`Engine::dispatch`]'s fast path is not touched: a call by address costs what it cost before,
//! whatever the switch says. The flag follows the diagnostics in [`Engine::call_full`], the path
//! every call takes while a diagnostic is on.
//!
//! [`Engine::dispatch`]: crate::Engine
//! [`Engine::call_full`]: crate::Engine

// Type-aware where the repository's text rule against `as` from a float can only read words. It
// also stops `u32 as u8` and `f64 as f32`: those go through a named helper with a local `allow`,
// where the rounding or the cut is what the code means.
#![deny(clippy::cast_possible_truncation)]

use crate::abi::AbiFn;
use crate::{AddrMap, Engine};
use std::ffi::OsString;
use std::sync::{Mutex, OnceLock};

#[cfg(any(test, feature = "test-support"))]
pub mod diff;
pub mod leaf;

/// A family's bit in the per-engine flag words. Every family has its own, for good: it is written
/// in code at each [`typed_call!`](crate::typed_call) of the family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FamilyId(u8);

impl FamilyId {
    /// The family numbered `n` (below 32).
    pub const fn new(n: u8) -> FamilyId {
        assert!(n < 32, "the flag words have 32 bits");
        FamilyId(n)
    }

    #[inline(always)]
    pub(crate) const fn bit(self) -> u32 {
        1 << self.0
    }
}

/// The control word of the main thread once Direct3D has made its device: 24-bit precision.
pub const MAIN_THREAD: u16 = 0x007f;
/// The control word of the other threads, and of the main thread before that: 53-bit precision.
pub const OTHER_THREADS: u16 = crate::x87::DEFAULT_CONTROL_WORD;

/// Which threads a family runs on, as the float rule needs it (ADR-0014, point 7): main-thread-only
/// code computes in plain `f32`, which matches the exe only at the main thread's 24-bit precision;
/// other threads' code, or code on both, computes in `f64` locals rounded at each `float` store. The
/// differential test runs a family under this row's control words, so what a family claims is
/// stated once, here. Which threads really run it comes from the per-thread call counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Threads {
    /// Both control words: anything that isn't known to be one or the other.
    Both,
    /// The main thread only.
    Main,
    /// The other threads only.
    Other,
}

impl Threads {
    /// The control words this runs under.
    pub const fn control_words(self) -> &'static [u16] {
        match self {
            Threads::Both => &[OTHER_THREADS, MAIN_THREAD],
            Threads::Main => &[MAIN_THREAD],
            Threads::Other => &[OTHER_THREADS],
        }
    }
}

/// One row of the family table.
pub struct Family {
    /// What `NV_TYPED` calls it.
    pub name: &'static str,
    pub id: FamilyId,
    /// Past its gate (the checks of its step passed): on unless the switch says otherwise.
    pub past_gate: bool,
    /// The threads it runs on ([`Threads`]).
    pub threads: Threads,
    /// The typed functions by exe address, in the form of a unit's `funcs()`.
    pub funcs: fn() -> Vec<(u32, AbiFn)>,
}

/// The families of this crate.
pub static FAMILIES: &[&Family] = &[&leaf::LEAVES];

/// Families of the crates above this one ([`provide`]).
static PROVIDED: Mutex<Vec<&'static Family>> = Mutex::new(Vec::new());

/// Adds a family that lives in a crate above this one, which the table can't name. Its crate calls
/// this from its `units::funcs`, which [`Engine::new`] runs before [`apply`], as the platform layer
/// installs itself (`units::platform::install_layer`). Calling it again with the same family does
/// nothing; another family with the same name or number stops the run.
pub fn provide(family: &'static Family) {
    let mut provided = PROVIDED.lock().unwrap_or_else(|e| e.into_inner());
    if provided.iter().any(|f| std::ptr::eq(*f, family)) {
        return;
    }
    let clash = FAMILIES
        .iter()
        .chain(provided.iter())
        .find(|f| f.name == family.name || f.id == family.id)
        .copied();
    if clash.is_none() {
        provided.push(family);
    }
    drop(provided);
    if let Some(f) = clash {
        panic!(
            "typed family {} ({:?}) clashes with {} ({:?})",
            family.name, family.id, f.name, f.id
        );
    }
}

/// A number or a name used twice in the table: `families` must have neither. Checked at every
/// [`apply`], whichever families the switch has on, so a clash doesn't wait for both to be on.
pub(crate) fn check_table(families: &[&Family]) -> Result<(), String> {
    for (i, a) in families.iter().enumerate() {
        if let Some(b) = families[..i]
            .iter()
            .find(|b| b.name == a.name || b.id == a.id)
        {
            return Err(format!(
                "typed family {} ({:?}) clashes with {} ({:?})",
                a.name, a.id, b.name, b.id
            ));
        }
    }
    Ok(())
}

/// What the switch says about one family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// The translation runs.
    Off,
    /// The typed functions run.
    On,
    /// The typed functions run, and the family's own code keeps both forms live and compares them
    /// ([`Engine::typed_dual`]).
    Dual,
}

/// `NV_TYPED`'s answer for each of `families`. `None` or an empty text is the default: a family is
/// on when it is past its gate. Otherwise a comma-separated list read left to right, each item
/// changing what the items before it left:
///
/// - `off`: every family off;
/// - `all`: back to the default;
/// - `<family>`: that family on, past its gate or not (how a family is tried before its gate);
/// - `<family>:dual`: on, with both forms live;
/// - `<family>:off`: that family off.
///
/// An unknown family or mode is an error: a mistyped switch must not leave a comparison run on the
/// wrong code without saying so.
pub fn parse_switch(spec: Option<&str>, families: &[&Family]) -> Result<Vec<Mode>, String> {
    let default = || -> Vec<Mode> {
        families
            .iter()
            .map(|f| if f.past_gate { Mode::On } else { Mode::Off })
            .collect()
    };
    let mut modes = default();
    let items = spec
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|i| !i.is_empty());
    for item in items {
        match item {
            "off" => modes.fill(Mode::Off),
            "all" => modes = default(),
            _ => {
                let (name, mode) = match item.split_once(':') {
                    None => (item, Mode::On),
                    Some((name, "dual")) => (name, Mode::Dual),
                    Some((name, "off")) => (name, Mode::Off),
                    Some((_, other)) => {
                        return Err(format!(
                            "unknown mode `{other}` in `{item}` (dual or off, or none for on)"
                        ))
                    }
                };
                let Some(k) = families.iter().position(|f| f.name == name) else {
                    let known: Vec<&str> = families.iter().map(|f| f.name).collect();
                    return Err(format!(
                        "unknown family `{name}` (off, all, or one of: {})",
                        known.join(", ")
                    ));
                };
                modes[k] = mode;
            }
        }
    }
    Ok(modes)
}

/// The per-engine state of the seam ([`Engine`]'s `typed` field).
#[derive(Default)]
pub(crate) struct State {
    /// The families whose [`typed_call!`](crate::typed_call)s are Rust calls now: [`State::allowed`]
    /// while no diagnostic is on.
    pub(crate) direct: u32,
    /// The families that are on and that nothing has been registered over.
    allowed: u32,
    /// The families the switch put on `dual`.
    dual: u32,
    /// The addresses of the families that are on, with the family's bit.
    pub(crate) members: AddrMap<u32, u32>,
    /// The differential test's direct run: every `typed_call!` goes direct with the call log on,
    /// and notes itself in the log ([`Engine::typed_note_direct`]).
    forced: bool,
    /// [`Engine::typed_note_direct`] is running: the call hook is seeing a direct call.
    noting: bool,
    /// The differential test keeps `with_stack` frames in [`Engine::stack_frames`], to read a stack
    /// word as an offset into its frame.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) keep_frames: bool,
    /// The differential test's direct run: each `typed_call!` that went by address because no family
    /// the test names holds its address, so the function it names has not run.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) unnamed: std::cell::RefCell<Vec<u32>>,
}

impl State {
    /// Whether any family can go direct at all: the one test [`Engine::call_full`] pays for the seam
    /// with none on.
    ///
    /// [`Engine::call_full`]: crate::Engine
    #[inline(always)]
    pub(crate) fn any_allowed(&self) -> bool {
        self.allowed != 0
    }

    /// Something was registered over `addr`. A member's family then goes by address for the rest
    /// of this engine's life, so what was registered is what runs.
    #[inline]
    pub(crate) fn replaced(&mut self, addr: u32) {
        if self.allowed != 0 {
            if let Some(&bit) = self.members.get(&addr) {
                self.allowed &= !bit;
                self.direct &= !bit;
            }
        }
    }
}

impl Engine {
    /// Whether a [`typed_call!`](crate::typed_call) of `family` at `addr` is a Rust call now. In a
    /// play or release build one load and one test, and `addr` isn't read. A build with debug
    /// assertions (the tests) also stops when `addr` isn't in `family`, and asks
    /// [`Engine::diagnosing`] itself, so a test that turns a diagnostic on sees the very next typed
    /// call by address; the others see it from the next call by address on
    /// ([`Engine::typed_sync`]).
    ///
    /// [`Engine::diagnosing`]: crate::Engine
    #[inline(always)]
    pub fn typed_direct(&self, family: FamilyId, addr: u32) -> bool {
        if self.typed.direct & family.bit() == 0 {
            return false;
        }
        !cfg!(debug_assertions) || self.typed_direct_checked(family, addr)
    }

    /// [`Engine::typed_direct`]'s checks in a build with debug assertions, with the flag set.
    ///
    /// The address has to be in the named family: what is registered over an address clears the
    /// family that holds it ([`State::replaced`]), so a call naming another family would go on
    /// past an override, and one at an address in no family would never see one.
    #[inline(never)]
    fn typed_direct_checked(&self, family: FamilyId, addr: u32) -> bool {
        match self.typed.members.get(&addr).copied() {
            Some(bit) if bit == family.bit() => {}
            // The differential test's direct run, for a family the test doesn't name: by address,
            // as a play run with that family off goes. The function it names hasn't run, which the
            // report holds against the test.
            None if self.typed.forced => {
                #[cfg(any(test, feature = "test-support"))]
                self.typed.unnamed.borrow_mut().push(addr);
                return false;
            }
            Some(bit) => panic!(
                "typed_call! at {addr:08x} names {family:?}, but the address is in the family \
                 with bit {bit:#x}"
            ),
            None => panic!(
                "typed_call! at {addr:08x} names {family:?}, but the address is in no family \
                 that is on"
            ),
        }
        self.typed.forced || !self.diagnosing()
    }

    /// Whether the differential test's direct run is on: a build with debug assertions asks at each
    /// direct `typed_call!`.
    #[doc(hidden)]
    #[inline]
    pub fn typed_noting_direct(&self) -> bool {
        self.typed.forced
    }

    /// The differential test's direct run: a direct `typed_call!` shows in the call log and the
    /// call hook as the call by address does, so the two runs compare call for call.
    #[doc(hidden)]
    #[cold]
    pub fn typed_note_direct(&mut self, addr: u32, words: &[u32]) {
        if let Some(log) = &mut self.call_log {
            log.push((addr, words.to_vec()));
        }
        if let Some(hook) = self.call_hook {
            self.typed.noting = true;
            hook(self, addr, false);
            self.typed.noting = false;
        }
    }

    /// Whether the call the hook is seeing is a direct one.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn typed_noted_direct(&self) -> bool {
        self.typed.noting
    }

    /// The differential test's direct run: from here on a `typed_call!` goes direct when its address
    /// is in the family it names, with the diagnostics on. An address in no family goes by address,
    /// and one in another family stops ([`Engine::typed_direct_checked`]).
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn typed_force_direct(&mut self) {
        self.typed.forced = true;
        self.typed.direct = u32::MAX;
    }

    /// Registers over `addr` without clearing its family: the differential test's function under
    /// test, once its family is on.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn typed_register_under_test(&mut self, addr: u32, f: AbiFn) {
        let (allowed, direct) = (self.typed.allowed, self.typed.direct);
        self.register(addr, f);
        self.typed.allowed = allowed;
        self.typed.direct = direct;
    }

    /// Whether `family` runs with both forms live (`NV_TYPED=<family>:dual`).
    #[inline]
    pub fn typed_dual(&self, family: FamilyId) -> bool {
        self.typed.dual & family.bit() != 0
    }

    /// Brings the direct flags up to the diagnostics: clear while any is on.
    pub(crate) fn typed_sync(&mut self) {
        if self.typed.forced {
            return;
        }
        self.typed.direct = if self.diagnosing() {
            0
        } else {
            self.typed.allowed
        };
    }
}

/// `NV_TYPED` as text. A value that isn't Unicode can't name a family, so it is refused rather than
/// read as unset, which would run the default where something else was asked for.
fn switch_value(value: Option<OsString>) -> Result<Option<String>, String> {
    value
        .map(|v| {
            v.into_string()
                .map_err(|v| format!("its value {v:?} is not Unicode text"))
        })
        .transpose()
}

/// `NV_TYPED` as the process started with it.
fn switch_text() -> Option<&'static str> {
    static TEXT: OnceLock<Option<String>> = OnceLock::new();
    TEXT.get_or_init(|| {
        switch_value(std::env::var_os("NV_TYPED")).unwrap_or_else(|why| panic!("NV_TYPED: {why}"))
    })
    .as_deref()
}

/// Registers every family the switch has on over the translations ([`Engine::new`], after all
/// registration). With no family, or with `NV_TYPED=off`, it registers nothing and leaves the flags
/// clear.
pub fn apply(e: &mut Engine) {
    let provided = PROVIDED.lock().unwrap_or_else(|p| p.into_inner()).clone();
    let families: Vec<&Family> = FAMILIES.iter().copied().chain(provided).collect();
    apply_families(e, &families, switch_text());
}

/// [`apply`] with the table and the switch given.
pub(crate) fn apply_families(e: &mut Engine, families: &[&Family], spec: Option<&str>) {
    check_table(families).unwrap_or_else(|why| panic!("{why}"));
    let modes = parse_switch(spec, families).unwrap_or_else(|why| panic!("NV_TYPED: {why}"));
    let (mut allowed, mut dual) = (0, 0);
    for (family, mode) in families.iter().zip(modes) {
        if mode == Mode::Off {
            continue;
        }
        let bit = family.id.bit();
        for (addr, f) in (family.funcs)() {
            // One implementation per address (the extender notes' R1).
            if e.typed.members.insert(addr, bit).is_some() {
                panic!(
                    "{addr:08x} is in typed family {} and another family that is on",
                    family.name
                );
            }
            e.register(addr, f);
        }
        allowed |= bit;
        if mode == Mode::Dual {
            dual |= bit;
        }
    }
    e.typed.allowed = allowed;
    e.typed.dual = dual;
    e.typed_sync();
}

/// A call between typed functions: `typed_call!(FAMILY, ADDR, path::f(e, a, b))`, where `e` is the
/// `&mut Engine` in scope and `ADDR` the exe address `f` is registered at. A Rust call when the
/// family's direct flag is set, else [`Engine::call`] with the same words; the result has `f`'s
/// type either way. The arguments are evaluated once, first, in order, so they may read through
/// `e`, and their types are `Copy`, as every argument type is. The branches agree but for a signalling
/// NaN in an `f32` result, which only the by-address branch quiets.
///
/// Nothing here can check that `ADDR` is `f`'s address at compile time. The repository's hygiene
/// test does: the literal has to be registered under that name. A test build also stops when `ADDR`
/// isn't in `FAMILY`, and the differential test runs this direct branch.
///
/// Never at an address in `data/overridable.tsv`: there the call stays `e.call` (the hygiene test
/// enforces that too). `f`'s crate must be this crate or one below it, or the name does not resolve;
/// a callee above stays `e.call`.
///
/// `typed_call!(FAMILY, ADDR, f(e); words a, b)` is for a callee that takes no parameters and whose
/// callers push words it never reads (an empty function shared by many signatures): the words are
/// evaluated, go along by address, as the translations pass them, and stay out of the Rust call.
#[macro_export]
macro_rules! typed_call {
    ($family:expr, $addr:expr, $($f:ident)::+ ($e:ident) ; words $($w:expr),+ $(,)?) => {{
        let words = $crate::args![$($w),+];
        if $e.typed_direct($family, $addr) {
            if cfg!(debug_assertions) && $e.typed_noting_direct() {
                $e.typed_note_direct($addr, &words);
            }
            $($f)::+($e)
        } else {
            $crate::abi::RetVal::from_ret($e.call($addr, &words))
        }
    }};
    ($family:expr, $addr:expr, $($f:ident)::+ ($e:ident $(, $a:expr)* $(,)?)) => {
        $crate::typed_call!(@bind $family, $addr, [$($f)::+], $e, [], $($a,)*)
    };
    // Each argument into a local of its own (macro hygiene keeps the `arg`s of each round apart).
    (@bind $family:expr, $addr:expr, [$($f:tt)*], $e:ident, [$($bound:ident)*], $a:expr, $($rest:expr,)*) => {{
        let arg = $a;
        $crate::typed_call!(@bind $family, $addr, [$($f)*], $e, [$($bound)* arg], $($rest,)*)
    }};
    (@bind $family:expr, $addr:expr, [$($f:tt)*], $e:ident, [$($bound:ident)*],) => {
        if $e.typed_direct($family, $addr) {
            if cfg!(debug_assertions) && $e.typed_noting_direct() {
                $e.typed_note_direct($addr, &$crate::args![$($bound),*]);
            }
            $($f)*($e, $($bound),*)
        } else {
            $crate::abi::RetVal::from_ret($e.call($addr, &$crate::args![$($bound),*]))
        }
    };
}

#[cfg(test)]
mod tests {
    // Stand-ins cast freely; the rule is for the functions they stand in for.
    #![allow(clippy::cast_possible_truncation)]

    use super::*;
    use crate::abi::{Ret, RetVal};
    use std::cell::RefCell;

    const FAMILY: FamilyId = FamilyId::new(3);
    const LEAF: u32 = 0x0040_1000;
    const OTHER: u32 = 0x0040_1010;
    const CALLER: u32 = 0x0040_1020;

    /// The stand-in leaf's translation and typed version answer differently here, so a test can
    /// see which one ran.
    fn translated_leaf(_: &mut Engine, x: u32) -> u32 {
        x + 1
    }
    fn typed_leaf(_: &mut Engine, x: u32) -> u32 {
        x + 1000
    }

    fn funcs() -> Vec<(u32, AbiFn)> {
        vec![crate::entry!(0x0040_1000, typed_leaf(u32) -> u32)]
    }

    static STAND_IN: Family = Family {
        name: "stand_in",
        id: FAMILY,
        past_gate: false,
        threads: Threads::Both,
        funcs,
    };
    static PAST_GATE: Family = Family {
        name: "past_gate",
        id: FamilyId::new(4),
        past_gate: true,
        threads: Threads::Both,
        funcs: || vec![],
    };

    /// A caller as the translation writes it.
    fn plain_caller(e: &mut Engine, x: u32) -> u32 {
        let a = e.call(LEAF, &crate::args![x]).u32();
        a + e.call(OTHER, &crate::args![a]).u32()
    }

    /// The same caller after a typed-call rewrite.
    fn typed_caller(e: &mut Engine, x: u32) -> u32 {
        let a: u32 = crate::typed_call!(FAMILY, LEAF, typed_leaf(e, x));
        a + e.call(OTHER, &crate::args![a]).u32()
    }

    fn engine(caller: AbiFn) -> Engine {
        let mut e = Engine::new_untyped();
        let (leaf, f) = crate::entry!(0x0040_1000, translated_leaf(u32) -> u32);
        e.register(leaf, f);
        e.register(OTHER, |_, a| Ret {
            eax: a[0] * 2,
            ..Ret::default()
        });
        e.register(CALLER, caller);
        e
    }

    fn plain() -> AbiFn {
        crate::entry!(0x0040_1020, plain_caller(u32) -> u32).1
    }

    fn typed() -> AbiFn {
        crate::entry!(0x0040_1020, typed_caller(u32) -> u32).1
    }

    thread_local! {
        static CALLS: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
    }

    /// Records every call by address with no per-call diagnostic on: a used-up slice asks the hook
    /// at each call, on [`Engine::call`]'s fast path.
    fn record_call(_: &mut Engine, addr: u32) {
        CALLS.with(|c| c.borrow_mut().push(addr));
    }

    /// What one engine does: (the answer, the calls by address in order, the words logged).
    fn observe(e: &mut Engine) -> (u32, Vec<u32>, Vec<(u32, Vec<u32>)>) {
        CALLS.with(|c| c.borrow_mut().clear());
        e.slice_hook = Some(record_call);
        e.slice_left = 0;
        let answer = e.call(CALLER, &[5]).u32();
        e.slice_hook = None;
        let calls = CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(CALLER, &[5]).u32(), answer);
        let log = e.call_log.take().unwrap();
        (answer, calls, log)
    }

    /// The acceptance test of the seam: with no family, or the family off, a caller that goes
    /// through `typed_call!` makes the same calls in the same order with the same words as the
    /// same caller written with `e.call`. The family on (direct) shows the test can see a
    /// difference.
    #[test]
    fn with_no_family_or_off_the_same_calls_in_the_same_order() {
        let want = observe(&mut engine(plain()));
        assert_eq!(want.0, 6 + 12);
        assert_eq!(want.1, [CALLER, LEAF, OTHER]);
        assert_eq!(
            want.2,
            [(CALLER, vec![5]), (LEAF, vec![5]), (OTHER, vec![6]),]
        );

        // No family registered: the rewritten caller, on a plain engine.
        let mut no_family = engine(typed());
        apply_families(&mut no_family, &[], None);
        assert_eq!(observe(&mut no_family), want);
        // The switch off, and a family not past its gate left alone.
        for spec in [Some("off"), Some("stand_in:off"), Some(""), None] {
            let mut off = engine(typed());
            apply_families(&mut off, &[&STAND_IN], spec);
            assert_eq!(off.typed.allowed, 0);
            assert_eq!(observe(&mut off), want, "{spec:?}");
        }

        // On: the typed answer, and the leaf is no longer a call.
        let mut on = engine(typed());
        apply_families(&mut on, &[&STAND_IN], Some("stand_in"));
        let (answer, calls, log) = observe(&mut on);
        assert_eq!(answer, 1005 + 2010);
        assert_eq!(calls, [CALLER, OTHER]);
        // With the log on it is a call again, to the typed version: every tool sees it.
        assert_eq!(
            log,
            [(CALLER, vec![5]), (LEAF, vec![5]), (OTHER, vec![1005]),]
        );
    }

    /// `Engine::new` runs the seam: with no switch a family not past its gate changes nothing.
    #[test]
    fn engine_new_leaves_a_family_before_its_gate_off() {
        assert!(FAMILIES.iter().all(|f| !f.past_gate));
        let e = Engine::new();
        assert!(!e.typed.any_allowed());
        assert_eq!(e.typed.direct, 0);
        assert!(e.typed.members.is_empty());
    }

    /// A family that is on replaces the translation at its address; one that is off leaves it.
    #[test]
    fn apply_registers_over_the_translation() {
        let mut e = engine(plain());
        apply_families(&mut e, &[&STAND_IN], Some("off"));
        assert_eq!(e.call(LEAF, &[1]).u32(), 2);
        let mut e = engine(plain());
        apply_families(&mut e, &[&STAND_IN], Some("stand_in"));
        assert_eq!(e.call(LEAF, &[1]).u32(), 1001);
    }

    /// Any per-call diagnostic clears the flag from the next call by address on, and calls by
    /// address go through the typed entry so the tool sees them; off again, the next trip down
    /// the full path restores it.
    #[test]
    fn the_direct_flag_follows_the_diagnostics() {
        fn hook(_: &mut Engine, _: u32, _: bool) {}
        let mut e = engine(typed());
        apply_families(&mut e, &[&STAND_IN], Some("stand_in"));
        assert_eq!(e.typed.direct, FAMILY.bit());
        assert!(e.typed_direct(FAMILY, LEAF));

        e.call_hook = Some(hook);
        // A test build sees it at once; the flag itself moves at the next call by address. A build
        // without debug assertions (`cargo test --release`) still reads the flag alone here.
        assert_eq!(e.typed_direct(FAMILY, LEAF), !cfg!(debug_assertions));
        assert_eq!(e.typed.direct, FAMILY.bit());
        e.call(OTHER, &[1]);
        assert_eq!(e.typed.direct, 0);

        e.call_hook = None;
        // Stays by address (always right) until the full path runs again: a cache miss.
        e.register(0x0040_1030, |_, _| Ret::default());
        assert_eq!(e.typed.direct, 0);
        e.call(0x0040_1030, &[]);
        assert_eq!(e.typed.direct, FAMILY.bit());

        // Every diagnostic `Engine::diagnosing` lists counts.
        let count_each = |set: fn(&mut Engine)| {
            let mut e = engine(typed());
            apply_families(&mut e, &[&STAND_IN], Some("stand_in"));
            set(&mut e);
            e.call(CALLER, &[1]);
            assert_eq!(e.typed.direct, 0);
        };
        count_each(|e| e.call_log = Some(vec![]));
        count_each(|e| e.call_budget = Some(u64::MAX));
        count_each(|e| e.call_counts = Some(Default::default()));
        count_each(|e| e.trace = Some(Default::default()));
        count_each(|e| e.mem.enable_block_check(false, false));
    }

    /// A family that starts with a diagnostic already on (`NV_PROFILE`, `NV_TRACE_*`, `NV_WATCH`:
    /// all set up before [`apply`]) starts by address.
    #[test]
    fn a_diagnostic_on_at_start_keeps_the_flag_clear() {
        let mut e = engine(typed());
        e.trace = Some(Default::default());
        apply_families(&mut e, &[&STAND_IN], Some("stand_in"));
        assert_eq!(e.typed.direct, 0);
    }

    /// A double (or a built-in fix) registered over a member stays in the path: the family goes by
    /// address for good on that engine, with no `cfg(test)` needed, so also from a crate below the
    /// one under test.
    #[test]
    fn a_double_over_a_member_is_what_runs() {
        let mut e = engine(typed());
        apply_families(&mut e, &[&STAND_IN], Some("stand_in"));
        e.register(LEAF, |_, a| Ret {
            eax: a[0] + 7,
            ..Ret::default()
        });
        assert!(!e.typed_direct(FAMILY, LEAF));
        assert_eq!(e.call(CALLER, &[5]).u32(), 12 + 24);
        // No later trip down the full path brings it back.
        e.typed_sync();
        assert!(!e.typed_direct(FAMILY, LEAF));

        let mut e = engine(typed());
        apply_families(&mut e, &[&STAND_IN], Some("stand_in"));
        e.register_double(LEAF, |_, a| Ret {
            eax: a[0] + 9,
            ..Ret::default()
        });
        assert_eq!(e.call(CALLER, &[5]).u32(), 14 + 28);

        // Another address leaves the family alone.
        let mut e = engine(typed());
        apply_families(&mut e, &[&STAND_IN], Some("stand_in"));
        e.register(OTHER, |_, _| Ret::default());
        assert!(e.typed_direct(FAMILY, LEAF));
    }

    /// Two engines in one process don't share flags.
    #[test]
    fn the_flag_is_per_engine() {
        let mut on = engine(typed());
        apply_families(&mut on, &[&STAND_IN], Some("stand_in"));
        let off = engine(typed());
        assert!(on.typed_direct(FAMILY, LEAF));
        assert!(!off.typed_direct(FAMILY, LEAF));
    }

    /// `typed_call!`'s `; words` form: the words evaluated once and first, passed by address, left out
    /// of the Rust call, and noted for the differential test as the call by address logs them.
    #[test]
    fn words_a_callee_never_reads_go_by_address_only() {
        fn empty(_: &mut Engine) {}
        const EMPTY: u32 = 0x0040_1070;
        let run = |e: &mut Engine| {
            e.map(0x0200_0000, 0x10);
            e.mem.set_u32(0x0200_0004, 40);
            let mut evaluated = 0;
            let () = crate::typed_call!(FAMILY, EMPTY, empty(e); words {
                evaluated += 1;
                e.mem.u32(0x0200_0004)
            }, 2u32);
            evaluated
        };
        let mut by_address = Engine::new_untyped();
        by_address.register(EMPTY, |_, _| Ret::default());
        by_address.call_log = Some(vec![]);
        assert_eq!(run(&mut by_address), 1);
        assert_eq!(by_address.call_log.take().unwrap(), [(EMPTY, vec![40, 2])]);
        let mut direct = Engine::new_untyped();
        direct.typed.members.insert(EMPTY, FAMILY.bit());
        direct.typed.allowed = FAMILY.bit();
        direct.typed_sync();
        // No function registered: a call by address would stop.
        assert_eq!(run(&mut direct), 1);
        direct.call_log = Some(vec![]);
        direct.typed_force_direct();
        assert_eq!(run(&mut direct), 1);
        assert_eq!(direct.call_log.take().unwrap(), [(EMPTY, vec![40, 2])]);
    }

    /// `typed_call!`: arguments evaluated once and first (they may read through `e`), floats in two
    /// words by address, a result of any type, a unit result, a path.
    #[test]
    fn typed_call_takes_any_arguments_and_results() {
        mod inner {
            use crate::prelude::*;
            pub fn add(e: &mut Engine, a: u32, b: f64) -> f32 {
                let _ = e;
                a as f32 + b as f32
            }
            pub fn nothing(e: &mut Engine) {
                e.mem.set_u32(0x0200_0000, 5);
            }
        }
        const ADD: u32 = 0x0040_1040;
        const NOTHING: u32 = 0x0040_1050;
        let run = |e: &mut Engine| -> (f32, u32, u32) {
            e.map(0x0200_0000, 0x10);
            e.mem.set_u32(0x0200_0004, 40);
            let mut evaluated = 0;
            let sum: f32 = crate::typed_call!(
                FAMILY,
                ADD,
                inner::add(
                    e,
                    {
                        evaluated += 1;
                        e.mem.u32(0x0200_0004)
                    },
                    2.5f64,
                )
            );
            let () = crate::typed_call!(FAMILY, NOTHING, inner::nothing(e));
            (sum, evaluated, e.mem.u32(0x0200_0000))
        };
        let (add, add_f) = crate::entry!(0x0040_1040, inner::add(u32, f64) -> f32);
        let (nothing, nothing_f) = crate::entry!(0x0040_1050, inner::nothing());
        let by_address = {
            let mut e = Engine::new_untyped();
            e.register(add, add_f);
            e.register(nothing, nothing_f);
            run(&mut e)
        };
        assert_eq!(by_address, (42.5, 1, 5));
        let direct = {
            let mut e = Engine::new_untyped();
            e.typed.members.insert(ADD, FAMILY.bit());
            e.typed.members.insert(NOTHING, FAMILY.bit());
            e.typed.allowed = FAMILY.bit();
            e.typed_sync();
            // No function registered: a call by address would stop.
            run(&mut e)
        };
        assert_eq!(direct, by_address);
        // The two forms read the result alike.
        assert_eq!(<f32 as RetVal>::from_ret(42.5f32.into_ret()), 42.5);
    }

    #[test]
    fn the_switch_reads_left_to_right() {
        use Mode::*;
        let table: &[&Family] = &[&STAND_IN, &PAST_GATE];
        let parse = |s: Option<&str>| parse_switch(s, table);
        assert_eq!(parse(None), Ok(vec![Off, On]));
        assert_eq!(parse(Some("")), Ok(vec![Off, On]));
        assert_eq!(parse(Some("all")), Ok(vec![Off, On]));
        assert_eq!(parse(Some("off")), Ok(vec![Off, Off]));
        assert_eq!(parse(Some("stand_in")), Ok(vec![On, On]));
        assert_eq!(parse(Some("stand_in:dual")), Ok(vec![Dual, On]));
        assert_eq!(parse(Some("past_gate:off")), Ok(vec![Off, Off]));
        assert_eq!(parse(Some("off, stand_in")), Ok(vec![On, Off]));
        assert_eq!(parse(Some("stand_in,off")), Ok(vec![Off, Off]));
        assert_eq!(parse(Some("stand_in,all")), Ok(vec![Off, On]));
        assert!(parse(Some("nope")).unwrap_err().contains("unknown family"));
        assert!(parse(Some("stand_in:maybe"))
            .unwrap_err()
            .contains("unknown mode"));
        // With no table only the words stand.
        assert_eq!(parse_switch(Some("off,all"), &[]), Ok(vec![]));
        assert!(parse_switch(Some("leaf"), &[]).is_err());
    }

    /// Test builds tie the address to the family: what is registered over an address clears the
    /// family holding it, so a call naming another family, or at an address in no family, would go
    /// past an override in play.
    #[test]
    fn a_typed_call_outside_its_family_stops_a_test_build() {
        if !cfg!(debug_assertions) {
            return;
        }
        const OTHER_FAMILY: FamilyId = FamilyId::new(4);
        let run = |family: FamilyId, addr: u32| {
            let mut e = engine(plain());
            apply_families(&mut e, &[&STAND_IN, &PAST_GATE], Some("stand_in"));
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.typed_direct(family, addr)
            }))
            .map_err(|p| p.downcast_ref::<String>().cloned().unwrap_or_default())
        };
        assert_eq!(run(FAMILY, LEAF), Ok(true));
        let why = run(OTHER_FAMILY, LEAF).unwrap_err();
        assert!(why.contains("in the family with bit 0x8"), "{why}");
        let why = run(FAMILY, OTHER).unwrap_err();
        assert!(why.contains("in no family that is on"), "{why}");
    }

    /// The differential test's direct run: an address in the named family goes direct, one in no
    /// family by address, and one in another family stops.
    #[test]
    fn the_direct_run_goes_by_address_outside_every_family() {
        if !cfg!(debug_assertions) {
            return;
        }
        let mut e = engine(plain());
        apply_families(&mut e, &[&STAND_IN, &PAST_GATE], Some("stand_in"));
        e.typed_force_direct();
        assert!(e.typed_direct(FAMILY, LEAF));
        assert!(!e.typed_direct(FAMILY, OTHER));
        const OTHER_FAMILY: FamilyId = FamilyId::new(4);
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.typed_direct(OTHER_FAMILY, LEAF)
        }));
        assert!(stopped.is_err());
    }

    /// What the two branches of a `typed_call!` do with a signalling NaN in an `f32` result: the
    /// docs say they differ, and this is the difference.
    #[test]
    fn an_f32_result_by_address_quiets_a_signalling_nan() {
        fn signalling(_: &mut Engine) -> f32 {
            f32::from_bits(0x7f80_0001)
        }
        const ADDR: u32 = 0x0040_1060;
        let run = |e: &mut Engine| -> f32 { crate::typed_call!(FAMILY, ADDR, signalling(e)) };
        let mut by_address = Engine::new_untyped();
        let (addr, f) = crate::entry!(0x0040_1060, signalling() -> f32);
        by_address.register(addr, f);
        assert_eq!(run(&mut by_address).to_bits(), 0x7fc0_0001);
        let mut direct = Engine::new_untyped();
        direct.typed.members.insert(ADDR, FAMILY.bit());
        direct.typed.allowed = FAMILY.bit();
        direct.typed_sync();
        assert_eq!(run(&mut direct).to_bits(), 0x7f80_0001);
    }

    #[test]
    fn a_family_states_its_control_words_once() {
        assert_eq!(Threads::Both.control_words(), [OTHER_THREADS, MAIN_THREAD]);
        assert_eq!(Threads::Main.control_words(), [MAIN_THREAD]);
        assert_eq!(Threads::Other.control_words(), [OTHER_THREADS]);
        assert_eq!(MAIN_THREAD, 0x007f);
    }

    /// A clash in the table stops every start, whichever families the switch has on.
    #[test]
    fn a_number_or_name_used_twice_in_the_table_stops_the_start() {
        static SAME_ID: Family = Family {
            name: "same_id",
            id: FAMILY,
            past_gate: false,
            threads: Threads::Both,
            funcs: || vec![],
        };
        static SAME_NAME: Family = Family {
            name: "stand_in",
            id: FamilyId::new(9),
            past_gate: false,
            threads: Threads::Both,
            funcs: || vec![],
        };
        assert!(check_table(&[&STAND_IN, &PAST_GATE]).is_ok());
        for clash in [&SAME_ID, &SAME_NAME] {
            let why = check_table(&[&STAND_IN, clash]).unwrap_err();
            assert!(why.contains("clashes with stand_in"), "{why}");
            let stopped = std::panic::catch_unwind(|| {
                // Both off: the clash still stops it.
                apply_families(&mut Engine::new_untyped(), &[&STAND_IN, clash], Some("off"));
            });
            assert!(stopped.is_err());
        }
    }

    /// A value that isn't text is refused, not read as unset.
    #[test]
    fn a_switch_value_that_is_not_text_is_refused() {
        assert_eq!(switch_value(None), Ok(None));
        assert_eq!(
            switch_value(Some(OsString::from("off"))),
            Ok(Some("off".to_string()))
        );
        #[cfg(windows)]
        let bad = {
            use std::os::windows::ffi::OsStringExt;
            OsString::from_wide(&[0x006f, 0xd800])
        };
        #[cfg(unix)]
        let bad = {
            use std::os::unix::ffi::OsStringExt;
            OsString::from_vec(vec![0x6f, 0xff])
        };
        let why = switch_value(Some(bad)).unwrap_err();
        assert!(why.contains("not Unicode"), "{why}");
    }

    #[test]
    fn dual_is_on_and_marked() {
        let mut e = engine(plain());
        apply_families(&mut e, &[&STAND_IN], Some("stand_in:dual"));
        assert!(e.typed_dual(FAMILY));
        assert_eq!(e.call(LEAF, &[1]).u32(), 1001);
        let mut e = engine(plain());
        apply_families(&mut e, &[&STAND_IN], Some("stand_in"));
        assert!(!e.typed_dual(FAMILY));
    }

    #[test]
    #[should_panic(expected = "00401000 is in typed family")]
    fn an_address_in_two_families_that_are_on_stops() {
        static TWIN: Family = Family {
            name: "twin",
            id: FamilyId::new(5),
            past_gate: true,
            threads: Threads::Both,
            funcs,
        };
        let mut e = engine(plain());
        apply_families(&mut e, &[&STAND_IN, &TWIN], Some("stand_in"));
    }

    #[test]
    #[should_panic(expected = "NV_TYPED: unknown family")]
    fn a_mistyped_switch_stops_the_start() {
        apply_families(&mut engine(plain()), &[&STAND_IN], Some("stand_inn"));
    }

    #[test]
    fn a_crate_above_provides_its_family_once() {
        static ABOVE: Family = Family {
            name: "provided_above",
            id: FamilyId::new(30),
            past_gate: false,
            threads: Threads::Both,
            funcs: || vec![],
        };
        provide(&ABOVE);
        provide(&ABOVE);
        let provided = PROVIDED.lock().unwrap();
        assert_eq!(
            provided
                .iter()
                .filter(|f| f.name == "provided_above")
                .count(),
            1
        );
    }

    #[test]
    #[should_panic(expected = "clashes with")]
    fn a_clash_between_provided_families_stops() {
        static FIRST: Family = Family {
            name: "clash_first",
            id: FamilyId::new(29),
            past_gate: false,
            threads: Threads::Both,
            funcs: || vec![],
        };
        static SECOND: Family = Family {
            name: "clash_second",
            id: FamilyId::new(29),
            past_gate: false,
            threads: Threads::Both,
            funcs: || vec![],
        };
        provide(&FIRST);
        provide(&SECOND);
    }

    /// The switch end to end: a child process of this test binary, started with `NV_TYPED`.
    /// `Engine::new` reads it and registers the provided family.
    #[test]
    fn nv_typed_reaches_engine_new_in_a_child_process() {
        let run = |spec: Option<&str>| {
            let mut child =
                std::process::Command::new(std::env::current_exe().expect("this test binary"));
            child.args([
                "--ignored",
                "--exact",
                "--nocapture",
                "typed::tests::child_prints_what_engine_new_did",
            ]);
            child.env_remove("NV_TYPED");
            if let Some(spec) = spec {
                child.env("NV_TYPED", spec);
            }
            let out = child.output().expect("the child runs");
            (
                out.status.success(),
                String::from_utf8_lossy(&out.stdout).into_owned(),
                String::from_utf8_lossy(&out.stderr).into_owned(),
            )
        };
        let (ok, stdout, _) = run(None);
        assert!(ok && stdout.contains("answer 2 direct false"), "{stdout}");
        let (ok, stdout, _) = run(Some("off"));
        assert!(ok && stdout.contains("answer 2 direct false"), "{stdout}");
        let (ok, stdout, _) = run(Some("child_family"));
        assert!(ok && stdout.contains("answer 1001 direct true"), "{stdout}");
        let (ok, _, stderr) = run(Some("child_famly"));
        assert!(
            !ok && stderr.contains("NV_TYPED: unknown family `child_famly`"),
            "{stderr}"
        );
    }

    /// What a `typed_call!` costs with the flag set and clear, a call to the same one-line getter
    /// as `dispatch_speed` in `engine.rs`: `cargo test --release -p engine-core --lib -- --ignored
    /// --nocapture typed_call_speed`. `NV_BENCH_CALLS` sets how many (default 300M).
    #[test]
    #[ignore]
    fn typed_call_speed() {
        #[inline(always)]
        fn getter(e: &mut Engine, this: u32) -> u32 {
            e.mem.u32(this + 4)
        }
        let n: u64 = std::env::var("NV_BENCH_CALLS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300_000_000);
        let mut e = Engine::new_untyped();
        let (addr, f) = crate::entry!(0x0055_9450, getter(u32) -> u32);
        e.register(addr, f);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 1);
        e.typed.members.insert(addr, FAMILY.bit());
        let e = &mut e;
        for (name, direct) in [("flag set", FAMILY.bit()), ("flag clear", 0)] {
            e.typed.direct = direct;
            let start = std::time::Instant::now();
            let mut sum = 0u32;
            for _ in 0..n {
                let got: u32 =
                    crate::typed_call!(FAMILY, 0x0055_9450, getter(e, std::hint::black_box(this)));
                sum = sum.wrapping_add(got);
            }
            let ns = start.elapsed().as_secs_f64() * 1e9 / n as f64;
            assert_eq!(sum, n as u32);
            println!("typed_call!, {name}: {ns:.2} ns a call");
        }
    }

    /// Run only by [`nv_typed_reaches_engine_new_in_a_child_process`].
    #[test]
    #[ignore]
    fn child_prints_what_engine_new_did() {
        // Past `.text`: no translation here, so a family that is on shows by being there at all.
        const CHILD_ADDR: u32 = 0x00ff_0000;
        static CHILD: Family = Family {
            name: "child_family",
            id: FamilyId::new(6),
            past_gate: false,
            threads: Threads::Both,
            funcs: || vec![crate::entry!(0x00ff_0000, typed_leaf(u32) -> u32)],
        };
        provide(&CHILD);
        let mut e = Engine::new();
        if !e.is_registered(CHILD_ADDR) {
            let (addr, f) = crate::entry!(0x00ff_0000, translated_leaf(u32) -> u32);
            e.register(addr, f);
        }
        println!(
            "answer {} direct {}",
            e.call(CHILD_ADDR, &[1]).u32(),
            e.is_registered(CHILD_ADDR) && e.typed_direct(CHILD.id, CHILD_ADDR)
        );
    }
}
