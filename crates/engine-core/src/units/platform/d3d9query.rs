//! The platform's Direct3D 9 queries (ADR-0007, ADR-0008): `IDirect3DDevice9::CreateQuery` and
//! the `IDirect3DQuery9` it makes, as the retail runtime answered on this machine
//! (`d3d9-capture device extra`, 2026-10-27, a windowed HAL device):
//! - `CreateQuery(type, NULL)` says whether the device takes the type: the machine table's
//!   answer (`d3d9-capture device queries`); a type the table does not have stops the run.
//! - An event query (8) or an occlusion query (9): `GetType` the type, `GetDataSize` 4.
//!   `Issue(D3DISSUE_BEGIN)` and `Issue(D3DISSUE_END)` answer `D3D_OK` in any order (also
//!   `BEGIN` twice, `END` first).
//! - `GetData(data, size, flags)` before any `Issue` answers `S_OK` and writes `dddddddd`;
//!   after `BEGIN` without `END`, `S_FALSE` (nothing written); after `END`, `S_FALSE` until the
//!   GPU is done, then `S_OK` with the result written as `size` bytes (zero-extended past 4; a
//!   null data with size 0 answers `S_OK`).
//!
//! When the work is done is the GPU's: with `NV_D3D9=real` each query has a real counterpart and
//! `GetData` after `END` answers what the real query answers (its result is the real GPU's
//! count of drawn pixels, or its event). Without it nothing is drawn: an event query is done at
//! once (`S_OK`, TRUE), and an occlusion query's result is not provided (the run stops).

use super::d3d9::{CODE_BASE, D3DERR_INVALIDCALL, D3D_OK, E_NOINTERFACE};
use super::d3d9device::{device_add_ref, device_release, guid, iid_is, IID_IUNKNOWN};
use crate::{Engine, Ptr};
use std::collections::HashMap;

/// Where `IDirect3DQuery9`'s slots sit.
pub const QUERY_CODE: u32 = CODE_BASE + 0x0800;
/// `IDirect3DQuery9`'s slots (d3d9.h order).
pub const QUERY_SLOTS: [&str; 8] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetDevice",
    "GetType",
    "GetDataSize",
    "Issue",
    "GetData",
];

const IID_QUERY9: [u8; 16] = guid(
    0xd9771460,
    0xa695,
    0x41ba,
    [0x9b, 0x3d, 0xd1, 0x2a, 0x2b, 0x1f, 0xd5, 0x1e],
);
/// `D3DQUERYTYPE_EVENT`, `D3DQUERYTYPE_OCCLUSION`; `D3DISSUE_END`, `D3DISSUE_BEGIN`; `S_FALSE`.
const EVENT: u32 = 8;
const OCCLUSION: u32 = 9;
const ISSUE_END: u32 = 1;
const ISSUE_BEGIN: u32 = 2;
const S_FALSE: u32 = 1;

/// Where a query is in its life.
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Fresh,
    Begun,
    Ended,
}

#[derive(Debug, Clone)]
struct Query {
    references: u32,
    device: u32,
    kind: u32,
    state: State,
}

/// The queries (`Objects::queries`).
#[derive(Debug, Default)]
pub struct Queries {
    vtable: u32,
    queries: HashMap<u32, Query>,
}

fn queries(e: &mut Engine) -> &mut Queries {
    &mut e.kernel.graphics.device_objects.queries
}

fn query(e: &mut Engine, this: Ptr) -> &mut Query {
    queries(e)
        .queries
        .get_mut(&this.addr())
        .expect("an IDirect3DQuery9 of this layer")
}

/// Registers the query's slots and makes its vtable (once).
pub(super) fn register_code(e: &mut Engine) {
    let slots: [(u32, crate::AbiFn); 8] = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (3, crate::entry!(0, get_device(Ptr, Ptr) -> u32).1),
        (4, crate::entry!(0, get_type(Ptr) -> u32).1),
        (5, crate::entry!(0, get_data_size(Ptr) -> u32).1),
        (6, crate::entry!(0, issue(Ptr, u32) -> u32).1),
        (7, crate::entry!(0, get_data(Ptr, Ptr, u32, u32) -> u32).1),
    ];
    for (slot, f) in slots {
        e.register(QUERY_CODE + 4 * slot, f);
    }
    if queries(e).vtable == 0 {
        let table = e.mem.alloc(4 * QUERY_SLOTS.len() as u32);
        for i in 0..QUERY_SLOTS.len() as u32 {
            e.mem.set_u32(table + 4 * i, QUERY_CODE + 4 * i);
        }
        queries(e).vtable = table;
    }
}

// Platform: replaces D3D9.DLL IDirect3DDevice9::CreateQuery (vtable +0x1D8)
/// `CreateQuery(type, out)`: a null out asks whether the device takes the type (the machine
/// table's answer); else an event or occlusion query (holding the device), or the table's
/// refusal. Other types the device takes are not provided (the run stops).
pub(super) fn create_query(e: &mut Engine, this: Ptr, kind: u32, out: Ptr) -> u32 {
    let Some(support) = super::d3d9::query_support(e, kind) else {
        panic!(
            "IDirect3DDevice9::CreateQuery({kind}) is not in the machine table: run \
             `d3d9-capture device queries` and add its lines: {}",
            super::d3d9::CAPTURE_HINT
        );
    };
    if out.is_null() || support != D3D_OK {
        return support;
    }
    if kind != EVENT && kind != OCCLUSION {
        panic!("IDirect3DDevice9::CreateQuery({kind}): queries of this type are not provided");
    }
    register_code(e);
    let object = e.mem.alloc(8);
    let table = queries(e).vtable;
    e.mem.set_u32(object, table);
    queries(e).queries.insert(
        object,
        Query {
            references: 1,
            device: this.addr(),
            kind,
            state: State::Fresh,
        },
    );
    device_add_ref(e, this);
    #[cfg(windows)]
    super::d3d9real::query_created(e, this.addr(), object, kind);
    e.mem.set_u32(out.addr(), object);
    D3D_OK
}

fn query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    if iid_is(e, iid, &[IID_IUNKNOWN, IID_QUERY9]) {
        add_ref(e, this);
        e.mem.set_u32(out.addr(), this.addr());
        return D3D_OK;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

fn add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let q = query(e, this);
    q.references += 1;
    q.references
}

fn release(e: &mut Engine, this: Ptr) -> u32 {
    let q = query(e, this);
    q.references -= 1;
    let left = q.references;
    if left == 0 {
        let q = queries(e).queries.remove(&this.addr()).expect("the query");
        #[cfg(windows)]
        super::d3d9real::forget(e, this.addr());
        e.mem.free(this.addr());
        device_release(e, Ptr::new(q.device));
    }
    left
}

fn get_device(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    if out.is_null() {
        return D3DERR_INVALIDCALL;
    }
    let device = query(e, this).device;
    device_add_ref(e, Ptr::new(device));
    e.mem.set_u32(out.addr(), device);
    D3D_OK
}

fn get_type(e: &mut Engine, this: Ptr) -> u32 {
    query(e, this).kind
}

/// Event and occlusion results are 4 bytes (a `BOOL`, a `DWORD`).
fn get_data_size(_e: &mut Engine, _this: Ptr) -> u32 {
    4
}

// Platform: replaces D3D9.DLL IDirect3DQuery9::Issue (vtable +0x18)
/// `Issue(flags)`: `D3DISSUE_BEGIN` and `D3DISSUE_END` answer `D3D_OK` in any order (measured);
/// other flags are not provided.
fn issue(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    let state = match flags {
        ISSUE_BEGIN => State::Begun,
        ISSUE_END => State::Ended,
        _ => panic!("IDirect3DQuery9::Issue({flags:#x}) is not provided"),
    };
    query(e, this).state = state;
    #[cfg(windows)]
    super::d3d9real::query_issue(e, this.addr(), flags);
    D3D_OK
}

/// Writes `size` bytes of `value` (zero-extended past its 4 bytes) at `data`.
fn write_result(e: &mut Engine, data: Ptr, size: u32, value: u32) {
    if data.is_null() {
        return;
    }
    // The bytes on the stack for the sizes the game asks (4, 8).
    let mut inline = [0u8; 16];
    let mut heap = Vec::new();
    let bytes: &mut [u8] = if size as usize <= inline.len() {
        &mut inline[..size as usize]
    } else {
        heap.resize(size as usize, 0);
        &mut heap
    };
    let n = bytes.len().min(4);
    bytes[..n].copy_from_slice(&value.to_le_bytes()[..n]);
    e.mem.write(data.addr(), bytes);
}

// Platform: replaces D3D9.DLL IDirect3DQuery9::GetData (vtable +0x1C)
/// `GetData(data, size, flags)`: as the module documentation says.
fn get_data(e: &mut Engine, this: Ptr, data: Ptr, size: u32, flags: u32) -> u32 {
    let (kind, state) = {
        let q = query(e, this);
        (q.kind, q.state)
    };
    match state {
        State::Fresh => {
            write_result(e, data, size.min(4), 0xdddd_dddd);
            D3D_OK
        }
        State::Begun => S_FALSE,
        State::Ended => {
            #[cfg(windows)]
            if let Some((hr, value)) = super::d3d9real::query_get_data(e, this.addr(), flags) {
                if hr == D3D_OK {
                    write_result(e, data, size, value);
                }
                return hr;
            }
            let _ = flags;
            if kind == OCCLUSION {
                panic!(
                    "IDirect3DQuery9::GetData of an occlusion query without NV_D3D9=real (nothing \
                     is drawn, so no pixel count) is not provided"
                );
            }
            write_result(e, data, size, 1);
            D3D_OK
        }
    }
}
