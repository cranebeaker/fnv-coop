//! `fallout/ai/actorvaluecache.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The cache keeps 8 bytes per actor value (0x4d of them); the byte at the
//! start of each entry is its out-of-date flag.

#[allow(unused_imports)]
use crate::prelude::*;

/// The number of actor values the cache holds.
const ACTOR_VALUE_COUNT: i32 = 0x4d;
/// `ActorValue::GetDependentActorValues` (engine map), `cdecl`: actor value,
/// address of the count; answers the array of dependent actor values.
const GET_DEPENDENT_ACTOR_VALUES: u32 = 0x0066_edc0;

// Translated from 008c6f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValueCache::SetCachedActorValueOutOfDate` (Xbox PDB): for an actor
/// value in range (0 to 0x4c, signed), sets its entry's out-of-date byte and
/// marks the values that depend on it (`008c6f70`).
pub fn actor_value_cache_set_cached_actor_value_out_of_date(
    e: &mut Engine,
    this: Ptr,
    actor_value: i32,
) {
    if (0..ACTOR_VALUE_COUNT).contains(&actor_value) {
        let entry = this
            .addr()
            .wrapping_add((actor_value as u32).wrapping_mul(8));
        e.mem.set_u8(entry, 1);
        actor_value_cache_set_related_actor_values_out_of_date(e, this, actor_value as u32);
    }
}

// Translated from 008c6f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValueCache::SetRelatedActorValuesOutOfDate` (Xbox PDB): asks for
/// the actor values that depend on `actor_value` (`0066edc0`) and sets the
/// out-of-date byte of each one in range.
pub fn actor_value_cache_set_related_actor_values_out_of_date(
    e: &mut Engine,
    this: Ptr,
    actor_value: u32,
) {
    e.with_stack(4, |e, count_slot| {
        e.mem.set_u32(count_slot.addr(), 0);
        let dependents = e
            .call(GET_DEPENDENT_ACTOR_VALUES, &args![actor_value, count_slot])
            .u32();
        let count = e.mem.i32(count_slot.addr());
        if dependents != 0 && count != 0 {
            let mut index = 0i32;
            while index < count {
                let dependent = e
                    .mem
                    .i32(dependents.wrapping_add((index as u32).wrapping_mul(4)));
                if (0..ACTOR_VALUE_COUNT).contains(&dependent) {
                    let entry = this.addr().wrapping_add((dependent as u32).wrapping_mul(8));
                    e.mem.set_u8(entry, 1);
                }
                index = index.wrapping_add(1);
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008c_6f40,
            actor_value_cache_set_cached_actor_value_out_of_date(Ptr, i32)
        ),
        entry!(
            0x008c_6f70,
            actor_value_cache_set_related_actor_values_out_of_date(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dependents_double(e: &mut Engine, values: &[i32]) {
        let array = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_i32(array + 4 * i as u32, *v);
        }
        let count = values.len() as u32;
        e.register_double(GET_DEPENDENT_ACTOR_VALUES, move |e, a| {
            e.mem.set_u32(a[1], count);
            Ret {
                eax: array,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn related_values_in_range_are_marked() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x4d * 8);
        dependents_double(&mut e, &[2, -1, 0x4d, 0x4c]);
        e.call(0x008c_6f70, &args![this, 7u32]);
        assert_eq!(e.mem.u8(this + 2 * 8), 1);
        assert_eq!(e.mem.u8(this + 0x4c * 8), 1);
        assert_eq!(e.mem.u8(this + 7 * 8), 0);
        assert_eq!(e.mem.u8(this), 0);
    }

    #[test]
    fn no_array_or_no_count_marks_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x4d * 8);
        e.register(GET_DEPENDENT_ACTOR_VALUES, |e, a| {
            e.mem.set_u32(a[1], 3);
            Ret::default()
        });
        e.call(0x008c_6f70, &args![this, 7u32]);
        for i in 0..0x4d {
            assert_eq!(e.mem.u8(this + i * 8), 0);
        }
    }

    #[test]
    fn set_cached_value_out_of_date_checks_the_range() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x4d * 8);
        dependents_double(&mut e, &[1]);
        e.call_log = Some(vec![]);
        e.call(0x008c_6f40, &args![this, 5i32]);
        assert_eq!(e.mem.u8(this + 5 * 8), 1);
        assert_eq!(e.mem.u8(this + 8), 1);
        let asked = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == GET_DEPENDENT_ACTOR_VALUES)
            .count();
        assert_eq!(asked, 1);
        e.call_log = Some(vec![]);
        e.call(0x008c_6f40, &args![this, -1i32]);
        e.call(0x008c_6f40, &args![this, 0x4di32]);
        let asked = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == GET_DEPENDENT_ACTOR_VALUES)
            .count();
        assert_eq!(asked, 0);
    }
}
