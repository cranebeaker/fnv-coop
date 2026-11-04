//! `(unplaced)/static_init_irregular_c` (no source unit: the compiler's dynamic initializers),
//! subsystem `compiler: static init/exit`: ten dynamic initializers of FalloutNV.exe 1.4.0.525
//! that the recorded game ran and parts 1 and 2 (`static_init_irregular.rs`,
//! `static_init_irregular_b.rs`) do not have, each translated from its own disassembly. Part 3
//! of the list.
//!
//! The CRT runs every initializer once at start-up, before `main`, in table order. They are
//! cdecl functions with no parameters; what they leave in `EAX` nobody reads, so they return
//! nothing. The functions:
//!
//! - `00f7a300`, `00f7dd10`, `00f80050` (fallout shared/menugamesettings.cpp): long runs of
//!   objects made the same way (`00401000(0xc)`, then the constructor `0040c150` on the block
//!   with two addresses, the answer or 0 stored in a global), `00f7a300` with zero or constant
//!   words stored between them; each run is a table of [`InitStep`]s in the code's order (the
//!   code was read block by block, every instruction checked).
//! - `00f90480` (fallout/interface/menus/loadingmenu.cpp): five `0045cfe0` constructors and an
//!   `_atexit`.
//! - `00fb33f0` (BSMenu/tile.cpp): an object built with its vtables and given to the settings
//!   collection singleton, then an `_atexit`.
//! - `00fbcd10`, `00fbd0a0`, `00fbd160`: a float setting (`0044f440`) and an `_atexit`.
//! - `00fbdde0`: a 64-byte copy; `00fc791c`: a tail jump to `00ec3a1b` on a stream.
//!
//! Six of them (`00f7a300`, `00f7dd10`, `00f80050`, `00f90480`, `00fb33f0`) sit in an `FS:[0]`
//! exception frame and are marked `// Translation incomplete`: the frame (a handler address and
//! state numbers) is the compiler's unwinding on an exception and is not translated; every call
//! in their bodies is. None calls the `/GS` cookie check `00ec408c`.
//!
//! Callees are called by address and never re-implemented.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_atexit` (`00ec658f`, cdecl, one word: the function to run at exit).
const ATEXIT: u32 = 0x00ec_658f;
/// `00401000` (cdecl, one word: a size; answers `EAX`).
const FN_00401000: u32 = 0x0040_1000;
/// `0040c150` (`thiscall`, `RET 8`): `(this, word, word)`, answers `EAX`.
const FN_0040C150: u32 = 0x0040_c150;
/// `0045cfe0` (`thiscall`, `RET 8`): `(this, word, word)`.
const FN_0045CFE0: u32 = 0x0045_cfe0;
/// `0044f440` (`thiscall`, `RET 8`): `(this, name, float)`, the constructor of a float
/// setting; `0044f570` (cdecl, no arguments): makes the settings collection kept at `011f96a0`
/// when it is absent; `00ec3a1b` (`thiscall`, no stack argument).
const FN_0044F440: u32 = 0x0044_f440;
const SETTINGS_GETTER_0044F570: u32 = 0x0044_f570;
const SETTINGS_COLLECTION: u32 = 0x011f_96a0;
const FN_00EC3A1B: u32 = 0x00ec_3a1b;

/// `PUSH exit_fn; CALL _atexit; ADD ESP,4` (the result is not used).
fn register_at_exit(e: &mut Engine, exit_fn: u32) {
    e.call(ATEXIT, &args![exit_fn]);
}

/// One step of the long object initializers below, in the code's order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum InitStep {
    /// `PUSH 0xc; CALL 00401000`; when that answers a block, `PUSH second; PUSH first; MOV
    /// ECX,block; CALL 0040c150`; its answer, or 0 for a null block, stored in the global
    /// (through two frame locals, with the `FS:[0]` state set around the constructor):
    /// `(global, first, second)`.
    Object(u32, u32, u32),
    /// `MOV dword ptr [global], value`: `(global, value)`.
    Store(u32, u32),
}

/// Runs the steps of a long object initializer in order.
pub(super) fn run_steps(e: &mut Engine, steps: &[InitStep]) {
    for step in steps {
        match *step {
            InitStep::Object(global, first, second) => {
                let block = e.call(FN_00401000, &args![0xc_u32]).u32();
                let object = if block != 0 {
                    e.call(FN_0040C150, &args![block, first, second]).u32()
                } else {
                    0
                };
                e.set_global::<u32>(global, object);
            }
            InitStep::Store(global, value) => e.set_global::<u32>(global, value),
        }
    }
}

const STEPS_00F7A300: [InitStep; 237] = [
    InitStep::Object(0x011d_52f4, 0x0106_3f4c, 0x0106_3f54),
    InitStep::Object(0x011d_52f8, 0x0106_3f40, 0x0106_3f48),
    InitStep::Object(0x011d_52fc, 0x0106_3f34, 0x0106_3f3c),
    InitStep::Object(0x011d_5300, 0x0106_3f28, 0x0106_3f30),
    InitStep::Object(0x011d_5304, 0x0106_3f1c, 0x0106_3f24),
    InitStep::Object(0x011d_5308, 0x0106_3f10, 0x0106_3f18),
    InitStep::Object(0x011d_530c, 0x0106_3f04, 0x0106_3f0c),
    InitStep::Object(0x011d_5310, 0x0106_3ef8, 0x0106_3f00),
    InitStep::Object(0x011d_5314, 0x0106_3eec, 0x0106_3ef4),
    InitStep::Object(0x011d_5318, 0x0106_3ee0, 0x0106_3ee8),
    InitStep::Object(0x011d_531c, 0x0106_3ed8, 0x0102_0760),
    InitStep::Object(0x011d_5320, 0x0106_3ecc, 0x0103_45e0),
    InitStep::Object(0x011d_5324, 0x0106_3ebc, 0x0106_3ec8),
    InitStep::Object(0x011d_5328, 0x0106_3ea8, 0x0106_3eb0),
    InitStep::Object(0x011d_532c, 0x0106_3e9c, 0x0106_3ea4),
    InitStep::Object(0x011d_5330, 0x0106_3e90, 0x0106_3e98),
    InitStep::Object(0x011d_5334, 0x0106_3e88, 0x0102_d288),
    InitStep::Object(0x011d_5338, 0x0106_3e7c, 0x0106_3e84),
    InitStep::Object(0x011d_533c, 0x0106_3e70, 0x0106_3e78),
    InitStep::Object(0x011d_5340, 0x0106_3e64, 0x0106_3e6c),
    InitStep::Object(0x011d_5344, 0x0106_3e5c, 0x0101_7b68),
    InitStep::Object(0x011d_5348, 0x0106_3e50, 0x0106_3e58),
    InitStep::Object(0x011d_534c, 0x0106_3e44, 0x0106_3e4c),
    InitStep::Object(0x011d_5350, 0x0106_3e38, 0x0106_3e40),
    InitStep::Object(0x011d_5354, 0x0106_3e2c, 0x0106_3e34),
    InitStep::Object(0x011d_5358, 0x0106_3e1c, 0x0106_3e28),
    InitStep::Object(0x011d_535c, 0x0106_3e0c, 0x0106_3e18),
    InitStep::Object(0x011d_5360, 0x0106_3df8, 0x0106_3e04),
    InitStep::Object(0x011d_5364, 0x0106_3de4, 0x0106_3df0),
    InitStep::Object(0x011d_5368, 0x0106_3dd8, 0x0106_3de0),
    InitStep::Object(0x011d_536c, 0x0106_3dcc, 0x0106_3dd4),
    InitStep::Object(0x011d_5370, 0x0106_3dc0, 0x0106_3dc8),
    InitStep::Object(0x011d_5374, 0x0106_3db4, 0x0106_3dbc),
    InitStep::Object(0x011d_5378, 0x0106_3da8, 0x0106_3db0),
    InitStep::Object(0x011d_537c, 0x0106_3da0, 0x0103_9560),
    InitStep::Object(0x011d_5380, 0x0106_3d94, 0x0106_3d9c),
    InitStep::Object(0x011d_5384, 0x0106_3d88, 0x0106_3d90),
    InitStep::Object(0x011d_5388, 0x0106_3d80, 0x0103_9564),
    InitStep::Object(0x011d_538c, 0x0106_3d6c, 0x0106_3d7c),
    InitStep::Object(0x011d_5390, 0x0106_3d58, 0x0106_3d68),
    InitStep::Object(0x011d_5394, 0x0106_3d48, 0x0106_3d54),
    InitStep::Object(0x011d_5398, 0x0106_3d34, 0x0106_3d40),
    InitStep::Object(0x011d_539c, 0x0106_3d24, 0x0101_3444),
    InitStep::Object(0x011d_53a0, 0x0106_3d1c, 0x0104_49d8),
    InitStep::Object(0x011d_53a4, 0x0106_3d14, 0x0104_49dc),
    InitStep::Object(0x011d_53a8, 0x0106_3d08, 0x0106_3d10),
    InitStep::Object(0x011d_53ac, 0x0106_3cfc, 0x0106_3d04),
    InitStep::Object(0x011d_53b0, 0x0106_3cf0, 0x0106_3cf8),
    InitStep::Object(0x011d_53b4, 0x0106_3ce8, 0x0101_7b64),
    InitStep::Object(0x011d_53b8, 0x0106_3cdc, 0x0106_3ce4),
    InitStep::Object(0x011d_53bc, 0x0106_3ccc, 0x0106_3cd8),
    InitStep::Object(0x011d_53c0, 0x0106_3cc0, 0x0104_4c28),
    InitStep::Object(0x011d_53c4, 0x0106_3cb0, 0x0106_3cbc),
    InitStep::Object(0x011d_53c8, 0x0106_3c9c, 0x0106_3ca8),
    InitStep::Object(0x011d_53cc, 0x0106_3c88, 0x0106_3c94),
    InitStep::Object(0x011d_53d0, 0x0106_3c74, 0x0106_3c80),
    InitStep::Object(0x011d_53d4, 0x0106_3c60, 0x0106_3c6c),
    InitStep::Object(0x011d_53d8, 0x0106_3c54, 0x0105_b634),
    InitStep::Object(0x011d_53dc, 0x0106_3c48, 0x0106_3c50),
    InitStep::Object(0x011d_53e0, 0x0106_3c3c, 0x0106_3c44),
    InitStep::Object(0x011d_53e4, 0x0106_3c30, 0x0106_3c38),
    InitStep::Object(0x011d_53e8, 0x0106_3c24, 0x0106_3c2c),
    InitStep::Object(0x011d_53ec, 0x0106_3c18, 0x0106_3c20),
    InitStep::Object(0x011d_53f0, 0x0106_3c0c, 0x0106_3c14),
    InitStep::Object(0x011d_53f4, 0x0106_3c00, 0x0106_3c08),
    InitStep::Object(0x011d_53f8, 0x0106_3bf4, 0x0106_3bfc),
    InitStep::Object(0x011d_53fc, 0x0106_3be8, 0x0106_3bf0),
    InitStep::Object(0x011d_5400, 0x0106_3bdc, 0x0106_3be4),
    InitStep::Object(0x011d_5404, 0x0106_3bcc, 0x0106_3bd8),
    InitStep::Object(0x011d_5408, 0x0106_3bb8, 0x0106_3bc4),
    InitStep::Object(0x011d_540c, 0x0106_3ba4, 0x0106_3bb0),
    InitStep::Object(0x011d_5410, 0x0106_3b90, 0x0106_3b9c),
    InitStep::Object(0x011d_5414, 0x0106_3b7c, 0x0106_3b88),
    InitStep::Object(0x011d_5418, 0x0106_3b68, 0x0106_3b74),
    InitStep::Object(0x011d_541c, 0x0106_3b54, 0x0106_3b60),
    InitStep::Object(0x011d_5420, 0x0106_3b40, 0x0106_3b4c),
    InitStep::Object(0x011d_5424, 0x0106_3b2c, 0x0106_3b38),
    InitStep::Object(0x011d_5428, 0x0106_3b1c, 0x0106_3b24),
    InitStep::Object(0x011d_542c, 0x0106_3b08, 0x0106_3b14),
    InitStep::Object(0x011d_5430, 0x0106_3af4, 0x0106_3b00),
    InitStep::Object(0x011d_5434, 0x0106_3ae0, 0x0106_3aec),
    InitStep::Object(0x011d_5438, 0x0106_3acc, 0x0106_3ad8),
    InitStep::Object(0x011d_543c, 0x0106_3ab8, 0x0106_3ac4),
    InitStep::Store(0x011d_5440, 0x0000_0000),
    InitStep::Store(0x011d_5444, 0x0000_0000),
    InitStep::Object(0x011d_5448, 0x0106_3aa8, 0x0106_3ab4),
    InitStep::Object(0x011d_544c, 0x0106_3a9c, 0x0106_3aa4),
    InitStep::Object(0x011d_5450, 0x0106_3a90, 0x0106_3a98),
    InitStep::Store(0x011d_5454, 0x0000_0000),
    InitStep::Store(0x011d_5458, 0x0000_0000),
    InitStep::Store(0x011d_545c, 0x0000_0000),
    InitStep::Store(0x011d_5460, 0x0000_0000),
    InitStep::Store(0x011d_5464, 0x0000_0000),
    InitStep::Store(0x011d_5468, 0x0000_0000),
    InitStep::Store(0x011d_546c, 0x0000_0000),
    InitStep::Store(0x011d_5470, 0x0000_0000),
    InitStep::Store(0x011d_5474, 0x0000_0000),
    InitStep::Store(0x011d_5478, 0x0000_0000),
    InitStep::Store(0x011d_547c, 0x0000_0000),
    InitStep::Object(0x011d_5480, 0x0106_3a84, 0x0106_3a8c),
    InitStep::Object(0x011d_5484, 0x0106_3a78, 0x0106_3a80),
    InitStep::Object(0x011d_5488, 0x0106_3a6c, 0x0106_3a74),
    InitStep::Store(0x011d_548c, 0x0000_0000),
    InitStep::Store(0x011d_5490, 0x0000_0000),
    InitStep::Store(0x011d_5494, 0x0000_0000),
    InitStep::Store(0x011d_5498, 0x0000_0000),
    InitStep::Store(0x011d_549c, 0x0000_0000),
    InitStep::Store(0x011d_54a0, 0x0000_0000),
    InitStep::Store(0x011d_54a4, 0x0000_0000),
    InitStep::Store(0x011d_54a8, 0x0000_0000),
    InitStep::Store(0x011d_54ac, 0x0000_0000),
    InitStep::Store(0x011d_54b0, 0x0000_0000),
    InitStep::Store(0x011d_54b4, 0x0000_0000),
    InitStep::Store(0x011d_54b8, 0x0000_0000),
    InitStep::Object(0x011d_54bc, 0x0106_3a60, 0x0106_3cbc),
    InitStep::Store(0x011d_54c0, 0x0000_0000),
    InitStep::Store(0x011d_54c4, 0x0000_0000),
    InitStep::Store(0x011d_54c8, 0x0000_0000),
    InitStep::Store(0x011d_54cc, 0x0000_0000),
    InitStep::Store(0x011d_54d0, 0x0000_0000),
    InitStep::Store(0x011d_54d4, 0x0000_0000),
    InitStep::Store(0x011d_54d8, 0x0000_0000),
    InitStep::Store(0x011d_54dc, 0x0000_0000),
    InitStep::Store(0x011d_54e0, 0x0000_0000),
    InitStep::Store(0x011d_54e4, 0x0000_0000),
    InitStep::Object(0x011d_54e8, 0x0106_3a54, 0x0106_3ac4),
    InitStep::Store(0x011d_54ec, 0x0000_0000),
    InitStep::Store(0x011d_54f0, 0x0000_0000),
    InitStep::Store(0x011d_54f4, 0x0000_0000),
    InitStep::Store(0x011d_54f8, 0x0000_0000),
    InitStep::Store(0x011d_54fc, 0x0000_0000),
    InitStep::Store(0x011d_5500, 0x0000_0000),
    InitStep::Store(0x011d_5504, 0x0000_0000),
    InitStep::Store(0x011d_5508, 0x0000_0000),
    InitStep::Store(0x011d_550c, 0x0000_0000),
    InitStep::Store(0x011d_5510, 0x0000_0000),
    InitStep::Store(0x011d_5514, 0x0000_0000),
    InitStep::Store(0x011d_5518, 0x0000_0000),
    InitStep::Store(0x011d_551c, 0x0000_0000),
    InitStep::Store(0x011d_5520, 0x0000_0000),
    InitStep::Object(0x011d_5524, 0x0106_3a3c, 0x0106_3a4c),
    InitStep::Store(0x011d_5528, 0x0000_0000),
    InitStep::Store(0x011d_552c, 0x0000_0000),
    InitStep::Object(0x011d_5530, 0x0106_3a24, 0x0106_3a34),
    InitStep::Object(0x011d_5534, 0x0106_3a18, 0x0106_3a20),
    InitStep::Object(0x011d_5538, 0x0106_3a08, 0x0106_3a14),
    InitStep::Object(0x011d_553c, 0x0106_39f4, 0x0106_3a04),
    InitStep::Store(0x011d_5540, 0x0000_0000),
    InitStep::Object(0x011d_5544, 0x0106_39e4, 0x0106_39ec),
    InitStep::Store(0x011d_5548, 0x0000_0000),
    InitStep::Object(0x011d_554c, 0x0106_39c8, 0x0106_39d8),
    InitStep::Store(0x011d_5550, 0x0000_0000),
    InitStep::Object(0x011d_5554, 0x0106_39b8, 0x0105_9d50),
    InitStep::Store(0x011d_5558, 0x0000_0000),
    InitStep::Store(0x011d_555c, 0x0000_0000),
    InitStep::Object(0x011d_5560, 0x0106_399c, 0x0106_39ac),
    InitStep::Object(0x011d_5564, 0x0106_3988, 0x0106_3994),
    InitStep::Store(0x011d_5568, 0x0000_0000),
    InitStep::Store(0x011d_556c, 0x0000_0000),
    InitStep::Object(0x011d_5570, 0x0106_3978, 0x0106_3980),
    InitStep::Object(0x011d_5574, 0x0106_3960, 0x0106_3970),
    InitStep::Object(0x011d_5578, 0x0106_3948, 0x0106_3958),
    InitStep::Store(0x011d_557c, 0x0000_0000),
    InitStep::Object(0x011d_5580, 0x0106_3938, 0x0106_39ec),
    InitStep::Store(0x011d_5584, 0x0000_0000),
    InitStep::Store(0x011d_5588, 0x0000_0000),
    InitStep::Store(0x011d_558c, 0x0000_0000),
    InitStep::Store(0x011d_5590, 0x0000_0000),
    InitStep::Store(0x011d_5594, 0x0000_0000),
    InitStep::Store(0x011d_5598, 0x0000_0000),
    InitStep::Store(0x011d_559c, 0x0000_0000),
    InitStep::Store(0x011d_55a0, 0x0000_0000),
    InitStep::Store(0x011d_55a4, 0x0000_0000),
    InitStep::Object(0x011d_55a8, 0x0106_3920, 0x0106_3930),
    InitStep::Store(0x011d_55ac, 0x0000_0000),
    InitStep::Object(0x011d_55b0, 0x0106_390c, 0x0106_3918),
    InitStep::Store(0x011d_55b4, 0x0000_0000),
    InitStep::Object(0x011d_55b8, 0x0106_38fc, 0x0106_3908),
    InitStep::Object(0x011d_55bc, 0x0106_38e4, 0x0106_38f4),
    InitStep::Store(0x011d_55c0, 0x0000_0000),
    InitStep::Object(0x011d_55c4, 0x0106_38d0, 0x0106_38dc),
    InitStep::Store(0x011d_55c8, 0x0000_0000),
    InitStep::Object(0x011d_55cc, 0x0106_38b8, 0x0106_38c4),
    InitStep::Object(0x011d_55d0, 0x0106_38a4, 0x0106_38b0),
    InitStep::Store(0x011d_55d4, 0x0000_0000),
    InitStep::Store(0x011d_55d8, 0x0000_0000),
    InitStep::Store(0x011d_55dc, 0x0000_0000),
    InitStep::Store(0x011d_55e0, 0x0000_0000),
    InitStep::Store(0x011d_55e4, 0x0000_0000),
    InitStep::Store(0x011d_55e8, 0x0000_0000),
    InitStep::Store(0x011d_55ec, 0x0000_0000),
    InitStep::Store(0x011d_55f0, 0x0000_0000),
    InitStep::Store(0x011d_55f4, 0x0000_0000),
    InitStep::Store(0x011d_55f8, 0x0000_0000),
    InitStep::Store(0x011d_55fc, 0x0000_0000),
    InitStep::Store(0x011d_5600, 0x0000_0000),
    InitStep::Object(0x011d_5604, 0x0106_3898, 0x0105_e954),
    InitStep::Store(0x011d_5608, 0x0000_0000),
    InitStep::Object(0x011d_560c, 0x0106_3888, 0x0106_3890),
    InitStep::Object(0x011d_5610, 0x0106_3880, 0x0101_d92c),
    InitStep::Object(0x011d_5614, 0x0106_386c, 0x0106_3878),
    InitStep::Store(0x011d_5618, 0x0000_0000),
    InitStep::Object(0x011d_561c, 0x0106_3864, 0x0104_8444),
    InitStep::Store(0x011d_5620, 0x0000_0000),
    InitStep::Object(0x011d_5624, 0x0106_3858, 0x0104_843c),
    InitStep::Store(0x011d_5628, 0x0000_0000),
    InitStep::Object(0x011d_562c, 0x0106_3850, 0x0104_4c90),
    InitStep::Object(0x011d_5630, 0x0106_3848, 0x0101_d924),
    InitStep::Object(0x011d_5634, 0x0106_3838, 0x0106_3840),
    InitStep::Object(0x011d_5638, 0x0106_3824, 0x0106_3830),
    InitStep::Object(0x011d_563c, 0x0106_3818, 0x0106_2100),
    InitStep::Store(0x011d_5640, 0x0000_0000),
    InitStep::Store(0x011d_5644, 0x0000_0000),
    InitStep::Store(0x011d_5648, 0x0000_0000),
    InitStep::Store(0x011d_564c, 0x0000_0000),
    InitStep::Store(0x011d_5650, 0x0000_0000),
    InitStep::Store(0x011d_5654, 0x0000_0000),
    InitStep::Store(0x011d_5658, 0x0000_0000),
    InitStep::Object(0x011d_565c, 0x0106_3808, 0x0106_3810),
    InitStep::Object(0x011d_5660, 0x0106_37f8, 0x0106_3800),
    InitStep::Object(0x011d_5664, 0x0106_37e8, 0x0106_37f0),
    InitStep::Object(0x011d_5668, 0x0106_37dc, 0x0101_3ae0),
    InitStep::Object(0x011d_566c, 0x0106_37c8, 0x0106_37d4),
    InitStep::Store(0x011d_5670, 0x0000_0000),
    InitStep::Store(0x011d_5674, 0x0000_0000),
    InitStep::Store(0x011d_5678, 0x0000_0000),
    InitStep::Object(0x011d_567c, 0x0106_37b8, 0x0106_37c0),
    InitStep::Store(0x011d_5680, 0x0000_0000),
    InitStep::Object(0x011d_5684, 0x0106_37a8, 0x0106_2acc),
    InitStep::Object(0x011d_5688, 0x0106_3794, 0x0106_37a4),
    InitStep::Object(0x011d_568c, 0x0106_377c, 0x0106_378c),
    InitStep::Object(0x011d_5690, 0x0106_3764, 0x0106_3770),
    InitStep::Object(0x011d_5694, 0x0106_374c, 0x0106_375c),
    InitStep::Object(0x011d_5698, 0x0106_3738, 0x0106_3744),
    InitStep::Object(0x011d_569c, 0x0106_3720, 0x0106_3730),
    InitStep::Object(0x011d_56a0, 0x0106_3710, 0x0106_3718),
    InitStep::Object(0x011d_56a4, 0x0106_3700, 0x0105_acb4),
];

// Translated from 00f7a300 (disassembled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the key and button names of the menu game settings (fallout
/// shared/menugamesettings.cpp, Ghidra's `_dynamic_initializer_for_psK..`): 138 objects
/// (`00401000(0xc)`, then `0040c150` on the block with its two addresses; the answer or 0 in the
/// global) and 99 zero or constant words stored between them, in the code's order
/// ([`STEPS_00F7A300`], `011d52f4` .. `011d56a4`).
// Translation incomplete: 00f7a300 - FS:[0] exception frame (handler 00efded2, states 0 to 0x89, -1 between the blocks) not translated.
pub fn fn_00f7a300(e: &mut Engine) {
    run_steps(e, &STEPS_00F7A300);
}

const STEPS_00F7DD10: [InitStep; 9] = [
    InitStep::Object(0x011d_5240, 0x0106_4038, 0x0106_404c),
    InitStep::Object(0x011d_5244, 0x0106_401c, 0x0106_4030),
    InitStep::Object(0x011d_5248, 0x0106_4000, 0x0106_4014),
    InitStep::Object(0x011d_524c, 0x0106_3fe4, 0x0106_3ff4),
    InitStep::Object(0x011d_5250, 0x0106_3fc8, 0x0106_3fd8),
    InitStep::Object(0x011d_5254, 0x0106_3fac, 0x0106_3fbc),
    InitStep::Object(0x011d_5258, 0x0106_3f90, 0x0106_3fa0),
    InitStep::Object(0x011d_525c, 0x0106_3f74, 0x0106_3f84),
    InitStep::Object(0x011d_5260, 0x0106_3f58, 0x0106_3f68),
];

// Translated from 00f7dd10 (disassembled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of nine menu game settings objects (fallout shared/menugamesettings.cpp,
/// Ghidra's `_dynamic_initializer_for_psM..`, `011d5240` .. `011d5260`): each as an
/// [`InitStep::Object`] ([`STEPS_00F7DD10`]).
// Translation incomplete: 00f7dd10 - FS:[0] exception frame (handler 00efdf53, states 0 to 8, -1 between the blocks) not translated.
pub fn fn_00f7dd10(e: &mut Engine) {
    run_steps(e, &STEPS_00F7DD10);
}

const STEPS_00F80050: [InitStep; 19] = [
    InitStep::Object(0x011d_5164, 0x0106_4ae0, 0x0106_4af0),
    InitStep::Object(0x011d_5168, 0x0106_4ad0, 0x0101_1f30),
    InitStep::Object(0x011d_516c, 0x0106_4abc, 0x0106_4acc),
    InitStep::Object(0x011d_5170, 0x0106_4aa8, 0x0104_1730),
    InitStep::Object(0x011d_5174, 0x0106_4a94, 0x0106_37d4),
    InitStep::Object(0x011d_5178, 0x0106_4a7c, 0x0106_4a8c),
    InitStep::Object(0x011d_517c, 0x0106_4a6c, 0x0106_2aa8),
    InitStep::Object(0x011d_5180, 0x0106_4a58, 0x0101_1f30),
    InitStep::Object(0x011d_5184, 0x0106_4a3c, 0x0106_4a50),
    InitStep::Object(0x011d_5188, 0x0106_4a28, 0x0106_2aa8),
    InitStep::Object(0x011d_518c, 0x0106_4a08, 0x0106_4a1c),
    InitStep::Object(0x011d_5190, 0x0106_49f4, 0x0104_837c),
    InitStep::Object(0x011d_5194, 0x0106_49e0, 0x0102_047c),
    InitStep::Object(0x011d_5198, 0x0106_49c4, 0x0106_49d8),
    InitStep::Object(0x011d_519c, 0x0106_49b0, 0x0106_49c0),
    InitStep::Object(0x011d_51a0, 0x0106_499c, 0x0104_ce20),
    InitStep::Object(0x011d_51a4, 0x0106_4984, 0x0106_4994),
    InitStep::Object(0x011d_51a8, 0x0106_4968, 0x0106_497c),
    InitStep::Object(0x011d_51ac, 0x0106_4954, 0x0104_ce58),
];

// Translated from 00f80050 (disassembled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of nineteen menu game settings objects (fallout
/// shared/menugamesettings.cpp, Ghidra's `_dynamic_initializer_for_psT..`, `011d5164` ..
/// `011d51ac`): each as an [`InitStep::Object`] ([`STEPS_00F80050`]).
// Translation incomplete: 00f80050 - FS:[0] exception frame (handler 00efe510, states 0 to 0x12, -1 between the blocks) not translated.
pub fn fn_00f80050(e: &mut Engine) {
    run_steps(e, &STEPS_00F80050);
}

/// `(global, first, second)` of the five `0045cfe0` calls of `00f90480`.
const PAIRS_00F90480: [(u32, u32, u32); 5] = [
    (0x011d_a1b0, 0x0107_41d4, 0x0107_41ec),
    (0x011d_a1bc, 0x0107_41a4, 0x0107_41bc),
    (0x011d_a1c8, 0x0107_4178, 0x0107_4190),
    (0x011d_a1d4, 0x0107_4148, 0x0107_4160),
    (0x011d_a1e0, 0x0107_411c, 0x0107_4138),
];

// Translated from 00f90480 (disassembled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the five globals of the loading menu
/// (fallout/interface/menus/loadingmenu.cpp, Ghidra's `_dynamic_initializer_for_Loa..`,
/// `011da1b0` .. `011da1e0`, 12 bytes apart): `MOV ECX,global; CALL 0045cfe0` with two pushed
/// addresses each ([`PAIRS_00F90480`]), then `_atexit(00fd7f30)`.
// Translation incomplete: 00f90480 - FS:[0] exception frame (handler 00f0bf78, states 0 to 3, -1 before the atexit call) not translated.
pub fn fn_00f90480(e: &mut Engine) {
    for &(global, first, second) in &PAIRS_00F90480 {
        e.call(FN_0045CFE0, &args![global, first, second]);
    }
    register_at_exit(e, 0x00fd_7f30);
}

// Translated from 00fb33f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the object at `011f3368` (BSMenu/tile.cpp, Ghidra's
/// `_dynamic_initializer_for_sDe..`): its vtable word `01012120`, the words `01094ed8` at +8 and
/// `01094eb8` at +4, then the vtable `01017720`; `0044f570` (the settings collection singleton,
/// made when absent) and the virtual function at slot +4 of the collection (`[011f96a0]`) with
/// the object pushed; then `_atexit(00fda210)`.
// Translation incomplete: 00fb33f0 - FS:[0] exception frame (handler 00f1d40a, state 0 around the vtable store and the virtual call) not translated.
pub fn fn_00fb33f0(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3368, 0x0101_2120);
    e.set_global::<u32>(0x011f_3370, 0x0109_4ed8);
    e.set_global::<u32>(0x011f_336c, 0x0109_4eb8);
    e.set_global::<u32>(0x011f_3368, 0x0101_7720);
    e.call(SETTINGS_GETTER_0044F570, &args![]);
    let collection = e.global::<u32>(SETTINGS_COLLECTION);
    e.vcall(collection, 4, &args![0x011f_3368_u32]);
    register_at_exit(e, 0x00fd_a210);
}

/// `FLD float ptr [value]; PUSH ECX; FSTP float ptr [ESP]; PUSH name; MOV ECX,global; CALL
/// 0044f440`, then `_atexit(exit_fn)`: a float setting constructed on the global with its name
/// and the float (the `FLD`/`FSTP` copy, a signalling NaN quieted).
fn float_setting_at_exit(e: &mut Engine, global: u32, name: u32, value: u32, exit_fn: u32) {
    let float = x87_quiet_bits(e.global::<u32>(value));
    e.call(FN_0044F440, &args![global, name, float]);
    register_at_exit(e, exit_fn);
}

// Translated from 00fbcd10 (disassembled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the float setting at `01267bd4` (name `010c4af4`, value from
/// `010c4b18`), its destructor `00fdde60` given to `_atexit`.
pub fn fn_00fbcd10(e: &mut Engine) {
    float_setting_at_exit(e, 0x0126_7bd4, 0x010c_4af4, 0x010c_4b18, 0x00fd_de60);
}

// Translated from 00fbd0a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the float setting at `01267d08` (name `010c5034`, value from
/// `010223ec`), its destructor `00fddfc0` given to `_atexit`.
pub fn fn_00fbd0a0(e: &mut Engine) {
    float_setting_at_exit(e, 0x0126_7d08, 0x010c_5034, 0x0102_23ec, 0x00fd_dfc0);
}

// Translated from 00fbd160 (disassembled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the float setting at `01267d38` (name `010c5090`, value from
/// `01016248`), its destructor `00fde000` given to `_atexit`.
pub fn fn_00fbd160(e: &mut Engine) {
    float_setting_at_exit(e, 0x0126_7d38, 0x010c_5090, 0x0101_6248, 0x00fd_e000);
}

// Translated from 00fbdde0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the 64 bytes at `01268370`: four `MOVAPS` copies of 16 bytes from
/// `011b1c50` .. `011b1c8f` (a plain copy of the words).
pub fn fn_00fbdde0(e: &mut Engine) {
    for offset in (0..0x40_u32).step_by(4) {
        let word = e.global::<u32>(0x011b_1c50 + offset);
        e.set_global::<u32>(0x0126_8370 + offset, word);
    }
}

// Translated from 00fc791c (disassembled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the stream at `0126fc48` (no label; listed at `00fdf4b8` of the C++
/// initializer table): `MOV ECX,0126fc48; JMP 00ec3a1b`.
pub fn fn_00fc791c(e: &mut Engine) {
    e.call(FN_00EC3A1B, &args![0x0126_fc48_u32]);
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00f7_a300, fn_00f7a300()),
        entry!(0x00f7_dd10, fn_00f7dd10()),
        entry!(0x00f8_0050, fn_00f80050()),
        entry!(0x00f9_0480, fn_00f90480()),
        entry!(0x00fb_33f0, fn_00fb33f0()),
        entry!(0x00fb_cd10, fn_00fbcd10()),
        entry!(0x00fb_d0a0, fn_00fbd0a0()),
        entry!(0x00fb_d160, fn_00fbd160()),
        entry!(0x00fb_dde0, fn_00fbdde0()),
        entry!(0x00fc_791c, fn_00fc791c()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    type Calls = Vec<(u32, Vec<u32>)>;

    /// Runs the function at `entry` and returns the calls it made, in order (the top-level call
    /// itself is filtered out by its address).
    fn run(e: &mut Engine, entry: u32) -> Calls {
        e.call_log = Some(vec![]);
        e.call(entry, &args![]);
        let log = e.call_log.take().unwrap();
        log.into_iter().filter(|(a, _)| *a != entry).collect()
    }

    /// Doubles that do nothing for each address.
    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    /// The calls of `callee` in `calls`.
    fn only(calls: &Calls, callee: u32) -> Calls {
        calls
            .iter()
            .filter(|(a, _)| *a == callee)
            .cloned()
            .collect()
    }

    /// A double for `00401000` that answers successive blocks (`0x0800_0010`, `0x0800_0020`,
    /// ...) and a `0040c150` double answering one past its `this`.
    fn object_doubles(e: &mut Engine) {
        let next = Rc::new(Cell::new(0_u32));
        let counter = next.clone();
        e.register_double(FN_00401000, move |_, _| {
            counter.set(counter.get() + 1);
            Ret {
                eax: 0x0800_0000 + 0x10 * counter.get(),
                ..Ret::default()
            }
        });
        e.register(FN_0040C150, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
    }

    /// Checks a run of `steps` against the calls made and the globals left (each global as its
    /// last step left it).
    fn check_steps(e: &Engine, calls: &Calls, steps: &[InitStep]) {
        let mut expected: Calls = Vec::new();
        let mut finals: Vec<(u32, u32)> = Vec::new();
        let mut block = 0_u32;
        for step in steps {
            let (global, value) = match *step {
                InitStep::Object(global, first, second) => {
                    block += 1;
                    let address = 0x0800_0000 + 0x10 * block;
                    expected.push((FN_00401000, vec![0xc]));
                    expected.push((FN_0040C150, vec![address, first, second]));
                    (global, address + 1)
                }
                InitStep::Store(global, value) => (global, value),
            };
            finals.retain(|(g, _)| *g != global);
            finals.push((global, value));
        }
        assert_eq!(calls, &expected);
        for (global, value) in finals {
            assert_eq!(e.global::<u32>(global), value, "{global:08x}");
        }
    }

    #[test]
    fn long_object_initializers_run_their_steps_in_order() {
        type Case = (u32, &'static [InitStep], usize, usize);
        let cases: [Case; 3] = [
            (0x00f7_a300, &STEPS_00F7A300, 138, 99),
            (0x00f7_dd10, &STEPS_00F7DD10, 9, 0),
            (0x00f8_0050, &STEPS_00F80050, 19, 0),
        ];
        for (entry, steps, objects, stores) in cases {
            let mut e = Engine::new();
            e.map(0x011d_5000, 0x1000);
            object_doubles(&mut e);
            let calls = run(&mut e, entry);
            let count = |object: bool| {
                steps
                    .iter()
                    .filter(|s| matches!(s, InitStep::Object(..)) == object)
                    .count()
            };
            assert_eq!((count(true), count(false)), (objects, stores));
            check_steps(&e, &calls, steps);
        }
        // The first and last steps of 00f7a300 as the code has them.
        assert_eq!(
            STEPS_00F7A300[0],
            InitStep::Object(0x011d_52f4, 0x0106_3f4c, 0x0106_3f54)
        );
        assert_eq!(
            STEPS_00F7A300[236],
            InitStep::Object(0x011d_56a4, 0x0106_3700, 0x0105_acb4)
        );
    }

    #[test]
    fn a_null_block_stores_zero_without_the_constructor() {
        let mut e = Engine::new();
        e.map(0x011d_5000, 0x1000);
        e.set_global::<u32>(0x011d_5240, 0xffff_ffff);
        quiet(&mut e, &[FN_00401000, FN_0040C150]);
        let calls = run(&mut e, 0x00f7_dd10);
        assert!(only(&calls, FN_0040C150).is_empty());
        assert_eq!(only(&calls, FN_00401000).len(), 9);
        assert_eq!(e.global::<u32>(0x011d_5240), 0);
    }

    #[test]
    fn fn_00f90480_runs_five_constructors_then_atexit() {
        let mut e = Engine::new();
        quiet(&mut e, &[FN_0045CFE0, ATEXIT]);
        let calls = run(&mut e, 0x00f9_0480);
        let mut expected: Calls = PAIRS_00F90480
            .iter()
            .map(|&(g, a, b)| (FN_0045CFE0, vec![g, a, b]))
            .collect();
        expected.push((ATEXIT, vec![0x00fd_7f30]));
        assert_eq!(calls, expected);
    }

    #[test]
    fn fn_00fb33f0_stores_the_vtables_and_registers_with_the_settings() {
        const COLLECTION: u32 = 0x0900_0000;
        const VTABLE: u32 = 0x0900_0100;
        const ADD: u32 = 0x0900_0200;
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.map(SETTINGS_COLLECTION & !0xfff, 0x1000);
        e.map(COLLECTION, 0x1000);
        e.set_global::<u32>(SETTINGS_COLLECTION, COLLECTION);
        e.set_global::<u32>(COLLECTION, VTABLE);
        e.set_global::<u32>(VTABLE + 4, ADD);
        quiet(&mut e, &[SETTINGS_GETTER_0044F570, ADD, ATEXIT]);
        let calls = run(&mut e, 0x00fb_33f0);
        assert_eq!(
            calls,
            vec![
                (SETTINGS_GETTER_0044F570, vec![]),
                (ADD, vec![COLLECTION, 0x011f_3368]),
                (ATEXIT, vec![0x00fd_a210]),
            ]
        );
        assert_eq!(e.global::<u32>(0x011f_3368), 0x0101_7720);
        assert_eq!(e.global::<u32>(0x011f_336c), 0x0109_4eb8);
        assert_eq!(e.global::<u32>(0x011f_3370), 0x0109_4ed8);
    }

    #[test]
    fn float_settings_pass_the_float_quieted() {
        type Case = (u32, u32, u32, u32, u32);
        let cases: [Case; 3] = [
            (
                0x00fb_cd10,
                0x0126_7bd4,
                0x010c_4af4,
                0x010c_4b18,
                0x00fd_de60,
            ),
            (
                0x00fb_d0a0,
                0x0126_7d08,
                0x010c_5034,
                0x0102_23ec,
                0x00fd_dfc0,
            ),
            (
                0x00fb_d160,
                0x0126_7d38,
                0x010c_5090,
                0x0101_6248,
                0x00fd_e000,
            ),
        ];
        for (entry, global, name, value, exit_fn) in cases {
            for (bits, passed) in [
                (0x3f80_0000_u32, 0x3f80_0000_u32),
                (0x7f80_0001, 0x7fc0_0001),
            ] {
                let mut e = Engine::new();
                e.map(value & !0xfff, 0x1000);
                e.set_global::<u32>(value, bits);
                quiet(&mut e, &[FN_0044F440, ATEXIT]);
                let calls = run(&mut e, entry);
                assert_eq!(
                    calls,
                    vec![
                        (FN_0044F440, vec![global, name, passed]),
                        (ATEXIT, vec![exit_fn]),
                    ]
                );
            }
        }
    }

    #[test]
    fn fn_00fbdde0_copies_64_bytes() {
        let mut e = Engine::new();
        e.map(0x011b_1000, 0x1000);
        e.map(0x0126_8000, 0x1000);
        for i in 0..16_u32 {
            e.set_global::<u32>(0x011b_1c50 + 4 * i, 0x1000 + i);
        }
        e.set_global::<u32>(0x0126_83b0, 0xdead);
        run(&mut e, 0x00fb_dde0);
        for i in 0..16_u32 {
            assert_eq!(e.global::<u32>(0x0126_8370 + 4 * i), 0x1000 + i);
        }
        assert_eq!(e.global::<u32>(0x0126_83b0), 0xdead);
    }

    #[test]
    fn fn_00fc791c_runs_00ec3a1b_on_the_stream() {
        let mut e = Engine::new();
        quiet(&mut e, &[FN_00EC3A1B]);
        assert_eq!(
            run(&mut e, 0x00fc_791c),
            vec![(FN_00EC3A1B, vec![0x0126_fc48])]
        );
    }

    #[test]
    fn every_function_is_registered_once() {
        let funcs = funcs();
        let addrs: Vec<u32> = funcs.iter().map(|(a, _)| *a).collect();
        assert_eq!(addrs.len(), 10);
        assert!(
            addrs.windows(2).all(|w| w[0] < w[1]),
            "ascending and unique"
        );
        let e = Engine::new();
        for a in addrs {
            assert!(e.is_translated(a), "{a:08x}");
        }
    }
}
