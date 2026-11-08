//! The counted references, the game's recursive spin lock, lists and arrays, and the small getters
//! the AI, the inventory, the HUD's compass and the process levels call on every actor.

use super::layout::{LightLock, ListNode, RefObject};
use super::{fld, fstp, m32_local, FAMILY};
use crate::culling::layout::NiPoint3;
use crate::pathing::{self, fcom, m64_global, parity_even};
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::x87::{x87_add, x87_mul, x87_quiet, x87_sub};
use engine_core::{args, typed_call, Engine};

/// The imports `InterlockedIncrement`, `InterlockedDecrement`, `InterlockedCompareExchange` and
/// `GetCurrentThreadId`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// The game's `Sleep` wrapper (`0040fca0`).
const SLEEP: u32 = 0x0040_fca0;
/// The spins the lock makes with `Sleep(0)` before it sleeps a millisecond at a time.
const SPINS_BEFORE_SLEEPING: u32 = 0x2710;
/// `NiRefObject`'s vtable and the count of live objects.
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// The reaction form's list (`00717e50`).
const REACTION_LIST: u32 = 0x0071_7e50;
/// A container's count of a form (`00482a90`), a reference's container (`0055d310`) and an extra
/// list's reference (`0041c8d0`).
const CONTAINER_COUNT: u32 = 0x0048_2a90;
const REFERENCE_CONTAINER: u32 = 0x0055_d310;
const EXTRA_REFERENCE: u32 = 0x0041_c8d0;
/// A reference's water height (`0057b0a0`), parent cell (`008d6f30`), the cell's water test
/// (`004518e0`) and an actor's height (`008853a0`).
const WATER_HEIGHT: u32 = 0x0057_b0a0;
const PARENT_CELL: u32 = 0x008d_6f30;
const CELL_HAS_WATER: u32 = 0x0045_18e0;
const ACTOR_HEIGHT: u32 = 0x0088_53a0;
/// The `double`s: no water (`-FLT_MAX`), the margin under the water, the eye height's share.
const NO_WATER: u32 = 0x0102_41b0;
const WATER_MARGIN: u32 = 0x0102_0998;
const HEIGHT_FACTOR: u32 = 0x0101_de30;
/// The ring of five locations and its index.
const LOCATION_RING: u32 = 0x011d_f7a0;
const LOCATION_RING_INDEX: u32 = 0x011d_f880;
/// The calendar's hour (a float), month, day (a signed byte) and year.
const GAME_HOUR: u32 = 0x011e_0358;
const GAME_MONTH: u32 = 0x011a_39c4;
const GAME_DAY: u32 = 0x011e_0360;
const GAME_YEAR: u32 = 0x011e_035c;
/// The player.
const PLAYER: u32 = 0x011d_ea3c;
/// An array's element destructor run and its buffer's free.
const DESTROY_ELEMENTS: u32 = 0x0072_ba80;
const FREE_BUFFER: u32 = 0x006a_8500;
/// The navmesh order array's vtable (`0106b1a4`) and its set-up (`006b3eb0`).
const ORDER_ARRAY_VTABLE: u32 = 0x0106_b1a4;
const ORDER_ARRAY_SET_UP: u32 = 0x006b_3eb0;

/// A list node's item: the word at the node (`006815c0` gives the node's own address).
#[inline(always)]
fn item(e: &mut Engine, node: u32) -> u32 {
    let at = typed_call!(
        leaf::FAMILY,
        0x0068_15c0,
        leaf::same_word(e, Ptr::<ListNode>::new(node))
    );
    e.mem.u32(at)
}

/// A list node's next (`00726070`).
#[inline(always)]
fn next(e: &mut Engine, node: u32) -> u32 {
    typed_call!(
        leaf::FAMILY,
        0x0072_6070,
        leaf::word_at_4(e, Ptr::<ListNode>::new(node))
    )
}

// Typed from 0040b460
/// `InterlockedIncrement` of the word at `target`, the new count.
pub fn increment(e: &mut Engine, target: Ptr) -> i32 {
    e.call(INTERLOCKED_INCREMENT, &args![target]).i32()
}

// Typed from 004019a0
/// `InterlockedDecrement` of the word at `target`, the new count.
pub fn decrement(e: &mut Engine, target: Ptr) -> i32 {
    e.call(INTERLOCKED_DECREMENT, &args![target]).i32()
}

// Typed from 0040f6e0
/// `NiRefObject::IncRefCount` (`thiscall(this)`): the count (`+4`) up (`0040b460`).
pub fn add_reference(e: &mut Engine, this: Ptr<RefObject>) {
    typed_call!(FAMILY, 0x0040_b460, increment(e, this.byte_add(4)));
}

// Typed from 00401970
/// `NiRefObject::DecRefCount` (`thiscall(this)`): the count (`+4`) down (`004019a0`); at zero the
/// object deletes itself (vtable +4).
pub fn release(e: &mut Engine, this: Ptr<RefObject>) {
    if typed_call!(FAMILY, 0x0040_19a0, decrement(e, this.byte_add(4))) == 0 {
        e.vcall(this.addr(), 4, &args![]);
    }
}

// Typed from 0045cec0
/// `NiPointer::~NiPointer` (`thiscall(this)`): the held object released (`00401970`), if any.
pub fn release_held(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr());
    if held != 0 {
        typed_call!(FAMILY, 0x0040_1970, release(e, Ptr::new(held)));
    }
}

// Typed from 004968b0
/// `NiRefObject::NiRefObject` (`thiscall(this)`): its vtable, a count of 0, one more live object
/// (`0040b460`).
pub fn ref_object_new(e: &mut Engine, this: Ptr<RefObject>) -> Ptr<RefObject> {
    e.set(this, RefObject::vtable, NI_REF_OBJECT_VTABLE);
    e.set(this, RefObject::m_uiRefCount, 0);
    typed_call!(
        FAMILY,
        0x0040_b460,
        increment(e, Ptr::new(NI_REF_OBJECT_COUNT))
    );
    this
}

// Typed from 00496910
/// `NiRefObject::~NiRefObject` (`thiscall(this)`): its vtable back, one live object fewer
/// (`004019a0`).
pub fn ref_object_drop(e: &mut Engine, this: Ptr<RefObject>) {
    e.set(this, RefObject::vtable, NI_REF_OBJECT_VTABLE);
    typed_call!(
        FAMILY,
        0x0040_19a0,
        decrement(e, Ptr::new(NI_REF_OBJECT_COUNT))
    );
}

// Typed from 0040fc90
/// `GetCurrentThreadId`.
pub fn current_thread(e: &mut Engine) -> u32 {
    e.call(GET_CURRENT_THREAD_ID, &[]).u32()
}

// Typed from 0043b460
/// `InterlockedCompareExchange(target, new, old)`: what `target` held.
pub fn compare_exchange(e: &mut Engine, target: Ptr, old: i32, new: i32) -> i32 {
    e.call(INTERLOCKED_COMPARE_EXCHANGE, &args![target, new, old])
        .i32()
}

// Typed from 0040fbf0
/// The recursive spin lock taken (`thiscall(this, unused)`): by its owner, one more count; otherwise
/// the owner word swapped from 0 to this thread's id (`0043b460`), `Sleep(0)` between tries and
/// `Sleep(1)` after 10,000, then a count of 1.
pub fn lock(e: &mut Engine, this: Ptr<LightLock>, _unused: u32) {
    let thread = typed_call!(FAMILY, 0x0040_fc90, current_thread(e));
    if e.get(this, LightLock::owner) == thread {
        let count = e.get(this, LightLock::count).wrapping_add(1);
        e.set(this, LightLock::count, count);
        return;
    }
    let mut attempts = 0u32;
    let new = thread as i32;
    while typed_call!(
        FAMILY,
        0x0043_b460,
        compare_exchange(e, this.cast(), 0, new)
    ) != 0
    {
        attempts = attempts.wrapping_add(1);
        let millis = u32::from(attempts > SPINS_BEFORE_SLEEPING);
        e.call(SLEEP, &args![millis]);
    }
    typed_call!(leaf::FAMILY, 0x0040_fbe0, leaf::nothing(e));
    e.set(this, LightLock::count, 1);
}

// Typed from 0040fba0
/// The lock given back (`thiscall(this)`): one count fewer; at zero no owner.
pub fn unlock(e: &mut Engine, this: Ptr<LightLock>) {
    typed_call!(leaf::FAMILY, 0x0040_fbe0, leaf::nothing(e));
    let count = e.get(this, LightLock::count).wrapping_sub(1);
    e.set(this, LightLock::count, count);
    if count == 0 {
        e.set(this, LightLock::owner, 0);
    }
}

// Typed from 004534f0
/// Does nothing (`RET 8`: two stack words).
pub fn nothing_2(_e: &mut Engine, _this: u32, _value: u32) {}

// Typed from 00483710
/// Does nothing (`RET 4`).
pub fn nothing_1(_e: &mut Engine, _this: u32) {}

// Typed from 00460140
/// The address `this + 0x10` (the data handler's world space list, a holder's value).
pub fn at_10(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x10)
}

// Typed from 008c71b0
/// The word at `this` becomes `value`; `this`.
pub fn store_word(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), value);
    this
}

// Typed from 0048c1b0
/// `TESReactionForm::GetReaction` (`thiscall(this, form)`): the reaction (+8) of the first entry of the
/// list (`00717e50`) whose form is `form`; 0 for none or a null form.
pub fn reaction(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    if form == 0 {
        return 0;
    }
    let mut node = e.call(REACTION_LIST, &args![this]).u32();
    while node != 0 && item(e, node) != 0 {
        let entry = item(e, node);
        if e.mem.u32(entry) == form {
            return e.mem.u32(entry + 8);
        }
        node = next(e, node);
    }
    0
}

// Typed from 008f6650
/// `HighProcess::GetDetectionState` (`thiscall(this, actor, which)`): the first detection state of the
/// detected list (`+0x25c`, `which` 0) or the detecting list (`+0x260`) whose actor is `actor`.
pub fn detection_state(e: &mut Engine, this: Ptr, actor: u32, which: i32) -> Ptr {
    let mut found = 0;
    let mut node = e
        .mem
        .u32(this.addr() + if which == 0 { 0x25c } else { 0x260 });
    while node != 0 {
        let state = item(e, node);
        if state == 0 || found != 0 {
            break;
        }
        if e.mem.u32(state) == actor {
            found = state;
        }
        node = next(e, node);
    }
    Ptr::new(found)
}

// Typed from 00968670
/// The word at `index` of an array (`00877a30` gives the slot).
pub fn array_item(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let slot = typed_call!(
        leaf::FAMILY,
        0x0087_7a30,
        leaf::array_element(e, this, index)
    );
    e.mem.u32(slot)
}

// Typed from 004c8f30
/// `InventoryChanges::GetItemCount` (`thiscall(this, form)`): the container's count of the form
/// (`004bffb0`, `00482a90`, made positive), plus the changes' entry's count (`004bfba0`); an entry
/// that counts 0 where the container has none counts 1.
pub fn item_count(e: &mut Engine, this: Ptr, form: u32) -> i32 {
    let container = typed_call!(FAMILY, 0x004b_ffb0, container_of(e, this));
    let mut count = if container != 0 {
        e.call(CONTAINER_COUNT, &args![container, form]).i32()
    } else {
        0
    };
    if count < 0 {
        count = count.wrapping_mul(-1);
    }
    let entry = typed_call!(FAMILY, 0x004b_fba0, entry_for(e, this, form, 1, 0));
    if entry != 0 {
        let number = e.mem.i32(entry + 4);
        count = if count == 0 && number == 0 {
            1
        } else {
            e.mem.i32(entry + 4).wrapping_add(count)
        };
    }
    count
}

// Typed from 004bffb0
/// The container of the changes' reference (`+4`, `0055d310`), or 0 without one.
pub fn container_of(e: &mut Engine, this: Ptr) -> u32 {
    let owner = e.mem.u32(this.addr() + 4);
    if owner == 0 {
        return 0;
    }
    e.call(REFERENCE_CONTAINER, &args![owner]).u32()
}

// Typed from 004bfba0
/// `InventoryChanges::GetEntryForObject` (`thiscall(this, form, unused, reference)`): the change entry
/// of `form` (its `+8`) in the list (`+0`); with a reference id, only when one of the entry's extra
/// lists has a reference (`0041c8d0`) whose id (`+0xc`) is it.
pub fn entry_for(e: &mut Engine, this: Ptr, form: u32, _unused: u32, reference: u32) -> u32 {
    let mut cursor = e.mem.u32(this.addr());
    let mut searching = true;
    while cursor != 0 && searching {
        let entry = item(e, cursor);
        if entry != 0 && e.mem.u32(entry + 8) == form {
            searching = false;
        } else {
            cursor = next(e, cursor);
        }
    }
    if cursor == 0 {
        return 0;
    }
    let entry = item(e, cursor);
    if entry == 0 || reference == 0 {
        return entry;
    }
    let mut inner = e.mem.u32(entry);
    while inner != 0 {
        let extra = item(e, inner);
        if extra != 0 && e.call(EXTRA_REFERENCE, &args![extra]).u32() != 0 {
            let found = e.call(EXTRA_REFERENCE, &args![extra]).u32();
            if typed_call!(leaf::FAMILY, 0x0084_e3a0, leaf::word_at_c(e, found)) == reference {
                return entry;
            }
        }
        inner = next(e, inner);
    }
    0
}

// Typed from 008ae4c0
/// `Actor::GetLocationOnReference` (`thiscall(this)`): the position (`00436aa0`), or, when the actor
/// has the flag at `+0x14d` and a water height (`0057b0a0`) that isn't `-FLT_MAX`, a copy of it in the
/// next of five slots of the ring at `011df7a0`, its z raised by the actor's height (`008853a0`) times
/// the factor at `0101de30`, but at most to the water height minus the margin at `01020998`, in a
/// cell with water (`004518e0`). All in x87, each operand as the exe reads it.
pub fn location_on_reference(e: &mut Engine, this: Ptr) -> u32 {
    if typed_call!(FAMILY, 0x005a_2030, byte_14d(e, this)) == 0 {
        return typed_call!(FAMILY, 0x0043_6aa0, position(e, this)).addr();
    }
    let water = e.call(WATER_HEIGHT, &args![this]).f64();
    if !parity_even(fcom(water, m64_global(e, NO_WATER)), 0x44) {
        return typed_call!(FAMILY, 0x0043_6aa0, position(e, this)).addr();
    }
    let index = e.global::<i32>(LOCATION_RING_INDEX).wrapping_add(1);
    e.set_global(LOCATION_RING_INDEX, index);
    if index >= 5 {
        e.set_global(LOCATION_RING_INDEX, 0i32);
    }
    let position = typed_call!(FAMILY, 0x0043_6aa0, position(e, this)).addr();
    let slot = |e: &Engine| LOCATION_RING + 12 * e.global::<u32>(LOCATION_RING_INDEX);
    super::copy_words(e, position, slot(e), 3);
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    if cell != 0 && e.call(CELL_HAS_WATER, &args![cell]).u32() & 0xff != 0 {
        let ceiling = fstp(x87_sub(
            e.call(WATER_HEIGHT, &args![this]).f64(),
            m64_global(e, WATER_MARGIN),
        ));
        let rise = fstp(x87_mul(
            e.call(ACTOR_HEIGHT, &args![this]).f64(),
            m64_global(e, HEIGHT_FACTOR),
        ));
        let z = Ptr::<NiPoint3>::new(slot(e));
        let top = x87_add(fld(e, z, NiPoint3::z), m32_local(rise));
        if fcom(super::fld_local(ceiling), top) & 0x41 != 0 {
            e.set(z, NiPoint3::z, x87_quiet(ceiling));
        } else {
            let z = Ptr::<NiPoint3>::new(slot(e));
            let raised = fstp(x87_add(fld(e, z, NiPoint3::z), m32_local(rise)));
            e.set(z, NiPoint3::z, raised);
        }
    }
    slot(e)
}

// Typed from 00436aa0
/// A reference's position: the address `this + 0x30`.
pub fn position(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x30)
}

// Typed from 005a2030
/// The byte at `+0x14d`.
pub fn byte_14d(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x14d)
}

// Typed from 008844f0
/// Whether the actor's state (`004f8960`, read again for each test) is 2 or 1, or 6 unless
/// `exclude_six`.
pub fn in_state(e: &mut Engine, this: Ptr, exclude_six: bool) -> bool {
    let is = |e: &mut Engine, v: u32| typed_call!(FAMILY, 0x004f_8960, word_108(e, this)) == v;
    if exclude_six {
        is(e, 2) || is(e, 1)
    } else {
        is(e, 2) || is(e, 1) || is(e, 6)
    }
}

// Typed from 004f8960
/// The word at `+0x108`.
pub fn word_108(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x108))
}

// Typed from 00907650
/// `LowProcess::ComputeLastTimeProcessed` (`thiscall(this)`): the hour last processed becomes the
/// game's hour (`00693d50`) and the date `month << 9 | day | year << 13` (`007037c0`; the day byte
/// sign-extended).
pub fn compute_last_time_processed(e: &mut Engine, this: Ptr) {
    let hour = x87_quiet(e.global::<f32>(GAME_HOUR));
    typed_call!(FAMILY, 0x0069_3d50, set_hour(e, this, hour));
    let month = e.global::<u32>(GAME_MONTH);
    let day = i32::from(e.global::<i8>(GAME_DAY)) as u32;
    let year = e.global::<u32>(GAME_YEAR);
    typed_call!(
        FAMILY,
        0x0070_37c0,
        set_date(e, this, (month << 9) | day | (year << 0xd))
    );
}

// Typed from 00693d50
/// The float at `+0x20` becomes `value`, through the x87 stack.
pub fn set_hour(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x20, x87_quiet(value));
}

// Typed from 007037c0
/// The word at `+0x24` becomes `value`.
pub fn set_date(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x24), value);
}

/// An actor's position for the compass (vtable +0x1f4), its three floats.
fn compass_position(e: &mut Engine, object: u32) -> [u32; 3] {
    let p = e.vcall(object, 0x1f4, &args![]).u32();
    [e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8)]
}

/// `|from - to|^2` as the compass sorts by it: the difference (`00439ef0`), its squared length
/// (`004a7290`) and that through the float `fabs` (`00408840`), in the translation's frames.
fn compass_distance(e: &mut Engine, from: [u32; 3], to: [u32; 3]) -> f32 {
    e.with_stack(24, |e, points| {
        for (index, word) in (0u32..).zip(from.into_iter().chain(to)) {
            e.mem.set_u32(points.addr() + 4 * index, word);
        }
        e.with_stack(12, |e, difference| {
            let a = points.cast::<NiPoint3>();
            let b = Ptr::<NiPoint3>::new(points.addr() + 12);
            let vector = typed_call!(
                pathing::FAMILY,
                0x0043_9ef0,
                pathing::point::subtract(e, a, difference.cast(), b)
            );
            let squared = typed_call!(
                pathing::FAMILY,
                0x004a_7290,
                pathing::point::length_squared(e, vector)
            );
            typed_call!(
                pathing::FAMILY,
                0x0040_8840,
                pathing::point::abs(e, squared)
            )
        })
    })
}

// Typed from 0077b300
/// The HUD's compass order of two actors (`cdecl(first, second)`, slots holding them): 1 when the
/// second is nearer the player, -1 when the first is, else 0; 0 when a slot or its actor is null.
pub fn compass_order(e: &mut Engine, first: Ptr<u32>, second: Ptr<u32>) -> i32 {
    if first.is_null()
        || e.mem.u32(first.addr()) == 0
        || second.is_null()
        || e.mem.u32(second.addr()) == 0
    {
        return 0;
    }
    let player = e.mem.u32(PLAYER);
    let player_position = compass_position(e, player);
    let first_actor = e.mem.u32(first.addr());
    let first_position = compass_position(e, first_actor);
    let first_distance = compass_distance(e, first_position, player_position);
    let second_actor = e.mem.u32(second.addr());
    let second_position = compass_position(e, second_actor);
    let second_distance = compass_distance(e, second_position, player_position);
    if second_distance < first_distance {
        1
    } else if first_distance < second_distance {
        -1
    } else {
        0
    }
}

// Typed from 0086a050
/// The factor of a source's object (`009ad610`): its vtable +0x420 when its +0x100 says so, else 1.0.
pub fn object_factor(e: &mut Engine, source: u32) -> f64 {
    let object = typed_call!(FAMILY, 0x009a_d610, word_cc(e, Ptr::new(source))).addr();
    if object != 0 && e.vcall(object, 0x100, &args![]).bool() {
        return e.vcall(object, 0x420, &args![]).f64();
    }
    1.0
}

// Typed from 009ad610
/// The word at `+0xcc`.
pub fn word_cc(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0xcc))
}

// Typed from 008454f0
/// `BSSimpleArray::Clear` (`thiscall(this, free)`): with a buffer, its elements destroyed
/// (`0072ba80`), with `free` the buffer freed (`006a8500`) and the buffer and reserved size cleared;
/// the size 0.
pub fn array_clear(e: &mut Engine, this: Ptr, free: u8) {
    let at = this.addr();
    let buffer = e.mem.u32(at + 4);
    if buffer == 0 {
        return;
    }
    let count = e.mem.u32(at + 8);
    e.call(DESTROY_ELEMENTS, &args![this, buffer, count]);
    if free != 0 {
        e.call(FREE_BUFFER, &args![this]);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 0xc, 0);
    }
    e.mem.set_u32(at + 8, 0);
}

// Typed from 0069aee0
/// The navmesh order array's constructor: its vtable, then its set-up with no buffer (`006b3eb0`).
pub fn order_array_new(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ORDER_ARRAY_VTABLE);
    e.call(ORDER_ARRAY_SET_UP, &args![this, 0u32, 0u32]);
    this
}

// Typed from 0069af10
/// The navmesh info pointer array's destructor: its vtable, then cleared and freed (`008454f0`).
pub fn info_array_drop(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ORDER_ARRAY_VTABLE);
    typed_call!(FAMILY, 0x0084_54f0, array_clear(e, this, 1));
    this
}

// Typed from 004f1540
/// `BSExtraData`'s type byte (`+4`).
pub fn extra_kind(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 4)
}
