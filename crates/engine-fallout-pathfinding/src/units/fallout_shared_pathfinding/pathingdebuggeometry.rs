//! `fallout shared/pathfinding/pathingdebuggeometry.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450` (unnamed in the engine map): the word at +0 of its `this`
/// (a smart pointer's object).
const FIRST_OF_00559450: u32 = 0x0055_9450;
/// `009611e0` (unnamed in the engine map): the node's parent (0 when none).
const PARENT_009611E0: u32 = 0x0096_11e0;
/// `008c7aa0` (unnamed in the engine map; no arguments; AL).
const MAIN_THREAD_TEST_008C7AA0: u32 = 0x008c_7aa0;
/// The parent's vtable slot (byte offset) that detaches a child.
const SLOT_DETACH: u32 = 0xe8;
/// `004537b0` (unnamed in the engine map; no arguments): the queue object
/// `0087ac60` runs on (the global at `011df1a8`).
const TASK_QUEUE_004537B0: u32 = 0x0045_37b0;
/// `0087ac60` (unnamed in the engine map; `RET 4`): queues a node's
/// detachment.
const QUEUE_DETACH_0087AC60: u32 = 0x0087_ac60;
/// `0066b0d0` (unnamed in the engine map; `RET 4`): sets a smart pointer.
const POINTER_SET_0066B0D0: u32 = 0x0066_b0d0;
/// The player (`011dea3c`) and the object (`011dea24`) the drawing is
/// checked against.
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
const SELECTED_011DEA24: u32 = 0x011d_ea24;
/// `008d6f30` (unnamed in the engine map): the player's 3D holder;
/// `00524cf0`: its node.
const HOLDER_008D6F30: u32 = 0x008d_6f30;
const NODE_00524CF0: u32 = 0x0052_4cf0;
/// `TESObjectREFR::GetSpace` (`00575ca0`, engine map).
const GET_SPACE_00575CA0: u32 = 0x0057_5ca0;
/// The selected object's vtable slots (byte offsets): a test (+0x100) and
/// its combat group (+0x3F8).
const SLOT_SELECTED_TEST: u32 = 0x100;
const SLOT_SELECTED_GROUP: u32 = 0x3f8;
/// `00408d60` (unnamed in the engine map): the address of a setting's value
/// (byte); the three settings that switch the parts of the drawing.
const SETTING_ADDRESS_00408D60: u32 = 0x0040_8d60;
const SETTING_011F1824: u32 = 0x011f_1824;
const SETTING_011F1858: u32 = 0x011f_1858;
const SETTING_011F1814: u32 = 0x011f_1814;
/// A byte (`011a4d34`) that turns the first part off.
const SWITCH_011A4D34: u32 = 0x011a_4d34;
/// `0044ddc0` (unnamed in the engine map): a `BSSimpleArray`'s size.
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
/// `006a7ad0`, `006a7af0` (navmeshsearch unit; `RET 4`): the addresses of
/// elements of the arrays at +0x138 and +0x120.
const ELEMENT_006A7AD0: u32 = 0x006a_7ad0;
const ELEMENT_006A7AF0: u32 = 0x006a_7af0;
/// `00aa13e0` (unnamed in the engine map; cdecl, a size): an allocation.
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `00a5ecb0` (unnamed in the engine map, `NiMain/ninode.cpp`; `RET 4`): a
/// node constructor.
const NODE_CTOR_00A5ECB0: u32 = 0x00a5_ecb0;
/// `00414430` (unnamed in the engine map; `RET 0x10`): a colour from four
/// floats.
const COLOUR_00414430: u32 = 0x0041_4430;
/// The colour components and sizes (floats) the drawing uses.
const HALF_01016248: u32 = 0x0101_6248;
const ARROW_SIZE_0101E570: u32 = 0x0101_e570;
const MARKER_SIZE_0101E57C: u32 = 0x0101_e57c;
const MARKER_HEIGHT_010311A8: u32 = 0x0103_11a8;
const RING_SIZE_011A4D2C: u32 = 0x011a_4d2c;
const SPHERE_SIZE_0101E340: u32 = 0x0101_e340;
const BOX_HEIGHT_01032ADC: u32 = 0x0103_2adc;
/// The `double`s the rings are stacked by (`0104f2f0`) and an actor's
/// radius is scaled by (`01016ff0`).
const RING_STEP_0104F2F0: u32 = 0x0104_f2f0;
const RADIUS_SCALE_01016FF0: u32 = 0x0101_6ff0;
/// `006815c0` (unnamed in the engine map): returns its `this`.
const IDENTITY_006815C0: u32 = 0x0068_15c0;
/// `00990890`, `00988790` (`RET 4`): the group's member count and member.
const MEMBER_COUNT_00990890: u32 = 0x0099_0890;
const MEMBER_00988790: u32 = 0x0098_8790;
/// The member reference's vtable slot (byte offset) with its position.
const SLOT_POSITION: u32 = 0x1f4;
/// `BSTestObjects::Create3DArrow` (`00c5a0f0`, engine map; cdecl, six words).
const CREATE_ARROW_00C5A0F0: u32 = 0x00c5_a0f0;
/// The root's vtable slot (byte offset) that attaches a child.
const SLOT_ATTACH: u32 = 0xdc;
/// `00c58f00` (`BSMain/bstestobjects.obj`; cdecl, three words): a ring.
const CREATE_RING_00C58F00: u32 = 0x00c5_8f00;
/// `004bc1f0` (unnamed in the engine map; `RET 0xC`): sets an object's
/// translation from three floats.
const SET_TRANSLATE_XYZ_004BC1F0: u32 = 0x004b_c1f0;
/// `00c58d30` (`BSMain/bstestobjects.obj`; cdecl, six words): a box.
const CREATE_BOX_00C58D30: u32 = 0x00c5_8d30;
/// `00440460` (unnamed in the engine map; `RET 4`): sets an object's
/// translation.
const SET_TRANSLATE_00440460: u32 = 0x0044_0460;
/// `00903600` (`RET 4`): an actor of the group.
const ACTOR_00903600: u32 = 0x0090_3600;
/// `Actor::GetRadius` (`008be1b0`, engine map; ST0).
const GET_RADIUS_008BE1B0: u32 = 0x008b_e1b0;
/// `005d8550` (unnamed in the engine map; cdecl, one word): the address of
/// a colour (four words) for the group's first word.
const GROUP_COLOUR_005D8550: u32 = 0x005d_8550;
/// `00988750`, `00988770` (`RET 4`): the count and the elements of a third
/// list of the group.
const LIST_COUNT_00988750: u32 = 0x0098_8750;
const LIST_ELEMENT_00988770: u32 = 0x0098_8770;
/// `0084e3a0` (unnamed in the engine map): the element's space.
const ELEMENT_SPACE_0084E3A0: u32 = 0x0084_e3a0;
/// `00985be0` (unnamed in the engine map; `RET 4`): the element's colour
/// into the address it is given.
const ELEMENT_COLOUR_00985BE0: u32 = 0x0098_5be0;
/// `BSTestObjects::CreateSphere` (`00c56680`, engine map; cdecl, four words).
const CREATE_SPHERE_00C56680: u32 = 0x00c5_6680;
/// `00508050` (unnamed in the engine map): a float of the target (ST0).
const TARGET_VALUE_00508050: u32 = 0x0050_8050;
/// `006aabd0` (navmeshsearchlos unit; `RET 4`): the target's position into
/// `out`.
const TARGET_POSITION_006AABD0: u32 = 0x006a_abd0;
/// `006e1370` (unnamed in the engine map; `RET 0x10`): the 0x50-byte LOS
/// map constructor (a point by value and a float).
const LOS_MAP_CTOR_006E1370: u32 = 0x006e_1370;
/// `00633c90` (unnamed in the engine map; `RET 4`): a smart pointer set to
/// the object it is given; `0045cec0`: its destructor.
const POINTER_INIT_00633C90: u32 = 0x0063_3c90;
const POINTER_RELEASE_0045CEC0: u32 = 0x0045_cec0;
/// `006e07a0` (unnamed in the engine map; `RET 4`): fills a LOS map from
/// the target.
const LOS_MAP_FILL_006E07A0: u32 = 0x006e_07a0;
/// `006e1800` (pathinglosmap unit): the LOS map's debug geometry.
const LOS_MAP_GEOMETRY_006E1800: u32 = 0x006e_1800;
/// `004391c0` (unnamed in the engine map): the 0x1C-byte property
/// constructor; `0049ed90` (`RET 4`) run on it with 1.
const PROPERTY_CTOR_004391C0: u32 = 0x0043_91c0;
const PROPERTY_SET_0049ED90: u32 = 0x0049_ed90;
/// `NiAVObject::AttachProperty` (`00439410`, engine map; `RET 4`).
const ATTACH_PROPERTY_00439410: u32 = 0x0043_9410;
/// `AddNoLightingPropertyRecurse` (`004b6e90`, engine map; cdecl).
const NO_LIGHTING_004B6E90: u32 = 0x004b_6e90;
/// `BSShaderManager::PrepareObject` (`00b57e30`, engine map; cdecl, three
/// words).
const PREPARE_OBJECT_00B57E30: u32 = 0x00b5_7e30;
/// `NiAVObject::UpdateProperties` (`00a5a040`, engine map).
const UPDATE_PROPERTIES_00A5A040: u32 = 0x00a5_a040;
/// `0043d410` (unnamed in the engine map; `RET 0xC`): a three-word
/// constructor.
const THREE_WORD_CTOR_0043D410: u32 = 0x0043_d410;
/// `00a59c60` (unnamed in the engine map; `RET 4`), run on an object with a
/// record of three zero words.
const ZERO_RECORD_STEP_00A59C60: u32 = 0x00a5_9c60;

/// The fields of the object `009878b0` runs on (offsets as the code uses
/// them): the debug root node pointer (+0x158), the arrays at +0x138 and
/// +0x120, the LOS target pointer (+0xD4), the LOS geometry pointer
/// (+0x130) and its rebuild flag (+0x134).
const DEBUG_ROOT: u32 = 0x158;
const TARGETS: u32 = 0x138;
const PAIRS: u32 = 0x120;
const LOS_TARGET: u32 = 0xd4;
const LOS_GEOMETRY: u32 = 0x130;
const LOS_REBUILD: u32 = 0x134;

fn float_bits(e: &mut Engine, address: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(address))
}

fn setting_on(e: &mut Engine, setting: u32) -> bool {
    let value = e.call(SETTING_ADDRESS_00408D60, &args![setting]).u32();
    e.mem.u8(value) != 0
}

fn root_object(e: &mut Engine, this: u32) -> u32 {
    e.call(FIRST_OF_00559450, &args![this + DEBUG_ROOT]).u32()
}

/// Makes the debug root node (`00aa13e0(0xAC)`, `00a5ecb0(block, 0)`; null
/// when the allocation fails) when the pointer has none.
fn ensure_root(e: &mut Engine, this: u32) {
    if root_object(e, this) == 0 {
        let block = e.call(ALLOCATE_00AA13E0, &args![0xacu32]).u32();
        let node = if block != 0 {
            e.call(NODE_CTOR_00A5ECB0, &args![block, 0u32]).u32()
        } else {
            0
        };
        e.call(POINTER_SET_0066B0D0, &args![this + DEBUG_ROOT, node]);
    }
}

/// Attaches `object` to the debug root (vtable slot +0xDC with 1).
fn attach(e: &mut Engine, this: u32, object: u32) {
    let root = root_object(e, this);
    e.vcall(root, SLOT_ATTACH, &args![object, 1u32]);
}

/// `00414430` into `out` with four floats (bits).
fn colour(e: &mut Engine, out: u32, parts: [u32; 4]) -> u32 {
    e.call(
        COLOUR_00414430,
        &args![out, parts[0], parts[1], parts[2], parts[3]],
    )
    .u32()
}

// Translated from 009878b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009878b0 - its FS:[0] exception frame (the unwind state of the allocations and the LOS map pointer) is not translated
/// Unnamed by its body (the engine map names it
/// `PathingDebugGeometry::MakeSmoothingData`, but its `this` is the object
/// the combat-group code passes, with fields up to +0x158; `RET 4`):
/// rebuilds the debug geometry. Detaches the old root (vtable slot +0xE8 of
/// its parent, or queued through `0087ac60` when `008c7aa0` says so) and
/// clears it; with `skip` set, or without the player's node, that is all.
/// Then, each part switched on by its setting: arrows and stacked rings to
/// each target of +0x138 from the members standing on it (in the player's
/// space, when the selected object `011dea24` belongs to `this`); a box per
/// actor (sized by its radius, coloured `005d8550`); spheres for the list
/// `00988750`, boxes for the pairs at +0x120 (coloured by their flags at
/// +9 and +0xA), and the LOS map's geometry (rebuilt when +0x134 is set).
/// The root gets a property, no lighting, is prepared, gets the LOS
/// geometry, is attached to the player's node and updated.
pub fn fn_009878b0(e: &mut Engine, this: Ptr, skip: u8) {
    let base = this.addr();
    e.with_stack(0x1c0, |e, locals| {
        let ebp = locals.addr() + 0x1c0;
        if root_object(e, base) != 0 {
            let root = root_object(e, base);
            if e.call(PARENT_009611E0, &args![root]).u32() != 0 {
                if e.call(MAIN_THREAD_TEST_008C7AA0, &args![]).u8() == 0 {
                    let root = root_object(e, base);
                    let parent = e.call(PARENT_009611E0, &args![root]).u32();
                    let root = root_object(e, base);
                    e.vcall(parent, SLOT_DETACH, &args![root]);
                } else {
                    let root = root_object(e, base);
                    let queue = e.call(TASK_QUEUE_004537B0, &args![]).u32();
                    e.call(QUEUE_DETACH_0087AC60, &args![queue, root]);
                }
            }
        }
        e.call(POINTER_SET_0066B0D0, &args![base + DEBUG_ROOT, 0u32]);
        if skip != 0 {
            return;
        }
        let player = e.global::<u32>(PLAYER_011DEA3C);
        if e.call(HOLDER_008D6F30, &args![player]).u32() == 0 {
            return;
        }
        let player = e.global::<u32>(PLAYER_011DEA3C);
        let holder = e.call(HOLDER_008D6F30, &args![player]).u32();
        if e.call(NODE_00524CF0, &args![holder]).u32() == 0 {
            return;
        }
        let player = e.global::<u32>(PLAYER_011DEA3C);
        let space = e.call(GET_SPACE_00575CA0, &args![player]).u32();
        let mut selected = 0u32;
        let candidate = e.global::<u32>(SELECTED_011DEA24);
        if candidate != 0 && e.vcall(candidate, SLOT_SELECTED_TEST, &args![]).u8() != 0 {
            let candidate = e.global::<u32>(SELECTED_011DEA24);
            if e.vcall(candidate, SLOT_SELECTED_GROUP, &args![]).u32() == base {
                selected = candidate;
            }
        }
        let half = float_bits(e, HALF_01016248);
        let zero = x87_store32(0.0);
        let one = x87_store32(1.0);

        if setting_on(e, SETTING_011F1824) && e.global::<u8>(SWITCH_011A4D34) == 0 && selected != 0
        {
            let mut index = 0u32;
            while index < e.call(ARRAY_SIZE_0044DDC0, &args![base + TARGETS]).u32() {
                let slot = e
                    .call(ELEMENT_006A7AD0, &args![base + TARGETS, index])
                    .u32();
                let target = e.mem.u32(slot);
                ensure_root(e, base);
                let ring_colour = ebp - 0x3c;
                colour(e, ring_colour, [half, zero, half, half]);
                let mut drawn = false;
                let position = e.call(IDENTITY_006815C0, &args![target]).u32();
                let at = ebp - 0x2c;
                copy_words(e, at, position, 3);
                let count = e.call(MEMBER_COUNT_00990890, &args![this]).u32();
                let mut member = 0u32;
                while member < count {
                    let entry = e.call(MEMBER_00988790, &args![this, member]).u32();
                    let reference = e.mem.u32(entry);
                    if e.mem.u32(entry + 0x10) == target
                        && e.call(GET_SPACE_00575CA0, &args![reference]).u32() == space
                    {
                        let reference = e.mem.u32(entry);
                        let from = e.vcall(reference, SLOT_POSITION, &args![]).u32();
                        copy_words(e, ebp - 0x80, from, 3);
                        colour(e, ebp - 0x60, [half, zero, half, half]);
                        colour(e, ebp - 0x74, [one, one, one, half]);
                        let size = float_bits(e, ARROW_SIZE_0101E570);
                        let arrow = e
                            .call(
                                CREATE_ARROW_00C5A0F0,
                                &args![ebp - 0x80, at, size, size, ebp - 0x60, ebp - 0x74],
                            )
                            .u32();
                        attach(e, base, arrow);
                        drawn = true;
                    }
                    member = member.wrapping_add(1);
                }
                if drawn {
                    let mut ring = 0u32;
                    while ring < 5 {
                        let size = float_bits(e, RING_SIZE_011A4D2C);
                        let shape = e
                            .call(CREATE_RING_00C58F00, &args![size, 0x24u32, ring_colour])
                            .u32();
                        let step: f64 = e.global(RING_STEP_0104F2F0);
                        let z = x87_add(x87_mul(f64::from(ring), step), x87_m32(e.mem.u32(at + 8)));
                        let z = x87_quiet_bits(x87_store32(z));
                        let x = x87_quiet_bits(e.mem.u32(at));
                        let y = x87_quiet_bits(e.mem.u32(at + 4));
                        e.call(SET_TRANSLATE_XYZ_004BC1F0, &args![shape, x, y, z]);
                        attach(e, base, shape);
                        ring = ring.wrapping_add(1);
                    }
                    let side = float_bits(e, MARKER_SIZE_0101E57C);
                    let height = float_bits(e, MARKER_HEIGHT_010311A8);
                    let marker = e
                        .call(
                            CREATE_BOX_00C58D30,
                            &args![side, side, height, 10u32, 10u32, ring_colour],
                        )
                        .u32();
                    e.call(SET_TRANSLATE_00440460, &args![marker, at]);
                    attach(e, base, marker);
                }
                index = index.wrapping_add(1);
            }
        }

        if setting_on(e, SETTING_011F1858) {
            ensure_root(e, base);
            let count = e.call(MEMBER_COUNT_00990890, &args![this]).u32();
            let mut index = 0u32;
            while index < count {
                let actor = e.call(ACTOR_00903600, &args![this, index]).u32();
                if e.call(GET_SPACE_00575CA0, &args![actor]).u32() == space {
                    let radius = e.call(GET_RADIUS_008BE1B0, &args![actor]).f64();
                    let scale: f64 = e.global(RADIUS_SCALE_01016FF0);
                    e.mem
                        .set_u32(ebp - 0xac, x87_store32(x87_mul(radius, scale)));
                    let first = e.mem.u32(base);
                    let source = e.call(GROUP_COLOUR_005D8550, &args![first]).u32();
                    let box_colour = ebp - 0xa4;
                    copy_words(e, box_colour, source, 4);
                    e.mem.set_u32(box_colour + 0xc, half);
                    let size = x87_quiet_bits(e.mem.u32(ebp - 0xac));
                    let height = float_bits(e, MARKER_SIZE_0101E57C);
                    let shape = e
                        .call(
                            CREATE_BOX_00C58D30,
                            &args![size, size, height, 10u32, 10u32, box_colour],
                        )
                        .u32();
                    let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
                    e.call(SET_TRANSLATE_00440460, &args![shape, position]);
                    attach(e, base, shape);
                }
                index = index.wrapping_add(1);
            }
        }

        let mut los_geometry = 0u32;
        if setting_on(e, SETTING_011F1814) && selected != 0 {
            ensure_root(e, base);
            let count = e.call(LIST_COUNT_00988750, &args![this]).u32();
            let mut index = 0u32;
            while index < count {
                let element = e.call(LIST_ELEMENT_00988770, &args![this, index]).u32();
                if e.call(ELEMENT_SPACE_0084E3A0, &args![element]).u32() == space {
                    let sphere_colour = ebp - 0xc8;
                    e.call(ELEMENT_COLOUR_00985BE0, &args![element, sphere_colour]);
                    let size = float_bits(e, SPHERE_SIZE_0101E340);
                    let sphere = e
                        .call(
                            CREATE_SPHERE_00C56680,
                            &args![size, 10u32, 10u32, sphere_colour],
                        )
                        .u32();
                    let position = e.call(IDENTITY_006815C0, &args![element]).u32();
                    e.call(SET_TRANSLATE_00440460, &args![sphere, position]);
                    attach(e, base, sphere);
                }
                index = index.wrapping_add(1);
            }
            let mut index = 0u32;
            while index < e.call(ARRAY_SIZE_0044DDC0, &args![base + PAIRS]).u32() {
                let pair = e.call(ELEMENT_006A7AF0, &args![base + PAIRS, index]).u32();
                let mut reference = 0u32;
                if e.mem.u32(pair) != 0
                    && e.call(GET_SPACE_00575CA0, &args![e.mem.u32(pair)]).u32() == space
                {
                    reference = e.mem.u32(pair);
                } else if e.mem.u32(pair + 4) != 0
                    && e.call(GET_SPACE_00575CA0, &args![e.mem.u32(pair + 4)])
                        .u32()
                        == space
                {
                    reference = e.mem.u32(pair + 4);
                }
                if reference == 0 {
                    index = index.wrapping_add(1);
                    continue;
                }
                let box_colour = ebp - 0xe4;
                colour(e, box_colour, [zero, zero, zero, zero]);
                let chosen = if e.mem.u8(pair + 9) != 0 {
                    colour(e, ebp - 0x138, [zero, zero, one, half])
                } else if e.mem.u8(pair + 0xa) != 0 {
                    colour(e, ebp - 0x148, [zero, one, zero, half])
                } else {
                    colour(e, ebp - 0x158, [one, one, zero, half])
                };
                copy_words(e, box_colour, chosen, 4);
                let position = e.vcall(reference, SLOT_POSITION, &args![]).u32();
                copy_words(e, ebp - 0xfc, position, 3);
                let side = float_bits(e, SPHERE_SIZE_0101E340);
                let height = float_bits(e, BOX_HEIGHT_01032ADC);
                let shape = e
                    .call(
                        CREATE_BOX_00C58D30,
                        &args![side, side, height, 10u32, 10u32, box_colour],
                    )
                    .u32();
                e.call(SET_TRANSLATE_00440460, &args![shape, ebp - 0xfc]);
                attach(e, base, shape);
                index = index.wrapping_add(1);
            }
            if e.call(FIRST_OF_00559450, &args![base + LOS_TARGET]).u32() != 0
                && e.mem.u8(base + LOS_REBUILD) != 0
                && e.call(GET_SPACE_00575CA0, &args![selected]).u32() == space
            {
                let block = e.call(ALLOCATE_00AA13E0, &args![0x50u32]).u32();
                let map = if block != 0 {
                    let target = e.call(FIRST_OF_00559450, &args![base + LOS_TARGET]).u32();
                    let float = x87_store32(e.call(TARGET_VALUE_00508050, &args![target]).f64());
                    let target = e.call(FIRST_OF_00559450, &args![base + LOS_TARGET]).u32();
                    let position = e
                        .call(TARGET_POSITION_006AABD0, &args![target, ebp - 0x16c])
                        .u32();
                    let x = e.mem.u32(position);
                    let y = e.mem.u32(position + 4);
                    let z = e.mem.u32(position + 8);
                    e.call(LOS_MAP_CTOR_006E1370, &args![block, x, y, z, float])
                        .u32()
                } else {
                    0
                };
                let pointer = ebp - 0x100;
                e.call(POINTER_INIT_00633C90, &args![pointer, map]);
                let target = e.call(FIRST_OF_00559450, &args![base + LOS_TARGET]).u32();
                e.call(LOS_MAP_FILL_006E07A0, &args![target, pointer]);
                let map = e.call(FIRST_OF_00559450, &args![pointer]).u32();
                let geometry = e.call(LOS_MAP_GEOMETRY_006E1800, &args![map]).u32();
                e.call(POINTER_SET_0066B0D0, &args![base + LOS_GEOMETRY, geometry]);
                e.mem.set_u8(base + LOS_REBUILD, 0);
                e.call(POINTER_RELEASE_0045CEC0, &args![pointer]);
            }
            los_geometry = e.call(FIRST_OF_00559450, &args![base + LOS_GEOMETRY]).u32();
        }

        if root_object(e, base) != 0 {
            let block = e.call(ALLOCATE_00AA13E0, &args![0x1cu32]).u32();
            let property = if block != 0 {
                e.call(PROPERTY_CTOR_004391C0, &args![block]).u32()
            } else {
                0
            };
            e.call(PROPERTY_SET_0049ED90, &args![property, 1u32]);
            let root = root_object(e, base);
            e.call(ATTACH_PROPERTY_00439410, &args![root, property]);
            let root = root_object(e, base);
            e.call(NO_LIGHTING_004B6E90, &args![root]);
            let root = root_object(e, base);
            e.call(PREPARE_OBJECT_00B57E30, &args![root, 0u32, 0u32]);
            if los_geometry != 0 {
                attach(e, base, los_geometry);
            }
            let player = e.global::<u32>(PLAYER_011DEA3C);
            let holder = e.call(HOLDER_008D6F30, &args![player]).u32();
            let node = e.call(NODE_00524CF0, &args![holder]).u32();
            let root = root_object(e, base);
            e.vcall(node, SLOT_ATTACH, &args![root, 1u32]);
            let root = root_object(e, base);
            e.call(UPDATE_PROPERTIES_00A5A040, &args![root]);
            let zero_record = ebp - 0x110;
            e.call(
                THREE_WORD_CTOR_0043D410,
                &args![zero_record, x87_store32(0.0), 0u32, 0u32],
            );
            let root = root_object(e, base);
            e.call(ZERO_RECORD_STEP_00A59C60, &args![root, zero_record]);
        }
    });
}

fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0098_78b0, fn_009878b0(Ptr, u8))]
}

#[cfg(test)]
mod n031_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn logged(e: &mut Engine, run: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        run(e);
        e.call_log.take().unwrap()
    }

    const DETACH: u32 = 0x0f40_0000;
    const NODE_ATTACH: u32 = 0x0f40_1000;

    /// An object with a debug root whose parent detaches it, a player
    /// node, the settings off.
    fn world(e: &mut Engine) -> (u32, u32, u32) {
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        stub(
            e,
            &[
                DETACH,
                NODE_ATTACH,
                POINTER_SET_0066B0D0,
                MAIN_THREAD_TEST_008C7AA0,
                GET_SPACE_00575CA0,
                PROPERTY_SET_0049ED90,
                ATTACH_PROPERTY_00439410,
                NO_LIGHTING_004B6E90,
                PREPARE_OBJECT_00B57E30,
                UPDATE_PROPERTIES_00A5A040,
                THREE_WORD_CTOR_0043D410,
                ZERO_RECORD_STEP_00A59C60,
            ],
        );
        let mut slots = vec![0u32; 0x40];
        slots[(SLOT_DETACH / 4) as usize] = DETACH;
        slots[(SLOT_ATTACH / 4) as usize] = NODE_ATTACH;
        e.put_vtable(0x0f41_0000, &slots);
        let parent = e.mem.alloc(4);
        e.mem.set_u32(parent, 0x0f41_0000);
        e.register_double(PARENT_009611E0, move |_, _| parent.into_ret());
        let root = e.mem.alloc(4);
        e.mem.set_u32(root, 0x0f41_0000);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + DEBUG_ROOT, root);
        let node = e.mem.alloc(4);
        e.mem.set_u32(node, 0x0f41_0000);
        e.register_double(HOLDER_008D6F30, move |_, _| 1u32.into_ret());
        e.register_double(NODE_00524CF0, move |_, _| node.into_ret());
        e.map(0x011d_e000, 0x1000);
        e.map(0x011a_4000, 0x1000);
        e.map(0x0101_0000, 0x10_0000);
        let setting = e.mem.alloc(4);
        e.register_double(SETTING_ADDRESS_00408D60, move |_, _| setting.into_ret());
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PROPERTY_CTOR_004391C0, |_, a| a[0].into_ret());
        (this, root, node)
    }

    #[test]
    fn at_009878b0_rebuilds_the_debug_root() {
        let mut e = Engine::new();
        let (this, root, node) = world(&mut e);
        // With `skip`: only the old root is detached and cleared.
        let log = logged(&mut e, |e| {
            e.call(0x0098_78b0, &args![this, 1u32]);
        });
        assert_eq!(calls_to(&log, DETACH).len(), 1);
        assert_eq!(calls_to(&log, DETACH)[0][1], root);
        assert_eq!(
            calls_to(&log, POINTER_SET_0066B0D0),
            vec![vec![this + DEBUG_ROOT, 0]]
        );
        assert!(calls_to(&log, GET_SPACE_00575CA0).is_empty());
        // Without `skip`, the settings off: the root (kept by the stub
        // pointer) gets its property and is attached to the player's node.
        let log = logged(&mut e, |e| {
            e.call(0x0098_78b0, &args![this, 0u32]);
        });
        assert_eq!(calls_to(&log, ATTACH_PROPERTY_00439410).len(), 1);
        assert_eq!(calls_to(&log, NODE_ATTACH), vec![vec![node, root, 1]]);
        assert_eq!(calls_to(&log, UPDATE_PROPERTIES_00A5A040), vec![vec![root]]);
        // Queued detachment off the main thread.
        e.register(MAIN_THREAD_TEST_008C7AA0, |_, _| 1u32.into_ret());
        e.register(TASK_QUEUE_004537B0, |_, _| 0x7777u32.into_ret());
        stub(&mut e, &[QUEUE_DETACH_0087AC60]);
        let log = logged(&mut e, |e| {
            e.call(0x0098_78b0, &args![this, 1u32]);
        });
        assert_eq!(
            calls_to(&log, QUEUE_DETACH_0087AC60),
            vec![vec![0x7777, root]]
        );
        assert!(calls_to(&log, DETACH).is_empty());
    }

    #[test]
    fn at_009878b0_draws_a_box_per_actor_in_the_space() {
        let mut e = Engine::new();
        let (this, _, _) = world(&mut e);
        let on = e.mem.alloc(4);
        e.mem.set_u8(on, 1);
        e.register_double(SETTING_ADDRESS_00408D60, move |_, a| {
            let address = if a[0] == SETTING_011F1858 { on } else { on + 1 };
            address.into_ret()
        });
        e.register(MEMBER_COUNT_00990890, |_, _| 2u32.into_ret());
        let mut slots = vec![0u32; 0x80];
        slots[(SLOT_POSITION / 4) as usize] = 0x0f42_1000;
        e.put_vtable(0x0f42_0000, &slots);
        let position = e.mem.alloc(0xc);
        e.register_double(0x0f42_1000, move |_, _| position.into_ret());
        let actors = [e.mem.alloc(4), e.mem.alloc(4)];
        for actor in actors {
            e.mem.set_u32(actor, 0x0f42_0000);
        }
        e.register_double(ACTOR_00903600, move |_, a| actors[a[1] as usize].into_ret());
        e.register(GET_RADIUS_008BE1B0, |_, _| 2.0f64.into_ret());
        e.set_global::<f64>(RADIUS_SCALE_01016FF0, 1.5);
        let colour = e.mem.alloc(0x10);
        e.register_double(GROUP_COLOUR_005D8550, move |_, _| colour.into_ret());
        e.register(CREATE_BOX_00C58D30, |_, _| 0x5555u32.into_ret());
        stub(&mut e, &[SET_TRANSLATE_00440460]);
        // The player (no object here) and the second actor share a space.
        let second = actors[1];
        e.register_double(GET_SPACE_00575CA0, move |_, a| {
            u32::from(a[0] == second || a[0] == 0).into_ret()
        });
        let log = logged(&mut e, |e| {
            e.call(0x0098_78b0, &args![this, 0u32]);
        });
        let boxes = calls_to(&log, CREATE_BOX_00C58D30);
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0][0], 3.0f32.to_bits());
    }
}
