//! The differential test of a typed function (docs/ENGINE_CRATE.md, "Typed code"): two
//! [`Engine`]s, the same setup, the translation on one and the typed version on the other, called
//! at the same address with the same generated inputs. What they did is compared exactly: the
//! return value, the memory after the call, the outgoing calls with their words, whether and how
//! the run stopped, the heap, and the control word.
//!
//! ```ignore
//! Diff::new(entry!(0x00d87410, typed_version(Ptr<Thing>, f32) -> f32))
//!     .family(&MY_FAMILY)
//!     .translation_calls_plainly(&[0x00d87400]) // what the translation calls with no e.call
//!     .inputs(&[Input::Word, Input::F32])
//!     .setup(|e, case| {
//!         // Both engines run this: same objects at the same addresses.
//!         let thing = e.mem.alloc(0x40);
//!         e.mem.set_u32(thing + 8, case.word(0));
//!         e.register(0x00abcdef, |_, _| Ret::default()); // a callee's double
//!         vec![thing, case.f32(1).to_bits()]
//!     })
//!     .run()
//!     .assert_same();
//! ```
//!
//! The engines hold the translations only ([`Engine::new_untyped`]): whatever `NV_TYPED` says, the
//! function differs, and with [`Diff::family`] the families named, and everything else is the same.
//!
//! Three runs per case:
//!
//! 1. The translation.
//! 2. The typed version with every `typed_call!` by address, against the translation.
//! 3. The typed version with each `typed_call!` into a named family direct (Rust calls, with the call
//!    log still seeing them), against run 2, call for call. A `typed_call!` naming the wrong function
//!    shows here, and one whose address is in another family stops here. An address in no named
//!    family goes by address, as it does in play with that family off, but the function it names then
//!    hasn't run at all, so [`Report::assert_same`] fails on it unless the test says so with
//!    [`Diff::goes_by_address`]. [`Report::direct_calls`] says how many went direct. This run needs
//!    debug assertions: under `--release` it is skipped and [`Report::direct_runs`] says 0.
//!
//! Compared, bit for bit:
//!
//! - the return value;
//! - every outgoing call in order: address, words, and at its start a hash of all mapped memory
//!   outside the stack for locals and the clock, so a store moved across a call shows even when the
//!   end result is the same;
//! - a word pointing into the stack for locals, as its offset in its frame, the frame's size and the
//!   bytes behind it ([`Diff::stack_bytes`]), not by value; or, for a word [`Diff::local_at`] names,
//!   as which local it is and that local's own bytes;
//! - memory after the call, every mapped page, and which pages are mapped;
//! - whether and how the run stopped: a run where both stop alike fails unless the test says that
//!   input stops ([`Diff::expect_stops`]), since it compared nothing past the stop;
//! - the heap's top and free bytes, the control word and the clock after the call.
//!
//! A stack address stored in memory, held in the bytes behind a stack word, or returned compares by
//! value, so it fails when the frames differ. With a [`Diff::local_at`] in the test, the stored ones
//! compare as "some stack address" instead, whichever local they point at.
//!
//! Not compared, so a test of those has to look itself:
//!
//! - the stack for locals outside what an outgoing word points at (a local may become native);
//! - the kernel's objects (events, threads, files) and everything else in [`Engine::kernel`];
//! - the heap's block map and free bins: only its top and free bytes;
//! - the time slice and [`Engine::call_stack`], which a direct call doesn't touch by design;
//! - calls another game thread makes, and a by-address call that runs a row of the `atexit` table;
//! - what a translation's plain Rust call does as a call: it isn't logged
//!   ([`Diff::translation_calls_plainly`]).
//!
//! Fresh engines per case, so nothing a case leaves behind reaches the next. That is the harness's
//! cost: [`Diff::max_cases`] caps it.

// This file splits words and offsets: no game code, and the type-aware ban on truncating casts in
// `typed` is for what is.
#![allow(clippy::cast_possible_truncation)]

pub use super::{MAIN_THREAD, OTHER_THREADS};

use super::{apply_families, Family, Threads};
use crate::abi::{AbiFn, Ret};
use crate::cxx_exception::Thrown;
use crate::x87;
use crate::Engine;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};

/// The words tried for an integer or pointer-sized argument, the plainest first.
pub const WORD_VALUES: &[u32] = &[
    1,
    0,
    2,
    0x7f,
    0xff,
    0x100,
    0x7fff_ffff,
    0x8000_0000,
    u32::MAX,
];

/// The bit patterns tried for an `f32`, an ordinary value first (it is what the other arguments
/// hold while one is swept).
pub const F32_VALUES: &[u32] = &[
    0x3f80_0000, // 1
    0xbfc0_0000, // -1.5
    0x3f00_0000, // 0.5
    0x3eaa_aaab, // 1/3: rounds
    0x3dcc_cccd, // 0.1: rounds
    0x3f80_0001, // 1 + 2^-23: the next float above 1
    0x3f7f_ffff, // the float below 1
    0x0000_0000, // 0
    0x8000_0000, // -0
    0x0000_0001, // the smallest denormal
    0x8000_0001, // the same, negative
    0x007f_ffff, // the largest denormal
    0x0080_0000, // the smallest normal
    0x4eff_ffff, // just below 2^31
    0x4f00_0000, // 2^31: one past the int range
    0xcf00_0000, // -2^31: the end of it
    0x4f80_0000, // 2^32
    0x5f00_0000, // 2^63
    0x7f7f_ffff, // the largest finite
    0xff7f_ffff, // the same, negative
    0x7f80_0000, // infinity
    0xff80_0000, // negative infinity
    0x7fc0_0000, // a quiet NaN
    0xffc0_0000, // the x87 default NaN
    0x7f80_0001, // a signalling NaN
    0x7fc1_2345, // a quiet NaN with a payload
];

/// The bit patterns tried for an `f64`, an ordinary value first.
pub const F64_VALUES: &[u64] = &[
    0x3ff0_0000_0000_0000, // 1
    0xbff8_0000_0000_0000, // -1.5
    0x3fe0_0000_0000_0000, // 0.5
    0x3fd5_5555_5555_5555, // 1/3: rounds
    0x3fb9_9999_9999_999a, // 0.1: rounds
    0x3ff0_0000_0000_0001, // 1 + 2^-52: the next double above 1
    0x3ff0_0000_2000_0000, // 1 + 2^-23: the next float above 1
    0x0000_0000_0000_0000, // 0
    0x8000_0000_0000_0000, // -0
    0x0000_0000_0000_0001, // the smallest denormal
    0x000f_ffff_ffff_ffff, // the largest denormal
    0x0010_0000_0000_0000, // the smallest normal
    0x36a0_0000_0000_0000, // the smallest f32 denormal
    0x3ff0_0000_1000_0000, // needs rounding to fit an f32, halfway
    0x41df_ffff_ffc0_0000, // just below 2^31
    0x41e0_0000_0000_0000, // 2^31
    0xc1e0_0000_0000_0000, // -2^31
    0x43e0_0000_0000_0000, // 2^63
    0x47e0_0000_0000_0000, // 2^127
    0x47f0_0000_0000_0000, // 2^128: past every f32
    0x7fef_ffff_ffff_ffff, // the largest finite
    0xffef_ffff_ffff_ffff, // the same, negative
    0x7ff0_0000_0000_0000, // infinity
    0xfff0_0000_0000_0000, // negative infinity
    0x7ff8_0000_0000_0000, // a quiet NaN
    0xfff8_0000_0000_0000, // the x87 default NaN
    0x7ff0_0000_0000_0001, // a signalling NaN
    0x7ff8_0000_0012_3456, // a quiet NaN with a payload
];

/// One generated argument.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Value {
    Word(u32),
    /// Its bits.
    F32(u32),
    /// Its bits: two words, low word first, as [`crate::abi`] passes it.
    F64(u64),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Word(w) => write!(f, "{w:#x}"),
            Value::F32(b) => write!(f, "{:?}f32({b:#010x})", f32::from_bits(*b)),
            Value::F64(b) => write!(f, "{:?}f64({b:#018x})", f64::from_bits(*b)),
        }
    }
}

/// What to generate for one argument.
#[derive(Clone, Copy, Debug)]
pub enum Input {
    /// [`WORD_VALUES`].
    Word,
    /// 1 and 0.
    Flag,
    /// [`F32_VALUES`]: NaNs, zeros, denormals and huge values.
    F32,
    /// [`F64_VALUES`].
    F64,
    /// These words: one per branch that matters.
    Of(&'static [u32]),
}

impl Input {
    fn values(self) -> Vec<Value> {
        match self {
            Input::Word => WORD_VALUES.iter().map(|w| Value::Word(*w)).collect(),
            Input::Flag => vec![Value::Word(1), Value::Word(0)],
            Input::F32 => F32_VALUES.iter().map(|b| Value::F32(*b)).collect(),
            Input::F64 => F64_VALUES.iter().map(|b| Value::F64(*b)).collect(),
            Input::Of(words) => words.iter().map(|w| Value::Word(*w)).collect(),
        }
    }
}

/// One generated set of arguments.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Case(pub Vec<Value>);

impl Case {
    /// Argument `i`, a [`Value::Word`].
    pub fn word(&self, i: usize) -> u32 {
        match self.0[i] {
            Value::Word(w) => w,
            other => panic!("argument {i} is {other:?}, not a word"),
        }
    }

    /// Argument `i`, a [`Value::F32`] (NaN bits kept).
    pub fn f32(&self, i: usize) -> f32 {
        match self.0[i] {
            Value::F32(b) => f32::from_bits(b),
            other => panic!("argument {i} is {other:?}, not an f32"),
        }
    }

    /// Argument `i`, a [`Value::F64`] (NaN bits kept).
    pub fn f64(&self, i: usize) -> f64 {
        match self.0[i] {
            Value::F64(b) => f64::from_bits(b),
            other => panic!("argument {i} is {other:?}, not an f64"),
        }
    }

    /// The arguments as call words.
    pub fn words(&self) -> Vec<u32> {
        let mut out = Vec::with_capacity(self.0.len());
        for v in &self.0 {
            match *v {
                Value::Word(w) | Value::F32(w) => out.push(w),
                Value::F64(b) => {
                    out.push(b as u32);
                    out.push((b >> 32) as u32);
                }
            }
        }
        out
    }
}

/// The cases for `inputs`: all combinations when there are at most `max_cases`. Otherwise every
/// value of each argument with the others at their first value, then pseudo-random combinations
/// (a fixed sequence, so a failure repeats) up to `max_cases`.
pub fn cases(inputs: &[Input], max_cases: usize) -> Vec<Case> {
    let lists: Vec<Vec<Value>> = inputs.iter().map(|i| i.values()).collect();
    if lists.iter().any(Vec::is_empty) {
        return Vec::new();
    }
    let all = lists
        .iter()
        .try_fold(1usize, |n, l| n.checked_mul(l.len()))
        .unwrap_or(usize::MAX);
    if all <= max_cases {
        let mut out = vec![Vec::new()];
        for list in &lists {
            out = out
                .into_iter()
                .flat_map(|head: Vec<Value>| {
                    list.iter().map(move |v| {
                        let mut case = head.clone();
                        case.push(*v);
                        case
                    })
                })
                .collect();
        }
        return out.into_iter().map(Case).collect();
    }
    let plain: Vec<Value> = lists.iter().map(|l| l[0]).collect();
    let mut seen: HashSet<Vec<Value>> = HashSet::new();
    let mut out = Vec::new();
    let mut add = |case: Vec<Value>, out: &mut Vec<Case>| {
        if seen.insert(case.clone()) {
            out.push(Case(case));
        }
    };
    add(plain.clone(), &mut out);
    for (i, list) in lists.iter().enumerate() {
        for v in list {
            let mut case = plain.clone();
            case[i] = *v;
            add(case, &mut out);
        }
    }
    // xorshift64, seeded with a fixed odd word; a bounded number of tries so a small space ends.
    let mut x = 0x2545_f491_4f6c_dd1du64;
    let mut next = move || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x
    };
    let mut tries = 16 * max_cases;
    while out.len() < max_cases && tries > 0 {
        tries -= 1;
        let case = lists
            .iter()
            .map(|l| l[(next() % l.len() as u64) as usize])
            .collect();
        add(case, &mut out);
    }
    out
}

/// A setup: runs on each engine with the case, makes what the function reads and the doubles of
/// its callees, and answers the words of the call.
type Setup = Box<dyn Fn(&mut Engine, &Case) -> Vec<u32>>;

/// The inputs a test says stop ([`Diff::expect_stops`]).
type StopCases = Box<dyn Fn(&Case) -> bool>;

/// One function's comparison ([`Diff::run`]).
pub struct Diff {
    addr: u32,
    typed: AbiFn,
    translation: Option<AbiFn>,
    families: Vec<&'static Family>,
    inputs: Vec<Input>,
    setup: Option<Setup>,
    ignored_calls: Vec<u32>,
    plain_callees: Vec<u32>,
    by_address: Vec<u32>,
    local_words: Vec<LocalWord>,
    control_words: Option<Vec<u16>>,
    max_cases: usize,
    stop_cases: Option<StopCases>,
    stack_bytes: u32,
    nan_payloads: bool,
}

impl Diff {
    /// The typed version as `entry!` gives it: its exe address and its uniform form.
    pub fn new((addr, typed): (u32, AbiFn)) -> Diff {
        Diff {
            addr,
            typed,
            translation: None,
            families: Vec::new(),
            inputs: Vec::new(),
            setup: None,
            ignored_calls: Vec::new(),
            plain_callees: Vec::new(),
            by_address: Vec::new(),
            local_words: Vec::new(),
            control_words: None,
            max_cases: 512,
            stop_cases: None,
            stack_bytes: 64,
            nan_payloads: false,
        }
    }

    /// The function to compare with, for a test whose engine doesn't assemble the translation
    /// (the harness's own tests, or a translation in a crate above). By default the one registered
    /// at the address.
    pub fn translation(mut self, f: AbiFn) -> Diff {
        // Only with debug assertions: an optimised build can merge two functions with the same code,
        // and a typed version may well compile to the same code as its translation.
        assert!(
            !cfg!(debug_assertions) || f as usize != self.typed as usize,
            "the translation given is the typed version: the test would compare it with itself"
        );
        self.translation = Some(f);
        self
    }

    /// A family the typed runs have on, as play would: its members go over their translations, after
    /// the setup, and each `typed_call!` into it goes direct on the direct run. Call again for another
    /// family, as many as the call tree reaches. A `typed_call!` into a family nobody names goes by
    /// address on every run, so the function it names runs nowhere: [`Report::assert_same`] fails on
    /// it unless [`Diff::goes_by_address`] lists its address. The translation run stays translations
    /// only.
    ///
    /// The setup may not put a double over a member ([`Diff::run`] stops): the typed runs would replace
    /// it, and a double on one side only is no comparison.
    pub fn family(mut self, family: &'static Family) -> Diff {
        self.families.push(family);
        self
    }

    /// `typed_call!`s at these addresses go by address in this test on purpose: no family it names
    /// holds them, and their direct branch runs in another test. Without this a `typed_call!` in no named
    /// family fails [`Report::assert_same`], because the function it names has run nowhere.
    pub fn goes_by_address(mut self, addrs: &[u32]) -> Diff {
        self.by_address.extend_from_slice(addrs);
        self
    }

    /// What to generate for each argument.
    pub fn inputs(mut self, inputs: &[Input]) -> Diff {
        self.inputs = inputs.to_vec();
        self
    }

    /// Runs on every engine before the call. Without it the call's words are the case's.
    pub fn setup(mut self, setup: impl Fn(&mut Engine, &Case) -> Vec<u32> + 'static) -> Diff {
        self.setup = Some(Box::new(setup));
        self
    }

    /// Callees the translation calls and the typed version does not call at all, because it does
    /// their work in its own body with no call: left out of the translation's calls. The typed
    /// version calling one still shows, with its words. For a function it reaches as a function,
    /// write `typed_call!`, which the comparison sees through: the hygiene test bans a plain Rust
    /// call to a registered function in typed code. An overridable address can't be listed
    /// (`data/overridable.tsv`): a fix replacing it would never run.
    pub fn ignore_calls(mut self, addrs: &[u32]) -> Diff {
        for addr in addrs {
            refuse_overridable(*addr, "it has to be called, so it can't be ignored");
        }
        self.ignored_calls.extend_from_slice(addrs);
        self
    }

    /// Callees the translation reaches with a plain Rust call, which isn't logged, and the typed
    /// version with `e.call` or `typed_call!`, which is: left out of the typed version's calls. Their
    /// own outgoing calls and what they do to memory still compare, but the words of the listed call
    /// can't, since one side has no record of them: a wrong word shows only by what it changes.
    ///
    /// The real function has to run on both sides, so the setup may not replace one that is registered
    /// ([`Diff::run`] stops): a double would run on the typed side only. An overridable address can't be
    /// listed: its words have to compare, so its family's step moves the translation's call to `e.call`
    /// and lowers the frozen count in `data/overridable.tsv`.
    pub fn translation_calls_plainly(mut self, addrs: &[u32]) -> Diff {
        for addr in addrs {
            refuse_overridable(
                *addr,
                "its words have to compare: e.call in the translation",
            );
        }
        self.plain_callees.extend_from_slice(addrs);
        self
    }

    /// The `word`th word of each call to `callee` is a local the typed version may place differently:
    /// a smaller frame, or siblings left native. It compares as which local it is (the same one
    /// as before, or a new one) and that local's own `size` bytes, not by its offset or its frame's
    /// size. With any of these in the test, a stack address in memory or behind a stack word compares
    /// as "some stack address".
    ///
    /// What it then can't see: a callee reading past `size`, into what the translation had beside the
    /// local; which local a stored stack address points at; and a new local that a later frame puts
    /// at the address of one already gone, which reads as the same local.
    pub fn local_at(mut self, callee: u32, word: usize, size: u32) -> Diff {
        self.local_words.push(LocalWord { callee, word, size });
        self
    }

    /// The control words to run each case under, whatever the family says. By default those of the
    /// family holding the function ([`Family::threads`]), else both [`OTHER_THREADS`] and
    /// [`MAIN_THREAD`].
    pub fn control_words(mut self, words: &[u16]) -> Diff {
        self.control_words = Some(words.to_vec());
        self
    }

    /// The most cases to generate (512 by default); each runs under each control word.
    pub fn max_cases(mut self, n: usize) -> Diff {
        self.max_cases = n;
        self
    }

    /// The inputs that stop, which are this test's point: a stop is the behaviour being compared
    /// there. On any other input a run where both sides stop is a failure, because it compared nothing
    /// after the stop: a bad pointer in the setup would otherwise pass. An input stated here that
    /// doesn't stop fails too. A stop at an untranslated address fails either way: that is a double the
    /// setup lacks.
    pub fn expect_stops(mut self, on: impl Fn(&Case) -> bool + 'static) -> Diff {
        self.stop_cases = Some(Box::new(on));
        self
    }

    /// How many bytes behind a stack word in an outgoing call are compared (64 by default), up to
    /// the end of its frame.
    pub fn stack_bytes(mut self, n: u32) -> Diff {
        self.stack_bytes = n;
        self
    }

    /// NaN payloads may differ, where the float rule accepts it (x87 and SSE differ there and the
    /// game can't see it): in the return value, and in memory and call words that hold a NaN. This
    /// reads aligned words as floats, so an integer that looks like a NaN is also let through; an
    /// infinity or a number against a NaN still differs.
    pub fn allow_nan_payloads(mut self) -> Diff {
        self.nan_payloads = true;
        self
    }

    /// The control words this runs under, and where they came from.
    fn words_to_run(&self) -> Vec<u16> {
        if let Some(words) = &self.control_words {
            return words.clone();
        }
        let holder = self
            .families
            .iter()
            .find(|f| (f.funcs)().iter().any(|(addr, _)| *addr == self.addr));
        holder
            .map_or(Threads::Both, |f| f.threads)
            .control_words()
            .to_vec()
    }

    /// Runs every case, and compares.
    #[must_use = "the report says whether they agreed: call `assert_same`"]
    pub fn run(&self) -> Report {
        let mut report = Report {
            control_words: self.words_to_run(),
            ..Report::default()
        };
        for case in cases(&self.inputs, self.max_cases) {
            for word in report.control_words.clone() {
                report.runs += 1;
                self.run_case(&case, word, &mut report);
            }
        }
        report
    }

    fn run_case(&self, case: &Case, word: u16, report: &mut Report) {
        let translation = self.side(case, word, Branch::Translation);
        let typed = self.side(case, word, Branch::ByAddress);
        assert_eq!(
            translation.words, typed.words,
            "the setup gave the two engines different call words: it must do the same on both"
        );
        same_registered(&translation, &typed);
        let expected = self.stop_cases.as_ref().is_some_and(|on| on(case));
        match (&translation.stop, &typed.stop) {
            (Some(a), Some(b)) if a == b => {
                report.stopped += 1;
                if !expected || a.starts_with("open function") {
                    report.note(
                        case,
                        word,
                        Branch::ByAddress,
                        Found::StoppedAlike { message: a.clone() },
                    );
                }
            }
            (None, None) if expected => {
                report.note(case, word, Branch::ByAddress, Found::ExpectedStop);
            }
            _ => {}
        }
        self.compare(
            &translation,
            &typed,
            &self.ignored_calls,
            &self.plain_callees,
            case,
            word,
            Branch::ByAddress,
            report,
        );
        // Each `typed_call!`'s direct branch, against the same function by address. Only where debug
        // assertions make the macro note its calls, so the logs can match.
        if cfg!(debug_assertions) {
            report.direct_runs += 1;
            let direct = self.side(case, word, Branch::Direct);
            same_registered(&translation, &direct);
            report.direct_calls += direct.calls.iter().filter(|c| c.direct).count();
            for addr in direct.engine.typed.unnamed.take() {
                if !self.by_address.contains(&addr) && !report.unnamed.contains(&addr) {
                    report.unnamed.push(addr);
                }
            }
            self.compare(
                &typed,
                &direct,
                &[],
                &[],
                case,
                word,
                Branch::Direct,
                report,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compare(
        &self,
        a: &Side,
        b: &Side,
        ignored: &[u32],
        plain: &[u32],
        case: &Case,
        word: u16,
        branch: Branch,
        report: &mut Report,
    ) {
        let mut found = |what: Found| report.note(case, word, branch, what);
        if a.stop != b.stop {
            found(Found::Stop {
                translation: a.stop.clone(),
                typed: b.stop.clone(),
            });
        }
        if !same_ret(&a.ret, &b.ret, self.nan_payloads) {
            found(Found::Return {
                translation: a.ret,
                typed: b.ret,
            });
        }
        compare_calls(a, b, ignored, plain, &mut found);
        if a.control_word != b.control_word {
            found(Found::ControlWord {
                translation: a.control_word,
                typed: b.control_word,
            });
        }
        if a.clock != b.clock {
            found(Found::Clock {
                translation: a.clock,
                typed: b.clock,
            });
        }
        let (a_heap, b_heap) = (a.heap(), b.heap());
        if a_heap != b_heap {
            found(Found::Heap {
                translation: a_heap,
                typed: b_heap,
            });
        }
        compare_memory(
            a,
            b,
            self.nan_payloads,
            !self.local_words.is_empty(),
            &mut found,
        );
    }

    /// One engine's run of the case.
    fn side(&self, case: &Case, word: u16, branch: Branch) -> Side {
        let mut e = Engine::new_untyped();
        // Made first on every side, so an engine's first local or TLS read doesn't move the heap
        // blocks after it: a typed version with native locals may never make them.
        e.with_stack(8, |_, _| ());
        e.tls();
        // What a setup can't replace: see `translation_calls_plainly` and `family`.
        let mut watched = self.plain_callees.clone();
        for family in &self.families {
            watched.extend((family.funcs)().iter().map(|(addr, _)| *addr));
        }
        let before: Vec<_> = watched.iter().map(|a| e.registered_identity(*a)).collect();
        let words = match &self.setup {
            Some(setup) => setup(&mut e, case),
            None => case.words(),
        };
        let mut registered = Vec::with_capacity(watched.len());
        for (addr, before) in watched.iter().zip(before) {
            let now = e.registered_identity(*addr);
            if now.is_some_and(|(_, double)| double) || (before.is_some() && now != before) {
                panic!(
                    "the setup replaced what is registered at {addr:08x}, a family's member or a \
                     callee the translation reaches plainly: that double would run on one side only"
                );
            }
            registered.push((*addr, now));
        }
        match branch {
            Branch::Translation => {
                if let Some(f) = self.translation {
                    e.register(self.addr, f);
                }
                assert!(
                    !cfg!(debug_assertions)
                        || e.registered_identity(self.addr).map(|(f, _)| f)
                            != Some(self.typed as usize),
                    "the translation at {:08x} is the typed version: the test would compare it \
                     with itself",
                    self.addr
                );
            }
            Branch::ByAddress | Branch::Direct => {
                if !self.families.is_empty() {
                    let names: Vec<&str> = self.families.iter().map(|f| f.name).collect();
                    apply_families(&mut e, &self.families, Some(&names.join(",")));
                }
                e.typed_register_under_test(self.addr, self.typed);
            }
        }
        e.typed.keep_frames = true;
        e.call_log = Some(Vec::new());
        e.call_hook = Some(record_call);
        if branch == Branch::Direct {
            e.typed_force_direct();
        }
        RECORDING.with(|r| {
            *r.borrow_mut() = Some(Recording {
                calls: Vec::new(),
                stack_bytes: self.stack_bytes,
                nan_payloads: self.nan_payloads,
                local_words: self.local_words.clone(),
                locals: Vec::new(),
                next_local: 0,
            })
        });
        x87::set_control_word(word);
        let result = catch_unwind(AssertUnwindSafe(|| e.call(self.addr, &words)));
        let control_word = x87::control_word();
        x87::set_control_word(x87::DEFAULT_CONTROL_WORD);
        e.call_hook = None;
        let calls = RECORDING
            .with(|r| r.borrow_mut().take())
            .map(|r| r.calls)
            .unwrap_or_default();
        let stack = e.guest_stack_region();
        let (ret, stop) = match result {
            Ok(ret) => (ret, None),
            Err(payload) => (
                Ret::default(),
                Some(stop_text(&e, stack, self.stack_bytes, payload.as_ref())),
            ),
        };
        let clock = (e.clock_ms, e.clock_us);
        Side {
            engine: e,
            words,
            registered,
            ret,
            stop,
            calls,
            stack,
            control_word,
            clock,
        }
    }
}

/// Stops the test when a setup left a different function at a watched address on two sides: one
/// that was empty before it may get a function, but the same one on every side, or a double runs on
/// one side only and the comparison can't see it.
fn same_registered(a: &Side, b: &Side) {
    for ((addr, on_a), (_, on_b)) in a.registered.iter().zip(&b.registered) {
        assert!(
            on_a == on_b,
            "the setup registered different functions at {addr:08x} on the two sides, a family's \
             member or a callee the translation reaches plainly: it must do the same on both"
        );
    }
}

/// Stops the test when `addr` is in `data/overridable.tsv`.
fn refuse_overridable(addr: u32, why: &str) {
    let hex = format!("{addr:08x}\t");
    if include_str!("../../data/overridable.tsv")
        .lines()
        .any(|l| l.starts_with(&hex))
    {
        panic!("{addr:08x} is overridable: {why}");
    }
}

/// How many differences a [`Report`] keeps in full.
const KEEP_FOUND: usize = 16;

fn same_ret(a: &Ret, b: &Ret, nan_payloads: bool) -> bool {
    a.eax == b.eax
        && a.edx == b.edx
        && (a.st0.to_bits() == b.st0.to_bits()
            || (nan_payloads && a.st0.is_nan() && b.st0.is_nan()))
}

/// What a stop says. A thrown object compares by what it is: its `ThrowInfo` and its address, whose
/// contents the memory comparison covers, or its bytes when it lies on the stack for locals, where
/// its address may differ.
fn stop_text(
    e: &Engine,
    stack: Option<(u32, u32)>,
    stack_bytes: u32,
    payload: &(dyn Any + Send),
) -> String {
    if let Some(thrown) = payload.downcast_ref::<Thrown>() {
        let object = match stack {
            Some((lo, hi)) if (lo..hi).contains(&thrown.object) => {
                format!(
                    "on the stack, {:?}",
                    stack_word(e, thrown.object, hi, stack_bytes, None)
                )
            }
            _ => format!("{:08x}", thrown.object),
        };
        return format!(
            "a thrown object: ThrowInfo {:08x}, object {object}",
            thrown.throw_info
        );
    }
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "a stop with no message".to_string())
}

/// Which run of a function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Branch {
    Translation,
    /// The typed version, every `typed_call!` by address, against the translation.
    ByAddress,
    /// The typed version, every `typed_call!` direct, against its by-address run.
    Direct,
}

/// A word of an outgoing call as it compares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Word {
    /// Compared by value.
    Plain(u32),
    /// An address on the thread's stack for locals: where it points inside its frame, the frame's
    /// size and a hash of the bytes behind it, because a frame's address may differ. A word on the
    /// stack outside every live frame has `frame` 0, with `offset` counted from the stack's end.
    Stack { offset: u32, frame: u32, bytes: u64 },
    /// A word [`Diff::local_at`] names: which local of the run it is, in order of first appearance,
    /// and a hash of its own bytes.
    Local { ordinal: u32, bytes: u64 },
}

/// A word of a call that is a local the typed version may place differently ([`Diff::local_at`]).
#[derive(Clone, Copy, Debug)]
struct LocalWord {
    callee: u32,
    word: usize,
    size: u32,
}

/// One call by address as it was made.
struct Seen {
    addr: u32,
    words: Vec<Word>,
    /// A hash of all mapped memory outside the stack for locals, at the moment of the call.
    memory: u64,
    clock: (u32, u32),
    /// A direct `typed_call!`, noted, not a call by address.
    direct: bool,
}

/// What the running side has seen so far ([`record_call`]).
struct Recording {
    calls: Vec<Seen>,
    stack_bytes: u32,
    nan_payloads: bool,
    local_words: Vec<LocalWord>,
    /// The locals [`Diff::local_at`] words have pointed at, while their frames live, with their
    /// ordinals.
    locals: Vec<(u32, u32)>,
    next_local: u32,
}

thread_local! {
    /// The call hook is a plain function, so what it writes lives here for the side's run. A side
    /// runs on one host thread; calls another game thread makes aren't seen.
    static RECORDING: RefCell<Option<Recording>> = const { RefCell::new(None) };
}

/// The call hook of every run: each call by address (or a direct `typed_call!`, which notes itself)
/// as it starts, after the call log took it.
fn record_call(e: &mut Engine, addr: u32, returning: bool) {
    if returning {
        return;
    }
    let Some((_, words)) = e.call_log.as_ref().and_then(|l| l.last()) else {
        return;
    };
    RECORDING.with(|r| {
        if let Some(recording) = &mut *r.borrow_mut() {
            let seen = seen_call(e, addr, words, recording);
            recording.calls.push(seen);
        }
    });
}

fn seen_call(e: &Engine, addr: u32, words: &[u32], recording: &mut Recording) -> Seen {
    let stack = e.guest_stack_region();
    let mut plain = words.to_vec();
    if recording.nan_payloads {
        canonical_nans(&mut plain);
    }
    let live = |word: u32| {
        e.stack_frames
            .iter()
            .any(|(base, size)| (*base..base + size).contains(&word))
    };
    recording.locals.retain(|(word, _)| live(*word));
    let loose = !recording.local_words.is_empty();
    let blur = if loose { stack } else { None };
    let mut seen_words = Vec::with_capacity(words.len());
    for (i, (canonical, word)) in plain.iter().zip(words).enumerate() {
        let on_stack = stack.is_some_and(|(lo, hi)| (lo..hi).contains(word));
        let local = recording
            .local_words
            .iter()
            .find(|l| l.callee == addr && l.word == i)
            .copied();
        seen_words.push(match (on_stack, local) {
            (true, Some(local)) => {
                let ordinal = match recording.locals.iter().find(|(w, _)| w == word) {
                    Some((_, ordinal)) => *ordinal,
                    None => {
                        recording.next_local += 1;
                        recording.locals.push((*word, recording.next_local));
                        recording.next_local
                    }
                };
                Word::Local {
                    ordinal,
                    bytes: bytes_hash(e, *word, local.size, blur),
                }
            }
            (true, None) => {
                let end = stack.map_or(0, |(_, hi)| hi);
                stack_word(e, *word, end, recording.stack_bytes, blur)
            }
            (false, _) => Word::Plain(*canonical),
        });
    }
    Seen {
        addr,
        words: seen_words,
        memory: memory_hash(e, stack, recording.nan_payloads, loose),
        clock: (e.clock_ms, e.clock_us),
        direct: e.typed_noted_direct(),
    }
}

/// A word on the stack, relative to the live frame that holds it; `blur` is the stack's range when
/// stack addresses behind it compare as "some stack address".
fn stack_word(
    e: &Engine,
    word: u32,
    stack_end: u32,
    stack_bytes: u32,
    blur: Option<(u32, u32)>,
) -> Word {
    let frame = e
        .stack_frames
        .iter()
        .rev()
        .find(|(base, size)| (*base..base + size).contains(&word));
    let (offset, frame, end) = match frame {
        Some((base, size)) => (word - base, *size, base + size),
        None => (stack_end - word, 0, stack_end),
    };
    let len = stack_bytes.min(end - word);
    Word::Stack {
        offset,
        frame,
        bytes: bytes_hash(e, word, len, blur),
    }
}

/// A hash of `len` bytes at `word`.
fn bytes_hash(e: &Engine, word: u32, len: u32, blur: Option<(u32, u32)>) -> u64 {
    let mut bytes = e.mem.bytes(word, len);
    if let Some(stack) = blur {
        blur_stack_addresses(&mut bytes, stack);
    }
    hash_bytes(0, &bytes)
}

/// What an aligned word holding an address on the stack compares as, when frames may differ.
const SOME_STACK_ADDRESS: u32 = 0xdead_57ac;

fn blur_stack_addresses(bytes: &mut [u8], (lo, hi): (u32, u32)) {
    for chunk in bytes.chunks_exact_mut(4) {
        let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        if (lo..hi).contains(&word) {
            chunk.copy_from_slice(&SOME_STACK_ADDRESS.to_le_bytes());
        }
    }
}

fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    let mut chunks = bytes.chunks_exact(8);
    for chunk in &mut chunks {
        let mut word = [0; 8];
        word.copy_from_slice(chunk);
        hash = (hash ^ u64::from_le_bytes(word))
            .wrapping_mul(0x0100_0000_01b3)
            .rotate_left(23);
    }
    for byte in chunks.remainder() {
        hash = (hash ^ u64::from(*byte))
            .wrapping_mul(0x0100_0000_01b3)
            .rotate_left(23);
    }
    hash
}

/// All mapped memory outside the stack for locals, as one hash.
fn memory_hash(e: &Engine, stack: Option<(u32, u32)>, nan_payloads: bool, loose: bool) -> u64 {
    let mut hash = 0;
    let on_stack = |a: u32| stack.is_some_and(|(lo, hi)| (lo..hi).contains(&a));
    for page in e.mem.mapped_pages() {
        if on_stack(page) && on_stack(page + 0xfff) {
            continue;
        }
        let bytes = comparable_page(page, &e.mem.page_bytes(page), stack, nan_payloads, loose);
        hash = hash_bytes(hash ^ u64::from(page), &bytes);
    }
    hash
}

/// A page as it compares: the stack's bytes cleared, and NaNs in one form when their payloads may
/// differ.
fn comparable_page(
    page: u32,
    bytes: &[u8],
    stack: Option<(u32, u32)>,
    nan_payloads: bool,
    loose: bool,
) -> Vec<u8> {
    let mut out = bytes.to_vec();
    if let Some((stack_lo, stack_hi)) = stack {
        let (lo, hi) = (stack_lo.max(page), stack_hi.min(page + 0x1000));
        if lo < hi {
            out[(lo - page) as usize..(hi - page) as usize].fill(0);
        }
        if loose {
            blur_stack_addresses(&mut out, (stack_lo, stack_hi));
        }
    }
    if nan_payloads {
        let mut words: Vec<u32> = out
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        canonical_nans(&mut words);
        for (chunk, word) in out.chunks_exact_mut(4).zip(words) {
            chunk.copy_from_slice(&word.to_le_bytes());
        }
    }
    out
}

/// Every word that reads as an `f32` NaN becomes one NaN; one that reads as the high word of an
/// `f64` NaN also clears the word below it. An `f64` infinity (high word 0x7ff00000, low word 0)
/// is left alone, so it still differs from a NaN.
fn canonical_nans(words: &mut [u32]) {
    for i in 0..words.len() {
        let word = words[i];
        let magnitude = word & 0x7fff_ffff;
        if magnitude <= 0x7f80_0000 {
            continue;
        }
        if magnitude == 0x7ff0_0000 && i > 0 && words[i - 1] == 0 {
            continue;
        }
        if magnitude >= 0x7ff0_0000 && i > 0 {
            words[i - 1] = 0;
        }
        words[i] = 0x7fc0_0000;
    }
}

/// What one engine did.
struct Side {
    engine: Engine,
    words: Vec<u32>,
    /// What the setup left at each address it may not replace.
    registered: Vec<(u32, Option<(usize, bool)>)>,
    ret: Ret,
    stop: Option<String>,
    calls: Vec<Seen>,
    stack: Option<(u32, u32)>,
    control_word: u16,
    clock: (u32, u32),
}

impl Side {
    /// (the top of the heap, the bytes it has left): the same on both unless one leaked or
    /// allocated more.
    fn heap(&self) -> (u32, u64) {
        (self.engine.mem.heap_top(), self.engine.mem.heap_available())
    }

    fn on_stack(&self, a: u32) -> bool {
        self.stack.is_some_and(|(lo, hi)| (lo..hi).contains(&a))
    }
}

/// Compares the two sides' calls, in order: the first that differs in address or words, and the
/// first with different memory or clock at the moment of the call, before that. `ignored` calls
/// come out of the first side only, the translation's, and `plain` calls out of the second, the typed
/// version's.
fn compare_calls(
    a: &Side,
    b: &Side,
    ignored: &[u32],
    plain: &[u32],
    found: &mut impl FnMut(Found),
) {
    let a_calls: Vec<&Seen> = a
        .calls
        .iter()
        .filter(|c| !ignored.contains(&c.addr))
        .collect();
    let b_calls: Vec<&Seen> = b
        .calls
        .iter()
        .filter(|c| !plain.contains(&c.addr))
        .collect();
    let same = |x: &Seen, y: &Seen| x.addr == y.addr && x.words == y.words;
    let at = a_calls
        .iter()
        .zip(&b_calls)
        .position(|(x, y)| !same(x, y))
        .unwrap_or(a_calls.len().min(b_calls.len()));
    let state = a_calls[..at]
        .iter()
        .zip(&b_calls[..at])
        .position(|(x, y)| x.memory != y.memory || x.clock != y.clock);
    if let Some(k) = state {
        found(Found::BeforeCall {
            at: k,
            addr: a_calls[k].addr,
            memory: a_calls[k].memory != b_calls[k].memory,
            clock: a_calls[k].clock != b_calls[k].clock,
        });
    }
    if at < a_calls.len() || at < b_calls.len() {
        let call = |c: Option<&&Seen>| c.map(|c| (c.addr, c.words.clone()));
        found(Found::Call {
            at,
            translation: call(a_calls.get(at)),
            typed: call(b_calls.get(at)),
        });
    }
}

/// Reports the bytes that differ between the two engines' memories, a run of them at a time,
/// outside the stack for locals, and the pages only one has.
fn compare_memory(
    a: &Side,
    b: &Side,
    nan_payloads: bool,
    loose: bool,
    found: &mut impl FnMut(Found),
) {
    let (mem_a, mem_b) = (&a.engine.mem, &b.engine.mem);
    let (pages_a, pages_b) = (mem_a.mapped_pages(), mem_b.mapped_pages());
    let (mut i, mut j) = (0, 0);
    while i < pages_a.len() || j < pages_b.len() {
        match (pages_a.get(i), pages_b.get(j)) {
            (Some(&pa), Some(&pb)) if pa == pb => {
                let (bytes_a, bytes_b) = (mem_a.page_bytes(pa), mem_b.page_bytes(pa));
                if bytes_a != bytes_b {
                    let bytes_a = comparable_page(pa, &bytes_a, a.stack, nan_payloads, loose);
                    let bytes_b = comparable_page(pa, &bytes_b, a.stack, nan_payloads, loose);
                    compare_page(pa, &bytes_a, &bytes_b, found);
                }
                i += 1;
                j += 1;
            }
            (Some(&pa), pb) if pb.map_or(true, |&pb| pa < pb) => {
                if !a.on_stack(pa) {
                    found(Found::Mapped {
                        page: pa,
                        by_typed: false,
                    });
                }
                i += 1;
            }
            (_, Some(&pb)) => {
                if !a.on_stack(pb) {
                    found(Found::Mapped {
                        page: pb,
                        by_typed: true,
                    });
                }
                j += 1;
            }
            _ => unreachable!("the loop's condition says one list has a page left"),
        }
    }
}

fn compare_page(page: u32, a: &[u8], b: &[u8], found: &mut impl FnMut(Found)) {
    let mut k = 0;
    while k < a.len() {
        let start = k;
        while k < a.len() && a[k] != b[k] {
            k += 1;
        }
        if k > start {
            found(Found::Memory {
                addr: page + start as u32,
                translation: a[start..k].to_vec(),
                typed: b[start..k].to_vec(),
            });
        } else {
            k += 1;
        }
    }
}

/// One thing two runs did differently. In a [`Branch::Direct`] difference, `translation` is the
/// typed version's by-address run and `typed` its direct run.
#[derive(Clone, Debug, PartialEq)]
pub enum Found {
    /// One stopped and the other didn't, or they stopped with different messages.
    Stop {
        translation: Option<String>,
        typed: Option<String>,
    },
    /// Both stopped alike where the test didn't expect it ([`Diff::expect_stops`]), or at an
    /// untranslated address: nothing after the stop was compared.
    StoppedAlike { message: String },
    /// [`Diff::expect_stops`] said this input stops, and neither side did.
    ExpectedStop,
    /// The return value, bit for bit.
    Return { translation: Ret, typed: Ret },
    /// The `at`th call (the call itself is 0): address and words.
    Call {
        at: usize,
        translation: Option<(u32, Vec<Word>)>,
        typed: Option<(u32, Vec<Word>)>,
    },
    /// At the start of the `at`th call, to `addr`, memory outside the stack or the clock differed:
    /// a store or a clock step moved across a call. The hash doesn't say where; [`Found::Memory`]
    /// does if the difference lasts to the end.
    BeforeCall {
        at: usize,
        addr: u32,
        memory: bool,
        clock: bool,
    },
    /// A run of bytes after the call.
    Memory {
        addr: u32,
        translation: Vec<u8>,
        typed: Vec<u8>,
    },
    /// A page only one side has mapped.
    Mapped { page: u32, by_typed: bool },
    /// The heap's top and free bytes after the call: one side kept or freed more.
    Heap {
        translation: (u32, u64),
        typed: (u32, u64),
    },
    /// The control word after the call.
    ControlWord { translation: u16, typed: u16 },
    /// The clock after the call, milliseconds and microseconds.
    Clock {
        translation: (u32, u32),
        typed: (u32, u32),
    },
}

/// A difference and the input that showed it.
#[derive(Clone, Debug, PartialEq)]
pub struct Difference {
    pub case: Case,
    pub control_word: u16,
    pub branch: Branch,
    pub what: Found,
}

impl fmt::Display for Difference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "case {:?}, control word {:04x}{}: ",
            self.case.0,
            self.control_word,
            if self.branch == Branch::Direct {
                ", direct branch against by address"
            } else {
                ""
            }
        )?;
        match &self.what {
            Found::Stop { translation, typed } => {
                write!(f, "stop: translation {translation:?}, typed {typed:?}")
            }
            Found::StoppedAlike { message } => write!(
                f,
                "both stopped, so nothing after it was compared: {message:?} (a test of the stop \
                 says `.expect_stops(|_| true)`; an open function needs a double)"
            ),
            Found::ExpectedStop => {
                write!(f, "the test says this input stops, and neither side did")
            }
            Found::Return { translation, typed } => write!(
                f,
                "return: translation eax {:08x} edx {:08x} st0 {:?} ({:016x}), \
                 typed eax {:08x} edx {:08x} st0 {:?} ({:016x})",
                translation.eax,
                translation.edx,
                translation.st0,
                translation.st0.to_bits(),
                typed.eax,
                typed.edx,
                typed.st0,
                typed.st0.to_bits()
            ),
            Found::Call {
                at,
                translation,
                typed,
            } => write!(
                f,
                "outgoing call {at}: translation {translation:08x?}, typed {typed:08x?}"
            ),
            Found::BeforeCall {
                at,
                addr,
                memory,
                clock,
            } => write!(
                f,
                "at the start of outgoing call {at} ({addr:08x}): {}{}differs",
                if *memory { "memory " } else { "" },
                if *clock { "clock " } else { "" }
            ),
            Found::Memory {
                addr,
                translation,
                typed,
            } => write!(
                f,
                "memory at {addr:08x}: translation {translation:02x?}, typed {typed:02x?}"
            ),
            Found::Mapped { page, by_typed } => write!(
                f,
                "page {page:08x} mapped only by the {}",
                if *by_typed {
                    "typed version"
                } else {
                    "translation"
                }
            ),
            Found::Heap { translation, typed } => write!(
                f,
                "heap (top, free): translation {translation:x?}, typed {typed:x?}"
            ),
            Found::ControlWord { translation, typed } => write!(
                f,
                "control word after: translation {translation:04x}, typed {typed:04x}"
            ),
            Found::Clock { translation, typed } => write!(
                f,
                "clock after: translation {translation:?}, typed {typed:?}"
            ),
        }
    }
}

/// What a [`Diff`] found.
#[must_use = "the report says whether they agreed: call `assert_same`"]
#[derive(Default, Debug)]
pub struct Report {
    /// The control words each case ran under: a family's row can narrow them, and this says so.
    pub control_words: Vec<u16>,
    /// Cases run, each control word counted.
    pub runs: usize,
    /// Of those, the runs where both sides stopped alike.
    pub stopped: usize,
    /// Of those, the runs whose direct branch ran too (none without debug assertions).
    pub direct_runs: usize,
    /// The `typed_call!`s that went direct on those runs. 0 for a function with one means no family
    /// named holds its address, and the direct branch has run nowhere.
    pub direct_calls: usize,
    /// The addresses of `typed_call!`s on those runs that no named family holds and
    /// [`Diff::goes_by_address`] doesn't list: the functions they name ran nowhere, so
    /// [`Report::assert_same`] fails.
    pub unnamed: Vec<u32>,
    /// Differences found.
    pub found: usize,
    /// The first of them in full.
    pub first: Vec<Difference>,
}

impl Report {
    fn note(&mut self, case: &Case, word: u16, branch: Branch, what: Found) {
        self.found += 1;
        if self.first.len() < KEEP_FOUND {
            self.first.push(Difference {
                case: case.clone(),
                control_word: word,
                branch,
                what,
            });
        }
    }

    /// Stops the test when nothing ran, anything differed, or a `typed_call!`'s function ran nowhere.
    #[track_caller]
    pub fn assert_same(&self) {
        assert!(self.runs > 0, "nothing ran: no case was generated");
        assert!(self.found == 0, "{self}");
        assert!(
            self.unnamed.is_empty(),
            "typed_call! at {:08x?} went by address: no family this test names holds it, so the \
             function it names ran nowhere. Name its family with `.family(..)`, or list the address \
             in `.goes_by_address(..)` when another test runs it",
            self.unnamed
        );
    }

    /// Whether something ran, nothing differed and every `typed_call!`'s function ran.
    pub fn is_same(&self) -> bool {
        self.runs > 0 && self.found == 0 && self.unnamed.is_empty()
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} difference(s) in {} run(s), {} of them stopped alike, under control words {:04x?}:",
            self.found, self.runs, self.stopped, self.control_words
        )?;
        for d in &self.first {
            writeln!(f, "  {d}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use crate::typed::FamilyId;

    const ADDR: u32 = 0x0040_2000;
    const CALLEE: u32 = 0x0040_2100;
    const OTHER_CALLEE: u32 = 0x0040_2110;

    // ---- The generator ------------------------------------------------------

    /// Every float class the plan names is in the lists.
    #[test]
    fn the_float_lists_hold_nan_zero_denormal_and_huge() {
        let f32s: Vec<f32> = F32_VALUES.iter().map(|b| f32::from_bits(*b)).collect();
        assert!(f32s.iter().any(|f| f.is_nan()));
        assert!(F32_VALUES.contains(&0));
        assert!(F32_VALUES.contains(&0x8000_0000));
        assert!(f32s.iter().any(|f| f.is_subnormal()));
        assert!(f32s.iter().any(|f| f.is_infinite()));
        assert!(F32_VALUES.contains(&f32::MAX.to_bits()));
        // A signalling NaN: quiet bit clear.
        assert!(f32s
            .iter()
            .any(|f| f.is_nan() && f.to_bits() & 0x0040_0000 == 0));
        let f64s: Vec<f64> = F64_VALUES.iter().map(|b| f64::from_bits(*b)).collect();
        assert!(f64s.iter().any(|f| f.is_nan()));
        assert!(F64_VALUES.contains(&0));
        assert!(f64s.iter().any(|f| f.is_subnormal()));
        assert!(F64_VALUES.contains(&f64::MAX.to_bits()));
        // Past the f32 range but finite in f64.
        assert!(f64s
            .iter()
            .any(|f| f.is_finite() && f.abs() > f64::from(f32::MAX)));
    }

    #[test]
    fn small_spaces_are_run_whole() {
        assert_eq!(cases(&[], 10), vec![Case(vec![])]);
        assert_eq!(cases(&[Input::Flag], 10).len(), 2);
        let both = cases(&[Input::Flag, Input::Of(&[7, 8, 9])], 10);
        assert_eq!(both.len(), 6);
        assert_eq!(both[0].0, [Value::Word(1), Value::Word(7)]);
        assert_eq!(both[5].0, [Value::Word(0), Value::Word(9)]);
        assert!(cases(&[Input::Of(&[])], 10).is_empty());
    }

    /// Past the cap every value of each argument still appears, and no case twice.
    #[test]
    fn large_spaces_sweep_each_argument_then_sample() {
        let inputs = [Input::F32, Input::Word, Input::F64];
        let list = cases(&inputs, 200);
        assert_eq!(list.len(), 200);
        let unique: HashSet<&Case> = list.iter().collect();
        assert_eq!(unique.len(), 200);
        for b in F32_VALUES {
            assert!(list.iter().any(|c| c.0[0] == Value::F32(*b)), "{b:08x}");
        }
        for b in F64_VALUES {
            assert!(list.iter().any(|c| c.0[2] == Value::F64(*b)), "{b:016x}");
        }
        // The same sequence each time, so a failure repeats.
        assert_eq!(list, cases(&inputs, 200));
    }

    #[test]
    fn case_words_pass_doubles_low_word_first() {
        let c = Case(vec![
            Value::Word(5),
            Value::F32(0x7f80_0001),
            Value::F64(0x1122_3344_5566_7788),
        ]);
        assert_eq!(c.words(), [5, 0x7f80_0001, 0x5566_7788, 0x1122_3344]);
        // A signalling NaN arrives with its bits.
        assert_eq!(c.f32(1).to_bits(), 0x7f80_0001);
    }

    // ---- Things that are the same pass -----------------------------------------

    fn truncate(_: &mut Engine, x: f32) -> i32 {
        crate::x87::fistp_chop(f64::from(x))
    }

    fn truncate_again(_: &mut Engine, x: f32) -> i32 {
        crate::x87::fistp_chop(f64::from(x))
    }

    #[test]
    fn the_same_behaviour_passes_every_input() {
        let report = Diff::new(crate::entry!(0x0040_2000, truncate_again(f32) -> i32))
            .translation(crate::entry!(0x0040_2000, truncate(f32) -> i32).1)
            .inputs(&[Input::F32])
            .run();
        report.assert_same();
        assert!(report.is_same());
        // Every value, under both control words.
        assert_eq!(report.runs, 2 * F32_VALUES.len());
    }

    /// A translation's `with_stack` local may become a native one: the stack for locals isn't
    /// compared, nor does a local move a later heap block, and an outgoing word into a local compares
    /// by its place in its frame, so a typed version with fewer frames passes.
    #[test]
    fn a_local_may_leave_the_guest_stack() {
        fn more_frames(e: &mut Engine, a: u32) -> u32 {
            e.with_stack(0x40, |e, scratch| {
                e.mem.set_u32(scratch.addr(), a);
                let block = e.mem.alloc(4);
                let value = e.mem.u32(scratch.addr()).wrapping_add(1);
                e.mem.set_u32(block, value);
                e.with_stack(16, |e, out| {
                    e.mem.set_u32(out.addr(), a);
                    e.call(CALLEE, &[out.addr() + 4]);
                });
                block
            })
        }
        fn fewer_frames(e: &mut Engine, a: u32) -> u32 {
            let block = e.mem.alloc(4);
            e.mem.set_u32(block, a.wrapping_add(1));
            e.with_stack(16, |e, out| {
                e.mem.set_u32(out.addr(), a);
                e.call(CALLEE, &[out.addr() + 4]);
            });
            block
        }
        let report = Diff::new(crate::entry!(0x0040_2000, fewer_frames(u32) -> u32))
            .translation(crate::entry!(0x0040_2000, more_frames(u32) -> u32).1)
            .inputs(&[Input::Word])
            .setup(|e, case| {
                e.register(CALLEE, |_, _| Ret::default());
                case.words()
            })
            .run();
        report.assert_same();
        assert!(report.is_same());
    }

    /// A callee the typed version inlines is ignored once it says so.
    #[test]
    fn ignored_callees_are_not_compared() {
        fn calling(e: &mut Engine, a: u32) -> u32 {
            e.call(CALLEE, &[a]).u32() + 1
        }
        fn inlined(_: &mut Engine, a: u32) -> u32 {
            a * 2 + 1
        }
        let diff = |ignore: &[u32]| {
            Diff::new(crate::entry!(0x0040_2000, inlined(u32) -> u32))
                .translation(crate::entry!(0x0040_2000, calling(u32) -> u32).1)
                .inputs(&[Input::Of(&[1, 2])])
                .setup(|e, case| {
                    e.register(CALLEE, |_, a| Ret {
                        eax: a[0] * 2,
                        ..Ret::default()
                    });
                    case.words()
                })
                .ignore_calls(ignore)
                .run()
        };
        assert!(diff(&[CALLEE]).is_same());
        assert!(!diff(&[]).is_same());
    }

    // ---- The stand-ins that are deliberately wrong: the harness catches each --------

    /// The class the plan expects (compare-1106's float-as-int): a typed version with Rust's
    /// saturating cast where the exe's `FISTP` gives the integer indefinite. It agrees on ordinary
    /// values and differs on NaN and past the int range, and the harness names those inputs.
    #[test]
    fn a_wrong_float_to_int_is_caught_on_nan_and_huge_only() {
        fn wrong_cast(_: &mut Engine, x: f32) -> i32 {
            x as i32
        }
        let report = Diff::new(crate::entry!(0x0040_2000, wrong_cast(f32) -> i32))
            .translation(crate::entry!(0x0040_2000, truncate(f32) -> i32).1)
            .inputs(&[Input::F32])
            .control_words(&[OTHER_THREADS])
            .max_cases(1000)
            .run();
        assert!(!report.is_same());
        assert!(report.first.len() <= KEEP_FOUND);
        for d in &report.first {
            assert!(matches!(d.what, Found::Return { .. }), "{d}");
            // Never on a value inside the int range.
            let x = d.case.f32(0);
            assert!(
                x.is_nan() || f64::from(x).abs() >= 2_147_483_648.0,
                "not an edge: {d}"
            );
        }
        let flagged = |bits: u32| report.first.iter().any(|d| d.case.0[0] == Value::F32(bits));
        assert!(flagged(0x7fc0_0000), "a quiet NaN:\n{report}");
        assert!(flagged(0x7f80_0001), "a signalling NaN:\n{report}");
        assert!(flagged(0x4f00_0000), "2^31:\n{report}");
        assert!(flagged(0x7f80_0000), "infinity:\n{report}");
        // -2^31 fits: both give it.
        assert!(!flagged(0xcf00_0000));
        // And the message names the input and the two answers.
        let text = report.to_string();
        assert!(text.contains("return: translation eax 80000000"), "{text}");
        assert!(text.contains("typed eax 00000000"), "{text}");
    }

    /// A wrong float result by bits: a NaN's sign, or a flushed denormal, compare unequal though
    /// `==` on values would call NaNs different and -0 equal to 0.
    #[test]
    fn a_wrong_float_return_is_caught_by_bits() {
        fn exact(_: &mut Engine, x: f64) -> f64 {
            x
        }
        fn flushes(_: &mut Engine, x: f64) -> f64 {
            if x.is_subnormal() || x == 0.0 {
                0.0
            } else {
                x
            }
        }
        let report = Diff::new(crate::entry!(0x0040_2000, flushes(f64) -> f64))
            .translation(crate::entry!(0x0040_2000, exact(f64) -> f64).1)
            .inputs(&[Input::F64])
            .control_words(&[OTHER_THREADS])
            .run();
        let cases: Vec<u64> = report
            .first
            .iter()
            .map(|d| d.case.f64(0).to_bits())
            .collect();
        // -0 and both denormals, and no NaN is called different from itself.
        assert_eq!(
            cases,
            [
                0x8000_0000_0000_0000,
                0x0000_0000_0000_0001,
                0x000f_ffff_ffff_ffff
            ],
            "{report}"
        );
    }

    /// x87 rounding: a typed version without the helper agrees at 53 bits and differs at the
    /// main thread's 24, so each control word has to run.
    #[test]
    fn a_missing_precision_step_shows_under_the_main_threads_control_word() {
        fn with_helper(_: &mut Engine, a: f64, b: f64) -> f64 {
            x87_add(a, b)
        }
        fn plain_add(_: &mut Engine, a: f64, b: f64) -> f64 {
            a + b
        }
        let report = Diff::new(crate::entry!(0x0040_2000, plain_add(f64, f64) -> f64))
            .translation(crate::entry!(0x0040_2000, with_helper(f64, f64) -> f64).1)
            .inputs(&[Input::F64, Input::Of(&[])])
            .run();
        assert_eq!(report.runs, 0, "an empty input list has no cases");
        let report = Diff::new(crate::entry!(0x0040_2000, plain_add(f64, f64) -> f64))
            .translation(crate::entry!(0x0040_2000, with_helper(f64, f64) -> f64).1)
            .inputs(&[Input::F64, Input::F64])
            .max_cases(300)
            .run();
        assert!(report.found > 0);
        assert!(
            report.first.iter().any(|d| d.control_word == MAIN_THREAD),
            "{report}"
        );
        assert!(
            report.first.iter().all(|d| d.control_word == MAIN_THREAD),
            "53 bits needs no helper: {report}"
        );
    }

    /// Memory written: a stand-in that stores the same value at the wrong offset, one that skips a
    /// store on one input, and one that writes a word too many.
    #[test]
    fn wrong_memory_is_caught() {
        fn stores(e: &mut Engine, p: Ptr, v: u32) {
            e.mem.set_u32(p.addr() + 8, v);
        }
        fn wrong_offset(e: &mut Engine, p: Ptr, v: u32) {
            e.mem.set_u32(p.addr() + 12, v);
        }
        fn skips_zero(e: &mut Engine, p: Ptr, v: u32) {
            if v != 0 {
                e.mem.set_u32(p.addr() + 8, v);
            }
        }
        fn one_too_many(e: &mut Engine, p: Ptr, v: u32) {
            e.mem.set_u32(p.addr() + 8, v);
            e.mem.set_u32(p.addr() + 4, 1);
        }
        let translation = crate::entry!(0x0040_2000, stores(Ptr, u32)).1;
        let run = |typed: (u32, AbiFn)| {
            Diff::new(typed)
                .translation(translation)
                .inputs(&[Input::Of(&[0, 0x1234])])
                .control_words(&[OTHER_THREADS])
                .setup(|e, case| {
                    let block = e.mem.alloc(0x20);
                    e.mem.set_u32(block + 8, 0xaaaa_aaaa);
                    vec![block, case.word(0)]
                })
                .run()
        };

        let report = run(crate::entry!(0x0040_2000, wrong_offset(Ptr, u32)));
        assert_eq!(report.found, 2, "{report}");
        // Storing 0: the old word stays where the translation cleared it.
        let Found::Memory {
            ref translation,
            ref typed,
            ..
        } = report.first[0].what
        else {
            panic!("{report}");
        };
        assert_eq!(
            (&translation[..], &typed[..]),
            (&[0; 4][..], &[0xaa; 4][..])
        );
        // Storing 0x1234: one run from the word the translation wrote into the next one.
        let Found::Memory {
            ref translation,
            ref typed,
            ..
        } = report.first[1].what
        else {
            panic!("{report}");
        };
        assert_eq!(translation[..], [0x34, 0x12, 0, 0, 0, 0]);
        assert_eq!(typed[..], [0xaa, 0xaa, 0xaa, 0xaa, 0x34, 0x12]);

        let report = run(crate::entry!(0x0040_2000, skips_zero(Ptr, u32)));
        assert_eq!(report.found, 1, "{report}");
        assert_eq!(report.first[0].case.word(0), 0);
        assert!(matches!(report.first[0].what, Found::Memory { .. }));

        let report = run(crate::entry!(0x0040_2000, one_too_many(Ptr, u32)));
        assert_eq!(report.found, 2, "{report}");
    }

    /// Outgoing calls: the wrong word, the wrong order, one missing, one extra.
    #[test]
    fn wrong_outgoing_calls_are_caught() {
        fn two_calls(e: &mut Engine, a: u32) {
            e.call(CALLEE, &[a]);
            e.call(OTHER_CALLEE, &[a + 1]);
        }
        fn wrong_word(e: &mut Engine, a: u32) {
            e.call(CALLEE, &[a]);
            e.call(OTHER_CALLEE, &[a + 2]);
        }
        fn wrong_order(e: &mut Engine, a: u32) {
            e.call(OTHER_CALLEE, &[a + 1]);
            e.call(CALLEE, &[a]);
        }
        fn missing(e: &mut Engine, a: u32) {
            e.call(CALLEE, &[a]);
        }
        fn extra(e: &mut Engine, a: u32) {
            e.call(CALLEE, &[a]);
            e.call(OTHER_CALLEE, &[a + 1]);
            e.call(CALLEE, &[a]);
        }
        let translation = crate::entry!(0x0040_2000, two_calls(u32)).1;
        let run = |typed: (u32, AbiFn)| {
            let report = Diff::new(typed)
                .translation(translation)
                .inputs(&[Input::Of(&[5])])
                .control_words(&[OTHER_THREADS])
                .setup(|e, case| {
                    e.register(CALLEE, |_, _| Ret::default());
                    e.register(OTHER_CALLEE, |_, _| Ret::default());
                    case.words()
                })
                .run();
            assert_eq!(report.found, 1, "{report}");
            report.first[0].what.clone()
        };
        assert_eq!(
            run(crate::entry!(0x0040_2000, wrong_word(u32))),
            Found::Call {
                at: 2,
                translation: Some((OTHER_CALLEE, vec![Word::Plain(6)])),
                typed: Some((OTHER_CALLEE, vec![Word::Plain(7)])),
            }
        );
        assert_eq!(
            run(crate::entry!(0x0040_2000, wrong_order(u32))),
            Found::Call {
                at: 1,
                translation: Some((CALLEE, vec![Word::Plain(5)])),
                typed: Some((OTHER_CALLEE, vec![Word::Plain(6)])),
            }
        );
        assert_eq!(
            run(crate::entry!(0x0040_2000, missing(u32))),
            Found::Call {
                at: 2,
                translation: Some((OTHER_CALLEE, vec![Word::Plain(6)])),
                typed: None,
            }
        );
        assert_eq!(
            run(crate::entry!(0x0040_2000, extra(u32))),
            Found::Call {
                at: 3,
                translation: None,
                typed: Some((CALLEE, vec![Word::Plain(5)])),
            }
        );
    }

    /// A stop is behaviour: the translation stops on a bad pointer where a stand-in tests it first.
    #[test]
    fn a_missing_stop_is_caught() {
        fn tests_first(e: &mut Engine, p: u32) -> u32 {
            if e.mem.is_mapped(p) {
                e.mem.u32(p)
            } else {
                0
            }
        }
        let report = Diff::new(crate::entry!(0x0040_2000, tests_first(u32) -> u32))
            .translation(crate::entry!(0x0040_2000, reads(u32) -> u32).1)
            .inputs(&[Input::Of(&[0x0300_0000])])
            .control_words(&[OTHER_THREADS])
            .run();
        assert_eq!(report.found, 1, "{report}");
        assert_eq!(
            report.first[0].what,
            Found::Stop {
                translation: Some("read of unmapped memory at 03000000".to_string()),
                typed: None,
            }
        );
        // Both stopping alike is the same behaviour, when the test says that is its point.
        Diff::new(crate::entry!(0x0040_2000, reads(u32) -> u32))
            .translation(crate::entry!(0x0040_2000, reads(u32) -> u32).1)
            .inputs(&[Input::Of(&[0x0300_0000])])
            .expect_stops(|_| true)
            .run()
            .assert_same();
    }

    /// A block the translation frees and a stand-in keeps.
    #[test]
    fn a_leak_is_caught() {
        fn frees(e: &mut Engine) {
            let block = e.mem.alloc(0x100);
            e.mem.free(block);
        }
        fn leaks(e: &mut Engine) {
            e.mem.alloc(0x100);
        }
        let report = Diff::new(crate::entry!(0x0040_2000, leaks()))
            .translation(crate::entry!(0x0040_2000, frees()).1)
            .control_words(&[OTHER_THREADS])
            .run();
        assert!(
            report
                .first
                .iter()
                .any(|d| matches!(d.what, Found::Heap { .. })),
            "{report}"
        );
    }

    /// A control word left changed.
    #[test]
    fn a_changed_control_word_is_caught() {
        fn leaves_it(_: &mut Engine) {}
        fn changes_it(_: &mut Engine) {
            crate::x87::set_control_word(MAIN_THREAD);
        }
        let report = Diff::new(crate::entry!(0x0040_2000, changes_it()))
            .translation(crate::entry!(0x0040_2000, leaves_it()).1)
            .control_words(&[OTHER_THREADS])
            .run();
        assert_eq!(report.found, 1, "{report}");
        assert_eq!(
            report.first[0].what,
            Found::ControlWord {
                translation: OTHER_THREADS,
                typed: MAIN_THREAD,
            }
        );
    }

    #[test]
    #[should_panic(expected = "different call words")]
    fn a_setup_that_differs_between_engines_is_refused() {
        use std::cell::Cell;
        use std::rc::Rc;
        let n = Rc::new(Cell::new(0));
        let shared = n.clone();
        let _ = Diff::new((ADDR, |_, _| Ret::default()))
            .translation(|_, _| Ret::default())
            .inputs(&[Input::Of(&[1])])
            .setup(move |_, _| {
                shared.set(shared.get() + 1);
                vec![shared.get()]
            })
            .run();
    }

    /// A report says what it found, and doesn't grow without bound.
    #[test]
    fn a_report_keeps_the_first_differences() {
        fn one(_: &mut Engine, _: u32) -> u32 {
            1
        }
        fn two(_: &mut Engine, _: u32) -> u32 {
            2
        }
        let report = Diff::new(crate::entry!(0x0040_2000, two(u32) -> u32))
            .translation(crate::entry!(0x0040_2000, one(u32) -> u32).1)
            .inputs(&[Input::Word])
            .max_cases(1000)
            .control_words(&[OTHER_THREADS, MAIN_THREAD])
            .run();
        assert_eq!(report.runs, 2 * WORD_VALUES.len());
        assert_eq!(report.found, report.runs);
        assert_eq!(report.first.len(), KEEP_FOUND);
        let text = report.to_string();
        assert!(
            text.starts_with("18 difference(s) in 18 run(s), 0 of them stopped alike, under control words [027f, 007f]:\n"),
            "{text}"
        );
        assert!(
            text.contains("case [0x1], control word 027f: return"),
            "{text}"
        );
    }

    // ---- A comparison that compared nothing fails ------------------------------------

    /// No case generated: a green test here would have checked nothing.
    #[test]
    #[should_panic(expected = "nothing ran")]
    fn assert_same_fails_when_nothing_ran() {
        fn f(_: &mut Engine, _: u32) {}
        let report = Diff::new(crate::entry!(0x0040_2000, f(u32)))
            .translation(crate::entry!(0x0040_2000, f(u32)).1)
            .inputs(&[Input::Of(&[])])
            .run();
        assert_eq!(report.runs, 0);
        report.assert_same();
    }

    fn reads(e: &mut Engine, p: u32) -> u32 {
        e.mem.u32(p)
    }

    /// Every run stopping alike is no comparison: a bad pointer in the setup gives this.
    #[test]
    fn both_stopping_alike_fails_unless_the_test_expects_it() {
        let diff = || {
            Diff::new(crate::entry!(0x0040_2000, reads(u32) -> u32))
                .translation(crate::entry!(0x0040_2000, reads(u32) -> u32).1)
                .inputs(&[Input::Of(&[0x0300_0000])])
                .control_words(&[OTHER_THREADS])
        };
        let report = diff().run();
        assert!(!report.is_same());
        assert_eq!(report.stopped, 1);
        assert_eq!(
            report.first[0].what,
            Found::StoppedAlike {
                message: "read of unmapped memory at 03000000".to_string()
            }
        );
        let report = diff().expect_stops(|_| true).run();
        assert!(report.is_same(), "{report}");
        assert_eq!(report.stopped, 1);
    }

    /// One unexpected stop among good runs fails too.
    #[test]
    fn one_stop_among_good_runs_fails() {
        let report = Diff::new(crate::entry!(0x0040_2000, reads(u32) -> u32))
            .translation(crate::entry!(0x0040_2000, reads(u32) -> u32).1)
            .inputs(&[Input::Of(&[0x0200_0000, 0x0300_0000])])
            .control_words(&[OTHER_THREADS])
            .setup(|e, case| {
                e.map(0x0200_0000, 4);
                case.words()
            })
            .run();
        assert_eq!((report.runs, report.stopped, report.found), (2, 1, 1));
    }

    /// A callee with no double stops both sides at its address, and no flag excuses it.
    #[test]
    fn an_untranslated_callee_fails_even_when_stops_are_expected() {
        fn calls(e: &mut Engine) {
            e.call(CALLEE, &[]);
        }
        let report = Diff::new(crate::entry!(0x0040_2000, calls()))
            .translation(crate::entry!(0x0040_2000, calls()).1)
            .expect_stops(|_| true)
            .control_words(&[OTHER_THREADS])
            .run();
        assert!(!report.is_same(), "{report}");
        assert!(matches!(report.first[0].what, Found::StoppedAlike { .. }));
    }

    // ---- The direct branch runs ---------------------------------------------------------

    const FAMILY_ID: FamilyId = FamilyId::new(11);
    const OTHER_FAMILY_ID: FamilyId = FamilyId::new(12);
    const MEMBER: u32 = 0x0040_2200;

    fn member(_: &mut Engine, x: u32) -> u32 {
        x + 1
    }
    fn member_translated(_: &mut Engine, x: u32) -> u32 {
        x + 1
    }
    fn not_the_member(_: &mut Engine, x: u32) -> u32 {
        x + 2
    }

    static FAMILY: Family = Family {
        name: "diff_test_family",
        id: FAMILY_ID,
        past_gate: false,
        threads: Threads::Both,
        funcs: || vec![crate::entry!(0x0040_2200, member(u32) -> u32)],
    };

    /// The translation of the caller, calling the member by address.
    fn caller_translated(e: &mut Engine, x: u32) -> u32 {
        e.call(MEMBER, &[x]).u32() + 1
    }

    fn diff_of_caller(typed: (u32, AbiFn)) -> Diff {
        Diff::new(typed)
            .translation(crate::entry!(0x0040_2000, caller_translated(u32) -> u32).1)
            .inputs(&[Input::Of(&[5])])
            .control_words(&[OTHER_THREADS])
            .setup(|e, case| {
                let (addr, f) = crate::entry!(0x0040_2200, member_translated(u32) -> u32);
                e.register(addr, f);
                case.words()
            })
    }

    /// With the right function, family and address, all three runs agree, and the direct run ran.
    #[test]
    fn the_direct_branch_runs_and_agrees() {
        fn right(e: &mut Engine, x: u32) -> u32 {
            let got: u32 = crate::typed_call!(FAMILY_ID, 0x0040_2200, member(e, x));
            got + 1
        }
        let report = diff_of_caller(crate::entry!(0x0040_2000, right(u32) -> u32))
            .family(&FAMILY)
            .run();
        report.assert_same();
        assert_eq!(
            report.direct_runs,
            if cfg!(debug_assertions) {
                report.runs
            } else {
                0
            }
        );
    }

    /// A `typed_call!` naming the wrong function: by address the registered function runs and all
    /// looks right; only the direct branch runs the wrong one.
    #[test]
    fn a_wrong_function_in_typed_call_is_caught_on_the_direct_branch() {
        fn wrong(e: &mut Engine, x: u32) -> u32 {
            let got: u32 = crate::typed_call!(FAMILY_ID, 0x0040_2200, not_the_member(e, x));
            got + 1
        }
        let report = diff_of_caller(crate::entry!(0x0040_2000, wrong(u32) -> u32))
            .family(&FAMILY)
            .run();
        if !cfg!(debug_assertions) {
            // No direct run without debug assertions, and by address it is right.
            assert_eq!(report.direct_runs, 0);
            return;
        }
        assert!(!report.is_same());
        assert!(
            report.first.iter().all(|d| d.branch == Branch::Direct),
            "by address is right: {report}"
        );
        assert!(matches!(report.first[0].what, Found::Return { .. }));
    }

    /// A wrong word on the direct branch only can't happen by construction, but a direct run
    /// that does something else in memory shows.
    #[test]
    fn a_typed_call_to_a_member_that_differs_is_caught_against_the_translation() {
        fn wrong_member(_: &mut Engine, x: u32) -> u32 {
            x + 3
        }
        static WRONG: Family = Family {
            name: "diff_test_wrong",
            id: FAMILY_ID,
            past_gate: false,
            threads: Threads::Both,
            funcs: || vec![crate::entry!(0x0040_2200, wrong_member(u32) -> u32)],
        };
        fn right(e: &mut Engine, x: u32) -> u32 {
            let got: u32 = crate::typed_call!(FAMILY_ID, 0x0040_2200, wrong_member(e, x));
            got + 1
        }
        let report = diff_of_caller(crate::entry!(0x0040_2000, right(u32) -> u32))
            .family(&WRONG)
            .run();
        assert!(!report.is_same());
        assert_eq!(report.first[0].branch, Branch::ByAddress, "{report}");
    }

    /// A `typed_call!` naming a family that doesn't hold the address: an override would clear the
    /// holder and this call would go past it. The direct run stops.
    #[test]
    fn a_family_that_does_not_hold_the_address_stops_the_direct_run() {
        fn other_family(e: &mut Engine, x: u32) -> u32 {
            let got: u32 = crate::typed_call!(OTHER_FAMILY_ID, 0x0040_2200, member(e, x));
            got + 1
        }
        let report = diff_of_caller(crate::entry!(0x0040_2000, other_family(u32) -> u32))
            .family(&FAMILY)
            .run();
        if !cfg!(debug_assertions) {
            return;
        }
        assert!(!report.is_same());
        let Found::Stop {
            typed: Some(why), ..
        } = &report.first[0].what
        else {
            panic!("{report}");
        };
        assert!(why.contains("names FamilyId(12)"), "{why}");
        assert_eq!(report.first[0].branch, Branch::Direct);
    }

    /// A function with a `typed_call!` and a test that names no family: everything goes by address,
    /// as in play with that family off, so the function the `typed_call!` names ran nowhere. That fails
    /// unless the test says it means it.
    #[test]
    fn a_typed_call_with_no_family_named_goes_by_address() {
        fn right(e: &mut Engine, x: u32) -> u32 {
            let got: u32 = crate::typed_call!(FAMILY_ID, 0x0040_2200, member(e, x));
            got + 1
        }
        let diff = diff_of_caller(crate::entry!(0x0040_2000, right(u32) -> u32));
        let report = diff.run();
        assert_eq!(report.found, 0, "{report}");
        assert_eq!(report.direct_calls, 0);
        if cfg!(debug_assertions) {
            assert_eq!(report.unnamed, [0x0040_2200]);
            assert!(!report.is_same());
            let stopped = catch_unwind(AssertUnwindSafe(|| report.assert_same()));
            assert!(stopped.is_err());
        }
        let report = diff.goes_by_address(&[0x0040_2200]).run();
        report.assert_same();
        assert_eq!(report.direct_calls, 0);
        assert!(report.is_same());
    }

    /// The hole this closes: the wrong function inside a `typed_call!` whose family the test doesn't
    /// name runs nowhere and compares equal, so only the report's `unnamed` catches it.
    #[test]
    fn the_wrong_function_in_an_unnamed_family_does_not_pass() {
        if !cfg!(debug_assertions) {
            return;
        }
        fn not_the_member(_: &mut Engine, _: u32) -> u32 {
            0xdead
        }
        fn wrong(e: &mut Engine, x: u32) -> u32 {
            let got: u32 = crate::typed_call!(FAMILY_ID, 0x0040_2200, not_the_member(e, x));
            got + 1
        }
        let report = diff_of_caller(crate::entry!(0x0040_2000, wrong(u32) -> u32)).run();
        assert_eq!(report.found, 0, "{report}");
        assert!(!report.is_same());
        assert!(catch_unwind(AssertUnwindSafe(|| report.assert_same())).is_err());
    }

    /// A setup that registers at a watched address where nothing was, on one side only: a double on
    /// the typed side alone, which no comparison sees. The same function on every side is fine.
    #[test]
    fn a_setup_registering_a_new_callee_on_one_side_stops() {
        fn plain_translation(e: &mut Engine, x: u32) -> u32 {
            let _ = e;
            x
        }
        fn calls_it(e: &mut Engine, x: u32) -> u32 {
            e.call(CALLEE, &[x]).u32()
        }
        let run = |one_side: bool| {
            let seen = std::cell::Cell::new(0);
            let diff = Diff::new(crate::entry!(0x0040_2000, calls_it(u32) -> u32))
                .translation(crate::entry!(0x0040_2000, plain_translation(u32) -> u32).1)
                .translation_calls_plainly(&[CALLEE])
                .inputs(&[Input::Of(&[3])])
                .control_words(&[OTHER_THREADS])
                .setup(move |e, case| {
                    seen.set(seen.get() + 1);
                    if !one_side || seen.get() > 1 {
                        e.register(CALLEE, |_, a| Ret {
                            eax: a[0],
                            ..Ret::default()
                        });
                    }
                    case.words()
                });
            catch_unwind(AssertUnwindSafe(|| diff.run()))
        };
        let on_all = run(false).expect("the same function on every side");
        on_all.assert_same();
        assert!(run(true).is_err());
    }

    // ---- Stack words, memory at each call, the clock ----------------------------------

    /// Hands the callee a word into a 16-byte local holding `a`.
    fn passes_local(e: &mut Engine, a: u32) {
        e.with_stack(16, |e, local| {
            e.mem.set_u32(local.addr(), a);
            e.call(CALLEE, &[local.addr()]);
        });
    }

    fn with_callee_double(typed: (u32, AbiFn)) -> Report {
        Diff::new(typed)
            .translation(crate::entry!(0x0040_2000, passes_local(u32)).1)
            .inputs(&[Input::Of(&[7])])
            .control_words(&[OTHER_THREADS])
            .setup(|e, case| {
                e.register(CALLEE, |_, _| Ret::default());
                case.words()
            })
            .run()
    }

    /// `local + 8` is not `local`, though both are stack addresses.
    #[test]
    fn a_stack_word_at_the_wrong_offset_is_caught() {
        fn eight_on(e: &mut Engine, a: u32) {
            e.with_stack(16, |e, local| {
                e.mem.set_u32(local.addr(), a);
                e.call(CALLEE, &[local.addr() + 8]);
            });
        }
        let report = with_callee_double(crate::entry!(0x0040_2000, eight_on(u32)));
        assert_eq!(report.found, 1, "{report}");
        let Found::Call {
            at: 1,
            translation: Some((_, translation)),
            typed: Some((_, typed)),
        } = &report.first[0].what
        else {
            panic!("{report}");
        };
        assert!(matches!(
            translation[0],
            Word::Stack {
                offset: 0,
                frame: 16,
                ..
            }
        ));
        assert!(matches!(
            typed[0],
            Word::Stack {
                offset: 8,
                frame: 16,
                ..
            }
        ));
    }

    /// The right word, with the wrong contents behind it.
    #[test]
    fn a_local_with_the_wrong_contents_is_caught() {
        fn wrong_contents(e: &mut Engine, a: u32) {
            e.with_stack(16, |e, local| {
                e.mem.set_u32(local.addr(), a + 1);
                e.call(CALLEE, &[local.addr()]);
            });
        }
        let report = with_callee_double(crate::entry!(0x0040_2000, wrong_contents(u32)));
        assert_eq!(report.found, 1, "{report}");
        assert!(matches!(report.first[0].what, Found::Call { at: 1, .. }));
    }

    /// A local the wrong size is a different local.
    #[test]
    fn a_local_of_another_size_is_caught() {
        fn smaller(e: &mut Engine, a: u32) {
            e.with_stack(8, |e, local| {
                e.mem.set_u32(local.addr(), a);
                e.call(CALLEE, &[local.addr()]);
            });
        }
        let report = with_callee_double(crate::entry!(0x0040_2000, smaller(u32)));
        assert_eq!(report.found, 1, "{report}");
    }

    /// The same result by the end, but the callee would have seen another memory.
    #[test]
    fn a_store_moved_across_a_call_is_caught() {
        fn store_then_call(e: &mut Engine, p: Ptr) {
            e.mem.set_u32(p.addr(), 1);
            e.call(CALLEE, &[]);
        }
        fn call_then_store(e: &mut Engine, p: Ptr) {
            e.call(CALLEE, &[]);
            e.mem.set_u32(p.addr(), 1);
        }
        let report = Diff::new(crate::entry!(0x0040_2000, call_then_store(Ptr)))
            .translation(crate::entry!(0x0040_2000, store_then_call(Ptr)).1)
            .inputs(&[Input::Flag])
            .control_words(&[OTHER_THREADS])
            .setup(|e, _| {
                e.register(CALLEE, |_, _| Ret::default());
                vec![e.mem.alloc(8)]
            })
            .run();
        assert_eq!(report.found, 2, "two inputs, one difference each: {report}");
        assert_eq!(
            report.first[0].what,
            Found::BeforeCall {
                at: 1,
                addr: CALLEE,
                memory: true,
                clock: false
            }
        );
    }

    /// A clock step moved or dropped.
    #[test]
    fn the_clock_is_compared() {
        fn steps(e: &mut Engine) {
            e.advance_clock_us(1500);
        }
        fn does_not(_: &mut Engine) {}
        let report = Diff::new(crate::entry!(0x0040_2000, does_not()))
            .translation(crate::entry!(0x0040_2000, steps()).1)
            .control_words(&[OTHER_THREADS])
            .run();
        assert_eq!(report.found, 1, "{report}");
        assert!(matches!(report.first[0].what, Found::Clock { .. }));
    }

    /// An ignored callee the typed version still calls shows, with its words.
    #[test]
    fn ignore_calls_does_not_hide_a_callee_still_called() {
        fn calling(e: &mut Engine, a: u32) {
            e.call(CALLEE, &[a]);
        }
        fn wrong_word(e: &mut Engine, a: u32) {
            e.call(CALLEE, &[a + 1]);
        }
        let report = Diff::new(crate::entry!(0x0040_2000, wrong_word(u32)))
            .translation(crate::entry!(0x0040_2000, calling(u32)).1)
            .inputs(&[Input::Of(&[1])])
            .control_words(&[OTHER_THREADS])
            .setup(|e, case| {
                e.register(CALLEE, |_, _| Ret::default());
                case.words()
            })
            .ignore_calls(&[CALLEE])
            .run();
        assert_eq!(report.found, 1, "{report}");
        assert!(matches!(report.first[0].what, Found::Call { .. }));
    }

    #[test]
    #[should_panic(expected = "00aa4ee0 is overridable")]
    fn ignore_calls_refuses_an_overridable_address() {
        fn f(_: &mut Engine) {}
        let _ = Diff::new(crate::entry!(0x0040_2000, f())).ignore_calls(&[0x00aa_4ee0]);
    }

    // ---- The rest of what is compared -----------------------------------------------------

    /// A page only one side maps, either way round.
    #[test]
    fn a_page_only_one_side_maps_is_caught() {
        fn maps(e: &mut Engine) {
            e.map(0x0250_0000, 0x1000);
        }
        fn does_not(_: &mut Engine) {}
        let report = Diff::new(crate::entry!(0x0040_2000, does_not()))
            .translation(crate::entry!(0x0040_2000, maps()).1)
            .control_words(&[OTHER_THREADS])
            .run();
        assert_eq!(
            report.first.last().map(|d| d.what.clone()),
            Some(Found::Mapped {
                page: 0x0250_0000,
                by_typed: false
            }),
            "{report}"
        );
        let report = Diff::new(crate::entry!(0x0040_2000, maps()))
            .translation(crate::entry!(0x0040_2000, does_not()).1)
            .control_words(&[OTHER_THREADS])
            .run();
        assert_eq!(
            report.first.last().map(|d| d.what.clone()),
            Some(Found::Mapped {
                page: 0x0250_0000,
                by_typed: true
            }),
            "{report}"
        );
    }

    /// Thrown objects compare by what was thrown, not as "a stop with no message".
    #[test]
    fn thrown_objects_compare_by_value() {
        fn throw(object: u32, throw_info: u32) -> ! {
            std::panic::resume_unwind(Box::new(Thrown { object, throw_info }))
        }
        fn one(_: &mut Engine) {
            throw(0x0200_0000, 0x0101_0000);
        }
        fn other_type(_: &mut Engine) {
            throw(0x0200_0000, 0x0101_0040);
        }
        fn other_object(_: &mut Engine) {
            throw(0x0200_0010, 0x0101_0000);
        }
        fn on_stack(e: &mut Engine, value: u32) {
            e.with_stack(16, |e, local| {
                e.mem.set_u32(local.addr(), value);
                throw(local.addr(), 0x0101_0000);
            });
        }
        fn on_stack_deeper(e: &mut Engine, value: u32) {
            e.with_stack(32, |e, _| on_stack(e, value));
        }
        fn on_stack_other_value(e: &mut Engine, value: u32) {
            on_stack(e, value + 1);
        }
        let run = |translation: (u32, AbiFn), typed: (u32, AbiFn), word: &'static [u32]| {
            Diff::new(typed)
                .translation(translation.1)
                .inputs(&[Input::Of(word)])
                .control_words(&[OTHER_THREADS])
                .expect_stops(|_| true)
                .setup(|e, case| {
                    e.map(0x0200_0000, 0x100);
                    case.words()
                })
                .run()
        };
        const NO_WORD: &[u32] = &[0];
        let same = run(
            crate::entry!(0x0040_2000, one()),
            crate::entry!(0x0040_2000, one()),
            NO_WORD,
        );
        assert!(same.is_same(), "{same}");
        for typed in [
            crate::entry!(0x0040_2000, other_type()),
            crate::entry!(0x0040_2000, other_object()),
        ] {
            let report = run(crate::entry!(0x0040_2000, one()), typed, NO_WORD);
            assert_eq!(report.found, 1, "{report}");
            assert!(matches!(report.first[0].what, Found::Stop { .. }));
        }
        // On the stack its address may differ, not its contents.
        let report = run(
            crate::entry!(0x0040_2000, on_stack(u32)),
            crate::entry!(0x0040_2000, on_stack_deeper(u32)),
            &[5],
        );
        assert!(report.is_same(), "{report}");
        let report = run(
            crate::entry!(0x0040_2000, on_stack(u32)),
            crate::entry!(0x0040_2000, on_stack_other_value(u32)),
            &[5],
        );
        assert!(!report.is_same(), "{report}");
    }

    /// NaN payloads differ by default, and may be let through where the float rule accepts it:
    /// in the return, in memory and in a call's words. A NaN against a number never is.
    #[test]
    fn nan_payloads_are_allowed_only_when_asked() {
        fn quiet(e: &mut Engine, p: Ptr) -> f64 {
            e.mem.set_u32(p.addr(), 0x7fc0_0000);
            e.mem.set_u64(p.addr() + 8, 0x7ff8_0000_0000_0000);
            e.call(CALLEE, &[0x7fc0_0000]);
            f64::from_bits(0x7ff8_0000_0000_0000)
        }
        fn payload(e: &mut Engine, p: Ptr) -> f64 {
            e.mem.set_u32(p.addr(), 0xffc1_2345);
            e.mem.set_u64(p.addr() + 8, 0x7ff8_0000_0012_3456);
            e.call(CALLEE, &[0x7fc1_2345]);
            f64::from_bits(0xfff8_0000_0000_0001)
        }
        fn infinity(e: &mut Engine, p: Ptr) -> f64 {
            e.mem.set_u32(p.addr(), 0x7fc0_0000);
            e.mem.set_u64(p.addr() + 8, 0x7ff0_0000_0000_0000);
            e.call(CALLEE, &[0x7fc0_0000]);
            f64::from_bits(0x7ff8_0000_0000_0000)
        }
        let diff = |typed: (u32, AbiFn)| {
            Diff::new(typed)
                .translation(crate::entry!(0x0040_2000, quiet(Ptr) -> f64).1)
                .inputs(&[Input::Flag])
                .control_words(&[OTHER_THREADS])
                .setup(|e, _| {
                    e.register(CALLEE, |_, _| Ret::default());
                    vec![e.mem.alloc(0x10)]
                })
        };
        assert!(!diff(crate::entry!(0x0040_2000, payload(Ptr) -> f64))
            .run()
            .is_same());
        let report = diff(crate::entry!(0x0040_2000, payload(Ptr) -> f64))
            .allow_nan_payloads()
            .run();
        assert!(report.is_same(), "{report}");
        let report = diff(crate::entry!(0x0040_2000, infinity(Ptr) -> f64))
            .allow_nan_payloads()
            .run();
        assert!(!report.is_same(), "an infinity is no NaN: {report}");
    }

    /// The return alone: a payload there differs by default, bit for bit.
    #[test]
    fn a_nan_payload_in_the_return_alone_differs_by_default() {
        fn quiet(_: &mut Engine) -> f64 {
            f64::from_bits(0x7ff8_0000_0000_0000)
        }
        fn payload(_: &mut Engine) -> f64 {
            f64::from_bits(0x7ff8_0000_0012_3456)
        }
        let diff = || {
            Diff::new(crate::entry!(0x0040_2000, payload() -> f64))
                .translation(crate::entry!(0x0040_2000, quiet() -> f64).1)
                .control_words(&[OTHER_THREADS])
        };
        let report = diff().run();
        assert_eq!(report.found, 1, "{report}");
        assert!(matches!(report.first[0].what, Found::Return { .. }));
        assert!(diff().allow_nan_payloads().run().is_same());
    }

    /// The values whose rounding shows the difference between x87 and SSE.
    #[test]
    fn the_float_lists_hold_values_that_round() {
        for bits in [0x3eaa_aaab, 0x3dcc_cccd, 0x3f80_0001] {
            assert!(F32_VALUES.contains(&bits), "{bits:08x}");
        }
        assert_eq!(f32::from_bits(0x3eaa_aaab), 1.0 / 3.0);
        assert_eq!(f32::from_bits(0x3dcc_cccd), 0.1);
        assert_eq!(f32::from_bits(0x3f80_0001), 1.0 + f32::EPSILON);
        assert_eq!(f64::from_bits(0x3fd5_5555_5555_5555), 1.0 / 3.0);
        assert_eq!(f64::from_bits(0x3fb9_9999_9999_999a), 0.1);
        assert_eq!(f64::from_bits(0x3ff0_0000_0000_0001), 1.0 + f64::EPSILON);
        for bits in [
            0x3fd5_5555_5555_5555,
            0x3fb9_9999_9999_999a,
            0x3ff0_0000_0000_0001,
        ] {
            assert!(F64_VALUES.contains(&bits), "{bits:016x}");
        }
    }

    /// Whatever `NV_TYPED` has on, the harness's engines hold translations: a child process with a
    /// family on, whose callee only `Engine::new` has.
    #[test]
    fn the_engines_hold_translations_whatever_the_switch_says() {
        let out = std::process::Command::new(std::env::current_exe().expect("this test binary"))
            .args([
                "--ignored",
                "--exact",
                "--nocapture",
                "typed::diff::tests::child_compares_with_the_family_on",
            ])
            .env("NV_TYPED", "diff_child_family")
            .output()
            .expect("the child runs");
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "{text}");
        assert!(text.contains("1 passed"), "{text}");
    }

    /// Run only by [`the_engines_hold_translations_whatever_the_switch_says`].
    #[test]
    #[ignore]
    fn child_compares_with_the_family_on() {
        static CHILD: Family = Family {
            name: "diff_child_family",
            id: FamilyId::new(13),
            past_gate: false,
            threads: Threads::Both,
            funcs: || vec![crate::entry!(0x0040_2100, member(u32) -> u32)],
        };
        super::super::provide(&CHILD);
        assert!(Engine::new().is_registered(CALLEE), "the switch has it on");
        fn sees_no_family(e: &mut Engine) -> u32 {
            assert!(!e.is_registered(CALLEE));
            assert_eq!(e.call_stack.last(), Some(&ADDR));
            1
        }
        let report = Diff::new(crate::entry!(0x0040_2000, sees_no_family() -> u32))
            .translation(crate::entry!(0x0040_2000, sees_no_family() -> u32).1)
            .run();
        report.assert_same();
    }

    // ---- What the second review asked ----------------------------------------------------

    /// `expect_stops` covers the inputs it states: a bad pointer outside them still fails, and an
    /// input it states has to stop.
    #[test]
    fn expect_stops_covers_only_the_inputs_it_states() {
        let diff = |inputs: &'static [u32]| {
            Diff::new(crate::entry!(0x0040_2000, reads(u32) -> u32))
                .translation(crate::entry!(0x0040_2000, reads(u32) -> u32).1)
                .inputs(&[Input::Of(inputs)])
                .control_words(&[OTHER_THREADS])
                .expect_stops(|case| case.word(0) == 0x0300_0000)
                .setup(|e, case| {
                    e.map(0x0200_0000, 4);
                    case.words()
                })
        };
        let report = diff(&[0x0200_0000, 0x0300_0000]).run();
        assert!(report.is_same(), "{report}");
        assert_eq!(report.stopped, 1);
        // A stop on another input.
        let report = diff(&[0x0400_0000]).run();
        assert_eq!(report.found, 1, "{report}");
        assert!(matches!(report.first[0].what, Found::StoppedAlike { .. }));
        // The stated input maps after all: it says it stops, and neither side does.
        let report = Diff::new(crate::entry!(0x0040_2000, reads(u32) -> u32))
            .translation(crate::entry!(0x0040_2000, reads(u32) -> u32).1)
            .inputs(&[Input::Of(&[0x0200_0000])])
            .control_words(&[OTHER_THREADS])
            .expect_stops(|_| true)
            .setup(|e, case| {
                e.map(0x0200_0000, 4);
                case.words()
            })
            .run();
        assert_eq!(report.first[0].what, Found::ExpectedStop, "{report}");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore)]
    #[should_panic(expected = "the translation given is the typed version")]
    fn the_typed_version_can_not_be_its_own_translation() {
        fn f(_: &mut Engine) {}
        let same = crate::entry!(0x0040_2000, f());
        let _ = Diff::new(same).translation(same.1);
    }

    /// With no `.translation`, the one registered at the address; the same check holds.
    #[test]
    #[cfg_attr(not(debug_assertions), ignore)]
    #[should_panic(expected = "is the typed version")]
    fn a_setup_can_not_register_the_typed_version_as_the_translation() {
        fn f(_: &mut Engine) {}
        let same = crate::entry!(0x0040_2000, f());
        let _ = Diff::new(same)
            .control_words(&[OTHER_THREADS])
            .setup(move |e, _| {
                e.register(same.0, same.1);
                vec![]
            })
            .run();
    }

    // ---- A non-leaf: a caller whose translation calls a leaf plainly, in two families ------

    const CALLER_ID: FamilyId = FamilyId::new(14);
    const LEAF_ID: FamilyId = FamilyId::new(15);
    const LEAF: u32 = 0x0040_2300;

    /// The leaf, translated and typed. It reads a field and stores a count, so a call that goes wrong
    /// shows in memory as well as in its answer.
    fn leaf_translated(e: &mut Engine, p: Ptr) -> u32 {
        let n = e.mem.u32(p.addr() + 4);
        e.mem.set_u32(p.addr() + 8, n + 1);
        n
    }
    fn leaf_typed(e: &mut Engine, p: Ptr) -> u32 {
        let n = e.mem.u32(p.addr() + 4);
        e.mem.set_u32(p.addr() + 8, n + 1);
        n
    }
    /// A leaf of its family that answers wrong.
    fn leaf_wrong(e: &mut Engine, p: Ptr) -> u32 {
        e.mem.u32(p.addr() + 4) + 1
    }

    static LEAF_FAMILY: Family = Family {
        name: "diff_test_leaf",
        id: LEAF_ID,
        past_gate: false,
        threads: Threads::Both,
        funcs: || vec![crate::entry!(0x0040_2300, leaf_typed(Ptr) -> u32)],
    };

    /// The caller as the translation has it: the leaf is in its unit, so it calls it plainly, and
    /// another function by address.
    fn non_leaf_translated(e: &mut Engine, p: Ptr) -> u32 {
        let n = leaf_translated(e, p);
        e.call(CALLEE, &[n]);
        n + 1
    }
    fn caller_typed(e: &mut Engine, p: Ptr) -> u32 {
        let n: u32 = crate::typed_call!(LEAF_ID, 0x0040_2300, leaf_typed(e, p));
        e.call(CALLEE, &[n]);
        n + 1
    }
    /// The wrong function for the address: by address the registered one runs, so only the direct
    /// branch shows it.
    fn caller_wrong_function(e: &mut Engine, p: Ptr) -> u32 {
        let n: u32 = crate::typed_call!(LEAF_ID, 0x0040_2300, leaf_wrong(e, p));
        e.call(CALLEE, &[n]);
        n + 1
    }
    /// A wrong word to the leaf, with the leaf's own effect showing it.
    fn caller_wrong_word(e: &mut Engine, p: Ptr) -> u32 {
        let wrong = Ptr::new(p.addr() + 16);
        let n: u32 = crate::typed_call!(LEAF_ID, 0x0040_2300, leaf_typed(e, wrong));
        e.call(CALLEE, &[n]);
        n + 1
    }
    /// A wrong word to the callee the translation calls by address.
    fn caller_wrong_callee_word(e: &mut Engine, p: Ptr) -> u32 {
        let n: u32 = crate::typed_call!(LEAF_ID, 0x0040_2300, leaf_typed(e, p));
        e.call(CALLEE, &[n + 1]);
        n + 1
    }

    static CALLER_FAMILY: Family = Family {
        name: "diff_test_caller",
        id: CALLER_ID,
        past_gate: false,
        threads: Threads::Both,
        funcs: || vec![crate::entry!(0x0040_2310, caller_typed(Ptr) -> u32)],
    };

    fn diff_of_non_leaf(typed: (u32, AbiFn)) -> Diff {
        Diff::new(typed)
            .translation(crate::entry!(0x0040_2310, non_leaf_translated(Ptr) -> u32).1)
            .family(&CALLER_FAMILY)
            .family(&LEAF_FAMILY)
            .translation_calls_plainly(&[LEAF])
            .inputs(&[Input::Of(&[3, 0xffff])])
            .setup(|e, case| {
                // In this crate's tests nothing assembles the leaf's translation, so the setup
                // registers it, as a part crate's engine would have it already.
                let (leaf, f) = crate::entry!(0x0040_2300, leaf_translated(Ptr) -> u32);
                e.register(leaf, f);
                e.register(CALLEE, |_, _| Ret::default());
                let p = e.mem.alloc(0x20);
                e.mem.set_u32(p + 4, case.word(0));
                vec![p]
            })
    }

    /// The whole path of a non-leaf function through the harness. `--nocapture` prints each report.
    #[test]
    fn a_non_leaf_caller_in_two_families_passes_when_right_and_fails_when_wrong() {
        let show = |name: &str, report: &Report| eprintln!("{name}: {report}");
        // Right: the plain call in the translation is dropped from the typed side only, and the two
        // typed runs agree call for call, with the leaf call direct on the third run.
        let right = diff_of_non_leaf(crate::entry!(0x0040_2310, caller_typed(Ptr) -> u32)).run();
        show("right", &right);
        right.assert_same();
        assert_eq!(right.runs, 4);
        if cfg!(debug_assertions) {
            assert_eq!(right.direct_runs, 4);
            assert_eq!(right.direct_calls, 4, "the leaf, once a run");
        }

        // Without saying the translation calls it plainly: the typed side has a call the translation
        // hasn't.
        let unsaid = Diff::new(crate::entry!(0x0040_2310, caller_typed(Ptr) -> u32))
            .translation(crate::entry!(0x0040_2310, non_leaf_translated(Ptr) -> u32).1)
            .family(&CALLER_FAMILY)
            .family(&LEAF_FAMILY)
            .inputs(&[Input::Of(&[3])])
            .control_words(&[OTHER_THREADS])
            .setup(|e, case| {
                let (leaf, f) = crate::entry!(0x0040_2300, leaf_translated(Ptr) -> u32);
                e.register(leaf, f);
                e.register(CALLEE, |_, _| Ret::default());
                let p = e.mem.alloc(0x20);
                e.mem.set_u32(p + 4, case.word(0));
                vec![p]
            })
            .run();
        show("unsaid", &unsaid);
        assert!(
            unsaid.first.iter().any(|d| matches!(
                &d.what,
                Found::Call { at: 1, translation: Some((addr, _)), typed: Some((leaf, _)) }
                    if *addr == CALLEE && *leaf == LEAF
            )),
            "{unsaid}"
        );

        // The wrong function: only the direct branch runs it.
        let wrong =
            diff_of_non_leaf(crate::entry!(0x0040_2310, caller_wrong_function(Ptr) -> u32)).run();
        show("wrong function", &wrong);
        if cfg!(debug_assertions) {
            assert!(!wrong.is_same());
            assert!(wrong.first.iter().all(|d| d.branch == Branch::Direct));
        }

        // A wrong word to the leaf: its word isn't compared, what it did is.
        let wrong =
            diff_of_non_leaf(crate::entry!(0x0040_2310, caller_wrong_word(Ptr) -> u32)).run();
        show("wrong word to the leaf", &wrong);
        assert!(wrong
            .first
            .iter()
            .any(|d| d.branch == Branch::ByAddress && matches!(d.what, Found::Memory { .. })));

        // A wrong word to the callee the translation calls by address.
        let wrong =
            diff_of_non_leaf(crate::entry!(0x0040_2310, caller_wrong_callee_word(Ptr) -> u32))
                .run();
        show("wrong word to the callee", &wrong);
        assert!(wrong
            .first
            .iter()
            .any(|d| d.branch == Branch::ByAddress && matches!(d.what, Found::Call { at: 1, .. })));
    }

    /// A caller in one family into a leaf in another, with only the caller's family named: by
    /// address, no finding, and the report says nothing went direct. Naming the leaf's family as
    /// well sends it direct.
    #[test]
    fn a_family_not_named_goes_by_address_and_naming_it_goes_direct() {
        let diff = Diff::new(crate::entry!(0x0040_2310, caller_typed(Ptr) -> u32))
            .translation(crate::entry!(0x0040_2310, non_leaf_translated(Ptr) -> u32).1)
            .family(&CALLER_FAMILY)
            .translation_calls_plainly(&[LEAF])
            .inputs(&[Input::Of(&[3])])
            .control_words(&[OTHER_THREADS])
            .setup(|e, case| {
                let (leaf, f) = crate::entry!(0x0040_2300, leaf_translated(Ptr) -> u32);
                e.register(leaf, f);
                e.register(CALLEE, |_, _| Ret::default());
                let p = e.mem.alloc(0x20);
                e.mem.set_u32(p + 4, case.word(0));
                vec![p]
            });
        let report = diff.run();
        assert_eq!(report.found, 0, "{report}");
        assert_eq!(report.direct_calls, 0);
        let report = diff.family(&LEAF_FAMILY).run();
        report.assert_same();
        assert_eq!(report.direct_calls, usize::from(cfg!(debug_assertions)));
        // In either order: each family named goes on.
        let report = Diff::new(crate::entry!(0x0040_2310, caller_typed(Ptr) -> u32))
            .translation(crate::entry!(0x0040_2310, non_leaf_translated(Ptr) -> u32).1)
            .family(&LEAF_FAMILY)
            .family(&CALLER_FAMILY)
            .translation_calls_plainly(&[LEAF])
            .inputs(&[Input::Of(&[3])])
            .control_words(&[OTHER_THREADS])
            .setup(|e, case| {
                let (leaf, f) = crate::entry!(0x0040_2300, leaf_translated(Ptr) -> u32);
                e.register(leaf, f);
                e.register(CALLEE, |_, _| Ret::default());
                let p = e.mem.alloc(0x20);
                e.mem.set_u32(p + 4, case.word(0));
                vec![p]
            })
            .run();
        report.assert_same();
        assert_eq!(report.direct_calls, usize::from(cfg!(debug_assertions)));
    }

    /// `translation_calls_plainly` drops the typed side only: the translation logging the call
    /// (it isn't plain after all) still differs when the typed version doesn't make it.
    #[test]
    fn translation_calls_plainly_drops_the_typed_side_only() {
        fn calls_by_address(e: &mut Engine) {
            e.call(CALLEE, &[1]);
        }
        fn does_not(_: &mut Engine) {}
        let report = Diff::new(crate::entry!(0x0040_2000, does_not()))
            .translation(crate::entry!(0x0040_2000, calls_by_address()).1)
            .translation_calls_plainly(&[CALLEE])
            .control_words(&[OTHER_THREADS])
            .setup(|e, _| {
                e.register(CALLEE, |_, _| Ret::default());
                vec![]
            })
            .run();
        assert_eq!(report.found, 1, "{report}");
        assert!(matches!(report.first[0].what, Found::Call { .. }));
    }

    #[test]
    #[should_panic(expected = "00aa4ee0 is overridable")]
    fn translation_calls_plainly_refuses_an_overridable_address() {
        fn f(_: &mut Engine) {}
        let _ =
            Diff::new(crate::entry!(0x0040_2000, f())).translation_calls_plainly(&[0x00aa_4ee0]);
    }

    /// A double over a callee the translation reaches plainly runs on the typed side only.
    #[test]
    #[should_panic(expected = "the setup replaced what is registered at 00402100")]
    fn a_double_over_a_plain_callee_is_refused() {
        fn f(_: &mut Engine) {}
        let _ = Diff::new(crate::entry!(0x0040_2000, f()))
            .translation(|_, _| Ret::default())
            .translation_calls_plainly(&[CALLEE])
            .control_words(&[OTHER_THREADS])
            .setup(|e, _| {
                e.register_double(CALLEE, |_, _| Ret::default());
                vec![]
            })
            .run();
    }

    /// A double over a family's member: the typed runs would replace it.
    #[test]
    #[should_panic(expected = "the setup replaced what is registered at 00402200")]
    fn a_double_over_a_family_member_is_refused() {
        fn f(_: &mut Engine) {}
        let _ = Diff::new(crate::entry!(0x0040_2000, f()))
            .translation(|_, _| Ret::default())
            .family(&FAMILY)
            .control_words(&[OTHER_THREADS])
            .setup(|e, _| {
                e.register_double(MEMBER, |_, _| Ret::default());
                vec![]
            })
            .run();
    }

    // ---- A local that reaches a call, where the frames differ ---------------------------------

    /// A translation's shared frame with an out parameter inside it, as the exe's stack layout has it.
    fn shared_frame(e: &mut Engine) -> u32 {
        e.with_stack(0x30, |e, frame| {
            e.mem.set_u32(frame.addr(), 0x1234);
            let out = frame.addr() + 0x10;
            e.call(CALLEE, &[out]);
            e.mem.u32(out)
        })
    }
    /// The typed version: the sibling local is native, the out parameter its own four bytes.
    fn own_frame(e: &mut Engine) -> u32 {
        e.with_stack(4, |e, out| {
            e.call(CALLEE, &[out.addr()]);
            e.mem.u32(out.addr())
        })
    }
    fn callee_writes_out(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[0], 77);
        Ret::default()
    }

    fn diff_of_frames(typed: (u32, AbiFn)) -> Diff {
        Diff::new(typed)
            .translation(crate::entry!(0x0040_2000, shared_frame() -> u32).1)
            .control_words(&[OTHER_THREADS])
            .setup(|e, _| {
                e.register(CALLEE, callee_writes_out);
                vec![]
            })
    }

    #[test]
    fn a_shrunk_frame_needs_local_at_to_pass() {
        let exact = diff_of_frames(crate::entry!(0x0040_2000, own_frame() -> u32)).run();
        assert_eq!(exact.found, 1, "{exact}");
        assert!(matches!(exact.first[0].what, Found::Call { at: 1, .. }));
        let loose = diff_of_frames(crate::entry!(0x0040_2000, own_frame() -> u32))
            .local_at(CALLEE, 0, 4)
            .run();
        loose.assert_same();
    }

    /// The looser form still compares the local's own bytes and which local it is.
    #[test]
    fn local_at_still_compares_contents_and_identity() {
        fn wrong_contents(e: &mut Engine) -> u32 {
            e.with_stack(4, |e, out| {
                e.mem.set_u32(out.addr(), 1);
                e.call(CALLEE, &[out.addr()]);
                e.mem.u32(out.addr())
            })
        }
        let report = diff_of_frames(crate::entry!(0x0040_2000, wrong_contents() -> u32))
            .local_at(CALLEE, 0, 4)
            .run();
        assert_eq!(report.found, 1, "{report}");

        // The same local twice, against two locals with the same contents.
        fn same_local(e: &mut Engine) {
            e.with_stack(8, |e, a| {
                e.call(CALLEE, &[a.addr()]);
                e.call(CALLEE, &[a.addr()]);
            });
        }
        fn two_locals(e: &mut Engine) {
            e.with_stack(8, |e, a| {
                e.with_stack(8, |e, b| {
                    e.call(CALLEE, &[a.addr()]);
                    e.call(CALLEE, &[b.addr()]);
                });
            });
        }
        let report = Diff::new(crate::entry!(0x0040_2000, two_locals()))
            .translation(crate::entry!(0x0040_2000, same_local()).1)
            .control_words(&[OTHER_THREADS])
            .local_at(CALLEE, 0, 8)
            .setup(|e, _| {
                e.register(CALLEE, |_, _| Ret::default());
                vec![]
            })
            .run();
        assert_eq!(report.found, 1, "{report}");
        let Found::Call {
            at: 2,
            translation: Some((_, a)),
            typed: Some((_, b)),
        } = &report.first[0].what
        else {
            panic!("{report}");
        };
        assert!(matches!(a[0], Word::Local { ordinal: 1, .. }));
        assert!(matches!(b[0], Word::Local { ordinal: 2, .. }));
    }

    /// A stack address stored in memory or behind a stack word compares by value, so it fails where
    /// the frames differ; `local_at` lets it through as "some stack address".
    #[test]
    fn a_stored_stack_address_fails_on_different_frames_unless_local_at() {
        fn deeper(e: &mut Engine, p: Ptr) {
            e.with_stack(0x20, |e, _| stores(e, p));
        }
        fn stores(e: &mut Engine, p: Ptr) {
            e.with_stack(8, |e, local| {
                e.mem.set_u32(p.addr(), local.addr());
                e.call(CALLEE, &[local.addr()]);
                e.mem.set_u32(p.addr(), 0);
            });
        }
        let diff = || {
            Diff::new(crate::entry!(0x0040_2000, stores(Ptr)))
                .translation(crate::entry!(0x0040_2000, deeper(Ptr)).1)
                .inputs(&[Input::Flag])
                .control_words(&[OTHER_THREADS])
                .setup(|e, _| {
                    e.register(CALLEE, |_, _| Ret::default());
                    vec![e.mem.alloc(8)]
                })
        };
        let report = diff().run();
        assert!(
            report
                .first
                .iter()
                .any(|d| matches!(d.what, Found::BeforeCall { memory: true, .. })),
            "{report}"
        );
        diff().local_at(CALLEE, 0, 8).run().assert_same();
    }

    // ---- The rest ------------------------------------------------------------------------

    /// A family's row picks the control words, and the report says which ran. Only this function's
    /// family counts, and a test can still ask for others.
    #[test]
    fn a_familys_threads_row_picks_the_control_words() {
        fn as_on_the_main_thread(_: &mut Engine) -> u32 {
            1
        }
        fn depends_on_the_word(_: &mut Engine) -> u32 {
            u32::from(crate::x87::control_word() != MAIN_THREAD) + 1
        }
        static MAIN_ONLY: Family = Family {
            name: "diff_test_main_only",
            id: FamilyId::new(16),
            past_gate: false,
            threads: Threads::Main,
            funcs: || vec![crate::entry!(0x0040_2000, as_on_the_main_thread() -> u32)],
        };
        let diff = || {
            Diff::new(crate::entry!(0x0040_2000, as_on_the_main_thread() -> u32))
                .translation(crate::entry!(0x0040_2000, depends_on_the_word() -> u32).1)
        };
        let both = diff().run();
        assert_eq!(both.control_words, [OTHER_THREADS, MAIN_THREAD]);
        assert_eq!(both.found, 1, "the other threads' word differs: {both}");
        let main = diff().family(&MAIN_ONLY).run();
        assert_eq!(main.control_words, [MAIN_THREAD]);
        main.assert_same();
        assert!(main.to_string().contains("007f"), "{main}");
        let asked = diff()
            .family(&MAIN_ONLY)
            .control_words(&[OTHER_THREADS])
            .run();
        assert_eq!(asked.found, 1, "{asked}");
    }

    #[test]
    fn a_clock_step_moved_across_a_call_is_caught() {
        fn step_then_call(e: &mut Engine) {
            e.advance_clock_us(1500);
            e.call(CALLEE, &[]);
        }
        fn call_then_step(e: &mut Engine) {
            e.call(CALLEE, &[]);
            e.advance_clock_us(1500);
        }
        let report = Diff::new(crate::entry!(0x0040_2000, call_then_step()))
            .translation(crate::entry!(0x0040_2000, step_then_call()).1)
            .control_words(&[OTHER_THREADS])
            .setup(|e, _| {
                e.register(CALLEE, |_, _| Ret::default());
                vec![]
            })
            .run();
        assert_eq!(report.found, 1, "the end is the same: {report}");
        assert_eq!(
            report.first[0].what,
            Found::BeforeCall {
                at: 1,
                addr: CALLEE,
                memory: false,
                clock: true
            }
        );
    }

    /// `stack_bytes` sets how far behind a word contents compare.
    #[test]
    fn stack_bytes_sets_how_far_contents_compare() {
        fn second_word(e: &mut Engine, value: u32) {
            e.with_stack(16, |e, local| {
                e.mem.set_u32(local.addr() + 8, value);
                e.call(CALLEE, &[local.addr()]);
            });
        }
        fn one(e: &mut Engine) {
            second_word(e, 1);
        }
        fn two(e: &mut Engine) {
            second_word(e, 2);
        }
        let diff = || {
            Diff::new(crate::entry!(0x0040_2000, two()))
                .translation(crate::entry!(0x0040_2000, one()).1)
                .control_words(&[OTHER_THREADS])
                .setup(|e, _| {
                    e.register(CALLEE, |_, _| Ret::default());
                    vec![]
                })
        };
        assert_eq!(diff().run().found, 1);
        assert_eq!(diff().stack_bytes(12).run().found, 1);
        diff().stack_bytes(8).run().assert_same();
        diff().stack_bytes(0).run().assert_same();
    }

    /// A word on the stack outside every live frame: frame 0, counted from the stack's end.
    #[test]
    fn a_stale_stack_word_compares_from_the_stacks_end() {
        fn stale(e: &mut Engine, back: u32) {
            let gone = e.with_stack(8, |_, local| local.addr());
            e.call(CALLEE, &[gone - back]);
        }
        fn right(e: &mut Engine) {
            stale(e, 0);
        }
        fn wrong(e: &mut Engine) {
            stale(e, 4);
        }
        let run = |typed: (u32, AbiFn)| {
            Diff::new(typed)
                .translation(crate::entry!(0x0040_2000, right()).1)
                .control_words(&[OTHER_THREADS])
                .setup(|e, _| {
                    e.register(CALLEE, |_, _| Ret::default());
                    vec![]
                })
                .run()
        };
        run(crate::entry!(0x0040_2000, right())).assert_same();
        let report = run(crate::entry!(0x0040_2000, wrong()));
        let Found::Call {
            typed: Some((_, typed)),
            translation: Some((_, translation)),
            ..
        } = &report.first[0].what
        else {
            panic!("{report}");
        };
        let (
            Word::Stack {
                offset: a,
                frame: 0,
                ..
            },
            Word::Stack {
                offset: b,
                frame: 0,
                ..
            },
        ) = (typed[0], translation[0])
        else {
            panic!("{report}");
        };
        assert_eq!(a, b + 4);
    }

    /// A thrown object's bytes stop at its frame: what lies past it is the frame gap, never zeroed.
    #[test]
    fn a_thrown_objects_bytes_stop_at_its_frame() {
        fn throw_from(e: &mut Engine, gap: u32) {
            e.with_stack(16, |e, local| {
                // Past the local, in the gap above it.
                e.mem.set_u32(local.addr() + 20, gap);
                std::panic::resume_unwind(Box::new(Thrown {
                    object: local.addr(),
                    throw_info: 0x0101_0000,
                }));
            });
        }
        fn one(e: &mut Engine) {
            throw_from(e, 1);
        }
        fn two(e: &mut Engine) {
            throw_from(e, 2);
        }
        let report = Diff::new(crate::entry!(0x0040_2000, two()))
            .translation(crate::entry!(0x0040_2000, one()).1)
            .control_words(&[OTHER_THREADS])
            .expect_stops(|_| true)
            .run();
        report.assert_same();
    }
}
