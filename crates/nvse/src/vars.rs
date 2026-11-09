//! String and array variables (ADR-0016 §5; build step I-5): xNVSE's `VarMap`, `StringVarMap`
//! and `ArrayVarMap` (`VarMap.h`, `StringVar.cpp`, `ArrayVar.cpp`, `ArrayVarElementContainer.cpp`
//! (xNVSE)). They are the extender's own state, not game state: the exe never reads them, so they
//! live host side, keyed by the ids scripts hold in their locals.
//!
//! - **Ids** as xNVSE allocates them: the smallest id freed earlier, else one past the largest
//!   in use, else 1 (`GetUnusedID`), so a co-save made by the original keeps its ids.
//! - **Temporaries**: a string made by a command outside a `set`/`let` target, and every array
//!   until something refers to it, is temporary; the main loop's tick (the seam at `0086B386`)
//!   deletes them (`Clean`), the last id first.
//! - **References**: an array keeps the mod index of each variable or element that refers to it
//!   (`m_refs`); with none left it becomes temporary again.
//! - **Event lists**: the locals that hold strings or arrays are recorded per event list
//!   (`g_nvseVarGarbageCollectionMap`); when the game destroys an event list (`0041af70`,
//!   wrapped) its strings become temporary and its arrays lose that reference (`CleanUpNVSEVars`,
//!   `Hooks_Other.cpp` (xNVSE)).
//!
//! Strings compare without case through the exe's own `toupper` (`game_toupper`, `0xECA7F4`, as
//! xNVSE calls it): its answers for every byte are read once, the first time a comparison needs
//! them.

use crate::Nvse;
use engine_core::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// A sorted set of ids kept as runs (first id to last id), so the free ids a co-save's gap
/// leaves (every id below the next one read) are one entry however many they are. As a set it
/// is xNVSE's `availableIDs`.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct IdSet {
    runs: BTreeMap<u32, u32>,
}

impl IdSet {
    pub fn insert(&mut self, id: u32) {
        self.insert_run(id, id);
    }

    /// Every id from `first` to `last`, both included.
    pub fn insert_run(&mut self, mut first: u32, mut last: u32) {
        if first > last {
            return;
        }
        // A run that ends at or just before `first` joins it.
        if let Some((&s, &l)) = self.runs.range(..=first).next_back() {
            if l == u32::MAX || l + 1 >= first {
                first = s;
                last = last.max(l);
                self.runs.remove(&s);
            }
        }
        // Runs that start inside or just after the new one join it.
        while let Some((&s, &l)) = self
            .runs
            .range(first..)
            .next()
            .filter(|(&s, _)| last == u32::MAX || s <= last + 1)
        {
            last = last.max(l);
            self.runs.remove(&s);
        }
        self.runs.insert(first, last);
    }

    pub fn remove(&mut self, id: u32) {
        let Some((&s, &l)) = self.runs.range(..=id).next_back() else {
            return;
        };
        if l < id {
            return;
        }
        self.runs.remove(&s);
        if s < id {
            self.runs.insert(s, id - 1);
        }
        if id < l {
            self.runs.insert(id + 1, l);
        }
    }

    pub fn contains(&self, id: u32) -> bool {
        self.runs
            .range(..=id)
            .next_back()
            .is_some_and(|(_, &l)| l >= id)
    }

    /// The smallest id, taken out.
    pub fn pop_first(&mut self) -> Option<u32> {
        let (&s, _) = self.runs.iter().next()?;
        self.remove(s);
        Some(s)
    }

    /// The ids in order (every one of them: for tests and small sets).
    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.runs.iter().flat_map(|(&s, &l)| s..=l)
    }

    /// How many runs there are.
    pub fn runs(&self) -> usize {
        self.runs.len()
    }
}

/// `VarMap`'s id sets (`usedIDs`, `tempIDs`, `availableIDs`, sorted sets in xNVSE).
#[derive(Default, Clone, Debug)]
pub struct VarIds {
    pub used: BTreeSet<u32>,
    pub temp: BTreeSet<u32>,
    pub available: IdSet,
}

impl VarIds {
    /// `GetUnusedID`: the first freed id (taken out of the free set), else one past the
    /// largest used, else 1. The largest is never `u32::MAX` from a file (a co-save naming it
    /// is corrupt and stops the read); were every id up to it used, the first unused one.
    pub fn unused(&mut self) -> u32 {
        if let Some(first) = self.available.pop_first() {
            return first;
        }
        match self.used.iter().next_back() {
            None => 1,
            Some(&last) => last.checked_add(1).unwrap_or_else(|| {
                let mut want = 1u32;
                for &id in &self.used {
                    if id != want {
                        break;
                    }
                    want = want.saturating_add(1);
                }
                want
            }),
        }
    }

    /// `Delete`'s bookkeeping: the id is free again.
    fn free(&mut self, id: u32) {
        self.used.remove(&id);
        self.temp.remove(&id);
        if id != 0 {
            self.available.insert(id);
        }
    }

    fn mark_temporary(&mut self, id: u32, temporary: bool) {
        if temporary {
            self.temp.insert(id);
        } else {
            self.temp.remove(&id);
        }
    }
}

/// The order xNVSE's string and array maps list their variables in (`UnorderedMap<UInt32, Var>`,
/// `containers.h` (xNVSE)), which is the order the co-save stores them in: buckets of a power of
/// two (8 at first, doubled before an insert finds more entries than buckets, up to 0x40000),
/// each a chain with the newest entry first; a key's bucket is `(k * 13) ^ (k >> 15)` masked;
/// a doubling moves the old buckets' chains, bucket by bucket, each entry to the head of its new
/// chain. Removing an entry unlinks it; a reset (`VarMap::Reset`) removes every entry and keeps
/// the buckets. Only the keys are kept here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashOrder {
    buckets: Vec<Vec<u32>>,
    num_buckets: u32,
    entries: u32,
}

impl Default for HashOrder {
    fn default() -> HashOrder {
        HashOrder {
            buckets: Vec::new(),
            num_buckets: 8,
            entries: 0,
        }
    }
}

impl HashOrder {
    /// A map made with `count` buckets (a power of two: `AlignBucketCount` keeps 0x40 and 0x80).
    pub fn with_buckets(count: u32) -> HashOrder {
        HashOrder {
            buckets: Vec::new(),
            num_buckets: count,
            entries: 0,
        }
    }

    /// `HashKey<UInt32>`.
    fn hash(key: u32) -> u32 {
        key.wrapping_mul(0xd) ^ (key >> 0xf)
    }

    fn bucket(&self, key: u32) -> usize {
        (Self::hash(key) & (self.num_buckets - 1)) as usize
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), containers.h: UnorderedMap::InsertKey,
    // ResizeTable.
    pub fn insert(&mut self, key: u32) {
        if self.buckets.is_empty() {
            // `AlignBucketCount(8)`.
            self.buckets = vec![Vec::new(); self.num_buckets as usize];
        } else if self.entries > self.num_buckets && self.num_buckets < 0x4_0000 {
            let count = self.num_buckets << 1;
            let mut grown = vec![Vec::new(); count as usize];
            for chain in &self.buckets {
                for &k in chain {
                    grown[(Self::hash(k) & (count - 1)) as usize].insert(0, k);
                }
            }
            self.buckets = grown;
            self.num_buckets = count;
        }
        let b = self.bucket(key);
        if self.buckets[b].contains(&key) {
            return;
        }
        self.entries += 1;
        self.buckets[b].insert(0, key);
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), containers.h: UnorderedMap::Erase.
    pub fn remove(&mut self, key: u32) {
        if self.entries == 0 {
            return;
        }
        let b = self.bucket(key);
        if let Some(i) = self.buckets[b].iter().position(|&k| k == key) {
            self.buckets[b].remove(i);
            self.entries -= 1;
        }
    }

    /// `VarMap::Reset`'s removals: no entry left, the buckets kept.
    pub fn clear(&mut self) {
        for chain in &mut self.buckets {
            chain.clear();
        }
        self.entries = 0;
    }

    /// The keys in the map's iteration order (`Iterator`: buckets in order, each chain from its
    /// head).
    pub fn keys(&self) -> Vec<u32> {
        self.buckets.iter().flatten().copied().collect()
    }

    /// The head of the first chain from bucket `from` on that has one, with its bucket
    /// (`Bucket::Clear`'s walk).
    pub fn head_from(&self, from: usize) -> Option<(usize, u32)> {
        self.buckets
            .iter()
            .enumerate()
            .skip(from)
            .find_map(|(b, chain)| chain.first().map(|&k| (b, k)))
    }
}
/// A string variable (`StringVar`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StringVar {
    pub data: Vec<u8>,
    pub owning_mod: u8,
    /// The per-thread string a command's result goes to inside an expression
    /// (`isFunctionResultCache`): never temporary, never deleted by `Delete`.
    pub function_result_cache: bool,
}

/// What a script local holds for the garbage collection (`NVSEVarType`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VarType {
    Number,
    Ref,
    String,
    Array,
}

/// An element's or a key's value (`ArrayData`): a number, a form id, a string, an array id.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Data {
    #[default]
    Invalid,
    Number(f64),
    Form(u32),
    Str(Vec<u8>),
    Array(u32),
}

impl Data {
    /// `DataType`: 0 invalid, 1 number, 2 form, 3 string, 4 array.
    pub fn type_code(&self) -> u8 {
        match self {
            Data::Invalid => 0,
            Data::Number(_) => 1,
            Data::Form(_) => 2,
            Data::Str(_) => 3,
            Data::Array(_) => 4,
        }
    }
}

/// An array element (`ArrayElement`): its value and the array that holds it (0 for a free
/// element, whose array value is then not a reference).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Element {
    pub data: Data,
    pub owning_array: u32,
}

/// The key types (`m_keyType`): numbers or strings.
pub const KEY_NUMBER: u8 = 1;
pub const KEY_STRING: u8 = 3;

/// A key (`ArrayKey`).
#[derive(Clone, Debug, PartialEq)]
pub enum Key {
    Number(f64),
    Str(Vec<u8>),
}

impl Key {
    pub fn type_code(&self) -> u8 {
        match self {
            Key::Number(_) => KEY_NUMBER,
            Key::Str(_) => KEY_STRING,
        }
    }
}

/// The element container (`ArrayVarElementContainer`): a packed array, a map by number (sorted
/// ascending) or a map by string (sorted without case, the first spelling kept).
#[derive(Clone, Debug, PartialEq)]
pub enum Container {
    Array(Vec<Element>),
    NumMap(Vec<(f64, Element)>),
    StrMap(Vec<(Vec<u8>, Element)>),
}

/// An array variable (`ArrayVar`).
#[derive(Clone, Debug, PartialEq)]
pub struct ArrayVar {
    pub id: u32,
    pub owning_mod: u8,
    pub key_type: u8,
    pub packed: bool,
    /// The mod index of each reference (`m_refs`).
    pub refs: Vec<u8>,
    pub elements: Container,
}

impl ArrayVar {
    pub fn len(&self) -> usize {
        match &self.elements {
            Container::Array(v) => v.len(),
            Container::NumMap(v) => v.len(),
            Container::StrMap(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The keys in order.
    pub fn keys(&self) -> Vec<Key> {
        match &self.elements {
            Container::Array(v) => (0..v.len()).map(|i| Key::Number(i as f64)).collect(),
            Container::NumMap(v) => v.iter().map(|(k, _)| Key::Number(*k)).collect(),
            Container::StrMap(v) => v.iter().map(|(k, _)| Key::Str(k.clone())).collect(),
        }
    }

    /// The element at `key` (`Get(key, false)`).
    pub fn get(&self, upper: &[u8; 256], key: &Key) -> Option<&Element> {
        let at = self.position(upper, key)?;
        Some(self.at(at))
    }

    fn at(&self, i: usize) -> &Element {
        match &self.elements {
            Container::Array(v) => &v[i],
            Container::NumMap(v) => &v[i].1,
            Container::StrMap(v) => &v[i].1,
        }
    }

    fn at_mut(&mut self, i: usize) -> &mut Element {
        match &mut self.elements {
            Container::Array(v) => &mut v[i],
            Container::NumMap(v) => &mut v[i].1,
            Container::StrMap(v) => &mut v[i].1,
        }
    }

    /// Where `key` is, if it is there. A packed array takes the number as an `int`
    /// (`cvttsd2si`), counting from the end when negative.
    fn position(&self, upper: &[u8; 256], key: &Key) -> Option<usize> {
        if key.type_code() != self.key_type {
            return None;
        }
        match (&self.elements, key) {
            (Container::Array(v), Key::Number(n)) => {
                let mut idx = crate::args::f64_to_i32(*n);
                if idx < 0 {
                    idx = idx.wrapping_add(v.len() as i32);
                }
                let idx = idx as u32 as usize;
                (idx < v.len()).then_some(idx)
            }
            (Container::NumMap(v), Key::Number(n)) => v.iter().position(|(k, _)| k == n),
            (Container::StrMap(v), Key::Str(s)) => v
                .iter()
                .position(|(k, _)| str_compare(upper, k, s) == std::cmp::Ordering::Equal),
            _ => None,
        }
    }
}

/// `StrCompare` (xNVSE `utility.cpp`): each byte through the exe's `toupper`, compared as
/// unsigned bytes; a string that ends first is less.
pub fn str_compare(upper: &[u8; 256], l: &[u8], r: &[u8]) -> std::cmp::Ordering {
    for (i, &lc) in l.iter().enumerate() {
        let Some(&rc) = r.get(i) else {
            return std::cmp::Ordering::Greater;
        };
        let (lu, ru) = (upper[usize::from(lc)], upper[usize::from(rc)]);
        if lu != ru {
            return lu.cmp(&ru);
        }
    }
    if r.len() > l.len() {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Equal
    }
}

/// The string and array variables of an engine.
#[derive(Default)]
pub struct State {
    pub strings: HashMap<u32, StringVar>,
    pub string_ids: VarIds,
    pub arrays: HashMap<u32, ArrayVar>,
    pub array_ids: VarIds,
    /// The maps' iteration orders as xNVSE's hash maps have them (the co-save's order).
    pub string_order: HashOrder,
    pub array_order: HashOrder,
    /// The locals recorded per event list (`g_nvseVarGarbageCollectionMap`).
    pub collected: HashMap<u32, HashMap<u32, VarType>>,
    /// The exe's `toupper` for every byte (filled on first use).
    upper: Option<Box<[u8; 256]>>,
}

/// The exe's `toupper` (`game_toupper` (xNVSE), `0xECA7F4`).
const GAME_TOUPPER: u32 = 0x00ec_a7f4;

fn state(e: &mut Engine) -> &mut State {
    &mut e.ext::<Nvse>().vars
}

/// The exe's `toupper` of every byte.
pub fn upper_table(e: &mut Engine) -> [u8; 256] {
    if let Some(t) = &state(e).upper {
        return **t;
    }
    let mut t = [0u8; 256];
    for (c, slot) in t.iter_mut().enumerate() {
        *slot = e.call(GAME_TOUPPER, &args![c as u32]).u32() as u8;
    }
    state(e).upper = Some(Box::new(t));
    t
}

// ---- strings (`StringVarMap`) ----

/// The text of string `id` (`g_StringMap.Get`), if it exists.
pub fn string(e: &Engine, id: u32) -> Option<Vec<u8>> {
    e.ext_get::<Nvse>()?
        .vars
        .strings
        .get(&id)
        .map(|s| s.data.clone())
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVarMap::Add.
/// A new string owned by mod `owning_mod`; temporary with `temporary`.
pub fn add_string(e: &mut Engine, owning_mod: u8, data: &[u8], temporary: bool) -> u32 {
    let s = state(e);
    let id = s.string_ids.unused();
    s.string_ids.used.insert(id);
    s.string_order.insert(id);
    s.strings.insert(
        id,
        StringVar {
            data: data.to_vec(),
            owning_mod,
            function_result_cache: false,
        },
    );
    if temporary {
        mark_string_temporary(e, id, true);
    }
    id
}

/// Sets string `id`'s text (`StringVar::Set`); false when there is no such string.
pub fn set_string(e: &mut Engine, id: u32, data: &[u8]) -> bool {
    match state(e).strings.get_mut(&id) {
        Some(s) => {
            s.data = data.to_vec();
            true
        }
        None => false,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp:
// StringVarMap::MarkTemporary.
/// The function result string is never made temporary.
pub fn mark_string_temporary(e: &mut Engine, id: u32, temporary: bool) {
    let s = state(e);
    if temporary && s.strings.get(&id).is_some_and(|v| v.function_result_cache) {
        return;
    }
    s.string_ids.mark_temporary(id, temporary);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVarMap::Delete.
pub fn delete_string(e: &mut Engine, id: u32) {
    let s = state(e);
    if s.strings.get(&id).is_some_and(|v| v.function_result_cache) {
        s.string_ids.temp.remove(&id);
        return;
    }
    s.strings.remove(&id);
    s.string_ids.free(id);
    s.string_order.remove(id);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVarMap::Clean.
/// Deletes the temporary strings, the largest id first.
pub fn clean_strings(e: &mut Engine) {
    while let Some(&last) = state(e).string_ids.temp.iter().next_back() {
        delete_string(e, last);
    }
}

// ---- arrays (`ArrayVarMap`) ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVarMap::Create and
// the ArrayVar constructor.
/// A new, temporary array: string keys make a string map, else `packed` an array, else a
/// number map.
pub fn create_array(e: &mut Engine, key_type: u8, packed: bool, owning_mod: u8) -> u32 {
    let s = state(e);
    let id = s.array_ids.unused();
    s.array_ids.used.insert(id);
    s.array_order.insert(id);
    let elements = if key_type == KEY_STRING {
        Container::StrMap(Vec::new())
    } else if packed {
        Container::Array(Vec::new())
    } else {
        Container::NumMap(Vec::new())
    };
    s.arrays.insert(
        id,
        ArrayVar {
            id,
            owning_mod,
            key_type,
            packed,
            refs: Vec::new(),
            elements,
        },
    );
    s.array_ids.mark_temporary(id, true);
    id
}

pub fn array(e: &Engine, id: u32) -> Option<&ArrayVar> {
    e.ext_get::<Nvse>()?.vars.arrays.get(&id)
}

pub fn array_exists(e: &Engine, id: u32) -> bool {
    id != 0 && array(e, id).is_some()
}

/// `GetArrayOwningModIndex`: an array's owning mod, 0 for none.
pub fn array_owning_mod(e: &Engine, id: u32) -> u8 {
    array(e, id).map_or(0, |a| a.owning_mod)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp:
// ArrayVarMap::AddReference.
/// `*reference` is replaced by a reference to `to` from mod `referring_mod`: the old one is
/// removed first; a missing array leaves the reference 0.
pub fn add_reference(e: &mut Engine, reference: &mut u32, to: u32, referring_mod: u8) {
    if *reference != 0 {
        remove_reference(e, reference, referring_mod);
    }
    let s = state(e);
    if let Some(a) = s.arrays.get_mut(&to) {
        a.refs.push(referring_mod);
        *reference = to;
        s.array_ids.mark_temporary(to, false);
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp:
// ArrayVarMap::RemoveReference.
/// One reference from `referring_mod` goes; with none left the array is temporary. The
/// reference becomes 0.
pub fn remove_reference(e: &mut Engine, reference: &mut u32, referring_mod: u8) {
    let s = state(e);
    if let Some(a) = s.arrays.get_mut(reference) {
        if let Some(i) = a.refs.iter().position(|&m| m == referring_mod) {
            a.refs.remove(i);
        }
        if a.refs.is_empty() {
            let id = a.id;
            s.array_ids.mark_temporary(id, true);
        }
    }
    *reference = 0;
}

/// The same for a local's double (`AddReference(double*, ...)`): the id goes through a
/// `UInt32` (`cvttsd2si`, then the bits).
pub fn add_reference_double(e: &mut Engine, local: u32, to: u32, referring_mod: u8) {
    let mut id = crate::args::f64_to_i32(e.mem.f64(local)) as u32;
    add_reference(e, &mut id, to, referring_mod);
    e.mem.set_f64(local, f64::from(id));
}

pub fn remove_reference_double(e: &mut Engine, local: u32, referring_mod: u8) {
    let mut id = crate::args::f64_to_i32(e.mem.f64(local)) as u32;
    remove_reference(e, &mut id, referring_mod);
    e.mem.set_f64(local, f64::from(id));
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayElement::UnsetDefault.
/// What clearing an element does to the arrays it refers to: an array value held inside an
/// array loses its reference, from the referenced array's own owning mod (as xNVSE passes
/// `GetArrayOwningModIndex(m_data.arrID)`). Lambdas' saved variables come with I-7.
fn unset(e: &mut Engine, element: &Element) {
    if let Data::Array(id) = element.data {
        if element.owning_array != 0 {
            let owner = array_owning_mod(e, id);
            let mut reference = id;
            remove_reference(e, &mut reference, owner);
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVarMap::Delete (through
// VarMap::Delete and the ArrayVar destructor).
/// The array and its elements go; the arrays its elements referred to lose those references.
pub fn delete_array(e: &mut Engine, id: u32) {
    let s = state(e);
    let gone = s.arrays.remove(&id);
    s.array_ids.free(id);
    s.array_order.remove(id);
    if let Some(a) = gone {
        for i in 0..a.len() {
            unset(e, a.at(i));
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVarMap::Clean.
/// Deletes the temporary arrays, the largest id first; deleting one may make others temporary,
/// which the same loop deletes.
pub fn clean_arrays(e: &mut Engine) {
    while let Some(&last) = state(e).array_ids.temp.iter().next_back() {
        delete_array(e, last);
    }
}

/// The value an element is set to (`ArrayElement::Set`, `SetNumber`, ...).
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Form(u32),
    Str(Vec<u8>),
    Array(u32),
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::Get (with
// bCanCreateNew) and ArrayVar::SetElement.
/// Sets the element at `key` of array `id`, making it when it is not there: a packed array
/// appends at its end (whatever the index), a map inserts in order. False when the array does
/// not exist or the key's type is not the array's.
pub fn set_element(e: &mut Engine, id: u32, key: &Key, value: Value) -> bool {
    let upper = upper_table(e);
    let Some(a) = state(e).arrays.get_mut(&id) else {
        return false;
    };
    if key.type_code() != a.key_type {
        return false;
    }
    let at = match a.position(&upper, key) {
        Some(at) => at,
        None => match (&mut a.elements, key) {
            (Container::Array(v), _) => {
                v.push(Element::default());
                v.len() - 1
            }
            (Container::NumMap(v), Key::Number(n)) => {
                let at = v.partition_point(|(k, _)| k < n);
                v.insert(at, (*n, Element::default()));
                at
            }
            (Container::StrMap(v), Key::Str(text)) => {
                let at = v.partition_point(|(k, _)| {
                    str_compare(&upper, k, text) == std::cmp::Ordering::Less
                });
                v.insert(at, (text.clone(), Element::default()));
                at
            }
            _ => return false,
        },
    };
    let old = std::mem::take(a.at_mut(at));
    unset(e, &old);
    let owner_mod = array_owning_mod(e, id);
    let data = match value {
        Value::Number(n) => Data::Number(n),
        Value::Form(f) => Data::Form(f),
        Value::Str(s) => Data::Str(s),
        // `SetArray` inside an array: a reference from the holding array's owning mod.
        Value::Array(to) => {
            let mut reference = 0;
            add_reference(e, &mut reference, to, owner_mod);
            Data::Array(reference)
        }
    };
    if let Some(a) = state(e).arrays.get_mut(&id) {
        *a.at_mut(at) = Element {
            data,
            owning_array: id,
        };
    }
    true
}

/// The element at `key` of array `id`, cloned.
pub fn get_element(e: &mut Engine, id: u32, key: &Key) -> Option<Element> {
    let upper = upper_table(e);
    array(e, id)?.get(&upper, key).cloned()
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVarElementContainer.cpp:
// ArrayVarElementContainer::erase(const ArrayKey *) (through ArrayVar::EraseElement).
/// Erases the element at `key`; answers how many were erased (`-1` as `u32` for an empty array
/// or a key of the wrong type, as `EraseElement` answers).
pub fn erase_element(e: &mut Engine, id: u32, key: &Key) -> u32 {
    let upper = upper_table(e);
    let Some(a) = state(e).arrays.get_mut(&id) else {
        return 0;
    };
    if a.is_empty() || a.key_type != key.type_code() {
        return u32::MAX;
    }
    // A packed array erases by `(UInt32)(int)key`, no counting from the end.
    let at = match (&a.elements, key) {
        (Container::Array(v), Key::Number(n)) => {
            let idx = crate::args::f64_to_i32(*n) as u32 as usize;
            (idx < v.len()).then_some(idx)
        }
        _ => a.position(&upper, key),
    };
    let Some(at) = at else {
        return 0;
    };
    let old = match &mut a.elements {
        Container::Array(v) => v.remove(at),
        Container::NumMap(v) => v.remove(at).1,
        Container::StrMap(v) => v.remove(at).1,
    };
    unset(e, &old);
    1
}

/// Every element, cleared (`EraseAllElements`); answers how many there were.
pub fn erase_all(e: &mut Engine, id: u32) -> u32 {
    let Some(a) = state(e).arrays.get_mut(&id) else {
        return 0;
    };
    let old: Vec<Element> = match &mut a.elements {
        Container::Array(v) => std::mem::take(v),
        Container::NumMap(v) => std::mem::take(v).into_iter().map(|(_, x)| x).collect(),
        Container::StrMap(v) => std::mem::take(v).into_iter().map(|(_, x)| x).collect(),
    };
    for x in &old {
        unset(e, x);
    }
    old.len() as u32
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), VarMap.h: VarMap::Reset (for both maps).
/// Every string and array variable gone, the id sets emptied (ids start from 1 again).
pub fn reset(e: &mut Engine) {
    let st = state(e);
    st.strings.clear();
    st.arrays.clear();
    st.string_ids = VarIds::default();
    st.array_ids = VarIds::default();
    st.string_order.clear();
    st.array_order.clear();
}

/// `ArrayElement::Unset` of an element taken out of its array by a caller (a range erased, an
/// array shrunk): what [`erase_element`] does to the element it removes.
pub fn release_element(e: &mut Engine, element: &Element) {
    unset(e, element);
}

/// `g_nvseVarGarbageCollectionMap.Clear()`: no event list's locals are recorded any more.
pub fn forget_collected(e: &mut Engine) {
    state(e).collected.clear();
}
// ---- event lists ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ScriptTokens.cpp: AddToGarbageCollection.
pub fn collect(e: &mut Engine, event_list: u32, local: u32, kind: VarType) {
    state(e)
        .collected
        .entry(event_list)
        .or_default()
        .insert(local, kind);
}

/// The recorded kinds of an event list's locals.
pub fn collected_kinds(e: &mut Engine, event_list: u32) -> HashMap<u32, VarType> {
    state(e)
        .collected
        .get(&event_list)
        .cloned()
        .unwrap_or_default()
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp: CleanUpNVSEVar.
/// One local of a list gives up its string (made temporary) or array reference, and is no
/// longer recorded.
pub fn clean_up_local(e: &mut Engine, event_list: u32, local: u32) {
    let kind = state(e)
        .collected
        .get_mut(&event_list)
        .and_then(|m| m.remove(&local));
    let modi = script_mod_index(e, e.mem.u32(event_list));
    match kind {
        Some(VarType::String) => {
            let id = crate::args::f64_to_i32(e.mem.f64(local + 8)) as u32;
            mark_string_temporary(e, id, true);
        }
        Some(VarType::Array) => remove_reference_double(e, local + 8, modi),
        _ => {}
    }
}
/// `ScriptEventList::Destroy` (`0041af70`), wrapped.
pub const EVENT_LIST_DESTROY: engine_core::extend::WrapSlot =
    engine_core::extend::WrapSlot::new(engine_core::extend::Owner::Nvse, 0x0041_af70);

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Other.cpp: CleanUpNVSEVars.
/// Before the game destroys an event list: each recorded local of it still in its variable
/// list (`m_vars`, `+0xC`) gives up its string (made temporary) or its array reference (from the
/// event list's script's mod).
pub fn clean_up_event_list(e: &mut Engine, event_list: u32) {
    let Some(recorded) = state(e).collected.remove(&event_list) else {
        return;
    };
    let script = e.mem.u32(event_list);
    let modi = script_mod_index(e, script);
    let mut node = e.mem.u32(event_list + 0xc);
    while node != 0 {
        let local = e.mem.u32(node);
        if local != 0 {
            match recorded.get(&local) {
                Some(VarType::String) => {
                    let id = crate::args::f64_to_i32(e.mem.f64(local + 8)) as u32;
                    mark_string_temporary(e, id, true);
                }
                Some(VarType::Array) => remove_reference_double(e, local + 8, modi),
                _ => {}
            }
        }
        node = e.mem.u32(node + 4);
    }
}

/// `TESForm::GetModIndex`: the top byte of the form's id (`+0xC`); 0 for no form.
pub fn script_mod_index(e: &Engine, script: u32) -> u8 {
    if script == 0 {
        0
    } else {
        (e.mem.u32(script + 0xc) >> 24) as u8
    }
}

/// `ScriptEventListsDestroyedHook` (xNVSE `Hooks_Other.cpp`) replaces the whole function and
/// always frees the list; every caller of `0041af70` in the exe passes 1 (`push 1` before each of
/// its eight calls, Ghidra), so the exe's own body, called here after the clean-up, frees alike.
/// Before the clean-up, lambdas lose the list as their parent (`DeleteEventList` calls
/// `LambdaManager::MarkParentAsDeleted`), which keeps a copy for saved lambdas.
fn event_list_destroy(e: &mut Engine, a: &[u32]) -> Ret {
    crate::lambda::mark_parent_deleted(e, a[0]);
    clean_up_event_list(e, a[0]);
    e.call_next(EVENT_LIST_DESTROY.addr(), a)
}

/// Installs the event list wrap (the tick that cleans the temporaries is [`crate::tick`]).
pub fn install(e: &mut Engine) {
    e.wrap(EVENT_LIST_DESTROY, event_list_destroy);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0200_0000, 0x1_0000);
        e.register(GAME_TOUPPER, |_, a| Ret {
            eax: u32::from((a[0] as u8).to_ascii_uppercase()),
            ..Ret::default()
        });
        e
    }

    /// The free-id set in runs: neighbours join, a removal splits, the smallest comes first.
    #[test]
    fn free_ids_are_kept_as_runs() {
        let mut s = IdSet::default();
        s.insert_run(5, 9);
        s.insert(4);
        s.insert(10);
        s.insert_run(20, u32::MAX);
        assert_eq!(s.runs(), 2);
        s.remove(7);
        assert_eq!(s.runs(), 3);
        assert!(s.contains(6) && !s.contains(7) && s.contains(u32::MAX));
        assert_eq!(s.iter().take(4).collect::<Vec<_>>(), [4, 5, 6, 8]);
        assert_eq!(s.pop_first(), Some(4));
        s.insert_run(6, 30);
        assert_eq!(s.runs(), 1);
        assert_eq!(s.pop_first(), Some(5));
    }

    /// Ids: the smallest freed id comes back first, then one past the largest.
    #[test]
    fn ids_are_reused_as_xnvse_allocates_them() {
        let mut e = engine();
        let a = add_string(&mut e, 1, b"a", false);
        let b = add_string(&mut e, 1, b"b", false);
        let c = add_string(&mut e, 1, b"c", false);
        assert_eq!((a, b, c), (1, 2, 3));
        delete_string(&mut e, 2);
        delete_string(&mut e, 1);
        assert_eq!(add_string(&mut e, 1, b"x", false), 1);
        assert_eq!(add_string(&mut e, 1, b"y", false), 2);
        assert_eq!(add_string(&mut e, 1, b"z", false), 4);
    }

    /// Temporary strings and unreferenced arrays go at the clean-up (each tick, [`crate::tick`]);
    /// a referenced array stays.
    #[test]
    fn the_tick_cleans_temporaries() {
        let mut e = engine();
        e.register(EVENT_LIST_DESTROY.addr(), |_, _| Ret::default());
        install(&mut e);
        let kept = add_string(&mut e, 1, b"kept", false);
        let temp = add_string(&mut e, 1, b"temp", true);
        let held = create_array(&mut e, KEY_NUMBER, true, 1);
        let free = create_array(&mut e, KEY_NUMBER, true, 1);
        e.mem.set_f64(0x0200_0000, 0.0);
        add_reference_double(&mut e, 0x0200_0000, held, 1);
        assert_eq!(e.mem.f64(0x0200_0000), f64::from(held));
        clean_arrays(&mut e);
        clean_strings(&mut e);
        assert!(string(&e, kept).is_some());
        assert!(string(&e, temp).is_none());
        assert!(array_exists(&e, held));
        assert!(!array_exists(&e, free));
        // Without its reference the array goes at the next tick.
        remove_reference_double(&mut e, 0x0200_0000, 1);
        assert_eq!(e.mem.f64(0x0200_0000), 0.0);
        clean_arrays(&mut e);
        clean_strings(&mut e);
        assert!(!array_exists(&e, held));
    }

    /// An array held by an element lives while its holder does; deleting the holder lets it go
    /// at the same clean-up.
    #[test]
    fn nested_arrays_follow_their_holders() {
        let mut e = engine();
        let outer = create_array(&mut e, KEY_STRING, false, 1);
        let inner = create_array(&mut e, KEY_NUMBER, true, 1);
        let mut var = 0;
        add_reference(&mut e, &mut var, outer, 1);
        assert!(set_element(
            &mut e,
            outer,
            &Key::Str(b"Inner".to_vec()),
            Value::Array(inner)
        ));
        assert_eq!(array(&e, inner).unwrap().refs, vec![1]);
        clean_arrays(&mut e);
        assert!(array_exists(&e, inner));
        // String keys ignore case and keep the first spelling.
        assert!(set_element(
            &mut e,
            outer,
            &Key::Str(b"INNER".to_vec()),
            Value::Number(2.0)
        ));
        assert_eq!(
            array(&e, outer).unwrap().keys(),
            vec![Key::Str(b"Inner".to_vec())]
        );
        // Overwritten: the inner array lost its reference and goes.
        clean_arrays(&mut e);
        assert!(!array_exists(&e, inner));
        remove_reference(&mut e, &mut var, 1);
        clean_arrays(&mut e);
        assert!(!array_exists(&e, outer));
    }

    /// Packed arrays append whatever the index and read negative indexes from the end; number
    /// maps keep their keys in order.
    #[test]
    fn containers_keep_xnvses_order() {
        let mut e = engine();
        let arr = create_array(&mut e, KEY_NUMBER, true, 1);
        set_element(&mut e, arr, &Key::Number(5.0), Value::Number(10.0));
        set_element(&mut e, arr, &Key::Number(0.0), Value::Number(11.0));
        set_element(&mut e, arr, &Key::Number(1.0), Value::Str(b"s".to_vec()));
        let last = get_element(&mut e, arr, &Key::Number(-1.0)).unwrap();
        assert_eq!(last.data, Data::Str(b"s".to_vec()));
        assert_eq!(erase_element(&mut e, arr, &Key::Number(0.0)), 1);
        assert_eq!(array(&e, arr).unwrap().len(), 1);
        let map = create_array(&mut e, KEY_NUMBER, false, 1);
        for k in [3.0, -1.0, 2.5] {
            set_element(&mut e, map, &Key::Number(k), Value::Number(k));
        }
        assert_eq!(
            array(&e, map).unwrap().keys(),
            vec![Key::Number(-1.0), Key::Number(2.5), Key::Number(3.0)]
        );
        assert!(!set_element(
            &mut e,
            map,
            &Key::Str(b"x".to_vec()),
            Value::Number(1.0)
        ));
    }

    /// Destroying an event list frees what its recorded locals held.
    #[test]
    fn a_destroyed_event_list_gives_up_its_strings_and_arrays() {
        let mut e = engine();
        let (list, node, string_local, array_local, script) = (
            0x0200_1000,
            0x0200_1100,
            0x0200_1200,
            0x0200_1300,
            0x0200_1400,
        );
        e.mem.set_u32(list, script);
        e.mem.set_u32(script + 0xc, 0x0500_0800);
        e.mem.set_u32(list + 0xc, node);
        e.mem.set_u32(node, string_local);
        e.mem.set_u32(node + 4, node + 8);
        e.mem.set_u32(node + 8, array_local);
        let s = add_string(&mut e, 5, b"text", false);
        e.mem.set_f64(string_local + 8, f64::from(s));
        let a = create_array(&mut e, KEY_NUMBER, true, 5);
        e.mem.set_f64(array_local + 8, 0.0);
        add_reference_double(&mut e, array_local + 8, a, 5);
        collect(&mut e, list, string_local, VarType::String);
        collect(&mut e, list, array_local, VarType::Array);
        clean_up_event_list(&mut e, list);
        clean_strings(&mut e);
        clean_arrays(&mut e);
        assert!(string(&e, s).is_none());
        assert!(!array_exists(&e, a));
    }
}
