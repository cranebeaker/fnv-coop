//! The game's Direct3D 9 device, for list lines that name its methods.
//!
//! A list names a device method by the address the port's platform layer gives it:
//! `DEVICE_CODE + 4 * slot` with `DEVICE_CODE = 0x7e020200`
//! (`crates/engine-core/src/units/platform/d3d9device.rs`), the slot being the method's index in
//! `IDirect3DDevice9`'s vtable (`SetTexture` 65 is `7e020304`). The port records its calls under
//! that address natively; here the exe's `Direct3DCreate9` import is redirected
//! ([`crate::iat`]), `IDirect3D9::CreateDevice` (slot 16) is patched in the object's vtable,
//! and when the device exists the listed slots of its vtable get entry stubs that jump to the
//! real methods ([`crate::hooks::install_device`]), so their calls are recorded as the port's
//! are, under the same addresses.

use nv_win as win;
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering::*};

/// The port's code address of the device's slot 0.
pub const DEVICE_CODE: u32 = 0x7e02_0200;
/// `IDirect3DDevice9` has 119 slots.
pub const DEVICE_SLOTS: u32 = 119;

pub static ORIG_CREATE9: AtomicUsize = AtomicUsize::new(0);
static ORIG_CREATE_DEVICE: AtomicUsize = AtomicUsize::new(0);

/// The device slot a list address names, if it is one.
pub fn device_slot(addr: u32) -> Option<u32> {
    let off = addr.checked_sub(DEVICE_CODE)?;
    (off % 4 == 0 && off / 4 < DEVICE_SLOTS).then_some(off / 4)
}

type Create9 = unsafe extern "system" fn(u32) -> *mut c_void;
type CreateDevice = unsafe extern "system" fn(
    *mut c_void,
    u32,
    u32,
    *mut c_void,
    u32,
    *mut c_void,
    *mut *mut c_void,
) -> i32;

pub unsafe extern "system" fn direct3d_create9(sdk: u32) -> *mut c_void {
    let orig: Create9 = std::mem::transmute(ORIG_CREATE9.load(Acquire));
    let d3d = orig(sdk);
    // Nothing is patched unless the list names device methods.
    if !d3d.is_null() && crate::hooks::device_hooks_pending() {
        let vtable = *(d3d as *mut *mut usize);
        let ok = crate::dinput::patch_slot(
            vtable,
            16,
            create_device as *const () as usize,
            &ORIG_CREATE_DEVICE,
        );
        crate::log(&format!("d3d: Direct3DCreate9; CreateDevice patched: {ok}"));
    }
    d3d
}

unsafe extern "system" fn create_device(
    this: *mut c_void,
    adapter: u32,
    kind: u32,
    window: *mut c_void,
    flags: u32,
    params: *mut c_void,
    out: *mut *mut c_void,
) -> i32 {
    let orig: CreateDevice = std::mem::transmute(ORIG_CREATE_DEVICE.load(Acquire));
    let hr = orig(this, adapter, kind, window, flags, params, out);
    if hr >= 0 && !out.is_null() && !(*out).is_null() {
        let vtable = *(*out as *mut *mut usize);
        let lines = crate::hooks::install_device(vtable);
        crate::log(&format!(
            "d3d: device created; {} device hooks",
            lines.len()
        ));
        for l in lines {
            crate::log(&l);
        }
    }
    hr
}

/// Replaces `vtable[slot]` with `ours`; returns the original.
///
/// # Safety
/// `vtable` must point at a COM vtable with at least `slot + 1` entries.
pub unsafe fn swap_slot(vtable: *mut usize, slot: u32, ours: usize) -> Option<usize> {
    let at = vtable.add(slot as usize);
    let mut old = 0u32;
    if win::VirtualProtect(at as *mut c_void, 4, win::PAGE_READWRITE, &mut old) == 0 {
        return None;
    }
    let original = *at;
    *at = ours;
    let mut ignored = 0;
    win::VirtualProtect(at as *mut c_void, 4, old, &mut ignored);
    Some(original)
}
