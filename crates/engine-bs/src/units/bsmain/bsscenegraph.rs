//! `BSMain/bsscenegraph.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSSceneGraph` is an `NiNode` (0xC0 bytes on PC) that owns a camera, a
//! culling process and a visible array. Its constructor builds the root
//! node, a camera node under it and (unless one is passed) a camera;
//! `SetCameraFOV` builds the camera's view frustum from a field of view, the
//! far distance and the viewport (or the renderer's target) proportions;
//! `SetViewDistanceBasedOnFrameRate` moves the far plane by the frame time
//! when the `bAutoViewDistance` setting is on.
//!
//! The settings the code reads are `Setting` objects whose value is the word
//! at +4 (their names are in the exe's static initializers, e.g.
//! `00fbbf80` for `fNearDistance:Display`). The code tests each setting's
//! address for null before reading it; the addresses are constants, so only
//! the non-null branch runs and the other one is left out.
//!
//! x87: values are computed in `f64` (the game's 53-bit precision) and
//! rounded to `f32` at each `float` store; every `FLD` of a `float` goes
//! through `x87_quiet`.
//!
//! Not translated: the C++ exception-unwinding frames (constructor,
//! destructor, `SetViewDistanceBasedOnFrameRate`) and the constructor's
//! stack cookie check; those functions carry the `Translation incomplete`
//! marker.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiAVObject, NiTArray};
use crate::units::nimain::nicamera::NiCamera;

/// Address of the `BSSceneGraph` vtable.
pub(crate) const BS_SCENE_GRAPH_VTABLE: u32 = 0x010c_2714;
/// Address of `BSSceneGraph`'s run-time type information.
pub(crate) const BS_SCENE_GRAPH_RTTI: u32 = 0x0120_3180;
/// Size of a `BSSceneGraph` on PC (the Xbox build is 0xF0).
pub(crate) const BS_SCENE_GRAPH_SIZE: u32 = 0xc0;

// Settings (the value word of each `Setting`).
/// `fNoLODFarDistanceMin:Display` (`float`; the setting is at `01203120`).
const NO_LOD_FAR_DISTANCE_MIN: u32 = 0x0120_3124;
/// `fNoLODFarDistanceMax:Display` (`float`; setting at `0120312c`).
const NO_LOD_FAR_DISTANCE_MAX: u32 = 0x0120_3130;
/// `fNoLODFarDistancePct:Display` (`float`; setting at `01203138`).
const NO_LOD_FAR_DISTANCE_PCT: u32 = 0x0120_313c;
/// `fNearDistance:Display` (`float`; setting at `01203144`).
const NEAR_DISTANCE: u32 = 0x0120_3148;
/// `fDefaultFOV:Display` (`float`; setting at `01203150`).
const DEFAULT_FOV: u32 = 0x0120_3154;
/// `bAutoViewDistance:Display` (byte; setting at `01203188`).
const AUTO_VIEW_DISTANCE: u32 = 0x0120_318c;
/// `iAutoViewLowFrameRate:Display` (`int`, default 20; setting at
/// `01203194`).
const AUTO_VIEW_LOW_FRAME_RATE: u32 = 0x0120_3198;
/// `iAutoViewHiFrameRate:Display` (`int`, default 40; setting at
/// `012031a0`).
const AUTO_VIEW_HI_FRAME_RATE: u32 = 0x0120_31a4;
/// `iAutoViewMinDistance:Display` (`int`, default 2000; setting at
/// `012031ac`).
const AUTO_VIEW_MIN_DISTANCE: u32 = 0x0120_31b0;

/// `SetViewDistanceBasedOnFrameRate`'s function-local `static float` (the
/// far distance it moves) and its initialization guard (bit 0).
const AUTO_VIEW_CURRENT_FAR: u32 = 0x0120_3118;
const AUTO_VIEW_CURRENT_FAR_INITIALIZED: u32 = 0x0120_311c;

// Constants read from the exe.
/// `20480.0` (`float`): the far distance of a menu scene graph.
const MENU_FAR_DISTANCE: u32 = 0x0108_3c64;
/// About `pi / 180` (`double`, `0.017453292` widened from a `float`).
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// `0.75` (`double`): the aspect the frustum's horizontal extent is scaled
/// against.
const ASPECT_0_75: u32 = 0x0101_de30;
/// `10.0` (`double`): the smallest far-distance change applied.
const AUTO_VIEW_STEP: u32 = 0x0102_0758;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;

// Other globals.
/// The global object `SetCameraFOV` asks for the target size (vtable slots
/// 0xC8 and 0xCC; `004dee70` and `004dee10` take it as `this`).
const RENDERER_GLOBAL: u32 = 0x011f_4748;
/// The memory manager instance (`this` of the allocator `00aa3e40`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The handle the constructor compares a new name string with before
/// releasing it (read from memory as the code does).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `"%sRoot Node"`, `"%sRoot CameraNode"` and `"%sRoot Camera"`.
const ROOT_NODE_FORMAT: u32 = 0x010c_2700;
const ROOT_CAMERA_NODE_FORMAT: u32 = 0x010c_26ec;
const ROOT_CAMERA_FORMAT: u32 = 0x010c_26dc;

// Callees.
/// The allocator `00aa13e0` (cdecl, size).
const ALLOCATE: u32 = 0x00aa_13e0;
/// The sized free `00aa1460` (cdecl, block, size).
const FREE_SIZED: u32 = 0x00aa_1460;
/// The memory manager's allocation (`00aa3e40`, thiscall, size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The memory manager's free (`00aa4060`, thiscall, block).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `00aa10f0` (cdecl, one word): called with the visible array's first
/// word before the array is freed.
const VISIBLE_ARRAY_RELEASE: u32 = 0x00aa_10f0;
/// The `NiNode` constructor (`00a5ecb0`, thiscall, initial child count).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The `NiNode` destructor body (`00a5e930`, thiscall).
const NI_NODE_DESTRUCT: u32 = 0x00a5_e930;
/// `NiNode::AttachChild(this, child, firstAvailable)` (Xbox PDB).
const NI_NODE_ATTACH_CHILD: u32 = 0x00a5_ed10;
/// The `NiCamera` constructor (`00a712f0`, thiscall, no arguments).
const NI_CAMERA_CONSTRUCT: u32 = 0x00a7_12f0;
/// `NiCamera::SetViewFrustum(this, frustum)` (Xbox PDB).
const NI_CAMERA_SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
/// `NiFrustum::NiFrustum(this, ortho)` (Xbox PDB).
const NI_FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
/// The update `00a59c60` (`this` = the object, then an update-data block):
/// calls the object's slot 0xA4 and its parent's slot 0xFC.
const NI_AV_OBJECT_UPDATE: u32 = 0x00a5_9c60;
/// The `NiPointer` assignment (`0066b0d0`: `this` = the holder,
/// then the new pointer; returns the holder).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// Copies the `NiFixedString` at the address passed into `this`'s name
/// (`00a5b950`).
const SET_NAME: u32 = 0x00a5_b950;
/// Makes an `NiFixedString` handle from a C string (`00a5b690`, cdecl).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// Releases the `NiFixedString` handle at the address passed (`004381d0`,
/// cdecl).
const FIXED_STRING_RELEASE: u32 = 0x0043_81d0;
/// `sprintf_s`-style formatting (`00406d00`, cdecl: buffer, size, format,
/// arguments; it calls `_vsprintf_s`).
const FORMAT: u32 = 0x0040_6d00;
/// The 200-byte object's constructor (`004a0eb0`, thiscall, one word).
const CULLER_CONSTRUCT: u32 = 0x004a_0eb0;
/// `00546780` (thiscall, one byte argument) that the constructor ends
/// with.
const SCENE_GRAPH_FINISH: u32 = 0x0054_6780;
/// Two getters of the renderer global (`this` = it): the first answers the
/// count `SetCameraFOV` scales the viewport height by, the second the one
/// it scales the width by.
const RENDERER_HEIGHT: u32 = 0x004d_ee70;
const RENDERER_WIDTH: u32 = 0x004d_ee10;
/// The CRT routine `00eca590` (double in `ST0`): the frustum uses it on the
/// half angle, as a tangent (identity not confirmed from the exe).
const TANGENT: u32 = 0x00ec_a590;
/// `fabs` (`00ec6cde`), double in, double out.
const FABS: u32 = 0x00ec_6cde;
/// Imported `InterlockedDecrement`.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Vtable slots (byte offsets).
const SLOT_DELETE_THIS: u32 = 0x04;
/// The culling process's slot called with 1 by the destructor (its
/// deleting destructor in the Xbox PDB).
const SLOT_CULLER_DELETE: u32 = 0x40;
/// `this`'s `GetFarDistance` (Xbox PDB slot name).
const SLOT_GET_FAR_DISTANCE: u32 = 0x100;
/// A node's slot taking a child and a flag (`NiNode::AttachChild` in the
/// Xbox PDB).
const SLOT_ATTACH_CHILD: u32 = 0xdc;
/// The renderer's slots: the render-target object (0xCC), compared with the
/// one slot 0xC8 answers.
const SLOT_RENDERER_TARGET: u32 = 0xcc;
const SLOT_RENDERER_DEFAULT_TARGET: u32 = 0xc8;
/// The render target's slot taking an index (`NiRenderTargetGroup::GetBuffer`
/// in the Xbox PDB); the buffer's words +8 and +0xC are read as width and
/// height.
const SLOT_TARGET_BUFFER: u32 = 0xb0;

/// `NiAVObject::m_kLocal.m_Rotate` on PC (nine floats).
const LOCAL_ROTATE: u32 = 0x34;
/// `NiFrustum` (Xbox PDB) field offsets: left, right, top, bottom, near,
/// far, ortho; 0x1C bytes.
const FRUSTUM_LEFT: u32 = 0x00;
const FRUSTUM_RIGHT: u32 = 0x04;
const FRUSTUM_TOP: u32 = 0x08;
const FRUSTUM_BOTTOM: u32 = 0x0c;
const FRUSTUM_NEAR: u32 = 0x10;
const FRUSTUM_FAR: u32 = 0x14;
const FRUSTUM_SIZE: u32 = 0x1c;

layout! {
    /// `BSSceneGraph` (Xbox PDB), 0xC0 bytes on PC (members 0x24 lower than
    /// in the Xbox PDB).
    pub struct BSSceneGraph: 0xc0 {
        /// `spCamera` (Xbox PDB): `NiPointer<NiCamera>`.
        0xAC spCamera: Ptr,
        /// `pVisArray` (Xbox PDB): `NiVisibleArray*`.
        0xB0 pVisArray: Ptr,
        /// `pCuller` (Xbox PDB): `BSCullingProcess*`.
        0xB4 pCuller: Ptr,
        /// `bMenuSceneGraph` (Xbox PDB).
        0xB8 bMenuSceneGraph: bool,
        /// `fCurrentFOV` (Xbox PDB).
        0xBC fCurrentFOV: f32,
    }
}

type Graph = Ptr<BSSceneGraph>;

/// A `float` from memory as `FLD` loads it.
fn load_float(e: &Engine, address: u32) -> f64 {
    x87_quiet(e.mem.f32(address)) as f64
}

/// Runs the update `00a59c60` on `object` with a zeroed update-data block
/// (a `float` time of 0 and five zero bytes), as the code builds it on its
/// stack.
fn update_with_zero_time(e: &mut Engine, object: u32) {
    e.with_stack(0x0c, |e, data| {
        e.mem.set_f32(data.addr(), 0.0);
        for byte in 4..9 {
            e.mem.set_u8(data.addr() + byte, 0);
        }
        e.call(NI_AV_OBJECT_UPDATE, &args![object, data]);
    });
}

/// Formats `"<prefix><suffix>"` into a 0x200-byte stack buffer and makes an
/// `NiFixedString` of it; returns the handle (the buffer is on the stack,
/// so the code's null-buffer branch never runs).
fn root_name(e: &mut Engine, format: u32, prefix: u32) -> u32 {
    e.with_stack(0x200, |e, buffer| {
        e.call(FORMAT, &args![buffer, 0x200u32, format, prefix]);
        e.call(FIXED_STRING_CREATE, &args![buffer]).u32()
    })
}

/// The tangent routine on a `float` half angle, rounded to `float`.
fn tangent(e: &mut Engine, half_angle: f32) -> f64 {
    (e.call(TANGENT, &args![x87_quiet(half_angle) as f64]).f64() as f32) as f64
}

// Translated from 00c517b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSceneGraph::BSSceneGraph` (Xbox PDB): builds the `NiNode`, sets the
/// vtable, clears the camera pointer and the visible array, stores the menu
/// flag, makes the 200-byte culling process (`004a0eb0`; null if the
/// allocation failed) and names the node `"<name>Root Node"`. A new
/// `NiNode` named `"<name>Root CameraNode"` is attached as a child.
///
/// With `camera` set, it becomes the scene graph's camera. Otherwise a new
/// `NiCamera` (`00a712f0`) named `"<name>Root Camera"` does, with the
/// viewport (0, 1, 1, 0), the rotation rows (0, 0, 1), (1, 0, 0), (0, 1, 0),
/// a minimum near-plane distance of 1, the far-to-near ratio
/// `GetFarDistance() / fNearDistance` and the field of view `fDefaultFOV`
/// (`SetCameraFOV` forced). Then the camera is put under the first child
/// (`00c51f20`), `00546780` runs with 1 and flag 0x2000 is set.
// Translation incomplete: 00c517b0 - C++ exception unwinding frame and the stack cookie check (00ec408c) are not translated.
pub fn bs_scene_graph_bs_scene_graph(
    e: &mut Engine,
    this: Graph,
    name: u32,
    menu_scene_graph: bool,
    camera: Ptr,
) -> Graph {
    e.call(NI_NODE_CONSTRUCT, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), BS_SCENE_GRAPH_VTABLE);
    // `NiPointer` initialised to null (its reference increment is skipped
    // since the pointer just stored is null).
    e.set(this, BSSceneGraph::spCamera, Ptr::NULL);
    e.set(this, BSSceneGraph::bMenuSceneGraph, menu_scene_graph);
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 200u32])
        .u32();
    let culler = if block == 0 {
        0
    } else {
        e.call(CULLER_CONSTRUCT, &args![block, 0u32]).u32()
    };
    e.set(this, BSSceneGraph::pCuller, Ptr::new(culler));
    e.set(this, BSSceneGraph::pVisArray, Ptr::NULL);

    // "<name>Root Node": the temporary handle is released inline unless it
    // is the value at `0109b220`.
    let handle = root_name(e, ROOT_NODE_FORMAT, name);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), handle);
        e.call(SET_NAME, &args![this, slot]);
        let handle = e.mem.u32(slot.addr());
        if handle != e.mem.u32(EMPTY_FIXED_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });

    // The camera node.
    let block = e.call(ALLOCATE, &args![0xacu32]).u32();
    let camera_node = if block == 0 {
        0
    } else {
        e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
    };
    let handle = root_name(e, ROOT_CAMERA_NODE_FORMAT, name);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), handle);
        e.call(SET_NAME, &args![camera_node, slot]);
        e.call(FIXED_STRING_RELEASE, &args![slot]);
    });
    e.call(NI_NODE_ATTACH_CHILD, &args![this, camera_node, 1u32]);

    let holder = this.addr() + 0xac;
    if !camera.is_null() {
        e.call(NI_POINTER_ASSIGN, &args![holder, camera]);
    } else {
        let block = e.call(ALLOCATE, &args![0x114u32]).u32();
        let new_camera = if block == 0 {
            0
        } else {
            e.call(NI_CAMERA_CONSTRUCT, &args![block]).u32()
        };
        e.call(NI_POINTER_ASSIGN, &args![holder, new_camera]);
        let handle = root_name(e, ROOT_CAMERA_FORMAT, name);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), handle);
            let camera = e.get(this, BSSceneGraph::spCamera);
            e.call(SET_NAME, &args![camera, slot]);
            e.call(FIXED_STRING_RELEASE, &args![slot]);
        });

        let camera = e.get(this, BSSceneGraph::spCamera).cast::<NiCamera>();
        e.set(camera, NiCamera::m_kPort_m_left, 0.0);
        e.set(camera, NiCamera::m_kPort_m_right, 1.0);
        e.set(camera, NiCamera::m_kPort_m_top, 1.0);
        e.set(camera, NiCamera::m_kPort_m_bottom, 0.0);
        let rotation: [f32; 9] = [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let camera = e.get(this, BSSceneGraph::spCamera);
        for (i, value) in rotation.iter().enumerate() {
            e.mem
                .set_f32(camera.addr() + LOCAL_ROTATE + 4 * i as u32, *value);
        }
        let camera = e.get(this, BSSceneGraph::spCamera).cast::<NiCamera>();
        e.set(camera, NiCamera::m_fMinNearPlaneDist, 1.0);
        let near = load_float(e, NEAR_DISTANCE);
        let far = bs_scene_graph_get_far_distance(e, this);
        e.set(
            camera,
            NiCamera::m_fMaxFarNearRatio,
            (far as f64 / near) as f32,
        );
        let fov = x87_quiet(e.mem.f32(DEFAULT_FOV));
        bs_scene_graph_set_camera_fov(e, this, fov, true, Ptr::NULL, false);
    }

    fn_00c51f20(e, this);
    e.call(SCENE_GRAPH_FINISH, &args![this, 1u32]);
    let flags_field = this.cast::<NiAVObject>();
    let flags = e.get(flags_field, NiAVObject::m_uFlags);
    e.set(flags_field, NiAVObject::m_uFlags, flags | 0x2000);
    this
}

// Translated from 00c51df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSceneGraph` destructor body: restores the vtable, clears the
/// camera pointer (`NiPointer` assignment of null), deletes the culling
/// process (its slot 0x40 with 1), frees the visible array (`00aa10f0` on
/// its first word, then the memory manager's free), releases whatever the
/// camera pointer still holds and runs the `NiNode` destructor.
// Translation incomplete: 00c51df0 - C++ exception unwinding frame is not translated.
pub fn fn_00c51df0(e: &mut Engine, this: Graph) {
    e.mem.set_u32(this.addr(), BS_SCENE_GRAPH_VTABLE);
    let holder = this.addr() + 0xac;
    e.call(NI_POINTER_ASSIGN, &args![holder, 0u32]);
    let culler = e.get(this, BSSceneGraph::pCuller);
    if !culler.is_null() {
        e.vcall(culler.addr(), SLOT_CULLER_DELETE, &args![1u32]);
    }
    let visible = e.get(this, BSSceneGraph::pVisArray);
    if !visible.is_null() {
        let first = e.mem.u32(visible.addr());
        e.call(VISIBLE_ARRAY_RELEASE, &args![first]);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, visible]);
    }
    let camera = e.mem.u32(holder);
    if camera != 0 && e.call(INTERLOCKED_DECREMENT, &args![camera + 4]).u32() == 0 {
        e.vcall(camera, SLOT_DELETE_THIS, &args![]);
    }
    e.call(NI_NODE_DESTRUCT, &args![this]);
}

// Translated from 00c51f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the camera under the scene graph's first child: when the camera's
/// parent is not child 0 (null when there are no children), child 0's slot
/// 0xDC gets the camera and 1, then the camera is updated with a zero time
/// (`00a59c60`).
pub fn fn_00c51f20(e: &mut Engine, this: Graph) {
    let camera = e.get(this, BSSceneGraph::spCamera);
    let parent = e.get(camera.cast::<NiAVObject>(), NiAVObject::m_pkParent);
    if parent != first_child(e, this) {
        let child = first_child(e, this);
        let camera = e.get(this, BSSceneGraph::spCamera);
        e.vcall(child, SLOT_ATTACH_CHILD, &args![camera, 1u32]);
        let camera = e.get(this, BSSceneGraph::spCamera);
        update_with_zero_time(e, camera.addr());
    }
}

/// Child 0 of the scene graph, or 0 when the child array is empty.
fn first_child(e: &Engine, this: Graph) -> u32 {
    let children = Ptr::<NiTArray>::new(this.addr() + 0x9c);
    if e.get(children, NiTArray::m_usSize) == 0 {
        0
    } else {
        let base = e.get(children, NiTArray::m_pBase);
        e.mem.u32(base)
    }
}

// Translated from 00c52020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSceneGraph::SetCameraFOV` (Xbox PDB): sets the view frustum of
/// `camera` (the scene graph's own when null) for the field of view `fov`
/// (degrees).
///
/// For the own camera, nothing is rebuilt when `force` is clear and `fov`
/// equals `fCurrentFOV` (only the LOD adjustment below is set);
/// otherwise `fCurrentFOV` becomes `fov`. The frustum's near plane is
/// `fNearDistance`, its far plane `GetFarDistance` (slot 0x100), and the
/// camera's far-to-near ratio is their quotient. The vertical and
/// horizontal extents are the viewport's (top - bottom, right - left),
/// scaled by the renderer's two counts when the renderer global is set; if
/// the renderer's target (slot 0xCC) is set and differs from slot 0xC8's,
/// its buffer 0 (slot 0xB0) gives them instead (words +0xC and +8). With
/// `ratio` their quotient and `scale` = 1 for the own camera of a menu
/// scene graph, `0.75 / (vertical / horizontal)` otherwise, and `t` the
/// tangent routine of half the angle in radians, the frustum is left
/// `-t * scale`, right `t * scale`, top `t * ratio * scale`, bottom
/// `-t * ratio * scale`. The camera is updated with a zero time. With
/// `set_lod_adjust`, the camera's `m_fLODAdjust` becomes
/// `fov / fDefaultFOV`.
pub fn bs_scene_graph_set_camera_fov(
    e: &mut Engine,
    this: Graph,
    fov: f32,
    force: bool,
    camera: Ptr,
    set_lod_adjust: bool,
) {
    let target = if camera.is_null() {
        e.get(this, BSSceneGraph::spCamera)
    } else {
        camera
    }
    .cast::<NiCamera>();
    let lod_adjust = |e: &mut Engine| {
        let default_fov = load_float(e, DEFAULT_FOV);
        let value = (fov as f64 / default_fov) as f32;
        e.set(target, NiCamera::m_fLODAdjust, value);
    };
    if camera.is_null() {
        let current = e.get(this, BSSceneGraph::fCurrentFOV);
        if !force && fov == current {
            if set_lod_adjust {
                lod_adjust(e);
            }
            return;
        }
        e.set(this, BSSceneGraph::fCurrentFOV, x87_quiet(fov));
    }
    let degrees_to_radians: f64 = e.global(DEGREES_TO_RADIANS);
    let half: f64 = e.global(HALF);
    let half_angle = ((x87_quiet(fov) as f64 * degrees_to_radians) * half) as f32;

    e.with_stack(FRUSTUM_SIZE, |e, frustum| {
        let frustum = frustum.addr();
        e.call(NI_FRUSTUM_CONSTRUCT, &args![frustum, 0u32]);
        let far = e.vcall(this.addr(), SLOT_GET_FAR_DISTANCE, &args![]).f32();
        e.mem.set_f32(frustum + FRUSTUM_FAR, far);
        let near = x87_quiet(e.mem.f32(NEAR_DISTANCE));
        e.mem.set_f32(frustum + FRUSTUM_NEAR, near);
        let ratio_far_near =
            (load_float(e, frustum + FRUSTUM_FAR) / load_float(e, frustum + FRUSTUM_NEAR)) as f32;
        e.set(target, NiCamera::m_fMaxFarNearRatio, ratio_far_near);

        let port_height =
            load_float(e, target.addr() + 0x108) - load_float(e, target.addr() + 0x10c);
        let port_width =
            load_float(e, target.addr() + 0x104) - load_float(e, target.addr() + 0x100);
        let renderer = e.mem.u32(RENDERER_GLOBAL);
        let (mut vertical, mut horizontal) = if renderer != 0 {
            let rows = e.call(RENDERER_HEIGHT, &args![renderer]).u32();
            let vertical = (rows as f64 * port_height) as f32;
            let columns = e.call(RENDERER_WIDTH, &args![renderer]).u32();
            (vertical, (columns as f64 * port_width) as f32)
        } else {
            (port_height as f32, port_width as f32)
        };
        if renderer != 0 && e.vcall(renderer, SLOT_RENDERER_TARGET, &args![]).u32() != 0 {
            let current = e.vcall(renderer, SLOT_RENDERER_TARGET, &args![]).u32();
            let default = e
                .vcall(renderer, SLOT_RENDERER_DEFAULT_TARGET, &args![])
                .u32();
            if current != default {
                let group = e.vcall(renderer, SLOT_RENDERER_TARGET, &args![]).u32();
                let buffer = e.vcall(group, SLOT_TARGET_BUFFER, &args![0u32]).u32();
                vertical = e.mem.u32(buffer + 0x0c) as f32;
                let group = e.vcall(renderer, SLOT_RENDERER_TARGET, &args![]).u32();
                let buffer = e.vcall(group, SLOT_TARGET_BUFFER, &args![0u32]).u32();
                horizontal = e.mem.u32(buffer + 0x08) as f32;
            }
        }
        let ratio = (vertical as f64 / horizontal as f64) as f32;
        let scale = if e.get(this, BSSceneGraph::bMenuSceneGraph) && camera.is_null() {
            1.0f32
        } else {
            let aspect: f64 = e.global(ASPECT_0_75);
            (aspect / (vertical as f64 / horizontal as f64)) as f32
        };
        let (ratio, scale) = (ratio as f64, scale as f64);

        let t = tangent(e, half_angle);
        e.mem.set_f32(frustum + FRUSTUM_LEFT, (-t * scale) as f32);
        let t = tangent(e, half_angle);
        e.mem.set_f32(frustum + FRUSTUM_RIGHT, (t * scale) as f32);
        let t = tangent(e, half_angle);
        e.mem
            .set_f32(frustum + FRUSTUM_BOTTOM, (-t * ratio * scale) as f32);
        let t = tangent(e, half_angle);
        e.mem
            .set_f32(frustum + FRUSTUM_TOP, (t * ratio * scale) as f32);
        e.call(NI_CAMERA_SET_VIEW_FRUSTUM, &args![target, frustum]);
    });
    update_with_zero_time(e, target.addr());
    if set_lod_adjust {
        lod_adjust(e);
    }
}

// Translated from 00c52450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSceneGraph::GetFarDistance` (Xbox PDB): 20480 for a menu scene graph,
/// otherwise `((pct - 0) / 1) * (max - min) + min` with the
/// `fNoLODFarDistancePct`, `Max` and `Min` settings.
pub fn bs_scene_graph_get_far_distance(e: &mut Engine, this: Graph) -> f32 {
    if e.get(this, BSSceneGraph::bMenuSceneGraph) {
        return x87_quiet(e.mem.f32(MENU_FAR_DISTANCE));
    }
    let max = load_float(e, NO_LOD_FAR_DISTANCE_MAX);
    let min = load_float(e, NO_LOD_FAR_DISTANCE_MIN);
    let pct = load_float(e, NO_LOD_FAR_DISTANCE_PCT);
    (((pct - 0.0) / 1.0) * (max - min) + min) as f32
}

// Translated from 00c52520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the camera's viewport to (0, 1, 1, 0) and reapplies the current
/// field of view (`SetCameraFOV` forced, own camera, no LOD adjustment).
pub fn fn_00c52520(e: &mut Engine, this: Graph) {
    let camera = e.get(this, BSSceneGraph::spCamera).cast::<NiCamera>();
    e.set(camera, NiCamera::m_kPort_m_left, 0.0);
    e.set(camera, NiCamera::m_kPort_m_right, 1.0);
    e.set(camera, NiCamera::m_kPort_m_top, 1.0);
    e.set(camera, NiCamera::m_kPort_m_bottom, 0.0);
    let fov = x87_quiet(e.get(this, BSSceneGraph::fCurrentFOV));
    bs_scene_graph_set_camera_fov(e, this, fov, true, Ptr::NULL, false);
}

/// The x87 test `FCOMP`, `TEST AH,0x41`, `JZ`: taken only when `a > b`
/// (ordered).
fn greater(a: f64, b: f64) -> bool {
    a > b
}

/// The x87 test `FCOMPP`, `TEST AH,5`, `JP` (not taken): `a < b`
/// (ordered).
fn less(a: f64, b: f64) -> bool {
    a < b
}

// Translated from 00c52590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSceneGraph::SetViewDistanceBasedOnFrameRate` (Xbox PDB): with
/// `bAutoViewDistance` on, moves the camera's far plane by the frame time
/// `frame_time` (seconds). A function-local `float` (`01203118`) holds the
/// current far distance, first set from `GetFarDistance` (slot 0x100).
///
/// Only when the frame time is above `1 / iAutoViewLowFrameRate` or below
/// `1 / iAutoViewHiFrameRate`: `s = ((1 / frame_time - low) * 1) / hi`;
/// the goal is `GetFarDistance` when `s > 1`, `(far - min) * s + min` when
/// `0 < s <= 1` and `iAutoViewMinDistance` otherwise. When the goal is more
/// than 10 away from the current distance, the current distance moves by
/// `(goal - current) * frame_time` and the camera's frustum is set again
/// with it as the far plane.
// Translation incomplete: 00c52590 - C++ exception unwinding frame is not translated.
pub fn bs_scene_graph_set_view_distance_based_on_frame_rate(
    e: &mut Engine,
    this: Graph,
    frame_time: f32,
) {
    if e.mem.u8(AUTO_VIEW_DISTANCE) == 0 {
        return;
    }
    let guard = e.mem.u32(AUTO_VIEW_CURRENT_FAR_INITIALIZED);
    if guard & 1 == 0 {
        e.mem.set_u32(AUTO_VIEW_CURRENT_FAR_INITIALIZED, guard | 1);
        let far = e.vcall(this.addr(), SLOT_GET_FAR_DISTANCE, &args![]).f32();
        e.mem.set_f32(AUTO_VIEW_CURRENT_FAR, far);
    }
    let time = x87_quiet(frame_time) as f64;
    let low = e.mem.i32(AUTO_VIEW_LOW_FRAME_RATE) as f64;
    if !greater(time, 1.0 / low) {
        let hi = e.mem.i32(AUTO_VIEW_HI_FRAME_RATE) as f64;
        // `TEST AH,5`, `JP`: continue only when `time < 1 / hi` (ordered).
        if !less(time, 1.0 / hi) {
            return;
        }
    }
    let far = e.vcall(this.addr(), SLOT_GET_FAR_DISTANCE, &args![]).f32();
    let min = e.mem.i32(AUTO_VIEW_MIN_DISTANCE) as f32;
    let mut goal = min;
    let one: f64 = e.global(ONE);
    let zero: f64 = e.global(ZERO);
    let low = e.mem.i32(AUTO_VIEW_LOW_FRAME_RATE) as f64;
    let hi = e.mem.i32(AUTO_VIEW_HI_FRAME_RATE) as f64;
    let s = ((((1.0 / time) - low) * one) / hi) as f32;
    if greater(s as f64, one) {
        goal = far;
    } else if greater(s as f64, zero) {
        goal = ((far as f64 - min as f64) * s as f64 + goal as f64) as f32;
    }
    let current = load_float(e, AUTO_VIEW_CURRENT_FAR);
    let difference = (goal as f64 - current) as f32;
    let distance = e.call(FABS, &args![difference as f64]).f64() as f32;
    let step: f64 = e.global(AUTO_VIEW_STEP);
    if !greater(distance as f64, step) {
        return;
    }
    let camera = e.get(this, BSSceneGraph::spCamera);
    e.with_stack(FRUSTUM_SIZE, |e, frustum| {
        for word in 0..7 {
            let value = e.mem.u32(camera.addr() + 0xdc + 4 * word);
            e.mem.set_u32(frustum.addr() + 4 * word, value);
        }
        let current = load_float(e, AUTO_VIEW_CURRENT_FAR);
        let moved = (difference as f64 * time + current) as f32;
        e.mem.set_f32(AUTO_VIEW_CURRENT_FAR, moved);
        e.mem
            .set_f32(frustum.addr() + FRUSTUM_FAR, x87_quiet(moved));
        e.call(NI_CAMERA_SET_VIEW_FRUSTUM, &args![camera, frustum]);
    });
}

// Translated from 00c52830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSceneGraph::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body and, when bit 0 of `flags` is set, frees the 0xC0 bytes.
/// Returns `this`.
pub fn bs_scene_graph_scalar_deleting_destructor(e: &mut Engine, this: Graph, flags: u32) -> Graph {
    fn_00c51df0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, BS_SCENE_GRAPH_SIZE]);
    }
    this
}

// Translated from 00c52870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSceneGraph::GetRTTI` (Xbox PDB): the address of the class's run-time
/// type information.
pub fn bs_scene_graph_get_rtti(_e: &mut Engine, _this: Graph) -> u32 {
    BS_SCENE_GRAPH_RTTI
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c517b0,
            bs_scene_graph_bs_scene_graph(Ptr<BSSceneGraph>, u32, bool, Ptr) -> Ptr<BSSceneGraph>
        ),
        entry!(0x00c51df0, fn_00c51df0(Ptr<BSSceneGraph>)),
        entry!(0x00c51f20, fn_00c51f20(Ptr<BSSceneGraph>)),
        entry!(
            0x00c52020,
            bs_scene_graph_set_camera_fov(Ptr<BSSceneGraph>, f32, bool, Ptr, bool)
        ),
        entry!(
            0x00c52450,
            bs_scene_graph_get_far_distance(Ptr<BSSceneGraph>) -> f32
        ),
        entry!(0x00c52520, fn_00c52520(Ptr<BSSceneGraph>)),
        entry!(
            0x00c52590,
            bs_scene_graph_set_view_distance_based_on_frame_rate(Ptr<BSSceneGraph>, f32)
        ),
        entry!(
            0x00c52830,
            bs_scene_graph_scalar_deleting_destructor(Ptr<BSSceneGraph>, u32) -> Ptr<BSSceneGraph>
        ),
        entry!(
            0x00c52870,
            bs_scene_graph_get_rtti(Ptr<BSSceneGraph>) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    /// Fake vtables, slot functions and a record area.
    const GRAPH_VTABLE: u32 = 0x7500_0000;
    const RENDERER_VTABLE: u32 = 0x7500_1000;
    const TARGET_VTABLE: u32 = 0x7500_2000;
    const OBJECT_VTABLE: u32 = 0x7500_3000;
    const FAR_DISTANCE: u32 = 0x7501_0000;
    const NOTHING: u32 = 0x7501_1000;
    const RENDERER_TARGET: u32 = 0x7501_2000;
    const RENDERER_DEFAULT_TARGET: u32 = 0x7501_3000;
    const TARGET_BUFFER: u32 = 0x7501_4000;
    /// Where the `SetViewFrustum` double copies the frustum it gets.
    const FRUSTUM_RECORD: u32 = 0x7600_0000;
    /// The buffer object the fake render target answers.
    const BUFFER: u32 = 0x7600_1000;
    /// Where the test keeps the render target the fake slot 0xCC answers.
    const TARGET_HOLDER: u32 = 0x7600_2000;
    /// The far distance the fake slot 0x100 answers.
    const SLOT_FAR: f64 = 1000.0;

    fn map_page(e: &mut Engine, address: u32) {
        e.map(address & !0xfff, 0x1000);
    }

    fn test_engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            NO_LOD_FAR_DISTANCE_MIN,
            MENU_FAR_DISTANCE,
            DEGREES_TO_RADIANS,
            HALF,
            ASPECT_0_75,
            AUTO_VIEW_STEP,
            ONE,
            RENDERER_GLOBAL,
            EMPTY_FIXED_STRING,
            FRUSTUM_RECORD,
            BUFFER,
            TARGET_HOLDER,
        ] {
            map_page(&mut e, address);
        }
        e.set_global(NO_LOD_FAR_DISTANCE_MIN, 100.0f32);
        e.set_global(NO_LOD_FAR_DISTANCE_MAX, 1100.0f32);
        e.set_global(NO_LOD_FAR_DISTANCE_PCT, 0.5f32);
        e.set_global(NEAR_DISTANCE, 10.0f32);
        e.set_global(DEFAULT_FOV, 75.0f32);
        e.set_global(AUTO_VIEW_LOW_FRAME_RATE, 20i32);
        e.set_global(AUTO_VIEW_HI_FRAME_RATE, 40i32);
        e.set_global(AUTO_VIEW_MIN_DISTANCE, 2000i32);
        e.set_global(MENU_FAR_DISTANCE, 20480.0f32);
        e.set_global(DEGREES_TO_RADIANS, f64::from_bits(0x3f91_df46_a000_0000));
        e.set_global(HALF, 0.5f64);
        e.set_global(ASPECT_0_75, 0.75f64);
        e.set_global(AUTO_VIEW_STEP, 10.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(ZERO, 0.0f64);
        for address in [
            NI_NODE_DESTRUCT,
            NI_NODE_ATTACH_CHILD,
            NI_FRUSTUM_CONSTRUCT,
            NI_AV_OBJECT_UPDATE,
            SET_NAME,
            FIXED_STRING_RELEASE,
            FORMAT,
            SCENE_GRAPH_FINISH,
            VISIBLE_ARRAY_RELEASE,
            MEMORY_DEALLOCATE,
            FREE_SIZED,
            NOTHING,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        for address in [NI_NODE_CONSTRUCT, NI_CAMERA_CONSTRUCT, CULLER_CONSTRUCT] {
            e.register(address, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        // A handle with a reference count 8 bytes before it.
        e.register(FIXED_STRING_CREATE, |e, _| {
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, 1);
            Ret {
                eax: block + 8,
                ..Ret::default()
            }
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        e.register(NI_CAMERA_SET_VIEW_FRUSTUM, |e, a| {
            for word in 0..7 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(FRUSTUM_RECORD + 4 * word, value);
            }
            Ret::default()
        });
        e.register(TANGENT, |_, a| Ret {
            st0: f64::take(a, &mut 0).tan(),
            ..Ret::default()
        });
        e.register(FABS, |_, a| Ret {
            st0: f64::take(a, &mut 0).abs(),
            ..Ret::default()
        });
        e.register(FAR_DISTANCE, |_, _| Ret {
            st0: SLOT_FAR,
            ..Ret::default()
        });
        e.register(RENDERER_HEIGHT, |_, _| Ret {
            eax: 600,
            ..Ret::default()
        });
        e.register(RENDERER_WIDTH, |_, _| Ret {
            eax: 800,
            ..Ret::default()
        });
        e.register(TARGET_BUFFER, |_, _| Ret {
            eax: BUFFER,
            ..Ret::default()
        });
        e.register(RENDERER_DEFAULT_TARGET, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(RENDERER_TARGET, |e, _| Ret {
            eax: e.mem.u32(TARGET_HOLDER),
            ..Ret::default()
        });
        let mut table = vec![NOTHING; 0x48];
        table[(SLOT_GET_FAR_DISTANCE / 4) as usize] = FAR_DISTANCE;
        e.put_vtable(GRAPH_VTABLE, &table);
        e.put_vtable(BS_SCENE_GRAPH_VTABLE, &table);
        let mut table = vec![NOTHING; 0x48];
        table[(SLOT_RENDERER_TARGET / 4) as usize] = RENDERER_TARGET;
        table[(SLOT_RENDERER_DEFAULT_TARGET / 4) as usize] = RENDERER_DEFAULT_TARGET;
        e.put_vtable(RENDERER_VTABLE, &table);
        let mut table = vec![NOTHING; 0x48];
        table[(SLOT_TARGET_BUFFER / 4) as usize] = TARGET_BUFFER;
        e.put_vtable(TARGET_VTABLE, &table);
        e.put_vtable(OBJECT_VTABLE, &[NOTHING; 0x48]);
        e
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A scene graph with the fake vtable and a camera with viewport
    /// (0, 1, 1, 0).
    fn graph_with_camera(e: &mut Engine) -> (Graph, Ptr<NiCamera>) {
        let graph: Graph = e.new_object();
        e.mem.set_u32(graph.addr(), GRAPH_VTABLE);
        let camera: Ptr<NiCamera> = e.new_object();
        e.mem.set_u32(camera.addr(), OBJECT_VTABLE);
        e.set(camera, NiCamera::m_kPort_m_right, 1.0);
        e.set(camera, NiCamera::m_kPort_m_top, 1.0);
        e.set(graph, BSSceneGraph::spCamera, camera.cast());
        (graph, camera)
    }

    fn recorded_frustum(e: &Engine) -> [f32; 6] {
        let mut out = [0.0; 6];
        for (i, value) in out.iter_mut().enumerate() {
            *value = e.mem.f32(FRUSTUM_RECORD + 4 * i as u32);
        }
        out
    }

    /// The frustum sides `SetCameraFOV` builds for `fov`, `ratio` and
    /// `scale` (left, right, top, bottom).
    fn expected_frustum(fov: f32, ratio: f32, scale: f32) -> [f32; 4] {
        let half = ((fov as f64 * f64::from_bits(0x3f91_df46_a000_0000)) * 0.5) as f32;
        let t = ((half as f64).tan() as f32) as f64;
        let (ratio, scale) = (ratio as f64, scale as f64);
        [
            (-t * scale) as f32,
            (t * scale) as f32,
            (t * ratio * scale) as f32,
            (-t * ratio * scale) as f32,
        ]
    }

    #[test]
    fn get_far_distance_interpolates_or_uses_the_menu_value() {
        let mut e = test_engine();
        let graph: Graph = e.new_object();
        assert_eq!(e.call(0x00c5_2450, &args![graph]).f32(), 600.0);
        e.set(graph, BSSceneGraph::bMenuSceneGraph, true);
        assert_eq!(e.call(0x00c5_2450, &args![graph]).f32(), 20480.0);
    }

    #[test]
    fn set_camera_fov_skips_an_unchanged_fov() {
        let mut e = test_engine();
        let (graph, camera) = graph_with_camera(&mut e);
        e.set(graph, BSSceneGraph::fCurrentFOV, 90.0);
        e.call_log = Some(vec![]);
        e.call(0x00c5_2020, &args![graph, 90.0f32, false, 0u32, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(
            e.get(camera, NiCamera::m_fLODAdjust),
            (90.0f64 / 75.0) as f32
        );

        // Without the LOD flag, nothing at all.
        e.set(camera, NiCamera::m_fLODAdjust, 0.0);
        e.call(0x00c5_2020, &args![graph, 90.0f32, false, 0u32, false]);
        assert_eq!(e.get(camera, NiCamera::m_fLODAdjust), 0.0);
    }

    #[test]
    fn set_camera_fov_builds_the_frustum_from_the_viewport() {
        let mut e = test_engine();
        let (graph, camera) = graph_with_camera(&mut e);
        e.set(graph, BSSceneGraph::fCurrentFOV, 90.0);
        e.call_log = Some(vec![]);
        e.call(0x00c5_2020, &args![graph, 70.0f32, false, 0u32, false]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(graph, BSSceneGraph::fCurrentFOV), 70.0);
        assert_eq!(e.get(camera, NiCamera::m_fMaxFarNearRatio), 100.0);
        let frustum = recorded_frustum(&e);
        assert_eq!(frustum[..4], expected_frustum(70.0, 1.0, 0.75));
        assert_eq!(frustum[4], 10.0);
        assert_eq!(frustum[5], 1000.0);
        assert_eq!(calls_to(&log, TANGENT).len(), 4);
        assert_eq!(calls_to(&log, NI_FRUSTUM_CONSTRUCT)[0][1], 0);
        assert_eq!(
            calls_to(&log, NI_CAMERA_SET_VIEW_FRUSTUM)[0][0],
            camera.addr()
        );
        assert_eq!(calls_to(&log, NI_AV_OBJECT_UPDATE)[0][0], camera.addr());
        assert_eq!(e.get(camera, NiCamera::m_fLODAdjust), 0.0);
    }

    #[test]
    fn set_camera_fov_on_a_menu_graph_keeps_scale_one() {
        let mut e = test_engine();
        let (graph, camera) = graph_with_camera(&mut e);
        e.set(graph, BSSceneGraph::bMenuSceneGraph, true);
        e.set(camera, NiCamera::m_kPort_m_right, 2.0);
        e.call(0x00c5_2020, &args![graph, 60.0f32, true, 0u32, true]);
        let frustum = recorded_frustum(&e);
        assert_eq!(frustum[..4], expected_frustum(60.0, 0.5, 1.0));
        assert_eq!(
            e.get(camera, NiCamera::m_fLODAdjust),
            (60.0f64 / 75.0) as f32
        );
    }

    #[test]
    fn set_camera_fov_on_another_camera_leaves_the_current_fov() {
        let mut e = test_engine();
        let (graph, _) = graph_with_camera(&mut e);
        e.set(graph, BSSceneGraph::bMenuSceneGraph, true);
        e.set(graph, BSSceneGraph::fCurrentFOV, 90.0);
        let other: Ptr<NiCamera> = e.new_object();
        e.set(other, NiCamera::m_kPort_m_right, 1.0);
        e.set(other, NiCamera::m_kPort_m_top, 1.0);
        e.call(0x00c5_2020, &args![graph, 90.0f32, false, other, false]);
        assert_eq!(e.get(graph, BSSceneGraph::fCurrentFOV), 90.0);
        assert_eq!(e.get(other, NiCamera::m_fMaxFarNearRatio), 100.0);
        // A camera passed in is never a menu scene graph's own: scale 0.75.
        assert_eq!(recorded_frustum(&e)[..4], expected_frustum(90.0, 1.0, 0.75));
    }

    #[test]
    fn set_camera_fov_uses_the_renderer_counts_and_target() {
        let mut e = test_engine();
        let (graph, _) = graph_with_camera(&mut e);
        let renderer = e.mem.alloc(8);
        e.mem.set_u32(renderer, RENDERER_VTABLE);
        e.set_global(RENDERER_GLOBAL, renderer);
        // No target: the counts scale the viewport.
        e.call(0x00c5_2020, &args![graph, 80.0f32, true, 0u32, false]);
        let ratio = (600.0f64 / 800.0) as f32;
        let scale = (0.75 / (600.0f64 / 800.0)) as f32;
        assert_eq!(
            recorded_frustum(&e)[..4],
            expected_frustum(80.0, ratio, scale)
        );

        // A target different from the default: its buffer's size wins.
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, TARGET_VTABLE);
        e.mem.set_u32(BUFFER + 8, 1280);
        e.mem.set_u32(BUFFER + 0x0c, 720);
        e.mem.set_u32(TARGET_HOLDER, target);
        e.call_log = Some(vec![]);
        e.call(0x00c5_2020, &args![graph, 80.0f32, true, 0u32, false]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RENDERER_TARGET).len(), 4);
        assert_eq!(calls_to(&log, TARGET_BUFFER), vec![vec![target, 0]; 2]);
        let ratio = (720.0f64 / 1280.0) as f32;
        let scale = (0.75 / (720.0f64 / 1280.0)) as f32;
        assert_eq!(
            recorded_frustum(&e)[..4],
            expected_frustum(80.0, ratio, scale)
        );
    }

    #[test]
    fn reset_viewport_reapplies_the_current_fov() {
        let mut e = test_engine();
        let (graph, camera) = graph_with_camera(&mut e);
        e.set(camera, NiCamera::m_kPort_m_left, 5.0);
        e.set(camera, NiCamera::m_kPort_m_bottom, 5.0);
        e.set(graph, BSSceneGraph::fCurrentFOV, 65.0);
        e.call(0x00c5_2520, &args![graph]);
        assert_eq!(e.get(camera, NiCamera::m_kPort_m_left), 0.0);
        assert_eq!(e.get(camera, NiCamera::m_kPort_m_right), 1.0);
        assert_eq!(e.get(camera, NiCamera::m_kPort_m_top), 1.0);
        assert_eq!(e.get(camera, NiCamera::m_kPort_m_bottom), 0.0);
        assert_eq!(recorded_frustum(&e)[..4], expected_frustum(65.0, 1.0, 0.75));
    }

    #[test]
    fn camera_goes_under_the_first_child_unless_it_is_there() {
        let mut e = test_engine();
        let (graph, camera) = graph_with_camera(&mut e);
        let child = e.mem.alloc(8);
        e.mem.set_u32(child, OBJECT_VTABLE);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, child);
        e.mem.set_u32(graph.addr() + 0xa0, array);
        e.mem.set_u16(graph.addr() + 0xa6, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c5_1f20, &args![graph]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, NOTHING), vec![vec![child, camera.addr(), 1]]);
        assert_eq!(calls_to(&log, NI_AV_OBJECT_UPDATE)[0][0], camera.addr());

        e.mem.set_u32(camera.addr() + 0x18, child);
        e.call_log = Some(vec![]);
        e.call(0x00c5_1f20, &args![graph]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn constructor_builds_root_camera_node_and_camera() {
        let mut e = test_engine();
        let graph: Graph = e.new_object();
        e.call_log = Some(vec![]);
        let back = e
            .call(0x00c5_17b0, &args![graph, 0x4444u32, false, 0u32])
            .ptr::<BSSceneGraph>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, graph);
        assert_eq!(e.mem.u32(graph.addr()), BS_SCENE_GRAPH_VTABLE);
        assert!(!e.get(graph, BSSceneGraph::bMenuSceneGraph));
        assert_eq!(
            calls_to(&log, MEMORY_ALLOCATE)[0],
            vec![MEMORY_MANAGER, 200]
        );
        let culler = e.get(graph, BSSceneGraph::pCuller);
        assert_eq!(
            calls_to(&log, CULLER_CONSTRUCT),
            vec![vec![culler.addr(), 0]]
        );
        assert!(e.get(graph, BSSceneGraph::pVisArray).is_null());
        let formats: Vec<u32> = calls_to(&log, FORMAT).iter().map(|w| w[2]).collect();
        assert_eq!(
            formats,
            vec![
                ROOT_NODE_FORMAT,
                ROOT_CAMERA_NODE_FORMAT,
                ROOT_CAMERA_FORMAT
            ]
        );
        assert!(calls_to(&log, FORMAT)
            .iter()
            .all(|w| w[1] == 0x200 && w[3] == 0x4444));
        let camera = e.get(graph, BSSceneGraph::spCamera).cast::<NiCamera>();
        let names: Vec<u32> = calls_to(&log, SET_NAME).iter().map(|w| w[0]).collect();
        let attach = calls_to(&log, NI_NODE_ATTACH_CHILD);
        assert_eq!(attach.len(), 1);
        let camera_node = attach[0][1];
        assert_eq!(attach[0], vec![graph.addr(), camera_node, 1]);
        assert_eq!(names, vec![graph.addr(), camera_node, camera.addr()]);
        // The root name's temporary handle is released inline.
        assert_eq!(calls_to(&log, INTERLOCKED_DECREMENT).len(), 1);
        assert_eq!(calls_to(&log, FIXED_STRING_RELEASE).len(), 2);
        assert_eq!(
            calls_to(&log, NI_CAMERA_CONSTRUCT),
            vec![vec![camera.addr()]]
        );
        assert_eq!(e.get(camera, NiCamera::m_kPort_m_left), 0.0);
        assert_eq!(e.get(camera, NiCamera::m_kPort_m_right), 1.0);
        assert_eq!(e.get(camera, NiCamera::m_kPort_m_top), 1.0);
        assert_eq!(e.get(camera, NiCamera::m_kPort_m_bottom), 0.0);
        let rotation: Vec<f32> = (0..9)
            .map(|i| e.mem.f32(camera.addr() + LOCAL_ROTATE + 4 * i))
            .collect();
        assert_eq!(rotation, vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.get(camera, NiCamera::m_fMinNearPlaneDist), 1.0);
        // SetCameraFOV ran after the ratio (from GetFarDistance) was set and
        // replaced it with slot 0x100's far distance over the near one.
        assert_eq!(e.get(camera, NiCamera::m_fMaxFarNearRatio), 100.0);
        assert_eq!(e.get(graph, BSSceneGraph::fCurrentFOV), 75.0);
        assert_eq!(
            calls_to(&log, SCENE_GRAPH_FINISH),
            vec![vec![graph.addr(), 1]]
        );
        assert_eq!(
            e.get(graph.cast::<NiAVObject>(), NiAVObject::m_uFlags) & 0x2000,
            0x2000
        );
    }

    #[test]
    fn constructor_takes_a_given_camera() {
        let mut e = test_engine();
        let graph: Graph = e.new_object();
        let camera = e.mem.alloc(0x114);
        e.call_log = Some(vec![]);
        e.call(0x00c5_17b0, &args![graph, 0x4444u32, true, camera]);
        let log = e.call_log.take().unwrap();
        assert!(e.get(graph, BSSceneGraph::bMenuSceneGraph));
        assert_eq!(e.get(graph, BSSceneGraph::spCamera).addr(), camera);
        assert!(calls_to(&log, NI_CAMERA_CONSTRUCT).is_empty());
        assert!(calls_to(&log, NI_CAMERA_SET_VIEW_FRUSTUM).is_empty());
        assert_eq!(calls_to(&log, SET_NAME).len(), 2);
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN),
            vec![vec![graph.addr() + 0xac, camera]]
        );
    }

    #[test]
    fn destructor_releases_everything() {
        let mut e = test_engine();
        let graph: Graph = e.new_object();
        let culler = e.mem.alloc(8);
        e.mem.set_u32(culler, OBJECT_VTABLE);
        e.set(graph, BSSceneGraph::pCuller, Ptr::new(culler));
        let visible = e.mem.alloc(8);
        e.mem.set_u32(visible, 0x9876);
        e.set(graph, BSSceneGraph::pVisArray, Ptr::new(visible));
        e.set(graph, BSSceneGraph::spCamera, Ptr::new(0x5555));
        e.call_log = Some(vec![]);
        e.call(0x00c5_1df0, &args![graph]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(graph.addr()), BS_SCENE_GRAPH_VTABLE);
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN),
            vec![vec![graph.addr() + 0xac, 0]]
        );
        assert_eq!(calls_to(&log, NOTHING), vec![vec![culler, 1]]);
        assert_eq!(calls_to(&log, VISIBLE_ARRAY_RELEASE), vec![vec![0x9876]]);
        assert_eq!(
            calls_to(&log, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, visible]]
        );
        // The pointer was cleared first: nothing left to release.
        assert!(calls_to(&log, INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(calls_to(&log, NI_NODE_DESTRUCT), vec![vec![graph.addr()]]);
    }

    #[test]
    fn destructor_releases_a_camera_the_assignment_left() {
        let mut e = test_engine();
        e.register(NI_POINTER_ASSIGN, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let graph: Graph = e.new_object();
        let camera = e.mem.alloc(8);
        e.mem.set_u32(camera, OBJECT_VTABLE);
        e.mem.set_u32(camera + 4, 1);
        e.set(graph, BSSceneGraph::spCamera, Ptr::new(camera));
        e.call_log = Some(vec![]);
        e.call(0x00c5_1df0, &args![graph]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, INTERLOCKED_DECREMENT),
            vec![vec![camera + 4]]
        );
        assert_eq!(calls_to(&log, NOTHING), vec![vec![camera]]);
        assert!(calls_to(&log, VISIBLE_ARRAY_RELEASE).is_empty());
    }

    #[test]
    fn auto_view_distance_does_nothing_when_off() {
        let mut e = test_engine();
        let (graph, _) = graph_with_camera(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c5_2590, &args![graph, 0.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(e.mem.u32(AUTO_VIEW_CURRENT_FAR_INITIALIZED), 0);
    }

    #[test]
    fn auto_view_distance_initialises_and_keeps_steady_frame_times() {
        let mut e = test_engine();
        let (graph, _) = graph_with_camera(&mut e);
        e.set_global(AUTO_VIEW_DISTANCE, 1u8);
        // 1/30 s lies between 1/40 and 1/20: only the static is set.
        e.call_log = Some(vec![]);
        e.call(0x00c5_2590, &args![graph, 1.0f32 / 30.0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(AUTO_VIEW_CURRENT_FAR_INITIALIZED), 1);
        assert_eq!(e.mem.f32(AUTO_VIEW_CURRENT_FAR), 1000.0);
        assert_eq!(calls_to(&log, FAR_DISTANCE).len(), 1);
        assert!(calls_to(&log, NI_CAMERA_SET_VIEW_FRUSTUM).is_empty());
    }

    #[test]
    fn auto_view_distance_moves_the_far_plane_on_slow_frames() {
        let mut e = test_engine();
        let (graph, camera) = graph_with_camera(&mut e);
        e.set_global(AUTO_VIEW_DISTANCE, 1u8);
        e.set_global(AUTO_VIEW_CURRENT_FAR_INITIALIZED, 1u32);
        e.set_global(AUTO_VIEW_CURRENT_FAR, 5000.0f32);
        for i in 0..7 {
            e.mem.set_f32(camera.addr() + 0xdc + 4 * i, i as f32);
        }
        // 0.1 s: s = (10 - 20) / 40 < 0, the goal is the minimum (2000).
        e.call(0x00c5_2590, &args![graph, 0.1f32]);
        let difference = (2000.0f64 - 5000.0) as f32;
        let moved = (difference as f64 * 0.1f32 as f64 + 5000.0) as f32;
        assert_eq!(e.mem.f32(AUTO_VIEW_CURRENT_FAR), moved);
        let frustum = recorded_frustum(&e);
        assert_eq!(frustum[..5], [0.0, 1.0, 2.0, 3.0, 4.0]);
        assert_eq!(frustum[5], moved);
    }

    #[test]
    fn auto_view_distance_on_fast_frames_aims_at_the_far_distance() {
        let mut e = test_engine();
        let (graph, _) = graph_with_camera(&mut e);
        e.set_global(AUTO_VIEW_DISTANCE, 1u8);
        e.set_global(AUTO_VIEW_CURRENT_FAR_INITIALIZED, 1u32);
        e.set_global(AUTO_VIEW_CURRENT_FAR, 500.0f32);
        // 0.01 s: s = (100 - 20) / 40 = 2 > 1, the goal is slot 0x100 (1000).
        e.call(0x00c5_2590, &args![graph, 0.01f32]);
        let moved = ((1000.0f64 - 500.0) * 0.01f32 as f64 + 500.0) as f32;
        assert_eq!(e.mem.f32(AUTO_VIEW_CURRENT_FAR), moved);

        // s between 0 and 1: the far distance is interpolated. 1/45 s gives
        // s = (45 - 20) / 40.
        e.set_global(AUTO_VIEW_CURRENT_FAR, 0.0f32);
        let time = 1.0f32 / 45.0;
        e.call(0x00c5_2590, &args![graph, time]);
        let s = (((1.0 / time as f64) - 20.0) * 1.0 / 40.0) as f32;
        let goal = ((1000.0f64 - 2000.0) * s as f64 + 2000.0) as f32;
        let moved = (goal as f64 * time as f64) as f32;
        assert_eq!(e.mem.f32(AUTO_VIEW_CURRENT_FAR), moved);
    }

    #[test]
    fn auto_view_distance_ignores_small_changes() {
        let mut e = test_engine();
        let (graph, _) = graph_with_camera(&mut e);
        e.set_global(AUTO_VIEW_DISTANCE, 1u8);
        e.set_global(AUTO_VIEW_CURRENT_FAR_INITIALIZED, 1u32);
        e.set_global(AUTO_VIEW_CURRENT_FAR, 995.0f32);
        e.call_log = Some(vec![]);
        e.call(0x00c5_2590, &args![graph, 0.01f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FABS).len(), 1);
        assert!(calls_to(&log, NI_CAMERA_SET_VIEW_FRUSTUM).is_empty());
        assert_eq!(e.mem.f32(AUTO_VIEW_CURRENT_FAR), 995.0);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = test_engine();
        let graph: Graph = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c5_2830, &args![graph, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FREE_SIZED).is_empty());
        assert_eq!(calls_to(&log, NI_NODE_DESTRUCT).len(), 1);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x00c5_2830, &args![graph, 1u32])
            .ptr::<BSSceneGraph>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, graph);
        assert_eq!(calls_to(&log, FREE_SIZED), vec![vec![graph.addr(), 0xc0]]);
    }

    #[test]
    fn get_rtti_is_the_class_block() {
        let mut e = test_engine();
        assert_eq!(e.call(0x00c5_2870, &args![0u32]).u32(), 0x0120_3180);
    }
}
