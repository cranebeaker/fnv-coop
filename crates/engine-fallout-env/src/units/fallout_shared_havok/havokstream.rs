//! `fallout shared/havok/havokstream.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds the Havok start-up (`InitHavokSystem`), the game's Havok
//! error handler (`HavokError`, vtable `0104def4`, 0x124 bytes), the file
//! system object Havok opens streams through (`HavokStreambufFactory`, vtable
//! `0104dea4`) and the file stream reader and writer (`HavokFileStreambufReader`
//! and `HavokFileStreambufWriter`, vtables `0104e014` and `0104e054`, 0x10
//! bytes each: `pFile` at +8 and `bOwned` at +0xC). The class names are those
//! of the RTTI records the vtables point to.
//!
//! `HavokError` keeps the reports Havok may not print as an `hkArray` of
//! (file name, line) pairs at +8 and the ids it may not print as an
//! `hkPointerMap` at +0x14; both are reached through their `hkArray` and
//! `hkMap` helpers in other units, called by address.
//!
//! A `hkBool` argument is a stack word whose low byte is the value; it is
//! made and read through the game's own `hkBool` constructor (`00622570`) and
//! getter (`005609f0`).
//!
//! The exception-unwinding frames (`FS:[0]`) and the `/GS` cookie checks
//! (`00ec408c`) of the functions marked `Translation incomplete` are not
//! modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00cb1fb0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const COUNT_TARGET_00CB1FB0: u32 = 0x00cb_1fb0;

// Translated from 006264b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (only caller `006255f0`): forwards `this` and one integer to
/// `00cb1fb0` (Havok SDK `hkmap.cpp` in the engine map), which compares `00cb18f0(count)` with
/// the field at `this+8` and returns `EAX` = 1 when it is larger, else calls `00cb19b0` and
/// returns `EAX` = 0 (`MOV EAX,1` / `XOR EAX,EAX`, a full-register result). The wrapper
/// returns it unchanged.
pub fn fn_006264b0(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    e.call(COUNT_TARGET_00CB1FB0, &args![this, count]).u32()
}

/// `strstr` (`00ec7750`), `hkString::strCasecmp` (`00cb0fe0`), `hkString::strRchr`
/// (`00cb1150`) and `hkString::strDup` (`00cb1410`), all cdecl.
const STRSTR: u32 = 0x00ec_7750;
const STR_CASECMP: u32 = 0x00cb_0fe0;
const STR_RCHR: u32 = 0x00cb_1150;
const STR_DUP: u32 = 0x00cb_1410;
/// Bounded string copy and append (`00406d30`, `00406d50`: destination, size,
/// source) and the counted copy `(destination, source, count)` (`004add50`).
const STR_COPY: u32 = 0x0040_6d30;
const STR_CAT: u32 = 0x0040_6d50;
const COPY_BYTES: u32 = 0x004a_dd50;
/// `_splitpath_s` (`00462d40`, cdecl): path, drive and directory (unused),
/// file name buffer and size, extension buffer and size.
const SPLIT_PATH: u32 = 0x0046_2d40;
/// `"TES4"` and `"tes4"`: the folder the engine's source tree is rooted at.
const TES4_UPPER: u32 = 0x0101_c664;
const TES4_LOWER: u32 = 0x0104_de98;
/// `"TES4\\Havok\\SDK\\Include\\"`, `" : "`, `"%s:%d: %s : %s'"` and the two
/// separator characters `"/\\"`.
const HAVOK_INCLUDE_PATH: u32 = 0x0104_df58;
const TEXT_SEPARATOR: u32 = 0x0104_df70;
const LOG_FORMAT: u32 = 0x0104_df74;
const PATH_SEPARATORS: u32 = 0x0104_df84;
/// `"hkVisualDebugger.cpp"`: the file whose reports are switched off at start-up.
const VISUAL_DEBUGGER_FILE: u32 = 0x0104_ded8;
/// The labels `message` passes to the report handlers.
const LABEL_ERROR: u32 = 0x0104_df38;
const LABEL_ASSERT: u32 = 0x0104_df40;
const LABEL_WARNING: u32 = 0x0104_df48;
const LABEL_REPORT: u32 = 0x0104_df50;
/// The source file name `InitHavokSystem` gives its scope guard.
const SOURCE_FILE_NAME: u32 = 0x0104_de50;

/// Vtables the constructors install.
const VTABLE_FACTORY: u32 = 0x0104_dea4;
const VTABLE_FILE_SYSTEM: u32 = 0x0104_dec0;
const VTABLE_ERROR: u32 = 0x0104_def4;
const VTABLE_ERROR_BASE: u32 = 0x0104_df8c;
const VTABLE_STREAM_READER_BASE: u32 = 0x0104_dfb4;
const VTABLE_STREAM_WRITER_BASE: u32 = 0x0104_dfe8;
const VTABLE_FILE_READER: u32 = 0x0104_e014;
const VTABLE_FILE_WRITER: u32 = 0x0104_e054;

/// `hkBool::hkBool` (`00622570`, `this` = the byte, one stack word) and the
/// `hkBool` getter (`005609f0`, `this` = the byte).
const HK_BOOL_CONSTRUCT: u32 = 0x0062_2570;
const HK_BOOL_GET: u32 = 0x0056_09f0;
/// Helpers of the `hkArray<hkFileLine>` and `hkMap` templates, in other
/// units: the array's size getter (`00726070`, the word at +4), element
/// address `(index)` (`00626660`), constructor, destructor, append
/// (`this`, allocator, element), element destructor and element copy; the
/// map's constructor, insert, remove, lookup, clear and
/// clear-and-deallocate.
const ARRAY_SIZE: u32 = 0x0072_6070;
const ARRAY_ELEMENT: u32 = 0x0062_6660;
const ARRAY_CONSTRUCT: u32 = 0x0062_99a0;
const ARRAY_DESTRUCT: u32 = 0x0057_c530;
const ARRAY_APPEND: u32 = 0x0062_66f0;
const ARRAY_ELEMENT_DESTRUCT: u32 = 0x0062_6790;
const ARRAY_ELEMENT_COPY: u32 = 0x0062_67c0;
const MAP_CONSTRUCT: u32 = 0x0062_6560;
const MAP_INSERT: u32 = 0x0062_65c0;
const MAP_REMOVE: u32 = 0x0062_64b0;
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const MAP_CLEAR: u32 = 0x00cb_1a60;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
/// The container allocator accessor (`004a4840`: `this` = a one-byte temporary,
/// one stack word = the container) and the lock call the PC build compiled to
/// a bare `RET` (`00a29680`).
const CONTAINER_ALLOCATOR: u32 = 0x004a_4840;
const EMPTY_LOCK: u32 = 0x00a2_9680;
/// `hkMemoryRouter` lookup (`00c85750`), its allocator accessor (`0044edb0`)
/// and the allocator's free (`00c856c0`, cdecl `(allocator, pointer)`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const ROUTER_ALLOCATOR: u32 = 0x0044_edb0;
const ALLOCATOR_FREE: u32 = 0x00c8_56c0;
/// Allocation of a Havok-side object (`0056d280`, cdecl `(size)`) and the
/// reference-counted base's destructor body (`0056d6c0`), constructor
/// (`00538ef0`) and release of the memory (`00538eb0`).
const HAVOK_OBJECT_ALLOCATE: u32 = 0x0056_d280;
const REFERENCED_OBJECT_DESTRUCT: u32 = 0x0056_d6c0;
const REFERENCED_OBJECT_CONSTRUCT: u32 = 0x0053_8ef0;
const REFERENCED_OBJECT_FREE: u32 = 0x0053_8eb0;
/// The singleton member constructor body that returns `this` (`006815c0`).
const SINGLETON_CONSTRUCT: u32 = 0x0068_15c0;
/// Release of a Havok singleton's reference (`00c903b0`).
const REMOVE_REFERENCE: u32 = 0x00c9_03b0;
/// The game's operator new (`00401000`) and `BSFile::BSFile` (`00b00260`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const BS_FILE_CONSTRUCT: u32 = 0x00b0_0260;
/// `TESFile` read and write (`00462d80`, `00473180`: `this`, buffer, size),
/// its tell (`004720a0`) and the file's ok test (`008905f0`).
const FILE_READ: u32 = 0x0046_2d80;
const FILE_WRITE: u32 = 0x0047_3180;
const FILE_TELL: u32 = 0x0047_20a0;
const FILE_IS_OK: u32 = 0x0089_05f0;
/// The report printer (`005b5e40`, cdecl: level, format, arguments).
const LOG_PRINT: u32 = 0x005b_5e40;
/// The word `006259f0` returns: the message handler object.
const MESSAGE_HANDLER: u32 = 0x0120_2d74;
/// The assert handler singleton getter (`0043bd00`).
const ASSERT_HANDLER: u32 = 0x0043_bd00;
/// `bhkWorld::InitSystem` (`00c68230`), `hkMonitorStream::init` (`00cb0db0`),
/// `bhkWorld::Shutdown` (`00c66670`) and the world's update-mode setter
/// (`00c661d0`).
const BHK_WORLD_INIT_SYSTEM: u32 = 0x00c6_8230;
const HK_MONITOR_STREAM_INIT: u32 = 0x00cb_0db0;
const BHK_WORLD_SHUTDOWN: u32 = 0x00c6_6670;
const BHK_WORLD_SET_UPDATE_MODE: u32 = 0x00c6_61d0;
/// The settings guard `InitHavokSystem` opens (`00404eb0`) and closes
/// (`00404ee0`).
const GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// Getter of the pointer to the update-mode word (`0043d4d0`, `this` = the
/// holder at `011cc2e8`) and the import slot of `GetCurrentDirectoryA`
/// (stdcall: size, buffer).
const UPDATE_MODE_GETTER: u32 = 0x0043_d4d0;
const UPDATE_MODE_HOLDER: u32 = 0x011c_c2e8;
const GET_CURRENT_DIRECTORY: u32 = 0x00fd_f0c0;

/// The globals holding the `HavokError` (`01268360`) and the
/// `HavokStreambufFactory` (`01268364`) singletons.
const ERROR_SINGLETON: u32 = 0x0126_8360;
const FACTORY_SINGLETON: u32 = 0x0126_8364;

/// Ids `HavokError::HavokError` switches off (`setEnabled(id, false)`), in order.
const INITIALLY_DISABLED_IDS: [u32; 41] = [
    0x6e8d_163b,
    0x6cee_9071,
    0xf0de_4355,
    0x2ff8_c16f,
    0xf0fe_4356,
    0x275e_c1fd,
    0x1ada_ad0e,
    0x70dc_41cb,
    0x1ff8_8f0e,
    0x475d_86b1,
    0x6fe8_4a9b,
    0x21c8_ab2a,
    0x3093_14d9,
    0xad67_fa3a,
    0xad67_fa3a,
    0xf032_de34,
    0x34df_5494,
    0xf02e_32e1,
    0xf02e_32e2,
    0xf02e_32df,
    0xf043_d534,
    0xf021_84ed,
    0xf02d_e43e,
    0xf02e_32e0,
    0xf021_32df,
    0xf021_32ff,
    0x5400_6467,
    0x717c_3b1d,
    0xf032_df30,
    0x0f02_13de,
    0xad89_66aa,
    0x2a1d_b936,
    0x68c4_e1dc,
    0x38f1_276d,
    0xf032_43df,
    0x2ff8_c16d,
    0xabba_efbf,
    0x761e_c287,
    0x53fe_2ebc,
    0x1f06_90f0,
    0x56f8_0833,
];
/// Lines of `hkVisualDebugger.cpp` whose reports `HavokError::HavokError`
/// switches off (`disableReport`), in order.
const INITIALLY_DISABLED_LINES: [u32; 11] = [
    0x26, 0x56, 0x5d, 0x6a, 0x6e, 0x73, 0x7e, 0x85, 0xbf, 0xf6, 0x115,
];

layout! {
    /// `HavokError` (Xbox PDB), 0x124 bytes: the Havok error handler.
    pub struct HavokError: 0x124 {
        /// `m_disabledReports` (Xbox PDB): `hkArray<HavokError::hkFileLine>`,
        /// its data pointer (the array starts here).
        0x08 m_disabledReports: u32,
        /// `m_disabledReports.m_size` (Xbox PDB).
        0x0C m_disabledReportsSize: u32,
        /// `m_disabledAssertIds` (Xbox PDB): the `hkPointerMap<int,int>`
        /// starts here (its element pointer).
        0x14 m_disabledAssertIds: u32,
        /// `pRootDir` (Xbox PDB): `char[260]`, the folder in front of `TES4`
        /// in the working directory; its first byte says whether it is set.
        0x20 pRootDir: u8,
    }

    /// `HavokError::hkFileLine` (Xbox PDB): a file name and a line.
    pub struct HkFileLine: 0x08 {
        /// `m_file` (Xbox PDB): `const char *`, a copy made by `strDup`.
        0x00 m_file: u32,
        /// `m_line` (Xbox PDB).
        0x04 m_line: i32,
    }

    /// `HavokFileStreambufReader` (Xbox PDB), 0x10 bytes.
    pub struct HavokFileStreambufReader: 0x10 {
        /// `pFile` (Xbox PDB): `BSFile *`.
        0x08 pFile: u32,
        /// `bOwned` (Xbox PDB).
        0x0C bOwned: u8,
    }

    /// `HavokFileStreambufWriter` (Xbox PDB), 0x10 bytes.
    pub struct HavokFileStreambufWriter: 0x10 {
        /// `pFile` (Xbox PDB): `BSFile *`.
        0x08 pFile: u32,
        /// `bOwned` (Xbox PDB).
        0x0C bOwned: u8,
    }
}

/// An `hkBool` stack word made by the game's constructor.
fn hk_bool_word(e: &mut Engine, value: bool) -> u32 {
    e.with_stack(4, |e, temp| {
        e.call(HK_BOOL_CONSTRUCT, &args![temp, value as u32]);
        e.mem.u32(temp.addr())
    })
}

/// `_splitpath_s(file, 0, 0, 0, 0, name, 0x80, extension, 0x20)` followed by
/// appending the extension to the name: the file's base name with extension.
fn base_name_with_extension(e: &mut Engine, file: Ptr, name: u32, extension: u32) {
    e.call(
        SPLIT_PATH,
        &args![file, 0u32, 0u32, 0u32, 0u32, name, 0x80u32, extension, 0x20u32],
    );
    e.call(STR_CAT, &args![name, 0x80u32, extension]);
}

// Translated from 00624bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InitHavokSystem` (engine map): inside a settings guard, starts the Havok
/// world system and the monitor stream, makes the `HavokError` and the
/// `HavokStreambufFactory` the Havok singletons, remembers the folder in
/// front of `TES4` in the working directory as the error handler's root
/// directory, and sets the world's update mode from the holder at `011cc2e8`.
// Translation incomplete: 00624bf0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
pub fn init_havok_system(e: &mut Engine) {
    e.with_stack(4, |e, guard| {
        e.call(
            GUARD_CONSTRUCT,
            &args![guard, 0x10u32, true, SOURCE_FILE_NAME, 0xacu32],
        );
        e.call(BHK_WORLD_INIT_SYSTEM, &args![]);
        e.call(HK_MONITOR_STREAM_INIT, &args![]);
        let block = e
            .call(HAVOK_OBJECT_ALLOCATE, &args![0x124u32])
            .ptr::<HavokError>();
        let error = if block.is_null() {
            Ptr::NULL
        } else {
            havok_error_havok_error(e, block)
        };
        fn_006263c0(e, error.addr());
        let block = e.call(HAVOK_OBJECT_ALLOCATE, &args![8u32]).ptr::<()>();
        let factory = if block.is_null() {
            Ptr::NULL
        } else {
            fn_00624de0(e, block)
        };
        fn_006263f0(e, factory.addr());
        e.with_stack(0x104, |e, directory| {
            e.call(GET_CURRENT_DIRECTORY, &args![0x104u32, directory]);
            fn_00624d70(e, error, directory);
        });
        let mode_word = e.call(UPDATE_MODE_GETTER, &args![UPDATE_MODE_HOLDER]).u32();
        let mode = e.mem.u32(mode_word);
        e.call(BHK_WORLD_SET_UPDATE_MODE, &args![mode]);
        e.call(GUARD_DESTRUCT, &args![guard]);
    });
}

// Translated from 00624d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the part of `path` in front of the first `TES4` (or else `tes4`) as
/// the error handler's root directory (`pRootDir`, +0x20), terminated; does
/// nothing when `path` has neither. Not named in the engine map.
pub fn fn_00624d70(e: &mut Engine, this: Ptr<HavokError>, path: Ptr) {
    let mut found = e.call(STRSTR, &args![path, TES4_UPPER]).u32();
    if found == 0 {
        found = e.call(STRSTR, &args![path, TES4_LOWER]).u32();
    }
    if found != 0 {
        let length = found.wrapping_sub(path.addr());
        let root = this.byte_add(HavokError::pRootDir.off);
        e.call(COPY_BYTES, &args![root, path, length]);
        e.mem
            .set_u8(this.addr().wrapping_add(length).wrapping_add(0x20), 0);
    }
}

// Translated from 00624de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `HavokStreambufFactory` (the RTTI name of the vtable it
/// installs): the file system base constructor, then the factory's vtable.
pub fn fn_00624de0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00624e00(e, this);
    e.mem.set_u32(this.addr(), VTABLE_FACTORY);
    this
}

// Translated from 00624e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `hkFileSystem` (the RTTI name of the vtable it installs):
/// the reference-counted base, the singleton member at +8, the vtable.
// Translation incomplete: 00624e00 - the FS:[0] exception-unwinding frame is not modeled.
pub fn fn_00624e00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(REFERENCED_OBJECT_CONSTRUCT, &args![this]);
    e.call(SINGLETON_CONSTRUCT, &args![this.byte_add(8)]);
    e.mem.set_u32(this.addr(), VTABLE_FILE_SYSTEM);
    this
}

// Translated from 00624e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0x14 of the factory's vtable (`listDirectory` in the Xbox PDB's
/// `hkFileSystem`): takes two words it never reads and returns 1.
pub fn fn_00624e70(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    1
}

// Translated from 00624e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokStreambufFactory::openReader` (engine map): allocates 0x10 bytes
/// and constructs a `HavokFileStreambufReader` in them (null when the
/// allocation fails). The name word goes to the reader constructor, which
/// ignores it.
// Translation incomplete: 00624e90 - the FS:[0] exception-unwinding frame is not modeled.
pub fn havok_streambuf_factory_open_reader(e: &mut Engine, _this: Ptr, name: u32) -> Ptr {
    let block = e.call(HAVOK_OBJECT_ALLOCATE, &args![0x10u32]).ptr::<()>();
    if block.is_null() {
        Ptr::NULL
    } else {
        fn_00626000(e, block, name)
    }
}

// Translated from 00624f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokStreambufFactory::openWriter` (engine map): allocates 0x10 bytes
/// and constructs a `HavokFileStreambufWriter` for the file name in them
/// (null when the allocation fails).
// Translation incomplete: 00624f10 - the FS:[0] exception-unwinding frame is not modeled.
pub fn havok_streambuf_factory_open_writer(e: &mut Engine, _this: Ptr, file_name: u32) -> Ptr {
    let block = e.call(HAVOK_OBJECT_ALLOCATE, &args![0x10u32]).ptr::<()>();
    if block.is_null() {
        Ptr::NULL
    } else {
        havok_file_streambuf_writer_havok_file_streambuf_writer(e, block, file_name)
    }
}

// Translated from 00624f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::HavokError` (engine map): the `hkError` base, the
/// `HavokError` vtable, the empty report array and id map, then 41 ids
/// switched off (`setEnabled(id, false)`; the id `0xad67fa3a` twice), the
/// reports of eleven lines of `hkVisualDebugger.cpp` disabled, and an empty
/// root directory. Returns `this`.
// Translation incomplete: 00624f90 - the FS:[0] exception-unwinding frame is not modeled.
pub fn havok_error_havok_error(e: &mut Engine, this: Ptr<HavokError>) -> Ptr<HavokError> {
    fn_00625f10(e, this.cast());
    e.mem.set_u32(this.addr(), VTABLE_ERROR);
    e.call(ARRAY_CONSTRUCT, &args![this.byte_add(0x08)]);
    fn_00626450(e, this.byte_add(HavokError::m_disabledAssertIds.off));
    for id in INITIALLY_DISABLED_IDS {
        let disabled = hk_bool_word(e, false);
        havok_error_set_enabled(e, this, id, disabled);
    }
    for line in INITIALLY_DISABLED_LINES {
        havok_error_disable_report(e, this, Ptr::new(VISUAL_DEBUGGER_FILE), line);
    }
    e.set(this, HavokError::pRootDir, 0);
    this
}

// Translated from 006255d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::setEnabled` (engine map): `enabled` is an `hkBool` word.
/// Enabling an id removes it from the disabled map, disabling it stores it
/// there with the value 1.
pub fn havok_error_set_enabled(e: &mut Engine, this: Ptr<HavokError>, id: u32, enabled: u32) {
    let map = this.byte_add(HavokError::m_disabledAssertIds.off);
    let on = e.with_stack(4, |e, temp| {
        e.mem.set_u32(temp.addr(), enabled);
        e.call(HK_BOOL_GET, &args![temp]).bool()
    });
    if on {
        e.call(MAP_REMOVE, &args![map, id]);
    } else {
        fn_00626470(e, map, id, 1);
    }
}

// Translated from 00625610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::isEnabled` (engine map): writes into the `hkBool` at `result`
/// whether the id is absent from the disabled map (default 0), and returns
/// `result`.
pub fn havok_error_is_enabled(e: &mut Engine, this: Ptr<HavokError>, result: Ptr, id: u32) -> Ptr {
    let map = this.byte_add(HavokError::m_disabledAssertIds.off);
    let value = fn_00626490(e, map, id, 0);
    e.call(HK_BOOL_CONSTRUCT, &args![result, (value == 0) as u32]);
    result
}

// Translated from 00625650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::enableAll` (engine map): clears the disabled-id map.
pub fn havok_error_enable_all(e: &mut Engine, this: Ptr<HavokError>) {
    fn_006264d0(e, this.byte_add(HavokError::m_disabledAssertIds.off));
}

// Translated from 00625670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::message` (engine map): with id -1 the report must not be on
/// the disabled list for its file and line; with any other id the id must be
/// enabled (virtual `isEnabled`, slot +0x14). Then, by severity (0 report,
/// 1 warning, 2 and 3 assert, others error), calls the report handler of the
/// vtable (slots +0x38, +0x34, +0x2c, +0x30) with the label, id, text, file
/// and line and returns true. Returns false when nothing is reported.
pub fn havok_error_message(
    e: &mut Engine,
    this: Ptr<HavokError>,
    severity: u32,
    id: u32,
    text: u32,
    file: u32,
    line: u32,
) -> bool {
    if id == u32::MAX {
        if !havok_error_is_report_enabled(e, this, Ptr::new(file), line) {
            return false;
        }
    } else {
        let enabled = e.with_stack(4, |e, result| {
            let hk_bool = e.vcall(this.addr(), 0x14, &args![result, id]).ptr::<()>();
            e.call(HK_BOOL_GET, &args![hk_bool]).bool()
        });
        if !enabled {
            return false;
        }
    }
    let (slot, label) = match severity {
        0 => (0x38, LABEL_REPORT),
        1 => (0x34, LABEL_WARNING),
        2 | 3 => (0x2c, LABEL_ASSERT),
        _ => (0x30, LABEL_ERROR),
    };
    e.vcall(this.addr(), slot, &args![label, id, text, file, line]);
    true
}

// Translated from 00625790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::assertMessage` (engine map): builds `label` (or nothing) plus
/// `" : "` and `text` in a 1024-byte buffer; when a root directory is set,
/// shows the file as the root directory plus the part of `file` from `TES4`
/// (or `tes4`) on, or, for a name starting `..`, plus
/// `TES4\Havok\SDK\Include\` and the name from its third character; shows the
/// plain file otherwise. Asks the assert handler (slot +0xC, given the file,
/// line and text) what to do, and when it answers 5 calls `setEnabled(id,
/// false)` (slot +0x10).
// Translation incomplete: 00625790 - the /GS cookie check (00ec408c) is not modeled.
pub fn havok_error_assert_message(
    e: &mut Engine,
    this: Ptr<HavokError>,
    label: u32,
    id: u32,
    text: u32,
    file: u32,
    line: u32,
) {
    // The exe's frame (`SUB ESP,0x520` under `EBP`): the message at +0x118 (`EBP-0x408`) and
    // the path at +0x10 (`EBP-0x510`).
    e.with_stack(0x520, |e, frame| {
        let buffer = frame.addr() + 0x118;
        let path = frame.addr() + 0x10;
        if label == 0 {
            e.mem.set_u8(buffer, 0);
        } else {
            e.call(STR_COPY, &args![buffer, 0x400u32, label]);
        }
        if text != 0 {
            e.call(STR_CAT, &args![buffer, 0x400u32, TEXT_SEPARATOR]);
            e.call(STR_CAT, &args![buffer, 0x400u32, text]);
        }
        let root = this.byte_add(HavokError::pRootDir.off);
        let mut shown = 0;
        if e.get(this, HavokError::pRootDir) != 0 {
            let mut found = e.call(STRSTR, &args![file, TES4_UPPER]).u32();
            if found == 0 {
                found = e.call(STRSTR, &args![file, TES4_LOWER]).u32();
            }
            if found != 0 {
                e.call(STR_COPY, &args![path, 0x104u32, root]);
                e.call(STR_CAT, &args![path, 0x104u32, found]);
                shown = path;
            } else if e.mem.u8(file) == b'.' && e.mem.u8(file + 1) == b'.' {
                e.call(STR_COPY, &args![path, 0x104u32, root]);
                e.call(STR_CAT, &args![path, 0x104u32, HAVOK_INCLUDE_PATH]);
                e.call(STR_CAT, &args![path, 0x104u32, file + 3]);
                shown = path;
            }
        }
        if shown == 0 {
            shown = file;
        }
        let handler = e.call(ASSERT_HANDLER, &args![]).u32();
        let answer = e.vcall(handler, 0xc, &args![shown, line, buffer]).i32();
        if answer == 5 {
            let disabled = hk_bool_word(e, false);
            e.vcall(this.addr(), 0x10, &args![id, disabled]);
        }
    });
}

// Translated from 006259b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::errorMessage` (engine map): prints `file:line: label : text'`
/// at report level 0. The id word is not read.
pub fn havok_error_error_message(
    e: &mut Engine,
    _this: Ptr<HavokError>,
    label: u32,
    _unused_2: u32,
    text: u32,
    file: u32,
    line: u32,
) {
    fn_006259f0(e);
    e.call(LOG_PRINT, &args![0u32, LOG_FORMAT, file, line, label, text]);
}

// Translated from 006259f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `01202d74` (the engine map names the function
/// `MessageHandler::Send`, but its body is only this load; the report
/// functions call it and drop the result).
pub fn fn_006259f0(e: &mut Engine) -> u32 {
    e.global::<u32>(MESSAGE_HANDLER)
}

// Translated from 00625a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::warningMessage` (engine map): like `errorMessage` at report
/// level 1.
pub fn havok_error_warning_message(
    e: &mut Engine,
    _this: Ptr<HavokError>,
    label: u32,
    _unused_2: u32,
    text: u32,
    file: u32,
    line: u32,
) {
    fn_006259f0(e);
    e.call(LOG_PRINT, &args![1u32, LOG_FORMAT, file, line, label, text]);
}

// Translated from 00625a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::programFlowMessage` (engine map): like `errorMessage` at
/// report level 2.
pub fn havok_error_program_flow_message(
    e: &mut Engine,
    _this: Ptr<HavokError>,
    label: u32,
    _unused_2: u32,
    text: u32,
    file: u32,
    line: u32,
) {
    fn_006259f0(e);
    e.call(LOG_PRINT, &args![2u32, LOG_FORMAT, file, line, label, text]);
}

// Translated from 00625a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::showString` (engine map): prints the string at report level 4.
pub fn havok_error_show_string(e: &mut Engine, _this: Ptr<HavokError>, text: u32) {
    fn_006259f0(e);
    e.call(LOG_PRINT, &args![4u32, text]);
}

// Translated from 00625aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::hkGetFileBaseName` (engine map): for each of the separators
/// `/` and `\`, moves `path` to the character after its last occurrence
/// (null when there is none; the next search still runs on the null).
pub fn havok_error_hk_get_file_base_name(e: &mut Engine, _this: Ptr<HavokError>, path: u32) -> u32 {
    let mut path = path;
    for index in 0..2u32 {
        let separator = e.mem.i8(PATH_SEPARATORS + index) as i32;
        let found = e.call(STR_RCHR, &args![path, separator]).u32();
        path = if found == 0 { 0 } else { found + 1 };
    }
    path
}

// Translated from 00625b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::__scalar_deleting_destructor` (engine map): runs the
/// destructor body and, with bit 0 of `flags` set, frees the object. Returns
/// `this`.
pub fn havok_error_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HavokError>,
    flags: u32,
) -> Ptr<HavokError> {
    fn_00625b40(e, this);
    if flags & 1 != 0 {
        e.call(REFERENCED_OBJECT_FREE, &args![this]);
    }
    this
}

// Translated from 00625b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `HavokError`: restores the vtable, frees the file name
/// of each disabled report, destroys the id map and the report array, then the
/// reference-counted base. Not named in the engine map.
// Translation incomplete: 00625b40 - the FS:[0] exception-unwinding frame is not modeled.
pub fn fn_00625b40(e: &mut Engine, this: Ptr<HavokError>) {
    e.mem.set_u32(this.addr(), VTABLE_ERROR);
    let reports = this.byte_add(HavokError::m_disabledReports.off);
    fn_00625bf0(e, this.cast(), reports);
    fn_00625bd0(e, this.byte_add(HavokError::m_disabledAssertIds.off));
    e.call(ARRAY_DESTRUCT, &args![reports]);
    e.call(REFERENCED_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00625bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the `hkPointerMap` at `this` (its destructor body, `006264f0`).
pub fn fn_00625bd0(e: &mut Engine, this: Ptr) {
    fn_006264f0(e, this);
}

// Translated from 00625bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the file name of every element of the `hkArray<hkFileLine>` at
/// `array` (`this`, the error handler, is not read).
pub fn fn_00625bf0(e: &mut Engine, _this: Ptr, array: Ptr) {
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![array]).i32() {
        let element = e
            .call(ARRAY_ELEMENT, &args![array, index])
            .ptr::<HkFileLine>();
        let file = e.get(element, HkFileLine::m_file);
        fn_00626760(e, file);
        index += 1;
    }
}

// Translated from 00625c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::isReportEnabled` (engine map): false when the disabled list
/// has an entry with this file's base name (name and extension, compared
/// without regard to case) and this line, true otherwise.
// Translation incomplete: 00625c50 - the /GS cookie check (00ec408c) is not modeled.
pub fn havok_error_is_report_enabled(
    e: &mut Engine,
    this: Ptr<HavokError>,
    file: Ptr,
    line: u32,
) -> bool {
    let reports = this.byte_add(HavokError::m_disabledReports.off);
    if e.call(ARRAY_SIZE, &args![reports]).u32() == 0 {
        return true;
    }
    e.with_stack(0xa0, |e, locals| {
        let name = locals.addr();
        base_name_with_extension(e, file, name, name + 0x80);
        let mut index = 0;
        while index < e.call(ARRAY_SIZE, &args![reports]).i32() {
            let element = e
                .call(ARRAY_ELEMENT, &args![reports, index])
                .ptr::<HkFileLine>();
            let report_file = e.get(element, HkFileLine::m_file);
            let report_line = e.get(element, HkFileLine::m_line);
            let same_file = e.call(STR_CASECMP, &args![name, report_file]).i32() == 0;
            if same_file && line == report_line as u32 {
                return false;
            }
            index += 1;
        }
        true
    })
}

// Translated from 00625d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::enableReport` (engine map): finds the disabled-list entry for
/// this file's base name and line, frees its file name and removes it.
// Translation incomplete: 00625d60 - the /GS cookie check (00ec408c) is not modeled.
pub fn havok_error_enable_report(e: &mut Engine, this: Ptr<HavokError>, file: Ptr, line: u32) {
    let reports = this.byte_add(HavokError::m_disabledReports.off);
    e.with_stack(0xa0, |e, locals| {
        let name = locals.addr();
        base_name_with_extension(e, file, name, name + 0x80);
        let mut index = 0;
        while index < e.call(ARRAY_SIZE, &args![reports]).i32() {
            let element = e
                .call(ARRAY_ELEMENT, &args![reports, index])
                .ptr::<HkFileLine>();
            let report_file = e.get(element, HkFileLine::m_file);
            let report_line = e.get(element, HkFileLine::m_line);
            let same_file = e.call(STR_CASECMP, &args![name, report_file]).i32() == 0;
            if same_file && line == report_line as u32 {
                fn_00626760(e, report_file);
                fn_00626680(e, reports, index as u32);
                return;
            }
            index += 1;
        }
    });
}

// Translated from 00625e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokError::disableReport` (engine map): appends (a copy of this file's
/// base name with extension, `line`) to the disabled list.
// Translation incomplete: 00625e70 - the /GS cookie check (00ec408c) is not modeled.
pub fn havok_error_disable_report(e: &mut Engine, this: Ptr<HavokError>, file: Ptr, line: u32) {
    let reports = this.byte_add(HavokError::m_disabledReports.off);
    e.with_stack(0xa0 + 8, |e, locals| {
        let name = locals.addr();
        base_name_with_extension(e, file, name, name + 0x80);
        let copy = e.call(STR_DUP, &args![name]).u32();
        let entry = locals.byte_add(0xa0).cast::<HkFileLine>();
        e.set(entry, HkFileLine::m_file, copy);
        e.set(entry, HkFileLine::m_line, line as i32);
        fn_00626420(e, reports, entry.addr());
    });
}

// Translated from 00625f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `hkError` (the RTTI name of the vtable it installs): the
/// reference-counted base, the array member at +8, the vtable.
// Translation incomplete: 00625f10 - the FS:[0] exception-unwinding frame is not modeled.
pub fn fn_00625f10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(REFERENCED_OBJECT_CONSTRUCT, &args![this]);
    e.call(SINGLETON_CONSTRUCT, &args![this.byte_add(8)]);
    e.mem.set_u32(this.addr(), VTABLE_ERROR_BASE);
    this
}

// Translated from 00625f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of `hkError`: the reference-counted base's destructor
/// body, then (bit 0 of `flags`) freeing the object. Returns `this`.
pub fn fn_00625f80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(REFERENCED_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(REFERENCED_OBJECT_FREE, &args![this]);
    }
    this
}

// Translated from 00625fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shuts the Havok world system down (`bhkWorld::Shutdown`, `00c66670`).
pub fn fn_00625fb0(e: &mut Engine) {
    e.call(BHK_WORLD_SHUTDOWN, &args![]);
}

// Translated from 00625fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `hkStreamReader` (the RTTI name of the vtable it installs):
/// the reference-counted base, then the vtable.
pub fn fn_00625fc0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(REFERENCED_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_STREAM_READER_BASE);
    this
}

// Translated from 00625fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `hkStreamWriter` (the RTTI name of the vtable it installs):
/// the reference-counted base, then the vtable.
pub fn fn_00625fe0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(REFERENCED_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_STREAM_WRITER_BASE);
    this
}

// Translated from 00626000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `HavokFileStreambufReader` (the RTTI name of the vtable it
/// installs): the stream reader base, the reader's vtable and `bOwned` = 1.
/// Its one parameter is never read, and `pFile` is left as the base left it.
pub fn fn_00626000(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    fn_00625fc0(e, this);
    e.mem.set_u32(this.addr(), VTABLE_FILE_READER);
    e.set(
        this.cast::<HavokFileStreambufReader>(),
        HavokFileStreambufReader::bOwned,
        1,
    );
    this
}

// Translated from 00626030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokFileStreambufReader::__scalar_deleting_destructor` (engine map):
/// runs the destructor body and, with bit 0 of `flags` set, frees the
/// object. Returns `this`.
pub fn havok_file_streambuf_reader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HavokFileStreambufReader>,
    flags: u32,
) -> Ptr<HavokFileStreambufReader> {
    fn_00626060(e, this);
    if flags & 1 != 0 {
        e.call(REFERENCED_OBJECT_FREE, &args![this]);
    }
    this
}

// Translated from 00626060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `HavokFileStreambufReader`: restores the vtable, deletes
/// the file through its deleting destructor (slot 0, argument 1) when it is
/// set and owned, then the reference-counted base. The compiler's second null
/// test of the file is folded into the first. Not named in the engine map.
// Translation incomplete: 00626060 - the FS:[0] exception-unwinding frame is not modeled.
pub fn fn_00626060(e: &mut Engine, this: Ptr<HavokFileStreambufReader>) {
    e.mem.set_u32(this.addr(), VTABLE_FILE_READER);
    let file = e.get(this, HavokFileStreambufReader::pFile);
    if file != 0 && e.get(this, HavokFileStreambufReader::bOwned) != 0 {
        e.vcall(file, 0, &args![1u32]);
    }
    e.call(REFERENCED_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00626100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokFileStreambufReader::read` (engine map): reads `size` bytes into
/// `buffer` through the file's read (`00462d80`); returns its result.
pub fn havok_file_streambuf_reader_read(
    e: &mut Engine,
    this: Ptr<HavokFileStreambufReader>,
    buffer: u32,
    size: u32,
) -> u32 {
    let file = e.get(this, HavokFileStreambufReader::pFile);
    e.call(FILE_READ, &args![file, buffer, size]).u32()
}

// Translated from 00626120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokFileStreambufReader::sysGetFileSize` (engine map): the file's size
/// (its slot +0x28), or -1 without a file.
pub fn havok_file_streambuf_reader_sys_get_file_size(
    e: &mut Engine,
    this: Ptr<HavokFileStreambufReader>,
) -> u32 {
    let file = e.get(this, HavokFileStreambufReader::pFile);
    if file == 0 {
        u32::MAX
    } else {
        e.vcall(file, 0x28, &args![]).u32()
    }
}

// Translated from 00626160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokFileStreambufReader::isOk` (engine map): writes into the `hkBool` at
/// `result` whether the file exists and its ok test (`008905f0`) holds, and
/// returns `result`.
pub fn havok_file_streambuf_reader_is_ok(
    e: &mut Engine,
    this: Ptr<HavokFileStreambufReader>,
    result: Ptr,
) -> Ptr {
    let file = e.get(this, HavokFileStreambufReader::pFile);
    let ok = if file == 0 {
        0
    } else {
        e.call(FILE_IS_OK, &args![file]).u8()
    };
    e.call(HK_BOOL_CONSTRUCT, &args![result, ok as u32]);
    result
}

// Translated from 006261a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokFileStreambufWriter::HavokFileStreambufWriter` (engine map): the
/// stream writer base, the writer's vtable, `bOwned` = 1, and a new
/// `BSFile` (0x158 bytes) opened on `file_name` (mode 1, buffer 0x4000, 0) as
/// `pFile` (null when the allocation fails). Returns `this`.
// Translation incomplete: 006261a0 - the FS:[0] exception-unwinding frame is not modeled.
pub fn havok_file_streambuf_writer_havok_file_streambuf_writer(
    e: &mut Engine,
    this: Ptr,
    file_name: u32,
) -> Ptr {
    fn_00625fe0(e, this);
    e.mem.set_u32(this.addr(), VTABLE_FILE_WRITER);
    let writer = this.cast::<HavokFileStreambufWriter>();
    e.set(writer, HavokFileStreambufWriter::bOwned, 1);
    let block = e.call(OPERATOR_NEW, &args![0x158u32]).u32();
    let file = if block == 0 {
        0
    } else {
        e.call(
            BS_FILE_CONSTRUCT,
            &args![block, file_name, 1u32, 0x4000u32, 0u32],
        )
        .u32()
    };
    e.set(writer, HavokFileStreambufWriter::pFile, file);
    this
}

// Translated from 00626250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokFileStreambufWriter::__scalar_deleting_destructor` (engine map):
/// runs the destructor body and, with bit 0 of `flags` set, frees the
/// object. Returns `this`.
pub fn havok_file_streambuf_writer_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HavokFileStreambufWriter>,
    flags: u32,
) -> Ptr<HavokFileStreambufWriter> {
    fn_00626280(e, this);
    if flags & 1 != 0 {
        e.call(REFERENCED_OBJECT_FREE, &args![this]);
    }
    this
}

// Translated from 00626280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `HavokFileStreambufWriter`: like the reader's, with the
/// writer's vtable. Not named in the engine map.
// Translation incomplete: 00626280 - the FS:[0] exception-unwinding frame is not modeled.
pub fn fn_00626280(e: &mut Engine, this: Ptr<HavokFileStreambufWriter>) {
    e.mem.set_u32(this.addr(), VTABLE_FILE_WRITER);
    let file = e.get(this, HavokFileStreambufWriter::pFile);
    if file != 0 && e.get(this, HavokFileStreambufWriter::bOwned) != 0 {
        e.vcall(file, 0, &args![1u32]);
    }
    e.call(REFERENCED_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00626320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokFileStreambufWriter::write` (engine map): writes `size` bytes from
/// `buffer` through the file's write (`00473180`); returns its result.
pub fn havok_file_streambuf_writer_write(
    e: &mut Engine,
    this: Ptr<HavokFileStreambufWriter>,
    buffer: u32,
    size: u32,
) -> u32 {
    let file = e.get(this, HavokFileStreambufWriter::pFile);
    e.call(FILE_WRITE, &args![file, buffer, size]).u32()
}

// Translated from 00626340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokFileStreambufReader::seek` (engine map; the writer's vtable uses
/// the same body): with a file, calls its seek (slot +0x14) with the two
/// words and returns 0; without one returns 1.
pub fn havok_file_streambuf_reader_seek(
    e: &mut Engine,
    this: Ptr<HavokFileStreambufReader>,
    offset: u32,
    origin: u32,
) -> u32 {
    let file = e.get(this, HavokFileStreambufReader::pFile);
    if file == 0 {
        return 1;
    }
    e.vcall(file, 0x14, &args![offset, origin]);
    0
}

// Translated from 00626390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokFileStreambufReader::tell` (engine map; the writer's vtable uses the
/// same body): the file's position (`004720a0`), or 0 without a file.
pub fn havok_file_streambuf_reader_tell(
    e: &mut Engine,
    this: Ptr<HavokFileStreambufReader>,
) -> u32 {
    let file = e.get(this, HavokFileStreambufReader::pFile);
    if file == 0 {
        0
    } else {
        e.call(FILE_TELL, &args![file]).u32()
    }
}

// Translated from 006263c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the `HavokError` singleton (`01268360`): releases the old one
/// (`00c903b0`) when set, stores `error`.
pub fn fn_006263c0(e: &mut Engine, error: u32) {
    let old = e.global::<u32>(ERROR_SINGLETON);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.set_global(ERROR_SINGLETON, error);
}

// Translated from 006263f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the `HavokStreambufFactory` singleton (`01268364`): releases the
/// old one (`00c903b0`) when set, stores `factory`.
pub fn fn_006263f0(e: &mut Engine, factory: u32) {
    let old = e.global::<u32>(FACTORY_SINGLETON);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.set_global(FACTORY_SINGLETON, factory);
}

// Translated from 00626420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArray<hkFileLine>::pushBack` (by its use): appends the element at
/// `value` using the array's container allocator (`004a4840`, then the
/// append helper `006266f0`).
pub fn fn_00626420(e: &mut Engine, this: Ptr, value: u32) {
    let allocator = e.with_stack(4, |e, temp| {
        e.call(CONTAINER_ALLOCATOR, &args![temp, this]).u32()
    });
    e.call(ARRAY_APPEND, &args![this, allocator, value]);
}

// Translated from 00626450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `hkPointerMap` at `this`: the map constructor
/// (`00626560`) with capacity 0. Returns `this`.
pub fn fn_00626450(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MAP_CONSTRUCT, &args![this, 0u32]);
    this
}

// Translated from 00626470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPointerMap::insert` (by its use): the map's insert (`006265c0`) with the
/// key and value.
pub fn fn_00626470(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    e.call(MAP_INSERT, &args![this, key, value]);
}

// Translated from 00626490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPointerMap::getWithDefault` (by its use): the value stored under `key`,
/// or `default` (`00cb1930`).
pub fn fn_00626490(e: &mut Engine, this: Ptr, key: u32, default: u32) -> u32 {
    e.call(MAP_GET_WITH_DEFAULT, &args![this, key, default])
        .u32()
}

// Translated from 006264d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPointerMap::clear` (by its use): the map's clear (`00cb1a60`).
pub fn fn_006264d0(e: &mut Engine, this: Ptr) {
    e.call(MAP_CLEAR, &args![this]);
}

// Translated from 006264f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `hkPointerMap` at `this`: clears and deallocates it
/// with the container allocator (`004a4840`, `00cb1a90`), then the lock call
/// that the PC build compiled to a bare `RET`.
// Translation incomplete: 006264f0 - the FS:[0] exception-unwinding frame is not modeled.
pub fn fn_006264f0(e: &mut Engine, this: Ptr) {
    let allocator = e.with_stack(4, |e, temp| {
        e.call(CONTAINER_ALLOCATOR, &args![temp, this]).u32()
    });
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![this, allocator]);
    e.call(EMPTY_LOCK, &args![this]);
}

// Translated from 00626680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArray<hkFileLine>::removeAt` (by its use): runs the element
/// destructor helper on element `index`, shrinks the array by one and, unless
/// the removed element was the last, copies the last element into its place.
pub fn fn_00626680(e: &mut Engine, this: Ptr, index: u32) {
    let data = e.mem.u32(this.addr());
    e.call(
        ARRAY_ELEMENT_DESTRUCT,
        &args![data.wrapping_add(index.wrapping_mul(8)), 1u32, 0u32],
    );
    let size = e.mem.u32(this.addr() + 4).wrapping_sub(1);
    e.mem.set_u32(this.addr() + 4, size);
    if size != index {
        let data = e.mem.u32(this.addr());
        e.call(
            ARRAY_ELEMENT_COPY,
            &args![
                data.wrapping_add(index.wrapping_mul(8)),
                data.wrapping_add(size.wrapping_mul(8))
            ],
        );
    }
}

// Translated from 00626760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryRouter::easyFree` (engine map): frees `pointer` through the
/// router's allocator (`00c856c0`).
pub fn fn_00626760(e: &mut Engine, pointer: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.call(ROUTER_ALLOCATOR, &args![router]).u32();
    e.call(ALLOCATOR_FREE, &args![allocator, pointer]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_64b0, fn_006264b0(Ptr, u32) -> u32),
        entry!(0x00624bf0, init_havok_system()),
        entry!(0x00624d70, fn_00624d70(Ptr<HavokError>, Ptr)),
        entry!(0x00624de0, fn_00624de0(Ptr) -> Ptr),
        entry!(0x00624e00, fn_00624e00(Ptr) -> Ptr),
        entry!(0x00624e70, fn_00624e70(Ptr, u32, u32) -> u32),
        entry!(0x00624e90, havok_streambuf_factory_open_reader(Ptr, u32) -> Ptr),
        entry!(0x00624f10, havok_streambuf_factory_open_writer(Ptr, u32) -> Ptr),
        entry!(
            0x00624f90,
            havok_error_havok_error(Ptr<HavokError>) -> Ptr<HavokError>
        ),
        entry!(
            0x006255d0,
            havok_error_set_enabled(Ptr<HavokError>, u32, u32)
        ),
        entry!(
            0x00625610,
            havok_error_is_enabled(Ptr<HavokError>, Ptr, u32) -> Ptr
        ),
        entry!(0x00625650, havok_error_enable_all(Ptr<HavokError>)),
        entry!(
            0x00625670,
            havok_error_message(Ptr<HavokError>, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00625790,
            havok_error_assert_message(Ptr<HavokError>, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x006259b0,
            havok_error_error_message(Ptr<HavokError>, u32, u32, u32, u32, u32)
        ),
        entry!(0x006259f0, fn_006259f0() -> u32),
        entry!(
            0x00625a00,
            havok_error_warning_message(Ptr<HavokError>, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00625a40,
            havok_error_program_flow_message(Ptr<HavokError>, u32, u32, u32, u32, u32)
        ),
        entry!(0x00625a80, havok_error_show_string(Ptr<HavokError>, u32)),
        entry!(
            0x00625aa0,
            havok_error_hk_get_file_base_name(Ptr<HavokError>, u32) -> u32
        ),
        entry!(
            0x00625b10,
            havok_error_scalar_deleting_destructor(Ptr<HavokError>, u32) -> Ptr<HavokError>
        ),
        entry!(0x00625b40, fn_00625b40(Ptr<HavokError>)),
        entry!(0x00625bd0, fn_00625bd0(Ptr)),
        entry!(0x00625bf0, fn_00625bf0(Ptr, Ptr)),
        entry!(
            0x00625c50,
            havok_error_is_report_enabled(Ptr<HavokError>, Ptr, u32) -> bool
        ),
        entry!(
            0x00625d60,
            havok_error_enable_report(Ptr<HavokError>, Ptr, u32)
        ),
        entry!(
            0x00625e70,
            havok_error_disable_report(Ptr<HavokError>, Ptr, u32)
        ),
        entry!(0x00625f10, fn_00625f10(Ptr) -> Ptr),
        entry!(0x00625f80, fn_00625f80(Ptr, u32) -> Ptr),
        entry!(0x00625fb0, fn_00625fb0()),
        entry!(0x00625fc0, fn_00625fc0(Ptr) -> Ptr),
        entry!(0x00625fe0, fn_00625fe0(Ptr) -> Ptr),
        entry!(0x00626000, fn_00626000(Ptr, u32) -> Ptr),
        entry!(
            0x00626030,
            havok_file_streambuf_reader_scalar_deleting_destructor(
                Ptr<HavokFileStreambufReader>,
                u32,
            )
                -> Ptr<HavokFileStreambufReader>
        ),
        entry!(0x00626060, fn_00626060(Ptr<HavokFileStreambufReader>)),
        entry!(
            0x00626100,
            havok_file_streambuf_reader_read(Ptr<HavokFileStreambufReader>, u32, u32) -> u32
        ),
        entry!(
            0x00626120,
            havok_file_streambuf_reader_sys_get_file_size(Ptr<HavokFileStreambufReader>) -> u32
        ),
        entry!(
            0x00626160,
            havok_file_streambuf_reader_is_ok(Ptr<HavokFileStreambufReader>, Ptr) -> Ptr
        ),
        entry!(0x006261a0, havok_file_streambuf_writer_havok_file_streambuf_writer(Ptr, u32) -> Ptr),
        entry!(
            0x00626250,
            havok_file_streambuf_writer_scalar_deleting_destructor(
                Ptr<HavokFileStreambufWriter>,
                u32,
            )
                -> Ptr<HavokFileStreambufWriter>
        ),
        entry!(0x00626280, fn_00626280(Ptr<HavokFileStreambufWriter>)),
        entry!(
            0x00626320,
            havok_file_streambuf_writer_write(Ptr<HavokFileStreambufWriter>, u32, u32) -> u32
        ),
        entry!(
            0x00626340,
            havok_file_streambuf_reader_seek(Ptr<HavokFileStreambufReader>, u32, u32) -> u32
        ),
        entry!(
            0x00626390,
            havok_file_streambuf_reader_tell(Ptr<HavokFileStreambufReader>) -> u32
        ),
        entry!(0x006263c0, fn_006263c0(u32)),
        entry!(0x006263f0, fn_006263f0(u32)),
        entry!(0x00626420, fn_00626420(Ptr, u32)),
        entry!(0x00626450, fn_00626450(Ptr) -> Ptr),
        entry!(0x00626470, fn_00626470(Ptr, u32, u32)),
        entry!(0x00626490, fn_00626490(Ptr, u32, u32) -> u32),
        entry!(0x006264d0, fn_006264d0(Ptr)),
        entry!(0x006264f0, fn_006264f0(Ptr)),
        entry!(0x00626680, fn_00626680(Ptr, u32)),
        entry!(0x00626760, fn_00626760(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_006264b0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(COUNT_TARGET_00CB1FB0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_64b0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == COUNT_TARGET_00CB1FB0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}

#[cfg(test)]
mod tests_b {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// (file name, line) pairs a double saw appended to the disabled list.
    type Appended = Rc<RefCell<Vec<(String, i32)>>>;
    /// What the assert handler saw: shown file, line, text.
    type Seen = Rc<RefCell<Vec<(String, u32, String)>>>;

    /// Function addresses the tests give the fake vtables' slots.
    const SLOT_IS_ENABLED: u32 = 0x7000_0014;
    const SLOT_SET_ENABLED: u32 = 0x7000_0010;
    const SLOT_ASSERT: u32 = 0x7000_002c;
    const SLOT_ERROR: u32 = 0x7000_0030;
    const SLOT_WARNING: u32 = 0x7000_0034;
    const SLOT_REPORT: u32 = 0x7000_0038;
    const SLOT_HANDLER: u32 = 0x7000_000c;
    const SLOT_DELETE: u32 = 0x7000_0000;
    const SLOT_SEEK: u32 = 0x7000_0114;
    const SLOT_SIZE: u32 = 0x7000_0128;
    /// Where the fake vtables live.
    const TABLE_A: u32 = 0x0120_8000;
    const TABLE_B: u32 = 0x0120_9000;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An engine with the exe's data range mapped, the call log on, and
    /// doubles for the `hkBool` constructor and the reference-counted base.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e.register(HK_BOOL_CONSTRUCT, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            a[0].into_ret()
        });
        e.register(REFERENCED_OBJECT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(REFERENCED_OBJECT_DESTRUCT, nothing);
        e.register(REFERENCED_OBJECT_FREE, nothing);
        e.register(SINGLETON_CONSTRUCT, nothing);
        e.register(HK_BOOL_GET, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CONTAINER_ALLOCATOR, |_, _| 0x1234u32.into_ret());
        e
    }

    /// The argument words of every logged call to `address`.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The logged call addresses among `of`, in order.
    fn order(e: &Engine, of: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| of.contains(a))
            .collect()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for a in addresses {
            e.register(*a, nothing);
        }
    }

    /// A NUL-terminated copy of `text` on the heap.
    fn put(e: &mut Engine, text: &str) -> u32 {
        let a = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(a, text.as_bytes());
        a
    }

    fn text(e: &Engine, address: u32) -> String {
        String::from_utf8(e.mem.cstr(address)).unwrap()
    }

    /// Writes the strings of the exe the code compares against.
    fn put_exe_strings(e: &mut Engine) {
        e.mem.set_cstr(TES4_UPPER, b"TES4");
        e.mem.set_cstr(TES4_LOWER, b"tes4");
        e.mem
            .set_cstr(HAVOK_INCLUDE_PATH, b"TES4\\Havok\\SDK\\Include\\");
        e.mem.set_cstr(TEXT_SEPARATOR, b" : ");
        e.mem.set_cstr(PATH_SEPARATORS, b"/\\");
        e.mem
            .set_cstr(VISUAL_DEBUGGER_FILE, b"hkVisualDebugger.cpp");
    }

    /// Doubles for the string helpers: `_splitpath_s` (name and extension of
    /// the last path component), append, case-insensitive compare, `strrchr`,
    /// the counted copy and `strdup`.
    fn string_doubles(e: &mut Engine) {
        e.register(STRSTR, |e, a| {
            let haystack = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            match haystack
                .windows(needle.len())
                .position(|window| window == needle.as_slice())
            {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(STR_COPY, |e, a| {
            let source = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &source);
            0u32.into_ret()
        });
        e.register(SPLIT_PATH, |e, a| {
            let path = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            let base = path.rsplit(['/', char::from(0x5cu8)]).next().unwrap();
            let (name, extension) = match base.rfind('.') {
                Some(i) => (&base[..i], &base[i..]),
                None => (base, ""),
            };
            e.mem.set_cstr(a[5], name.as_bytes());
            e.mem.set_cstr(a[7], extension.as_bytes());
            0u32.into_ret()
        });
        e.register(STR_CAT, |e, a| {
            let mut joined = e.mem.cstr(a[0]);
            joined.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &joined);
            0u32.into_ret()
        });
        e.register(STR_CASECMP, |e, a| {
            let left = String::from_utf8(e.mem.cstr(a[0])).unwrap().to_lowercase();
            let right = String::from_utf8(e.mem.cstr(a[1])).unwrap().to_lowercase();
            (left != right).into_ret()
        });
        e.register(STR_RCHR, |e, a| {
            if a[0] == 0 {
                return 0u32.into_ret();
            }
            let bytes = e.mem.cstr(a[0]);
            match bytes.iter().rposition(|b| *b == a[1] as u8) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(COPY_BYTES, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            0u32.into_ret()
        });
        e.register(STR_DUP, |e, a| {
            let copy = e.mem.cstr(a[0]);
            let block = e.mem.alloc(copy.len() as u32 + 1);
            e.mem.set_cstr(block, &copy);
            block.into_ret()
        });
    }

    /// Doubles for the array and map helpers `HavokError` calls; appends are
    /// recorded in the returned list.
    fn container_doubles(e: &mut Engine) -> Appended {
        e.register(ARRAY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MAP_CONSTRUCT, nothing);
        e.register(MAP_INSERT, nothing);
        e.register(MAP_REMOVE, nothing);
        e.register(CONTAINER_ALLOCATOR, |_, _| 0x1234u32.into_ret());
        e.register(ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0]) + a[1] * 8).into_ret()
        });
        let appended: Appended = Rc::default();
        let log = appended.clone();
        e.register_double(ARRAY_APPEND, move |e, a| {
            let file = e.mem.u32(a[2]);
            let line = e.mem.i32(a[2] + 4);
            log.borrow_mut()
                .push((String::from_utf8(e.mem.cstr(file)).unwrap(), line));
            Ret::default()
        });
        appended
    }

    /// Doubles for the memory router and its allocator: the allocator is
    /// `router + 0x10`, i.e. `0x4010`; frees are logged.
    fn router_doubles(e: &mut Engine) {
        e.register(MEMORY_ROUTER, |_, _| 0x4000u32.into_ret());
        e.register(ROUTER_ALLOCATOR, |_, a| (a[0] + 0x10).into_ret());
        e.register(ALLOCATOR_FREE, nothing);
    }

    /// A `HavokError`-sized zeroed object.
    fn error_object(e: &mut Engine) -> Ptr<HavokError> {
        e.new_object::<HavokError>()
    }

    /// Points `object` at a vtable at `table` with the given (byte offset,
    /// function address) slots.
    fn set_vtable(e: &mut Engine, object: u32, table: u32, slots: &[(u32, u32)]) {
        let last = slots.iter().map(|s| s.0).max().unwrap() / 4 + 1;
        let mut words = vec![0u32; last as usize];
        for (offset, function) in slots {
            words[(*offset / 4) as usize] = *function;
        }
        e.put_vtable(table, &words);
        e.mem.set_u32(object, table);
    }

    /// An error object whose vtable has logging doubles at the `HavokError`
    /// slots; `isEnabled` enables the ids below 100.
    fn error_with_slots(e: &mut Engine) -> Ptr<HavokError> {
        let this = error_object(e);
        set_vtable(
            e,
            this.addr(),
            TABLE_A,
            &[
                (0x10, SLOT_SET_ENABLED),
                (0x14, SLOT_IS_ENABLED),
                (0x2c, SLOT_ASSERT),
                (0x30, SLOT_ERROR),
                (0x34, SLOT_WARNING),
                (0x38, SLOT_REPORT),
            ],
        );
        stub(
            e,
            &[
                SLOT_SET_ENABLED,
                SLOT_ASSERT,
                SLOT_ERROR,
                SLOT_WARNING,
                SLOT_REPORT,
            ],
        );
        e.register(SLOT_IS_ENABLED, |e, a| {
            e.mem.set_u8(a[1], (a[2] < 100) as u8);
            a[1].into_ret()
        });
        this
    }

    /// An error object whose disabled list holds `entries` (name, line).
    fn error_with_reports(e: &mut Engine, entries: &[(&str, i32)]) -> Ptr<HavokError> {
        let this = error_object(e);
        let data = e.mem.alloc(8 * entries.len() as u32 + 8);
        for (i, (name, line)) in entries.iter().enumerate() {
            let name = put(e, name);
            e.mem.set_u32(data + 8 * i as u32, name);
            e.mem.set_i32(data + 8 * i as u32 + 4, *line);
        }
        e.mem.set_u32(this.addr() + 8, data);
        e.mem.set_u32(this.addr() + 0xc, entries.len() as u32);
        this
    }

    /// A stream object (reader or writer layout) owning a fake file whose
    /// deleting destructor (slot 0), seek (slot 0x14) and size (slot 0x28)
    /// are doubles.
    fn stream_with_file(e: &mut Engine, owned: u8) -> (Ptr, u32) {
        let stream = Ptr::new(e.mem.alloc(0x10));
        let file = e.mem.alloc(8);
        set_vtable(
            e,
            file,
            TABLE_B,
            &[(0, SLOT_DELETE), (0x14, SLOT_SEEK), (0x28, SLOT_SIZE)],
        );
        e.register(SLOT_DELETE, nothing);
        e.register(SLOT_SEEK, nothing);
        e.register(SLOT_SIZE, |_, _| 0x4321u32.into_ret());
        e.mem.set_u32(stream.addr() + 8, file);
        e.mem.set_u8(stream.addr() + 0xc, owned);
        (stream, file)
    }

    /// Installs the assert handler object, answering what `answer` holds.
    fn assert_handler(e: &mut Engine, answer: Rc<Cell<u32>>) -> Seen {
        let handler = e.mem.alloc(8);
        set_vtable(e, handler, TABLE_B, &[(0xc, SLOT_HANDLER)]);
        e.register_double(ASSERT_HANDLER, move |_, _| handler.into_ret());
        let seen: Seen = Rc::default();
        let log = seen.clone();
        e.register_double(SLOT_HANDLER, move |e, a| {
            log.borrow_mut().push((
                String::from_utf8(e.mem.cstr(a[1])).unwrap(),
                a[2],
                String::from_utf8(e.mem.cstr(a[3])).unwrap(),
            ));
            answer.get().into_ret()
        });
        seen
    }

    // 00624bf0

    #[test]
    fn init_havok_system_builds_the_singletons() {
        let mut e = engine();
        put_exe_strings(&mut e);
        string_doubles(&mut e);
        container_doubles(&mut e);
        stub(
            &mut e,
            &[
                GUARD_CONSTRUCT,
                GUARD_DESTRUCT,
                BHK_WORLD_INIT_SYSTEM,
                HK_MONITOR_STREAM_INIT,
                BHK_WORLD_SET_UPDATE_MODE,
            ],
        );
        e.register(HAVOK_OBJECT_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(GET_CURRENT_DIRECTORY, |e, a| {
            e.mem.set_cstr(a[1], b"C:\\Games\\TES4\\Run");
            1u32.into_ret()
        });
        e.register(UPDATE_MODE_GETTER, |_, a| (a[0] + 4).into_ret());
        e.mem.set_u32(UPDATE_MODE_HOLDER + 4, 3);
        init_havok_system(&mut e);
        let steps = [
            GUARD_CONSTRUCT,
            BHK_WORLD_INIT_SYSTEM,
            HK_MONITOR_STREAM_INIT,
            HAVOK_OBJECT_ALLOCATE,
            GET_CURRENT_DIRECTORY,
            BHK_WORLD_SET_UPDATE_MODE,
            GUARD_DESTRUCT,
        ];
        assert_eq!(
            order(&e, &steps),
            [
                GUARD_CONSTRUCT,
                BHK_WORLD_INIT_SYSTEM,
                HK_MONITOR_STREAM_INIT,
                HAVOK_OBJECT_ALLOCATE,
                HAVOK_OBJECT_ALLOCATE,
                GET_CURRENT_DIRECTORY,
                BHK_WORLD_SET_UPDATE_MODE,
                GUARD_DESTRUCT,
            ]
        );
        assert_eq!(calls(&e, HAVOK_OBJECT_ALLOCATE), [vec![0x124], vec![8]]);
        let guard = &calls(&e, GUARD_CONSTRUCT)[0];
        assert_eq!(guard[1..], [0x10, 1, SOURCE_FILE_NAME, 0xac]);
        assert_eq!(calls(&e, GET_CURRENT_DIRECTORY)[0][0], 0x104);
        assert_eq!(calls(&e, BHK_WORLD_SET_UPDATE_MODE), [vec![3]]);
        let error = e.global::<u32>(ERROR_SINGLETON);
        let factory = e.global::<u32>(FACTORY_SINGLETON);
        assert_eq!(e.mem.u32(error), VTABLE_ERROR);
        assert_eq!(e.mem.u32(factory), VTABLE_FACTORY);
        assert_eq!(text(&e, error + 0x20), "C:\\Games\\");
    }

    // 00624d70

    #[test]
    fn fn_00624d70_keeps_the_part_before_tes4() {
        let mut e = engine();
        put_exe_strings(&mut e);
        string_doubles(&mut e);
        let this = error_object(&mut e);
        let upper = put(&mut e, "D:\\Games\\TES4\\Bin");
        fn_00624d70(&mut e, this, Ptr::new(upper));
        assert_eq!(text(&e, this.addr() + 0x20), "D:\\Games\\");
        let lower = put(&mut e, "E:\\x\\tes4\\y");
        fn_00624d70(&mut e, this, Ptr::new(lower));
        assert_eq!(text(&e, this.addr() + 0x20), "E:\\x\\");
        let plain = put(&mut e, "F:\\other");
        fn_00624d70(&mut e, this, Ptr::new(plain));
        assert_eq!(text(&e, this.addr() + 0x20), "E:\\x\\");
    }

    // 00624de0

    #[test]
    fn fn_00624de0_installs_the_factory_vtable() {
        let mut e = engine();
        let this = Ptr::new(e.mem.alloc(8));
        assert_eq!(fn_00624de0(&mut e, this), this);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_FACTORY);
        assert_eq!(calls(&e, REFERENCED_OBJECT_CONSTRUCT), [vec![this.addr()]]);
    }

    // 00624e00

    #[test]
    fn fn_00624e00_installs_the_file_system_vtable() {
        let mut e = engine();
        let this = Ptr::new(e.mem.alloc(16));
        assert_eq!(fn_00624e00(&mut e, this), this);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_FILE_SYSTEM);
        assert_eq!(calls(&e, SINGLETON_CONSTRUCT), [vec![this.addr() + 8]]);
    }

    // 00624e70

    #[test]
    fn fn_00624e70_returns_one() {
        let mut e = engine();
        assert_eq!(e.call(0x0062_4e70, &args![0x1000u32, 7u32, 9u32]).u32(), 1);
    }

    // 00624e90

    #[test]
    fn open_reader_constructs_a_reader() {
        let mut e = engine();
        e.register(HAVOK_OBJECT_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let reader = havok_streambuf_factory_open_reader(&mut e, Ptr::new(0x1000), 0x55);
        assert_eq!(calls(&e, HAVOK_OBJECT_ALLOCATE), [vec![0x10]]);
        assert_eq!(e.mem.u32(reader.addr()), VTABLE_FILE_READER);
        assert_eq!(e.mem.u8(reader.addr() + 0xc), 1);
        e.register(HAVOK_OBJECT_ALLOCATE, |_, _| 0u32.into_ret());
        let failed = havok_streambuf_factory_open_reader(&mut e, Ptr::new(0x1000), 0x55);
        assert!(failed.is_null());
    }

    // 00624f10

    #[test]
    fn open_writer_constructs_a_writer() {
        let mut e = engine();
        e.register(HAVOK_OBJECT_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BS_FILE_CONSTRUCT, |_, a| a[0].into_ret());
        let writer = havok_streambuf_factory_open_writer(&mut e, Ptr::new(0x1000), 0x77);
        assert_eq!(e.mem.u32(writer.addr()), VTABLE_FILE_WRITER);
        let file = e.mem.u32(writer.addr() + 8);
        assert_eq!(
            calls(&e, BS_FILE_CONSTRUCT),
            [vec![file, 0x77, 1, 0x4000, 0]]
        );
        e.register(HAVOK_OBJECT_ALLOCATE, |_, _| 0u32.into_ret());
        assert!(havok_streambuf_factory_open_writer(&mut e, Ptr::new(0x1000), 0x77).is_null());
    }

    // 00624f90

    #[test]
    fn havok_error_constructor_disables_the_start_up_reports() {
        let mut e = engine();
        put_exe_strings(&mut e);
        string_doubles(&mut e);
        let appended = container_doubles(&mut e);
        let this = error_object(&mut e);
        e.mem.set_u8(this.addr() + 0x20, 0x41);
        assert_eq!(havok_error_havok_error(&mut e, this), this);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ERROR);
        assert_eq!(e.mem.u8(this.addr() + 0x20), 0);
        let map = this.addr() + 0x14;
        let inserts = calls(&e, MAP_INSERT);
        assert_eq!(inserts.len(), 41);
        for (insert, id) in inserts.iter().zip(INITIALLY_DISABLED_IDS) {
            assert_eq!(insert, &vec![map, id, 1]);
        }
        assert_eq!(calls(&e, MAP_CONSTRUCT), [vec![map, 0]]);
        assert_eq!(calls(&e, ARRAY_CONSTRUCT), [vec![this.addr() + 8]]);
        assert!(calls(&e, MAP_REMOVE).is_empty());
        let appended = appended.borrow();
        let lines: Vec<i32> = appended.iter().map(|(_, line)| *line).collect();
        assert_eq!(
            lines,
            [0x26, 0x56, 0x5d, 0x6a, 0x6e, 0x73, 0x7e, 0x85, 0xbf, 0xf6, 0x115]
        );
        assert!(appended
            .iter()
            .all(|(file, _)| file == "hkVisualDebugger.cpp"));
    }

    // 006255d0

    #[test]
    fn set_enabled_removes_or_inserts() {
        let mut e = engine();
        e.register(MAP_INSERT, nothing);
        e.register(MAP_REMOVE, nothing);
        let this = error_object(&mut e);
        let map = this.addr() + 0x14;
        havok_error_set_enabled(&mut e, this, 0x1234, 1);
        assert_eq!(calls(&e, MAP_REMOVE), [vec![map, 0x1234]]);
        assert!(calls(&e, MAP_INSERT).is_empty());
        // Only the low byte of the hkBool word counts.
        havok_error_set_enabled(&mut e, this, 0x4321, 0x100);
        assert_eq!(calls(&e, MAP_INSERT), [vec![map, 0x4321, 1]]);
        assert_eq!(calls(&e, MAP_REMOVE).len(), 1);
    }

    // 00625610

    #[test]
    fn is_enabled_tests_for_a_missing_entry() {
        let mut e = engine();
        e.register(MAP_GET_WITH_DEFAULT, |_, a| (a[1] & 1).into_ret());
        let this = error_object(&mut e);
        let result = Ptr::new(e.mem.alloc(4));
        assert_eq!(havok_error_is_enabled(&mut e, this, result, 4), result);
        assert_eq!(e.mem.u8(result.addr()), 1);
        havok_error_is_enabled(&mut e, this, result, 5);
        assert_eq!(e.mem.u8(result.addr()), 0);
        assert_eq!(
            calls(&e, MAP_GET_WITH_DEFAULT)[0],
            [this.addr() + 0x14, 4, 0]
        );
    }

    // 00625650

    #[test]
    fn enable_all_clears_the_map() {
        let mut e = engine();
        e.register(MAP_CLEAR, nothing);
        let this = error_object(&mut e);
        havok_error_enable_all(&mut e, this);
        assert_eq!(calls(&e, MAP_CLEAR), [vec![this.addr() + 0x14]]);
    }

    // 00625670

    #[test]
    fn message_dispatches_by_severity() {
        let mut e = engine();
        let this = error_with_slots(&mut e);
        let t = this.addr();
        // An enabled id reaches the handler of its severity.
        for (severity, slot, label) in [
            (0, SLOT_REPORT, LABEL_REPORT),
            (1, SLOT_WARNING, LABEL_WARNING),
            (2, SLOT_ASSERT, LABEL_ASSERT),
            (3, SLOT_ASSERT, LABEL_ASSERT),
            (4, SLOT_ERROR, LABEL_ERROR),
            (99, SLOT_ERROR, LABEL_ERROR),
        ] {
            assert!(havok_error_message(
                &mut e, this, severity, 7, 0x10, 0x20, 30
            ));
            let last = calls(&e, slot).pop().unwrap();
            assert_eq!(last, [t, label, 7, 0x10, 0x20, 30]);
        }
        // A disabled id reports nothing: only the query and the hkBool read run.
        let before = e.call_log.as_ref().unwrap().len();
        assert!(!havok_error_message(&mut e, this, 0, 700, 0x10, 0x20, 30));
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(log.len(), before + 2);
        assert_eq!(log[before].0, SLOT_IS_ENABLED);
        assert_eq!(log[before + 1].0, HK_BOOL_GET);
        // The query goes through slot +0x14 with a result slot and the id.
        let query = calls(&e, SLOT_IS_ENABLED).pop().unwrap();
        assert_eq!((query[0], query[2]), (t, 700));
    }

    #[test]
    fn message_without_an_id_asks_the_disabled_list() {
        let mut e = engine();
        string_doubles(&mut e);
        container_doubles(&mut e);
        let this = error_with_slots(&mut e);
        // One disabled entry: "foo.cpp" line 10.
        let entry = put(&mut e, "foo.cpp");
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, entry);
        e.mem.set_u32(data + 4, 10);
        e.mem.set_u32(this.addr() + 8, data);
        e.mem.set_u32(this.addr() + 0xc, 1);
        let file = put(&mut e, "C:\\src\\Foo.cpp");
        let other = put(&mut e, "C:\\src\\Bar.cpp");
        assert!(!havok_error_message(&mut e, this, 0, u32::MAX, 1, file, 10));
        assert!(calls(&e, SLOT_REPORT).is_empty());
        assert!(havok_error_message(&mut e, this, 0, u32::MAX, 1, file, 11));
        assert!(havok_error_message(&mut e, this, 0, u32::MAX, 1, other, 10));
        assert_eq!(calls(&e, SLOT_REPORT).len(), 2);
        assert!(calls(&e, SLOT_IS_ENABLED).is_empty());
    }

    // 00625790

    #[test]
    fn assert_message_shows_the_file_under_the_root_directory() {
        let mut e = engine();
        put_exe_strings(&mut e);
        let answer = Rc::new(Cell::new(0));
        let seen = assert_handler(&mut e, answer);
        let this = error_with_slots(&mut e);
        let label = put(&mut e, "Assert");
        let message = put(&mut e, "x != y");
        let file = put(&mut e, "C:\\src\\foo.cpp");
        // No root directory: the file and the label, separator and text.
        havok_error_assert_message(&mut e, this, label, 9, message, file, 12);
        assert_eq!(
            seen.borrow()[0],
            (
                "C:\\src\\foo.cpp".to_string(),
                12,
                "Assert : x != y".to_string()
            )
        );
        // No label and no text: an empty buffer.
        havok_error_assert_message(&mut e, this, 0, 9, 0, file, 13);
        assert_eq!(seen.borrow()[1].2, "");
        // With a root directory: from TES4 on, and the `..` form.
        e.mem.set_cstr(this.addr() + 0x20, b"C:\\Games\\");
        let under = put(&mut e, "Z:\\build\\TES4\\Havok\\a.cpp");
        havok_error_assert_message(&mut e, this, label, 9, 0, under, 14);
        assert_eq!(seen.borrow()[2].0, "C:\\Games\\TES4\\Havok\\a.cpp");
        let lower = put(&mut e, "Z:\\build\\tes4\\b.cpp");
        havok_error_assert_message(&mut e, this, label, 9, 0, lower, 15);
        assert_eq!(seen.borrow()[3].0, "C:\\Games\\tes4\\b.cpp");
        let dots = put(&mut e, "..\\hk\\c.h");
        havok_error_assert_message(&mut e, this, label, 9, 0, dots, 16);
        assert_eq!(
            seen.borrow()[4].0,
            "C:\\Games\\TES4\\Havok\\SDK\\Include\\hk\\c.h"
        );
        let plain = put(&mut e, "Y:\\nowhere.cpp");
        havok_error_assert_message(&mut e, this, label, 9, 0, plain, 17);
        assert_eq!(seen.borrow()[5].0, "Y:\\nowhere.cpp");
        assert!(calls(&e, SLOT_SET_ENABLED).is_empty());
    }

    #[test]
    fn assert_message_disables_the_id_when_the_handler_answers_five() {
        let mut e = engine();
        put_exe_strings(&mut e);
        let answer = Rc::new(Cell::new(5));
        assert_handler(&mut e, answer);
        let this = error_with_slots(&mut e);
        let file = put(&mut e, "C:\\src\\foo.cpp");
        havok_error_assert_message(&mut e, this, 0, 0x99, 0, file, 12);
        let set = calls(&e, SLOT_SET_ENABLED);
        assert_eq!(set.len(), 1);
        assert_eq!(
            (set[0][0], set[0][1], set[0][2] & 0xff),
            (this.addr(), 0x99, 0)
        );
    }

    // 006259b0, 00625a00, 00625a40, 00625a80, 006259f0

    #[test]
    fn the_report_functions_print_at_their_level() {
        let mut e = engine();
        e.register(LOG_PRINT, nothing);
        e.set_global(MESSAGE_HANDLER, 0x5150u32);
        let this = error_object(&mut e);
        havok_error_error_message(&mut e, this, 1, 2, 3, 4, 5);
        havok_error_warning_message(&mut e, this, 1, 2, 3, 4, 5);
        havok_error_program_flow_message(&mut e, this, 1, 2, 3, 4, 5);
        havok_error_show_string(&mut e, this, 8);
        assert_eq!(
            calls(&e, LOG_PRINT),
            [
                vec![0, LOG_FORMAT, 4, 5, 1, 3],
                vec![1, LOG_FORMAT, 4, 5, 1, 3],
                vec![2, LOG_FORMAT, 4, 5, 1, 3],
                vec![4, 8],
            ]
        );
        assert_eq!(fn_006259f0(&mut e), 0x5150);
        assert_eq!(e.call(0x0062_59f0, &args![]).u32(), 0x5150);
    }

    // 00625aa0

    #[test]
    fn hk_get_file_base_name_skips_both_separators() {
        let mut e = engine();
        put_exe_strings(&mut e);
        string_doubles(&mut e);
        let this = error_object(&mut e);
        let path = put(&mut e, "a/b\\c.txt");
        let base = havok_error_hk_get_file_base_name(&mut e, this, path);
        assert_eq!(text(&e, base), "c.txt");
        let none = put(&mut e, "plain.txt");
        assert_eq!(havok_error_hk_get_file_base_name(&mut e, this, none), 0);
        // The second search still runs on the null left by the first.
        assert_eq!(calls(&e, STR_RCHR).len(), 4);
    }

    // 00625b10

    #[test]
    fn the_error_deleting_destructor_frees_with_bit_zero() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                CONTAINER_ALLOCATOR,
                MAP_CLEAR_AND_DEALLOCATE,
                EMPTY_LOCK,
                ARRAY_DESTRUCT,
            ],
        );
        let this = error_object(&mut e);
        assert_eq!(
            havok_error_scalar_deleting_destructor(&mut e, this, 0),
            this
        );
        assert!(calls(&e, REFERENCED_OBJECT_FREE).is_empty());
        assert_eq!(calls(&e, REFERENCED_OBJECT_DESTRUCT), [vec![this.addr()]]);
        havok_error_scalar_deleting_destructor(&mut e, this, 1);
        assert_eq!(calls(&e, REFERENCED_OBJECT_FREE), [vec![this.addr()]]);
    }

    // 00625b40

    #[test]
    fn the_error_destructor_frees_the_report_names_and_the_containers() {
        let mut e = engine();
        stub(
            &mut e,
            &[MAP_CLEAR_AND_DEALLOCATE, EMPTY_LOCK, ARRAY_DESTRUCT],
        );
        e.register(CONTAINER_ALLOCATOR, |_, _| 0x1234u32.into_ret());
        e.register(ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0]) + a[1] * 8).into_ret()
        });
        router_doubles(&mut e);
        let this = error_object(&mut e);
        let data = e.mem.alloc(16);
        e.mem.set_u32(data, 0x1111);
        e.mem.set_u32(data + 8, 0x2222);
        e.mem.set_u32(this.addr() + 8, data);
        e.mem.set_u32(this.addr() + 0xc, 2);
        fn_00625b40(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ERROR);
        assert_eq!(
            calls(&e, ALLOCATOR_FREE),
            [vec![0x4010, 0x1111], vec![0x4010, 0x2222]]
        );
        let map = this.addr() + 0x14;
        assert_eq!(calls(&e, MAP_CLEAR_AND_DEALLOCATE), [vec![map, 0x1234]]);
        assert_eq!(calls(&e, EMPTY_LOCK), [vec![map]]);
        assert_eq!(calls(&e, ARRAY_DESTRUCT), [vec![this.addr() + 8]]);
        assert_eq!(calls(&e, REFERENCED_OBJECT_DESTRUCT), [vec![this.addr()]]);
        let steps = [
            ALLOCATOR_FREE,
            MAP_CLEAR_AND_DEALLOCATE,
            ARRAY_DESTRUCT,
            REFERENCED_OBJECT_DESTRUCT,
        ];
        assert_eq!(
            order(&e, &steps),
            [
                ALLOCATOR_FREE,
                ALLOCATOR_FREE,
                MAP_CLEAR_AND_DEALLOCATE,
                ARRAY_DESTRUCT,
                REFERENCED_OBJECT_DESTRUCT
            ]
        );
    }

    // 00625bd0

    #[test]
    fn fn_00625bd0_destroys_the_map() {
        let mut e = engine();
        stub(&mut e, &[MAP_CLEAR_AND_DEALLOCATE, EMPTY_LOCK]);
        e.register(CONTAINER_ALLOCATOR, |_, _| 0x77u32.into_ret());
        let map = Ptr::new(e.mem.alloc(16));
        fn_00625bd0(&mut e, map);
        assert_eq!(
            calls(&e, MAP_CLEAR_AND_DEALLOCATE),
            [vec![map.addr(), 0x77]]
        );
    }

    // 00625bf0

    #[test]
    fn fn_00625bf0_frees_every_name() {
        let mut e = engine();
        e.register(ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0]) + a[1] * 8).into_ret()
        });
        router_doubles(&mut e);
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(24);
        for i in 0..3u32 {
            e.mem.set_u32(data + 8 * i, 0x100 + i);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 3);
        fn_00625bf0(&mut e, Ptr::new(0), Ptr::new(array));
        assert_eq!(
            calls(&e, ALLOCATOR_FREE),
            [
                vec![0x4010, 0x100],
                vec![0x4010, 0x101],
                vec![0x4010, 0x102]
            ]
        );
    }

    // 00626760

    #[test]
    fn fn_00626760_frees_through_the_router_allocator() {
        let mut e = engine();
        router_doubles(&mut e);
        fn_00626760(&mut e, 0xabcd);
        assert_eq!(calls(&e, ROUTER_ALLOCATOR), [vec![0x4000]]);
        assert_eq!(calls(&e, ALLOCATOR_FREE), [vec![0x4010, 0xabcd]]);
    }

    // 00625c50

    #[test]
    fn is_report_enabled_looks_for_the_file_and_line() {
        let mut e = engine();
        string_doubles(&mut e);
        container_doubles(&mut e);
        let empty = error_with_reports(&mut e, &[]);
        let file = put(&mut e, "C:\\src\\Foo.cpp");
        assert!(havok_error_is_report_enabled(
            &mut e,
            empty,
            Ptr::new(file),
            5
        ));
        assert!(calls(&e, SPLIT_PATH).is_empty());
        let this = error_with_reports(&mut e, &[("bar.cpp", 3), ("foo.cpp", 10), ("foo.cpp", 20)]);
        assert!(!havok_error_is_report_enabled(
            &mut e,
            this,
            Ptr::new(file),
            10
        ));
        assert!(!havok_error_is_report_enabled(
            &mut e,
            this,
            Ptr::new(file),
            20
        ));
        assert!(havok_error_is_report_enabled(
            &mut e,
            this,
            Ptr::new(file),
            11
        ));
        let other = put(&mut e, "C:\\src\\Baz.cpp");
        assert!(havok_error_is_report_enabled(
            &mut e,
            this,
            Ptr::new(other),
            10
        ));
        let split = calls(&e, SPLIT_PATH).remove(0);
        assert_eq!(split[0], file);
        assert_eq!(split[1..5], [0, 0, 0, 0]);
        assert_eq!((split[6], split[8]), (0x80, 0x20));
    }

    // 00625d60

    #[test]
    fn enable_report_removes_the_entry() {
        let mut e = engine();
        string_doubles(&mut e);
        container_doubles(&mut e);
        stub(&mut e, &[ARRAY_ELEMENT_DESTRUCT, ARRAY_ELEMENT_COPY]);
        router_doubles(&mut e);
        let this = error_with_reports(&mut e, &[("bar.cpp", 3), ("foo.cpp", 10), ("foo.cpp", 20)]);
        let data = e.mem.u32(this.addr() + 8);
        let first_foo = e.mem.u32(data + 8);
        let file = put(&mut e, "C:\\src\\Foo.cpp");
        havok_error_enable_report(&mut e, this, Ptr::new(file), 10);
        assert_eq!(calls(&e, ALLOCATOR_FREE), [vec![0x4010, first_foo]]);
        assert_eq!(calls(&e, ARRAY_ELEMENT_DESTRUCT), [vec![data + 8, 1, 0]]);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 2);
        // The last element moves into the gap.
        assert_eq!(calls(&e, ARRAY_ELEMENT_COPY), [vec![data + 8, data + 16]]);
        // No match: nothing changes.
        havok_error_enable_report(&mut e, this, Ptr::new(file), 99);
        assert_eq!(calls(&e, ALLOCATOR_FREE).len(), 1);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 2);
    }

    // 00625e70

    #[test]
    fn disable_report_appends_the_base_name_and_line() {
        let mut e = engine();
        string_doubles(&mut e);
        let appended = container_doubles(&mut e);
        let this = error_object(&mut e);
        let file = put(&mut e, "C:\\deep\\Thing.cpp");
        havok_error_disable_report(&mut e, this, Ptr::new(file), 77);
        assert_eq!(*appended.borrow(), [("Thing.cpp".to_string(), 77)]);
        assert_eq!(calls(&e, ARRAY_APPEND)[0][0], this.addr() + 8);
    }

    // 00625f10, 00625fc0, 00625fe0, 00626000

    #[test]
    fn the_stream_and_error_base_constructors_install_their_vtables() {
        let mut e = engine();
        let block = Ptr::new(e.mem.alloc(0x10));
        assert_eq!(fn_00625f10(&mut e, block), block);
        assert_eq!(e.mem.u32(block.addr()), VTABLE_ERROR_BASE);
        assert_eq!(calls(&e, SINGLETON_CONSTRUCT), [vec![block.addr() + 8]]);
        assert_eq!(fn_00625fc0(&mut e, block), block);
        assert_eq!(e.mem.u32(block.addr()), VTABLE_STREAM_READER_BASE);
        assert_eq!(fn_00625fe0(&mut e, block), block);
        assert_eq!(e.mem.u32(block.addr()), VTABLE_STREAM_WRITER_BASE);
        assert_eq!(fn_00626000(&mut e, block, 0x55), block);
        assert_eq!(e.mem.u32(block.addr()), VTABLE_FILE_READER);
        assert_eq!(e.mem.u8(block.addr() + 0xc), 1);
        assert_eq!(e.mem.u32(block.addr() + 8), 0);
        assert_eq!(calls(&e, REFERENCED_OBJECT_CONSTRUCT).len(), 4);
    }

    // 00625f80

    #[test]
    fn fn_00625f80_frees_with_bit_zero() {
        let mut e = engine();
        let block = Ptr::new(e.mem.alloc(0x10));
        assert_eq!(fn_00625f80(&mut e, block, 0), block);
        assert!(calls(&e, REFERENCED_OBJECT_FREE).is_empty());
        assert_eq!(fn_00625f80(&mut e, block, 3), block);
        assert_eq!(calls(&e, REFERENCED_OBJECT_FREE), [vec![block.addr()]]);
        assert_eq!(calls(&e, REFERENCED_OBJECT_DESTRUCT).len(), 2);
    }

    // 00625fb0

    #[test]
    fn fn_00625fb0_shuts_the_world_down() {
        let mut e = engine();
        e.register(BHK_WORLD_SHUTDOWN, nothing);
        fn_00625fb0(&mut e);
        assert_eq!(calls(&e, BHK_WORLD_SHUTDOWN), [Vec::<u32>::new()]);
    }

    // 00626030, 00626060

    #[test]
    fn the_reader_destructor_deletes_an_owned_file() {
        let mut e = engine();
        let (stream, file) = stream_with_file(&mut e, 1);
        let reader = stream.cast::<HavokFileStreambufReader>();
        assert_eq!(
            havok_file_streambuf_reader_scalar_deleting_destructor(&mut e, reader, 1),
            reader
        );
        assert_eq!(e.mem.u32(stream.addr()), VTABLE_FILE_READER);
        assert_eq!(calls(&e, SLOT_DELETE), [vec![file, 1]]);
        assert_eq!(calls(&e, REFERENCED_OBJECT_DESTRUCT), [vec![stream.addr()]]);
        assert_eq!(calls(&e, REFERENCED_OBJECT_FREE), [vec![stream.addr()]]);
        // Not owned, or without a file: the file is left alone.
        let (borrowed, _) = stream_with_file(&mut e, 0);
        fn_00626060(&mut e, borrowed.cast());
        let (empty, _) = stream_with_file(&mut e, 1);
        e.mem.set_u32(empty.addr() + 8, 0);
        fn_00626060(&mut e, empty.cast());
        assert_eq!(calls(&e, SLOT_DELETE).len(), 1);
    }

    // 00626250, 00626280

    #[test]
    fn the_writer_destructor_deletes_an_owned_file() {
        let mut e = engine();
        let (stream, file) = stream_with_file(&mut e, 1);
        let writer = stream.cast::<HavokFileStreambufWriter>();
        assert_eq!(
            havok_file_streambuf_writer_scalar_deleting_destructor(&mut e, writer, 0),
            writer
        );
        assert_eq!(e.mem.u32(stream.addr()), VTABLE_FILE_WRITER);
        assert_eq!(calls(&e, SLOT_DELETE), [vec![file, 1]]);
        assert!(calls(&e, REFERENCED_OBJECT_FREE).is_empty());
        let (borrowed, _) = stream_with_file(&mut e, 0);
        fn_00626280(&mut e, borrowed.cast());
        assert_eq!(calls(&e, SLOT_DELETE).len(), 1);
    }

    // 00626100, 00626120, 00626160, 00626340, 00626390

    #[test]
    fn the_reader_forwards_to_its_file() {
        let mut e = engine();
        e.register(FILE_READ, |_, a| (a[2] + 1).into_ret());
        e.register(FILE_TELL, |_, a| (a[0] + 2).into_ret());
        e.register(FILE_IS_OK, |_, _| 0x101u32.into_ret());
        let (stream, file) = stream_with_file(&mut e, 1);
        let reader = stream.cast::<HavokFileStreambufReader>();
        assert_eq!(
            havok_file_streambuf_reader_read(&mut e, reader, 0x500, 16),
            17
        );
        assert_eq!(calls(&e, FILE_READ), [vec![file, 0x500, 16]]);
        assert_eq!(
            havok_file_streambuf_reader_sys_get_file_size(&mut e, reader),
            0x4321
        );
        assert_eq!(havok_file_streambuf_reader_tell(&mut e, reader), file + 2);
        assert_eq!(havok_file_streambuf_reader_seek(&mut e, reader, 40, 1), 0);
        assert_eq!(calls(&e, SLOT_SEEK), [vec![file, 40, 1]]);
        let result = Ptr::new(e.mem.alloc(4));
        assert_eq!(
            havok_file_streambuf_reader_is_ok(&mut e, reader, result),
            result
        );
        assert_eq!(e.mem.u8(result.addr()), 1);
        // Without a file.
        e.mem.set_u32(stream.addr() + 8, 0);
        assert_eq!(
            havok_file_streambuf_reader_sys_get_file_size(&mut e, reader),
            u32::MAX
        );
        assert_eq!(havok_file_streambuf_reader_tell(&mut e, reader), 0);
        assert_eq!(havok_file_streambuf_reader_seek(&mut e, reader, 40, 1), 1);
        assert_eq!(calls(&e, SLOT_SEEK).len(), 1);
        havok_file_streambuf_reader_is_ok(&mut e, reader, result);
        assert_eq!(e.mem.u8(result.addr()), 0);
        assert_eq!(calls(&e, FILE_IS_OK).len(), 1);
    }

    // 00626320

    #[test]
    fn the_writer_writes_through_its_file() {
        let mut e = engine();
        e.register(FILE_WRITE, |_, a| a[2].into_ret());
        let (stream, file) = stream_with_file(&mut e, 1);
        let writer = stream.cast::<HavokFileStreambufWriter>();
        assert_eq!(
            havok_file_streambuf_writer_write(&mut e, writer, 0x600, 8),
            8
        );
        assert_eq!(calls(&e, FILE_WRITE), [vec![file, 0x600, 8]]);
    }

    // 006261a0

    #[test]
    fn the_writer_constructor_opens_the_file() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BS_FILE_CONSTRUCT, |_, a| a[0].into_ret());
        let block = Ptr::new(e.mem.alloc(0x10));
        let writer = havok_file_streambuf_writer_havok_file_streambuf_writer(&mut e, block, 0x1234);
        assert_eq!(writer, block);
        assert_eq!(calls(&e, OPERATOR_NEW), [vec![0x158]]);
        let file = e.mem.u32(block.addr() + 8);
        assert_ne!(file, 0);
        assert_eq!(
            calls(&e, BS_FILE_CONSTRUCT),
            [vec![file, 0x1234, 1, 0x4000, 0]]
        );
        assert_eq!(e.mem.u32(block.addr()), VTABLE_FILE_WRITER);
        assert_eq!(e.mem.u8(block.addr() + 0xc), 1);
        // A failed allocation leaves no file.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        havok_file_streambuf_writer_havok_file_streambuf_writer(&mut e, block, 0x1234);
        assert_eq!(e.mem.u32(block.addr() + 8), 0);
        assert_eq!(calls(&e, BS_FILE_CONSTRUCT).len(), 1);
    }

    // 006263c0, 006263f0

    #[test]
    fn the_singleton_setters_release_the_old_object() {
        let mut e = engine();
        e.register(REMOVE_REFERENCE, nothing);
        fn_006263c0(&mut e, 0x111);
        assert_eq!(e.global::<u32>(ERROR_SINGLETON), 0x111);
        assert!(calls(&e, REMOVE_REFERENCE).is_empty());
        fn_006263c0(&mut e, 0x222);
        assert_eq!(e.global::<u32>(ERROR_SINGLETON), 0x222);
        assert_eq!(calls(&e, REMOVE_REFERENCE), [vec![0x111]]);
        fn_006263f0(&mut e, 0x333);
        assert_eq!(e.global::<u32>(FACTORY_SINGLETON), 0x333);
        fn_006263f0(&mut e, 0);
        assert_eq!(e.global::<u32>(FACTORY_SINGLETON), 0);
        assert_eq!(calls(&e, REMOVE_REFERENCE), [vec![0x111], vec![0x333]]);
    }

    // 00626420, 00626450, 00626470, 00626490, 006264d0

    #[test]
    fn the_container_wrappers_call_the_template_helpers() {
        let mut e = engine();
        e.register(CONTAINER_ALLOCATOR, |_, _| 0x1234u32.into_ret());
        e.register(ARRAY_APPEND, nothing);
        e.register(MAP_CONSTRUCT, nothing);
        e.register(MAP_INSERT, nothing);
        e.register(MAP_GET_WITH_DEFAULT, |_, a| (a[1] + a[2]).into_ret());
        e.register(MAP_CLEAR, nothing);
        let array = Ptr::new(e.mem.alloc(16));
        fn_00626420(&mut e, array, 0x999);
        assert_eq!(calls(&e, ARRAY_APPEND), [vec![array.addr(), 0x1234, 0x999]]);
        assert_eq!(calls(&e, CONTAINER_ALLOCATOR)[0][1], array.addr());
        assert_eq!(fn_00626450(&mut e, array), array);
        assert_eq!(calls(&e, MAP_CONSTRUCT), [vec![array.addr(), 0]]);
        fn_00626470(&mut e, array, 5, 6);
        assert_eq!(calls(&e, MAP_INSERT), [vec![array.addr(), 5, 6]]);
        assert_eq!(fn_00626490(&mut e, array, 5, 6), 11);
        fn_006264d0(&mut e, array);
        assert_eq!(calls(&e, MAP_CLEAR), [vec![array.addr()]]);
        assert_eq!(e.call(0x0062_6490, &args![array, 1u32, 2u32]).u32(), 3);
    }

    // 006264f0

    #[test]
    fn fn_006264f0_clears_and_deallocates() {
        let mut e = engine();
        stub(&mut e, &[MAP_CLEAR_AND_DEALLOCATE, EMPTY_LOCK]);
        e.register(CONTAINER_ALLOCATOR, |_, _| 0x55u32.into_ret());
        let map = Ptr::new(e.mem.alloc(16));
        fn_006264f0(&mut e, map);
        let steps = [CONTAINER_ALLOCATOR, MAP_CLEAR_AND_DEALLOCATE, EMPTY_LOCK];
        assert_eq!(order(&e, &steps), steps);
        assert_eq!(
            calls(&e, MAP_CLEAR_AND_DEALLOCATE),
            [vec![map.addr(), 0x55]]
        );
    }

    // 00626680

    #[test]
    fn fn_00626680_removes_and_fills_the_gap() {
        let mut e = engine();
        stub(&mut e, &[ARRAY_ELEMENT_DESTRUCT, ARRAY_ELEMENT_COPY]);
        let array = Ptr::new(e.mem.alloc(12));
        let data = e.mem.alloc(32);
        e.mem.set_u32(array.addr(), data);
        e.mem.set_u32(array.addr() + 4, 4);
        fn_00626680(&mut e, array, 1);
        assert_eq!(e.mem.u32(array.addr() + 4), 3);
        assert_eq!(calls(&e, ARRAY_ELEMENT_DESTRUCT), [vec![data + 8, 1, 0]]);
        assert_eq!(calls(&e, ARRAY_ELEMENT_COPY), [vec![data + 8, data + 24]]);
        // Removing the last element copies nothing.
        fn_00626680(&mut e, array, 2);
        assert_eq!(e.mem.u32(array.addr() + 4), 2);
        assert_eq!(calls(&e, ARRAY_ELEMENT_COPY).len(), 1);
    }

    #[test]
    fn assert_message_keeps_the_buffer_at_frame_offset_0x118() {
        let mut e = engine();
        put_exe_strings(&mut e);
        let answer = Rc::new(Cell::new(0));
        assert_handler(&mut e, answer);
        let buffers = Rc::new(RefCell::new(Vec::new()));
        let log = buffers.clone();
        e.register_double(SLOT_HANDLER, move |_, a| {
            log.borrow_mut().push(a[3]);
            0u32.into_ret()
        });
        let this = error_with_slots(&mut e);
        let file = put(&mut e, "C:\\src\\foo.cpp");
        // No label, no text, no root directory: only the empty buffer at +0x118 of 0x520 bytes.
        let frame = e.next_stack_local(0x520);
        havok_error_assert_message(&mut e, this, 0, 9, 0, file, 13);
        assert_eq!(*buffers.borrow(), vec![frame + 0x118]);
    }
}
