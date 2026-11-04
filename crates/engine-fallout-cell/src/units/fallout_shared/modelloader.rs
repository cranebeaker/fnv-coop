//! `fallout shared/modelloader.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit has 517 functions (`ledger queue "fallout shared/modelloader.cpp"`);
//! it is translated in address order, a session at a time. State of this file:
//! the first 200 functions, `0043aaf0` to `0043fe40`, which is the whole address
//! range of this file (the rest of the unit is in the part files). The first 40, to
//! `0043baa0`, are the `Model` and `KFModel` classes, `Model::InitModel` and
//! the small helpers the unit's compiler emitted next to them (`BSStream`,
//! `NiNode` and `NiFixedString` accessors, interlocked-operation wrappers).
//! The next 40, from `0043bac0`, are `LoadedFile`, `QueuedFile`,
//! `QueuedTexture` (constructors, `QueueMe`, `Run`, `Finish`, `Cancel`) and
//! the start of `QueuedModel`, with their flag, TLS and texture-map helpers.
//! The third 40, from `0043c890`, finish `QueuedModel` (constructors,
//! `CheckFinished`, `QueueMe`, `Run`, `Finish`, `GetDescription`), the
//! `BSStream` helpers `Run` needs, and begin the tree classes
//! (`QueuedTreeBillboard`, `QueuedTreeModel`).
//! The fourth 40, from `0043d8c0`, finish `QueuedTreeModel`, translate
//! `QueuedMagicItem` (`QueueMe`, `CheckFinished`), the task key packing of
//! `IOTask::GenerateKey`, and `QueuedKF` with its subclasses `QueuedAnimIdle`
//! and `QueuedReplacementKF` (up to `PostProcess`).
//! The fifth 40, from `0043ea00`, finish `QueuedReplacementKF` and its list
//! (`GetDescription`, the post-processed counter, `Cancel`), and translate
//! `QueuedHead`, `QueuedHelmet` (with its `NiMatrix3` helpers), the
//! `AttachDistant3DTask` post-process and `QueuedReference`'s constructor and
//! destructors. This file has no further functions: the next one, `0043fed0`,
//! belongs to `modelloader_p2.rs`.
//!
//! Layouts and helpers added here live at the top of the file, below.
//!
//! Not translated: the compiler's exception-unwinding frames (the `FS:[0]`
//! chains and state variables of the constructors, destructors and
//! `InitModel`) and the stack-cookie checks (`0043b7e0`, `0043bb80`, `0043bd10`). Locals the game
//! keeps on its stack and passes by address (the temporary `NiFixedString`s
//! and the 268-byte name buffer of `0043b7e0`) are heap blocks here, freed
//! where the game's scope ends.
//!
//! Stack arguments: several functions of the unit call a `thiscall` helper
//! with one stack argument pushed before the `this` is loaded. The
//! decompiler attaches such an argument to the wrong call when a helper
//! returns with a plain `RET` (see `0043b5b0`, `0043b7e0`); the translations
//! follow the disassembly.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiFixedString;

/// `NiPointer<T>::operator T*` (`00559450`): returns the pointer stored at
/// the `NiPointer`'s address.
pub(crate) const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<T>::operator=(T*)` (`0066b0d0`): releases the old object,
/// stores and adds a reference to the new one; returns the `NiPointer`.
pub(crate) const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer<T>::NiPointer(T*)` (`00633c90`): stores the object and adds a
/// reference.
pub(crate) const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer<T>::~NiPointer` (`0045cec0`): releases the object.
pub(crate) const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `MemoryManager` allocation, `__cdecl(size) -> block` (`00401000`).
pub(crate) const MEMORY_ALLOC: u32 = 0x0040_1000;
/// `MemoryManager` deallocation, `__cdecl(block)` (`00401030`).
pub(crate) const MEMORY_FREE: u32 = 0x0040_1030;
/// `strlen` through the game's wrapper (`0044a670`).
pub(crate) const STRLEN: u32 = 0x0044_a670;
/// `strcpy_s(destination, size, source)` through the game's wrapper (`00406d30`).
pub(crate) const STRING_COPY: u32 = 0x0040_6d30;
/// `memcpy(destination, source, count)` through the game's wrapper (`00401460`).
pub(crate) const MEMORY_COPY: u32 = 0x0040_1460;
/// Address of the name slot of an object (`this + 8`; `00413f40`).
pub(crate) const NAME_SLOT: u32 = 0x0041_3f40;
/// The array-element address getter of `BSStream` (`00877a30`):
/// `base + index * 4` of the array at `this`.
pub(crate) const ARRAY_ELEMENT_ADDRESS: u32 = 0x0087_7a30;
/// The element-count getter of that array (`0084e3a0`): the dword at +0xC.
pub(crate) const ARRAY_COUNT: u32 = 0x0084_e3a0;
/// `NiNode::NiNode(capacity)` (`00a5ecb0`).
pub(crate) const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// Allocator of `NiMemObject`-derived objects, `__cdecl(size) -> block`
/// (`00aa13e0`).
pub(crate) const NI_ALLOC: u32 = 0x00aa_13e0;
/// `NiAVObject::GetProperty(type)` (Xbox PDB, `00a59d30`).
pub(crate) const GET_PROPERTY: u32 = 0x00a5_9d30;
/// `NiAVObject::RemoveProperty_ov2(type)` (Xbox PDB, `00a5b230`).
pub(crate) const REMOVE_PROPERTY: u32 = 0x00a5_b230;
/// `RemoveEditorMarkers(node)` (Xbox PDB, `004b5d10`).
pub(crate) const REMOVE_EDITOR_MARKERS: u32 = 0x004b_5d10;
/// `HasMorpherController(node)` (Xbox PDB, `004b5bf0`).
pub(crate) const HAS_MORPHER_CONTROLLER: u32 = 0x004b_5bf0;
/// `BSShaderManager::PrepareObject(node, flag, flag)` (Xbox PDB, `00b57e30`).
pub(crate) const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// `BSStream::FreeAllObjects` (Xbox PDB, `00c3b370`).
pub(crate) const STREAM_FREE_ALL_OBJECTS: u32 = 0x00c3_b370;
/// The game's `printf`-style log (`005b5e40`), `__cdecl(format, ...)`.
pub(crate) const LOG: u32 = 0x005b_5e40;
/// Run-time type check, `__cdecl(type descriptor, object) -> object or 0`
/// (`00653270`).
pub(crate) const DYNAMIC_CAST: u32 = 0x0065_3270;
/// Reads the pointer an object `InitModel` cast holds (`00537bd0`, a
/// `NiPointer` read in the land unit).
pub(crate) const CAST_RESULT_POINTER: u32 = 0x0053_7bd0;
/// Run-time type test, `__thiscall(object, type descriptor) -> bool`
/// (`006532c0`).
pub(crate) const IS_KIND_OF: u32 = 0x0065_32c0;
/// `TESAnimGroup::LoadAnimGroup(sequence, name)` (Xbox PDB, `005f3a20`).
pub(crate) const LOAD_ANIM_GROUP: u32 = 0x005f_3a20;
/// Builds the `NiControllerSequence` of a `KFModel`, `__cdecl(file, 0,
/// NiPointer out)` (`004eeb60`).
pub(crate) const LOAD_KF_SEQUENCE: u32 = 0x004e_eb60;
/// `KFModel` accessor returning its `spSequence` (`007fa950`).
pub(crate) const KF_MODEL_SEQUENCE: u32 = 0x007f_a950;
/// `NiFixedString::NiFixedString(const char*)` (`00438170`): stores the
/// pooled handle, returns the string.
pub(crate) const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
/// `NiFixedString::~NiFixedString` (`004381b0`).
pub(crate) const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// Releases a `NiFixedString` handle (`004381d0`), `__cdecl(handle pointer)`.
pub(crate) const FIXED_STRING_RELEASE: u32 = 0x0043_81d0;
/// Address of the reference count of a pooled string, `__cdecl(handle
/// pointer)` (`00438210`).
pub(crate) const FIXED_STRING_REFCOUNT: u32 = 0x0043_8210;
/// `InterlockedIncrement` wrapper, `__cdecl(address)` (`0040b460`).
pub(crate) const INTERLOCKED_INCREMENT: u32 = 0x0040_b460;
/// `InterlockedDecrement` wrapper, `__cdecl(address)` (`004019a0`).
pub(crate) const INTERLOCKED_DECREMENT: u32 = 0x0040_19a0;
/// `Sleep` wrapper, `__cdecl(milliseconds)` (`0040fca0`).
pub(crate) const SLEEP: u32 = 0x0040_fca0;
/// `KERNEL32 InterlockedCompareExchange` (import slot).
pub(crate) const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
/// `strchr` (`00ec7690`).
pub(crate) const STRCHR: u32 = 0x00ec_7690;
/// `Bip[0-9][0-9]`-style pattern match, `__cdecl(text, pattern) -> bool`
/// (`00af3bd0`).
pub(crate) const MATCHES_PATTERN: u32 = 0x00af_3bd0;
/// `__thiscall(object, 1)` (`004534f0`): the call `0043b5b0` makes on the
/// object embedded in the block at `this`.
pub(crate) const DELETE_OBJECT: u32 = 0x0045_34f0;
/// Returns the field at +8 of the object it is called on (`00620b80`, the
/// same as `0044ddc0`).
pub(crate) const REF_COUNT_OBJECT_POINTER: u32 = 0x0062_0b80;
/// Returns `this + 0x10` (`00460140`).
pub(crate) const HOLDER_VALUE: u32 = 0x0046_0140;
/// An empty function (`00483710`).
pub(crate) const HOLDER_DESTRUCT: u32 = 0x0048_3710;
/// Reads the field at +8 of an object (`0044ddc0`).
pub(crate) const FIELD_AT_8: u32 = 0x0044_ddc0;
/// Stores a pointer into the object it is called on (`008c71b0`).
pub(crate) const STORE_POINTER: u32 = 0x008c_71b0;
/// `NiAVObject::m_spCollisionObject` getter (`006838b0`).
pub(crate) const COLLISION_OBJECT: u32 = 0x0068_38b0;
/// The array-size getter of `0043b480`'s array (`00658930`): the word at +0xA.
pub(crate) const ARRAY_SIZE: u32 = 0x0065_8930;

/// `"Meshes\\Marker_Error.NIF"`.
pub(crate) const MARKER_ERROR_NIF: u32 = 0x0101_667c;
/// `"Bip[0-9][0-9]"`.
pub(crate) const BIP_PATTERN: u32 = 0x0101_666c;
/// `"MODELS: %s: Reexport '%s' to get rid of the ZBuffer and/or VertextColor
/// property."`.
pub(crate) const REEXPORT_WITH_FILE_MESSAGE: u32 = 0x0101_6618;
/// `"MODELS: Reexport '%s' to get rid of the ZBuffer and/or VertextColor
/// property."`.
pub(crate) const REEXPORT_MESSAGE: u32 = 0x0101_65c8;
/// `"ANIMATION: Could not create ControllerManager Sequence for \"%s\".\r\n"`.
pub(crate) const NO_SEQUENCE_MESSAGE: u32 = 0x0101_6698;

/// `MessageHandler`'s disable-warning counter (a signed dword, never below 0).
pub(crate) const DISABLE_WARNING_COUNT: u32 = 0x0120_2d6c;
/// A byte flag `InitGunWobble` writes and `InitModel` reads (its meaning is
/// not known).
pub(crate) const GUN_WOBBLE_FLAG: u32 = 0x0118_5520;
/// The pooled empty string handle `NiFixedString` leaves unreferenced.
pub(crate) const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// The word `0043b3b0` returns when it finds nothing (`0xFFFF`).
pub(crate) const NOT_FOUND_INDEX: u32 = 0x0109_6320;
/// `NiPointer` globals holding the shared properties `InitModel` compares
/// against (the property types are `0043b290` and `00702440`).
pub(crate) const SHARED_PROPERTY_FIRST: u32 = 0x011f_4444;
pub(crate) const SHARED_PROPERTY_SECOND: u32 = 0x011f_4438;
/// Type descriptor `InitModel` casts the root's child to (the class is not
/// identified).
pub(crate) const TYPE_DESCRIPTOR_011F36AC: u32 = 0x011f_36ac;
/// Type descriptor `0043b7e0` tests the loaded sequence against (the class is
/// not identified).
pub(crate) const TYPE_DESCRIPTOR_011C7D74: u32 = 0x011c_7d74;
/// Type descriptor `0043b610` casts to (`bhkCollisionObject`).
pub(crate) const TYPE_DESCRIPTOR_BHK_COLLISION_OBJECT: u32 = 0x0120_43f8;
/// The property type `0043b290` returns.
pub(crate) const PROPERTY_TYPE_FIRST: u32 = 0xb;
/// Returns the second property type `InitModel` strips (`00702440`, in the
/// interface unit).
pub(crate) const PROPERTY_TYPE_SECOND_GETTER: u32 = 0x0070_2440;

// --- Queued files (`0043bac0` onwards): `LoadedFile`, `QueuedTexture`, `QueuedModel`. ---

/// `QueuedFileEntry::QueuedFileEntry(context)` (`00c3ce60`): the shared base
/// constructor of the queued texture and model.
pub(crate) const QUEUED_FILE_ENTRY_CONSTRUCT: u32 = 0x00c3_ce60;
/// `QueuedFileEntry` destructor body (`00c3cea0`).
pub(crate) const QUEUED_FILE_ENTRY_DESTRUCT: u32 = 0x00c3_cea0;
/// Copies a file name into `pFileName` (`00c3cee0`, `__thiscall(name)`).
pub(crate) const QUEUED_FILE_ENTRY_SET_FILE_NAME: u32 = 0x00c3_cee0;
/// Looks the file entry of `pFileName` up (`00c3cf60`, `__thiscall(flag)`).
pub(crate) const QUEUED_FILE_ENTRY_FIND_FILE_ENTRY: u32 = 0x00c3_cf60;
/// Stores `pFileEntry` (`00c3cf40`, `__thiscall(file entry)`).
pub(crate) const QUEUED_FILE_ENTRY_SET_FILE_ENTRY: u32 = 0x00c3_cf40;
/// `QueuedFileEntry::GetFile` (Xbox PDB, `00c3cff0`), `__thiscall(1, 2)`:
/// opens the file of the entry and returns the `BSFile` (0 on failure).
pub(crate) const QUEUED_FILE_ENTRY_GET_FILE: u32 = 0x00c3_cff0;
/// `QueuedFileEntry::GetDescription` (Xbox PDB, `00c3d0e0`),
/// `__thiscall(buffer, size, kind text) -> bool`.
pub(crate) const QUEUED_FILE_ENTRY_GET_DESCRIPTION: u32 = 0x00c3_d0e0;
/// `QueuedFile::Cancel` (Xbox PDB, `00c3cb70`), `__thiscall(2 arguments)`.
pub(crate) const QUEUED_FILE_CANCEL: u32 = 0x00c3_cb70;
/// Getter of `QueuedFileEntry::pFileEntry` (`0055b980`, `this + 0x2c`; the
/// linker folded it with the same getter of the object unit).
pub(crate) const QUEUED_FILE_ENTRY_GET_FILE_ENTRY: u32 = 0x0055_b980;
/// Getter of `QueuedFileEntry::pFileName` (`0045cd60`, `this + 0x28`).
pub(crate) const QUEUED_FILE_ENTRY_GET_FILE_NAME: u32 = 0x0045_cd60;
/// `BSFileEntry::iSize & 0x3fffffff` (`0062a100`).
pub(crate) const FILE_ENTRY_GET_SIZE: u32 = 0x0062_a100;
/// The archive that holds a file entry, `__cdecl(file entry, 2) -> archive`
/// (`00af6910`, in the archive unit; 0 when there is none).
pub(crate) const FILE_ENTRY_FIND_ARCHIVE: u32 = 0x00af_6910;
/// `__thiscall(flag, byte)` helpers of this unit, `__cdecl(flag, byte
/// pointer)`: set (non-zero `flag`) or clear one bit of the byte.
pub(crate) const SET_FLAG_BIT_1: u32 = 0x0044_ae20;
pub(crate) const SET_FLAG_BIT_2: u32 = 0x0044_ae60;
pub(crate) const SET_FLAG_BIT_4: u32 = 0x0044_aea0;
/// `__cdecl(flags byte) -> bool`: whether bit 2 / bit 4 of the byte is set.
pub(crate) const TEST_FLAG_BIT_2: u32 = 0x0044_ae50;
pub(crate) const TEST_FLAG_BIT_4: u32 = 0x0044_ae90;
/// `__thiscall(flag)` on a `QueuedTexture` (`0043e480`, [`fn_0043e480`]):
/// sets or clears bit 1 of `cFlags`.
pub(crate) const QUEUED_TEXTURE_SET_FLAG_1: u32 = 0x0043_e480;
/// `__thiscall()` on a `QueuedTexture` (`0043e530`, [`fn_0043e530`]): whether
/// bit 1 of `cFlags` is set.
pub(crate) const QUEUED_TEXTURE_TEST_FLAG_1: u32 = 0x0043_e530;
/// The object at `011c3b3c`, whose methods `00448330`, `00448370`,
/// `00448ed0` and `00448f50` (this unit, not translated yet) keep the maps
/// of loaded files and of queued textures.
pub(crate) const FILE_MAP_OWNER: u32 = 0x011c_3b3c;
/// `__thiscall(name, loaded file) -> bool` on [`FILE_MAP_OWNER`]: adds a
/// `LoadedFile` to the map of loaded files.
pub(crate) const FILE_MAP_ADD_LOADED_FILE: u32 = 0x0044_8330;
/// `__thiscall(name)` on [`FILE_MAP_OWNER`]: removes it again.
pub(crate) const FILE_MAP_REMOVE_LOADED_FILE: u32 = 0x0044_8370;
/// `__thiscall(file entry, queued texture) -> bool` on [`FILE_MAP_OWNER`]:
/// adds the texture to the map of queued textures by file entry.
pub(crate) const FILE_MAP_ADD_QUEUED_TEXTURE: u32 = 0x0044_8ed0;
/// `__thiscall(file entry)` on [`FILE_MAP_OWNER`]: removes it.
pub(crate) const FILE_MAP_REMOVE_QUEUED_TEXTURE: u32 = 0x0044_8f50;
/// The object at `01202d98` (a task queue); slot `0x48` takes the queued
/// texture to run.
pub(crate) const TASK_QUEUE: u32 = 0x0120_2d98;
/// The object at `011f6388` that `LoadedFile`'s destructor reports to
/// (slot `0x14`: `(severity, text, 0, 0, 0)`), 0 when there is none.
pub(crate) const MESSAGE_SINK: u32 = 0x011f_6388;
/// `BSTexturePalette::pTexMapA` (Xbox PDB), `011f4468`: the texture map.
pub(crate) const TEXTURE_MAP: u32 = 0x011f_4468;
/// The word `0043c4b0` returns (`011f4748`, a default texture setting).
pub(crate) const DEFAULT_TEXTURE_SETTING: u32 = 0x011f_4748;
/// `BSTexturePalette::SetTexture` (Xbox PDB, `00a61c50`), `__cdecl(texture,
/// file entry)`.
pub(crate) const TEXTURE_PALETTE_SET_TEXTURE: u32 = 0x00a6_1c50;
/// `BSTexturePalette::GetTexture_ov2` (Xbox PDB, `00a61b90`), `__cdecl(file
/// name, NiPointer out)`.
pub(crate) const TEXTURE_PALETTE_GET_TEXTURE_BY_NAME: u32 = 0x00a6_1b90;
/// Hash-bucket lookup of the texture map (`00a61a60`), `__thiscall(bucket,
/// file entry, NiPointer out) -> bool`.
pub(crate) const TEXTURE_MAP_FIND: u32 = 0x00a6_1a60;
/// Creates a texture from an open file, `__cdecl(NiFixedString*, file,
/// setting, format preferences)` (`00a61040`, in the cube map unit).
pub(crate) const CREATE_TEXTURE_FROM_FIXED_NAME: u32 = 0x00a6_1040;
/// The engine map's `NiSourceCubeMap::Create` (`00a5fe30`), `__cdecl(file,
/// name, format preferences, 1)`; `Run` takes it for names without `_e.dd`,
/// so the map's name is doubtful.
pub(crate) const CREATE_TEXTURE_FROM_FILE: u32 = 0x00a5_fe30;
/// `strstr` (`00ec7750`), `__cdecl(text, pattern)`.
pub(crate) const STRSTR: u32 = 0x00ec_7750;
/// `sprintf` (`00ec623a`), `__cdecl(buffer, format, ...)`.
pub(crate) const SPRINTF: u32 = 0x00ec_623a;
/// Path-normalizing copy, `__cdecl(name, buffer, size)` (`00af4200`).
pub(crate) const NORMALIZE_PATH: u32 = 0x00af_4200;
/// The texture's surface getter (`0059bb30`, `this + 0x24`; the engine map
/// calls it `D3DTexture_LockRect`).
pub(crate) const TEXTURE_GET_SURFACE: u32 = 0x0059_bb30;
/// The scope guard that sets the memory context for the rest of a function:
/// `__thiscall(context, 1, source file, line)` and its destructor.
pub(crate) const MEMORY_CONTEXT_ENTER: u32 = 0x0040_4eb0;
pub(crate) const MEMORY_CONTEXT_LEAVE: u32 = 0x0040_4ee0;
/// Releases the `NiPointer<Model>` at `this` (`0040c110`).
pub(crate) const MODEL_POINTER_RELEASE: u32 = 0x0040_c110;

/// `QueuedTexture`'s virtual table (`01016788`) and `QueuedModel`'s
/// (`01016890`).
pub(crate) const QUEUED_TEXTURE_VTABLE: u32 = 0x0101_6788;
pub(crate) const QUEUED_MODEL_VTABLE: u32 = 0x0101_6890;

/// `"texture"`.
pub(crate) const TEXTURE_WORD: u32 = 0x0101_6884;
/// `"_e.dd"`.
pub(crate) const ENVIRONMENT_MAP_SUFFIX: u32 = 0x0101_6838;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\ModelLoader.cpp"`.
pub(crate) const MODEL_LOADER_SOURCE: u32 = 0x0101_6840;
/// The source line the memory context scope of `Run` records.
pub(crate) const RUN_SOURCE_LINE: u32 = 0x2b7;
/// `"MODELS: Could not get file for texture %s."`.
pub(crate) const NO_FILE_FOR_TEXTURE_NAME_MESSAGE: u32 = 0x0101_67b8;
/// `"MODELS: Could not get file for texture with file entry offset %i and
/// size %i."`.
pub(crate) const NO_FILE_FOR_TEXTURE_ENTRY_MESSAGE: u32 = 0x0101_67e8;
/// `"LoadedFile %s was loaded at %i and added to the map, but is being thrown
/// away without being used"`.
pub(crate) const LOADED_FILE_UNUSED_MESSAGE: u32 = 0x0101_6720;
/// `"%s ref object at mem %i being destroyed with %i references\r\n"`.
pub(crate) const LOADED_FILE_REFERENCES_MESSAGE: u32 = 0x0101_66dc;
/// The format preferences object `Run` passes to the texture creators.
pub(crate) const TEXTURE_FORMAT_PREFERENCES: u32 = 0x011a_9598;

// --- `QueuedModel` and the tree classes (`0043c890` onwards). ---

/// `NiPointer<Model>::operator=(Model*)` (Xbox PDB `NiPointer<Model>::operator_`,
/// `0044aed0`, in this unit, not translated yet), `__thiscall(model)` on the
/// `NiPointer`.
pub(crate) const MODEL_POINTER_ASSIGN: u32 = 0x0044_aed0;
/// `QueuedFile::CheckFinished` (Xbox PDB, `00c3c930`).
pub(crate) const QUEUED_FILE_CHECK_FINISHED: u32 = 0x00c3_c930;
/// `QueuedFileEntry::GenerateKey` (Xbox PDB, `00c3d440`).
pub(crate) const QUEUED_FILE_ENTRY_GENERATE_KEY: u32 = 0x00c3_d440;
/// `BSStream::Load` (Xbox PDB, `00c3a8a0`), `__thiscall(name, file)`: reads
/// the stream from the open `BSFile`; false on failure.
pub(crate) const BS_STREAM_LOAD: u32 = 0x00c3_a8a0;
/// `NiStream::NiStream` (`00a66150`) and its destructor body (`00a65300`).
pub(crate) const NI_STREAM_CONSTRUCT: u32 = 0x00a6_6150;
pub(crate) const NI_STREAM_DESTRUCT: u32 = 0x00a6_5300;
/// `NiStreamSaveBinary<unsigned int>` (Xbox PDB, `0044adf0`),
/// `__cdecl(stream, value pointer)`.
pub(crate) const NI_STREAM_SAVE_BINARY_U32: u32 = 0x0044_adf0;
/// `ModelLoader::FindModel` (Xbox PDB, `004472a0`) on [`FILE_MAP_OWNER`],
/// `__thiscall(name, NiPointer<Model> out)`: gives the pointer the model of
/// that name when the loader has it.
pub(crate) const MODEL_LOADER_FIND_MODEL: u32 = 0x0044_72a0;
/// Adds a model to the loader's map (`00447010`, in this unit, not
/// translated yet), `__thiscall(name, model) -> bool` on [`FILE_MAP_OWNER`];
/// false when the name is already there.
pub(crate) const MODEL_LOADER_ADD_MODEL: u32 = 0x0044_7010;
/// `ModelLoader::QueueTexture_ov2` (Xbox PDB, `00443af0`) on
/// [`FILE_MAP_OWNER`], `__thiscall(texture, key, parent queued file)`.
pub(crate) const MODEL_LOADER_QUEUE_TEXTURE: u32 = 0x0044_3af0;
/// `BGSTextureSet::QueueTextureSet` (Xbox PDB, `005930e0`),
/// `__thiscall(key, parent queued file, 0)`.
pub(crate) const QUEUE_TEXTURE_SET: u32 = 0x0059_30e0;
/// `__thiscall()` on a `TESModel` (`0048d150`): `this + 0xC`, the texture
/// hash block `QueueMe_ov2` reads.
pub(crate) const TES_MODEL_TEXTURE_BLOCK: u32 = 0x0048_d150;
/// `__thiscall()` getters of the texture hash block (`009373f0`: the byte at
/// +0; `00726070`: the dword at +4, also the "next" link of the list nodes).
pub(crate) const BYTE_AT_0: u32 = 0x0093_73f0;
pub(crate) const FIELD_AT_4: u32 = 0x0072_6070;
/// List helpers of the texture set list: `00500940` returns `this + 0x18`
/// (the first node), `006815c0` returns the node itself (its item is the
/// first dword); the next node is [`FIELD_AT_4`].
pub(crate) const LIST_FIRST_NODE: u32 = 0x0050_0940;
pub(crate) const LIST_NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// Whether the dword at +0xC (`eState`) is 0 (`0069b080`; the engine map
/// files it under the navmesh code).
pub(crate) const STATE_IS_ZERO: u32 = 0x0069_b080;
/// The finished-child count of a `QueuedChildren` (`0044edb0`, named
/// `BaseProcess::GetCurrentProcedureIndex` by the engine map but folded: it
/// returns the dword at +0x10).
pub(crate) const FINISHED_COUNT_GETTER: u32 = 0x0044_edb0;
/// `__cdecl(flags byte) -> bool` tests of the other bits of a queued file's
/// flag byte, and the setter of bit 8 (`__cdecl(flag, byte pointer)`).
pub(crate) const TEST_FLAG_BIT_1: u32 = 0x0044_ae10;
pub(crate) const TEST_FLAG_BIT_8: u32 = 0x0044_af20;
pub(crate) const TEST_FLAG_BIT_10: u32 = 0x0044_af60;
pub(crate) const TEST_FLAG_BIT_20: u32 = 0x0044_afa0;
pub(crate) const SET_FLAG_BIT_8: u32 = 0x0044_af30;
/// `NiRefObject::IncRefCount` (`0092c870`): one more reference.
pub(crate) const ADD_REFERENCE: u32 = 0x0092_c870;
/// `NiAVObject::Update(NiUpdateData*)` as `Finish` calls it (`00a59c60`,
/// `__thiscall(update data)`).
pub(crate) const NI_AV_OBJECT_UPDATE: u32 = 0x00a5_9c60;
/// Radius of a `NiBound` (`0084d030`, the float at +0xC, returned in ST0).
pub(crate) const BOUND_RADIUS: u32 = 0x0084_d030;
/// `TESObjectREFR::GetBoundMax` (Xbox PDB, `0056fc30`), `__cdecl(out, node)
/// -> vector pointer`.
pub(crate) const GET_BOUND_MAX: u32 = 0x0056_fc30;
/// Address of the value of a setting object (`00403e20`: `this + 4`, or a
/// zero scratch float when `this` is 0).
pub(crate) const SETTING_VALUE_POINTER: u32 = 0x0040_3e20;
/// `QueuedModel::mfOverriddenVisualDistance` getter (`00792760`, in ST0).
pub(crate) const OVERRIDDEN_VISUAL_DISTANCE: u32 = 0x0079_2760;
/// `BSFadeNode::SetRange` (Xbox PDB, `00b4dfd0`), `__thiscall(float, float)`.
pub(crate) const FADE_NODE_SET_RANGE: u32 = 0x00b4_dfd0;
/// `BSFadeNode::SetLODMultType` (Xbox PDB, `00b4dec0`).
pub(crate) const FADE_NODE_SET_LOD_MULT_TYPE: u32 = 0x00b4_dec0;
/// The setting objects `Finish` reads: the world size in cells (`011c63cc`,
/// +4 is the value), the scale of the visual distance of a model
/// (`011c3be4`) and the scale of the fade range (`011c3bb8`).
pub(crate) const CELL_COUNT_SETTING: u32 = 0x011c_63cc;
pub(crate) const VISUAL_DISTANCE_SCALE_SETTING: u32 = 0x011c_3be4;
pub(crate) const FADE_RANGE_SCALE_SETTING: u32 = 0x011c_3bb8;
/// The zero scratch dword `0043d4d0` returns the address of when it has no
/// object.
pub(crate) const SCRATCH_ZERO: u32 = 0x0120_2800;
/// The bound `NiAVObject::GetWorldBound` returns when the object has none.
pub(crate) const EMPTY_WORLD_BOUND: u32 = 0x011f_4288;
/// Doubles in `.rdata`: 0.001, 2048.0, 0.5 and 0.0.
pub(crate) const SMALL_RADIUS: u32 = 0x0101_6978;
pub(crate) const LOADED_AREA_MARGIN: u32 = 0x0101_6968;
pub(crate) const HALF: u32 = 0x0101_1588;
pub(crate) const ZERO: u32 = 0x0101_2060;
/// The file name of the model being loaded (`011f94c0`): `QueuedModel::Run`
/// sets it while the stream loads and clears it afterwards.
pub(crate) const LOADING_FILE_NAME: u32 = 0x011f_94c0;
/// `TESObjectTREE::BuildDistant3D` (Xbox PDB, `0051cba0`), `__thiscall(1)`.
pub(crate) const BUILD_DISTANT_3D: u32 = 0x0051_cba0;
/// Zeroes the three words of the object (`00b57e70`).
pub(crate) const ZERO_THREE_WORDS: u32 = 0x00b5_7e70;
/// Builds the instanced billboards (`00b59ae0`), `__cdecl(cell chunk, cell
/// key, instanced node, out, locations, colors, count)`.
pub(crate) const CREATE_BILLBOARD_INSTANCES: u32 = 0x00b5_9ae0;
/// The destructor body of `TREE_BILLBOARD_DATA` (`0051d400`).
pub(crate) const TREE_BILLBOARD_DATA_DESTRUCT: u32 = 0x0051_d400;
/// The smart pointer to a task: `__thiscall(task)` constructor that adds a
/// reference (`00528cb0`), and its destructor (`0044cbf0`).
pub(crate) const TASK_POINTER_CONSTRUCT: u32 = 0x0052_8cb0;
pub(crate) const TASK_POINTER_DESTRUCT: u32 = 0x0044_cbf0;
/// Puts a task into the queue of its priority (`00449210`),
/// `__thiscall(priority, task pointer)` on the table of queues at
/// `IOManager + 0x64`.
pub(crate) const QUEUE_TABLE_ADD: u32 = 0x0044_9210;

/// `BSStream`'s virtual table (`01016904`), `QueuedTreeBillboard`'s
/// (`0101698c`) and `QueuedTreeModel`'s (`010169d4`).
pub(crate) const BS_STREAM_VTABLE: u32 = 0x0101_6904;
pub(crate) const QUEUED_TREE_BILLBOARD_VTABLE: u32 = 0x0101_698c;
pub(crate) const QUEUED_TREE_MODEL_VTABLE: u32 = 0x0101_69d4;

/// `"MODEL ERROR: Could not %s file '%s'."`, and its two verbs.
pub(crate) const MODEL_ERROR_MESSAGE: u32 = 0x0101_68c4;
pub(crate) const FIND_WORD: u32 = 0x0101_68ec;
pub(crate) const LOAD_WORD: u32 = 0x0101_68f4;
/// `"model"` and `"Tree Billboard"`.
pub(crate) const MODEL_WORD: u32 = 0x0101_6980;
pub(crate) const TREE_BILLBOARD_WORD: u32 = 0x0101_69c0;
/// The source lines the memory context scopes of `QueuedModel::Run` and
/// `Finish` record.
pub(crate) const MODEL_RUN_SOURCE_LINE: u32 = 0x3b8;
pub(crate) const MODEL_FINISH_SOURCE_LINE: u32 = 0x3e5;
/// The `eContext` a queued tree model starts with (`0x1e`).
pub(crate) const TREE_MODEL_CONTEXT: u32 = 0x1e;

/// The offsets in a thread's TLS block `QueuedTexture` uses: a byte flag
/// (read by `0043c130`) and a word (`0043c410`/`0043c3f0`) that `Run` sets to
/// 1 while it loads a texture whose flag bit 2 is set.
pub(crate) const TLS_QUEUED_FLAG: u32 = 0x25c;
pub(crate) const TLS_QUEUED_VALUE: u32 = 0x29c;
/// Puts a queued task into state 5, done (`00449150`: `this + 0xC = 5`).
pub(crate) const TASK_SET_DONE: u32 = 0x0044_9150;

layout! {
    /// `Model` (Xbox PDB), 0x18 bytes on the Xbox, 0x10 on PC (the two
    /// leading memory-accounting words `iVBMem` and `iDefaultMem` are gone,
    /// so every field sits 8 bytes earlier).
    pub struct Model: 0x10 {
        /// `pFilename` (Xbox PDB): `char*`, a copy owned by the model.
        0x00 pFilename: u32,
        /// `iRefCount` (Xbox PDB).
        0x04 iRefCount: i32,
        /// `iManualRefCount` (Xbox PDB): changed with interlocked operations.
        0x08 iManualRefCount: i32,
        /// `spObject3D` (Xbox PDB): `NiPointer<NiNode>`.
        0x0C spObject3D: Ptr,
    }

    /// `KFModel` (Xbox PDB), 0x1C bytes on the Xbox, 0x14 on PC (without
    /// `iVBMem` and `iDefaultMem`).
    pub struct KFModel: 0x14 {
        /// `pFilename` (Xbox PDB): `char*`, a copy owned by the model.
        0x00 pFilename: u32,
        /// `spSequence` (Xbox PDB): `NiPointer<NiControllerSequence>`.
        0x04 spSequence: Ptr,
        /// `spAnimGroup` (Xbox PDB): `NiPointer<TESAnimGroup>`.
        0x08 spAnimGroup: Ptr,
        /// `iRefCount` (Xbox PDB).
        0x0C iRefCount: i32,
        /// `iManualRefCount` (Xbox PDB).
        0x10 iManualRefCount: i32,
    }
}

layout! {
    /// `LoadedFile` (Xbox PDB), 0x10 bytes (the same on PC): a file kept
    /// open by the loader, found by name in the map of loaded files.
    pub struct LoadedFile: 0x10 {
        /// `iRefCount` (Xbox PDB).
        0x00 iRefCount: i32,
        /// `pFileName` (Xbox PDB): `char*`, a copy owned by the object.
        0x04 pFileName: u32,
        /// `pFile` (Xbox PDB): `BSFile*`, deleted with the object.
        0x08 pFile: Ptr,
        /// `bInLoadedFileMap` (Xbox PDB).
        0x0C bInLoadedFileMap: bool,
        /// `bFileUsed` (Xbox PDB).
        0x0D bFileUsed: bool,
    }

    /// `QueuedChildren` (Xbox PDB), 0x14 bytes: the Xbox class derives from
    /// `BSSimpleArray<NiPointer<QueuedFile>>`; only the counter at the end
    /// is used here.
    pub struct QueuedChildren: 0x14 {
        /// `iNumChildrenFinished` (Xbox PDB).
        0x10 iNumChildrenFinished: u32,
    }

    /// `QueuedFile` (Xbox PDB), 0x28 bytes on the Xbox (virtual table, the
    /// `IOTask` base, then these); the PC offsets used here are the same.
    pub struct QueuedFile: 0x28 {
        /// `eContext` (Xbox PDB): the memory context the task runs in.
        0x18 eContext: u32,
        /// `pChildren` (Xbox PDB): `QueuedChildren*`.
        0x20 pChildren: Ptr,
    }

    /// `QueuedTexture` (Xbox PDB), 0x38 bytes on the Xbox (a `QueuedFile`,
    /// then `QueuedFileEntry`'s `pFileName` and `pFileEntry`, then these);
    /// the offsets used here are the same on PC.
    pub struct QueuedTexture: 0x38 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pChildren` (`QueuedFile`, Xbox PDB).
        0x20 pChildren: Ptr,
        /// `pFileName` (`QueuedFileEntry`, Xbox PDB).
        0x28 pFileName: u32,
        /// `pFileEntry` (`QueuedFileEntry`, Xbox PDB): `BSFileEntry*`.
        0x2C pFileEntry: Ptr,
        /// `spTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0x30 spTexture: Ptr,
        /// `cFlags` (Xbox PDB). Bit 4: the texture is in the map of queued
        /// textures (set from the result of the map's add, cleared by
        /// `Finish` and `Cancel` when they remove it). Bit 2: set when the
        /// thread flag `TLS_QUEUED_FLAG` was set at queueing time; `Run`
        /// then sets the TLS word while it loads. Bit 1 is set by
        /// `0043e480` and tested by `0043e530` (not translated yet).
        0x34 cFlags: u8,
    }

    /// `QueuedModel` (Xbox PDB), 0x48 bytes on the Xbox; the offsets used
    /// here are the same on PC.
    pub struct QueuedModel: 0x48 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pChildren` (`QueuedFile`, Xbox PDB): `QueuedChildren*`.
        0x20 pChildren: Ptr,
        /// `spModel` (Xbox PDB): `NiPointer<Model>`.
        0x30 spModel: Ptr,
        /// `pTESModel` (Xbox PDB): `TESModel*`.
        0x34 pTESModel: Ptr,
        /// `eLODFadeMult` (Xbox PDB).
        0x38 eLODFadeMult: u32,
        /// `cFlags` (Xbox PDB): bit 1 and bit 2 are set by the constructor's
        /// last two arguments.
        0x3C cFlags: u8,
        /// `mfOverriddenVisualDistance` (Xbox PDB).
        0x40 mfOverriddenVisualDistance: f32,
    }
}

layout! {
    /// `BSStream` (Xbox PDB), 0x5cc bytes (the `NiStream` base, then these):
    /// `QueuedModel::Run` keeps one on its stack.
    pub struct BSStream: 0x5cc {
        /// `pObjectRefMap` (Xbox PDB): `NiTStringMap<NiObjectNET*>*`, deleted
        /// (virtual destructor, flag 1) by `0043d100`.
        0x5C4 pObjectRefMap: Ptr,
        /// `spNodeReferences` (Xbox PDB): `NiPointer<BSNodeReferences>`.
        0x5C8 spNodeReferences: Ptr,
    }

    /// `IOTask` (Xbox PDB), 0x18 bytes: the base of the queued files.
    pub struct IOTask: 0x18 {
        /// `Key` (Xbox PDB): `i64`; bits 16 to 23 are the priority of the
        /// queue the task goes into.
        0x10 Key: u64,
    }

    /// `NiUpdateData` (Xbox PDB), 0xc bytes.
    pub struct NiUpdateData: 0xc {
        /// `fTime` (Xbox PDB).
        0x00 fTime: f32,
        /// `bUpdateControllers` (Xbox PDB).
        0x04 bUpdateControllers: u8,
        /// `bParallelUpdate` (Xbox PDB).
        0x05 bParallelUpdate: u8,
        /// `bFoundParticles` (Xbox PDB).
        0x06 bFoundParticles: u8,
        /// `bFoundMorphController` (Xbox PDB).
        0x07 bFoundMorphController: u8,
        /// `bSceneGraphChange` (Xbox PDB).
        0x08 bSceneGraphChange: u8,
    }

    /// `TREE_BILLBOARD_DATA` (Xbox PDB), 0x1c bytes (the same on PC).
    pub struct TreeBillboardData: 0x1c {
        /// `pTree` (Xbox PDB): `TESObjectTREE*`.
        0x00 pTree: Ptr,
        /// `iCellChunk` (Xbox PDB).
        0x04 iCellChunk: u32,
        /// `iCellKey` (Xbox PDB).
        0x08 iCellKey: u32,
        /// `pInstancedNode` (Xbox PDB): `NiNode*`.
        0x0C pInstancedNode: Ptr,
        /// `iArraySize` (Xbox PDB); the game reads the low word.
        0x10 iArraySize: u32,
        /// `pLocArray` (Xbox PDB): `NiPoint3*`.
        0x14 pLocArray: Ptr,
        /// `pColorArray` (Xbox PDB): `f32*`.
        0x18 pColorArray: Ptr,
    }

    /// `QueuedTreeBillboard` (Xbox PDB), 0x40 bytes: a `QueuedTexture` and
    /// the data to build the billboard from.
    pub struct QueuedTreeBillboard: 0x40 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pBillboardData` (Xbox PDB): `TREE_BILLBOARD_DATA*`, owned.
        0x38 pBillboardData: Ptr,
    }

    /// `QueuedTreeModel` (Xbox PDB), 0x50 bytes: a `QueuedModel` and the
    /// reference and tree it is for.
    pub struct QueuedTreeModel: 0x50 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x48 pRef: Ptr,
        /// `pTree` (Xbox PDB): `TESObjectTREE*`.
        0x4C pTree: Ptr,
    }
}

/// The address of `Model::spObject3D`, which the game passes to the
/// `NiPointer` functions.
pub(crate) fn object_3d_pointer(model: Ptr<Model>) -> Ptr {
    model.byte_add(Model::spObject3D.off)
}

/// The address of `KFModel::spSequence`.
pub(crate) fn sequence_pointer(model: Ptr<KFModel>) -> Ptr {
    model.byte_add(KFModel::spSequence.off)
}

/// The address of `KFModel::spAnimGroup`.
pub(crate) fn anim_group_pointer(model: Ptr<KFModel>) -> Ptr {
    model.byte_add(KFModel::spAnimGroup.off)
}

/// `NiPointer::operator T*` on the `NiPointer` at `pointer`.
pub(crate) fn ni_pointer_get(e: &mut Engine, pointer: Ptr) -> Ptr {
    e.call(NI_POINTER_GET, &args![pointer]).ptr()
}

/// Copies the NUL-terminated `name` into a fresh block of the memory manager
/// (what the constructors do for `pFilename`).
pub(crate) fn copy_name(e: &mut Engine, name: u32) -> u32 {
    let size = e.call(STRLEN, &args![name]).u32().wrapping_add(1);
    let copy = e.call(MEMORY_ALLOC, &args![size]).u32();
    e.call(STRING_COPY, &args![copy, size, name]);
    copy
}

/// A fresh `NiNode` for the model to hold (`operator new` then constructor,
/// 0 when the allocation failed).
pub(crate) fn new_ni_node(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOC, &args![0xacu32]).u32();
    if block == 0 {
        0
    } else {
        e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
    }
}

// Translated from 0043aaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Model::Model` (Xbox PDB): clears the object pointer and loads the model
/// `name` from `stream` ([`model_init_model`]); returns `this`.
pub fn model_model(
    e: &mut Engine,
    this: Ptr<Model>,
    name: u32,
    stream: Ptr,
    prepare: u8,
    force: u8,
) -> Ptr<Model> {
    let object_3d = object_3d_pointer(this);
    e.call(NI_POINTER_CONSTRUCT, &args![object_3d, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![object_3d, 0u32]);
    model_init_model(e, this, name, stream, prepare, force);
    this
}

// Translated from 0043ab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Model::~Model` (Xbox PDB): releases the 3D object, frees the file name
/// and destroys the `NiPointer`.
pub fn model_destructor(e: &mut Engine, this: Ptr<Model>) {
    let object_3d = object_3d_pointer(this);
    e.call(NI_POINTER_ASSIGN, &args![object_3d, 0u32]);
    let filename = e.get(this, Model::pFilename);
    e.call(MEMORY_FREE, &args![filename]);
    e.call(NI_POINTER_DESTRUCT, &args![object_3d]);
}

// Translated from 0043abf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A second `Model` constructor (the map has no name for it): copies `name`,
/// holds the already loaded `node` as the 3D object and clears both counts.
/// Returns `this`.
pub fn fn_0043abf0(e: &mut Engine, this: Ptr<Model>, name: u32, node: Ptr) -> Ptr<Model> {
    let object_3d = object_3d_pointer(this);
    e.call(NI_POINTER_CONSTRUCT, &args![object_3d, 0u32]);
    let copy = copy_name(e, name);
    e.set(this, Model::pFilename, copy);
    e.call(NI_POINTER_ASSIGN, &args![object_3d, node]);
    e.set(this, Model::iRefCount, 0);
    e.set(this, Model::iManualRefCount, 0);
    this
}

// Translated from 0043acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decrements `iManualRefCount` (interlocked) when it is positive.
pub fn fn_0043acb0(e: &mut Engine, this: Ptr<Model>) {
    if e.get(this, Model::iManualRefCount) > 0 {
        let counter = this.byte_add(Model::iManualRefCount.off);
        e.call(INTERLOCKED_DECREMENT, &args![counter]);
    }
}

// Translated from 0043ace0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Model::InitModel` (Xbox PDB): copies `name` into `pFilename`, takes the
/// `NiNode`s out of the loaded `stream` into `spObject3D` and prepares them.
///
/// A stream without objects first loads `Meshes\Marker_Error.NIF` (and leaves
/// `pFilename` null if even that fails). A first object that is a node
/// becomes the model's object directly, unless its name matches `Bip##`, in
/// which case it is added under a fresh wrapper node. Otherwise every node or
/// geometry of the stream is added, in order, to a wrapper node. When
/// something was taken, the editor markers are removed, the two shared
/// properties (z-buffer and vertex colour) are stripped from the root when
/// it holds exactly those objects (with a "Reexport" log message), and, if
/// `prepare` is set, the object is handed to the shader manager (`force` or a
/// morpher controller in the tree sets its last flag).
pub fn model_init_model(
    e: &mut Engine,
    this: Ptr<Model>,
    name: u32,
    stream: Ptr,
    prepare: u8,
    force: u8,
) {
    e.set(this, Model::iRefCount, 0);
    e.set(this, Model::iManualRefCount, 0);
    let copy = copy_name(e, name);
    e.set(this, Model::pFilename, copy);

    if fn_0043b1e0(e, stream) == 0 {
        e.call(STREAM_FREE_ALL_OBJECTS, &args![stream]);
        // Slot 0x60 of the stream: load a file into it.
        if !e
            .vcall(stream.addr(), 0x60, &args![MARKER_ERROR_NIF, 0u32])
            .bool()
        {
            e.set(this, Model::pFilename, 0);
            return;
        }
    }

    let object_3d = object_3d_pointer(this);
    let first = fn_0043b200(e, stream, 0);
    let mut taken = false;
    // Slot 0xC of an `NiObject`: `IsNode` (non-null for an `NiNode`).
    if !first.is_null() && e.vcall(first.addr(), 0xc, &args![]).u32() != 0 {
        let name_slot = e.call(NAME_SLOT, &args![first]).ptr();
        let node_name = fn_0043b1b0(e, name_slot);
        if node_name != 0
            && e.call(MATCHES_PATTERN, &args![node_name, BIP_PATTERN])
                .bool()
        {
            let wrapper = new_ni_node(e);
            e.call(NI_POINTER_ASSIGN, &args![object_3d, wrapper]);
            let root = ni_pointer_get(e, object_3d);
            // Slot 0xF8 of `NiNode`: `SetAt(index, child)`.
            e.vcall(root.addr(), 0xf8, &args![0u32, first]);
        } else {
            e.call(NI_POINTER_ASSIGN, &args![object_3d, first]);
        }
        taken = true;
    } else {
        let mut slot = 0u32;
        let mut index = 0u32;
        while index < fn_0043b1e0(e, stream) {
            let object = fn_0043b200(e, stream, index);
            if !object.is_null() {
                // Slot 0xC `IsNode`, slot 0x18 `IsGeometry`.
                let takes = e.vcall(object.addr(), 0xc, &args![]).u32() != 0
                    || e.vcall(object.addr(), 0x18, &args![]).u32() != 0;
                if takes {
                    if ni_pointer_get(e, object_3d).is_null() {
                        let wrapper = new_ni_node(e);
                        e.call(NI_POINTER_ASSIGN, &args![object_3d, wrapper]);
                    }
                    let root = ni_pointer_get(e, object_3d);
                    e.vcall(root.addr(), 0xf8, &args![slot, object]);
                    slot += 1;
                    taken = true;
                }
            }
            index += 1;
        }
    }
    if !taken {
        return;
    }

    let root = ni_pointer_get(e, object_3d);
    e.call(REMOVE_EDITOR_MARKERS, &args![root]);

    if e.global::<u8>(GUN_WOBBLE_FLAG) != 0 {
        let root = ni_pointer_get(e, object_3d);
        let child = fn_0043b230(e, root);
        let cast = e.call(DYNAMIC_CAST, &args![TYPE_DESCRIPTOR_011F36AC, child]);
        if cast.u32() != 0 {
            let target: Ptr = e.call(CAST_RESULT_POINTER, &args![cast.u32()]).ptr();
            if !target.is_null() {
                let root = ni_pointer_get(e, object_3d);
                let name_slot = e.call(NAME_SLOT, &args![root]).ptr();
                if fn_0043b1b0(e, name_slot) != 0 {
                    let root = ni_pointer_get(e, object_3d);
                    let name_slot = e.call(NAME_SLOT, &args![root]);
                    e.vcall(target.addr(), 0x90, &args![name_slot.u32(), 0u32]);
                }
            }
        }
    }

    let mut stripped = false;
    message_handler_inc_disable_warning_count(e, 1);
    let property_type = fn_0043b290(e);
    let root = ni_pointer_get(e, object_3d);
    let property: Ptr = e.call(GET_PROPERTY, &args![root, property_type]).ptr();
    if !property.is_null() {
        let shared = fn_0043b2a0(e);
        if fn_0043b260(e, property, shared) {
            let property_type = fn_0043b290(e);
            let root = ni_pointer_get(e, object_3d);
            e.call(REMOVE_PROPERTY, &args![root, property_type]);
            stripped = true;
        }
    }
    let property_type = e.call(PROPERTY_TYPE_SECOND_GETTER, &args![]).u32();
    let root = ni_pointer_get(e, object_3d);
    let property: Ptr = e.call(GET_PROPERTY, &args![root, property_type]).ptr();
    if !property.is_null() {
        let shared = fn_0043b250(e);
        if fn_0043b260(e, property, shared) {
            let property_type = e.call(PROPERTY_TYPE_SECOND_GETTER, &args![]).u32();
            let root = ni_pointer_get(e, object_3d);
            e.call(REMOVE_PROPERTY, &args![root, property_type]);
            stripped = true;
        }
    }
    if stripped {
        // `BSStream` byte at +8 (the file name's first character): non-zero
        // when the stream has a file name.
        if e.mem.i8(stream.addr() + 8) != 0 {
            e.call(
                LOG,
                &args![REEXPORT_WITH_FILE_MESSAGE, stream.addr() + 8, name],
            );
        } else {
            e.call(LOG, &args![REEXPORT_MESSAGE, name]);
        }
    }
    message_handler_inc_disable_warning_count(e, 0);

    if prepare == 0 {
        return;
    }
    let mut has_morpher = 0u32;
    if force != 0 {
        has_morpher = 1;
    } else {
        let root = ni_pointer_get(e, object_3d);
        if e.call(HAS_MORPHER_CONTROLLER, &args![root]).bool() {
            has_morpher = 1;
        }
    }
    let root = ni_pointer_get(e, object_3d);
    e.call(PREPARE_OBJECT, &args![root, force as u32, has_morpher]);
}

// Translated from 0043b1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at `pointer`, read through the acquire-load helper `0043b1d0`
/// (the name slot of an object is read this way).
pub fn fn_0043b1b0(e: &mut Engine, pointer: Ptr) -> u32 {
    fn_0043b1d0(e, pointer)
}

// Translated from 0043b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Concurrency::details::_Subatomic_impl<4>::_LoadWithAquire` (library): the
/// dword at `pointer`.
pub fn fn_0043b1d0(e: &mut Engine, pointer: Ptr) -> u32 {
    e.mem.u32(pointer.addr())
}

// Translated from 0043b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream`: the number of objects in the stream's array at +0x21C.
pub fn fn_0043b1e0(e: &mut Engine, stream: Ptr) -> u32 {
    e.call(ARRAY_COUNT, &args![stream.byte_add(0x21c)]).u32()
}

// Translated from 0043b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream`: the object at `index` of the stream's array at +0x21C.
pub fn fn_0043b200(e: &mut Engine, stream: Ptr, index: u32) -> Ptr {
    let element = e
        .call(ARRAY_ELEMENT_ADDRESS, &args![stream.byte_add(0x21c), index])
        .ptr::<()>();
    ni_pointer_get(e, element)
}

// Translated from 0043b230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPointer` at +0xC of `object` dereferenced (in Gamebryo's
/// `NiObjectNET` layout, the head of the controller list).
pub fn fn_0043b230(e: &mut Engine, object: Ptr) -> Ptr {
    ni_pointer_get(e, object.byte_add(0xc))
}

// Translated from 0043b250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The shared property held by the global `NiPointer` at `011f4438`.
pub fn fn_0043b250(e: &mut Engine) -> Ptr {
    ni_pointer_get(e, Ptr::new(SHARED_PROPERTY_SECOND))
}

// Translated from 0043b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `this` and `other` have the same value in the 16-bit field at
/// +0x18 (the shared-property check of `InitModel`).
pub fn fn_0043b260(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.mem.u16(this.addr() + 0x18) == e.mem.u16(other.addr() + 0x18)
}

// Translated from 0043b290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The property type `InitModel` strips first (11).
pub fn fn_0043b290(_e: &mut Engine) -> u32 {
    PROPERTY_TYPE_FIRST
}

// Translated from 0043b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The shared property held by the global `NiPointer` at `011f4444`.
pub fn fn_0043b2a0(e: &mut Engine) -> Ptr {
    ni_pointer_get(e, Ptr::new(SHARED_PROPERTY_FIRST))
}

// Translated from 0043b2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageHandler::IncDisableWarningCount` (Xbox PDB): increments (when
/// `increment` is non-zero) or decrements the disable-warning counter with an
/// interlocked operation and clamps it at 0.
pub fn message_handler_inc_disable_warning_count(e: &mut Engine, increment: u8) {
    if increment != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![DISABLE_WARNING_COUNT]);
    } else {
        e.call(INTERLOCKED_DECREMENT, &args![DISABLE_WARNING_COUNT]);
    }
    if e.global::<i32>(DISABLE_WARNING_COUNT) < 0 {
        e.set_global(DISABLE_WARNING_COUNT, 0i32);
    }
}

// Translated from 0043b300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `object` is an instance of the class with type descriptor
/// `type_descriptor`; false for a null `object`.
pub fn fn_0043b300(e: &mut Engine, type_descriptor: u32, object: Ptr) -> bool {
    if object.is_null() {
        return false;
    }
    e.call(IS_KIND_OF, &args![object, type_descriptor]).bool()
}

// Translated from 0043b320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode`: stores `child` at `index` of the child array (base at +0x38) and
/// sets the flag `0x80` of a non-null child ([`fn_0043b350`]).
pub fn fn_0043b320(e: &mut Engine, this: Ptr, index: u16, child: Ptr) {
    let base = e.mem.u32(this.addr() + 0x38);
    e.mem.set_u32(base + index as u32 * 4, child.addr());
    if !child.is_null() {
        fn_0043b350(e, child, 1);
    }
}

// Translated from 0043b350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears the flag `0x80` of the flag word at +0x30.
pub fn fn_0043b350(e: &mut Engine, this: Ptr, set: u8) {
    fn_0043b370(e, this, set, 0x80);
}

// Translated from 0043b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears the bits `mask` of the flag word at +0x30.
pub fn fn_0043b370(e: &mut Engine, this: Ptr, set: u8, mask: u32) {
    let flags = e.mem.u32(this.addr() + 0x30);
    let flags = if set != 0 {
        flags | mask
    } else {
        !mask & flags
    };
    e.mem.set_u32(this.addr() + 0x30, flags);
}

// Translated from 0043b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode`: the index of `child` in the child array (base at +0x38, 16-bit
/// count at +0x3C), or the word at `01096320` (`0xFFFF`) when it is not there.
pub fn fn_0043b3b0(e: &mut Engine, this: Ptr, child: Ptr) -> u16 {
    let base = e.mem.u32(this.addr() + 0x38);
    let count = e.mem.u16(this.addr() + 0x3c);
    for index in 0..count {
        if e.mem.u32(base + index as u32 * 4) == child.addr() {
            return index;
        }
    }
    e.global::<u16>(NOT_FOUND_INDEX)
}

// Translated from 0043b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Model::ModManualRefCount` (Xbox PDB): adds `delta` to `iManualRefCount`
/// with a compare-and-swap loop (sleeping between failed attempts).
pub fn model_mod_manual_ref_count(e: &mut Engine, this: Ptr<Model>, delta: i32) {
    let counter = this.byte_add(Model::iManualRefCount.off);
    let mut current = e.get(this, Model::iManualRefCount);
    while fn_0043b460(e, counter, current, current.wrapping_add(delta)) != current {
        e.call(SLEEP, &args![0u32]);
        current = e.get(this, Model::iManualRefCount);
    }
}

// Translated from 0043b460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterlockedCompareExchange(target, new, old)` wrapper: returns the value
/// `target` held.
pub fn fn_0043b460(e: &mut Engine, target: Ptr, old: i32, new: i32) -> i32 {
    e.call(INTERLOCKED_COMPARE_EXCHANGE, &args![target, new, old])
        .i32()
}

// Translated from 0043b480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of elements of the array at +0x9C.
pub fn fn_0043b480(e: &mut Engine, this: Ptr) -> u16 {
    e.call(ARRAY_SIZE, &args![this.byte_add(0x9c)]).u16()
}

// Translated from 0043b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object at `index` of the array at +0x9C.
pub fn fn_0043b4a0(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let element = e
        .call(ARRAY_ELEMENT_ADDRESS, &args![this.byte_add(0x9c), index])
        .ptr::<()>();
    ni_pointer_get(e, element)
}

// Translated from 0043b4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The low seven bits of the dword at `this`.
pub fn fn_0043b4d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr()) & 0x7f
}

// Translated from 0043b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compiler-emitted helper (the decompiler names it
/// `std::locale::id::id`): stores into `out` (`008c71b0`) the value
/// [`fn_0043b540`] reads from the block [`fn_0043b560`] finds for `this`
/// (0 when there is none); returns `out`.
pub fn fn_0043b4f0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let holder = fn_0043b560(e, this);
    let value = if holder != 0 {
        fn_0043b540(e, Ptr::new(holder))
    } else {
        0
    };
    e.call(STORE_POINTER, &args![out, value]);
    out
}

// Translated from 0043b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The field at +8 (`0044ddc0`) of the object embedded at +0x14.
pub fn fn_0043b540(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FIELD_AT_8, &args![this.byte_add(0x14)]).u32()
}

// Translated from 0043b560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compiler-emitted helper (the decompiler names it after
/// `std::_Ref_count_obj<_ExceptionHolder>`): finds the block at the field of
/// `this` that `00620b80` returns; if there is one, runs `0043b5b0` on it,
/// takes `block + 0x10` (`00460140`) as the result and runs `0043b5f0`.
/// Returns 0 when there is no block.
pub fn fn_0043b560(e: &mut Engine, this: Ptr) -> u32 {
    let mut value = 0;
    let held: Ptr = e.call(REF_COUNT_OBJECT_POINTER, &args![this]).ptr();
    if !held.is_null() {
        fn_0043b5b0(e, held);
        value = e.call(HOLDER_VALUE, &args![held]).u32();
        fn_0043b5f0(e, held);
    }
    value
}

// Translated from 0043b5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Ref_count_obj<_ExceptionHolder>::_Destroy` (library): calls
/// `004534f0` on the object embedded at +0x6C with the argument 1 (which the
/// game pushes before `0043b5d0` and `004534f0` takes).
pub fn fn_0043b5b0(e: &mut Engine, this: Ptr) {
    let held = fn_0043b5d0(e, this);
    e.call(DELETE_OBJECT, &args![held, 1u32]);
}

// Translated from 0043b5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Ref_count_obj<_ExceptionHolder>::_Getptr` (library): the held
/// object, embedded at +0x6C.
pub fn fn_0043b5d0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x6c)
}

// Translated from 0043b5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00483710` on the object embedded at +0x6C ([`fn_0043b5d0`]).
pub fn fn_0043b5f0(e: &mut Engine, this: Ptr) {
    let held = fn_0043b5d0(e, this);
    e.call(HOLDER_DESTRUCT, &args![held]);
}

// Translated from 0043b610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::GetbhkCollisionObject` (Xbox PDB): the collision
/// object of `object`, cast to `bhkCollisionObject` (null when it is not
/// one).
pub fn bhk_collision_object_get_bhk_collision_object(e: &mut Engine, object: Ptr) -> Ptr {
    let collision_object: Ptr = e.call(COLLISION_OBJECT, &args![object]).ptr();
    e.call(
        DYNAMIC_CAST,
        &args![TYPE_DESCRIPTOR_BHK_COLLISION_OBJECT, collision_object],
    )
    .ptr()
}

// Translated from 0043b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `KFModel::KFModel` (Xbox PDB): copies `name`, builds the sequence from the
/// loaded `file` and, when that worked, loads its animation group
/// ([`fn_0043b7e0`]); otherwise logs that no sequence could be created.
/// Returns `this`.
pub fn kf_model_kf_model(e: &mut Engine, this: Ptr<KFModel>, name: u32, file: Ptr) -> Ptr<KFModel> {
    let sequence = sequence_pointer(this);
    let anim_group = anim_group_pointer(this);
    e.call(NI_POINTER_CONSTRUCT, &args![sequence, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![anim_group, 0u32]);
    e.set(this, KFModel::iRefCount, 0);
    e.set(this, KFModel::iManualRefCount, 0);
    e.call(NI_POINTER_ASSIGN, &args![anim_group, 0u32]);
    let copy = copy_name(e, name);
    e.set(this, KFModel::pFilename, copy);
    e.call(LOAD_KF_SEQUENCE, &args![file, 0u32, sequence]);
    if !ni_pointer_get(e, sequence).is_null() {
        fn_0043b7e0(e, this, name);
    } else {
        let filename = e.get(this, KFModel::pFilename);
        e.call(LOG, &args![NO_SEQUENCE_MESSAGE, filename]);
    }
    this
}

// Translated from 0043b750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `KFModel::~KFModel` (the map has no name for it): frees the file name and
/// releases the animation group and the sequence.
pub fn fn_0043b750(e: &mut Engine, this: Ptr<KFModel>) {
    let sequence = sequence_pointer(this);
    let anim_group = anim_group_pointer(this);
    let filename = e.get(this, KFModel::pFilename);
    e.call(MEMORY_FREE, &args![filename]);
    e.call(NI_POINTER_ASSIGN, &args![sequence, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![anim_group, 0u32]);
    e.call(NI_POINTER_DESTRUCT, &args![anim_group]);
    e.call(NI_POINTER_DESTRUCT, &args![sequence]);
}

// Translated from 0043b7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finishes a `KFModel` once its sequence exists: when the sequence's name
/// has an underscore, the sequence is first renamed to the part before it;
/// then the animation group is loaded from `name` (`LoadAnimGroup`), and, if
/// the sequence is of the class `011c7d74`, the group is stored in the
/// sequence (`0043baa0`); finally the sequence is named `name`.
///
/// The stack-cookie check at the end is not translated.
pub fn fn_0043b7e0(e: &mut Engine, this: Ptr<KFModel>, name: u32) {
    let sequence_slot = sequence_pointer(this);
    if ni_pointer_get(e, sequence_slot).is_null() {
        return;
    }
    let sequence = ni_pointer_get(e, sequence_slot);
    let name_slot = e.call(NAME_SLOT, &args![sequence]).ptr();
    let sequence_name = fn_0043b1b0(e, name_slot);
    if sequence_name != 0 {
        let underscore = fn_0043b9d0(e, sequence_name, 0x5f);
        if !underscore.is_null() {
            let prefix_length = underscore.addr().wrapping_sub(sequence_name);
            // `char buffer[268]` of the game's stack frame.
            e.with_stack(0x10c, |e, buffer| {
                e.call(MEMORY_COPY, &args![buffer, sequence_name, prefix_length]);
                e.mem.set_u8(buffer.addr().wrapping_add(prefix_length), 0);
                e.with_stack(4, |e, temporary| {
                    let prefix: Ptr<NiFixedString> = e
                        .call(FIXED_STRING_CONSTRUCT, &args![temporary, buffer])
                        .ptr();
                    let sequence = ni_pointer_get(e, sequence_slot);
                    fn_0043b9f0(e, sequence, prefix);
                    e.call(FIXED_STRING_DESTRUCT, &args![temporary]);
                });
            });
        }
    }

    let sequence = ni_pointer_get(e, sequence_slot);
    let group: Ptr = e.call(LOAD_ANIM_GROUP, &args![sequence, name]).ptr();
    e.call(NI_POINTER_ASSIGN, &args![anim_group_pointer(this), group]);
    let sequence = ni_pointer_get(e, sequence_slot);
    if fn_0043b300(e, TYPE_DESCRIPTOR_011C7D74, sequence) {
        let group = ni_pointer_get(e, anim_group_pointer(this));
        // `007fa950` takes no stack argument: the pushed group is the
        // argument of `0043baa0`.
        let sequence: Ptr = e.call(KF_MODEL_SEQUENCE, &args![this]).ptr();
        fn_0043baa0(e, sequence, group);
    }

    e.with_stack(4, |e, temporary| {
        let full_name: Ptr<NiFixedString> = e
            .call(FIXED_STRING_CONSTRUCT, &args![temporary, name])
            .ptr();
        let sequence = ni_pointer_get(e, sequence_slot);
        fn_0043b9f0(e, sequence, full_name);
        e.call(FIXED_STRING_DESTRUCT, &args![temporary]);
    });
}

// Translated from 0043b9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strchr` (library): the first occurrence of `character` in `text`.
pub fn fn_0043b9d0(e: &mut Engine, text: u32, character: i32) -> Ptr {
    e.call(STRCHR, &args![text, character]).ptr()
}

// Translated from 0043b9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence`: assigns `name` to the `NiFixedString` at +8 (the
/// sequence's name).
pub fn fn_0043b9f0(e: &mut Engine, this: Ptr, name: Ptr<NiFixedString>) -> Ptr<NiFixedString> {
    fn_0043ba10(e, this.byte_add(8).cast(), name)
}

// Translated from 0043ba10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFixedString::operator=` (library): when the handles differ, adds a
/// reference to the new string, releases the old one and stores the new
/// handle. Returns `this`.
pub fn fn_0043ba10(
    e: &mut Engine,
    this: Ptr<NiFixedString>,
    other: Ptr<NiFixedString>,
) -> Ptr<NiFixedString> {
    let handle = e.get(this, NiFixedString::m_kHandle);
    let new_handle = e.get(other, NiFixedString::m_kHandle);
    if handle != new_handle {
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), new_handle);
            fn_0043ba60(e, local);
            e.call(FIXED_STRING_RELEASE, &args![this]);
            let stored = e.mem.u32(local.addr());
            e.set(this, NiFixedString::m_kHandle, stored);
        });
    }
    this
}

// Translated from 0043ba60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFixedString` reference add (library): increments the pooled string's
/// reference count unless the handle is the empty string's.
pub fn fn_0043ba60(e: &mut Engine, handle: Ptr) {
    if e.mem.u32(handle.addr()) != e.global::<u32>(EMPTY_FIXED_STRING) {
        let counter = e.call(FIXED_STRING_REFCOUNT, &args![handle]).u32();
        e.call(INTERLOCKED_INCREMENT, &args![counter]);
    }
}

// Translated from 0043baa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence`: assigns `group` to the `NiPointer` at +0x74 (the
/// sequence's animation group). Returns that `NiPointer`.
pub fn fn_0043baa0(e: &mut Engine, this: Ptr, group: Ptr) -> Ptr {
    e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x74), group])
        .ptr()
}

/// The address of `QueuedTexture::spTexture`, which the game passes to the
/// `NiPointer` functions.
pub(crate) fn texture_pointer(texture: Ptr<QueuedTexture>) -> Ptr {
    texture.byte_add(QueuedTexture::spTexture.off)
}

/// The address of `QueuedModel::spModel`.
pub(crate) fn model_pointer(model: Ptr<QueuedModel>) -> Ptr {
    model.byte_add(QueuedModel::spModel.off)
}

/// `QueuedFileEntry::pFileEntry` through the game's getter (`0055b980`).
pub(crate) fn queued_file_entry(e: &mut Engine, this: Ptr<QueuedTexture>) -> u32 {
    e.call(QUEUED_FILE_ENTRY_GET_FILE_ENTRY, &args![this]).u32()
}

/// `QueuedFileEntry::pFileName` through the game's getter (`0045cd60`).
pub(crate) fn queued_file_name(e: &mut Engine, this: Ptr<QueuedTexture>) -> u32 {
    e.call(QUEUED_FILE_ENTRY_GET_FILE_NAME, &args![this]).u32()
}

/// The address of a queued texture's `cFlags` byte, which the game passes to
/// the flag helpers.
pub(crate) fn texture_flags_pointer(texture: Ptr<QueuedTexture>) -> Ptr {
    texture.byte_add(QueuedTexture::cFlags.off)
}

// Translated from 0043bac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedChildren` method (no name in the engine map): decrements
/// `iNumChildrenFinished` (interlocked) when it is positive.
pub fn fn_0043bac0(e: &mut Engine, this: Ptr<QueuedChildren>) {
    // The comparison is signed although the Xbox PDB declares a `u32`.
    if e.get(this, QueuedChildren::iNumChildrenFinished) as i32 > 0 {
        let counter = this.byte_add(QueuedChildren::iNumChildrenFinished.off);
        e.call(INTERLOCKED_DECREMENT, &args![counter]);
    }
}

// Translated from 0043baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedFile::LoadedFile` (no name in the engine map): keeps `file`,
/// copies `name`, adds the object to the map of loaded files (the result
/// becomes `bInLoadedFileMap`) and starts it unused with no references.
/// Returns `this`.
pub fn fn_0043baf0(e: &mut Engine, this: Ptr<LoadedFile>, name: u32, file: Ptr) -> Ptr<LoadedFile> {
    e.set(this, LoadedFile::pFile, file);
    let copy = copy_name(e, name);
    e.set(this, LoadedFile::pFileName, copy);
    let owner = e.global::<u32>(FILE_MAP_OWNER);
    let added = e
        .call(FILE_MAP_ADD_LOADED_FILE, &args![owner, copy, this])
        .bool();
    e.set(this, LoadedFile::bInLoadedFileMap, added);
    e.set(this, LoadedFile::bFileUsed, false);
    e.set(this, LoadedFile::iRefCount, 0);
    this
}

// Translated from 0043bb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedFile::~LoadedFile` (Xbox PDB): complains (log) about a file that
/// was added to the map but never used, removes it from the map, reports a
/// non-zero reference count to the message sink, frees the name and deletes
/// the file.
///
/// The stack-cookie check is not translated.
pub fn loaded_file_destructor(e: &mut Engine, this: Ptr<LoadedFile>) {
    let name = e.get(this, LoadedFile::pFileName);
    let in_map = e.get(this, LoadedFile::bInLoadedFileMap);
    if !e.get(this, LoadedFile::bFileUsed) && in_map {
        e.call(LOG, &args![LOADED_FILE_UNUSED_MESSAGE, name, this]);
    }
    if in_map {
        let owner = e.global::<u32>(FILE_MAP_OWNER);
        e.call(FILE_MAP_REMOVE_LOADED_FILE, &args![owner, name]);
    }
    let references = e.get(this, LoadedFile::iRefCount);
    if references != 0 {
        // `char text[268]` of the game's stack frame.
        e.with_stack(0x10c, |e, text| {
            e.call(
                SPRINTF,
                &args![text, LOADED_FILE_REFERENCES_MESSAGE, name, this, references],
            );
            if fn_0043bce0(e) {
                let sink = fn_0043bd00(e);
                // Slot 0x14: report `(severity 4, text, 0, 0, 0)`.
                e.vcall(sink, 0x14, &args![4u32, text, 0u32, 0u32, 0u32]);
            }
        });
    }
    let name = e.get(this, LoadedFile::pFileName);
    e.call(MEMORY_FREE, &args![name]);
    let file = e.get(this, LoadedFile::pFile);
    if !file.is_null() {
        // Slot 0: the scalar deleting destructor, with the delete flag.
        e.vcall(file.addr(), 0, &args![1u32]);
    }
}

// Translated from 0043bce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the message sink at `011f6388` exists.
pub fn fn_0043bce0(e: &mut Engine) -> bool {
    e.global::<u32>(MESSAGE_SINK) != 0
}

// Translated from 0043bd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The message sink at `011f6388`.
pub fn fn_0043bd00(e: &mut Engine) -> u32 {
    e.global::<u32>(MESSAGE_SINK)
}

// Translated from 0043bd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `QueuedTexture` constructor (no name in the engine map) that loads by
/// file name: the base constructor with `context`, the texture's virtual
/// table, a null texture and no flags, then the path `name` normalized into
/// a 260-character buffer becomes `pFileName` and its file entry is looked
/// up (flag 1). Returns `this`.
///
/// The stack-cookie check and the exception-unwinding frame are not
/// translated.
pub fn fn_0043bd10(
    e: &mut Engine,
    this: Ptr<QueuedTexture>,
    name: u32,
    context: u32,
) -> Ptr<QueuedTexture> {
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_TEXTURE_VTABLE);
    let texture = texture_pointer(this);
    e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
    e.set(this, QueuedTexture::cFlags, 0);
    // `char path[260]` of the game's stack frame (268 bytes with padding).
    e.with_stack(0x10c, |e, path| {
        e.call(NORMALIZE_PATH, &args![name, path, 0x104u32]);
        e.call(QUEUED_FILE_ENTRY_SET_FILE_NAME, &args![this, path]);
    });
    e.call(QUEUED_FILE_ENTRY_FIND_FILE_ENTRY, &args![this, 1u32]);
    this
}

// Translated from 0043bde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFile::NotifyChildFinished` (Xbox PDB): counts one more finished
/// child ([`fn_0043be10`] on `pChildren`), then asks the file whether it is
/// finished (slot `0x28`, `CheckFinished`). The stack argument (`RET 4`,
/// the finished child) is not used.
pub fn queued_file_notify_child_finished(e: &mut Engine, this: Ptr<QueuedFile>, _child: Ptr) {
    let children = e.get(this, QueuedFile::pChildren);
    fn_0043be10(e, children.cast());
    e.vcall(this.addr(), 0x28, &args![]);
}

// Translated from 0043be10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedChildren` method (no name in the engine map): increments
/// `iNumChildrenFinished` (interlocked).
pub fn fn_0043be10(e: &mut Engine, this: Ptr<QueuedChildren>) {
    let counter = this.byte_add(QueuedChildren::iNumChildrenFinished.off);
    e.call(INTERLOCKED_INCREMENT, &args![counter]);
}

// Translated from 0043be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTexture::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_0043bf80`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn queued_texture_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<QueuedTexture>,
    flags: u32,
) -> Ptr<QueuedTexture> {
    fn_0043bf80(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043be60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `QueuedTexture` constructor (no name in the engine map) that is given
/// the file entry: like [`fn_0043bd10`] but it stores `file_entry` as
/// `pFileEntry` instead of resolving a name. Returns `this`.
pub fn fn_0043be60(
    e: &mut Engine,
    this: Ptr<QueuedTexture>,
    file_entry: Ptr,
    context: u32,
) -> Ptr<QueuedTexture> {
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_TEXTURE_VTABLE);
    let texture = texture_pointer(this);
    e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
    e.set(this, QueuedTexture::cFlags, 0);
    e.call(QUEUED_FILE_ENTRY_SET_FILE_ENTRY, &args![this, file_entry]);
    this
}

// Translated from 0043bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `QueuedTexture` constructor (no name in the engine map) that is given
/// the finished texture: like [`fn_0043be60`] but it stores `texture` in
/// `spTexture` and runs `00449150`, which puts the task in state 5 (done).
/// Returns `this`.
pub fn fn_0043bef0(
    e: &mut Engine,
    this: Ptr<QueuedTexture>,
    texture: Ptr,
    context: u32,
) -> Ptr<QueuedTexture> {
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_TEXTURE_VTABLE);
    let slot = texture_pointer(this);
    e.call(NI_POINTER_CONSTRUCT, &args![slot, 0u32]);
    e.set(this, QueuedTexture::cFlags, 0);
    e.call(NI_POINTER_ASSIGN, &args![slot, texture]);
    e.call(TASK_SET_DONE, &args![this]);
    this
}

// Translated from 0043bf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTexture` destructor (no name in the engine map): restores the
/// texture's virtual table, releases the texture and runs the base
/// destructor.
pub fn fn_0043bf80(e: &mut Engine, this: Ptr<QueuedTexture>) {
    e.mem.set_u32(this.addr(), QUEUED_TEXTURE_VTABLE);
    let texture = texture_pointer(this);
    e.call(NI_POINTER_DESTRUCT, &args![texture]);
    e.call(QUEUED_FILE_ENTRY_DESTRUCT, &args![this]);
}

// Translated from 0043bfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTexture::GetFileIndex` (Xbox PDB): 0 when the texture has no file
/// entry or its archive is not found, otherwise the archive's file index
/// ([`fn_0043c030`]) plus 4.
pub fn queued_texture_get_file_index(e: &mut Engine, this: Ptr<QueuedTexture>) -> u32 {
    if queued_file_entry(e, this) != 0 {
        let file_entry = queued_file_entry(e, this);
        let archive = e
            .call(FILE_ENTRY_FIND_ARCHIVE, &args![file_entry, 2u32])
            .u32();
        if archive != 0 {
            return fn_0043c030(e, Ptr::new(archive)).wrapping_add(4);
        }
    }
    0
}

// Translated from 0043c030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at +0x1C8 of an archive (what `GetFileIndex` adds 4 to).
pub fn fn_0043c030(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x1c8)
}

// Translated from 0043c050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTexture::QueueMe` (Xbox PDB): with a file entry, adds the texture
/// to the map of queued textures by it and records the result as flag bit 4;
/// when the thread flag is set, sets flag bit 2; then hands the task to the
/// task queue (slot `0x48`).
pub fn queued_texture_queue_me(e: &mut Engine, this: Ptr<QueuedTexture>) {
    if queued_file_entry(e, this) != 0 {
        let file_entry = queued_file_entry(e, this);
        let owner = e.global::<u32>(FILE_MAP_OWNER);
        let added = e
            .call(FILE_MAP_ADD_QUEUED_TEXTURE, &args![owner, file_entry, this])
            .u8();
        fn_0043c100(e, this, added);
    }
    // 0043c093: ECX is the model loader (`011c3b3c`).
    let loader = e.global::<u32>(FILE_MAP_OWNER);
    if fn_0043c130(e, Ptr::new(loader)) != 0 {
        fn_0043c0d0(e, this, 1);
    }
    let queue = e.global::<u32>(TASK_QUEUE);
    e.vcall(queue, 0x48, &args![this]);
}

// Translated from 0043c0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 2 of the texture's `cFlags`.
pub fn fn_0043c0d0(e: &mut Engine, this: Ptr<QueuedTexture>, flag: u8) {
    let flags = texture_flags_pointer(this);
    e.call(SET_FLAG_BIT_2, &args![flag as u32, flags]);
}

// Translated from 0043c100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 4 of the texture's `cFlags`.
pub fn fn_0043c100(e: &mut Engine, this: Ptr<QueuedTexture>, flag: u8) {
    let flags = texture_flags_pointer(this);
    e.call(SET_FLAG_BIT_4, &args![flag as u32, flags]);
}

// Translated from 0043c130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x25C of the thread's TLS block. (The code is a method of
/// the object at `011c3b3c` but does not use `this`.)
pub fn fn_0043c130(e: &mut Engine, _this: Ptr) -> u8 {
    let address = e.tls() + TLS_QUEUED_FLAG;
    e.mem.u8(address)
}

// Translated from 0043c150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTexture::Run` (Xbox PDB): finds the texture, or loads it from its
/// file.
///
/// Inside the memory context of the task (`eContext`), the texture comes
/// from the palette by file entry, or by file name when there is none. If it
/// is there already, flag bit 1 is set, and a texture whose surface format
/// byte ([`fn_0043c430`]) is 3 and whose flag bit 2 is clear gets its
/// virtual function `0x8c` called. Otherwise the file is opened
/// (`QueuedFileEntry::GetFile`); with it, and with the TLS word set to 1 for
/// flag bit 2, the texture is created from it (`00a61040` when the file name
/// contains `_e.dd`, else `00a5fe30`) and stored in `spTexture`; without it
/// the failure is logged with the file entry's offset and size, or with the
/// file name.
///
/// The exception-unwinding frame is not translated.
pub fn queued_texture_run(e: &mut Engine, this: Ptr<QueuedTexture>) {
    let context = e.get(this, QueuedTexture::eContext);
    // The scope guard of the game's stack frame (4 bytes).
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![guard, context, 1u32, MODEL_LOADER_SOURCE, RUN_SOURCE_LINE],
        );
        queued_texture_run_in_context(e, this);
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

/// The body of [`queued_texture_run`] inside its memory context.
pub(crate) fn queued_texture_run_in_context(e: &mut Engine, this: Ptr<QueuedTexture>) {
    let slot = texture_pointer(this);
    if queued_file_entry(e, this) != 0 {
        let file_entry = queued_file_entry(e, this);
        bs_texture_palette_get_texture(e, file_entry, slot);
    } else if queued_file_name(e, this) != 0 {
        let name = queued_file_name(e, this);
        e.call(TEXTURE_PALETTE_GET_TEXTURE_BY_NAME, &args![name, slot]);
    }

    if !ni_pointer_get(e, slot).is_null() {
        e.call(QUEUED_TEXTURE_SET_FLAG_1, &args![this, 1u32]);
        let texture = ni_pointer_get(e, slot);
        if fn_0043c430(e, texture) == 3 && !fn_0043c3d0(e, this) {
            let texture = ni_pointer_get(e, slot);
            e.vcall(texture.addr(), 0x8c, &args![]);
        }
        return;
    }

    let file: Ptr = e
        .call(QUEUED_FILE_ENTRY_GET_FILE, &args![this, 1u32, 2u32])
        .ptr();
    if !file.is_null() {
        // Slot 0x18 of the file: its name.
        let name = e.vcall(file.addr(), 0x18, &args![]).u32();
        let saved = fn_0043c410(e);
        if fn_0043c3d0(e, this) {
            fn_0043c3f0(e, 1);
        }
        if e.call(STRSTR, &args![name, ENVIRONMENT_MAP_SUFFIX]).u32() != 0 {
            let setting = fn_0043c4b0(e);
            // The `NiFixedString` temporary of the game's stack frame.
            e.with_stack(4, |e, local| {
                let fixed_name = e.call(FIXED_STRING_CONSTRUCT, &args![local, name]).u32();
                let texture = e
                    .call(
                        CREATE_TEXTURE_FROM_FIXED_NAME,
                        &args![fixed_name, file, setting, TEXTURE_FORMAT_PREFERENCES],
                    )
                    .u32();
                e.call(NI_POINTER_ASSIGN, &args![slot, texture]);
                e.call(FIXED_STRING_DESTRUCT, &args![local]);
            });
        } else {
            let texture = e
                .call(
                    CREATE_TEXTURE_FROM_FILE,
                    &args![file, name, TEXTURE_FORMAT_PREFERENCES, 1u32],
                )
                .u32();
            e.call(NI_POINTER_ASSIGN, &args![slot, texture]);
        }
        fn_0043c3f0(e, saved);
    } else if queued_file_entry(e, this) != 0 {
        let file_entry = queued_file_entry(e, this);
        let size = e.call(FILE_ENTRY_GET_SIZE, &args![file_entry]).u32();
        let file_entry = queued_file_entry(e, this);
        let offset = fn_0043c3b0(e, Ptr::new(file_entry));
        e.call(LOG, &args![NO_FILE_FOR_TEXTURE_ENTRY_MESSAGE, offset, size]);
    } else if queued_file_name(e, this) != 0 {
        let name = queued_file_name(e, this);
        e.call(LOG, &args![NO_FILE_FOR_TEXTURE_NAME_MESSAGE, name]);
    }
}

// Translated from 0043c3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFileEntry::iOffset` with the top bit masked off (the dword at +0xC).
pub fn fn_0043c3b0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xc) & 0x7fff_ffff
}

// Translated from 0043c3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the texture's `cFlags` is set.
pub fn fn_0043c3d0(e: &mut Engine, this: Ptr<QueuedTexture>) -> bool {
    let flags = e.get(this, QueuedTexture::cFlags);
    e.call(TEST_FLAG_BIT_2, &args![flags as u32]).bool()
}

// Translated from 0043c3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at +0x29C of the thread's TLS block.
pub fn fn_0043c3f0(e: &mut Engine, value: u32) {
    let address = e.tls() + TLS_QUEUED_VALUE;
    e.mem.set_u32(address, value);
}

// Translated from 0043c410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x29C of the thread's TLS block.
pub fn fn_0043c410(e: &mut Engine) -> u32 {
    let address = e.tls() + TLS_QUEUED_VALUE;
    e.mem.u32(address)
}

// Translated from 0043c430 (decompiled, FalloutNV.exe 1.4.0.525)
/// A texture's surface format byte: 0 for a texture without a surface,
/// otherwise the byte at +3 of the descriptor at +0x58 of its surface
/// (`0059bb30`, `this + 0x24`).
pub fn fn_0043c430(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(TEXTURE_GET_SURFACE, &args![this]).u32() != 0 {
        let surface = e.call(TEXTURE_GET_SURFACE, &args![this]).ptr();
        let descriptor = fn_0043c490(e, surface);
        fn_0043c470(e, descriptor) as u32
    } else {
        0
    }
}

// Translated from 0043c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +3 of `this`.
pub fn fn_0043c470(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 3)
}

// Translated from 0043c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address `this + 0x58`.
pub fn fn_0043c490(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x58)
}

// Translated from 0043c4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011f4748` (a default texture setting).
pub fn fn_0043c4b0(e: &mut Engine) -> u32 {
    e.global::<u32>(DEFAULT_TEXTURE_SETTING)
}

// Translated from 0043c4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTexturePalette::GetTexture` (Xbox PDB): clears the `NiPointer` at
/// `out` and looks the texture of `file_entry` up in the texture map into
/// it.
pub fn bs_texture_palette_get_texture(e: &mut Engine, file_entry: u32, out: Ptr) {
    e.call(NI_POINTER_ASSIGN, &args![out, 0u32]);
    let map = e.global::<u32>(TEXTURE_MAP);
    fn_0043c4f0(e, Ptr::new(map), file_entry, out);
}

// Translated from 0043c4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTexturePalette` method (no name in the engine map): looks
/// `file_entry` up in the hash bucket [`fn_0043c530`] gives for it in the
/// texture map `this` (`00a61a60`), storing the texture found into the
/// `NiPointer` at `out`. Returns whether it was found.
pub fn fn_0043c4f0(e: &mut Engine, this: Ptr, file_entry: u32, out: Ptr) -> bool {
    let bucket = fn_0043c530(e, file_entry);
    e.call(TEXTURE_MAP_FIND, &args![this, bucket, file_entry, out])
        .bool()
}

// Translated from 0043c530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The hash bucket of a key: `(key >> 4) % 1001`.
pub fn fn_0043c530(_e: &mut Engine, key: u32) -> u32 {
    (key >> 4) % 0x3e9
}

// Translated from 0043c550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTexture::Finish` (Xbox PDB): unless flag bit 1 is set, a loaded
/// texture goes into the palette (`SetTexture`, by file entry, or without
/// one when the texture has only a file name). If the texture is in the map
/// of queued textures (flag bit 4), it is taken out and the flag cleared.
/// Finally asks the task whether it is finished ([`fn_0043c610`]).
pub fn queued_texture_finish(e: &mut Engine, this: Ptr<QueuedTexture>) {
    let flag_1 = e.call(QUEUED_TEXTURE_TEST_FLAG_1, &args![this]).bool();
    let slot = texture_pointer(this);
    if !flag_1 && !ni_pointer_get(e, slot).is_null() {
        if queued_file_entry(e, this) != 0 {
            let file_entry = queued_file_entry(e, this);
            let texture = ni_pointer_get(e, slot);
            e.call(TEXTURE_PALETTE_SET_TEXTURE, &args![texture, file_entry]);
        } else if queued_file_name(e, this) != 0 {
            let texture = ni_pointer_get(e, slot);
            e.call(TEXTURE_PALETTE_SET_TEXTURE, &args![texture, 0u32]);
        }
    }
    leave_texture_map(e, this);
    fn_0043c610(e, this);
}

/// The common end of `Finish` and `Cancel`: a texture with a file entry that
/// is in the map of queued textures (flag bit 4) is removed from it.
pub(crate) fn leave_texture_map(e: &mut Engine, this: Ptr<QueuedTexture>) {
    if queued_file_entry(e, this) != 0 && fn_0043c630(e, this) {
        fn_0043c100(e, this, 0);
        let file_entry = queued_file_entry(e, this);
        let owner = e.global::<u32>(FILE_MAP_OWNER);
        e.call(FILE_MAP_REMOVE_QUEUED_TEXTURE, &args![owner, file_entry]);
    }
}

// Translated from 0043c610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the task's virtual function `0x28` (`CheckFinished`).
pub fn fn_0043c610(e: &mut Engine, this: Ptr<QueuedTexture>) {
    e.vcall(this.addr(), 0x28, &args![]);
}

// Translated from 0043c630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 4 of the texture's `cFlags` is set.
pub fn fn_0043c630(e: &mut Engine, this: Ptr<QueuedTexture>) -> bool {
    let flags = e.get(this, QueuedTexture::cFlags);
    e.call(TEST_FLAG_BIT_4, &args![flags as u32]).bool()
}

// Translated from 0043c650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTexture::Cancel` (Xbox PDB): `QueuedFile::Cancel` with its two
/// arguments, then takes the texture out of the map of queued textures as
/// `Finish` does.
pub fn queued_texture_cancel(e: &mut Engine, this: Ptr<QueuedTexture>, first: u32, second: u32) {
    e.call(QUEUED_FILE_CANCEL, &args![this, first, second]);
    leave_texture_map(e, this);
}

// Translated from 0043c6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTexture::GetDescription` (Xbox PDB):
/// `QueuedFileEntry::GetDescription` with the kind word `"texture"`;
/// returns its result (always true).
pub fn queued_texture_get_description(
    e: &mut Engine,
    this: Ptr<QueuedTexture>,
    buffer: u32,
    size: u32,
) -> bool {
    e.call(
        QUEUED_FILE_ENTRY_GET_DESCRIPTION,
        &args![this, buffer, size, TEXTURE_WORD],
    )
    .bool()
}

// Translated from 0043c6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel` constructor (no name in the engine map): the base
/// constructor with `context`, the model's virtual table, no model, no
/// `TESModel`, `lod_fade_mult`, no flags; then the file name is copied
/// (`name`), the file entry is looked up (flag 0), and the constructor's two
/// flag arguments set bits 1 and 2 of `cFlags`. The overridden visual
/// distance starts at 0. Returns `this`.
pub fn fn_0043c6e0(
    e: &mut Engine,
    this: Ptr<QueuedModel>,
    name: u32,
    context: u32,
    lod_fade_mult: u32,
    first_flag: u8,
    second_flag: u8,
) -> Ptr<QueuedModel> {
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_MODEL_VTABLE);
    let model = model_pointer(this);
    e.call(NI_POINTER_CONSTRUCT, &args![model, 0u32]);
    e.set(this, QueuedModel::pTESModel, Ptr::NULL);
    e.set(this, QueuedModel::eLODFadeMult, lod_fade_mult);
    e.set(this, QueuedModel::cFlags, 0);
    e.call(QUEUED_FILE_ENTRY_SET_FILE_NAME, &args![this, name]);
    e.call(QUEUED_FILE_ENTRY_FIND_FILE_ENTRY, &args![this, 0u32]);
    fn_0043c7a0(e, this, first_flag);
    fn_0043c7d0(e, this, second_flag);
    e.set(this, QueuedModel::mfOverriddenVisualDistance, 0.0);
    this
}

// Translated from 0043c7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 1 of the model's `cFlags`.
pub fn fn_0043c7a0(e: &mut Engine, this: Ptr<QueuedModel>, flag: u8) {
    let flags = this.byte_add(QueuedModel::cFlags.off);
    e.call(SET_FLAG_BIT_1, &args![flag as u32, flags]);
}

// Translated from 0043c7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 2 of the model's `cFlags`.
pub fn fn_0043c7d0(e: &mut Engine, this: Ptr<QueuedModel>, flag: u8) {
    let flags = this.byte_add(QueuedModel::cFlags.off);
    e.call(SET_FLAG_BIT_2, &args![flag as u32, flags]);
}

// Translated from 0043c800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_0043c830`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn queued_model_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<QueuedModel>,
    flags: u32,
) -> Ptr<QueuedModel> {
    fn_0043c830(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043c830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel` destructor (the engine map names the folded body after a
/// cancellation-token callback): releases the model pointer and runs the
/// base destructor. It does not restore the virtual table.
pub fn fn_0043c830(e: &mut Engine, this: Ptr<QueuedModel>) {
    let model = model_pointer(this);
    e.call(MODEL_POINTER_RELEASE, &args![model]);
    e.call(QUEUED_FILE_ENTRY_DESTRUCT, &args![this]);
}

/// `QueuedFileEntry::pFileName` of a queued model (the getter `0045cd60`).
pub(crate) fn model_file_name(e: &mut Engine, this: Ptr<QueuedModel>) -> u32 {
    e.call(QUEUED_FILE_ENTRY_GET_FILE_NAME, &args![this]).u32()
}

/// The address of a queued model's `cFlags` byte, which the game passes to
/// the flag helpers.
pub(crate) fn model_flags_pointer(model: Ptr<QueuedModel>) -> Ptr {
    model.byte_add(QueuedModel::cFlags.off)
}

// Translated from 0043c890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel` constructor taking a `TESModel` (no name in the engine map;
/// Xbox PDB `QueuedModel::QueuedModel(TESModel*, MEM_CONTEXT, ENUM_LOD_MULT,
/// bool, bool)`): the base constructor with `context`, the model's virtual
/// table, no model, `tes_model`, `lod_fade_mult`, no flags; the file name is
/// the model name (virtual function `0x14` of the `TESModel`), the file
/// entry is looked up (flag 0), and the last two arguments set bits 1 and 2
/// of `cFlags`. The overridden visual distance starts at 0. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043c890(
    e: &mut Engine,
    this: Ptr<QueuedModel>,
    tes_model: Ptr,
    context: u32,
    lod_fade_mult: u32,
    first_flag: u8,
    second_flag: u8,
) -> Ptr<QueuedModel> {
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_MODEL_VTABLE);
    let model = model_pointer(this);
    e.call(NI_POINTER_CONSTRUCT, &args![model, 0u32]);
    e.set(this, QueuedModel::pTESModel, tes_model);
    e.set(this, QueuedModel::eLODFadeMult, lod_fade_mult);
    e.set(this, QueuedModel::cFlags, 0);
    let name = e.vcall(tes_model.addr(), 0x14, &args![]).u32();
    e.call(QUEUED_FILE_ENTRY_SET_FILE_NAME, &args![this, name]);
    e.call(QUEUED_FILE_ENTRY_FIND_FILE_ENTRY, &args![this, 0u32]);
    fn_0043c7a0(e, this, first_flag);
    fn_0043c7d0(e, this, second_flag);
    e.set(this, QueuedModel::mfOverriddenVisualDistance, 0.0);
    this
}

// Translated from 0043c960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel` constructor taking a loaded model (no name in the engine
/// map): the base constructor with `context`, the model's virtual table, no
/// `TESModel`, no fade multiplier, no flags, then `model` is stored in
/// `spModel` and the task is put into state done. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043c960(
    e: &mut Engine,
    this: Ptr<QueuedModel>,
    model: Ptr,
    context: u32,
) -> Ptr<QueuedModel> {
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_MODEL_VTABLE);
    let slot = model_pointer(this);
    e.call(NI_POINTER_CONSTRUCT, &args![slot, 0u32]);
    e.set(this, QueuedModel::pTESModel, Ptr::NULL);
    e.set(this, QueuedModel::eLODFadeMult, 0);
    e.set(this, QueuedModel::cFlags, 0);
    e.call(MODEL_POINTER_ASSIGN, &args![slot, model]);
    e.call(TASK_SET_DONE, &args![this]);
    e.set(this, QueuedModel::mfOverriddenVisualDistance, 0.0);
    this
}

// Translated from 0043ca10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel::GenerateKey` (Xbox PDB): `QueuedFileEntry::GenerateKey`.
pub fn queued_model_generate_key(e: &mut Engine, this: Ptr<QueuedModel>) {
    e.call(QUEUED_FILE_ENTRY_GENERATE_KEY, &args![this]);
}

// Translated from 0043ca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel::CheckFinished` (Xbox PDB): a model whose bit 4 is not set
/// takes the plain `QueuedFile::CheckFinished`. With bit 4 set it does
/// nothing while the children are not all finished
/// ([`fn_0043caa0`]); when they are, a task still in state 0 is handed to
/// the task queue (slot `0x48`), any other goes through
/// `QueuedFile::CheckFinished`.
pub fn queued_model_check_finished(e: &mut Engine, this: Ptr<QueuedModel>) {
    if !fn_0043caf0(e, this) {
        e.call(QUEUED_FILE_CHECK_FINISHED, &args![this]);
    } else if fn_0043caa0(e, this) {
        if e.call(STATE_IS_ZERO, &args![this]).bool() {
            let queue = e.global::<u32>(TASK_QUEUE);
            e.vcall(queue, 0x48, &args![this]);
        } else {
            e.call(QUEUED_FILE_CHECK_FINISHED, &args![this]);
        }
    }
}

// Translated from 0043caa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all children are finished: true without a `pChildren` object,
/// otherwise the child count (dword at +8) equals the finished count (dword
/// at +0x10).
pub fn fn_0043caa0(e: &mut Engine, this: Ptr<QueuedModel>) -> bool {
    let children = e.get(this, QueuedModel::pChildren);
    if children.is_null() {
        return true;
    }
    let total = e.call(FIELD_AT_8, &args![children]).u32();
    let finished = e.call(FINISHED_COUNT_GETTER, &args![children]).u32();
    total == finished
}

// Translated from 0043caf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 4 of the model's `cFlags` is set.
pub fn fn_0043caf0(e: &mut Engine, this: Ptr<QueuedModel>) -> bool {
    let flags = e.get(this, QueuedModel::cFlags);
    e.call(TEST_FLAG_BIT_4, &args![flags as u32]).bool()
}

// Translated from 0043cb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel::QueueMe` (Xbox PDB): calls the virtual function `0x30`
/// (`QueueMe_ov2`) with the texture block of the `TESModel` (0 without one).
pub fn queued_model_queue_me(e: &mut Engine, this: Ptr<QueuedModel>) {
    let mut block = 0;
    let tes_model = e.get(this, QueuedModel::pTESModel);
    if !tes_model.is_null() {
        block = e.call(TES_MODEL_TEXTURE_BLOCK, &args![tes_model]).u32();
    }
    e.vcall(this.addr(), 0x30, &args![block]);
}

// Translated from 0043cb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel::QueueMe_ov2` (Xbox PDB): queues the textures of the model.
/// With a texture block (`block`: a count byte, then a pointer to that many
/// texture entries) every non-null entry goes to the loader's
/// `QueueTexture_ov2` with this model's key and as the parent. Then every
/// texture set of the `TESModel`'s list (virtual function `0x1c` of the
/// `TESModel`) that has an item is queued with `QueueTextureSet`. Last, bit
/// 4 of `cFlags` is set and the virtual function `0x28` (`CheckFinished`) is
/// called.
pub fn queued_model_queue_me_ov2(e: &mut Engine, this: Ptr<QueuedModel>, block: u32) {
    if block != 0 {
        let count = e.call(BYTE_AT_0, &args![block]).u8() as u32;
        if count != 0 {
            let textures = e.call(FIELD_AT_4, &args![block]).u32();
            for index in 0..count {
                let texture = e.mem.u32(textures + index * 4);
                if texture != 0 {
                    let key = fn_0043cc60(e, this.cast());
                    let owner = e.global::<u32>(FILE_MAP_OWNER);
                    e.call(
                        MODEL_LOADER_QUEUE_TEXTURE,
                        &args![owner, texture, key as u32, this],
                    );
                }
            }
        }
    }
    let tes_model = e.get(this, QueuedModel::pTESModel);
    if !tes_model.is_null() {
        let list = e.vcall(tes_model.addr(), 0x1c, &args![]).u32();
        if list != 0 {
            let mut node = e.call(LIST_FIRST_NODE, &args![list]).u32();
            while node != 0 {
                let item_slot = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
                let texture_set = e.mem.u32(item_slot);
                if texture_set != 0 && e.mem.u32(texture_set) != 0 {
                    let key = fn_0043cc60(e, this.cast());
                    let target = e.mem.u32(texture_set);
                    e.call(QUEUE_TEXTURE_SET, &args![target, key as u32, this, 0u32]);
                }
                node = e.call(FIELD_AT_4, &args![node]).u32();
            }
        }
    }
    fn_0043ccc0(e, this, 1);
    e.vcall(this.addr(), 0x28, &args![]);
}

// Translated from 0043cc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The priority byte of a task's key: the low byte of [`fn_0043cc80`].
pub fn fn_0043cc60(e: &mut Engine, this: Ptr<IOTask>) -> u8 {
    fn_0043cc80(e, this) as u8
}

// Translated from 0043cc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The task's `Key` shifted right by 16 ([`fn_0043cca0`] on `this + 0x10`).
pub fn fn_0043cc80(e: &mut Engine, this: Ptr<IOTask>) -> u64 {
    fn_0043cca0(e, this.byte_add(IOTask::Key.off))
}

// Translated from 0043cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__cdecl(pointer)`: the 64-bit value at `value` shifted right by 16 with
/// the sign kept (`__allshr`), returned in `EDX:EAX`.
pub fn fn_0043cca0(e: &mut Engine, value: Ptr) -> u64 {
    ((e.mem.u64(value.addr()) as i64) >> 16) as u64
}

// Translated from 0043ccc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 4 of the model's `cFlags`.
pub fn fn_0043ccc0(e: &mut Engine, this: Ptr<QueuedModel>, flag: u8) {
    let flags = model_flags_pointer(this);
    e.call(SET_FLAG_BIT_4, &args![flag as u32, flags]);
}

// Translated from 0043ccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel::Run` (Xbox PDB): inside the memory context of the task, a
/// model with a file name is loaded: if the loader already has a model of
/// that name it is taken (and bit 8 set); otherwise the file is opened
/// (`QueuedFileEntry::GetFile`) and read into a `BSStream` on the stack, and
/// a `Model` built from it is stored in `spModel` (when the file could be
/// read, or bit 0x10 allows a model without one); a failure is logged. The
/// name is published in `011f94c0` while the stream loads.
///
/// The exception-unwinding frame and the stack-cookie check are not
/// translated; the `BSStream` of the game's stack frame is a heap block here.
pub fn queued_model_run(e: &mut Engine, this: Ptr<QueuedModel>) {
    let context = e.get(this, QueuedModel::eContext);
    // The scope guard of the game's stack frame (4 bytes).
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![
                guard,
                context,
                1u32,
                MODEL_LOADER_SOURCE,
                MODEL_RUN_SOURCE_LINE
            ],
        );
        queued_model_run_in_context(e, this);
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

/// The body of [`queued_model_run`] inside its memory context.
pub(crate) fn queued_model_run_in_context(e: &mut Engine, this: Ptr<QueuedModel>) {
    let name = model_file_name(e, this);
    if e.call(STRLEN, &args![name]).u32() == 0 {
        return;
    }
    let slot = model_pointer(this);
    let name = model_file_name(e, this);
    let owner = e.global::<u32>(FILE_MAP_OWNER);
    e.call(MODEL_LOADER_FIND_MODEL, &args![owner, name, slot]);
    if !ni_pointer_get(e, slot).is_null() {
        fn_0043cf60(e, this, 1);
        return;
    }

    let file: Ptr = e
        .call(QUEUED_FILE_ENTRY_GET_FILE, &args![this, 0u32, 1u32])
        .ptr();
    e.with_stack(BSStream::SIZE, |e, stream| {
        fn_0043cfd0(e, stream.cast());
        let name = model_file_name(e, this);
        e.set_global(LOADING_FILE_NAME, name);
        let mut create = false;
        if !file.is_null() {
            let name = model_file_name(e, this);
            create = e.call(BS_STREAM_LOAD, &args![stream, name, file]).bool();
        }
        if create || fn_0043cf90(e, this) {
            let block = e.call(MEMORY_ALLOC, &args![0x10u32]).u32();
            let mut model = 0;
            if block != 0 {
                let force = fn_0043cfb0(e, this);
                let prepare = fn_0043cf40(e, this);
                let name = model_file_name(e, this);
                model = model_model(e, Ptr::new(block), name, stream, prepare as u8, force as u8)
                    .addr();
            }
            e.call(MODEL_POINTER_ASSIGN, &args![slot, model]);
        } else {
            let verb = if file.is_null() { FIND_WORD } else { LOAD_WORD };
            let name = model_file_name(e, this);
            e.call(LOG, &args![MODEL_ERROR_MESSAGE, verb, name]);
        }
        e.set_global(LOADING_FILE_NAME, 0u32);
        bs_stream_destructor(e, stream.cast());
    });
}

// Translated from 0043cf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 of the model's `cFlags` is set.
pub fn fn_0043cf40(e: &mut Engine, this: Ptr<QueuedModel>) -> bool {
    let flags = e.get(this, QueuedModel::cFlags);
    e.call(TEST_FLAG_BIT_1, &args![flags as u32]).bool()
}

// Translated from 0043cf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 8 of the model's `cFlags`.
pub fn fn_0043cf60(e: &mut Engine, this: Ptr<QueuedModel>, flag: u8) {
    let flags = model_flags_pointer(this);
    e.call(SET_FLAG_BIT_8, &args![flag as u32, flags]);
}

// Translated from 0043cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x10 of the model's `cFlags` is set.
pub fn fn_0043cf90(e: &mut Engine, this: Ptr<QueuedModel>) -> bool {
    let flags = e.get(this, QueuedModel::cFlags);
    e.call(TEST_FLAG_BIT_10, &args![flags as u32]).bool()
}

// Translated from 0043cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x20 of the model's `cFlags` is set.
pub fn fn_0043cfb0(e: &mut Engine, this: Ptr<QueuedModel>) -> bool {
    let flags = e.get(this, QueuedModel::cFlags);
    e.call(TEST_FLAG_BIT_20, &args![flags as u32]).bool()
}

// Translated from 0043cfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::BSStream` (the engine map has no name; Ghidra's library match
/// calls it `CMiniDockFrameWnd`'s, which it is not): the `NiStream`
/// constructor, the stream's virtual table, no node references and no
/// object reference map. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043cfd0(e: &mut Engine, this: Ptr<BSStream>) -> Ptr<BSStream> {
    e.call(NI_STREAM_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BS_STREAM_VTABLE);
    let references = this.byte_add(BSStream::spNodeReferences.off);
    e.call(NI_POINTER_CONSTRUCT, &args![references, 0u32]);
    e.set(this, BSStream::pObjectRefMap, Ptr::NULL);
    this
}

// Translated from 0043d050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::SaveLinkID` (Xbox PDB): asks the stream (virtual function
/// `0x2c`) for the link id of `object` and writes it with
/// `NiStreamSaveBinary<unsigned int>`.
pub fn ni_stream_save_link_id(e: &mut Engine, this: Ptr, object: Ptr) {
    let link_id = e.vcall(this.addr(), 0x2c, &args![object]).u32();
    // The local `unsigned int` of the game's stack frame.
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), link_id);
        e.call(NI_STREAM_SAVE_BINARY_U32, &args![this, value]);
    });
}

// Translated from 0043d090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::~BSStream` (Xbox PDB): restores the stream's virtual table,
/// deletes the object reference map ([`fn_0043d100`]), destroys the node
/// references pointer and runs the `NiStream` destructor body.
///
/// The exception-unwinding frame is not translated.
pub fn bs_stream_destructor(e: &mut Engine, this: Ptr<BSStream>) {
    e.mem.set_u32(this.addr(), BS_STREAM_VTABLE);
    fn_0043d100(e, this);
    let references = this.byte_add(BSStream::spNodeReferences.off);
    e.call(NI_POINTER_DESTRUCT, &args![references]);
    e.call(NI_STREAM_DESTRUCT, &args![this]);
}

// Translated from 0043d100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the stream's object reference map (virtual destructor, flag 1)
/// when there is one and clears the pointer.
pub fn fn_0043d100(e: &mut Engine, this: Ptr<BSStream>) {
    let map = e.get(this, BSStream::pObjectRefMap);
    if !map.is_null() {
        e.vcall(map.addr(), 0, &args![1u32]);
    }
    e.set(this, BSStream::pObjectRefMap, Ptr::NULL);
}

// Translated from 0043d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor
/// and, when bit 0 of `flags` is set, frees the stream. Returns `this`.
pub fn bs_stream_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSStream>,
    flags: u32,
) -> Ptr<BSStream> {
    bs_stream_destructor(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043d180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel::Finish` (Xbox PDB): inside the memory context of the task,
/// for a loaded model: bit 2 adds a reference to the model; unless bit 8 is
/// set, the model's 3D object is updated and, when it is a fade node, given
/// its fade range and LOD multiplier type ([`fade_range`]); then the model
/// is added to the loader's map, and when the map already has one of that
/// name `spModel` is replaced by it. Last the task's virtual function `0x28`
/// (`CheckFinished`) is called.
///
/// The exception-unwinding frame is not translated.
pub fn queued_model_finish(e: &mut Engine, this: Ptr<QueuedModel>) {
    let context = e.get(this, QueuedModel::eContext);
    // The scope guard of the game's stack frame (4 bytes).
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![
                guard,
                context,
                1u32,
                MODEL_LOADER_SOURCE,
                MODEL_FINISH_SOURCE_LINE
            ],
        );
        queued_model_finish_in_context(e, this);
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

/// The body of [`queued_model_finish`] inside its memory context.
pub(crate) fn queued_model_finish_in_context(e: &mut Engine, this: Ptr<QueuedModel>) {
    let slot = model_pointer(this);
    if !ni_pointer_get(e, slot).is_null() {
        if fn_0043d490(e, this) {
            let model = ni_pointer_get(e, slot);
            e.call(ADD_REFERENCE, &args![model]);
        }
        if !fn_0043d4b0(e, this) {
            let model = ni_pointer_get(e, slot);
            if !fn_0043b230(e, model).is_null() {
                let model = ni_pointer_get(e, slot);
                let object = fn_0043b230(e, model);
                // Virtual function `0x10` gives the object as a fade node
                // (0 for other classes).
                let fade_node: Ptr = e.vcall(object.addr(), 0x10, &args![]).ptr();
                if !fade_node.is_null() {
                    fade_range(e, this, fade_node);
                }
                let model = ni_pointer_get(e, slot);
                let name = model_file_name(e, this);
                let owner = e.global::<u32>(FILE_MAP_OWNER);
                let added = e
                    .call(MODEL_LOADER_ADD_MODEL, &args![owner, name, model])
                    .bool();
                if !added {
                    let name = model_file_name(e, this);
                    let owner = e.global::<u32>(FILE_MAP_OWNER);
                    e.call(MODEL_LOADER_FIND_MODEL, &args![owner, name, slot]);
                }
            }
        }
    }
    fn_0043c610(e, this.cast());
}

/// What `Finish` does with a model whose 3D object is a fade node: updates
/// the node, then sets its fade range and LOD multiplier type.
///
/// The radius is the world bound's radius, or when that is below 0.001 half
/// of the third component of the reference's maximum bound. The range is the
/// radius times the visual distance scale, unless the radius is 0 or the
/// world is too small for it: then the limit (half the world in units of
/// 4096 per cell, minus radius and 2048). A non-zero overridden visual
/// distance of the queued model replaces the range. The node gets the range
/// times the fade range scale and the range itself.
///
/// The floating-point steps follow the x87 code: values are computed in
/// `f64` and rounded to `f32` where the game stores a `float`.
pub(crate) fn fade_range(e: &mut Engine, this: Ptr<QueuedModel>, fade_node: Ptr) {
    // The update record of the game's stack frame (12 bytes).
    e.with_stack(NiUpdateData::SIZE, |e, update| {
        fn_0043d410(e, update.cast(), 0.0, 0, 0);
        e.call(NI_AV_OBJECT_UPDATE, &args![fade_node, update]);
    });
    let bound = ni_av_object_get_world_bound(e, fade_node);
    let mut radius = e.call(BOUND_RADIUS, &args![bound]).f32();
    if (radius as f64) < e.global::<f64>(SMALL_RADIUS) {
        let model = ni_pointer_get(e, model_pointer(this));
        let object = fn_0043b230(e, model);
        let depth = e.with_stack(12, |e, out| {
            let vector = e.call(GET_BOUND_MAX, &args![out, object]).u32();
            e.mem.f32(vector + 8)
        });
        radius = (depth as f64 * e.global::<f64>(HALF)) as f32;
    }
    // The game first stores `FLT_MAX` (`01016970`) into the limit; it is
    // overwritten at once.
    let cells_setting = fn_0043d4d0(e, Ptr::new(CELL_COUNT_SETTING));
    let cells = e.mem.u32(cells_setting.addr());
    let world_size = (cells << 12) as f64;
    let limit = (world_size * e.global::<f64>(HALF)
        - (radius as f64 + e.global::<f64>(LOADED_AREA_MARGIN))) as f32;
    let scale = e
        .call(SETTING_VALUE_POINTER, &args![VISUAL_DISTANCE_SCALE_SETTING])
        .u32();
    let scaled = (radius as f64 * e.mem.f32(scale) as f64) as f32;
    // `radius == 0`, `limit <= scaled` and unordered compares take the limit.
    let mut range = if radius != 0.0 && scaled < limit {
        scaled
    } else {
        limit
    };
    let overridden = e.call(OVERRIDDEN_VISUAL_DISTANCE, &args![this]).f32();
    if overridden as f64 != e.global::<f64>(ZERO) {
        range = e.call(OVERRIDDEN_VISUAL_DISTANCE, &args![this]).f32();
    }
    let scale = e
        .call(SETTING_VALUE_POINTER, &args![FADE_RANGE_SCALE_SETTING])
        .u32();
    let far = (range as f64 * e.mem.f32(scale) as f64) as f32;
    e.call(FADE_NODE_SET_RANGE, &args![fade_node, far, range]);
    let lod_fade_mult = e.get(this, QueuedModel::eLODFadeMult);
    e.call(
        FADE_NODE_SET_LOD_MULT_TYPE,
        &args![fade_node, lod_fade_mult],
    );
}

// Translated from 0043d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUpdateData::NiUpdateData(float, bool, bool)` (no name in the engine
/// map): stores the time and the first two flags, clears the other three.
/// Returns `this`.
pub fn fn_0043d410(
    e: &mut Engine,
    this: Ptr<NiUpdateData>,
    time: f32,
    update_controllers: u8,
    parallel_update: u8,
) -> Ptr<NiUpdateData> {
    e.set(this, NiUpdateData::fTime, x87_quiet(time));
    e.set(this, NiUpdateData::bUpdateControllers, update_controllers);
    e.set(this, NiUpdateData::bParallelUpdate, parallel_update);
    e.set(this, NiUpdateData::bFoundParticles, 0);
    e.set(this, NiUpdateData::bFoundMorphController, 0);
    e.set(this, NiUpdateData::bSceneGraphChange, 0);
    this
}

// Translated from 0043d450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::GetWorldBound` (Xbox PDB): the object's bound (the pointer
/// at +0x20), or the empty bound at `011f4288` when it has none.
pub fn ni_av_object_get_world_bound(e: &mut Engine, this: Ptr) -> Ptr {
    // NiAVObject::m_pkWorldBound (Xbox PDB) +0x20
    let bound = e.mem.u32(this.addr() + 0x20);
    if bound == 0 {
        Ptr::new(EMPTY_WORLD_BOUND)
    } else {
        Ptr::new(bound)
    }
}

// Translated from 0043d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the model's `cFlags` is set.
pub fn fn_0043d490(e: &mut Engine, this: Ptr<QueuedModel>) -> bool {
    let flags = e.get(this, QueuedModel::cFlags);
    e.call(TEST_FLAG_BIT_2, &args![flags as u32]).bool()
}

// Translated from 0043d4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 8 of the model's `cFlags` is set.
pub fn fn_0043d4b0(e: &mut Engine, this: Ptr<QueuedModel>) -> bool {
    let flags = e.get(this, QueuedModel::cFlags);
    e.call(TEST_FLAG_BIT_8, &args![flags as u32]).bool()
}

// Translated from 0043d4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of the value of a setting object (`this + 4`); without an object
/// (0) a scratch dword at `01202800` is zeroed and its address returned.
pub fn fn_0043d4d0(e: &mut Engine, this: Ptr) -> Ptr {
    if this.is_null() {
        e.set_global(SCRATCH_ZERO, 0u32);
        Ptr::new(SCRATCH_ZERO)
    } else {
        this.byte_add(4)
    }
}

// Translated from 0043d510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel::GetDescription` (Xbox PDB): `QueuedFileEntry::GetDescription`
/// with the kind word `"model"`; returns its result.
pub fn queued_model_get_description(
    e: &mut Engine,
    this: Ptr<QueuedModel>,
    buffer: u32,
    size: u32,
) -> bool {
    e.call(
        QUEUED_FILE_ENTRY_GET_DESCRIPTION,
        &args![this, buffer, size, MODEL_WORD],
    )
    .bool()
}

// Translated from 0043d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeBillboard` constructor (no name in the engine map): the
/// `QueuedTexture` constructor with `name` and `context`, the billboard's
/// virtual table and `data` as `pBillboardData`. Returns `this`.
pub fn fn_0043d540(
    e: &mut Engine,
    this: Ptr<QueuedTreeBillboard>,
    name: u32,
    context: u32,
    data: Ptr<TreeBillboardData>,
) -> Ptr<QueuedTreeBillboard> {
    fn_0043bd10(e, this.cast(), name, context);
    e.mem.set_u32(this.addr(), QUEUED_TREE_BILLBOARD_VTABLE);
    e.set(this, QueuedTreeBillboard::pBillboardData, data.cast());
    this
}

// Translated from 0043d580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeBillboard::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_0043d5b0`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn queued_tree_billboard_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<QueuedTreeBillboard>,
    flags: u32,
) -> Ptr<QueuedTreeBillboard> {
    fn_0043d5b0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043d5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeBillboard` destructor (no name in the engine map): deletes the
/// billboard data (when there is some) and runs the `QueuedTexture`
/// destructor.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043d5b0(e: &mut Engine, this: Ptr<QueuedTreeBillboard>) {
    e.mem.set_u32(this.addr(), QUEUED_TREE_BILLBOARD_VTABLE);
    let data = e.get(this, QueuedTreeBillboard::pBillboardData);
    if !data.is_null() {
        fn_0043d640(e, data.cast(), 1);
    }
    fn_0043bf80(e, this.cast());
}

// Translated from 0043d640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TREE_BILLBOARD_DATA` scalar deleting destructor (the decompiler names it
/// after an MFC class; the engine map has no name): runs the destructor body
/// (`0051d400`) and, when bit 0 of `flags` is set, frees the block. Returns
/// `this`.
pub fn fn_0043d640(
    e: &mut Engine,
    this: Ptr<TreeBillboardData>,
    flags: u32,
) -> Ptr<TreeBillboardData> {
    e.call(TREE_BILLBOARD_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043d670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeBillboard::Finish` (Xbox PDB): `QueuedTexture::Finish`, then
/// the task is added to the post-process queue ([`iomanager_add_post_process_task`]).
pub fn queued_tree_billboard_finish(e: &mut Engine, this: Ptr<QueuedTreeBillboard>) {
    queued_texture_finish(e, this.cast());
    iomanager_add_post_process_task(e, this.cast());
}

// Translated from 0043d690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOManager::AddPostProcessTask` (Xbox PDB): [`fn_0043d6b0`] on the IO
/// manager (the global at `01202d98`) with `task`.
pub fn iomanager_add_post_process_task(e: &mut Engine, task: Ptr<IOTask>) {
    let manager = e.global::<u32>(TASK_QUEUE);
    fn_0043d6b0(e, Ptr::new(manager), task);
}

// Translated from 0043d6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body of `IOManager::AddPostProcessTask` (no name in the engine map):
/// takes a counted reference to `task` and puts it into the queue its key
/// names (the priority byte, [`fn_0043cc80`]) in the table of queues at
/// `IOManager + 0x64` (a PC field, the Xbox class ends at 0x5c).
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043d6b0(e: &mut Engine, this: Ptr, task: Ptr<IOTask>) {
    // The task pointer of the game's stack frame.
    e.with_stack(4, |e, holder| {
        e.call(TASK_POINTER_CONSTRUCT, &args![holder, task]);
        let priority = fn_0043cc80(e, task) as u8;
        // IOManager + 0x64: the table of queues (not in the Xbox layout).
        let queues = e.mem.u32(this.addr() + 0x64);
        e.call(QUEUE_TABLE_ADD, &args![queues, priority as u32, holder]);
        e.call(TASK_POINTER_DESTRUCT, &args![holder]);
    });
}

// Translated from 0043d730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeBillboard::PostProcess` (Xbox PDB): calls the virtual function
/// `0x30` (`CreateBillboard`).
pub fn queued_tree_billboard_post_process(e: &mut Engine, this: Ptr<QueuedTreeBillboard>) {
    e.vcall(this.addr(), 0x30, &args![]);
}

// Translated from 0043d750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeBillboard::CreateBillboard` (Xbox PDB): builds the distant 3D
/// of the tree (`BuildDistant3D(1)`); with it, a three-word record on the
/// stack (0, the node, the count read from the tree) is passed with the
/// billboard data to the instance builder (`00b59ae0`), and the node is
/// deleted again (virtual destructor, flag 1).
pub fn queued_tree_billboard_create_billboard(e: &mut Engine, this: Ptr<QueuedTreeBillboard>) {
    let data: Ptr<TreeBillboardData> = e.get(this, QueuedTreeBillboard::pBillboardData).cast();
    let tree = e.get(data, TreeBillboardData::pTree);
    let node = e.call(BUILD_DISTANT_3D, &args![tree, 1u32]).u32();
    if node == 0 {
        return;
    }
    // The record of the game's stack frame: +0 zero, +4 the node, +8 the count.
    e.with_stack(12, |e, record| {
        e.call(ZERO_THREE_WORDS, &args![record]);
        let tree = e.get(data, TreeBillboardData::pTree);
        let count = e.call(ARRAY_COUNT, &args![tree]).u32();
        e.mem.set_u32(record.addr() + 8, count);
        e.mem.set_u32(record.addr() + 4, node);
        e.mem.set_u32(record.addr(), 0);
        let size = e.get(data, TreeBillboardData::iArraySize) & 0xffff;
        let chunk = e.get(data, TreeBillboardData::iCellChunk);
        let key = e.get(data, TreeBillboardData::iCellKey);
        let instanced = e.get(data, TreeBillboardData::pInstancedNode);
        let locations = e.get(data, TreeBillboardData::pLocArray);
        let colors = e.get(data, TreeBillboardData::pColorArray);
        e.call(
            CREATE_BILLBOARD_INSTANCES,
            &args![chunk, key, instanced, record, locations, colors, size],
        );
        let node = e.mem.u32(record.addr() + 4);
        if node != 0 {
            e.vcall(node, 0, &args![1u32]);
        }
    });
}

// Translated from 0043d820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeBillboard::GetDescription` (Xbox PDB):
/// `QueuedFileEntry::GetDescription` with the kind word `"Tree Billboard"`;
/// returns its result.
pub fn queued_tree_billboard_get_description(
    e: &mut Engine,
    this: Ptr<QueuedTreeBillboard>,
    buffer: u32,
    size: u32,
) -> bool {
    e.call(
        QUEUED_FILE_ENTRY_GET_DESCRIPTION,
        &args![this, buffer, size, TREE_BILLBOARD_WORD],
    )
    .bool()
}

// Translated from 0043d850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeModel` constructor (no name in the engine map): the
/// `QueuedModel` constructor over the `TESModel` embedded in the tree (at
/// +0x30; none for a null tree) with `context`, `lod_fade_mult`, bit 1 set
/// and bit 2 clear; then the tree model's virtual table, `pRef`, `pTree` and
/// the context `0x1e`. Returns `this`.
pub fn fn_0043d850(
    e: &mut Engine,
    this: Ptr<QueuedTreeModel>,
    reference: Ptr,
    tree: Ptr,
    context: u32,
    lod_fade_mult: u32,
) -> Ptr<QueuedTreeModel> {
    // TESObjectTREE::TESModel base (Xbox PDB) +0x30
    let tes_model = if tree.is_null() {
        Ptr::NULL
    } else {
        tree.byte_add(0x30)
    };
    fn_0043c890(e, this.cast(), tes_model, context, lod_fade_mult, 1, 0);
    e.mem.set_u32(this.addr(), QUEUED_TREE_MODEL_VTABLE);
    e.set(this, QueuedTreeModel::pRef, reference);
    e.set(this, QueuedTreeModel::pTree, tree);
    e.set(this, QueuedTreeModel::eContext, TREE_MODEL_CONTEXT);
    this
}

// --- The tree model, `QueuedMagicItem`, `QueuedKF` and its subclasses (`0043d8c0` onwards). ---

/// `BSTreeManager`'s global pointer (`011d5c48`) and the three methods
/// `QueuedTreeModel` calls on it: `00664f50` (`__thiscall(reference, tree)
/// -> tree node`, no name in the engine map), `00665b80`
/// (`__thiscall(reference, tree node)`, the same) and
/// `BSTreeManager::RemoveBackgroundLoadedTree` (Xbox PDB, `00665be0`,
/// `__thiscall(reference)`).
pub(crate) const TREE_MANAGER: u32 = 0x011d_5c48;
pub(crate) const TREE_MANAGER_BUILD_TREE: u32 = 0x0066_4f50;
pub(crate) const TREE_MANAGER_STORE_TREE: u32 = 0x0066_5b80;
pub(crate) const TREE_MANAGER_REMOVE_BACKGROUND_TREE: u32 = 0x0066_5be0;
/// The text of a `TESTexture` (`00408da0`, `this + 4` is a `BSStringT`): the
/// string, or an empty string without one.
pub(crate) const TEXTURE_TEXT: u32 = 0x0040_8da0;
/// `strcat_s(destination, size, source)` through the game's wrapper
/// (`00406d50`).
pub(crate) const STRING_APPEND: u32 = 0x0040_6d50;
/// `ModelLoader::QueueTexture` (Xbox PDB, `004436c0`) on
/// [`FILE_MAP_OWNER`], `__thiscall(file name, key, parent queued file)`.
pub(crate) const MODEL_LOADER_QUEUE_TEXTURE_BY_NAME: u32 = 0x0044_36c0;
/// `ModelLoader::QueueModel` taking a `TESModel` (`00443d30`, no name in the
/// engine map) on [`FILE_MAP_OWNER`], `__thiscall(TESModel, key, parent
/// queued file, LOD multiplier, 1, 0, 0)`.
pub(crate) const MODEL_LOADER_QUEUE_MODEL: u32 = 0x0044_3d30;
/// `ModelLoader` method (`004463b0`, no name in the engine map) on
/// [`FILE_MAP_OWNER`], `__thiscall(list of model names, 0, key, parent
/// queued file, 0)`: queues every model of the list.
pub(crate) const MODEL_LOADER_QUEUE_MODEL_LIST: u32 = 0x0044_63b0;
/// `ModelLoader::QueueAnimations` (Xbox PDB, `00445a10`) on
/// [`FILE_MAP_OWNER`], `__thiscall(animation list, key, parent queued file,
/// 0, 1, 1)`.
pub(crate) const MODEL_LOADER_QUEUE_ANIMATIONS: u32 = 0x0044_5a10;
/// `TES::GetLODMult` (Xbox PDB, `0045c6b0`), `__cdecl(form) -> LOD
/// multiplier`.
pub(crate) const GET_LOD_MULT: u32 = 0x0045_c6b0;
/// `TESBipedModelForm::GetBipedTESModel_ov2` (Xbox PDB, `00481150`),
/// `__thiscall(index) -> TESModel`.
pub(crate) const GET_BIPED_TES_MODEL: u32 = 0x0048_1150;
/// `TESNPC::BuildDefaultModelList` (Xbox PDB, `0060af90`), `__thiscall(1, 1)
/// -> list`.
pub(crate) const BUILD_DEFAULT_MODEL_LIST: u32 = 0x0060_af90;
/// Empties a list (`00470470`, releases every node after the first) and
/// deletes it (`004702f0`, `__thiscall(flags)`: destructor, then the block
/// when bit 0 of `flags` is set).
pub(crate) const LIST_CLEAR: u32 = 0x0047_0470;
pub(crate) const LIST_DELETE: u32 = 0x0047_02f0;
/// Length of the string of the `TESModel` at `this` (`0048cee0`: the
/// `BSStringT` at `this + 4`).
pub(crate) const MODEL_NAME_LENGTH: u32 = 0x0048_cee0;
/// `__thiscall(mask) -> bool` on an `EffectSetting` (`00403e60`): whether
/// `iFlags` (`this + 0x58`) has any bit of `mask`.
pub(crate) const EFFECT_SETTING_HAS_FLAG: u32 = 0x0040_3e60;
/// `EffectSetting` methods of its two queue counters (`+0xa8`, the Xbox PDB's
/// `iEffectLoadedCount`, and `+0xac`, `iAssociatedItemLoadedCount`): the
/// counter of the effect is negative while it is queued
/// (`EffectSetting::IsEffectQueued`, `00408a20`) and `00408970` takes one
/// more from it (logging a message instead when it is positive); the counter
/// of the associated item is negative while it is queued (`004088a0`),
/// positive once loaded (`00408880`), and `00408790` takes one from it the
/// same way.
pub(crate) const EFFECT_IS_QUEUED: u32 = 0x0040_8a20;
pub(crate) const EFFECT_COUNT_DOWN: u32 = 0x0040_8970;
pub(crate) const ASSOCIATED_ITEM_IS_QUEUED: u32 = 0x0040_88a0;
pub(crate) const ASSOCIATED_ITEM_IS_LOADED: u32 = 0x0040_8880;
pub(crate) const ASSOCIATED_ITEM_COUNT_DOWN: u32 = 0x0040_8790;
/// `EffectSetting` getter of the associated item (`005f5f80`, `this + 0x60`,
/// the Xbox PDB's `pAssociatedItem`).
pub(crate) const EFFECT_ASSOCIATED_ITEM: u32 = 0x005f_5f80;
/// Form type byte of a form (`00401170`, `this + 4`).
pub(crate) const FORM_TYPE: u32 = 0x0040_1170;
/// `EffectItem::pEffectSetting` getter (`00825c00`, `this + 0x14`).
pub(crate) const EFFECT_ITEM_SETTING: u32 = 0x0082_5c00;
/// A test of an `EffectSetting` (`004064b0`, no name in the engine map): looks
/// the dword at `this + 0x98` up in the 16-byte-entry table at `01183328`
/// and tests its first dword against `0x7c`.
pub(crate) const EFFECT_SETTING_IN_TABLE: u32 = 0x0040_64b0;
/// Whether the list at `this` (a `BSSimpleList` head) has no item and no next
/// node (`008256d0`).
pub(crate) const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Increments a global counter (`0040ab50`, the dword at `011c3438`).
pub(crate) const COUNT_UNLOADED_ASSOCIATED_ITEM: u32 = 0x0040_ab50;
/// `MagicItem::FinishedLoading` (Xbox PDB, `0040ab70`).
pub(crate) const MAGIC_ITEM_FINISHED_LOADING: u32 = 0x0040_ab70;
/// `__thiscall() -> bool` on a task (`00449110`): `eState >= 4`; and
/// (`00449130`): `eState == 6`.
pub(crate) const TASK_STATE_AT_LEAST_4: u32 = 0x0044_9110;
pub(crate) const TASK_STATE_IS_6: u32 = 0x0044_9130;
/// `QueuedFile::QueuedFile(context)` (Xbox PDB, `00c3c590`) and the body of
/// its destructor (`00c3c620`).
pub(crate) const QUEUED_FILE_CONSTRUCT: u32 = 0x00c3_c590;
pub(crate) const QUEUED_FILE_DESTRUCT: u32 = 0x00c3_c620;
/// The `NiPointer<KFModel>` functions (`0044afe0`, `0044b070`, `0044b030`;
/// no names in the engine map): `__thiscall(model)` constructor,
/// `__thiscall(model)` assignment (releases the old model, keeps the new) and
/// the release the destructor does.
pub(crate) const KF_POINTER_CONSTRUCT: u32 = 0x0044_afe0;
pub(crate) const KF_POINTER_ASSIGN: u32 = 0x0044_b070;
pub(crate) const KF_POINTER_RELEASE: u32 = 0x0044_b030;
/// `ModelLoader` methods of the map of `KFModel`s (no names in the engine
/// map) on [`FILE_MAP_OWNER`]: `004483e0` is `__thiscall(name) -> KFModel`
/// (0 when the map has none) and `00447040` `__thiscall(name, KFModel) ->
/// bool` (false when the name is already there).
pub(crate) const FIND_KF_MODEL: u32 = 0x0044_83e0;
pub(crate) const ADD_KF_MODEL: u32 = 0x0044_7040;
/// `QueuedKF::QueueMe` (Xbox PDB, `00441e10`).
pub(crate) const QUEUED_KF_QUEUE_ME: u32 = 0x0044_1e10;
/// `AnimIdle::Loaded` (Xbox PDB, `00496cd0`), `__thiscall(KFModel)`.
pub(crate) const ANIM_IDLE_LOADED: u32 = 0x0049_6cd0;
/// `Animation` method (`00490fa0`, no name in the engine map),
/// `__thiscall(KFModel)`: takes the loaded `KFModel` into the animation.
pub(crate) const ANIMATION_KF_LOADED: u32 = 0x0049_0fa0;

/// The counters `IOTask::GenerateKey` takes its values from (`01202d9c` and
/// `01202da0`; the Xbox PDB's statics `iStaticCounter` and `iStaticOffset`,
/// which of the two is which is not confirmed).
pub(crate) const KEY_COUNTER_FIRST: u32 = 0x0120_2d9c;
pub(crate) const KEY_COUNTER_SECOND: u32 = 0x0120_2da0;

/// `QueuedMagicItem`'s (`01016a18`), `QueuedKF`'s (`01016a48`),
/// `QueuedAnimIdle`'s (`01016a80`) and `QueuedReplacementKF`'s (`01016ac0`)
/// virtual tables.
pub(crate) const QUEUED_MAGIC_ITEM_VTABLE: u32 = 0x0101_6a18;
pub(crate) const QUEUED_KF_VTABLE: u32 = 0x0101_6a48;
pub(crate) const QUEUED_ANIM_IDLE_VTABLE: u32 = 0x0101_6a80;
pub(crate) const QUEUED_REPLACEMENT_KF_VTABLE: u32 = 0x0101_6ac0;
/// `"tree model"`, `"KF"` and `"AnimIdle"`: the kind words of the
/// descriptions.
pub(crate) const TREE_MODEL_WORD: u32 = 0x0101_6a08;
pub(crate) const KF_WORD: u32 = 0x0101_6a78;
pub(crate) const ANIM_IDLE_WORD: u32 = 0x0101_6ab0;
/// The source lines the memory context scopes of `QueuedTreeModel::Run` and
/// `QueuedKF::Run` record, and the context of the second (`0x33`).
pub(crate) const TREE_RUN_SOURCE_LINE: u32 = 0x476;
pub(crate) const KF_RUN_SOURCE_LINE: u32 = 0x529;
pub(crate) const KF_CONTEXT: u32 = 0x33;
/// Size of the path buffers of `QueuedTreeModel::QueueMe` (`strcpy_s` is
/// told this size).
pub(crate) const PATH_BUFFER_SIZE: u32 = 0x104;

layout! {
    /// `QueuedMagicItem` (Xbox PDB), 0x30 bytes: a `QueuedFile` and the magic
    /// item and effect it loads the models of.
    pub struct QueuedMagicItem: 0x30 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pMagicItem` (Xbox PDB): `MagicItem*`.
        0x28 pMagicItem: Ptr,
        /// `pEffectSetting` (Xbox PDB): `EffectSetting*`.
        0x2C pEffectSetting: Ptr,
    }

    /// `QueuedKF` (Xbox PDB), 0x38 bytes: a `QueuedFileEntry` and the
    /// animation file it loads.
    pub struct QueuedKF: 0x38 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pFileName` (`QueuedFileEntry`, Xbox PDB).
        0x28 pFileName: u32,
        /// `pFileEntry` (`QueuedFileEntry`, Xbox PDB).
        0x2C pFileEntry: Ptr,
        /// `spKFModel` (Xbox PDB): `NiPointer<KFModel>`.
        0x30 spKFModel: Ptr,
        /// `cFlags` (Xbox PDB). Bit 1 is set by `Run` when the loader
        /// already had the model (`0043e480`, `0043e530`).
        0x34 cFlags: u8,
    }

    /// `QueuedAnimIdle` (Xbox PDB), 0x40 bytes: a `QueuedKF` and the idle it
    /// is for.
    pub struct QueuedAnimIdle: 0x40 {
        /// `spKFModel` (`QueuedKF`, Xbox PDB): `NiPointer<KFModel>`.
        0x30 spKFModel: Ptr,
        /// `cFlags` (`QueuedKF`, Xbox PDB).
        0x34 cFlags: u8,
        /// `pQueuedForRef` (Xbox PDB): `TESObjectREFR*`.
        0x38 pQueuedForRef: Ptr,
        /// `spAnimIdle` (Xbox PDB): `NiPointer<AnimIdle>`.
        0x3C spAnimIdle: Ptr,
    }

    /// `QueuedReplacementKF` (Xbox PDB), 0x40 bytes: a `QueuedKF` and the
    /// animation and list it belongs to.
    pub struct QueuedReplacementKF: 0x40 {
        /// `spKFModel` (`QueuedKF`, Xbox PDB): `NiPointer<KFModel>`.
        0x30 spKFModel: Ptr,
        /// `cFlags` (`QueuedKF`, Xbox PDB).
        0x34 cFlags: u8,
        /// `pAnim` (Xbox PDB): `Animation*`.
        0x38 pAnim: Ptr,
        /// `pOwner` (Xbox PDB): `QueuedReplacementKFList*`.
        0x3C pOwner: Ptr,
    }

    /// `QueuedReplacementKFList` (Xbox PDB), 0x38 bytes (the same on PC): a
    /// `QueuedFile`, the animation and the two child counters.
    pub struct QueuedReplacementKFList: 0x38 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pAnim` (Xbox PDB): `Animation*`.
        0x28 pAnim: Ptr,
        /// `iPostProcessingChildCount` (Xbox PDB): the children that will
        /// be post-processed (`QueuedReplacementKF::Finish` counts one).
        0x2C iPostProcessingChildCount: u32,
        /// `iPostProcessedChildCount` (Xbox PDB): the children already
        /// post-processed (`0043ea30` counts one).
        0x30 iPostProcessedChildCount: u32,
    }
}

/// `QueuedFileEntry::pFileName` of a queued KF through the game's getter
/// (`0045cd60`).
pub(crate) fn kf_file_name(e: &mut Engine, this: Ptr<QueuedKF>) -> u32 {
    queued_file_name(e, this.cast())
}

/// The address of `QueuedKF::spKFModel`, which the game passes to the
/// `NiPointer` functions.
pub(crate) fn kf_model_pointer(this: Ptr<QueuedKF>) -> Ptr {
    this.byte_add(QueuedKF::spKFModel.off)
}

/// `ModelLoader::QueueModel` of `model` for the queued file `parent`, with
/// the priority of the parent's key and the given LOD multiplier (the other
/// three arguments are constant in every caller of this part).
pub(crate) fn queue_model(e: &mut Engine, model: u32, parent: Ptr, lod_mult: u32) {
    let key = fn_0043cc60(e, parent.cast()) as u32;
    let owner = e.global::<u32>(FILE_MAP_OWNER);
    e.call(
        MODEL_LOADER_QUEUE_MODEL,
        &args![owner, model, key, parent, lod_mult, 1u32, 0u32, 0u32],
    );
}

// Translated from 0043d8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeModel::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_0043d8f0`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn queued_tree_model_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<QueuedTreeModel>,
    flags: u32,
) -> Ptr<QueuedTreeModel> {
    fn_0043d8f0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043d8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeModel` destructor (no name in the engine map; the decompiler's
/// library match names the body after a cancellation-token callback): sets
/// the tree model's virtual table and runs the `QueuedModel` destructor
/// ([`fn_0043c830`]).
pub fn fn_0043d8f0(e: &mut Engine, this: Ptr<QueuedTreeModel>) {
    e.mem.set_u32(this.addr(), QUEUED_TREE_MODEL_VTABLE);
    fn_0043c830(e, this.cast());
}

// Translated from 0043d910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeModel::QueueMe` (Xbox PDB): when the tree's leaf texture
/// (`TESObjectTREE`'s icon part at +0x48, its text from `00408da0`) is not
/// empty, builds `<virtual function 0x18 of the icon part><texture text>`
/// in a 0x104-byte buffer, normalizes the path into a second buffer and
/// queues that texture (`ModelLoader::QueueTexture`) with this task's key and
/// as the parent. Then `QueuedModel::QueueMe` ([`queued_model_queue_me`]).
///
/// The stack-cookie check is not translated; the two path buffers of the
/// game's stack frame come from [`Engine::with_stack`].
pub fn queued_tree_model_queue_me(e: &mut Engine, this: Ptr<QueuedTreeModel>) {
    let tree = e.get(this, QueuedTreeModel::pTree);
    // TESObjectTREE's icon part (`TESIconTree`, Xbox PDB) +0x48 on PC.
    let icon = tree.byte_add(0x48);
    let text = e.call(TEXTURE_TEXT, &args![icon]).u32();
    if e.mem.u8(text) != 0 {
        e.with_stack(PATH_BUFFER_SIZE, |e, path| {
            e.with_stack(PATH_BUFFER_SIZE, |e, normalized| {
                let tree = e.get(this, QueuedTreeModel::pTree);
                let icon = tree.byte_add(0x48);
                let directory = e.vcall(icon.addr(), 0x18, &args![]).u32();
                e.call(STRING_COPY, &args![path, PATH_BUFFER_SIZE, directory]);
                let text = e.call(TEXTURE_TEXT, &args![icon]).u32();
                e.call(STRING_APPEND, &args![path, PATH_BUFFER_SIZE, text]);
                e.call(NORMALIZE_PATH, &args![path, normalized, PATH_BUFFER_SIZE]);
                let key = fn_0043cc60(e, this.cast()) as u32;
                let owner = e.global::<u32>(FILE_MAP_OWNER);
                e.call(
                    MODEL_LOADER_QUEUE_TEXTURE_BY_NAME,
                    &args![owner, normalized, key, this],
                );
            });
        });
    }
    queued_model_queue_me(e, this.cast());
}

// Translated from 0043da00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeModel::Run` (Xbox PDB): inside the memory context `0x1e`
/// (source line `0x476`), asks the tree manager (`011d5c48`) for the tree
/// node of the reference and tree (`00664f50`) and hands it back to the
/// manager with the reference (`00665b80`).
///
/// The exception-unwinding frame is not translated.
pub fn queued_tree_model_run(e: &mut Engine, this: Ptr<QueuedTreeModel>) {
    // The scope guard of the game's stack frame (4 bytes).
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![
                guard,
                TREE_MODEL_CONTEXT,
                1u32,
                MODEL_LOADER_SOURCE,
                TREE_RUN_SOURCE_LINE
            ],
        );
        let manager = e.global::<u32>(TREE_MANAGER);
        let reference = e.get(this, QueuedTreeModel::pRef);
        let tree = e.get(this, QueuedTreeModel::pTree);
        let node = e
            .call(TREE_MANAGER_BUILD_TREE, &args![manager, reference, tree])
            .u32();
        let manager = e.global::<u32>(TREE_MANAGER);
        let reference = e.get(this, QueuedTreeModel::pRef);
        e.call(TREE_MANAGER_STORE_TREE, &args![manager, reference, node]);
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

// Translated from 0043daa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeModel::Finish` (virtual function `0x8`; no name in the engine
/// map): calls the task's virtual function `0x28` (`CheckFinished`,
/// [`fn_0043c610`]).
pub fn fn_0043daa0(e: &mut Engine, this: Ptr<QueuedTreeModel>) {
    fn_0043c610(e, this.cast());
}

// Translated from 0043dac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeModel::Cancel` (Xbox PDB): `QueuedFile::Cancel` with its two
/// arguments; when `first` (signed) is 4 or more, the tree manager is told
/// to forget the background-loaded tree of the reference
/// (`BSTreeManager::RemoveBackgroundLoadedTree`).
pub fn queued_tree_model_cancel(
    e: &mut Engine,
    this: Ptr<QueuedTreeModel>,
    first: i32,
    second: u32,
) {
    e.call(QUEUED_FILE_CANCEL, &args![this, first as u32, second]);
    if first >= 4 {
        let manager = e.global::<u32>(TREE_MANAGER);
        let reference = e.get(this, QueuedTreeModel::pRef);
        e.call(
            TREE_MANAGER_REMOVE_BACKGROUND_TREE,
            &args![manager, reference],
        );
    }
}

// Translated from 0043db00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTreeModel::GetDescription` (Xbox PDB):
/// `QueuedFileEntry::GetDescription` with the kind word `"tree model"`;
/// returns its result.
pub fn queued_tree_model_get_description(
    e: &mut Engine,
    this: Ptr<QueuedTreeModel>,
    buffer: u32,
    size: u32,
) -> bool {
    e.call(
        QUEUED_FILE_ENTRY_GET_DESCRIPTION,
        &args![this, buffer, size, TREE_MODEL_WORD],
    )
    .bool()
}

// Translated from 0043db30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedMagicItem` constructor (no name in the engine map; the decompiler's
/// library match names it after a cancellation-token registration): the
/// `QueuedFile` constructor with `context`, the magic item's virtual table,
/// `pMagicItem` and `pEffectSetting`. Returns `this`.
pub fn fn_0043db30(
    e: &mut Engine,
    this: Ptr<QueuedMagicItem>,
    magic_item: Ptr,
    effect_setting: Ptr,
    context: u32,
) -> Ptr<QueuedMagicItem> {
    e.call(QUEUED_FILE_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_MAGIC_ITEM_VTABLE);
    e.set(this, QueuedMagicItem::pMagicItem, magic_item);
    e.set(this, QueuedMagicItem::pEffectSetting, effect_setting);
    this
}

// Translated from 0043db70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOTask::GenerateKey` (Xbox PDB): builds the task's `Key` ([`fn_0043dbb0`])
/// from the kind 3, the next value of the second counter ([`fn_0043dc30`]),
/// the priority of the current key ([`fn_0043cc80`], low byte) and the next
/// value of the first counter ([`fn_0043dc10`], low word).
pub fn io_task_generate_key(e: &mut Engine, this: Ptr<IOTask>) {
    let first = fn_0043dc10(e, this.cast()) & 0xffff;
    let priority = fn_0043cc80(e, this) as u8;
    let second = fn_0043dc30(e, this.cast());
    fn_0043dbb0(e, this, 3, second, priority, first as u16);
}

// Translated from 0043dbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs a task key (no name in the engine map): `kind` (a signed byte) in
/// bits 56 and up, `counter` in bits 24 to 55, `priority` in bits 16 to 23
/// and `low` in bits 0 to 15, stored in `Key`.
pub fn fn_0043dbb0(
    e: &mut Engine,
    this: Ptr<IOTask>,
    kind: i8,
    counter: u32,
    priority: u8,
    low: u16,
) {
    // `__allshl` of the sign-extended byte, the zero-extended counter and
    // priority, and the zero-extended word.
    let key = ((kind as i64 as u64) << 56)
        | ((counter as u64) << 24)
        | ((priority as u64) << 16)
        | low as u64;
    e.set(this, IOTask::Key, key);
}

// Translated from 0043dc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterlockedIncrement` of the counter at `01202d9c` (`this` is not used):
/// returns the new value.
pub fn fn_0043dc10(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(INTERLOCKED_INCREMENT, &args![KEY_COUNTER_FIRST])
        .u32()
}

// Translated from 0043dc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterlockedIncrement` of the counter at `01202da0` (`this` is not used):
/// returns the new value.
pub fn fn_0043dc30(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(INTERLOCKED_INCREMENT, &args![KEY_COUNTER_SECOND])
        .u32()
}

// Translated from 0043dc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedMagicItem` destructor (no name in the engine map): the body of the
/// `QueuedFile` destructor (`00c3c620`).
pub fn fn_0043dc50(e: &mut Engine, this: Ptr<QueuedMagicItem>) {
    e.call(QUEUED_FILE_DESTRUCT, &args![this]);
}

// Translated from 0043dc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedMagicItem::QueueMe` (Xbox PDB): queues the models of a magic
/// item's effects.
///
/// First the effect setting of the task: when its model has a name and the
/// effect is not already queued, its model is queued (`QueueModel` with the
/// task as parent); either way the effect's queue counter is counted down.
/// Then every effect of the magic item's effect list (`MagicItem + 0xc`):
/// for the ones the effect-setting test `004064b0` accepts, and (when this
/// task queued the effect's own model) whose associated item is not already
/// queued, the associated item (`EffectSetting + 0x60`) is queued according
/// to the effect's flags ([`queue_associated_item`]). Last, when the
/// associated item is neither loaded nor queued and the effect has flag
/// `0x40000`, a global counter is incremented (`0040ab50`), and the
/// associated item's queue counter is counted down. The task is then put into
/// state 5 (done) and its virtual function `0x28` (`CheckFinished`) is
/// called.
pub fn queued_magic_item_queue_me(e: &mut Engine, this: Ptr<QueuedMagicItem>) {
    let task: Ptr = this.cast();
    let mut queued_model = false;
    let effect = e.get(this, QueuedMagicItem::pEffectSetting);
    // EffectSetting's `TESModel` base (Xbox PDB +0x28) is at +0x18 on PC.
    let effect_model = effect.addr().wrapping_add(0x18);
    if e.call(MODEL_NAME_LENGTH, &args![effect_model]).u32() != 0
        && !e.call(EFFECT_IS_QUEUED, &args![effect]).bool()
    {
        let model = if effect.is_null() { 0 } else { effect_model };
        queue_model(e, model, task, 0);
        queued_model = true;
    }
    e.call(EFFECT_COUNT_DOWN, &args![effect]);

    let magic_item = e.get(this, QueuedMagicItem::pMagicItem);
    // MagicItem's `EffectItemList` (Xbox PDB +0xc) holds a `BSSimpleList` at
    // +4; `list` is the address of the node minus 4.
    let mut list = if magic_item.is_null() {
        0
    } else {
        magic_item.addr() + 0xc
    };
    if !e.call(LIST_IS_EMPTY, &args![list.wrapping_add(4)]).bool() {
        while list != 0 {
            let node = e.call(LIST_NODE_ITEM_ADDRESS, &args![list + 4]).u32();
            let item = e.mem.u32(node);
            let setting = e.call(EFFECT_ITEM_SETTING, &args![item]).u32();
            if e.call(EFFECT_SETTING_IN_TABLE, &args![setting]).bool()
                && (!queued_model || !e.call(ASSOCIATED_ITEM_IS_QUEUED, &args![setting]).bool())
            {
                queue_associated_item(e, task, setting);
                if !e.call(ASSOCIATED_ITEM_IS_LOADED, &args![setting]).bool()
                    && !e.call(ASSOCIATED_ITEM_IS_QUEUED, &args![setting]).bool()
                    && e.call(EFFECT_SETTING_HAS_FLAG, &args![setting, 0x40000u32])
                        .bool()
                {
                    e.call(COUNT_UNLOADED_ASSOCIATED_ITEM, &args![]);
                }
                e.call(ASSOCIATED_ITEM_COUNT_DOWN, &args![setting]);
            }
            let next = e.call(FIELD_AT_4, &args![list + 4]).u32();
            list = if next == 0 { 0 } else { next - 4 };
        }
    }
    e.call(TASK_SET_DONE, &args![this]);
    e.vcall(this.addr(), 0x28, &args![]);
}

/// The part of [`queued_magic_item_queue_me`] that queues the associated item
/// of the effect setting `setting` for the task `task`: nothing without an
/// item or when the effect's flags select no case. With flag `0x10000` and
/// form type `0x28` the model at `+0x3c` of the item (when it has a name);
/// with `0x20000` and form type `0x18` the biped models 0 and 1 of the part
/// at `+0x70`; with `0x40000` and form type `0x2a` the default model list of
/// the item (`BuildDefaultModelList`, emptied and deleted afterwards), and
/// with `0x40000` and form type `0x2b` the animations and the model at
/// `+0xdc`. Each of these uses the item's LOD multiplier (`TES::GetLODMult`)
/// where it queues a model.
pub(crate) fn queue_associated_item(e: &mut Engine, task: Ptr, setting: u32) {
    let item = e.call(EFFECT_ASSOCIATED_ITEM, &args![setting]).u32();
    if item == 0 {
        return;
    }
    if e.call(EFFECT_SETTING_HAS_FLAG, &args![setting, 0x10000u32])
        .bool()
    {
        if e.call(FORM_TYPE, &args![item]).u32() == 0x28
            && e.call(MODEL_NAME_LENGTH, &args![item + 0x3c]).u32() != 0
        {
            let lod_mult = e.call(GET_LOD_MULT, &args![item]).u32();
            queue_model(e, item + 0x3c, task, lod_mult);
        }
    } else if e
        .call(EFFECT_SETTING_HAS_FLAG, &args![setting, 0x20000u32])
        .bool()
    {
        if e.call(FORM_TYPE, &args![item]).u32() == 0x18 {
            for index in 0..2u32 {
                let lod_mult = e.call(GET_LOD_MULT, &args![item]).u32();
                // The biped model part of the item is at +0x70.
                let model = e
                    .call(GET_BIPED_TES_MODEL, &args![item + 0x70, index])
                    .u32();
                queue_model(e, model, task, lod_mult);
            }
        }
    } else if e
        .call(EFFECT_SETTING_HAS_FLAG, &args![setting, 0x40000u32])
        .bool()
    {
        if e.call(FORM_TYPE, &args![item]).u32() == 0x2a {
            let list = e
                .call(BUILD_DEFAULT_MODEL_LIST, &args![item, 1u32, 1u32])
                .u32();
            let key = fn_0043cc60(e, task.cast()) as u32;
            let owner = e.global::<u32>(FILE_MAP_OWNER);
            e.call(
                MODEL_LOADER_QUEUE_MODEL_LIST,
                &args![owner, list, 0u32, key, task, 0u32],
            );
            e.call(LIST_CLEAR, &args![list]);
            if list != 0 {
                e.call(LIST_DELETE, &args![list, 1u32]);
            }
        } else if e.call(FORM_TYPE, &args![item]).u32() == 0x2b {
            // The animation part of the item is at +0xdc.
            let animations = item + 0xdc;
            let key = fn_0043cc60(e, task.cast()) as u32;
            let owner = e.global::<u32>(FILE_MAP_OWNER);
            e.call(
                MODEL_LOADER_QUEUE_ANIMATIONS,
                &args![owner, animations, key, task, 0u32, 1u32, 1u32],
            );
            let lod_mult = e.call(GET_LOD_MULT, &args![item]).u32();
            queue_model(e, animations, task, lod_mult);
        }
    }
}

// Translated from 0043e080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedMagicItem::CheckFinished` (Xbox PDB): once the task's state is 4 or
/// more and all its children are finished ([`fn_0043caa0`]), tells the magic
/// item it has finished loading (`MagicItem::FinishedLoading`) and, when the
/// task has no references left, deletes it (virtual destructor, flag 1).
pub fn queued_magic_item_check_finished(e: &mut Engine, this: Ptr<QueuedMagicItem>) {
    if e.call(TASK_STATE_AT_LEAST_4, &args![this]).bool() && fn_0043caa0(e, this.cast()) {
        let magic_item = e.get(this, QueuedMagicItem::pMagicItem);
        e.call(MAGIC_ITEM_FINISHED_LOADING, &args![magic_item]);
        if e.call(FIELD_AT_8, &args![this]).u32() == 0 && !this.is_null() {
            e.vcall(this.addr(), 0, &args![1u32]);
        }
    }
}

// Translated from 0043e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedKF::QueuedKF` (Xbox PDB): the `QueuedFileEntry` constructor with
/// `context`, the KF's virtual table, no model, no flags, then the file name
/// is copied (`name`) and the file entry looked up (flag 0). Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn queued_kf_queued_kf(
    e: &mut Engine,
    this: Ptr<QueuedKF>,
    name: u32,
    context: u32,
) -> Ptr<QueuedKF> {
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_KF_VTABLE);
    e.call(KF_POINTER_CONSTRUCT, &args![kf_model_pointer(this), 0u32]);
    e.set(this, QueuedKF::cFlags, 0);
    e.call(QUEUED_FILE_ENTRY_SET_FILE_NAME, &args![this, name]);
    e.call(QUEUED_FILE_ENTRY_FIND_FILE_ENTRY, &args![this, 0u32]);
    this
}

// Translated from 0043e180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedKF` scalar deleting destructor (no name in the engine map): runs
/// the destructor ([`fn_0043e1b0`]) and, when bit 0 of `flags` is set, frees
/// the object. Returns `this`.
pub fn fn_0043e180(e: &mut Engine, this: Ptr<QueuedKF>, flags: u32) -> Ptr<QueuedKF> {
    fn_0043e1b0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043e1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedKF` destructor (no name in the engine map; the decompiler's library
/// match names it after a cancellation-token callback): releases the KF
/// model pointer and runs the `QueuedFileEntry` destructor. It does not
/// restore the virtual table.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043e1b0(e: &mut Engine, this: Ptr<QueuedKF>) {
    e.call(KF_POINTER_RELEASE, &args![kf_model_pointer(this)]);
    e.call(QUEUED_FILE_ENTRY_DESTRUCT, &args![this]);
}

// Translated from 0043e210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedKF` constructor taking a loaded model (no name in the engine map):
/// the `QueuedFileEntry` constructor with `context`, the KF's virtual table,
/// no flags, `spKFModel` set to `model`, and the task put into state 5
/// (done). Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043e210(e: &mut Engine, this: Ptr<QueuedKF>, model: Ptr, context: u32) -> Ptr<QueuedKF> {
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_KF_VTABLE);
    e.call(KF_POINTER_CONSTRUCT, &args![kf_model_pointer(this), 0u32]);
    e.set(this, QueuedKF::cFlags, 0);
    e.call(KF_POINTER_ASSIGN, &args![kf_model_pointer(this), model]);
    e.call(TASK_SET_DONE, &args![this]);
    this
}

// Translated from 0043e2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedKF::Run` (Xbox PDB): inside the memory context `0x33` (source line
/// `0x529`), takes the loader's `KFModel` of the file name into `spKFModel`;
/// when there is one, bit 1 of `cFlags` is set ([`fn_0043e480`]) and that is
/// all. Otherwise the file is opened (`QueuedFileEntry::GetFile`) and read
/// into a `BSStream` on the stack, and a `KFModel` built from the stream is
/// stored in `spKFModel`; a failure is logged.
///
/// The exception-unwinding frame and the stack-cookie check are not
/// translated.
pub fn queued_kf_run(e: &mut Engine, this: Ptr<QueuedKF>) {
    // The scope guard of the game's stack frame (4 bytes).
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![
                guard,
                KF_CONTEXT,
                1u32,
                MODEL_LOADER_SOURCE,
                KF_RUN_SOURCE_LINE
            ],
        );
        queued_kf_run_in_context(e, this);
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

/// The body of [`queued_kf_run`] inside its memory context.
pub(crate) fn queued_kf_run_in_context(e: &mut Engine, this: Ptr<QueuedKF>) {
    let slot = kf_model_pointer(this);
    let name = kf_file_name(e, this);
    let owner = e.global::<u32>(FILE_MAP_OWNER);
    let found = e.call(FIND_KF_MODEL, &args![owner, name]).u32();
    e.call(KF_POINTER_ASSIGN, &args![slot, found]);
    if !ni_pointer_get(e, slot).is_null() {
        fn_0043e480(e, this, 1);
        return;
    }
    let file: Ptr = e
        .call(QUEUED_FILE_ENTRY_GET_FILE, &args![this, 0u32, 1u32])
        .ptr();
    e.with_stack(BSStream::SIZE, |e, stream| {
        fn_0043cfd0(e, stream.cast());
        let mut loaded = false;
        if !file.is_null() {
            let name = kf_file_name(e, this);
            loaded = e.call(BS_STREAM_LOAD, &args![stream, name, file]).bool();
        }
        if loaded {
            let block = e.call(MEMORY_ALLOC, &args![0x14u32]).u32();
            let mut model = 0;
            if block != 0 {
                let name = kf_file_name(e, this);
                model = kf_model_kf_model(e, Ptr::new(block), name, stream).addr();
            }
            e.call(KF_POINTER_ASSIGN, &args![slot, model]);
        } else {
            let verb = if file.is_null() { FIND_WORD } else { LOAD_WORD };
            let name = kf_file_name(e, this);
            e.call(LOG, &args![MODEL_ERROR_MESSAGE, verb, name]);
        }
        bs_stream_destructor(e, stream.cast());
    });
}

// Translated from 0043e480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 1 of the `cFlags` byte at `this +
/// 0x34` (a `QueuedKF`'s, and a `QueuedTexture`'s, which is at the same
/// place).
pub fn fn_0043e480(e: &mut Engine, this: Ptr<QueuedKF>, flag: u8) {
    let flags = this.byte_add(QueuedKF::cFlags.off);
    e.call(SET_FLAG_BIT_1, &args![flag as u32, flags]);
}

// Translated from 0043e4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedKF::Finish` (Xbox PDB): with a loaded KF model whose bit 1 of
/// `cFlags` is clear (the loader did not already have it), the model is added
/// to the loader's map by file name; when the map already has one of that
/// name, `spKFModel` is replaced by it. Last the task's virtual function
/// `0x28` (`CheckFinished`) is called.
pub fn queued_kf_finish(e: &mut Engine, this: Ptr<QueuedKF>) {
    let slot = kf_model_pointer(this);
    if !ni_pointer_get(e, slot).is_null() && !fn_0043e530(e, this) {
        let model = ni_pointer_get(e, slot);
        let name = kf_file_name(e, this);
        let owner = e.global::<u32>(FILE_MAP_OWNER);
        if !e.call(ADD_KF_MODEL, &args![owner, name, model]).bool() {
            let name = kf_file_name(e, this);
            let found = e.call(FIND_KF_MODEL, &args![owner, name]).u32();
            e.call(KF_POINTER_ASSIGN, &args![slot, found]);
        }
    }
    fn_0043c610(e, this.cast());
}

// Translated from 0043e530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 of the `cFlags` byte at `this + 0x34` is set.
pub fn fn_0043e530(e: &mut Engine, this: Ptr<QueuedKF>) -> bool {
    let flags = e.get(this, QueuedKF::cFlags);
    e.call(TEST_FLAG_BIT_1, &args![flags as u32]).bool()
}

// Translated from 0043e550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedKF::GetDescription` (Xbox PDB): `QueuedFileEntry::GetDescription`
/// with the kind word `"KF"`; returns its result.
pub fn queued_kf_get_description(
    e: &mut Engine,
    this: Ptr<QueuedKF>,
    buffer: u32,
    size: u32,
) -> bool {
    e.call(
        QUEUED_FILE_ENTRY_GET_DESCRIPTION,
        &args![this, buffer, size, KF_WORD],
    )
    .bool()
}

// Translated from 0043e580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedAnimIdle` constructor (no name in the engine map): the `QueuedKF`
/// constructor ([`queued_kf_queued_kf`]) with `name` and `context`, the anim
/// idle's virtual table, `pQueuedForRef` and the `NiPointer<AnimIdle>`
/// constructed from `anim_idle`. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043e580(
    e: &mut Engine,
    this: Ptr<QueuedAnimIdle>,
    name: u32,
    context: u32,
    anim_idle: Ptr,
    reference: Ptr,
) -> Ptr<QueuedAnimIdle> {
    queued_kf_queued_kf(e, this.cast(), name, context);
    e.mem.set_u32(this.addr(), QUEUED_ANIM_IDLE_VTABLE);
    e.set(this, QueuedAnimIdle::pQueuedForRef, reference);
    let idle = this.byte_add(QueuedAnimIdle::spAnimIdle.off);
    e.call(NI_POINTER_CONSTRUCT, &args![idle, anim_idle]);
    this
}

// Translated from 0043e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedAnimIdle::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_0043e630`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn queued_anim_idle_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<QueuedAnimIdle>,
    flags: u32,
) -> Ptr<QueuedAnimIdle> {
    fn_0043e630(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043e630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedAnimIdle` destructor (no name in the engine map): destroys the
/// `NiPointer<AnimIdle>` and runs the `QueuedKF` destructor
/// ([`fn_0043e1b0`]).
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043e630(e: &mut Engine, this: Ptr<QueuedAnimIdle>) {
    let idle = this.byte_add(QueuedAnimIdle::spAnimIdle.off);
    e.call(NI_POINTER_DESTRUCT, &args![idle]);
    fn_0043e1b0(e, this.cast());
}

// Translated from 0043e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedAnimIdle::QueueMe` (Xbox PDB): with an anim idle, queues the model
/// of each of its two animation objects (`AnimIdle + 0x1c`, [`fn_0043e750`])
/// that exists: the object's `TESModel` part (at +0x18) with this task's key
/// and as the parent, LOD multiplier 2. Then `QueuedKF::QueueMe`.
pub fn queued_anim_idle_queue_me(e: &mut Engine, this: Ptr<QueuedAnimIdle>) {
    let slot = this.byte_add(QueuedAnimIdle::spAnimIdle.off);
    if !ni_pointer_get(e, slot).is_null() {
        for index in 0..2u32 {
            let idle = ni_pointer_get(e, slot);
            if fn_0043e750(e, idle, index) != 0 {
                let idle = ni_pointer_get(e, slot);
                let animation = fn_0043e750(e, idle, index);
                let model = if animation == 0 { 0 } else { animation + 0x18 };
                queue_model(e, model, this.cast(), 2);
            }
        }
    }
    e.call(QUEUED_KF_QUEUE_ME, &args![this]);
}

// Translated from 0043e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimIdle` accessor (no name in the engine map): the animation object
/// `index` of the two at `this + 0x1c` (`pAnimObj`, Xbox PDB).
pub fn fn_0043e750(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.mem.u32(this.addr() + 0x1c + index * 4)
}

// Translated from 0043e770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedAnimIdle::CheckFinished` (Xbox PDB): once the task's state is 4 or
/// more and all its children are finished ([`fn_0043caa0`]), the task is added
/// to the post-process queue ([`iomanager_add_post_process_task`]).
pub fn queued_anim_idle_check_finished(e: &mut Engine, this: Ptr<QueuedAnimIdle>) {
    if e.call(TASK_STATE_AT_LEAST_4, &args![this]).bool() && fn_0043caa0(e, this.cast()) {
        iomanager_add_post_process_task(e, this.cast());
    }
}

// Translated from 0043e7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedAnimIdle::PostProcess` (Xbox PDB): with a loaded KF model, and
/// unless the task's state is 6, tells the anim idle it has loaded
/// (`AnimIdle::Loaded` with the model). Then the loader drops the anim idle
/// from its map of queued anim idles ([`fn_0043e810`]).
pub fn queued_anim_idle_post_process(e: &mut Engine, this: Ptr<QueuedAnimIdle>) {
    let model_slot = this.byte_add(QueuedAnimIdle::spKFModel.off);
    let idle_slot = this.byte_add(QueuedAnimIdle::spAnimIdle.off);
    if !ni_pointer_get(e, model_slot).is_null() && !e.call(TASK_STATE_IS_6, &args![this]).bool() {
        let model = ni_pointer_get(e, model_slot);
        let idle = ni_pointer_get(e, idle_slot);
        e.call(ANIM_IDLE_LOADED, &args![idle, model]);
    }
    let idle = ni_pointer_get(e, idle_slot);
    let owner = e.global::<u32>(FILE_MAP_OWNER);
    fn_0043e810(e, Ptr::new(owner), idle);
}

// Translated from 0043e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map) on the loader at
/// `this`: calls the virtual function `0x14` of its `pQueuedAnimIdleMap`
/// (`this + 0x10`, a `LockFreeMap<AnimIdle*, ...>` in the Xbox PDB) with
/// `idle`. What that slot does is not confirmed; both callers use it when
/// the task is over.
pub fn fn_0043e810(e: &mut Engine, this: Ptr, idle: Ptr) {
    let map = e.mem.u32(this.addr() + 0x10);
    e.vcall(map, 0x14, &args![idle]);
}

// Translated from 0043e840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedAnimIdle::Cancel` (Xbox PDB): `QueuedFile::Cancel` with its two
/// arguments, then the loader's map of queued anim idles is told about the
/// anim idle ([`fn_0043e810`]).
pub fn queued_anim_idle_cancel(e: &mut Engine, this: Ptr<QueuedAnimIdle>, first: u32, second: u32) {
    e.call(QUEUED_FILE_CANCEL, &args![this, first, second]);
    let idle_slot = this.byte_add(QueuedAnimIdle::spAnimIdle.off);
    let idle = ni_pointer_get(e, idle_slot);
    let owner = e.global::<u32>(FILE_MAP_OWNER);
    fn_0043e810(e, Ptr::new(owner), idle);
}

// Translated from 0043e880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedAnimIdle::GetDescription` (Xbox PDB):
/// `QueuedFileEntry::GetDescription` with the kind word `"AnimIdle"`;
/// returns its result.
pub fn queued_anim_idle_get_description(
    e: &mut Engine,
    this: Ptr<QueuedAnimIdle>,
    buffer: u32,
    size: u32,
) -> bool {
    e.call(
        QUEUED_FILE_ENTRY_GET_DESCRIPTION,
        &args![this, buffer, size, ANIM_IDLE_WORD],
    )
    .bool()
}

// Translated from 0043e8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKF` constructor (no name in the engine map): the
/// `QueuedKF` constructor ([`queued_kf_queued_kf`]) with `name` and
/// `context`, the replacement KF's virtual table, `pAnim` and `pOwner`.
/// Returns `this`.
pub fn fn_0043e8b0(
    e: &mut Engine,
    this: Ptr<QueuedReplacementKF>,
    name: u32,
    context: u32,
    animation: Ptr,
    owner: Ptr,
) -> Ptr<QueuedReplacementKF> {
    queued_kf_queued_kf(e, this.cast(), name, context);
    e.mem.set_u32(this.addr(), QUEUED_REPLACEMENT_KF_VTABLE);
    e.set(this, QueuedReplacementKF::pAnim, animation);
    e.set(this, QueuedReplacementKF::pOwner, owner);
    this
}

// Translated from 0043e8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKF` scalar deleting destructor (no name in the engine
/// map): runs the destructor ([`fn_0043e920`]) and, when bit 0 of `flags` is
/// set, frees the object. Returns `this`.
pub fn fn_0043e8f0(
    e: &mut Engine,
    this: Ptr<QueuedReplacementKF>,
    flags: u32,
) -> Ptr<QueuedReplacementKF> {
    fn_0043e920(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043e920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKF` destructor (no name in the engine map): the
/// `QueuedKF` destructor ([`fn_0043e1b0`]).
pub fn fn_0043e920(e: &mut Engine, this: Ptr<QueuedReplacementKF>) {
    fn_0043e1b0(e, this.cast());
}

// Translated from 0043e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKF::Finish` (Xbox PDB): `QueuedKF::Finish`; then, with a
/// loaded KF model and unless the task's state is 6, the owning list (when
/// there is one) counts one more child ([`fn_0043e990`]) and the task is
/// added to the post-process queue ([`iomanager_add_post_process_task`]).
pub fn queued_replacement_kf_finish(e: &mut Engine, this: Ptr<QueuedReplacementKF>) {
    queued_kf_finish(e, this.cast());
    let model_slot = this.byte_add(QueuedReplacementKF::spKFModel.off);
    if !ni_pointer_get(e, model_slot).is_null() && !e.call(TASK_STATE_IS_6, &args![this]).bool() {
        let owner = e.get(this, QueuedReplacementKF::pOwner);
        if !owner.is_null() {
            fn_0043e990(e, owner.cast());
        }
        iomanager_add_post_process_task(e, this.cast());
    }
}

// Translated from 0043e990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKFList` method (no name in the engine map): one more
/// `iPostProcessingChildCount`.
pub fn fn_0043e990(e: &mut Engine, this: Ptr<QueuedReplacementKFList>) {
    let count = e.get(this, QueuedReplacementKFList::iPostProcessingChildCount);
    e.set(
        this,
        QueuedReplacementKFList::iPostProcessingChildCount,
        count.wrapping_add(1),
    );
}

// Translated from 0043e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKF::PostProcess` (Xbox PDB): unless the task's state is
/// 6, hands the loaded KF model to the animation (`pAnim`, `00490fa0`) and,
/// when there is an owning list, tells it a child was post-processed
/// ([`fn_0043ea30`]).
pub fn queued_replacement_kf_post_process(e: &mut Engine, this: Ptr<QueuedReplacementKF>) {
    if !e.call(TASK_STATE_IS_6, &args![this]).bool() {
        let model_slot = this.byte_add(QueuedReplacementKF::spKFModel.off);
        let model = ni_pointer_get(e, model_slot);
        let animation = e.get(this, QueuedReplacementKF::pAnim);
        e.call(ANIMATION_KF_LOADED, &args![animation, model]);
        let owner = e.get(this, QueuedReplacementKF::pOwner);
        if !owner.is_null() {
            fn_0043ea30(e, owner.cast());
        }
    }
}

// ---------------------------------------------------------------------------
// The fifth session: the rest of the replacement KFs, `QueuedHead`,
// `QueuedHelmet`, the distant-3D task and `QueuedReference`, up to
// `0043fe40`.

/// `"Replacement KF"`: the kind word of `QueuedReplacementKF::GetDescription`.
pub(crate) const REPLACEMENT_KF_WORD: u32 = 0x0101_6af0;
/// `QueuedHead`'s (`01016b04`), `QueuedHelmet`'s (`01016b4c`) and
/// `QueuedReference`'s (`01016ba4`) virtual tables.
pub(crate) const QUEUED_HEAD_VTABLE: u32 = 0x0101_6b04;
pub(crate) const QUEUED_HELMET_VTABLE: u32 = 0x0101_6b4c;
pub(crate) const QUEUED_REFERENCE_VTABLE: u32 = 0x0101_6ba4;
/// `"Queued head for NPC %s"` and `"Queued helmet for biped anim %08X"`.
pub(crate) const QUEUED_HEAD_FORMAT: u32 = 0x0101_6b30;
pub(crate) const QUEUED_HELMET_FORMAT: u32 = 0x0101_6b7c;
/// The memory contexts and source lines of the scopes of `QueuedHead::Run`
/// (`0x37`, line `0x5f9`), `0043ed90` (`0x37`, line `0x613`) and
/// `QueuedHelmet::CheckFinished` (`0x32`, line `0x690`).
pub(crate) const HEAD_CONTEXT: u32 = 0x37;
pub(crate) const HEAD_RUN_SOURCE_LINE: u32 = 0x5f9;
pub(crate) const HEAD_SETUP_SOURCE_LINE: u32 = 0x613;
pub(crate) const HELMET_CONTEXT: u32 = 0x32;
pub(crate) const HELMET_CHECK_FINISHED_SOURCE_LINE: u32 = 0x690;
/// The number of entries in each of the three arrays of `NiPointer`s of a
/// `QueuedHelmet` (`0x14`, the biped slots), and the pointer size.
pub(crate) const HELMET_SLOT_COUNT: u32 = 0x14;
/// Offsets of the three arrays in a `QueuedHelmet` (Xbox PDB:
/// `spQueuedHelmetModel`, `spHelmetFaceGenModel`, `spClonedHelmetNode`).
pub(crate) const HELMET_MODELS_OFFSET: u32 = 0x2c;
pub(crate) const HELMET_FACE_GEN_MODELS_OFFSET: u32 = 0x7c;
pub(crate) const HELMET_CLONED_NODES_OFFSET: u32 = 0xcc;
/// `QueuedFile::spParent` (Xbox PDB) in every queued file.
pub(crate) const QUEUED_FILE_PARENT_OFFSET: u32 = 0x1c;

/// `sprintf_s(buffer, size, format, ...)` through the game's wrapper
/// (`00406d00`), `__cdecl`.
pub(crate) const FORMAT_STRING: u32 = 0x0040_6d00;
/// Returns the string of the name object it is called on (`00408da0`, named
/// `MapMarkerData::GetLocationName` in the engine map): the string held by
/// the `NiPointer` at +4, or the empty string at `01011584`.
pub(crate) const NAME_OBJECT_STRING: u32 = 0x0040_8da0;
/// Enters (`00652140`) and leaves (`00652190`) a section of the face
/// generation code: the first calls `004538e0(0)` on the object at
/// `011d5a9c` and counts `011d59dc` up, the second calls `0082f1f0` on it
/// and counts down. Neither takes an argument.
pub(crate) const FACE_GEN_SECTION_ENTER: u32 = 0x0065_2140;
pub(crate) const FACE_GEN_SECTION_LEAVE: u32 = 0x0065_2190;
/// `TESNPC::InitHead` (Xbox PDB, `00607370`), `__thiscall(biped node out,
/// skinned node out)`.
pub(crate) const NPC_INIT_HEAD: u32 = 0x0060_7370;
/// `TESNPC` method of `tesnpc.cpp` (`00607420`, no name in the engine map),
/// `__thiscall(reference, argument, biped node, skinned node)` on the base
/// form of the reference.
pub(crate) const NPC_SETUP_HEAD: u32 = 0x0060_7420;
/// `TESRace::KillEGTData` (Xbox PDB, `00613fd0`).
pub(crate) const KILL_EGT_DATA: u32 = 0x0061_3fd0;
/// Returns the dword at +0xC of the object it is called on (`0084e3a0`; for a
/// form, its form ID).
pub(crate) const FIELD_AT_C: u32 = 0x0084_e3a0;
/// A function of `bsfacegenmanager.cpp` (`00657150`, no name in the engine
/// map), `__cdecl(NPC, priority, task)`.
pub(crate) const QUEUE_NPC_FACE_GEN: u32 = 0x0065_7150;
/// `SetReferenceIDOnScenegraph(node, form ID)` (Xbox PDB, `004b6dc0`),
/// `__cdecl`.
pub(crate) const SET_REFERENCE_ID_ON_SCENEGRAPH: u32 = 0x004b_6dc0;
/// The base form of a reference (`007af430`, `this + 0x20`; named
/// `BGSSaveFormBuffer::GetForm` in the engine map).
pub(crate) const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
/// The `PlayerCharacter` singleton pointer and the `TES` singleton pointer.
pub(crate) const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
pub(crate) const TES_GLOBAL: u32 = 0x011d_ea10;

/// The `NiPointer` default constructors the vector constructors of
/// `QueuedHelmet` call: `0043f060` (a task pointer, in this unit) for the
/// models and `006694e0` (`NiPointer()`) for the face generation models and
/// the cloned nodes.
pub(crate) const HELMET_MODEL_POINTER_CONSTRUCT: u32 = 0x0043_f060;
pub(crate) const NI_POINTER_DEFAULT_CONSTRUCT: u32 = 0x0066_94e0;
/// `_eh_vector_constructor_iterator_(array, size, count, constructor,
/// destructor)` and `_eh_vector_destructor_iterator_(array, size, count,
/// destructor)`.
pub(crate) const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
pub(crate) const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The constructor and destructor of the 0x20-byte elements `QueuedHelmet`'s
/// `CheckFinished` builds four of on its stack (`00449610`, `00449680`).
pub(crate) const FACE_COORD_CONSTRUCT: u32 = 0x0044_9610;
pub(crate) const FACE_COORD_DESTRUCT: u32 = 0x0044_9680;

/// `BipedAnim` methods of `bipedanim.cpp`, all `__thiscall` on the
/// `BipedAnim`: `004ae8a0(index) -> bool` (whether the slot at `+0x30 +
/// 0x10 * index` holds an object that `004ae8f0` accepts),
/// `BipedAnim::LoadFaceGenModel(index)` (Xbox PDB), `BipedAnim::CloneHelmet(
/// face generation model, 3D, index)` (Xbox PDB) and
/// `BipedAnim::AttachHelmet(face generation model, cloned node, index)`
/// (Xbox PDB).
pub(crate) const BIPED_ANIM_SLOT_IN_USE: u32 = 0x004a_e8a0;
pub(crate) const BIPED_ANIM_LOAD_FACE_GEN_MODEL: u32 = 0x004a_e790;
pub(crate) const BIPED_ANIM_CLONE_HELMET: u32 = 0x004a_e910;
pub(crate) const BIPED_ANIM_ATTACH_HELMET: u32 = 0x004a_eb20;
/// `ModelLoader` queueing methods, `__thiscall` on the loader: `00443dc0(
/// model data, NiPointer<QueuedModel> slot, priority, parent task, 3, 1, 1,
/// 1, 0.0)` and `ModelLoader::QueueEGMFile(model data, priority, parent
/// task, -1)` (Xbox PDB).
pub(crate) const MODEL_LOADER_QUEUE_HELMET_MODEL: u32 = 0x0044_3dc0;
pub(crate) const MODEL_LOADER_QUEUE_EGM_FILE: u32 = 0x0044_7bf0;
/// `QueuedModel` method (`004a8a90`): the `NiPointer<Model>` at +0x30
/// dereferenced.
pub(crate) const QUEUED_MODEL_GET_MODEL: u32 = 0x004a_8a90;
/// A method of `tesobjectcell.cpp` (`005495f0`): the `NiPointer` at +0xB8 of
/// the object dereferenced.
pub(crate) const OBJECT_POINTER_AT_B8: u32 = 0x0054_95f0;
/// `NiObject::CreateDeepCopy` (Xbox PDB, `00a5d510`), `__thiscall(object,
/// NiPointer out)`.
pub(crate) const CREATE_DEEP_COPY: u32 = 0x00a5_d510;
/// `TESNPC::GetFaceCoord` (Xbox PDB, `00603ad0`), `__thiscall(NPC, array of
/// four 0x20-byte elements)`.
pub(crate) const NPC_GET_FACE_COORD: u32 = 0x0060_3ad0;
/// A method of `bsfacegenmodel.cpp` (`0065a970`, no name in the engine map),
/// `__thiscall(face generation model, coordinates, node, 0) -> bool`.
pub(crate) const FACE_GEN_MODEL_APPLY_COORDS: u32 = 0x0065_a970;
/// The object whose byte `0043faf0` reads (`011d5a44`; which setting it is
/// is not confirmed) and the function that gives the address of that byte
/// (`00408d60`, `this + 4`, or the zero byte at `01202800` when `this` is
/// null).
pub(crate) const HELMET_ROTATION_SETTING: u32 = 0x011d_5a44;
pub(crate) const SETTING_BYTE_POINTER: u32 = 0x0040_8d60;
/// `-pi / 2` as a `float` in the exe (`01016b78`): the angle
/// `QueuedHelmet::CheckFinished` turns a cloned node by.
pub(crate) const HELMET_ROTATION_ANGLE: u32 = 0x0101_6b78;
/// A function of `navmeshsearchflee.cpp`'s range (`006a9540`) that returns
/// `this + 0x34`, the local rotation matrix of an `NiAVObject`.
pub(crate) const LOCAL_ROTATE_ADDRESS: u32 = 0x006a_9540;
/// `NiMatrix3` default constructor (`006815c0`): returns `this`.
pub(crate) const MATRIX_CONSTRUCT: u32 = 0x0068_15c0;
/// `sin` and `cos` of an angle through the game's helper (`004169a0`),
/// `__cdecl(angle, sin out, cos out)` (`FSINCOS`).
pub(crate) const SIN_COS: u32 = 0x0041_69a0;
/// `TESObjectREFR` methods: `00570f70(node)` gives the reference its 3D,
/// `008d6f30()` returns its parent cell (`this + 0x40`), `TESActorBase::
/// SetStartsDead(flag)` (Xbox PDB, `00565210`) sets or clears bit
/// `0x80000` of its flags.
pub(crate) const REFERENCE_SET_3D: u32 = 0x0057_0f70;
pub(crate) const REFERENCE_CELL: u32 = 0x008d_6f30;
pub(crate) const SET_STARTS_DEAD: u32 = 0x0056_5210;
/// `TES` method of `tes.cpp` (`00451ef0`, in `tes.rs`): loads one reference
/// into the scene, `__thiscall(reference, cell, source, flag)`.
pub(crate) const TES_LOAD_REFERENCE: u32 = 0x0045_1ef0;
/// The TLS words `0043fcd0` reads (offsets in the thread's TLS block): the
/// reference being worked on (`0x264`) and the 3D that goes with it
/// (`0x260`).
pub(crate) const TLS_REFERENCE: u32 = 0x264;
pub(crate) const TLS_REFERENCE_3D: u32 = 0x260;

layout! {
    /// `QueuedHead` (Xbox PDB), 0x38 bytes: a `QueuedFile`, the NPC and the
    /// two face generation nodes `Run` builds for it.
    pub struct QueuedHead: 0x38 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pNPC` (Xbox PDB): `TESNPC*`.
        0x28 pNPC: Ptr,
        /// `spBipedNode` (Xbox PDB): `NiPointer<BSFaceGenNiNode>`.
        0x2C spBipedNode: Ptr,
        /// `spSkinnedNode` (Xbox PDB): `NiPointer<BSFaceGenNiNode>`.
        0x30 spSkinnedNode: Ptr,
    }

    /// `QueuedHelmet` (Xbox PDB), 0x128 bytes: a `QueuedFile`, the biped
    /// animation and three arrays of twenty `NiPointer`s (one per biped
    /// slot; [`HELMET_MODELS_OFFSET`], [`HELMET_FACE_GEN_MODELS_OFFSET`] and
    /// [`HELMET_CLONED_NODES_OFFSET`]).
    pub struct QueuedHelmet: 0x128 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pBipedAnim` (Xbox PDB): `BipedAnim*`.
        0x28 pBipedAnim: Ptr,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x11C pRef: Ptr,
        /// `cFlags` (Xbox PDB). Bit 1 is set by `QueueMe` once the models
        /// are queued.
        0x120 cFlags: u8,
    }

    /// `AttachDistant3DTask` (Xbox PDB), 0x20 bytes: an `IOTask` and the
    /// reference and node it attaches.
    pub struct AttachDistant3DTask: 0x20 {
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x18 pRef: Ptr,
        /// `spNode` (Xbox PDB): `NiPointer<NiNode>`.
        0x1C spNode: Ptr,
    }

    /// `QueuedReference` (Xbox PDB), 0x40 bytes: a `QueuedFile`, the
    /// reference and what is loaded for it.
    pub struct QueuedReference: 0x40 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x28 pRef: Ptr,
        /// `spQueuedModel` (Xbox PDB): `NiPointer<QueuedModel>`.
        0x2C spQueuedModel: Ptr,
        /// `spModel` (Xbox PDB): `NiPointer<Model>`.
        0x30 spModel: Ptr,
        /// `spCloned3D` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x34 spCloned3D: Ptr,
        /// `spAttachDistant3DTask` (Xbox PDB):
        /// `NiPointer<AttachDistant3DTask>`.
        0x38 spAttachDistant3DTask: Ptr,
    }
}

/// The address of entry `index` of the array of `NiPointer`s that starts at
/// `offset` in a `QueuedHelmet`.
pub(crate) fn helmet_slot(this: Ptr<QueuedHelmet>, offset: u32, index: u32) -> Ptr {
    this.byte_add(offset + 4 * index)
}

/// The address of `QueuedHelmet::cFlags`, which the game passes to the flag
/// helpers.
pub(crate) fn helmet_flags_pointer(this: Ptr<QueuedHelmet>) -> Ptr {
    this.byte_add(QueuedHelmet::cFlags.off)
}

// Translated from 0043ea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKF::GetDescription` (Xbox PDB):
/// `QueuedFileEntry::GetDescription` with the kind word `"Replacement KF"`;
/// returns its result.
pub fn queued_replacement_kf_get_description(
    e: &mut Engine,
    this: Ptr<QueuedReplacementKF>,
    buffer: u32,
    size: u32,
) -> bool {
    e.call(
        QUEUED_FILE_ENTRY_GET_DESCRIPTION,
        &args![this, buffer, size, REPLACEMENT_KF_WORD],
    )
    .bool()
}

// Translated from 0043ea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKFList` method (no name in the engine map): counts one
/// more `iPostProcessedChildCount`; when the task's state is 4 or more, its
/// children are all finished ([`fn_0043caa0`]) and that count has reached
/// `iPostProcessingChildCount`, the loader's map of queued replacement KF
/// lists is told about the animation ([`fn_0043ea90`]).
pub fn fn_0043ea30(e: &mut Engine, this: Ptr<QueuedReplacementKFList>) {
    let processed = e.get(this, QueuedReplacementKFList::iPostProcessedChildCount);
    e.set(
        this,
        QueuedReplacementKFList::iPostProcessedChildCount,
        processed.wrapping_add(1),
    );
    if e.call(TASK_STATE_AT_LEAST_4, &args![this]).bool() && fn_0043caa0(e, this.cast()) {
        let processing = e.get(this, QueuedReplacementKFList::iPostProcessingChildCount);
        let processed = e.get(this, QueuedReplacementKFList::iPostProcessedChildCount);
        if processing == processed {
            let animation = e.get(this, QueuedReplacementKFList::pAnim);
            let loader = e.global::<u32>(FILE_MAP_OWNER);
            fn_0043ea90(e, Ptr::new(loader), animation);
        }
    }
}

// Translated from 0043ea90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map) on the loader at
/// `this`: calls the virtual function `0x14` of its
/// `pQueuedReplacementKFListMap` (`this + 0x14`, a `LockFreeMap<Animation*,
/// ...>` in the Xbox PDB) with `animation`. What that slot does is not
/// confirmed; both callers use it when the list is over.
pub fn fn_0043ea90(e: &mut Engine, this: Ptr, animation: Ptr) {
    let map = e.mem.u32(this.addr() + 0x14);
    e.vcall(map, 0x14, &args![animation]);
}

// Translated from 0043eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKFList::Cancel` (Xbox PDB): `QueuedFile::Cancel` with
/// its two arguments, then the loader's map of queued replacement KF lists
/// is told about the animation ([`fn_0043ea90`]).
pub fn queued_replacement_kf_list_cancel(
    e: &mut Engine,
    this: Ptr<QueuedReplacementKFList>,
    first: u32,
    second: u32,
) {
    e.call(QUEUED_FILE_CANCEL, &args![this, first, second]);
    let animation = e.get(this, QueuedReplacementKFList::pAnim);
    let loader = e.global::<u32>(FILE_MAP_OWNER);
    fn_0043ea90(e, Ptr::new(loader), animation);
}

// Translated from 0043eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHead` constructor (no name in the engine map): the `QueuedFile`
/// constructor with `context`, the head's virtual table, `pNPC` and two empty
/// node pointers. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043eaf0(
    e: &mut Engine,
    this: Ptr<QueuedHead>,
    npc: Ptr,
    context: u32,
) -> Ptr<QueuedHead> {
    e.call(QUEUED_FILE_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_HEAD_VTABLE);
    e.set(this, QueuedHead::pNPC, npc);
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedHead::spBipedNode.off), 0u32],
    );
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedHead::spSkinnedNode.off), 0u32],
    );
    this
}

// Translated from 0043eb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHead::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_0043ebb0`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn queued_head_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<QueuedHead>,
    flags: u32,
) -> Ptr<QueuedHead> {
    fn_0043ebb0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043ebb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHead` destructor (no name in the engine map; the decompiler's
/// library match names it after a cancellation-token destructor): releases
/// the two node pointers and runs the `QueuedFile` destructor body. It does
/// not restore the virtual table.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043ebb0(e: &mut Engine, this: Ptr<QueuedHead>) {
    e.call(
        NI_POINTER_DESTRUCT,
        &args![this.byte_add(QueuedHead::spSkinnedNode.off)],
    );
    e.call(
        NI_POINTER_DESTRUCT,
        &args![this.byte_add(QueuedHead::spBipedNode.off)],
    );
    e.call(QUEUED_FILE_DESTRUCT, &args![this]);
}

// Translated from 0043ec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHead::QueueMe` (Xbox PDB): hands the NPC to the face generation
/// manager (`00657150(NPC, priority byte of the key, this)`), then calls the
/// virtual function `0x28` (`CheckFinished`).
pub fn queued_head_queue_me(e: &mut Engine, this: Ptr<QueuedHead>) {
    let key = fn_0043cc60(e, this.cast()) as u32;
    let npc = e.get(this, QueuedHead::pNPC);
    e.call(QUEUE_NPC_FACE_GEN, &args![npc, key, this]);
    e.vcall(this.addr(), 0x28, &args![]);
}

// Translated from 0043ec60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHead::CheckFinished` (Xbox PDB): when all children are finished
/// ([`fn_0043caa0`]), a task still in state 0 is handed to the task queue
/// (slot `0x48`), any other goes through `QueuedFile::CheckFinished`.
pub fn queued_head_check_finished(e: &mut Engine, this: Ptr<QueuedHead>) {
    if fn_0043caa0(e, this.cast()) {
        if e.call(STATE_IS_ZERO, &args![this]).bool() {
            let queue = e.global::<u32>(TASK_QUEUE);
            e.vcall(queue, 0x48, &args![this]);
        } else {
            e.call(QUEUED_FILE_CHECK_FINISHED, &args![this]);
        }
    }
}

// Translated from 0043ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHead::Run` (Xbox PDB): inside a face generation section and the
/// memory context of the head (`0x37`, source line `0x5f9`), `TESNPC::InitHead`
/// builds the two nodes, which are stored in `spBipedNode` and
/// `spSkinnedNode`; when the NPC's form ID is 7 the race's EGT data is killed
/// (`TESRace::KillEGTData`, race from the object at NPC `+0x10C`).
///
/// The exception-unwinding frame is not translated.
pub fn queued_head_run(e: &mut Engine, this: Ptr<QueuedHead>) {
    e.call(FACE_GEN_SECTION_ENTER, &args![]);
    // The scope guard of the game's stack frame (4 bytes).
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![
                guard,
                HEAD_CONTEXT,
                1u32,
                MODEL_LOADER_SOURCE,
                HEAD_RUN_SOURCE_LINE
            ],
        );
        // The two out parameters of `InitHead`, zero to begin with.
        e.with_stack(8, |e, nodes| {
            let biped_out = nodes;
            let skinned_out = nodes.byte_add(4);
            let npc = e.get(this, QueuedHead::pNPC);
            e.call(NPC_INIT_HEAD, &args![npc, biped_out, skinned_out]);
            let biped = e.mem.u32(biped_out.addr());
            e.call(
                NI_POINTER_ASSIGN,
                &args![this.byte_add(QueuedHead::spBipedNode.off), biped],
            );
            let skinned = e.mem.u32(skinned_out.addr());
            e.call(
                NI_POINTER_ASSIGN,
                &args![this.byte_add(QueuedHead::spSkinnedNode.off), skinned],
            );
        });
        let npc = e.get(this, QueuedHead::pNPC);
        if e.call(FIELD_AT_C, &args![npc]).u32() == 7 {
            let npc = e.get(this, QueuedHead::pNPC);
            // The object embedded in the NPC at +0x10C; its dword at +4 is
            // the race.
            let race = e.call(FIELD_AT_4, &args![npc.byte_add(0x10c)]).u32();
            e.call(KILL_EGT_DATA, &args![race]);
        }
        e.call(FACE_GEN_SECTION_LEAVE, &args![]);
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

// Translated from 0043ed90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHead` method (no name in the engine map): inside the memory
/// context of the head (`0x37`, source line `0x613`), the NPC's base form
/// of `reference` gets `reference`, `argument` and the two nodes
/// (`00607420`). Unless `reference` is the player, both nodes are given the
/// form ID of `reference` (`SetReferenceIDOnScenegraph`); then the dword at
/// +0xE8 of each node that exists is set to `reference`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043ed90(e: &mut Engine, this: Ptr<QueuedHead>, reference: Ptr, argument: u32) {
    let biped_slot = this.byte_add(QueuedHead::spBipedNode.off);
    let skinned_slot = this.byte_add(QueuedHead::spSkinnedNode.off);
    // The scope guard of the game's stack frame (4 bytes).
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![
                guard,
                HEAD_CONTEXT,
                1u32,
                MODEL_LOADER_SOURCE,
                HEAD_SETUP_SOURCE_LINE
            ],
        );
        let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
        let skinned = ni_pointer_get(e, skinned_slot);
        let biped = ni_pointer_get(e, biped_slot);
        e.call(
            NPC_SETUP_HEAD,
            &args![base_form, reference, argument, biped, skinned],
        );
        if reference.addr() != e.global::<u32>(PLAYER_CHARACTER) {
            let form_id = e.call(FIELD_AT_C, &args![reference]).u32();
            let biped = ni_pointer_get(e, biped_slot);
            e.call(SET_REFERENCE_ID_ON_SCENEGRAPH, &args![biped, form_id]);
            if !ni_pointer_get(e, skinned_slot).is_null() {
                let form_id = e.call(FIELD_AT_C, &args![reference]).u32();
                let skinned = ni_pointer_get(e, skinned_slot);
                e.call(SET_REFERENCE_ID_ON_SCENEGRAPH, &args![skinned, form_id]);
            }
        }
        if !ni_pointer_get(e, skinned_slot).is_null() {
            let skinned = ni_pointer_get(e, skinned_slot);
            // The dword at +0xE8 of the face generation node.
            e.mem.set_u32(skinned.addr() + 0xe8, reference.addr());
        }
        if !ni_pointer_get(e, biped_slot).is_null() {
            let biped = ni_pointer_get(e, biped_slot);
            e.mem.set_u32(biped.addr() + 0xe8, reference.addr());
        }
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

// Translated from 0043eed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHead::GetDescription` (Xbox PDB): writes `"Queued head for NPC
/// %s"` with the name of the NPC (the name object at NPC `+0xD0`,
/// `00408da0`) into `buffer` (`size` bytes); returns true.
pub fn queued_head_get_description(
    e: &mut Engine,
    this: Ptr<QueuedHead>,
    buffer: u32,
    size: u32,
) -> bool {
    let npc = e.get(this, QueuedHead::pNPC);
    let name = e.call(NAME_OBJECT_STRING, &args![npc.byte_add(0xd0)]).u32();
    e.call(
        FORMAT_STRING,
        &args![buffer, size, QUEUED_HEAD_FORMAT, name],
    );
    true
}

// Translated from 0043ef10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet` constructor (no name in the engine map): the `QueuedFile`
/// constructor with `context`, the helmet's virtual table, `pBipedAnim`, the
/// three arrays of twenty `NiPointer`s (vector constructors), no flags, and
/// `pRef` from the biped animation ([`fn_0043f010`]). Returns `this`. The
/// third argument word is never read.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043ef10(
    e: &mut Engine,
    this: Ptr<QueuedHelmet>,
    biped_anim: Ptr,
    context: u32,
    _unused_3: u32,
) -> Ptr<QueuedHelmet> {
    e.call(QUEUED_FILE_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_HELMET_VTABLE);
    e.set(this, QueuedHelmet::pBipedAnim, biped_anim);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            helmet_slot(this, HELMET_MODELS_OFFSET, 0),
            4u32,
            HELMET_SLOT_COUNT,
            HELMET_MODEL_POINTER_CONSTRUCT,
            TASK_POINTER_DESTRUCT
        ],
    );
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            helmet_slot(this, HELMET_FACE_GEN_MODELS_OFFSET, 0),
            4u32,
            HELMET_SLOT_COUNT,
            NI_POINTER_DEFAULT_CONSTRUCT,
            NI_POINTER_DESTRUCT
        ],
    );
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            helmet_slot(this, HELMET_CLONED_NODES_OFFSET, 0),
            4u32,
            HELMET_SLOT_COUNT,
            NI_POINTER_DEFAULT_CONSTRUCT,
            NI_POINTER_DESTRUCT
        ],
    );
    e.set(this, QueuedHelmet::cFlags, 0);
    let reference = fn_0043f010(e, biped_anim);
    e.set(this, QueuedHelmet::pRef, reference);
    this
}

// Translated from 0043eff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet::PostProcess` (Xbox PDB): `QueuedHelmet::Attach`
/// ([`queued_helmet_attach`]).
pub fn queued_helmet_post_process(e: &mut Engine, this: Ptr<QueuedHelmet>) {
    queued_helmet_attach(e, this);
}

// Translated from 0043f010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim` getter (no name in the engine map): the dword at +0x2B0, the
/// reference the animation belongs to.
pub fn fn_0043f010(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x2b0))
}

// Translated from 0043f030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_0043f080`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn queued_helmet_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<QueuedHelmet>,
    flags: u32,
) -> Ptr<QueuedHelmet> {
    fn_0043f080(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043f060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default constructor of the task pointers in `QueuedHelmet`'s first array
/// (no name in the engine map): the task pointer constructor with a null
/// task.
pub fn fn_0043f060(e: &mut Engine, this: Ptr) {
    e.call(TASK_POINTER_CONSTRUCT, &args![this, 0u32]);
}

// Translated from 0043f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet` destructor (no name in the engine map; the decompiler's
/// library match names it after a cancellation-token destructor): destroys
/// the three arrays of twenty `NiPointer`s, last first (vector destructors),
/// and runs the `QueuedFile` destructor body. It does not restore the
/// virtual table.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043f080(e: &mut Engine, this: Ptr<QueuedHelmet>) {
    e.call(
        VECTOR_DESTRUCT,
        &args![
            helmet_slot(this, HELMET_CLONED_NODES_OFFSET, 0),
            4u32,
            HELMET_SLOT_COUNT,
            NI_POINTER_DESTRUCT
        ],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![
            helmet_slot(this, HELMET_FACE_GEN_MODELS_OFFSET, 0),
            4u32,
            HELMET_SLOT_COUNT,
            NI_POINTER_DESTRUCT
        ],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![
            helmet_slot(this, HELMET_MODELS_OFFSET, 0),
            4u32,
            HELMET_SLOT_COUNT,
            TASK_POINTER_DESTRUCT
        ],
    );
    e.call(QUEUED_FILE_DESTRUCT, &args![this]);
}

// Translated from 0043f120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet::QueueMe` (Xbox PDB): for each of the twenty biped slots
/// that is in use, the loader queues the slot's model data into
/// `spQueuedHelmetModel[slot]` (`00443dc0`, with the task's priority byte,
/// `this` as the parent, LOD multiplier 3 and the constant flags the code
/// pushes) and its EGM file (`ModelLoader::QueueEGMFile`, priority byte,
/// `this`, -1). Then bit 1 of `cFlags` is set ([`fn_0043f1f0`]) and the
/// virtual function `0x28` (`CheckFinished`) is called.
pub fn queued_helmet_queue_me(e: &mut Engine, this: Ptr<QueuedHelmet>) {
    for index in 0..HELMET_SLOT_COUNT {
        let biped = e.get(this, QueuedHelmet::pBipedAnim);
        if e.call(BIPED_ANIM_SLOT_IN_USE, &args![biped, index]).bool() {
            let biped = e.get(this, QueuedHelmet::pBipedAnim);
            let entry = fn_0043f220(e, biped, index);
            // The dword at +4 of the biped's slot entry.
            let model_data = e.mem.u32(entry.addr() + 4);
            let loader = e.global::<u32>(FILE_MAP_OWNER);
            let key = fn_0043cc60(e, this.cast()) as u32;
            e.call(
                MODEL_LOADER_QUEUE_HELMET_MODEL,
                &args![
                    loader,
                    model_data,
                    helmet_slot(this, HELMET_MODELS_OFFSET, index),
                    key,
                    this,
                    3u32,
                    1u32,
                    1u32,
                    1u32,
                    0.0f32
                ],
            );
            let key = fn_0043cc60(e, this.cast()) as u32;
            e.call(
                MODEL_LOADER_QUEUE_EGM_FILE,
                &args![loader, model_data, key, this, 0xffff_ffffu32],
            );
        }
    }
    fn_0043f1f0(e, this, 1);
    e.vcall(this.addr(), 0x28, &args![]);
}

// Translated from 0043f1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 1 of the helmet's `cFlags`.
pub fn fn_0043f1f0(e: &mut Engine, this: Ptr<QueuedHelmet>, flag: u8) {
    let flags = helmet_flags_pointer(this);
    e.call(SET_FLAG_BIT_1, &args![flag as u32, flags]);
}

// Translated from 0043f220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim` method (no name in the engine map): the address of the
/// 0x10-byte entry of biped slot `index`, `this + 0x2C + 0x10 * index`.
pub fn fn_0043f220(_e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    this.byte_add(0x2c + (index << 4))
}

// Translated from 0043f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet::Run` (Xbox PDB): for each of the twenty biped slots that
/// is in use, while the task's state is not 6, the face generation model of
/// the slot is loaded (`BipedAnim::LoadFaceGenModel`) into
/// `spHelmetFaceGenModel[slot]`.
pub fn queued_helmet_run(e: &mut Engine, this: Ptr<QueuedHelmet>) {
    for index in 0..HELMET_SLOT_COUNT {
        let biped = e.get(this, QueuedHelmet::pBipedAnim);
        if e.call(BIPED_ANIM_SLOT_IN_USE, &args![biped, index]).bool()
            && !e.call(TASK_STATE_IS_6, &args![this]).bool()
        {
            let biped = e.get(this, QueuedHelmet::pBipedAnim);
            let model = e
                .call(BIPED_ANIM_LOAD_FACE_GEN_MODEL, &args![biped, index])
                .u32();
            e.call(
                NI_POINTER_ASSIGN,
                &args![
                    helmet_slot(this, HELMET_FACE_GEN_MODELS_OFFSET, index),
                    model
                ],
            );
        }
    }
}

// Translated from 0043f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet::Finish` (Xbox PDB): a task without a parent whose state is
/// not 6 is added to the post-process queue
/// ([`iomanager_add_post_process_task`]); then the virtual function `0x28`
/// (`CheckFinished`) is called ([`fn_0043c610`]).
pub fn queued_helmet_finish(e: &mut Engine, this: Ptr<QueuedHelmet>) {
    let parent = ni_pointer_get(e, this.byte_add(QUEUED_FILE_PARENT_OFFSET));
    if parent.is_null() && !e.call(TASK_STATE_IS_6, &args![this]).bool() {
        iomanager_add_post_process_task(e, this.cast());
    }
    fn_0043c610(e, this.cast());
}

// Translated from 0043f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet::CheckFinished` (Xbox PDB). Without bit 1 of `cFlags` (set
/// once the models are queued) it is `QueuedFile::CheckFinished`. With it, a
/// task whose children are all finished and whose state is 0 goes to the task
/// queue (slot `0x48`). Any other task, inside the memory context `0x32`
/// (source line `0x690`): if its state is 4 or more, the children are
/// finished, the state is not 6 and the reference is not the player, every
/// biped slot with a loaded model gets a cloned node
/// (`BipedAnim::CloneHelmet`, stored in `spClonedHelmetNode[slot]`); unless
/// the clone already has a child with a set pointer at +0xBC, the face
/// generation model is taken (loaded when missing) and, for each child of
/// the clone that has a deep-copyable part and belongs to an NPC reference
/// (form type 0x2a), the part is deep-copied and given to the child (virtual
/// function `0xE4`), the NPC's face coordinates are read
/// (`TESNPC::GetFaceCoord`) and, when the setting byte [`fn_0043faf0`] is
/// clear or the face generation model accepts them (`0065a970`), the child's
/// local rotation is multiplied by a Y rotation of -pi/2
/// ([`ni_matrix3_make_y_rotation`], [`ni_matrix3_multiply`]). At the end
/// `QueuedFile::CheckFinished` runs.
///
/// The exception-unwinding frame is not translated; the locals of the game's
/// stack frame (the scope guard, two `NiPointer`s, the four face coordinates
/// and two matrices) are one block here.
pub fn queued_helmet_check_finished(e: &mut Engine, this: Ptr<QueuedHelmet>) {
    if !fn_0043fab0(e, this) {
        e.call(QUEUED_FILE_CHECK_FINISHED, &args![this]);
        return;
    }
    if fn_0043caa0(e, this.cast()) && e.call(STATE_IS_ZERO, &args![this]).bool() {
        let queue = e.global::<u32>(TASK_QUEUE);
        e.vcall(queue, 0x48, &args![this]);
        return;
    }
    e.with_stack(HELMET_FRAME_SIZE, |e, frame| {
        let guard = frame;
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![
                guard,
                HELMET_CONTEXT,
                1u32,
                MODEL_LOADER_SOURCE,
                HELMET_CHECK_FINISHED_SOURCE_LINE
            ],
        );
        let spare_pointer = frame.byte_add(HELMET_FRAME_SPARE_POINTER);
        e.call(NI_POINTER_CONSTRUCT, &args![spare_pointer, 0u32]);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        if e.call(TASK_STATE_AT_LEAST_4, &args![this]).bool()
            && fn_0043caa0(e, this.cast())
            && !e.call(TASK_STATE_IS_6, &args![this]).bool()
            && e.get(this, QueuedHelmet::pRef).addr() != player
        {
            for index in 0..HELMET_SLOT_COUNT {
                helmet_clone_slot(e, this, frame, index);
            }
        }
        e.call(QUEUED_FILE_CHECK_FINISHED, &args![this]);
        e.call(NI_POINTER_DESTRUCT, &args![spare_pointer]);
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

/// The block that stands for the locals of `QueuedHelmet::CheckFinished`:
/// the scope guard (0), a spare `NiPointer` (4: the deep copy), the
/// `NiPointer` holding the copy given to a child (8), the four 0x20-byte face
/// coordinates (0x10), the rotation matrix (0x90) and the product (0xB4).
pub(crate) const HELMET_FRAME_SIZE: u32 = 0xd8;
pub(crate) const HELMET_FRAME_SPARE_POINTER: u32 = 4;
pub(crate) const HELMET_FRAME_COPY_POINTER: u32 = 8;
pub(crate) const HELMET_FRAME_FACE_COORDS: u32 = 0x10;
pub(crate) const HELMET_FRAME_ROTATION: u32 = 0x90;
pub(crate) const HELMET_FRAME_PRODUCT: u32 = 0xb4;

/// The body of the loop of [`queued_helmet_check_finished`] for biped slot
/// `index`.
pub(crate) fn helmet_clone_slot(e: &mut Engine, this: Ptr<QueuedHelmet>, frame: Ptr, index: u32) {
    let model_slot = helmet_slot(this, HELMET_MODELS_OFFSET, index);
    let face_slot = helmet_slot(this, HELMET_FACE_GEN_MODELS_OFFSET, index);
    let clone_slot = helmet_slot(this, HELMET_CLONED_NODES_OFFSET, index);
    if ni_pointer_get(e, model_slot).is_null() {
        return;
    }
    if loaded_model(e, model_slot).is_null() {
        return;
    }
    let model = loaded_model(e, model_slot);
    if fn_0043b230(e, model).is_null() {
        return;
    }
    let model = loaded_model(e, model_slot);
    let node_3d = fn_0043b230(e, model);
    let face_model = ni_pointer_get(e, face_slot);
    let biped = e.get(this, QueuedHelmet::pBipedAnim);
    let clone = e
        .call(
            BIPED_ANIM_CLONE_HELMET,
            &args![biped, face_model, node_3d, index],
        )
        .u32();
    e.call(NI_POINTER_ASSIGN, &args![clone_slot, clone]);
    if ni_pointer_get(e, clone_slot).is_null() {
        return;
    }
    if helmet_clone_has_marked_child(e, clone_slot) {
        return;
    }
    let face_model = if ni_pointer_get(e, face_slot).is_null() {
        let biped = e.get(this, QueuedHelmet::pBipedAnim);
        e.call(BIPED_ANIM_LOAD_FACE_GEN_MODEL, &args![biped, index])
            .ptr()
    } else {
        ni_pointer_get(e, face_slot)
    };
    if face_model.is_null() {
        return;
    }
    let mut child_index = 0u32;
    loop {
        let clone = ni_pointer_get(e, clone_slot);
        if child_index >= fn_0043b480(e, clone) as u32 {
            return;
        }
        let clone = ni_pointer_get(e, clone_slot);
        let child = fn_0043b4a0(e, clone, child_index);
        child_index += 1;
        if child.is_null() || e.call(OBJECT_POINTER_AT_B8, &args![child]).u32() == 0 {
            continue;
        }
        let npc = match helmet_npc(e, this) {
            Some(npc) => npc,
            None => continue,
        };
        helmet_orient_child(e, frame, child, npc, face_model);
    }
}

/// `LoadedModel` of a `NiPointer<QueuedModel>` slot: the `Model` of the
/// queued model (`004a8a90`).
pub(crate) fn loaded_model(e: &mut Engine, model_slot: Ptr) -> Ptr {
    let queued = ni_pointer_get(e, model_slot);
    e.call(QUEUED_MODEL_GET_MODEL, &args![queued]).ptr()
}

/// Whether some child of the cloned node in `clone_slot` answers non-zero to
/// the virtual function `0x18` and has a set `NiPointer` at +0xBC
/// ([`fn_0043fad0`]).
pub(crate) fn helmet_clone_has_marked_child(e: &mut Engine, clone_slot: Ptr) -> bool {
    let mut child_index = 0u32;
    loop {
        let clone = ni_pointer_get(e, clone_slot);
        if child_index >= fn_0043b480(e, clone) as u32 {
            return false;
        }
        let clone = ni_pointer_get(e, clone_slot);
        let child = fn_0043b4a0(e, clone, child_index);
        if !child.is_null()
            && e.vcall(child.addr(), 0x18, &args![]).u32() != 0
            && !fn_0043fad0(e, child).is_null()
        {
            return true;
        }
        child_index += 1;
    }
}

/// The NPC base form of the helmet's reference, when the biped animation has
/// a reference whose base form is of type 0x2a.
pub(crate) fn helmet_npc(e: &mut Engine, this: Ptr<QueuedHelmet>) -> Option<Ptr> {
    let biped = e.get(this, QueuedHelmet::pBipedAnim);
    if fn_0043f010(e, biped).is_null() {
        return None;
    }
    let biped = e.get(this, QueuedHelmet::pBipedAnim);
    let reference = fn_0043f010(e, biped);
    if e.call(REFERENCE_BASE_FORM, &args![reference]).u32() == 0 {
        return None;
    }
    let biped = e.get(this, QueuedHelmet::pBipedAnim);
    let reference = fn_0043f010(e, biped);
    let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    if e.call(FORM_TYPE, &args![base_form]).u32() != 0x2a {
        return None;
    }
    let biped = e.get(this, QueuedHelmet::pBipedAnim);
    let reference = fn_0043f010(e, biped);
    Some(e.call(REFERENCE_BASE_FORM, &args![reference]).ptr())
}

/// The body of the inner loop of [`queued_helmet_check_finished`] for a
/// `child` of the cloned node: deep copy of its part, face coordinates, and
/// the rotation of its local matrix.
pub(crate) fn helmet_orient_child(
    e: &mut Engine,
    frame: Ptr,
    child: Ptr,
    npc: Ptr,
    face_model: Ptr,
) {
    let face_coords = frame.byte_add(HELMET_FRAME_FACE_COORDS);
    let copy_pointer = frame.byte_add(HELMET_FRAME_COPY_POINTER);
    let spare_pointer = frame.byte_add(HELMET_FRAME_SPARE_POINTER);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            face_coords,
            0x20u32,
            4u32,
            FACE_COORD_CONSTRUCT,
            FACE_COORD_DESTRUCT
        ],
    );
    e.call(NI_POINTER_CONSTRUCT, &args![copy_pointer, 0u32]);
    let part = e.call(OBJECT_POINTER_AT_B8, &args![child]).u32();
    e.call(CREATE_DEEP_COPY, &args![part, spare_pointer]);
    let deep_copy = ni_pointer_get(e, spare_pointer);
    e.call(NI_POINTER_ASSIGN, &args![copy_pointer, deep_copy]);
    let deep_copy = ni_pointer_get(e, copy_pointer);
    e.vcall(child.addr(), 0xe4, &args![deep_copy]);
    e.call(NPC_GET_FACE_COORD, &args![npc, face_coords]);
    if fn_0043faf0(e) == 0
        || e.call(
            FACE_GEN_MODEL_APPLY_COORDS,
            &args![face_model, face_coords, child, 0u32],
        )
        .bool()
    {
        let rotation = frame.byte_add(HELMET_FRAME_ROTATION);
        let product = frame.byte_add(HELMET_FRAME_PRODUCT);
        e.call(MATRIX_CONSTRUCT, &args![rotation]);
        let angle = e.global::<f32>(HELMET_ROTATION_ANGLE);
        ni_matrix3_make_y_rotation(e, rotation, angle);
        let local_rotate = e.call(LOCAL_ROTATE_ADDRESS, &args![child]).ptr();
        let result = ni_matrix3_multiply(e, local_rotate, product, rotation);
        fn_0043fa80(e, child, result);
    }
    e.call(NI_POINTER_DESTRUCT, &args![copy_pointer]);
    e.call(
        VECTOR_DESTRUCT,
        &args![face_coords, 0x20u32, 4u32, FACE_COORD_DESTRUCT],
    );
}

// Translated from 0043f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::MakeYRotation` (engine map): the rotation by `angle` around
/// the Y axis in the matrix at `this` (nine floats, row by row): `cos 0 -sin /
/// 0 1 0 / sin 0 cos`, with `sin` and `cos` from the game's helper
/// (`004169a0`).
pub fn ni_matrix3_make_y_rotation(e: &mut Engine, this: Ptr, angle: f32) {
    // The two floats the helper writes (sine, cosine) on the game's stack.
    let (sin, cos) = e.with_stack(8, |e, out| {
        e.call(SIN_COS, &args![angle, out, out.byte_add(4)]);
        (e.mem.f32(out.addr()), e.mem.f32(out.addr() + 4))
    });
    let matrix = this.addr();
    e.mem.set_f32(matrix, cos);
    e.mem.set_f32(matrix + 4, 0.0);
    e.mem.set_f32(matrix + 8, -sin);
    e.mem.set_f32(matrix + 0xc, 0.0);
    e.mem.set_f32(matrix + 0x10, 1.0);
    e.mem.set_f32(matrix + 0x14, 0.0);
    e.mem.set_f32(matrix + 0x18, sin);
    e.mem.set_f32(matrix + 0x1c, 0.0);
    e.mem.set_f32(matrix + 0x20, cos);
}

// Translated from 0043f8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::operator*` (engine map): the product of the matrix at `this`
/// (left) and `other` (right), nine floats each, row by row, written to
/// `out` (which may be either input); returns `out`. Every element is the sum
/// `left[row][0] * right[0][column] + left[row][1] * right[1][column] +
/// left[row][2] * right[2][column]`, added left to right and rounded to
/// `float` once, as the x87 code does.
pub fn ni_matrix3_multiply(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    // The result is built on the game's stack (after `NiMatrix3()`, which
    // does nothing but return `this`).
    e.with_stack(36, |e, result| {
        e.call(MATRIX_CONSTRUCT, &args![result]);
        let left: Vec<f64> = (0..9)
            .map(|i| e.mem.f32(this.addr() + 4 * i) as f64)
            .collect();
        let right: Vec<f64> = (0..9)
            .map(|i| e.mem.f32(other.addr() + 4 * i) as f64)
            .collect();
        for row in 0..3usize {
            for column in 0..3usize {
                let sum = left[3 * row] * right[column]
                    + left[3 * row + 1] * right[3 + column]
                    + left[3 * row + 2] * right[6 + column];
                e.mem
                    .set_f32(result.addr() + 4 * (3 * row + column) as u32, sum as f32);
            }
        }
        for i in 0..9 {
            let value = e.mem.u32(result.addr() + 4 * i);
            e.mem.set_u32(out.addr() + 4 * i, value);
        }
    });
    out
}

// Translated from 0043fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject` method (no name in the engine map): copies the nine floats of
/// `matrix` into the local rotation matrix at `this + 0x34`.
pub fn fn_0043fa80(e: &mut Engine, this: Ptr, matrix: Ptr) {
    for i in 0..9 {
        let value = e.mem.u32(matrix.addr() + 4 * i);
        e.mem.set_u32(this.addr() + 0x34 + 4 * i, value);
    }
}

// Translated from 0043fab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 of the helmet's `cFlags` is set.
pub fn fn_0043fab0(e: &mut Engine, this: Ptr<QueuedHelmet>) -> bool {
    let flags = e.get(this, QueuedHelmet::cFlags);
    e.call(TEST_FLAG_BIT_1, &args![flags as u32]).bool()
}

// Translated from 0043fad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPointer` at +0xBC of `this` dereferenced.
pub fn fn_0043fad0(e: &mut Engine, this: Ptr) -> Ptr {
    ni_pointer_get(e, this.byte_add(0xbc))
}

// Translated from 0043faf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +4 of the setting object at `011d5a44` (read through
/// `00408d60`, which gives the address of that byte).
pub fn fn_0043faf0(e: &mut Engine) -> u8 {
    let address = e
        .call(SETTING_BYTE_POINTER, &args![HELMET_ROTATION_SETTING])
        .u32();
    e.mem.u8(address)
}

// Translated from 0043fb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet::Cancel` (Xbox PDB): `QueuedFile::Cancel` with its two
/// arguments, then the loader's map of queued helmets is told about the
/// reference ([`fn_0043fb50`]).
pub fn queued_helmet_cancel(e: &mut Engine, this: Ptr<QueuedHelmet>, first: u32, second: u32) {
    e.call(QUEUED_FILE_CANCEL, &args![this, first, second]);
    let reference = e.get(this, QueuedHelmet::pRef);
    let loader = e.global::<u32>(FILE_MAP_OWNER);
    fn_0043fb50(e, Ptr::new(loader), reference);
}

// Translated from 0043fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map) on the loader at
/// `this`: calls the virtual function `0x14` of its `pQueuedHelmetMap`
/// (`this + 0x18`, a `LockFreeMap<TESObjectREFR*, ...>` in the Xbox PDB) with
/// `reference`. What that slot does is not confirmed; both callers use it
/// when the task is over.
pub fn fn_0043fb50(e: &mut Engine, this: Ptr, reference: Ptr) {
    let map = e.mem.u32(this.addr() + 0x18);
    e.vcall(map, 0x14, &args![reference]);
}

// Translated from 0043fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet::Attach` (Xbox PDB): for each of the twenty biped slots with
/// a cloned node, `BipedAnim::AttachHelmet` with the slot's face generation
/// model and cloned node; then the loader's map of queued helmets is told
/// about the reference ([`fn_0043fb50`]).
pub fn queued_helmet_attach(e: &mut Engine, this: Ptr<QueuedHelmet>) {
    for index in 0..HELMET_SLOT_COUNT {
        let clone_slot = helmet_slot(this, HELMET_CLONED_NODES_OFFSET, index);
        if !ni_pointer_get(e, clone_slot).is_null() {
            let clone = ni_pointer_get(e, clone_slot);
            let face_model =
                ni_pointer_get(e, helmet_slot(this, HELMET_FACE_GEN_MODELS_OFFSET, index));
            let biped = e.get(this, QueuedHelmet::pBipedAnim);
            e.call(
                BIPED_ANIM_ATTACH_HELMET,
                &args![biped, face_model, clone, index],
            );
        }
    }
    let reference = e.get(this, QueuedHelmet::pRef);
    let loader = e.global::<u32>(FILE_MAP_OWNER);
    fn_0043fb50(e, Ptr::new(loader), reference);
}

// Translated from 0043fc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedHelmet::GetDescription` (Xbox PDB): writes `"Queued helmet for
/// biped anim %08X"` with `pBipedAnim` into `buffer` (`size` bytes); returns
/// true.
pub fn queued_helmet_get_description(
    e: &mut Engine,
    this: Ptr<QueuedHelmet>,
    buffer: u32,
    size: u32,
) -> bool {
    let biped = e.get(this, QueuedHelmet::pBipedAnim);
    e.call(
        FORMAT_STRING,
        &args![buffer, size, QUEUED_HELMET_FORMAT, biped],
    );
    true
}

// Translated from 0043fc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AttachDistant3DTask::PostProcess` (Xbox PDB): for a task whose state is
/// not 6, that has a reference and whose reference does not pass
/// [`fn_0043fcd0`], the reference gets the node as its 3D (`00570f70`), is
/// loaded into the scene (`TES` method `00451ef0` with its parent cell, no
/// source, flag 0) and `TESActorBase::SetStartsDead(1)` is called on it.
/// Otherwise `spNode` is cleared.
pub fn attach_distant_3d_task_post_process(e: &mut Engine, this: Ptr<AttachDistant3DTask>) {
    let node_slot = this.byte_add(AttachDistant3DTask::spNode.off);
    if !e.call(TASK_STATE_IS_6, &args![this]).bool()
        && !e.get(this, AttachDistant3DTask::pRef).is_null()
    {
        let reference = e.get(this, AttachDistant3DTask::pRef);
        if fn_0043fcd0(e, reference).is_null() {
            let node = ni_pointer_get(e, node_slot);
            let reference = e.get(this, AttachDistant3DTask::pRef);
            e.call(REFERENCE_SET_3D, &args![reference, node]);
            let reference = e.get(this, AttachDistant3DTask::pRef);
            let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
            let reference = e.get(this, AttachDistant3DTask::pRef);
            let tes = e.global::<u32>(TES_GLOBAL);
            e.call(TES_LOAD_REFERENCE, &args![tes, reference, cell, 0u32, 0u32]);
            let reference = e.get(this, AttachDistant3DTask::pRef);
            e.call(SET_STARTS_DEAD, &args![reference, 1u32]);
            return;
        }
    }
    e.call(NI_POINTER_ASSIGN, &args![node_slot, 0u32]);
}

// Translated from 0043fcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR` method (no name in the engine map; `this` is a reference):
/// while the thread's TLS word at +0x264 is `this`, the thread's TLS word at
/// +0x260; otherwise, when the dword at `this + 0x64` is set, the `NiPointer`
/// at +0x14 of the object it points to dereferenced, else 0.
pub fn fn_0043fcd0(e: &mut Engine, this: Ptr) -> Ptr {
    let tls = e.tls();
    if this.addr() == e.mem.u32(tls + TLS_REFERENCE) {
        return Ptr::new(e.mem.u32(tls + TLS_REFERENCE_3D));
    }
    let data = e.mem.u32(this.addr() + 0x64);
    if data == 0 {
        Ptr::NULL
    } else {
        let data = e.mem.u32(this.addr() + 0x64);
        ni_pointer_get(e, Ptr::new(data + 0x14))
    }
}

// Translated from 0043fd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference` constructor (no name in the engine map): the
/// `QueuedFile` constructor with `context`, the reference task's virtual
/// table, `pRef`, and four empty smart pointers (a task pointer, two
/// `NiPointer`s, a task pointer). Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043fd40(
    e: &mut Engine,
    this: Ptr<QueuedReference>,
    reference: Ptr,
    context: u32,
) -> Ptr<QueuedReference> {
    e.call(QUEUED_FILE_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_REFERENCE_VTABLE);
    e.set(this, QueuedReference::pRef, reference);
    e.call(
        TASK_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedReference::spQueuedModel.off), 0u32],
    );
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedReference::spModel.off), 0u32],
    );
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedReference::spCloned3D.off), 0u32],
    );
    e.call(
        TASK_POINTER_CONSTRUCT,
        &args![
            this.byte_add(QueuedReference::spAttachDistant3DTask.off),
            0u32
        ],
    );
    this
}

// Translated from 0043fdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference::PostProcess` (Xbox PDB): calls the virtual function
/// `0x3c` (`Attach`).
pub fn queued_reference_post_process(e: &mut Engine, this: Ptr<QueuedReference>) {
    e.vcall(this.addr(), 0x3c, &args![]);
}

// Translated from 0043fe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference` scalar deleting destructor (no name in the engine map;
/// the decompiler's library match names it after `CBitmapButton`): runs the
/// destructor ([`fn_0043fe40`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn fn_0043fe10(e: &mut Engine, this: Ptr<QueuedReference>, flags: u32) -> Ptr<QueuedReference> {
    fn_0043fe40(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0043fe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference` destructor (no name in the engine map; the decompiler's
/// library match names it `CBitmapButton::~CBitmapButton`): releases the
/// attach task pointer, the cloned 3D, the model (`0040c110`) and the queued
/// model pointer, and runs the `QueuedFile` destructor body. It does not
/// restore the virtual table.
///
/// The exception-unwinding frame is not translated.
pub fn fn_0043fe40(e: &mut Engine, this: Ptr<QueuedReference>) {
    e.call(
        TASK_POINTER_DESTRUCT,
        &args![this.byte_add(QueuedReference::spAttachDistant3DTask.off)],
    );
    e.call(
        NI_POINTER_DESTRUCT,
        &args![this.byte_add(QueuedReference::spCloned3D.off)],
    );
    e.call(
        MODEL_POINTER_RELEASE,
        &args![this.byte_add(QueuedReference::spModel.off)],
    );
    e.call(
        TASK_POINTER_DESTRUCT,
        &args![this.byte_add(QueuedReference::spQueuedModel.off)],
    );
    e.call(QUEUED_FILE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0043aaf0,
            model_model(Ptr<Model>, u32, Ptr, u8, u8) -> Ptr<Model>
        ),
        entry!(0x0043ab70, model_destructor(Ptr<Model>)),
        entry!(0x0043abf0, fn_0043abf0(Ptr<Model>, u32, Ptr) -> Ptr<Model>),
        entry!(0x0043acb0, fn_0043acb0(Ptr<Model>)),
        entry!(0x0043ace0, model_init_model(Ptr<Model>, u32, Ptr, u8, u8)),
        entry!(0x0043b1b0, fn_0043b1b0(Ptr) -> u32),
        entry!(0x0043b1d0, fn_0043b1d0(Ptr) -> u32),
        entry!(0x0043b1e0, fn_0043b1e0(Ptr) -> u32),
        entry!(0x0043b200, fn_0043b200(Ptr, u32) -> Ptr),
        entry!(0x0043b230, fn_0043b230(Ptr) -> Ptr),
        entry!(0x0043b250, fn_0043b250() -> Ptr),
        entry!(0x0043b260, fn_0043b260(Ptr, Ptr) -> bool),
        entry!(0x0043b290, fn_0043b290() -> u32),
        entry!(0x0043b2a0, fn_0043b2a0() -> Ptr),
        entry!(0x0043b2b0, message_handler_inc_disable_warning_count(u8)),
        entry!(0x0043b300, fn_0043b300(u32, Ptr) -> bool),
        entry!(0x0043b320, fn_0043b320(Ptr, u16, Ptr)),
        entry!(0x0043b350, fn_0043b350(Ptr, u8)),
        entry!(0x0043b370, fn_0043b370(Ptr, u8, u32)),
        entry!(0x0043b3b0, fn_0043b3b0(Ptr, Ptr) -> u16),
        entry!(0x0043b410, model_mod_manual_ref_count(Ptr<Model>, i32)),
        entry!(0x0043b460, fn_0043b460(Ptr, i32, i32) -> i32),
        entry!(0x0043b480, fn_0043b480(Ptr) -> u16),
        entry!(0x0043b4a0, fn_0043b4a0(Ptr, u32) -> Ptr),
        entry!(0x0043b4d0, fn_0043b4d0(Ptr) -> u32),
        entry!(0x0043b4f0, fn_0043b4f0(Ptr, Ptr) -> Ptr),
        entry!(0x0043b540, fn_0043b540(Ptr) -> u32),
        entry!(0x0043b560, fn_0043b560(Ptr) -> u32),
        entry!(0x0043b5b0, fn_0043b5b0(Ptr)),
        entry!(0x0043b5d0, fn_0043b5d0(Ptr) -> Ptr),
        entry!(0x0043b5f0, fn_0043b5f0(Ptr)),
        entry!(
            0x0043b610,
            bhk_collision_object_get_bhk_collision_object(Ptr) -> Ptr
        ),
        entry!(
            0x0043b640,
            kf_model_kf_model(Ptr<KFModel>, u32, Ptr) -> Ptr<KFModel>
        ),
        entry!(0x0043b750, fn_0043b750(Ptr<KFModel>)),
        entry!(0x0043b7e0, fn_0043b7e0(Ptr<KFModel>, u32)),
        entry!(0x0043b9d0, fn_0043b9d0(u32, i32) -> Ptr),
        entry!(
            0x0043b9f0,
            fn_0043b9f0(Ptr, Ptr<NiFixedString>) -> Ptr<NiFixedString>
        ),
        entry!(
            0x0043ba10,
            fn_0043ba10(Ptr<NiFixedString>, Ptr<NiFixedString>) -> Ptr<NiFixedString>
        ),
        entry!(0x0043ba60, fn_0043ba60(Ptr)),
        entry!(0x0043baa0, fn_0043baa0(Ptr, Ptr) -> Ptr),
        entry!(0x0043bac0, fn_0043bac0(Ptr<QueuedChildren>)),
        entry!(
            0x0043baf0,
            fn_0043baf0(Ptr<LoadedFile>, u32, Ptr) -> Ptr<LoadedFile>
        ),
        entry!(0x0043bb80, loaded_file_destructor(Ptr<LoadedFile>)),
        entry!(0x0043bce0, fn_0043bce0() -> bool),
        entry!(0x0043bd00, fn_0043bd00() -> u32),
        entry!(
            0x0043bd10,
            fn_0043bd10(Ptr<QueuedTexture>, u32, u32) -> Ptr<QueuedTexture>
        ),
        entry!(
            0x0043bde0,
            queued_file_notify_child_finished(Ptr<QueuedFile>, Ptr)
        ),
        entry!(0x0043be10, fn_0043be10(Ptr<QueuedChildren>)),
        entry!(
            0x0043be30,
            queued_texture_scalar_deleting_destructor(
                Ptr<QueuedTexture>,
                u32,
            ) -> Ptr<QueuedTexture>
        ),
        entry!(
            0x0043be60,
            fn_0043be60(Ptr<QueuedTexture>, Ptr, u32) -> Ptr<QueuedTexture>
        ),
        entry!(
            0x0043bef0,
            fn_0043bef0(Ptr<QueuedTexture>, Ptr, u32) -> Ptr<QueuedTexture>
        ),
        entry!(0x0043bf80, fn_0043bf80(Ptr<QueuedTexture>)),
        entry!(
            0x0043bfe0,
            queued_texture_get_file_index(Ptr<QueuedTexture>) -> u32
        ),
        entry!(0x0043c030, fn_0043c030(Ptr) -> u32),
        entry!(0x0043c050, queued_texture_queue_me(Ptr<QueuedTexture>)),
        entry!(0x0043c0d0, fn_0043c0d0(Ptr<QueuedTexture>, u8)),
        entry!(0x0043c100, fn_0043c100(Ptr<QueuedTexture>, u8)),
        entry!(0x0043c130, fn_0043c130(Ptr) -> u8),
        entry!(0x0043c150, queued_texture_run(Ptr<QueuedTexture>)),
        entry!(0x0043c3b0, fn_0043c3b0(Ptr) -> u32),
        entry!(0x0043c3d0, fn_0043c3d0(Ptr<QueuedTexture>) -> bool),
        entry!(0x0043c3f0, fn_0043c3f0(u32)),
        entry!(0x0043c410, fn_0043c410() -> u32),
        entry!(0x0043c430, fn_0043c430(Ptr) -> u32),
        entry!(0x0043c470, fn_0043c470(Ptr) -> u8),
        entry!(0x0043c490, fn_0043c490(Ptr) -> Ptr),
        entry!(0x0043c4b0, fn_0043c4b0() -> u32),
        entry!(0x0043c4c0, bs_texture_palette_get_texture(u32, Ptr)),
        entry!(0x0043c4f0, fn_0043c4f0(Ptr, u32, Ptr) -> bool),
        entry!(0x0043c530, fn_0043c530(u32) -> u32),
        entry!(0x0043c550, queued_texture_finish(Ptr<QueuedTexture>)),
        entry!(0x0043c610, fn_0043c610(Ptr<QueuedTexture>)),
        entry!(0x0043c630, fn_0043c630(Ptr<QueuedTexture>) -> bool),
        entry!(
            0x0043c650,
            queued_texture_cancel(Ptr<QueuedTexture>, u32, u32)
        ),
        entry!(
            0x0043c6b0,
            queued_texture_get_description(Ptr<QueuedTexture>, u32, u32) -> bool
        ),
        entry!(
            0x0043c6e0,
            fn_0043c6e0(Ptr<QueuedModel>, u32, u32, u32, u8, u8) -> Ptr<QueuedModel>
        ),
        entry!(0x0043c7a0, fn_0043c7a0(Ptr<QueuedModel>, u8)),
        entry!(0x0043c7d0, fn_0043c7d0(Ptr<QueuedModel>, u8)),
        entry!(
            0x0043c800,
            queued_model_scalar_deleting_destructor(Ptr<QueuedModel>, u32) -> Ptr<QueuedModel>
        ),
        entry!(0x0043c830, fn_0043c830(Ptr<QueuedModel>)),
        entry!(
            0x0043c890,
            fn_0043c890(Ptr<QueuedModel>, Ptr, u32, u32, u8, u8) -> Ptr<QueuedModel>
        ),
        entry!(
            0x0043c960,
            fn_0043c960(Ptr<QueuedModel>, Ptr, u32) -> Ptr<QueuedModel>
        ),
        entry!(0x0043ca10, queued_model_generate_key(Ptr<QueuedModel>)),
        entry!(0x0043ca30, queued_model_check_finished(Ptr<QueuedModel>)),
        entry!(0x0043caa0, fn_0043caa0(Ptr<QueuedModel>) -> bool),
        entry!(0x0043caf0, fn_0043caf0(Ptr<QueuedModel>) -> bool),
        entry!(0x0043cb10, queued_model_queue_me(Ptr<QueuedModel>)),
        entry!(0x0043cb50, queued_model_queue_me_ov2(Ptr<QueuedModel>, u32)),
        entry!(0x0043cc60, fn_0043cc60(Ptr<IOTask>) -> u8),
        entry!(0x0043cc80, fn_0043cc80(Ptr<IOTask>) -> u64),
        entry!(0x0043cca0, fn_0043cca0(Ptr) -> u64),
        entry!(0x0043ccc0, fn_0043ccc0(Ptr<QueuedModel>, u8)),
        entry!(0x0043ccf0, queued_model_run(Ptr<QueuedModel>)),
        entry!(0x0043cf40, fn_0043cf40(Ptr<QueuedModel>) -> bool),
        entry!(0x0043cf60, fn_0043cf60(Ptr<QueuedModel>, u8)),
        entry!(0x0043cf90, fn_0043cf90(Ptr<QueuedModel>) -> bool),
        entry!(0x0043cfb0, fn_0043cfb0(Ptr<QueuedModel>) -> bool),
        entry!(0x0043cfd0, fn_0043cfd0(Ptr<BSStream>) -> Ptr<BSStream>),
        entry!(0x0043d050, ni_stream_save_link_id(Ptr, Ptr)),
        entry!(0x0043d090, bs_stream_destructor(Ptr<BSStream>)),
        entry!(0x0043d100, fn_0043d100(Ptr<BSStream>)),
        entry!(
            0x0043d150,
            bs_stream_scalar_deleting_destructor(Ptr<BSStream>, u32) -> Ptr<BSStream>
        ),
        entry!(0x0043d180, queued_model_finish(Ptr<QueuedModel>)),
        entry!(
            0x0043d410,
            fn_0043d410(Ptr<NiUpdateData>, f32, u8, u8) -> Ptr<NiUpdateData>
        ),
        entry!(0x0043d450, ni_av_object_get_world_bound(Ptr) -> Ptr),
        entry!(0x0043d490, fn_0043d490(Ptr<QueuedModel>) -> bool),
        entry!(0x0043d4b0, fn_0043d4b0(Ptr<QueuedModel>) -> bool),
        entry!(0x0043d4d0, fn_0043d4d0(Ptr) -> Ptr),
        entry!(
            0x0043d510,
            queued_model_get_description(Ptr<QueuedModel>, u32, u32) -> bool
        ),
        entry!(
            0x0043d540,
            fn_0043d540(
                Ptr<QueuedTreeBillboard>,
                u32,
                u32,
                Ptr<TreeBillboardData>,
            ) -> Ptr<QueuedTreeBillboard>
        ),
        entry!(
            0x0043d580,
            queued_tree_billboard_scalar_deleting_destructor(
                Ptr<QueuedTreeBillboard>,
                u32,
            )
                -> Ptr<QueuedTreeBillboard>
        ),
        entry!(0x0043d5b0, fn_0043d5b0(Ptr<QueuedTreeBillboard>)),
        entry!(
            0x0043d640,
            fn_0043d640(Ptr<TreeBillboardData>, u32) -> Ptr<TreeBillboardData>
        ),
        entry!(
            0x0043d670,
            queued_tree_billboard_finish(Ptr<QueuedTreeBillboard>)
        ),
        entry!(0x0043d690, iomanager_add_post_process_task(Ptr<IOTask>)),
        entry!(0x0043d6b0, fn_0043d6b0(Ptr, Ptr<IOTask>)),
        entry!(
            0x0043d730,
            queued_tree_billboard_post_process(Ptr<QueuedTreeBillboard>)
        ),
        entry!(
            0x0043d750,
            queued_tree_billboard_create_billboard(Ptr<QueuedTreeBillboard>)
        ),
        entry!(
            0x0043d820,
            queued_tree_billboard_get_description(Ptr<QueuedTreeBillboard>, u32, u32) -> bool
        ),
        entry!(
            0x0043d850,
            fn_0043d850(Ptr<QueuedTreeModel>, Ptr, Ptr, u32, u32) -> Ptr<QueuedTreeModel>
        ),
        entry!(
            0x0043d8c0,
            queued_tree_model_scalar_deleting_destructor(
                Ptr<QueuedTreeModel>,
                u32,
            ) -> Ptr<QueuedTreeModel>
        ),
        entry!(0x0043d8f0, fn_0043d8f0(Ptr<QueuedTreeModel>)),
        entry!(0x0043d910, queued_tree_model_queue_me(Ptr<QueuedTreeModel>)),
        entry!(0x0043da00, queued_tree_model_run(Ptr<QueuedTreeModel>)),
        entry!(0x0043daa0, fn_0043daa0(Ptr<QueuedTreeModel>)),
        entry!(
            0x0043dac0,
            queued_tree_model_cancel(Ptr<QueuedTreeModel>, i32, u32)
        ),
        entry!(
            0x0043db00,
            queued_tree_model_get_description(Ptr<QueuedTreeModel>, u32, u32) -> bool
        ),
        entry!(
            0x0043db30,
            fn_0043db30(Ptr<QueuedMagicItem>, Ptr, Ptr, u32) -> Ptr<QueuedMagicItem>
        ),
        entry!(0x0043db70, io_task_generate_key(Ptr<IOTask>)),
        entry!(0x0043dbb0, fn_0043dbb0(Ptr<IOTask>, i8, u32, u8, u16)),
        entry!(0x0043dc10, fn_0043dc10(Ptr) -> u32),
        entry!(0x0043dc30, fn_0043dc30(Ptr) -> u32),
        entry!(0x0043dc50, fn_0043dc50(Ptr<QueuedMagicItem>)),
        entry!(0x0043dc70, queued_magic_item_queue_me(Ptr<QueuedMagicItem>)),
        entry!(
            0x0043e080,
            queued_magic_item_check_finished(Ptr<QueuedMagicItem>)
        ),
        entry!(
            0x0043e0f0,
            queued_kf_queued_kf(Ptr<QueuedKF>, u32, u32) -> Ptr<QueuedKF>
        ),
        entry!(0x0043e180, fn_0043e180(Ptr<QueuedKF>, u32) -> Ptr<QueuedKF>),
        entry!(0x0043e1b0, fn_0043e1b0(Ptr<QueuedKF>)),
        entry!(
            0x0043e210,
            fn_0043e210(Ptr<QueuedKF>, Ptr, u32) -> Ptr<QueuedKF>
        ),
        entry!(0x0043e2a0, queued_kf_run(Ptr<QueuedKF>)),
        entry!(0x0043e480, fn_0043e480(Ptr<QueuedKF>, u8)),
        entry!(0x0043e4b0, queued_kf_finish(Ptr<QueuedKF>)),
        entry!(0x0043e530, fn_0043e530(Ptr<QueuedKF>) -> bool),
        entry!(
            0x0043e550,
            queued_kf_get_description(Ptr<QueuedKF>, u32, u32) -> bool
        ),
        entry!(
            0x0043e580,
            fn_0043e580(Ptr<QueuedAnimIdle>, u32, u32, Ptr, Ptr) -> Ptr<QueuedAnimIdle>
        ),
        entry!(
            0x0043e600,
            queued_anim_idle_scalar_deleting_destructor(
                Ptr<QueuedAnimIdle>,
                u32,
            ) -> Ptr<QueuedAnimIdle>
        ),
        entry!(0x0043e630, fn_0043e630(Ptr<QueuedAnimIdle>)),
        entry!(0x0043e690, queued_anim_idle_queue_me(Ptr<QueuedAnimIdle>)),
        entry!(0x0043e750, fn_0043e750(Ptr, u32) -> u32),
        entry!(
            0x0043e770,
            queued_anim_idle_check_finished(Ptr<QueuedAnimIdle>)
        ),
        entry!(
            0x0043e7b0,
            queued_anim_idle_post_process(Ptr<QueuedAnimIdle>)
        ),
        entry!(0x0043e810, fn_0043e810(Ptr, Ptr)),
        entry!(
            0x0043e840,
            queued_anim_idle_cancel(Ptr<QueuedAnimIdle>, u32, u32)
        ),
        entry!(
            0x0043e880,
            queued_anim_idle_get_description(Ptr<QueuedAnimIdle>, u32, u32) -> bool
        ),
        entry!(
            0x0043e8b0,
            fn_0043e8b0(Ptr<QueuedReplacementKF>, u32, u32, Ptr, Ptr) -> Ptr<QueuedReplacementKF>
        ),
        entry!(
            0x0043e8f0,
            fn_0043e8f0(Ptr<QueuedReplacementKF>, u32) -> Ptr<QueuedReplacementKF>
        ),
        entry!(0x0043e920, fn_0043e920(Ptr<QueuedReplacementKF>)),
        entry!(
            0x0043e940,
            queued_replacement_kf_finish(Ptr<QueuedReplacementKF>)
        ),
        entry!(0x0043e990, fn_0043e990(Ptr<QueuedReplacementKFList>)),
        entry!(
            0x0043e9b0,
            queued_replacement_kf_post_process(Ptr<QueuedReplacementKF>)
        ),
        entry!(
            0x0043ea00,
            queued_replacement_kf_get_description(Ptr<QueuedReplacementKF>, u32, u32) -> bool
        ),
        entry!(0x0043ea30, fn_0043ea30(Ptr<QueuedReplacementKFList>)),
        entry!(0x0043ea90, fn_0043ea90(Ptr, Ptr)),
        entry!(
            0x0043eac0,
            queued_replacement_kf_list_cancel(Ptr<QueuedReplacementKFList>, u32, u32)
        ),
        entry!(
            0x0043eaf0,
            fn_0043eaf0(Ptr<QueuedHead>, Ptr, u32) -> Ptr<QueuedHead>
        ),
        entry!(
            0x0043eb80,
            queued_head_scalar_deleting_destructor(Ptr<QueuedHead>, u32) -> Ptr<QueuedHead>
        ),
        entry!(0x0043ebb0, fn_0043ebb0(Ptr<QueuedHead>)),
        entry!(0x0043ec20, queued_head_queue_me(Ptr<QueuedHead>)),
        entry!(0x0043ec60, queued_head_check_finished(Ptr<QueuedHead>)),
        entry!(0x0043ecb0, queued_head_run(Ptr<QueuedHead>)),
        entry!(0x0043ed90, fn_0043ed90(Ptr<QueuedHead>, Ptr, u32)),
        entry!(
            0x0043eed0,
            queued_head_get_description(Ptr<QueuedHead>, u32, u32) -> bool
        ),
        entry!(
            0x0043ef10,
            fn_0043ef10(Ptr<QueuedHelmet>, Ptr, u32, u32) -> Ptr<QueuedHelmet>
        ),
        entry!(0x0043eff0, queued_helmet_post_process(Ptr<QueuedHelmet>)),
        entry!(0x0043f010, fn_0043f010(Ptr) -> Ptr),
        entry!(
            0x0043f030,
            queued_helmet_scalar_deleting_destructor(Ptr<QueuedHelmet>, u32) -> Ptr<QueuedHelmet>
        ),
        entry!(0x0043f060, fn_0043f060(Ptr)),
        entry!(0x0043f080, fn_0043f080(Ptr<QueuedHelmet>)),
        entry!(0x0043f120, queued_helmet_queue_me(Ptr<QueuedHelmet>)),
        entry!(0x0043f1f0, fn_0043f1f0(Ptr<QueuedHelmet>, u8)),
        entry!(0x0043f220, fn_0043f220(Ptr, u32) -> Ptr),
        entry!(0x0043f240, queued_helmet_run(Ptr<QueuedHelmet>)),
        entry!(0x0043f2b0, queued_helmet_finish(Ptr<QueuedHelmet>)),
        entry!(0x0043f2f0, queued_helmet_check_finished(Ptr<QueuedHelmet>)),
        entry!(0x0043f850, ni_matrix3_make_y_rotation(Ptr, f32)),
        entry!(0x0043f8d0, ni_matrix3_multiply(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0043fa80, fn_0043fa80(Ptr, Ptr)),
        entry!(0x0043fab0, fn_0043fab0(Ptr<QueuedHelmet>) -> bool),
        entry!(0x0043fad0, fn_0043fad0(Ptr) -> Ptr),
        entry!(0x0043faf0, fn_0043faf0() -> u8),
        entry!(
            0x0043fb10,
            queued_helmet_cancel(Ptr<QueuedHelmet>, u32, u32)
        ),
        entry!(0x0043fb50, fn_0043fb50(Ptr, Ptr)),
        entry!(0x0043fb80, queued_helmet_attach(Ptr<QueuedHelmet>)),
        entry!(
            0x0043fc10,
            queued_helmet_get_description(Ptr<QueuedHelmet>, u32, u32) -> bool
        ),
        entry!(
            0x0043fc40,
            attach_distant_3d_task_post_process(Ptr<AttachDistant3DTask>)
        ),
        entry!(0x0043fcd0, fn_0043fcd0(Ptr) -> Ptr),
        entry!(
            0x0043fd40,
            fn_0043fd40(Ptr<QueuedReference>, Ptr, u32) -> Ptr<QueuedReference>
        ),
        entry!(
            0x0043fdf0,
            queued_reference_post_process(Ptr<QueuedReference>)
        ),
        entry!(
            0x0043fe10,
            fn_0043fe10(Ptr<QueuedReference>, u32) -> Ptr<QueuedReference>
        ),
        entry!(0x0043fe40, fn_0043fe40(Ptr<QueuedReference>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A virtual function (any slot) that returns 1, and one that returns 0.
    const ANSWERS_YES: u32 = 0x0ff0_0001;
    const ANSWERS_NO: u32 = 0x0ff0_0000;
    /// Slot 0xF8 of the wrapper node (`SetAt`): does nothing but is logged.
    const SET_AT: u32 = 0x0ff0_0010;
    /// Slot 0x90 of the object `InitModel` looks up in the editor branch.
    const EDITOR_SLOT: u32 = 0x0ff0_0020;
    /// A stream's loader slot (0x60) that gives the stream one node.
    const STREAM_LOADS_A_NODE: u32 = 0x0ff0_0030;
    const NODE_VTABLE: u32 = 0x0ff1_0000;
    const GEOMETRY_VTABLE: u32 = 0x0ff1_1000;
    const PLAIN_VTABLE: u32 = 0x0ff1_2000;
    const STREAM_NO_VTABLE: u32 = 0x0ff1_4000;
    const WRAPPER_VTABLE: u32 = 0x0ff1_5000;
    const EDITOR_VTABLE: u32 = 0x0ff1_6000;
    const STREAM_LOADING_VTABLE: u32 = 0x0ff1_7000;
    /// The property type `00702440` returns in the tests.
    const SECOND_TYPE: u32 = 0x20;
    /// The pooled string's reference counter in the tests.
    const STRING_COUNTER: u32 = 0x0118_5700;

    /// An engine with test doubles for the callees every test may use: the
    /// `NiPointer` functions, the memory manager, string helpers, the stream
    /// array getters, interlocked counters, and the pages of the globals the
    /// unit reads.
    fn loader_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0109_6000,
            0x0109_b000,
            0x0118_5000,
            0x011f_4000,
            0x0120_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NI_POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_FREE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(MEMORY_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(STRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().position(|c| *c as u32 == a[1]) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(ARRAY_ELEMENT_ADDRESS, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 4).into_ret()
        });
        e.register(ARRAY_COUNT, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(ARRAY_SIZE, |e, a| e.mem.u16(a[0] + 0xa).into_ret());
        e.register(NAME_SLOT, |_, a| (a[0] + 8).into_ret());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.i32(a[0]) + 1;
            e.mem.set_i32(a[0], value);
            value.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.i32(a[0]) - 1;
            e.mem.set_i32(a[0], value);
            value.into_ret()
        });
        e.register(INTERLOCKED_COMPARE_EXCHANGE, |e, a| {
            let current = e.mem.u32(a[0]);
            if current == a[2] {
                e.mem.set_u32(a[0], a[1]);
            }
            current.into_ret()
        });
        e.register(SLEEP, |_, _| Ret::default());
        e.register(ANSWERS_YES, |_, _| 1u32.into_ret());
        e.register(ANSWERS_NO, |_, _| 0u32.into_ret());
        e.register(SET_AT, |_, _| Ret::default());
        e.register(EDITOR_SLOT, |_, _| Ret::default());
        e.register(LOG, |_, _| Ret::default());
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(FIXED_STRING_RELEASE, |_, _| Ret::default());
        e.register(FIXED_STRING_REFCOUNT, |_, _| STRING_COUNTER.into_ret());
        // Virtual tables: slot n is at byte 4n (0xC IsNode, 0x18 IsGeometry,
        // 0x60 loader, 0x90 editor slot, 0xF8 SetAt).
        let table = |is_node: u32, is_geometry: u32, loader: u32, editor: u32| {
            let mut slots = vec![ANSWERS_NO; 0x40];
            slots[3] = is_node;
            slots[6] = is_geometry;
            slots[0x18] = loader;
            slots[0x24] = editor;
            slots[0x3e] = SET_AT;
            slots
        };
        let tables = [
            (
                NODE_VTABLE,
                table(ANSWERS_YES, ANSWERS_NO, ANSWERS_NO, ANSWERS_NO),
            ),
            (
                GEOMETRY_VTABLE,
                table(ANSWERS_NO, ANSWERS_YES, ANSWERS_NO, ANSWERS_NO),
            ),
            (
                PLAIN_VTABLE,
                table(ANSWERS_NO, ANSWERS_NO, ANSWERS_NO, ANSWERS_NO),
            ),
            (
                STREAM_NO_VTABLE,
                table(ANSWERS_NO, ANSWERS_NO, ANSWERS_NO, ANSWERS_NO),
            ),
            (
                STREAM_LOADING_VTABLE,
                table(ANSWERS_NO, ANSWERS_NO, STREAM_LOADS_A_NODE, ANSWERS_NO),
            ),
            (
                WRAPPER_VTABLE,
                table(ANSWERS_YES, ANSWERS_NO, ANSWERS_NO, ANSWERS_NO),
            ),
            (
                EDITOR_VTABLE,
                table(ANSWERS_NO, ANSWERS_NO, ANSWERS_NO, EDITOR_SLOT),
            ),
        ];
        for (address, slots) in tables {
            e.put_vtable(address, &slots);
        }
        // The wrapper `InitModel` makes: a zeroed block with the wrapper
        // vtable.
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], WRAPPER_VTABLE);
            a[0].into_ret()
        });
        e
    }

    /// An object with the given virtual table and 0x80 bytes.
    fn object(e: &mut Engine, vtable: u32) -> Ptr {
        let block = e.mem.alloc(0x80);
        e.mem.set_u32(block, vtable);
        Ptr::new(block)
    }

    /// A string in game memory.
    fn text(e: &mut Engine, s: &str) -> u32 {
        let block = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(block, s.as_bytes());
        block
    }

    /// A `BSStream` holding `objects` (in the array at +0x21C), with the
    /// given virtual table and no file name.
    fn stream_with(e: &mut Engine, vtable: u32, objects: &[Ptr]) -> Ptr {
        let stream = Ptr::new(e.mem.alloc(0x230));
        e.mem.set_u32(stream.addr(), vtable);
        set_stream_objects(e, stream, objects);
        stream
    }

    fn set_stream_objects(e: &mut Engine, stream: Ptr, objects: &[Ptr]) {
        let base = e.mem.alloc(4 * objects.len() as u32 + 4);
        for (i, object) in objects.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, object.addr());
        }
        e.mem.set_u32(stream.addr() + 0x21c + 4, base);
        e.mem
            .set_u32(stream.addr() + 0x21c + 0xc, objects.len() as u32);
    }

    fn model(e: &mut Engine) -> Ptr<Model> {
        e.new_object()
    }

    /// The argument lists of the calls made to `addr` since `call_log` was
    /// switched on.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The addresses called, in order.
    fn call_order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    #[test]
    fn model_constructor_clears_the_pointer_then_initialises() {
        let mut e = loader_engine();
        let this = model(&mut e);
        let name = text(&mut e, "meshes\\a.nif");
        // An empty stream whose loader fails: InitModel gives up.
        let stream = stream_with(&mut e, STREAM_NO_VTABLE, &[]);
        e.register(STREAM_FREE_ALL_OBJECTS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_aaf0, &args![this, name, stream, 1u8, 0u8]);
        assert_eq!(back.ptr::<Model>(), this);
        let order = call_order(&e);
        assert_eq!(&order[1..3], &[NI_POINTER_CONSTRUCT, NI_POINTER_ASSIGN]);
        assert_eq!(
            calls_to(&e, NI_POINTER_CONSTRUCT)[0],
            vec![this.addr() + 0xc, 0]
        );
        assert!(order.contains(&STREAM_FREE_ALL_OBJECTS));
        assert_eq!(e.get(this, Model::pFilename), 0);
    }

    #[test]
    fn model_destructor_releases_frees_and_destroys() {
        let mut e = loader_engine();
        let this = model(&mut e);
        let filename = e.mem.alloc(16);
        e.set(this, Model::pFilename, filename);
        e.set(this, Model::spObject3D, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        e.call(0x0043_ab70, &args![this]);
        let pointer = this.addr() + 0xc;
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0043_ab70, vec![this.addr()]),
                (NI_POINTER_ASSIGN, vec![pointer, 0]),
                (MEMORY_FREE, vec![filename]),
                (NI_POINTER_DESTRUCT, vec![pointer]),
            ]
        );
        assert_eq!(e.mem.u32(pointer), 0);
    }

    #[test]
    fn second_constructor_copies_the_name_and_holds_the_node() {
        let mut e = loader_engine();
        let this = model(&mut e);
        e.set(this, Model::iRefCount, 7);
        e.set(this, Model::iManualRefCount, 9);
        let name = text(&mut e, "abc");
        let back = e.call(0x0043_abf0, &args![this, name, Ptr::<()>::new(0x4444)]);
        assert_eq!(back.ptr::<Model>(), this);
        let copy = e.get(this, Model::pFilename);
        assert_ne!(copy, name);
        assert_eq!(e.mem.cstr(copy), b"abc");
        assert_eq!(e.get(this, Model::spObject3D), Ptr::new(0x4444));
        assert_eq!(e.get(this, Model::iRefCount), 0);
        assert_eq!(e.get(this, Model::iManualRefCount), 0);
    }

    #[test]
    fn manual_count_is_decremented_only_when_positive() {
        let mut e = loader_engine();
        let this = model(&mut e);
        for (before, after) in [(3, 2), (1, 0), (0, 0), (-4, -4)] {
            e.set(this, Model::iManualRefCount, before);
            e.call(0x0043_acb0, &args![this]);
            assert_eq!(e.get(this, Model::iManualRefCount), after);
        }
    }

    /// A model and a stream holding `objects`, `InitModel` run with
    /// `prepare` and `force`.
    fn init(e: &mut Engine, objects: &[Ptr], prepare: u8, force: u8) -> (Ptr<Model>, Ptr) {
        let this = model(e);
        let name = text(e, "meshes\\a.nif");
        let stream = stream_with(e, STREAM_NO_VTABLE, objects);
        e.call(0x0043_ace0, &args![this, name, stream, prepare, force]);
        (this, stream)
    }

    fn object_3d(e: &Engine, model: Ptr<Model>) -> Ptr {
        e.get(model, Model::spObject3D)
    }

    /// Doubles for what `InitModel` calls after it has taken objects, with
    /// no properties on the root.
    fn quiet_init_engine() -> Engine {
        let mut e = loader_engine();
        e.register(REMOVE_EDITOR_MARKERS, |_, _| Ret::default());
        e.register(GET_PROPERTY, |_, _| 0u32.into_ret());
        e.register(PROPERTY_TYPE_SECOND_GETTER, |_, _| SECOND_TYPE.into_ret());
        e.register(MATCHES_PATTERN, |_, _| 0u32.into_ret());
        e.register(STREAM_FREE_ALL_OBJECTS, |_, _| Ret::default());
        e
    }

    #[test]
    fn init_model_keeps_a_single_root_node_directly() {
        let mut e = quiet_init_engine();
        let root = object(&mut e, NODE_VTABLE);
        // The node's name is not a `Bip##` one.
        let node_name = text(&mut e, "Scene Root");
        e.mem.set_u32(root.addr() + 8, node_name);
        e.call_log = Some(vec![]);
        let (this, _) = init(&mut e, &[root], 0, 0);
        assert_eq!(object_3d(&e, this), root);
        assert_eq!(calls_to(&e, REMOVE_EDITOR_MARKERS), vec![vec![root.addr()]]);
        assert_eq!(e.get(this, Model::iRefCount), 0);
        assert_eq!(e.get(this, Model::iManualRefCount), 0);
        let copy = e.get(this, Model::pFilename);
        assert_eq!(e.mem.cstr(copy), b"meshes\\a.nif");
        // No wrapper was made.
        assert!(calls_to(&e, NI_ALLOC).is_empty());
        // The matcher was asked about the name.
        assert_eq!(
            calls_to(&e, MATCHES_PATTERN),
            vec![vec![node_name, BIP_PATTERN]]
        );
    }

    #[test]
    fn init_model_wraps_a_bip_root_in_a_new_node() {
        let mut e = quiet_init_engine();
        e.register(MATCHES_PATTERN, |_, _| 1u32.into_ret());
        let root = object(&mut e, NODE_VTABLE);
        let node_name = text(&mut e, "Bip01");
        e.mem.set_u32(root.addr() + 8, node_name);
        e.call_log = Some(vec![]);
        let (this, _) = init(&mut e, &[root], 0, 0);
        let wrapper = object_3d(&e, this);
        assert_ne!(wrapper, root);
        assert_eq!(e.mem.u32(wrapper.addr()), WRAPPER_VTABLE);
        assert_eq!(calls_to(&e, NI_ALLOC), vec![vec![0xac]]);
        assert_eq!(calls_to(&e, NI_NODE_CONSTRUCT).len(), 1);
        // The Bip node is child 0 of the wrapper.
        assert_eq!(
            calls_to(&e, SET_AT),
            vec![vec![wrapper.addr(), 0, root.addr()]]
        );
    }

    #[test]
    fn init_model_without_a_node_name_keeps_the_root_directly() {
        let mut e = quiet_init_engine();
        e.register(MATCHES_PATTERN, |_, _| 1u32.into_ret());
        // A null name is never matched against the pattern.
        let root = object(&mut e, NODE_VTABLE);
        e.call_log = Some(vec![]);
        let (this, _) = init(&mut e, &[root], 0, 0);
        assert_eq!(object_3d(&e, this), root);
        assert!(calls_to(&e, MATCHES_PATTERN).is_empty());
    }

    #[test]
    fn init_model_collects_nodes_and_geometry_and_skips_the_rest() {
        let mut e = quiet_init_engine();
        let plain = object(&mut e, PLAIN_VTABLE);
        let geometry = object(&mut e, GEOMETRY_VTABLE);
        let node = object(&mut e, NODE_VTABLE);
        // Object 0 is not a node, so the single-root shortcut is not taken;
        // a null entry and a plain object are skipped, the others get
        // consecutive slots of one wrapper.
        e.call_log = Some(vec![]);
        let (this, _) = init(&mut e, &[plain, Ptr::NULL, geometry, node], 0, 0);
        let wrapper = object_3d(&e, this);
        assert_eq!(e.mem.u32(wrapper.addr()), WRAPPER_VTABLE);
        assert_eq!(
            calls_to(&e, SET_AT),
            vec![
                vec![wrapper.addr(), 0, geometry.addr()],
                vec![wrapper.addr(), 1, node.addr()],
            ]
        );
        // Only one wrapper is made.
        assert_eq!(calls_to(&e, NI_ALLOC).len(), 1);
        assert_eq!(
            calls_to(&e, REMOVE_EDITOR_MARKERS),
            vec![vec![wrapper.addr()]]
        );
    }

    #[test]
    fn init_model_with_nothing_to_take_does_no_post_processing() {
        let mut e = quiet_init_engine();
        let plain = object(&mut e, PLAIN_VTABLE);
        e.call_log = Some(vec![]);
        let (this, _) = init(&mut e, &[plain], 1, 0);
        assert_eq!(object_3d(&e, this), Ptr::NULL);
        assert!(calls_to(&e, REMOVE_EDITOR_MARKERS).is_empty());
        assert!(calls_to(&e, PREPARE_OBJECT).is_empty());
    }

    #[test]
    fn init_model_loads_the_error_marker_into_an_empty_stream() {
        let mut e = quiet_init_engine();
        // The stream's loader (slot 0x60) gives it one node and succeeds.
        let marker = object(&mut e, NODE_VTABLE);
        e.register_double(STREAM_LOADS_A_NODE, move |e, a| {
            let stream = Ptr::new(a[0]);
            set_stream_objects(e, stream, &[marker]);
            1u32.into_ret()
        });
        let stream = stream_with(&mut e, STREAM_LOADING_VTABLE, &[]);
        let this = model(&mut e);
        let name = text(&mut e, "meshes\\missing.nif");
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 0u8, 0u8]);
        assert_eq!(
            calls_to(&e, STREAM_FREE_ALL_OBJECTS),
            vec![vec![stream.addr()]]
        );
        // The loader was called with the marker's path and 0.
        assert_eq!(
            calls_to(&e, STREAM_LOADS_A_NODE),
            vec![vec![stream.addr(), MARKER_ERROR_NIF, 0]]
        );
        assert_eq!(object_3d(&e, this), marker);
        // The model keeps the requested name.
        let copy = e.get(this, Model::pFilename);
        assert_eq!(e.mem.cstr(copy), b"meshes\\missing.nif");
    }

    #[test]
    fn init_model_gives_up_when_even_the_error_marker_fails() {
        let mut e = quiet_init_engine();
        e.call_log = Some(vec![]);
        let (this, _) = init(&mut e, &[], 1, 1);
        assert_eq!(e.get(this, Model::pFilename), 0);
        assert_eq!(object_3d(&e, this), Ptr::NULL);
        assert!(calls_to(&e, REMOVE_EDITOR_MARKERS).is_empty());
    }

    /// A scene where the root has both shared properties (equal to the
    /// shared ones in the compared word), with the stream's name byte set or
    /// not. Returns the model, the stream, the model's name and the root.
    fn property_scene(e: &mut Engine, with_stream_name: bool) -> (Ptr<Model>, Ptr, u32, Ptr) {
        // The two shared properties and the root's own copies.
        let shared_first = object(e, PLAIN_VTABLE);
        let shared_second = object(e, PLAIN_VTABLE);
        e.mem.set_u16(shared_first.addr() + 0x18, 5);
        e.mem.set_u16(shared_second.addr() + 0x18, 6);
        e.mem.set_u32(SHARED_PROPERTY_FIRST, shared_first.addr());
        e.mem.set_u32(SHARED_PROPERTY_SECOND, shared_second.addr());
        let root = object(e, NODE_VTABLE);
        let own_first = object(e, PLAIN_VTABLE);
        let own_second = object(e, PLAIN_VTABLE);
        e.mem.set_u16(own_first.addr() + 0x18, 5);
        e.mem.set_u16(own_second.addr() + 0x18, 6);
        let (first, second) = (own_first.addr(), own_second.addr());
        e.register_double(GET_PROPERTY, move |_, a| {
            match a[1] {
                PROPERTY_TYPE_FIRST => first,
                SECOND_TYPE => second,
                _ => 0,
            }
            .into_ret()
        });
        e.register(REMOVE_PROPERTY, |_, _| Ret::default());
        let this = model(e);
        let name = text(e, "meshes\\a.nif");
        let stream = stream_with(e, STREAM_NO_VTABLE, &[root]);
        if with_stream_name {
            e.mem.set_cstr(stream.addr() + 8, b"x.nif");
        }
        (this, stream, name, root)
    }

    #[test]
    fn init_model_strips_both_properties_and_logs_with_the_stream_name() {
        let mut e = quiet_init_engine();
        let (this, stream, name, root) = property_scene(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 0u8, 0u8]);
        assert_eq!(
            calls_to(&e, REMOVE_PROPERTY),
            vec![
                vec![root.addr(), PROPERTY_TYPE_FIRST],
                vec![root.addr(), SECOND_TYPE]
            ]
        );
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![REEXPORT_WITH_FILE_MESSAGE, stream.addr() + 8, name]]
        );
        // The warning counter went up and back down around the checks.
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT).len(), 1);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
        assert_eq!(e.global::<i32>(DISABLE_WARNING_COUNT), 0);
    }

    #[test]
    fn init_model_logs_without_a_stream_name_when_it_has_none() {
        let mut e = quiet_init_engine();
        let (this, stream, name, _) = property_scene(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 0u8, 0u8]);
        assert_eq!(calls_to(&e, LOG), vec![vec![REEXPORT_MESSAGE, name]]);
    }

    #[test]
    fn init_model_keeps_properties_that_are_not_the_shared_ones() {
        let mut e = quiet_init_engine();
        let (this, stream, name, _) = property_scene(&mut e, true);
        // Make the stored shared properties differ from the root's.
        let first = e.mem.u32(SHARED_PROPERTY_FIRST);
        e.mem.set_u16(first + 0x18, 99);
        let second = e.mem.u32(SHARED_PROPERTY_SECOND);
        e.mem.set_u16(second + 0x18, 98);
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 0u8, 0u8]);
        assert!(calls_to(&e, REMOVE_PROPERTY).is_empty());
        assert!(calls_to(&e, LOG).is_empty());
    }

    #[test]
    fn init_model_strips_only_the_second_property_when_the_first_is_missing() {
        let mut e = quiet_init_engine();
        let (this, stream, name, root) = property_scene(&mut e, false);
        let own_second = object(&mut e, PLAIN_VTABLE);
        let shared_second = e.mem.u32(SHARED_PROPERTY_SECOND);
        let word = e.mem.u16(shared_second + 0x18);
        e.mem.set_u16(own_second.addr() + 0x18, word);
        let own_second = own_second.addr();
        // The root has no property of the first type.
        e.register_double(GET_PROPERTY, move |_, a| {
            if a[1] == SECOND_TYPE { own_second } else { 0 }.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 0u8, 0u8]);
        assert_eq!(
            calls_to(&e, REMOVE_PROPERTY),
            vec![vec![root.addr(), SECOND_TYPE]]
        );
    }

    #[test]
    fn init_model_prepares_the_object_when_asked() {
        // `prepare` set, `force` clear, no morpher: PrepareObject(root, 0, 0).
        let mut e = quiet_init_engine();
        let (this, stream, name, root) = property_scene(&mut e, false);
        e.register(HAS_MORPHER_CONTROLLER, |_, _| 0u32.into_ret());
        e.register(PREPARE_OBJECT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 1u8, 0u8]);
        assert_eq!(
            calls_to(&e, HAS_MORPHER_CONTROLLER),
            vec![vec![root.addr()]]
        );
        assert_eq!(calls_to(&e, PREPARE_OBJECT), vec![vec![root.addr(), 0, 0]]);

        // A morpher controller sets the last flag.
        let (this, stream, name, root) = property_scene(&mut e, false);
        e.register(HAS_MORPHER_CONTROLLER, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 1u8, 0u8]);
        assert_eq!(calls_to(&e, PREPARE_OBJECT), vec![vec![root.addr(), 0, 1]]);

        // `force` skips the morpher test and passes itself and 1.
        let (this, stream, name, root) = property_scene(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 1u8, 1u8]);
        assert!(calls_to(&e, HAS_MORPHER_CONTROLLER).is_empty());
        assert_eq!(calls_to(&e, PREPARE_OBJECT), vec![vec![root.addr(), 1, 1]]);

        // Without `prepare` nothing is prepared.
        let (this, stream, name, _) = property_scene(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 0u8, 1u8]);
        assert!(calls_to(&e, PREPARE_OBJECT).is_empty());
    }

    #[test]
    fn init_model_runs_the_editor_branch_only_when_the_flag_is_set() {
        let mut e = quiet_init_engine();
        let (this, stream, name, root) = property_scene(&mut e, false);
        // The root's NiPointer at +0xC and the cast result.
        let controller = 0x7000_1000;
        e.mem.set_u32(root.addr() + 0xc, controller);
        let casted = e.mem.alloc(8);
        let target = object(&mut e, EDITOR_VTABLE);
        e.mem.set_u32(casted, target.addr());
        let editor_name = text(&mut e, "Root");
        e.mem.set_u32(root.addr() + 8, editor_name);
        e.register_double(DYNAMIC_CAST, move |_, _| casted.into_ret());
        e.register(CAST_RESULT_POINTER, |e, a| e.mem.u32(a[0]).into_ret());

        // Flag clear: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 0u8, 0u8]);
        assert!(calls_to(&e, DYNAMIC_CAST).is_empty());
        assert!(calls_to(&e, EDITOR_SLOT).is_empty());

        // Flag set: the root's `NiPointer` at +0xC is cast, and slot 0x90 of
        // the pointer the cast result holds gets the root's name slot and 0.
        let (this, stream, name, root) = property_scene(&mut e, false);
        e.mem.set_u32(root.addr() + 0xc, controller);
        e.mem.set_u32(root.addr() + 8, editor_name);
        e.mem.set_u8(GUN_WOBBLE_FLAG, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 0u8, 0u8]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![TYPE_DESCRIPTOR_011F36AC, controller]]
        );
        assert_eq!(
            calls_to(&e, EDITOR_SLOT),
            vec![vec![target.addr(), root.addr() + 8, 0]]
        );

        // The cast failing (null) stops it.
        let (this, stream, name, root) = property_scene(&mut e, false);
        e.mem.set_u32(root.addr() + 0xc, controller);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0043_ace0, &args![this, name, stream, 0u8, 0u8]);
        assert!(calls_to(&e, EDITOR_SLOT).is_empty());
    }

    #[test]
    fn acquire_load_reads_the_dword() {
        let mut e = loader_engine();
        let slot = e.mem.alloc(8);
        e.mem.set_u32(slot, 0xcafe);
        assert_eq!(
            e.call(0x0043_b1d0, &args![Ptr::<()>::new(slot)]).u32(),
            0xcafe
        );
        assert_eq!(
            e.call(0x0043_b1b0, &args![Ptr::<()>::new(slot)]).u32(),
            0xcafe
        );
    }

    #[test]
    fn stream_accessors_read_the_array_at_0x21c() {
        let mut e = loader_engine();
        let a = object(&mut e, PLAIN_VTABLE);
        let b = object(&mut e, PLAIN_VTABLE);
        let stream = stream_with(&mut e, STREAM_NO_VTABLE, &[a, b]);
        assert_eq!(e.call(0x0043_b1e0, &args![stream]).u32(), 2);
        assert_eq!(e.call(0x0043_b200, &args![stream, 0u32]).ptr::<()>(), a);
        assert_eq!(e.call(0x0043_b200, &args![stream, 1u32]).ptr::<()>(), b);
    }

    #[test]
    fn pointer_field_at_0xc_is_dereferenced() {
        let mut e = loader_engine();
        let node = object(&mut e, NODE_VTABLE);
        e.mem.set_u32(node.addr() + 0xc, 0x1357);
        assert_eq!(e.call(0x0043_b230, &args![node]).u32(), 0x1357);
    }

    #[test]
    fn shared_property_getters_read_their_globals() {
        let mut e = loader_engine();
        e.mem.set_u32(SHARED_PROPERTY_SECOND, 0x1111);
        e.mem.set_u32(SHARED_PROPERTY_FIRST, 0x2222);
        assert_eq!(e.call(0x0043_b250, &args![]).u32(), 0x1111);
        assert_eq!(e.call(0x0043_b2a0, &args![]).u32(), 0x2222);
    }

    #[test]
    fn property_comparison_is_on_the_word_at_0x18() {
        let mut e = loader_engine();
        let a = object(&mut e, PLAIN_VTABLE);
        let b = object(&mut e, PLAIN_VTABLE);
        e.mem.set_u16(a.addr() + 0x18, 12);
        e.mem.set_u16(b.addr() + 0x18, 12);
        assert!(e.call(0x0043_b260, &args![a, b]).bool());
        e.mem.set_u16(b.addr() + 0x18, 13);
        assert!(!e.call(0x0043_b260, &args![a, b]).bool());
        // Bits above the word do not matter.
        e.mem.set_u16(b.addr() + 0x1a, 0xffff);
        e.mem.set_u16(b.addr() + 0x18, 12);
        assert!(e.call(0x0043_b260, &args![a, b]).bool());
    }

    #[test]
    fn first_property_type_is_eleven() {
        let mut e = loader_engine();
        assert_eq!(e.call(0x0043_b290, &args![]).u32(), 11);
    }

    #[test]
    fn warning_counter_moves_by_one_and_never_goes_negative() {
        let mut e = loader_engine();
        e.call(0x0043_b2b0, &args![1u8]);
        assert_eq!(e.global::<i32>(DISABLE_WARNING_COUNT), 1);
        e.call(0x0043_b2b0, &args![1u8]);
        assert_eq!(e.global::<i32>(DISABLE_WARNING_COUNT), 2);
        e.call(0x0043_b2b0, &args![0u8]);
        assert_eq!(e.global::<i32>(DISABLE_WARNING_COUNT), 1);
        e.call(0x0043_b2b0, &args![0u8]);
        e.call(0x0043_b2b0, &args![0u8]);
        // The second decrement went to -1 and was clamped.
        assert_eq!(e.global::<i32>(DISABLE_WARNING_COUNT), 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_b2b0, &args![1u8]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![DISABLE_WARNING_COUNT]]
        );
    }

    #[test]
    fn kind_test_is_false_for_null_and_asks_the_object_otherwise() {
        let mut e = loader_engine();
        e.register(IS_KIND_OF, |_, a| (a[0] == 0x4000).into_ret());
        e.call_log = Some(vec![]);
        let null = Ptr::<()>::NULL;
        assert!(!e.call(0x0043_b300, &args![0x1234u32, null]).bool());
        assert!(calls_to(&e, IS_KIND_OF).is_empty());
        let yes = Ptr::<()>::new(0x4000);
        let no = Ptr::<()>::new(0x5000);
        assert!(e.call(0x0043_b300, &args![0x1234u32, yes]).bool());
        assert!(!e.call(0x0043_b300, &args![0x1234u32, no]).bool());
        // Arguments: the object, then the type descriptor.
        assert_eq!(calls_to(&e, IS_KIND_OF)[0], vec![0x4000, 0x1234]);
    }

    /// A node-like block with a child array of four slots at +0x38 (base)
    /// and the 16-bit count `count` at +0x3C.
    fn parent_with_children(e: &mut Engine, children: &[u32], count: u16) -> Ptr {
        let parent = e.mem.alloc(0x80);
        let base = e.mem.alloc(16);
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *child);
        }
        e.mem.set_u32(parent + 0x38, base);
        e.mem.set_u16(parent + 0x3c, count);
        Ptr::new(parent)
    }

    #[test]
    fn set_child_stores_it_and_flags_a_non_null_child() {
        let mut e = loader_engine();
        let parent = parent_with_children(&mut e, &[0, 0, 0, 0], 4);
        let child = object(&mut e, NODE_VTABLE);
        e.call(0x0043_b320, &args![parent, 2u16, child]);
        let base = e.mem.u32(parent.addr() + 0x38);
        assert_eq!(e.mem.u32(base + 8), child.addr());
        assert_eq!(e.mem.u32(child.addr() + 0x30), 0x80);
        // Storing null clears the slot and flags nothing.
        e.call(0x0043_b320, &args![parent, 2u16, Ptr::<()>::NULL]);
        assert_eq!(e.mem.u32(base + 8), 0);
    }

    #[test]
    fn flag_setter_sets_and_clears_the_0x80_bit() {
        let mut e = loader_engine();
        let target = object(&mut e, NODE_VTABLE);
        e.mem.set_u32(target.addr() + 0x30, 0x0000_0101);
        e.call(0x0043_b350, &args![target, 1u8]);
        assert_eq!(e.mem.u32(target.addr() + 0x30), 0x181);
        e.call(0x0043_b350, &args![target, 0u8]);
        assert_eq!(e.mem.u32(target.addr() + 0x30), 0x101);
    }

    #[test]
    fn flag_mask_setter_sets_and_clears_arbitrary_bits() {
        let mut e = loader_engine();
        let target = object(&mut e, NODE_VTABLE);
        e.mem.set_u32(target.addr() + 0x30, 0xf0);
        e.call(0x0043_b370, &args![target, 1u8, 0x0fu32]);
        assert_eq!(e.mem.u32(target.addr() + 0x30), 0xff);
        e.call(0x0043_b370, &args![target, 0u8, 0x3cu32]);
        assert_eq!(e.mem.u32(target.addr() + 0x30), 0xc3);
        // Any non-zero `set` byte means set.
        e.call(0x0043_b370, &args![target, 2u8, 0x100u32]);
        assert_eq!(e.mem.u32(target.addr() + 0x30), 0x1c3);
    }

    #[test]
    fn child_index_search_returns_the_slot_or_the_not_found_word() {
        let mut e = loader_engine();
        e.mem.set_u16(NOT_FOUND_INDEX, 0xffff);
        let parent = parent_with_children(&mut e, &[0x10, 0x20, 0x30, 0x40], 3);
        let find = |e: &mut Engine, child: u32| {
            e.call(0x0043_b3b0, &args![parent, Ptr::<()>::new(child)])
                .u16()
        };
        assert_eq!(find(&mut e, 0x10), 0);
        assert_eq!(find(&mut e, 0x30), 2);
        // Slot 3 holds 0x40 but is beyond the count.
        assert_eq!(find(&mut e, 0x40), 0xffff);
        assert_eq!(find(&mut e, 0x99), 0xffff);
    }

    #[test]
    fn manual_count_changes_with_compare_and_swap() {
        let mut e = loader_engine();
        let this = model(&mut e);
        e.set(this, Model::iManualRefCount, 5);
        e.call_log = Some(vec![]);
        e.call(0x0043_b410, &args![this, 3i32]);
        assert_eq!(e.get(this, Model::iManualRefCount), 8);
        assert!(calls_to(&e, SLEEP).is_empty());
        // Import arguments: target, new value, expected value.
        assert_eq!(
            calls_to(&e, INTERLOCKED_COMPARE_EXCHANGE),
            vec![vec![this.addr() + 8, 8, 5]]
        );
        e.call(0x0043_b410, &args![this, -2i32]);
        assert_eq!(e.get(this, Model::iManualRefCount), 6);
    }

    #[test]
    fn manual_count_retries_after_a_lost_race() {
        let mut e = loader_engine();
        let this = model(&mut e);
        e.set(this, Model::iManualRefCount, 5);
        // Another thread changes the count to 7 during the first attempt.
        let mut attempts = 0;
        e.register_double(INTERLOCKED_COMPARE_EXCHANGE, move |e, a| {
            attempts += 1;
            if attempts == 1 {
                e.mem.set_u32(a[0], 7);
                return 7u32.into_ret();
            }
            let current = e.mem.u32(a[0]);
            if current == a[2] {
                e.mem.set_u32(a[0], a[1]);
            }
            current.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0043_b410, &args![this, 3i32]);
        assert_eq!(e.get(this, Model::iManualRefCount), 10);
        assert_eq!(calls_to(&e, SLEEP), vec![vec![0]]);
        let attempts = calls_to(&e, INTERLOCKED_COMPARE_EXCHANGE);
        assert_eq!(attempts.len(), 2);
        assert_eq!(attempts[1], vec![this.addr() + 8, 10, 7]);
    }

    #[test]
    fn compare_exchange_wrapper_reorders_the_arguments() {
        let mut e = loader_engine();
        let cell = e.mem.alloc(8);
        e.mem.set_i32(cell, 4);
        e.call_log = Some(vec![]);
        // (target, old, new): the import takes (target, new, old).
        let seen = e.call(0x0043_b460, &args![Ptr::<()>::new(cell), 4i32, 9i32]);
        assert_eq!(seen.i32(), 4);
        assert_eq!(e.mem.i32(cell), 9);
        assert_eq!(
            calls_to(&e, INTERLOCKED_COMPARE_EXCHANGE),
            vec![vec![cell, 9, 4]]
        );
        let seen = e.call(0x0043_b460, &args![Ptr::<()>::new(cell), 4i32, 1i32]);
        assert_eq!(seen.i32(), 9);
        assert_eq!(e.mem.i32(cell), 9);
    }

    #[test]
    fn array_at_0x9c_has_a_size_and_elements() {
        let mut e = loader_engine();
        let owner = e.mem.alloc(0xb0);
        let items = e.mem.alloc(16);
        e.mem.set_u32(items, 0xaaa0);
        e.mem.set_u32(items + 4, 0xbbb0);
        e.mem.set_u32(owner + 0x9c + 4, items);
        e.mem.set_u16(owner + 0x9c + 0xa, 2);
        let owner = Ptr::<()>::new(owner);
        assert_eq!(e.call(0x0043_b480, &args![owner]).u16(), 2);
        assert_eq!(e.call(0x0043_b4a0, &args![owner, 1u32]).u32(), 0xbbb0);
    }

    #[test]
    fn low_seven_bits_are_returned() {
        let mut e = loader_engine();
        let cell = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(cell.addr(), 0xffff_ffff);
        assert_eq!(e.call(0x0043_b4d0, &args![cell]).u32(), 0x7f);
        e.mem.set_u32(cell.addr(), 0x1234_5680);
        assert_eq!(e.call(0x0043_b4d0, &args![cell]).u32(), 0);
    }

    /// Test doubles for the helpers of `0043b4f0` .. `0043b5f0`, and a
    /// block standing for the object they find.
    fn holder_engine() -> (Engine, u32) {
        let mut e = loader_engine();
        let block = e.mem.alloc(0x100);
        e.register(REF_COUNT_OBJECT_POINTER, |e, a| {
            e.mem.u32(a[0] + 8).into_ret()
        });
        e.register(DELETE_OBJECT, |_, _| Ret::default());
        e.register(HOLDER_VALUE, |_, a| (a[0] + 0x10).into_ret());
        e.register(HOLDER_DESTRUCT, |_, _| Ret::default());
        e.register(FIELD_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(STORE_POINTER, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        (e, block)
    }

    #[test]
    fn holder_helper_destroys_the_block_and_returns_its_value() {
        let (mut e, block) = holder_engine();
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 8, block);
        e.call_log = Some(vec![]);
        let value = e.call(0x0043_b560, &args![Ptr::<()>::new(owner)]).u32();
        assert_eq!(value, block + 0x10);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0043_b560, vec![owner]),
                (REF_COUNT_OBJECT_POINTER, vec![owner]),
                (DELETE_OBJECT, vec![block + 0x6c, 1]),
                (HOLDER_VALUE, vec![block]),
                (HOLDER_DESTRUCT, vec![block + 0x6c]),
            ]
        );
        // No block: 0, and nothing else is called.
        let empty = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0043_b560, &args![Ptr::<()>::new(empty)]).u32(), 0);
        assert_eq!(call_order(&e), vec![0x0043_b560, REF_COUNT_OBJECT_POINTER]);
    }

    #[test]
    fn holder_helper_pieces_work_on_the_embedded_object() {
        let (mut e, block) = holder_engine();
        let block = Ptr::<()>::new(block);
        assert_eq!(
            e.call(0x0043_b5d0, &args![block]).u32(),
            block.addr() + 0x6c
        );
        e.call_log = Some(vec![]);
        e.call(0x0043_b5b0, &args![block]);
        e.call(0x0043_b5f0, &args![block]);
        assert_eq!(
            calls_to(&e, DELETE_OBJECT),
            vec![vec![block.addr() + 0x6c, 1]]
        );
        assert_eq!(
            calls_to(&e, HOLDER_DESTRUCT),
            vec![vec![block.addr() + 0x6c]]
        );
    }

    #[test]
    fn field_at_8_of_the_object_at_0x14() {
        let (mut e, block) = holder_engine();
        e.mem.set_u32(block + 0x14 + 8, 0x4242);
        assert_eq!(
            e.call(0x0043_b540, &args![Ptr::<()>::new(block)]).u32(),
            0x4242
        );
    }

    #[test]
    fn holder_store_writes_the_value_or_zero() {
        let (mut e, block) = holder_engine();
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 8, block);
        let out = e.mem.alloc(8);
        // `0043b560` returns block + 0x10; `0043b540` reads that object's
        // field at 0x14 + 8.
        e.mem.set_u32(block + 0x10 + 0x14 + 8, 0x77);
        let owner = Ptr::<()>::new(owner);
        let out_ptr = Ptr::<()>::new(out);
        let back = e.call(0x0043_b4f0, &args![owner, out_ptr]);
        assert_eq!(back.u32(), out);
        assert_eq!(e.mem.u32(out), 0x77);
        // Nothing found: stores 0.
        let empty = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_u32(out, 5);
        e.call(0x0043_b4f0, &args![empty, out_ptr]);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn collision_object_is_cast_to_its_class() {
        let mut e = loader_engine();
        e.register(COLLISION_OBJECT, |_, a| (a[0] + 1).into_ret());
        e.register(DYNAMIC_CAST, |_, a| {
            if a[1] == 0x9001 { a[1] } else { 0 }.into_ret()
        });
        e.call_log = Some(vec![]);
        let found = e.call(0x0043_b610, &args![Ptr::<()>::new(0x9000)]);
        assert_eq!(found.u32(), 0x9001);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![TYPE_DESCRIPTOR_BHK_COLLISION_OBJECT, 0x9001]]
        );
        assert_eq!(e.call(0x0043_b610, &args![Ptr::<()>::new(0x7000)]).u32(), 0);
    }

    /// An engine for the `KFModel` functions, and a sequence object with
    /// doubles for the animation group and its helpers.
    fn kf_engine() -> (Engine, Ptr) {
        let mut e = loader_engine();
        let sequence = object(&mut e, PLAIN_VTABLE);
        e.register(LOAD_ANIM_GROUP, |_, _| 0x5555u32.into_ret());
        e.register(IS_KIND_OF, |_, _| 0u32.into_ret());
        e.register(KF_MODEL_SEQUENCE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        (e, sequence)
    }

    #[test]
    fn kf_model_without_a_sequence_logs_and_stops() {
        let (mut e, _) = kf_engine();
        e.register(LOAD_KF_SEQUENCE, |_, _| Ret::default());
        let this: Ptr<KFModel> = e.new_object();
        e.set(this, KFModel::iRefCount, 3);
        let name = text(&mut e, "idle.kf");
        let file = object(&mut e, PLAIN_VTABLE);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_b640, &args![this, name, file]);
        assert_eq!(back.ptr::<KFModel>(), this);
        assert_eq!(e.get(this, KFModel::iRefCount), 0);
        assert_eq!(e.get(this, KFModel::iManualRefCount), 0);
        let copy = e.get(this, KFModel::pFilename);
        assert_eq!(e.mem.cstr(copy), b"idle.kf");
        // The sequence was requested from the file into the NiPointer at +4.
        assert_eq!(
            calls_to(&e, LOAD_KF_SEQUENCE),
            vec![vec![file.addr(), 0, this.addr() + 4]]
        );
        assert_eq!(calls_to(&e, LOG), vec![vec![NO_SEQUENCE_MESSAGE, copy]]);
        assert!(calls_to(&e, LOAD_ANIM_GROUP).is_empty());
    }

    #[test]
    fn kf_model_with_a_sequence_loads_its_animation_group() {
        let (mut e, sequence) = kf_engine();
        e.register_double(LOAD_KF_SEQUENCE, move |e, a| {
            e.mem.set_u32(a[2], sequence.addr());
            Ret::default()
        });
        let this: Ptr<KFModel> = e.new_object();
        let name = text(&mut e, "idle.kf");
        let file = object(&mut e, PLAIN_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x0043_b640, &args![this, name, file]);
        assert!(calls_to(&e, LOG).is_empty());
        assert_eq!(
            calls_to(&e, LOAD_ANIM_GROUP),
            vec![vec![sequence.addr(), name]]
        );
        assert_eq!(e.get(this, KFModel::spSequence), sequence);
        assert_eq!(e.get(this, KFModel::spAnimGroup), Ptr::new(0x5555));
    }

    #[test]
    fn kf_model_destructor_frees_the_name_and_releases_both_pointers() {
        let (mut e, _) = kf_engine();
        let this: Ptr<KFModel> = e.new_object();
        let filename = e.mem.alloc(16);
        e.set(this, KFModel::pFilename, filename);
        e.set(this, KFModel::spSequence, Ptr::new(0x1000));
        e.set(this, KFModel::spAnimGroup, Ptr::new(0x2000));
        e.call_log = Some(vec![]);
        e.call(0x0043_b750, &args![this]);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0043_b750, vec![this.addr()]),
                (MEMORY_FREE, vec![filename]),
                (NI_POINTER_ASSIGN, vec![this.addr() + 4, 0]),
                (NI_POINTER_ASSIGN, vec![this.addr() + 8, 0]),
                (NI_POINTER_DESTRUCT, vec![this.addr() + 8]),
                (NI_POINTER_DESTRUCT, vec![this.addr() + 4]),
            ]
        );
    }

    /// A `KFModel` holding `sequence`, whose name handle is `sequence_name`.
    fn kf_model_with(e: &mut Engine, sequence: Ptr, sequence_name: &str) -> Ptr<KFModel> {
        let this: Ptr<KFModel> = e.new_object();
        e.set(this, KFModel::spSequence, sequence);
        let name = text(e, sequence_name);
        e.mem.set_u32(sequence.addr() + 8, name);
        this
    }

    #[test]
    fn finishing_a_kf_model_without_a_sequence_does_nothing() {
        let (mut e, _) = kf_engine();
        let this: Ptr<KFModel> = e.new_object();
        let name = text(&mut e, "idle.kf");
        e.call_log = Some(vec![]);
        e.call(0x0043_b7e0, &args![this, name]);
        assert_eq!(
            call_order(&e),
            vec![0x0043_b7e0, NI_POINTER_GET],
            "only the NiPointer read"
        );
    }

    #[test]
    fn finishing_a_kf_model_renames_the_sequence_with_the_part_before_the_underscore() {
        let (mut e, sequence) = kf_engine();
        let this = kf_model_with(&mut e, sequence, "Walk_Fast");
        let original = e.mem.u32(sequence.addr() + 8);
        let name = text(&mut e, "walk.kf");
        e.call_log = Some(vec![]);
        e.call(0x0043_b7e0, &args![this, name]);
        // The first temporary string was built from the text "Walk"; the
        // second from the file name; the sequence ends up named by the latter.
        let built = calls_to(&e, FIXED_STRING_CONSTRUCT);
        assert_eq!(built.len(), 2);
        assert_eq!(e.mem.cstr(built[0][1]), b"Walk");
        assert_eq!(built[1][1], name);
        assert_eq!(e.mem.u32(sequence.addr() + 8), name);
        assert_ne!(e.mem.u32(sequence.addr() + 8), original);
        // The copy took the four characters before the underscore.
        assert_eq!(calls_to(&e, MEMORY_COPY)[0][1..], [original, 4]);
        // The temporaries were destroyed, the new handles referenced and the
        // old ones released.
        assert_eq!(calls_to(&e, FIXED_STRING_DESTRUCT).len(), 2);
        assert_eq!(calls_to(&e, FIXED_STRING_RELEASE).len(), 2);
        assert_eq!(e.mem.i32(STRING_COUNTER), 2);
        // The animation group was loaded from the file name and stored.
        assert_eq!(
            calls_to(&e, LOAD_ANIM_GROUP),
            vec![vec![sequence.addr(), name]]
        );
        assert_eq!(e.get(this, KFModel::spAnimGroup), Ptr::new(0x5555));
    }

    #[test]
    fn finishing_a_kf_model_with_a_plain_name_names_it_once() {
        let (mut e, sequence) = kf_engine();
        let this = kf_model_with(&mut e, sequence, "Walk");
        let name = text(&mut e, "walk.kf");
        e.call_log = Some(vec![]);
        e.call(0x0043_b7e0, &args![this, name]);
        assert_eq!(calls_to(&e, FIXED_STRING_CONSTRUCT).len(), 1);
        assert!(calls_to(&e, MEMORY_COPY).is_empty());
        assert_eq!(e.mem.u32(sequence.addr() + 8), name);
        // A sequence without a name is also named once.
        e.mem.set_u32(sequence.addr() + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_b7e0, &args![this, name]);
        assert_eq!(calls_to(&e, FIXED_STRING_CONSTRUCT).len(), 1);
        assert!(calls_to(&e, STRCHR).is_empty());
    }

    #[test]
    fn finishing_a_kf_model_stores_the_group_in_sequences_of_the_checked_class() {
        let (mut e, sequence) = kf_engine();
        e.register(IS_KIND_OF, |_, _| 1u32.into_ret());
        let this = kf_model_with(&mut e, sequence, "Walk");
        let name = text(&mut e, "walk.kf");
        e.call_log = Some(vec![]);
        e.call(0x0043_b7e0, &args![this, name]);
        assert_eq!(
            calls_to(&e, IS_KIND_OF),
            vec![vec![sequence.addr(), TYPE_DESCRIPTOR_011C7D74]]
        );
        // The sequence's NiPointer at +0x74 holds the group.
        assert_eq!(e.mem.u32(sequence.addr() + 0x74), 0x5555);
        // The accessor at 007fa950 was asked for the model's sequence.
        assert_eq!(calls_to(&e, KF_MODEL_SEQUENCE), vec![vec![this.addr()]]);

        // Not of the class: the sequence is left alone.
        let (mut e, sequence) = kf_engine();
        let this = kf_model_with(&mut e, sequence, "Walk");
        let name = text(&mut e, "walk.kf");
        e.call(0x0043_b7e0, &args![this, name]);
        assert_eq!(e.mem.u32(sequence.addr() + 0x74), 0);
    }

    #[test]
    fn strchr_wrapper_passes_text_and_character() {
        let mut e = loader_engine();
        let word = text(&mut e, "ab_cd");
        assert_eq!(e.call(0x0043_b9d0, &args![word, 0x5fi32]).u32(), word + 2);
        assert_eq!(e.call(0x0043_b9d0, &args![word, 0x7ai32]).u32(), 0);
    }

    #[test]
    fn sequence_name_is_assigned_at_offset_8() {
        let (mut e, sequence) = kf_engine();
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0xabc0);
        e.mem.set_u32(sequence.addr() + 8, 0xdef0);
        let back = e.call(
            0x0043_b9f0,
            &args![sequence, Ptr::<NiFixedString>::new(source)],
        );
        assert_eq!(back.u32(), sequence.addr() + 8);
        assert_eq!(e.mem.u32(sequence.addr() + 8), 0xabc0);
    }

    #[test]
    fn fixed_string_assignment_references_new_releases_old() {
        let mut e = loader_engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(this, 0x1000);
        e.mem.set_u32(other, 0x2000);
        e.call_log = Some(vec![]);
        let back = e.call(
            0x0043_ba10,
            &args![
                Ptr::<NiFixedString>::new(this),
                Ptr::<NiFixedString>::new(other)
            ],
        );
        assert_eq!(back.u32(), this);
        assert_eq!(e.mem.u32(this), 0x2000);
        // The reference is added first, then the old handle is released.
        let order = call_order(&e);
        let reference = order
            .iter()
            .position(|a| *a == INTERLOCKED_INCREMENT)
            .unwrap();
        let release = order
            .iter()
            .position(|a| *a == FIXED_STRING_RELEASE)
            .unwrap();
        assert!(reference < release);
        assert_eq!(calls_to(&e, FIXED_STRING_RELEASE), vec![vec![this]]);
        assert_eq!(e.mem.i32(STRING_COUNTER), 1);

        // Equal handles: nothing happens.
        e.call_log = Some(vec![]);
        e.call(
            0x0043_ba10,
            &args![
                Ptr::<NiFixedString>::new(this),
                Ptr::<NiFixedString>::new(this)
            ],
        );
        assert!(calls_to(&e, FIXED_STRING_RELEASE).is_empty());
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn fixed_string_reference_skips_the_empty_string() {
        let mut e = loader_engine();
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x7777);
        let handle = e.mem.alloc(8);
        e.mem.set_u32(handle, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x0043_ba60, &args![Ptr::<()>::new(handle)]);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
        e.mem.set_u32(handle, 0x8888);
        e.call(0x0043_ba60, &args![Ptr::<()>::new(handle)]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![STRING_COUNTER]]
        );
        assert_eq!(calls_to(&e, FIXED_STRING_REFCOUNT), vec![vec![handle]]);
    }

    #[test]
    fn sequence_group_is_assigned_at_offset_0x74() {
        let mut e = loader_engine();
        let sequence = e.mem.alloc(0x80);
        let back = e.call(
            0x0043_baa0,
            &args![Ptr::<()>::new(sequence), Ptr::<()>::new(0x6666)],
        );
        assert_eq!(back.u32(), sequence + 0x74);
        assert_eq!(e.mem.u32(sequence + 0x74), 0x6666);
    }

    // --- Queued files: `LoadedFile`, `QueuedTexture`, `QueuedModel`. ---

    /// A `BSFile`-like object's virtual table: slot 0 deletes, slot 0x18 is
    /// the file name.
    const FILE_VTABLE: u32 = 0x0ff2_0000;
    const FILE_DELETE: u32 = 0x0ff0_0100;
    const FILE_NAME_SLOT: u32 = 0x0ff0_0101;
    /// The message sink's table: slot 0x14 reports.
    const SINK_VTABLE: u32 = 0x0ff2_1000;
    const SINK_REPORT: u32 = 0x0ff0_0102;
    /// A task's table: slot 0x28 is `CheckFinished`.
    const TASK_VTABLE: u32 = 0x0ff2_2000;
    const CHECK_FINISHED: u32 = 0x0ff0_0103;
    /// The task queue's table: slot 0x48 adds a task.
    const QUEUE_VTABLE: u32 = 0x0ff2_3000;
    const QUEUE_ADD: u32 = 0x0ff0_0104;
    /// A texture's table: slot 0x8c is the hook `Run` calls.
    const TEXTURE_VTABLE: u32 = 0x0ff2_4000;
    const TEXTURE_HOOK: u32 = 0x0ff0_0105;
    /// Where the doubles for the texture map and object keep their state.
    const OWNER_OBJECT: u32 = 0x0ff3_0000;
    const MAP_OBJECT: u32 = 0x0ff3_1000;

    fn put_slots(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) {
        let mut table = vec![ANSWERS_NO; 0x80];
        for (offset, target) in slots {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &table);
    }

    /// Sets (`on`) or clears the bit `mask` of the byte at `address`, as the
    /// game's flag helpers do.
    fn write_flag(e: &mut Engine, address: u32, mask: u8, on: bool) {
        let byte = e.mem.u8(address);
        e.mem
            .set_u8(address, if on { byte | mask } else { byte & !mask });
    }

    /// An engine with doubles for what the queued-file functions call.
    fn queue_engine() -> Engine {
        let mut e = loader_engine();
        for page in [0x011c_3000, 0x011f_6000] {
            e.map(page, 0x1000);
        }
        // The exe's string the suffix test searches for.
        e.map(0x0101_6000, 0x1000);
        e.mem.set_cstr(ENVIRONMENT_MAP_SUFFIX, b"_e.dd");
        e.register(SET_FLAG_BIT_1, |e, a| {
            write_flag(e, a[1], 1, a[0] & 0xff != 0);
            Ret::default()
        });
        e.register(SET_FLAG_BIT_2, |e, a| {
            write_flag(e, a[1], 2, a[0] & 0xff != 0);
            Ret::default()
        });
        e.register(SET_FLAG_BIT_4, |e, a| {
            write_flag(e, a[1], 4, a[0] & 0xff != 0);
            Ret::default()
        });
        e.register(TEST_FLAG_BIT_2, |_, a| (a[0] & 2 != 0).into_ret());
        e.register(TEST_FLAG_BIT_4, |_, a| (a[0] & 4 != 0).into_ret());
        e.register(TEST_FLAG_BIT_1, |_, a| (a[0] & 1 != 0).into_ret());
        e.register(QUEUED_FILE_ENTRY_GET_FILE_ENTRY, |e, a| {
            e.mem.u32(a[0] + 0x2c).into_ret()
        });
        e.register(QUEUED_FILE_ENTRY_GET_FILE_NAME, |e, a| {
            e.mem.u32(a[0] + 0x28).into_ret()
        });
        e.register(FILE_ENTRY_GET_SIZE, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x3fff_ffff).into_ret()
        });
        e.register(TEXTURE_GET_SURFACE, |e, a| {
            e.mem.u32(a[0] + 0x24).into_ret()
        });
        e.register(STRSTR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let pattern = e.mem.cstr(a[1]);
            match text.windows(pattern.len()).position(|w| w == &pattern[..]) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(SPRINTF, |e, a| {
            e.mem.set_cstr(a[0], b"formatted");
            Ret::default()
        });
        e.register(NORMALIZE_PATH, |e, a| {
            let text = e.mem.cstr(a[0]);
            e.mem.set_cstr(a[1], &text);
            Ret::default()
        });
        for address in [
            QUEUED_FILE_ENTRY_CONSTRUCT,
            QUEUED_FILE_ENTRY_DESTRUCT,
            QUEUED_FILE_ENTRY_SET_FILE_NAME,
            QUEUED_FILE_ENTRY_FIND_FILE_ENTRY,
            QUEUED_FILE_ENTRY_SET_FILE_ENTRY,
            QUEUED_FILE_CANCEL,
            TASK_SET_DONE,
            MEMORY_CONTEXT_ENTER,
            MEMORY_CONTEXT_LEAVE,
            MODEL_POINTER_RELEASE,
            FILE_MAP_REMOVE_LOADED_FILE,
            FILE_MAP_REMOVE_QUEUED_TEXTURE,
            TEXTURE_PALETTE_SET_TEXTURE,
            TEXTURE_PALETTE_GET_TEXTURE_BY_NAME,
            FILE_DELETE,
            SINK_REPORT,
            CHECK_FINISHED,
            QUEUE_ADD,
            TEXTURE_HOOK,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(FILE_MAP_ADD_LOADED_FILE, |_, _| 1u32.into_ret());
        e.register(FILE_MAP_ADD_QUEUED_TEXTURE, |_, _| 1u32.into_ret());
        e.register(QUEUED_FILE_ENTRY_GET_DESCRIPTION, |_, _| 1u32.into_ret());
        e.register(FILE_NAME_SLOT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(TEXTURE_MAP_FIND, |_, _| 0u32.into_ret());
        e.register(CREATE_TEXTURE_FROM_FILE, |_, _| 0x7001u32.into_ret());
        e.register(CREATE_TEXTURE_FROM_FIXED_NAME, |_, _| 0x7002u32.into_ret());
        put_slots(
            &mut e,
            FILE_VTABLE,
            &[(0, FILE_DELETE), (0x18, FILE_NAME_SLOT)],
        );
        put_slots(&mut e, SINK_VTABLE, &[(0x14, SINK_REPORT)]);
        put_slots(&mut e, TASK_VTABLE, &[(0x28, CHECK_FINISHED)]);
        put_slots(&mut e, QUEUE_VTABLE, &[(0x48, QUEUE_ADD)]);
        put_slots(&mut e, TEXTURE_VTABLE, &[(0x8c, TEXTURE_HOOK)]);
        e
    }

    /// A queued texture whose first word is the task table (so slot `0x28`
    /// is `CheckFinished`).
    fn queued_texture(e: &mut Engine) -> Ptr<QueuedTexture> {
        let texture: Ptr<QueuedTexture> = e.new_object();
        e.mem.set_u32(texture.addr(), TASK_VTABLE);
        texture
    }

    /// A file entry with the given offset and size words.
    fn file_entry(e: &mut Engine, offset: u32, size: u32) -> Ptr {
        let entry = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(entry.addr() + 8, size);
        e.mem.set_u32(entry.addr() + 0xc, offset);
        entry
    }

    /// An open file whose name (slot `0x18`) is `name`.
    fn open_file(e: &mut Engine, name: &str) -> Ptr {
        let file = Ptr::new(e.mem.alloc(0x10));
        let text = text(e, name);
        e.mem.set_u32(file.addr(), FILE_VTABLE);
        e.mem.set_u32(file.addr() + 4, text);
        file
    }

    #[test]
    fn children_counter_is_decremented_only_when_positive() {
        let mut e = queue_engine();
        let children: Ptr<QueuedChildren> = e.new_object();
        for (before, after) in [(3, 2), (1, 0), (0, 0), (-5, -5)] {
            e.set(
                children,
                QueuedChildren::iNumChildrenFinished,
                before as u32,
            );
            e.call(0x0043_bac0, &args![children]);
            assert_eq!(
                e.get(children, QueuedChildren::iNumChildrenFinished) as i32,
                after
            );
        }
    }

    #[test]
    fn loaded_file_constructor_copies_the_name_and_joins_the_map() {
        let mut e = queue_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        let this: Ptr<LoadedFile> = e.new_object();
        e.set(this, LoadedFile::iRefCount, 9);
        e.set(this, LoadedFile::bFileUsed, true);
        let name = text(&mut e, "meshes\\a.nif");
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_baf0, &args![this, name, Ptr::<()>::new(0x4321)]);
        assert_eq!(back.ptr::<LoadedFile>(), this);
        let copy = e.get(this, LoadedFile::pFileName);
        assert_ne!(copy, name);
        assert_eq!(e.mem.cstr(copy), b"meshes\\a.nif");
        assert_eq!(e.get(this, LoadedFile::pFile), Ptr::new(0x4321));
        assert!(e.get(this, LoadedFile::bInLoadedFileMap));
        assert!(!e.get(this, LoadedFile::bFileUsed));
        assert_eq!(e.get(this, LoadedFile::iRefCount), 0);
        assert_eq!(
            calls_to(&e, FILE_MAP_ADD_LOADED_FILE),
            vec![vec![OWNER_OBJECT, copy, this.addr()]]
        );
        // A refused add leaves the flag clear.
        e.register(FILE_MAP_ADD_LOADED_FILE, |_, _| 0u32.into_ret());
        e.call(0x0043_baf0, &args![this, name, Ptr::<()>::new(0)]);
        assert!(!e.get(this, LoadedFile::bInLoadedFileMap));
    }

    /// A loaded file with a name, a file object and the given state.
    fn loaded_file(
        e: &mut Engine,
        in_map: bool,
        used: bool,
        references: i32,
    ) -> (Ptr<LoadedFile>, u32, Ptr) {
        let this: Ptr<LoadedFile> = e.new_object();
        let name = text(e, "a.nif");
        let file = open_file(e, "a.nif");
        e.set(this, LoadedFile::pFileName, name);
        e.set(this, LoadedFile::pFile, file);
        e.set(this, LoadedFile::bInLoadedFileMap, in_map);
        e.set(this, LoadedFile::bFileUsed, used);
        e.set(this, LoadedFile::iRefCount, references);
        (this, name, file)
    }

    #[test]
    fn loaded_file_destructor_reports_an_unused_mapped_file() {
        let mut e = queue_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        let (this, name, file) = loaded_file(&mut e, true, false, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_bb80, &args![this]);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![LOADED_FILE_UNUSED_MESSAGE, name, this.addr()]]
        );
        assert_eq!(
            calls_to(&e, FILE_MAP_REMOVE_LOADED_FILE),
            vec![vec![OWNER_OBJECT, name]]
        );
        // No references: nothing is formatted or reported.
        assert!(calls_to(&e, SPRINTF).is_empty());
        assert!(calls_to(&e, SINK_REPORT).is_empty());
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![name]]);
        // The file is deleted last, through slot 0 with the delete flag.
        assert_eq!(calls_to(&e, FILE_DELETE), vec![vec![file.addr(), 1]]);
        assert_eq!(*call_order(&e).last().unwrap(), FILE_DELETE);
    }

    #[test]
    fn loaded_file_destructor_reports_leftover_references_to_the_sink() {
        let mut e = queue_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        let sink = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(sink.addr(), SINK_VTABLE);
        e.set_global(MESSAGE_SINK, sink.addr());
        // Used and not in the map: no complaint, no map removal.
        let (this, name, _) = loaded_file(&mut e, false, true, 2);
        e.call_log = Some(vec![]);
        e.call(0x0043_bb80, &args![this]);
        assert!(calls_to(&e, LOG).is_empty());
        assert!(calls_to(&e, FILE_MAP_REMOVE_LOADED_FILE).is_empty());
        let formats = calls_to(&e, SPRINTF);
        assert_eq!(formats.len(), 1);
        assert_eq!(
            formats[0][1..],
            [LOADED_FILE_REFERENCES_MESSAGE, name, this.addr(), 2]
        );
        let reports = calls_to(&e, SINK_REPORT);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0][0], sink.addr());
        assert_eq!(reports[0][1], 4);
        assert_eq!(reports[0][2], formats[0][0]);
        assert_eq!(reports[0][3..], [0, 0, 0]);
        // Without a sink the references are formatted but not reported.
        e.set_global(MESSAGE_SINK, 0u32);
        let (this, _, _) = loaded_file(&mut e, false, true, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_bb80, &args![this]);
        assert_eq!(calls_to(&e, SPRINTF).len(), 1);
        assert!(calls_to(&e, SINK_REPORT).is_empty());
    }

    #[test]
    fn loaded_file_destructor_without_a_file_deletes_nothing() {
        let mut e = queue_engine();
        let (this, _, _) = loaded_file(&mut e, false, true, 0);
        e.set(this, LoadedFile::pFile, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0043_bb80, &args![this]);
        assert!(calls_to(&e, FILE_DELETE).is_empty());
        assert_eq!(calls_to(&e, MEMORY_FREE).len(), 1);
    }

    #[test]
    fn message_sink_is_the_word_at_011f6388() {
        let mut e = queue_engine();
        assert!(!e.call(0x0043_bce0, &args![]).bool());
        assert_eq!(e.call(0x0043_bd00, &args![]).u32(), 0);
        e.set_global(MESSAGE_SINK, 0x1234u32);
        assert!(e.call(0x0043_bce0, &args![]).bool());
        assert_eq!(e.call(0x0043_bd00, &args![]).u32(), 0x1234);
    }

    #[test]
    fn texture_constructor_by_name_normalizes_the_path() {
        let mut e = queue_engine();
        let this: Ptr<QueuedTexture> = e.new_object();
        e.set(this, QueuedTexture::cFlags, 0xff);
        let name = text(&mut e, "textures\\a.dds");
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_bd10, &args![this, name, 0x77u32]);
        assert_eq!(back.ptr::<QueuedTexture>(), this);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_TEXTURE_VTABLE);
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x77]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_CONSTRUCT),
            vec![vec![this.addr() + 0x30, 0]]
        );
        let normalized = calls_to(&e, NORMALIZE_PATH);
        assert_eq!(normalized.len(), 1);
        assert_eq!(normalized[0][0], name);
        assert_eq!(normalized[0][2], 0x104);
        // The normalized buffer is what becomes the file name.
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME),
            vec![vec![this.addr(), normalized[0][1]]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_FIND_FILE_ENTRY),
            vec![vec![this.addr(), 1]]
        );
    }

    #[test]
    fn child_notification_counts_and_asks_whether_finished() {
        let mut e = queue_engine();
        let this = Ptr::<QueuedFile>::new(e.mem.alloc(0x28));
        e.mem.set_u32(this.addr(), TASK_VTABLE);
        let children: Ptr<QueuedChildren> = e.new_object();
        e.set(this, QueuedFile::pChildren, children.cast());
        e.call_log = Some(vec![]);
        e.call(0x0043_bde0, &args![this, Ptr::<()>::new(0x99)]);
        assert_eq!(e.get(children, QueuedChildren::iNumChildrenFinished), 1);
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![this.addr()]]);
    }

    #[test]
    fn children_counter_is_incremented() {
        let mut e = queue_engine();
        let children: Ptr<QueuedChildren> = e.new_object();
        e.call(0x0043_be10, &args![children]);
        e.call(0x0043_be10, &args![children]);
        assert_eq!(e.get(children, QueuedChildren::iNumChildrenFinished), 2);
    }

    #[test]
    fn texture_scalar_deleting_destructor_frees_only_on_request() {
        let mut e = queue_engine();
        let this = queued_texture(&mut e);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_be30, &args![this, 0u32]);
        assert_eq!(back.ptr::<QueuedTexture>(), this);
        assert_eq!(
            call_order(&e),
            vec![0x0043_be30, NI_POINTER_DESTRUCT, QUEUED_FILE_ENTRY_DESTRUCT]
        );
        e.call_log = Some(vec![]);
        e.call(0x0043_be30, &args![this, 3u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
        // Only bit 0 of the flags counts.
        e.call_log = Some(vec![]);
        e.call(0x0043_be30, &args![this, 2u32]);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
    }

    #[test]
    fn texture_constructor_by_file_entry_stores_the_entry() {
        let mut e = queue_engine();
        let this: Ptr<QueuedTexture> = e.new_object();
        e.set(this, QueuedTexture::cFlags, 0xff);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_be60, &args![this, Ptr::<()>::new(0x5150), 0x66u32]);
        assert_eq!(back.ptr::<QueuedTexture>(), this);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_TEXTURE_VTABLE);
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x66]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_ENTRY),
            vec![vec![this.addr(), 0x5150]]
        );
        assert!(calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME).is_empty());
    }

    #[test]
    fn texture_constructor_by_texture_marks_the_task_done() {
        let mut e = queue_engine();
        let this: Ptr<QueuedTexture> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_bef0, &args![this, Ptr::<()>::new(0x4040), 0x66u32]);
        assert_eq!(back.ptr::<QueuedTexture>(), this);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_TEXTURE_VTABLE);
        assert_eq!(e.get(this, QueuedTexture::spTexture), Ptr::new(0x4040));
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0);
        let order = call_order(&e);
        // Constructed empty first, then assigned, then marked done.
        let construct = order
            .iter()
            .position(|a| *a == NI_POINTER_CONSTRUCT)
            .unwrap();
        let assign = order.iter().position(|a| *a == NI_POINTER_ASSIGN).unwrap();
        let done = order.iter().position(|a| *a == TASK_SET_DONE).unwrap();
        assert!(construct < assign && assign < done);
        assert_eq!(calls_to(&e, TASK_SET_DONE), vec![vec![this.addr()]]);
    }

    #[test]
    fn texture_destructor_restores_the_table_and_runs_the_base() {
        let mut e = queue_engine();
        let this: Ptr<QueuedTexture> = e.new_object();
        e.mem.set_u32(this.addr(), 0x1111);
        e.call_log = Some(vec![]);
        e.call(0x0043_bf80, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_TEXTURE_VTABLE);
        assert_eq!(
            call_order(&e)[1..],
            [NI_POINTER_DESTRUCT, QUEUED_FILE_ENTRY_DESTRUCT]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_DESTRUCT),
            vec![vec![this.addr() + 0x30]]
        );
    }

    #[test]
    fn file_index_is_the_archive_word_plus_four() {
        let mut e = queue_engine();
        let this = queued_texture(&mut e);
        // No file entry.
        assert_eq!(e.call(0x0043_bfe0, &args![this]).u32(), 0);
        let entry = file_entry(&mut e, 0, 0);
        e.set(this, QueuedTexture::pFileEntry, entry);
        // A file entry without an archive.
        e.register(FILE_ENTRY_FIND_ARCHIVE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0043_bfe0, &args![this]).u32(), 0);
        assert_eq!(
            calls_to(&e, FILE_ENTRY_FIND_ARCHIVE),
            vec![vec![entry.addr(), 2]]
        );
        // With an archive.
        let archive = e.mem.alloc(0x200);
        e.mem.set_u32(archive + 0x1c8, 11);
        e.register_double(FILE_ENTRY_FIND_ARCHIVE, move |_, _| archive.into_ret());
        assert_eq!(e.call(0x0043_bfe0, &args![this]).u32(), 15);
    }

    #[test]
    fn archive_word_is_read_at_0x1c8() {
        let mut e = queue_engine();
        let archive = e.mem.alloc(0x200);
        e.mem.set_u32(archive + 0x1c8, 0xabcd);
        assert_eq!(
            e.call(0x0043_c030, &args![Ptr::<()>::new(archive)]).u32(),
            0xabcd
        );
    }

    /// A task queue object for `QueueMe`.
    fn task_queue(e: &mut Engine) -> u32 {
        let queue = e.mem.alloc(8);
        e.mem.set_u32(queue, QUEUE_VTABLE);
        e.set_global(TASK_QUEUE, queue);
        queue
    }

    #[test]
    fn queue_me_registers_the_file_entry_and_sets_the_thread_flag_bit() {
        let mut e = queue_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        let queue = task_queue(&mut e);
        let this = queued_texture(&mut e);
        let entry = file_entry(&mut e, 0, 0);
        e.set(this, QueuedTexture::pFileEntry, entry);
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_QUEUED_FLAG, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_c050, &args![this]);
        assert_eq!(
            calls_to(&e, FILE_MAP_ADD_QUEUED_TEXTURE),
            vec![vec![OWNER_OBJECT, entry.addr(), this.addr()]]
        );
        // Added to the map (bit 4) and queued with the thread flag set (bit 2).
        assert_eq!(e.get(this, QueuedTexture::cFlags), 6);
        assert_eq!(calls_to(&e, QUEUE_ADD), vec![vec![queue, this.addr()]]);
    }

    #[test]
    fn queue_me_without_entry_or_thread_flag_only_queues() {
        let mut e = queue_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        task_queue(&mut e);
        let this = queued_texture(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0043_c050, &args![this]);
        assert!(calls_to(&e, FILE_MAP_ADD_QUEUED_TEXTURE).is_empty());
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0);
        assert_eq!(calls_to(&e, QUEUE_ADD).len(), 1);
        // A refused add clears bit 4.
        let entry = file_entry(&mut e, 0, 0);
        e.set(this, QueuedTexture::pFileEntry, entry);
        e.set(this, QueuedTexture::cFlags, 4);
        e.register(FILE_MAP_ADD_QUEUED_TEXTURE, |_, _| 0u32.into_ret());
        e.call(0x0043_c050, &args![this]);
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0);
    }

    #[test]
    fn texture_flag_bit_2_is_set_and_cleared() {
        let mut e = queue_engine();
        let this = queued_texture(&mut e);
        e.set(this, QueuedTexture::cFlags, 0x01);
        e.call(0x0043_c0d0, &args![this, 1u8]);
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0x03);
        e.call(0x0043_c0d0, &args![this, 0u8]);
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0x01);
    }

    #[test]
    fn texture_flag_bit_4_is_set_and_cleared() {
        let mut e = queue_engine();
        let this = queued_texture(&mut e);
        e.set(this, QueuedTexture::cFlags, 0x02);
        e.call(0x0043_c100, &args![this, 1u8]);
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0x06);
        e.call(0x0043_c100, &args![this, 0u8]);
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0x02);
    }

    #[test]
    fn thread_flag_is_the_tls_byte_at_0x25c() {
        let mut e = queue_engine();
        assert_eq!(e.call(0x0043_c130, &args![0u32]).u8(), 0);
        let tls = e.tls();
        e.mem.set_u8(tls + 0x25c, 7);
        assert_eq!(e.call(0x0043_c130, &args![0u32]).u8(), 7);
    }

    /// Doubles and objects for `Run`: the texture map finds nothing unless
    /// told otherwise, the memory context is a guard of 4 bytes.
    fn run_scene(e: &mut Engine) -> Ptr<QueuedTexture> {
        e.set_global(TEXTURE_MAP, MAP_OBJECT);
        e.set_global(DEFAULT_TEXTURE_SETTING, 0x5a5a_u32);
        let this = queued_texture(e);
        e.set(this, QueuedTexture::eContext, 0x55);
        this
    }

    #[test]
    fn run_keeps_a_texture_the_palette_already_has() {
        let mut e = queue_engine();
        let this = run_scene(&mut e);
        let entry = file_entry(&mut e, 0, 0);
        e.set(this, QueuedTexture::pFileEntry, entry);
        // The texture has a surface whose format byte is 3.
        let texture = e.mem.alloc(0x40);
        e.mem.set_u32(texture, TEXTURE_VTABLE);
        let surface = e.mem.alloc(0x80);
        e.mem.set_u32(texture + 0x24, surface);
        e.mem.set_u8(surface + 0x58 + 3, 3);
        e.register_double(TEXTURE_MAP_FIND, move |e, a| {
            e.mem.set_u32(a[3], texture);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0043_c150, &args![this]);
        // The guard is entered with the task's context and left again.
        let enter = calls_to(&e, MEMORY_CONTEXT_ENTER);
        assert_eq!(enter.len(), 1);
        assert_eq!(enter[0][1..], [0x55, 1, MODEL_LOADER_SOURCE, 0x2b7]);
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE), vec![vec![enter[0][0]]]);
        // Looked up by file entry in the bucket of the entry's address.
        let bucket = (entry.addr() >> 4) % 1001;
        assert_eq!(
            calls_to(&e, TEXTURE_MAP_FIND),
            vec![vec![MAP_OBJECT, bucket, entry.addr(), this.addr() + 0x30]]
        );
        assert_eq!(e.get(this, QueuedTexture::spTexture), Ptr::new(texture));
        // Flag bit 1 is set; the format byte 3 with bit 2 clear calls the hook.
        assert_eq!(
            calls_to(&e, QUEUED_TEXTURE_SET_FLAG_1),
            vec![vec![this.addr(), 1]]
        );
        assert_eq!(calls_to(&e, TEXTURE_HOOK), vec![vec![texture]]);
        // Nothing was loaded from a file.
        assert!(calls_to(&e, QUEUED_FILE_ENTRY_GET_FILE).is_empty());
        // With bit 2 set the hook is not called.
        e.set(this, QueuedTexture::cFlags, 2);
        e.call_log = Some(vec![]);
        e.call(0x0043_c150, &args![this]);
        assert!(calls_to(&e, TEXTURE_HOOK).is_empty());
        // Nor with another format byte.
        e.set(this, QueuedTexture::cFlags, 0);
        e.mem.set_u8(surface + 0x58 + 3, 4);
        e.call(0x0043_c150, &args![this]);
        assert!(calls_to(&e, TEXTURE_HOOK).is_empty());
    }

    #[test]
    fn run_looks_a_texture_up_by_name_without_a_file_entry() {
        let mut e = queue_engine();
        let this = run_scene(&mut e);
        let name = text(&mut e, "a.dds");
        e.set(this, QueuedTexture::pFileName, name);
        let texture = e.mem.alloc(0x40);
        e.mem.set_u32(texture, TEXTURE_VTABLE);
        e.register_double(TEXTURE_PALETTE_GET_TEXTURE_BY_NAME, move |e, a| {
            e.mem.set_u32(a[1], texture);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0043_c150, &args![this]);
        assert_eq!(
            calls_to(&e, TEXTURE_PALETTE_GET_TEXTURE_BY_NAME),
            vec![vec![name, this.addr() + 0x30]]
        );
        assert!(calls_to(&e, TEXTURE_MAP_FIND).is_empty());
        assert_eq!(e.get(this, QueuedTexture::spTexture), Ptr::new(texture));
        // A texture without a surface: no hook.
        assert!(calls_to(&e, TEXTURE_HOOK).is_empty());
    }

    #[test]
    fn run_loads_a_plain_texture_from_its_file() {
        let mut e = queue_engine();
        let this = run_scene(&mut e);
        let entry = file_entry(&mut e, 0, 0);
        e.set(this, QueuedTexture::pFileEntry, entry);
        let file = open_file(&mut e, "textures\\wall.dds");
        let name = e.mem.u32(file.addr() + 4);
        e.register(QUEUED_FILE_ENTRY_GET_FILE, |e, a| {
            // The task's file: the one stored in the scene.
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.mem.set_u32(this.addr() + 0x10, file.addr());
        // The thread value is 9 before and 1 while the file is opened with
        // bit 2 set; the creator sees it.
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_QUEUED_VALUE, 9);
        e.set(this, QueuedTexture::cFlags, 2);
        e.register_double(CREATE_TEXTURE_FROM_FILE, move |e, _| {
            let tls = e.tls();
            assert_eq!(e.mem.u32(tls + TLS_QUEUED_VALUE), 1);
            0x7001u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0043_c150, &args![this]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_FILE),
            vec![vec![this.addr(), 1, 2]]
        );
        assert_eq!(
            calls_to(&e, CREATE_TEXTURE_FROM_FILE),
            vec![vec![file.addr(), name, TEXTURE_FORMAT_PREFERENCES, 1]]
        );
        assert!(calls_to(&e, CREATE_TEXTURE_FROM_FIXED_NAME).is_empty());
        assert_eq!(e.get(this, QueuedTexture::spTexture), Ptr::new(0x7001));
        // The thread value is back to what it was.
        assert_eq!(e.mem.u32(tls + TLS_QUEUED_VALUE), 9);
    }

    #[test]
    fn run_loads_an_environment_map_through_a_fixed_string() {
        let mut e = queue_engine();
        let this = run_scene(&mut e);
        let entry = file_entry(&mut e, 0, 0);
        e.set(this, QueuedTexture::pFileEntry, entry);
        let file = open_file(&mut e, "textures\\sky_e.dds");
        let name = e.mem.u32(file.addr() + 4);
        e.register(QUEUED_FILE_ENTRY_GET_FILE, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.mem.set_u32(this.addr() + 0x10, file.addr());
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_QUEUED_VALUE, 9);
        e.call_log = Some(vec![]);
        e.call(0x0043_c150, &args![this]);
        // The fixed string is built from the name and destroyed afterwards.
        let constructed = calls_to(&e, FIXED_STRING_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0][1], name);
        let local = constructed[0][0];
        assert_eq!(
            calls_to(&e, CREATE_TEXTURE_FROM_FIXED_NAME),
            vec![vec![local, file.addr(), 0x5a5a, TEXTURE_FORMAT_PREFERENCES]]
        );
        assert_eq!(calls_to(&e, FIXED_STRING_DESTRUCT), vec![vec![local]]);
        assert!(calls_to(&e, CREATE_TEXTURE_FROM_FILE).is_empty());
        assert_eq!(e.get(this, QueuedTexture::spTexture), Ptr::new(0x7002));
        // Bit 2 was clear, so the thread value was never changed.
        assert_eq!(e.mem.u32(tls + TLS_QUEUED_VALUE), 9);
    }

    #[test]
    fn run_logs_why_no_file_could_be_opened() {
        let mut e = queue_engine();
        let this = run_scene(&mut e);
        e.register(QUEUED_FILE_ENTRY_GET_FILE, |_, _| 0u32.into_ret());
        // With a file entry: its offset (top bit masked) and size (top two
        // bits masked).
        let entry = file_entry(&mut e, 0x8000_1234, 0xc000_0777);
        e.set(this, QueuedTexture::pFileEntry, entry);
        e.call_log = Some(vec![]);
        e.call(0x0043_c150, &args![this]);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![NO_FILE_FOR_TEXTURE_ENTRY_MESSAGE, 0x1234, 0x777]]
        );
        // With only a name.
        let this = run_scene(&mut e);
        let name = text(&mut e, "missing.dds");
        e.set(this, QueuedTexture::pFileName, name);
        e.call_log = Some(vec![]);
        e.call(0x0043_c150, &args![this]);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![NO_FILE_FOR_TEXTURE_NAME_MESSAGE, name]]
        );
        // With neither: silence.
        let this = run_scene(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0043_c150, &args![this]);
        assert!(calls_to(&e, LOG).is_empty());
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    #[test]
    fn file_entry_offset_drops_the_top_bit() {
        let mut e = queue_engine();
        let entry = file_entry(&mut e, 0xffff_ffff, 0);
        assert_eq!(e.call(0x0043_c3b0, &args![entry]).u32(), 0x7fff_ffff);
    }

    #[test]
    fn texture_flag_bit_2_test() {
        let mut e = queue_engine();
        let this = queued_texture(&mut e);
        for (flags, expected) in [(0, false), (1, false), (2, true), (7, true), (4, false)] {
            e.set(this, QueuedTexture::cFlags, flags);
            assert_eq!(e.call(0x0043_c3d0, &args![this]).bool(), expected);
        }
    }

    #[test]
    fn thread_value_is_stored_and_read_at_0x29c() {
        let mut e = queue_engine();
        e.call(0x0043_c3f0, &args![0x1234u32]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + 0x29c), 0x1234);
        assert_eq!(e.call(0x0043_c410, &args![]).u32(), 0x1234);
    }

    #[test]
    fn surface_format_byte_is_zero_without_a_surface() {
        let mut e = queue_engine();
        let texture = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0043_c430, &args![Ptr::<()>::new(texture)]).u32(),
            0
        );
        let surface = e.mem.alloc(0x80);
        e.mem.set_u32(texture + 0x24, surface);
        e.mem.set_u8(surface + 0x58 + 3, 0x2a);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0043_c430, &args![Ptr::<()>::new(texture)]).u32(),
            0x2a
        );
        // The surface getter is asked twice, as the game does.
        assert_eq!(calls_to(&e, TEXTURE_GET_SURFACE).len(), 2);
    }

    #[test]
    fn byte_at_offset_3_is_read() {
        let mut e = queue_engine();
        let block = e.mem.alloc(8);
        e.mem.set_u8(block + 3, 0x42);
        assert_eq!(
            e.call(0x0043_c470, &args![Ptr::<()>::new(block)]).u8(),
            0x42
        );
    }

    #[test]
    fn descriptor_address_is_surface_plus_0x58() {
        let mut e = queue_engine();
        assert_eq!(
            e.call(0x0043_c490, &args![Ptr::<()>::new(0x1000)]).u32(),
            0x1058
        );
    }

    #[test]
    fn default_texture_setting_is_the_word_at_011f4748() {
        let mut e = queue_engine();
        e.set_global(DEFAULT_TEXTURE_SETTING, 0xfeedu32);
        assert_eq!(e.call(0x0043_c4b0, &args![]).u32(), 0xfeed);
    }

    #[test]
    fn palette_get_texture_clears_the_output_and_searches_the_map() {
        let mut e = queue_engine();
        e.set_global(TEXTURE_MAP, MAP_OBJECT);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xdead);
        e.register_double(TEXTURE_MAP_FIND, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0043_c4c0, &args![0x0001_2340u32, Ptr::<()>::new(out)]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(call_order(&e)[1..], [NI_POINTER_ASSIGN, TEXTURE_MAP_FIND]);
        assert_eq!(
            calls_to(&e, TEXTURE_MAP_FIND),
            vec![vec![MAP_OBJECT, 0x1234 % 1001, 0x0001_2340, out]]
        );
    }

    #[test]
    fn palette_lookup_returns_what_the_map_says() {
        let mut e = queue_engine();
        let out = e.mem.alloc(4);
        for found in [false, true] {
            e.register_double(TEXTURE_MAP_FIND, move |_, _| found.into_ret());
            let back = e.call(
                0x0043_c4f0,
                &args![Ptr::<()>::new(MAP_OBJECT), 0x100u32, Ptr::<()>::new(out)],
            );
            assert_eq!(back.bool(), found);
        }
    }

    #[test]
    fn palette_bucket_is_the_key_over_16_modulo_1001() {
        let mut e = queue_engine();
        for key in [0u32, 15, 16, 0x1000, 0x0123_4560, 0xffff_ffff] {
            assert_eq!(e.call(0x0043_c530, &args![key]).u32(), (key >> 4) % 1001);
        }
        assert_eq!(e.call(0x0043_c530, &args![1001u32 * 16]).u32(), 0);
    }

    #[test]
    fn finish_stores_the_texture_in_the_palette_by_file_entry() {
        let mut e = queue_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        let this = queued_texture(&mut e);
        let entry = file_entry(&mut e, 0, 0);
        e.set(this, QueuedTexture::pFileEntry, entry);
        e.set(this, QueuedTexture::spTexture, Ptr::new(0x6001));
        // Bit 4: the texture is in the map of queued textures.
        e.set(this, QueuedTexture::cFlags, 4);
        e.call_log = Some(vec![]);
        e.call(0x0043_c550, &args![this]);
        assert_eq!(
            calls_to(&e, TEXTURE_PALETTE_SET_TEXTURE),
            vec![vec![0x6001, entry.addr()]]
        );
        // Taken out of the queued map, bit 4 cleared, then CheckFinished.
        assert_eq!(
            calls_to(&e, FILE_MAP_REMOVE_QUEUED_TEXTURE),
            vec![vec![OWNER_OBJECT, entry.addr()]]
        );
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0);
        assert_eq!(*call_order(&e).last().unwrap(), CHECK_FINISHED);
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![this.addr()]]);
    }

    #[test]
    fn finish_stores_a_name_only_texture_without_an_entry() {
        let mut e = queue_engine();
        let this = queued_texture(&mut e);
        let name = text(&mut e, "a.dds");
        e.set(this, QueuedTexture::pFileName, name);
        e.set(this, QueuedTexture::spTexture, Ptr::new(0x6002));
        e.call_log = Some(vec![]);
        e.call(0x0043_c550, &args![this]);
        assert_eq!(
            calls_to(&e, TEXTURE_PALETTE_SET_TEXTURE),
            vec![vec![0x6002, 0]]
        );
        assert!(calls_to(&e, FILE_MAP_REMOVE_QUEUED_TEXTURE).is_empty());
        assert_eq!(calls_to(&e, CHECK_FINISHED).len(), 1);
    }

    #[test]
    fn finish_skips_the_palette_when_flag_bit_1_is_set_or_no_texture() {
        let mut e = queue_engine();
        let this = queued_texture(&mut e);
        let entry = file_entry(&mut e, 0, 0);
        e.set(this, QueuedTexture::pFileEntry, entry);
        e.set(this, QueuedTexture::spTexture, Ptr::new(0x6003));
        e.set(this, QueuedTexture::cFlags, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_c550, &args![this]);
        assert!(calls_to(&e, TEXTURE_PALETTE_SET_TEXTURE).is_empty());
        // No texture at all.
        e.set(this, QueuedTexture::cFlags, 0);
        e.set(this, QueuedTexture::spTexture, Ptr::NULL);
        e.call(0x0043_c550, &args![this]);
        assert!(calls_to(&e, TEXTURE_PALETTE_SET_TEXTURE).is_empty());
        // Bit 4 clear: the queued map is left alone, but CheckFinished runs.
        assert!(calls_to(&e, FILE_MAP_REMOVE_QUEUED_TEXTURE).is_empty());
        assert_eq!(calls_to(&e, CHECK_FINISHED).len(), 2);
    }

    #[test]
    fn check_finished_slot_is_called() {
        let mut e = queue_engine();
        let this = queued_texture(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0043_c610, &args![this]);
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![this.addr()]]);
    }

    #[test]
    fn texture_flag_bit_4_test() {
        let mut e = queue_engine();
        let this = queued_texture(&mut e);
        for (flags, expected) in [(0, false), (2, false), (4, true), (7, true)] {
            e.set(this, QueuedTexture::cFlags, flags);
            assert_eq!(e.call(0x0043_c630, &args![this]).bool(), expected);
        }
    }

    #[test]
    fn cancel_runs_the_base_and_leaves_the_queued_map() {
        let mut e = queue_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        let this = queued_texture(&mut e);
        let entry = file_entry(&mut e, 0, 0);
        e.set(this, QueuedTexture::pFileEntry, entry);
        e.set(this, QueuedTexture::cFlags, 4);
        e.call_log = Some(vec![]);
        e.call(0x0043_c650, &args![this, 0x11u32, 0x22u32]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CANCEL),
            vec![vec![this.addr(), 0x11, 0x22]]
        );
        assert_eq!(
            calls_to(&e, FILE_MAP_REMOVE_QUEUED_TEXTURE),
            vec![vec![OWNER_OBJECT, entry.addr()]]
        );
        assert_eq!(e.get(this, QueuedTexture::cFlags), 0);
        // Unlike `Finish`, nothing asks whether the task is finished.
        assert!(calls_to(&e, CHECK_FINISHED).is_empty());
        // Not in the map: left alone.
        e.call_log = Some(vec![]);
        e.call(0x0043_c650, &args![this, 0u32, 0u32]);
        assert!(calls_to(&e, FILE_MAP_REMOVE_QUEUED_TEXTURE).is_empty());
    }

    #[test]
    fn texture_description_passes_the_word_texture() {
        let mut e = queue_engine();
        let this = queued_texture(&mut e);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_c6b0, &args![this, 0x9000u32, 0x400u32]);
        assert!(back.bool());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_DESCRIPTION),
            vec![vec![this.addr(), 0x9000, 0x400, TEXTURE_WORD]]
        );
    }

    #[test]
    fn model_constructor_sets_up_the_fields_and_flags() {
        let mut e = queue_engine();
        let this: Ptr<QueuedModel> = e.new_object();
        e.set(this, QueuedModel::pTESModel, Ptr::new(0x1111));
        e.set(this, QueuedModel::mfOverriddenVisualDistance, 5.0);
        e.set(this, QueuedModel::cFlags, 0xff);
        let name = text(&mut e, "meshes\\a.nif");
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_c6e0, &args![this, name, 0x77u32, 0x3u32, 1u8, 0u8]);
        assert_eq!(back.ptr::<QueuedModel>(), this);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_MODEL_VTABLE);
        assert_eq!(e.get(this, QueuedModel::spModel), Ptr::NULL);
        assert_eq!(e.get(this, QueuedModel::pTESModel), Ptr::NULL);
        assert_eq!(e.get(this, QueuedModel::eLODFadeMult), 3);
        // Flag argument 1 sets bit 1, argument 2 (zero) clears bit 2.
        assert_eq!(e.get(this, QueuedModel::cFlags), 1);
        assert_eq!(e.get(this, QueuedModel::mfOverriddenVisualDistance), 0.0);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x77]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME),
            vec![vec![this.addr(), name]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_FIND_FILE_ENTRY),
            vec![vec![this.addr(), 0]]
        );
        // The other combination.
        e.call(0x0043_c6e0, &args![this, name, 0x77u32, 0u32, 0u8, 1u8]);
        assert_eq!(e.get(this, QueuedModel::cFlags), 2);
    }

    #[test]
    fn model_flag_bits_1_and_2_are_set_and_cleared() {
        let mut e = queue_engine();
        let this: Ptr<QueuedModel> = e.new_object();
        e.set(this, QueuedModel::cFlags, 0x04);
        e.call(0x0043_c7a0, &args![this, 1u8]);
        assert_eq!(e.get(this, QueuedModel::cFlags), 0x05);
        e.call(0x0043_c7d0, &args![this, 1u8]);
        assert_eq!(e.get(this, QueuedModel::cFlags), 0x07);
        e.call(0x0043_c7a0, &args![this, 0u8]);
        e.call(0x0043_c7d0, &args![this, 0u8]);
        assert_eq!(e.get(this, QueuedModel::cFlags), 0x04);
    }

    #[test]
    fn model_scalar_deleting_destructor_frees_only_on_request() {
        let mut e = queue_engine();
        let this: Ptr<QueuedModel> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_c800, &args![this, 0u32]);
        assert_eq!(back.ptr::<QueuedModel>(), this);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_c800,
                MODEL_POINTER_RELEASE,
                QUEUED_FILE_ENTRY_DESTRUCT
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0043_c800, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn model_destructor_releases_the_model_pointer_and_keeps_the_table() {
        let mut e = queue_engine();
        let this: Ptr<QueuedModel> = e.new_object();
        e.mem.set_u32(this.addr(), 0x2222);
        e.call_log = Some(vec![]);
        e.call(0x0043_c830, &args![this]);
        assert_eq!(
            calls_to(&e, MODEL_POINTER_RELEASE),
            vec![vec![this.addr() + 0x30]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_DESTRUCT),
            vec![vec![this.addr()]]
        );
        // The virtual table is left as it was.
        assert_eq!(e.mem.u32(this.addr()), 0x2222);
    }

    // --- `QueuedModel` (third part) and the tree classes. ---

    /// A queued model's table: slot 0x28 is `CheckFinished`, slot 0x30 the
    /// `QueueMe_ov2` hook that records its argument.
    const MODEL_TASK_VTABLE: u32 = 0x0ff2_5000;
    const QUEUE_ME_HOOK: u32 = 0x0ff0_0110;
    /// A `TESModel`'s table: slot 0x14 is the model name, slot 0x1c the list.
    const TES_MODEL_VTABLE: u32 = 0x0ff2_6000;
    const TES_NAME_SLOT: u32 = 0x0ff0_0111;
    const TES_LIST_SLOT: u32 = 0x0ff0_0112;
    /// A 3D object's table: slot 0 deletes, slot 0x10 casts to a fade node.
    const OBJECT_VTABLE: u32 = 0x0ff2_7000;
    const OBJECT_DELETE: u32 = 0x0ff0_0113;
    const FADE_CAST: u32 = 0x0ff0_0114;
    /// A stream's object reference map table: slot 0 deletes.
    const REF_MAP_VTABLE: u32 = 0x0ff2_8000;
    const REF_MAP_DELETE: u32 = 0x0ff0_0115;
    /// A stream's table: slot 0x2c gives the link id.
    const LINK_STREAM_VTABLE: u32 = 0x0ff2_9000;
    const LINK_ID_SLOT: u32 = 0x0ff0_0116;
    /// A fixed answer of `QueuedFileEntry::GetFile`, the file name and the
    /// name of the tests' models.
    const OPEN_FILE: u32 = 0x0ff4_0000;
    /// Scratch dwords on the mapped page at `011f9000`.
    const SCRATCH: u32 = 0x011f_9100;

    /// An engine with doubles for what the `QueuedModel` and tree functions
    /// call.
    fn model_engine() -> Engine {
        let mut e = queue_engine();
        for page in [0x0101_1000, 0x0101_2000, 0x011c_6000, 0x011f_9000] {
            e.map(page, 0x1000);
        }
        e.register(TEST_FLAG_BIT_1, |_, a| (a[0] & 1 != 0).into_ret());
        e.register(TEST_FLAG_BIT_8, |_, a| (a[0] & 8 != 0).into_ret());
        e.register(TEST_FLAG_BIT_10, |_, a| (a[0] & 0x10 != 0).into_ret());
        e.register(TEST_FLAG_BIT_20, |_, a| (a[0] & 0x20 != 0).into_ret());
        e.register(SET_FLAG_BIT_8, |e, a| {
            write_flag(e, a[1], 8, a[0] & 0xff != 0);
            Ret::default()
        });
        e.register(STATE_IS_ZERO, |e, a| {
            (e.mem.u32(a[0] + 0xc) == 0).into_ret()
        });
        e.register(FIELD_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(FIELD_AT_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(BYTE_AT_0, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(FINISHED_COUNT_GETTER, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(LIST_FIRST_NODE, |_, a| (a[0] + 0x18).into_ret());
        e.register(LIST_NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(TES_MODEL_TEXTURE_BLOCK, |_, a| (a[0] + 0xc).into_ret());
        e.register(MODEL_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(TASK_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(ZERO_THREE_WORDS, |e, a| {
            for word in 0..3 {
                e.mem.set_u32(a[0] + word * 4, 0);
            }
            Ret::default()
        });
        e.register(MODEL_LOADER_FIND_MODEL, |_, _| 0u32.into_ret());
        e.register(MODEL_LOADER_ADD_MODEL, |_, _| 1u32.into_ret());
        e.register(BS_STREAM_LOAD, |_, _| 1u32.into_ret());
        e.register(QUEUED_FILE_ENTRY_GET_FILE, |_, _| OPEN_FILE.into_ret());
        // What `Model::Model` (`InitModel`) calls once it has its objects.
        e.register(REMOVE_EDITOR_MARKERS, |_, _| Ret::default());
        e.register(GET_PROPERTY, |_, _| 0u32.into_ret());
        e.register(PROPERTY_TYPE_SECOND_GETTER, |_, _| SECOND_TYPE.into_ret());
        e.register(MATCHES_PATTERN, |_, _| 0u32.into_ret());
        e.register(STREAM_FREE_ALL_OBJECTS, |_, _| Ret::default());
        e.register(HAS_MORPHER_CONTROLLER, |_, _| 0u32.into_ret());
        e.register(PREPARE_OBJECT, |_, _| Ret::default());
        e.register(SETTING_VALUE_POINTER, |_, a| (a[0] + 4).into_ret());
        e.register(BOUND_RADIUS, |e, a| e.mem.f32(a[0] + 0xc).into_ret());
        e.register(OVERRIDDEN_VISUAL_DISTANCE, |e, a| {
            e.mem.f32(a[0] + 0x40).into_ret()
        });
        for address in [
            QUEUED_FILE_CHECK_FINISHED,
            QUEUED_FILE_ENTRY_GENERATE_KEY,
            MODEL_LOADER_QUEUE_TEXTURE,
            QUEUE_TEXTURE_SET,
            NI_STREAM_CONSTRUCT,
            NI_STREAM_DESTRUCT,
            NI_STREAM_SAVE_BINARY_U32,
            ADD_REFERENCE,
            NI_AV_OBJECT_UPDATE,
            FADE_NODE_SET_RANGE,
            FADE_NODE_SET_LOD_MULT_TYPE,
            CREATE_BILLBOARD_INSTANCES,
            TREE_BILLBOARD_DATA_DESTRUCT,
            TASK_POINTER_DESTRUCT,
            QUEUE_TABLE_ADD,
            QUEUE_ME_HOOK,
            OBJECT_DELETE,
            REF_MAP_DELETE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(FADE_CAST, |_, _| 0u32.into_ret());
        e.register(LINK_ID_SLOT, |_, a| (a[1] + 0x100).into_ret());
        put_slots(
            &mut e,
            MODEL_TASK_VTABLE,
            &[(0x28, CHECK_FINISHED), (0x30, QUEUE_ME_HOOK)],
        );
        put_slots(
            &mut e,
            TES_MODEL_VTABLE,
            &[(0x14, TES_NAME_SLOT), (0x1c, TES_LIST_SLOT)],
        );
        put_slots(
            &mut e,
            OBJECT_VTABLE,
            &[(0, OBJECT_DELETE), (0x10, FADE_CAST)],
        );
        put_slots(&mut e, REF_MAP_VTABLE, &[(0, REF_MAP_DELETE)]);
        // A `BSStream` whose own loader (slot 0x60) fails.
        put_slots(&mut e, BS_STREAM_VTABLE, &[]);
        // After the tables: the table of the stream shares the page.
        e.mem.set_f64(HALF, 0.5);
        e.mem.set_f64(ZERO, 0.0);
        e.mem.set_f64(LOADED_AREA_MARGIN, 2048.0);
        e.mem.set_f64(SMALL_RADIUS, 0.001f32 as f64);
        put_slots(&mut e, LINK_STREAM_VTABLE, &[(0x2c, LINK_ID_SLOT)]);
        e
    }

    /// A queued model whose first word is the model task table.
    fn queued_model(e: &mut Engine) -> Ptr<QueuedModel> {
        let model: Ptr<QueuedModel> = e.new_object();
        e.mem.set_u32(model.addr(), MODEL_TASK_VTABLE);
        model
    }

    /// A `TESModel` object whose name slot is a double returning `name` and
    /// whose list slot returns `list`.
    fn tes_model(e: &mut Engine) -> Ptr {
        let tes_model = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(tes_model.addr(), TES_MODEL_VTABLE);
        tes_model
    }

    #[test]
    fn model_constructor_with_a_tes_model_sets_up_the_fields_and_flags() {
        let mut e = model_engine();
        e.register(TES_NAME_SLOT, |e, _| e.mem.u32(SCRATCH).into_ret());
        let name = text(&mut e, "meshes\\tree.nif");
        e.mem.set_u32(SCRATCH, name);
        let tes = tes_model(&mut e);
        let this: Ptr<QueuedModel> = e.new_object();
        e.set(this, QueuedModel::cFlags, 0xff);
        e.set(this, QueuedModel::mfOverriddenVisualDistance, 5.0);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_c890, &args![this, tes, 0x66u32, 0x3u32, 0u8, 1u8]);
        assert_eq!(back.ptr::<QueuedModel>(), this);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_MODEL_VTABLE);
        assert_eq!(e.get(this, QueuedModel::spModel), Ptr::NULL);
        assert_eq!(e.get(this, QueuedModel::pTESModel), tes);
        assert_eq!(e.get(this, QueuedModel::eLODFadeMult), 3);
        // The first flag argument (0) clears bit 1, the second sets bit 2.
        assert_eq!(e.get(this, QueuedModel::cFlags), 2);
        assert_eq!(e.get(this, QueuedModel::mfOverriddenVisualDistance), 0.0);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x66]]
        );
        // The file name is what the TESModel's slot 0x14 answered.
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME),
            vec![vec![this.addr(), name]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_FIND_FILE_ENTRY),
            vec![vec![this.addr(), 0]]
        );
    }

    #[test]
    fn model_constructor_with_a_loaded_model_stores_it_and_finishes_the_task() {
        let mut e = model_engine();
        let this: Ptr<QueuedModel> = e.new_object();
        e.set(this, QueuedModel::pTESModel, Ptr::new(0x1111));
        e.set(this, QueuedModel::cFlags, 0xff);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_c960, &args![this, Ptr::<()>::new(0x7777), 0x55u32]);
        assert_eq!(back.ptr::<QueuedModel>(), this);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_MODEL_VTABLE);
        assert_eq!(e.get(this, QueuedModel::spModel), Ptr::new(0x7777));
        assert_eq!(e.get(this, QueuedModel::pTESModel), Ptr::NULL);
        assert_eq!(e.get(this, QueuedModel::eLODFadeMult), 0);
        assert_eq!(e.get(this, QueuedModel::cFlags), 0);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x55]]
        );
        assert_eq!(calls_to(&e, TASK_SET_DONE), vec![vec![this.addr()]]);
        // No file name is set.
        assert!(calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME).is_empty());
    }

    #[test]
    fn model_generate_key_is_the_file_entry_one() {
        let mut e = model_engine();
        let this = queued_model(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0043_ca10, &args![this]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GENERATE_KEY),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn model_check_finished_follows_bit_4_children_and_state() {
        let mut e = model_engine();
        let queue = task_queue(&mut e);
        let children = Ptr::<()>::new(e.mem.alloc(0x20));
        let this = queued_model(&mut e);
        e.set(this, QueuedModel::pChildren, children);
        // Bit 4 clear: the base `CheckFinished`.
        e.call_log = Some(vec![]);
        e.call(0x0043_ca30, &args![this]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CHECK_FINISHED),
            vec![vec![this.addr()]]
        );
        assert!(calls_to(&e, QUEUE_ADD).is_empty());
        // Bit 4 set, 1 of 2 children finished: nothing.
        e.set(this, QueuedModel::cFlags, 4);
        e.mem.set_u32(children.addr() + 8, 2);
        e.mem.set_u32(children.addr() + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_ca30, &args![this]);
        assert!(calls_to(&e, QUEUED_FILE_CHECK_FINISHED).is_empty());
        assert!(calls_to(&e, QUEUE_ADD).is_empty());
        // All finished and the task in state 0: given to the task queue.
        e.mem.set_u32(children.addr() + 0x10, 2);
        e.call_log = Some(vec![]);
        e.call(0x0043_ca30, &args![this]);
        assert_eq!(calls_to(&e, QUEUE_ADD), vec![vec![queue, this.addr()]]);
        assert!(calls_to(&e, QUEUED_FILE_CHECK_FINISHED).is_empty());
        // All finished, state not 0: the base `CheckFinished`.
        e.mem.set_u32(this.addr() + 0xc, 3);
        e.call_log = Some(vec![]);
        e.call(0x0043_ca30, &args![this]);
        assert!(calls_to(&e, QUEUE_ADD).is_empty());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CHECK_FINISHED),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn model_children_finished_compares_the_counts() {
        let mut e = model_engine();
        let this = queued_model(&mut e);
        // No children object: finished.
        assert!(e.call(0x0043_caa0, &args![this]).bool());
        let children = Ptr::<()>::new(e.mem.alloc(0x20));
        e.set(this, QueuedModel::pChildren, children);
        e.mem.set_u32(children.addr() + 8, 3);
        e.mem.set_u32(children.addr() + 0x10, 3);
        assert!(e.call(0x0043_caa0, &args![this]).bool());
        e.mem.set_u32(children.addr() + 0x10, 2);
        assert!(!e.call(0x0043_caa0, &args![this]).bool());
    }

    #[test]
    fn model_bit_4_test_reads_the_flags() {
        let mut e = model_engine();
        let this = queued_model(&mut e);
        assert!(!e.call(0x0043_caf0, &args![this]).bool());
        e.set(this, QueuedModel::cFlags, 4);
        assert!(e.call(0x0043_caf0, &args![this]).bool());
    }

    #[test]
    fn model_queue_me_passes_the_tes_model_block_or_zero() {
        let mut e = model_engine();
        let this = queued_model(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0043_cb10, &args![this]);
        assert_eq!(calls_to(&e, QUEUE_ME_HOOK), vec![vec![this.addr(), 0]]);
        assert!(calls_to(&e, TES_MODEL_TEXTURE_BLOCK).is_empty());
        let tes = tes_model(&mut e);
        e.set(this, QueuedModel::pTESModel, tes);
        e.call_log = Some(vec![]);
        e.call(0x0043_cb10, &args![this]);
        assert_eq!(
            calls_to(&e, QUEUE_ME_HOOK),
            vec![vec![this.addr(), tes.addr() + 0xc]]
        );
    }

    #[test]
    fn model_queue_me_ov2_queues_the_textures_and_texture_sets() {
        let mut e = model_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        let this = queued_model(&mut e);
        // The key of the task: priority byte 7 in bits 16 to 23.
        e.mem.set_u64(this.addr() + 0x10, 0x0007_0000);
        // A block of three texture entries, the middle one empty.
        let textures = e.mem.alloc(12);
        e.mem.set_u32(textures, 0xaa);
        e.mem.set_u32(textures + 8, 0xcc);
        let block = e.mem.alloc(8);
        e.mem.set_u8(block, 3);
        e.mem.set_u32(block + 4, textures);
        // A TESModel with a list of three nodes (the first is embedded in
        // the list at +0x18; a node is an item pointer and a next link): a
        // texture set, a set holder whose set is null, no holder at all.
        let tes = tes_model(&mut e);
        e.set(this, QueuedModel::pTESModel, tes);
        let list = e.mem.alloc(0x20);
        e.register(TES_LIST_SLOT, |e, _| e.mem.u32(SCRATCH).into_ret());
        e.mem.set_u32(SCRATCH, list);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, 0x5e7);
        let empty_holder = e.mem.alloc(8);
        e.mem.set_u32(empty_holder, 0);
        let third = e.mem.alloc(8);
        e.mem.set_u32(third, 0);
        e.mem.set_u32(third + 4, 0);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, empty_holder);
        e.mem.set_u32(second + 4, third);
        let first = list + 0x18;
        e.mem.set_u32(first, holder);
        e.mem.set_u32(first + 4, second);
        e.call_log = Some(vec![]);
        e.call(0x0043_cb50, &args![this, block]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_TEXTURE),
            vec![
                vec![OWNER_OBJECT, 0xaa, 7, this.addr()],
                vec![OWNER_OBJECT, 0xcc, 7, this.addr()]
            ]
        );
        assert_eq!(
            calls_to(&e, QUEUE_TEXTURE_SET),
            vec![vec![0x5e7, 7, this.addr(), 0]]
        );
        // Bit 4 is set, then `CheckFinished` is called.
        assert_eq!(e.get(this, QueuedModel::cFlags), 4);
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![this.addr()]]);
    }

    #[test]
    fn model_queue_me_ov2_without_block_or_tes_model_only_flags_and_checks() {
        let mut e = model_engine();
        let this = queued_model(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0043_cb50, &args![this, 0u32]);
        assert!(calls_to(&e, MODEL_LOADER_QUEUE_TEXTURE).is_empty());
        assert!(calls_to(&e, QUEUE_TEXTURE_SET).is_empty());
        assert_eq!(e.get(this, QueuedModel::cFlags), 4);
        assert_eq!(calls_to(&e, CHECK_FINISHED).len(), 1);
        // A block with a zero count is skipped as well.
        let block = e.mem.alloc(8);
        e.mem.set_u8(block, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_cb50, &args![this, block]);
        assert!(calls_to(&e, MODEL_LOADER_QUEUE_TEXTURE).is_empty());
    }

    #[test]
    fn task_key_is_shifted_arithmetically_by_16() {
        let mut e = model_engine();
        let task = queued_model(&mut e);
        e.mem.set_u64(task.addr() + 0x10, 0x8000_0000_0012_0000);
        let shifted = e.call(0x0043_cc80, &args![task]).u64();
        assert_eq!(shifted, 0xffff_8000_0000_0012);
        assert_eq!(e.call(0x0043_cc60, &args![task]).u32(), 0x12);
        assert_eq!(
            e.call(0x0043_cca0, &args![Ptr::<()>::new(task.addr() + 0x10)])
                .u64(),
            0xffff_8000_0000_0012
        );
    }

    #[test]
    fn model_bit_4_setter_sets_and_clears() {
        let mut e = model_engine();
        let this = queued_model(&mut e);
        e.set(this, QueuedModel::cFlags, 1);
        e.call(0x0043_ccc0, &args![this, 1u8]);
        assert_eq!(e.get(this, QueuedModel::cFlags), 5);
        e.call(0x0043_ccc0, &args![this, 0u8]);
        assert_eq!(e.get(this, QueuedModel::cFlags), 1);
    }

    /// A queued model ready for `Run`: a file name and the context 0x21.
    fn runnable_model(e: &mut Engine, name: &str) -> Ptr<QueuedModel> {
        let this = queued_model(e);
        let name = text(e, name);
        e.mem.set_u32(this.addr() + 0x28, name);
        e.set(this, QueuedModel::eContext, 0x21);
        this
    }

    #[test]
    fn model_run_without_a_file_name_does_nothing() {
        let mut e = model_engine();
        let this = runnable_model(&mut e, "");
        e.call_log = Some(vec![]);
        e.call(0x0043_ccf0, &args![this]);
        assert!(calls_to(&e, MODEL_LOADER_FIND_MODEL).is_empty());
        assert!(calls_to(&e, QUEUED_FILE_ENTRY_GET_FILE).is_empty());
        // The memory context was entered and left.
        let enter = calls_to(&e, MEMORY_CONTEXT_ENTER);
        assert_eq!(enter.len(), 1);
        assert_eq!(&enter[0][1..], &[0x21, 1, MODEL_LOADER_SOURCE, 0x3b8]);
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    #[test]
    fn model_run_takes_a_model_the_loader_already_has() {
        let mut e = model_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        e.register(MODEL_LOADER_FIND_MODEL, |e, a| {
            e.mem.set_u32(a[2], 0x5555);
            1u32.into_ret()
        });
        let this = runnable_model(&mut e, "meshes\\a.nif");
        let name = e.mem.u32(this.addr() + 0x28);
        e.call_log = Some(vec![]);
        e.call(0x0043_ccf0, &args![this]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_FIND_MODEL),
            vec![vec![OWNER_OBJECT, name, this.addr() + 0x30]]
        );
        assert_eq!(e.get(this, QueuedModel::spModel), Ptr::new(0x5555));
        // Bit 8 is set; the file is never opened.
        assert_eq!(e.get(this, QueuedModel::cFlags), 8);
        assert!(calls_to(&e, QUEUED_FILE_ENTRY_GET_FILE).is_empty());
        assert!(calls_to(&e, 0x0043_aaf0).is_empty());
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    /// The loader of the stream (`BSStream::Load`) gives the stream a root
    /// node named `Scene Root`; the returned address is that node. The
    /// loader also records the name published in `011f94c0` at `SCRATCH`.
    fn stream_loads_a_root(e: &mut Engine) -> Ptr {
        let root = object(e, NODE_VTABLE);
        let root_name = text(e, "Scene Root");
        e.mem.set_u32(root.addr() + 8, root_name);
        e.register_double(BS_STREAM_LOAD, move |e, a| {
            let published = e.global::<u32>(LOADING_FILE_NAME);
            e.mem.set_u32(SCRATCH, published);
            set_stream_objects(e, Ptr::new(a[0]), &[root]);
            1u32.into_ret()
        });
        root
    }

    #[test]
    fn model_run_loads_the_file_into_a_stream_and_builds_the_model() {
        let mut e = model_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        let root = stream_loads_a_root(&mut e);
        let this = runnable_model(&mut e, "meshes\\a.nif");
        let name = e.mem.u32(this.addr() + 0x28);
        // Bit 1 (prepare) and bit 0x20 (force).
        e.set(this, QueuedModel::cFlags, 0x21);
        e.call_log = Some(vec![]);
        e.call(0x0043_ccf0, &args![this]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_FILE),
            vec![vec![this.addr(), 0, 1]]
        );
        let loads = calls_to(&e, BS_STREAM_LOAD);
        assert_eq!(loads.len(), 1);
        let stream = loads[0][0];
        assert_eq!(&loads[0][1..], &[name, OPEN_FILE]);
        // The load saw the name published in `011f94c0`.
        assert_eq!(e.mem.u32(SCRATCH), name);
        assert_eq!(calls_to(&e, NI_STREAM_CONSTRUCT), vec![vec![stream]]);
        // The model (a 0x10-byte block) is built from the stream and stored:
        // its 3D object is the stream's root, prepared with `force` and
        // `prepare` taken from bits 0x20 and 1.
        assert_eq!(calls_to(&e, MEMORY_ALLOC)[0], vec![0x10]);
        let model = e.get(this, QueuedModel::spModel);
        assert!(!model.is_null());
        assert_eq!(e.mem.u32(model.addr() + 0xc), root.addr());
        assert_eq!(calls_to(&e, PREPARE_OBJECT), vec![vec![root.addr(), 1, 1]]);
        // The name is withdrawn and the stream destroyed.
        assert_eq!(e.global::<u32>(LOADING_FILE_NAME), 0);
        assert_eq!(calls_to(&e, NI_STREAM_DESTRUCT), vec![vec![stream]]);
        assert!(calls_to(&e, LOG).is_empty());
        // Only bit 1: prepare without force.
        e.set(this, QueuedModel::spModel, Ptr::NULL);
        e.set(this, QueuedModel::cFlags, 0x01);
        e.call_log = Some(vec![]);
        e.call(0x0043_ccf0, &args![this]);
        assert_eq!(calls_to(&e, PREPARE_OBJECT), vec![vec![root.addr(), 0, 0]]);
        // Neither bit: not prepared at all.
        e.set(this, QueuedModel::spModel, Ptr::NULL);
        e.set(this, QueuedModel::cFlags, 0x00);
        e.call_log = Some(vec![]);
        e.call(0x0043_ccf0, &args![this]);
        assert!(calls_to(&e, PREPARE_OBJECT).is_empty());
    }

    #[test]
    fn model_run_logs_a_failed_load_unless_bit_10_allows_a_model() {
        let mut e = model_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        e.register(BS_STREAM_LOAD, |_, _| 0u32.into_ret());
        let this = runnable_model(&mut e, "meshes\\a.nif");
        let name = e.mem.u32(this.addr() + 0x28);
        e.call_log = Some(vec![]);
        e.call(0x0043_ccf0, &args![this]);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![MODEL_ERROR_MESSAGE, LOAD_WORD, name]]
        );
        // No model block is allocated and the pointer stays empty.
        assert!(calls_to(&e, MEMORY_ALLOC).is_empty());
        assert_eq!(e.get(this, QueuedModel::spModel), Ptr::NULL);
        assert_eq!(e.global::<u32>(LOADING_FILE_NAME), 0);
        assert_eq!(calls_to(&e, NI_STREAM_DESTRUCT).len(), 1);
        // Bit 0x10: a model is built anyway (from the empty stream).
        e.set(this, QueuedModel::cFlags, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0043_ccf0, &args![this]);
        assert!(calls_to(&e, LOG).is_empty());
        assert_eq!(calls_to(&e, MEMORY_ALLOC)[0], vec![0x10]);
        assert_ne!(e.get(this, QueuedModel::spModel), Ptr::NULL);
    }

    #[test]
    fn model_run_without_a_file_logs_find_or_builds_with_bit_10() {
        let mut e = model_engine();
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        e.register(QUEUED_FILE_ENTRY_GET_FILE, |_, _| 0u32.into_ret());
        let this = runnable_model(&mut e, "meshes\\a.nif");
        let name = e.mem.u32(this.addr() + 0x28);
        e.call_log = Some(vec![]);
        e.call(0x0043_ccf0, &args![this]);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![MODEL_ERROR_MESSAGE, FIND_WORD, name]]
        );
        // The stream is never read.
        assert!(calls_to(&e, BS_STREAM_LOAD).is_empty());
        assert_eq!(e.get(this, QueuedModel::spModel), Ptr::NULL);
        e.set(this, QueuedModel::cFlags, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0043_ccf0, &args![this]);
        assert!(calls_to(&e, LOG).is_empty());
        assert_ne!(e.get(this, QueuedModel::spModel), Ptr::NULL);
    }
    #[test]
    fn model_flag_tests_read_their_bits() {
        let mut e = model_engine();
        let this = queued_model(&mut e);
        for (address, bit) in [
            (0x0043_cf40u32, 1u8),
            (0x0043_cf90, 0x10),
            (0x0043_cfb0, 0x20),
            (0x0043_d490, 2),
            (0x0043_d4b0, 8),
        ] {
            e.set(this, QueuedModel::cFlags, !bit);
            assert!(!e.call(address, &args![this]).bool(), "{address:08x}");
            e.set(this, QueuedModel::cFlags, bit);
            assert!(e.call(address, &args![this]).bool(), "{address:08x}");
        }
    }

    #[test]
    fn model_bit_8_setter_sets_and_clears() {
        let mut e = model_engine();
        let this = queued_model(&mut e);
        e.call(0x0043_cf60, &args![this, 1u8]);
        assert_eq!(e.get(this, QueuedModel::cFlags), 8);
        e.call(0x0043_cf60, &args![this, 0u8]);
        assert_eq!(e.get(this, QueuedModel::cFlags), 0);
    }

    #[test]
    fn stream_constructor_builds_the_base_and_clears_the_pointers() {
        let mut e = model_engine();
        let stream: Ptr<BSStream> = e.new_object();
        e.set(stream, BSStream::pObjectRefMap, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_cfd0, &args![stream]);
        assert_eq!(back.ptr::<BSStream>(), stream);
        assert_eq!(calls_to(&e, NI_STREAM_CONSTRUCT), vec![vec![stream.addr()]]);
        assert_eq!(e.mem.u32(stream.addr()), BS_STREAM_VTABLE);
        assert_eq!(
            calls_to(&e, NI_POINTER_CONSTRUCT),
            vec![vec![stream.addr() + 0x5c8, 0]]
        );
        assert_eq!(e.get(stream, BSStream::pObjectRefMap), Ptr::NULL);
        // The base constructor comes first, the table is set after it.
        assert_eq!(call_order(&e)[1], NI_STREAM_CONSTRUCT);
    }

    #[test]
    fn save_link_id_writes_the_id_the_stream_gives() {
        let mut e = model_engine();
        let stream = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(stream.addr(), LINK_STREAM_VTABLE);
        e.register(NI_STREAM_SAVE_BINARY_U32, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(SCRATCH, value);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0043_d050, &args![stream, Ptr::<()>::new(0x40)]);
        // The slot got the object and answered object + 0x100.
        assert_eq!(calls_to(&e, LINK_ID_SLOT), vec![vec![stream.addr(), 0x40]]);
        assert_eq!(e.mem.u32(SCRATCH), 0x140);
        let saves = calls_to(&e, NI_STREAM_SAVE_BINARY_U32);
        assert_eq!(saves.len(), 1);
        assert_eq!(saves[0][0], stream.addr());
    }

    #[test]
    fn stream_destructor_deletes_the_map_and_destroys_the_pointer_and_base() {
        let mut e = model_engine();
        let stream: Ptr<BSStream> = e.new_object();
        let map = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(map.addr(), REF_MAP_VTABLE);
        e.set(stream, BSStream::pObjectRefMap, map);
        e.call_log = Some(vec![]);
        e.call(0x0043_d090, &args![stream]);
        assert_eq!(e.mem.u32(stream.addr()), BS_STREAM_VTABLE);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_d090,
                REF_MAP_DELETE,
                NI_POINTER_DESTRUCT,
                NI_STREAM_DESTRUCT
            ]
        );
        assert_eq!(calls_to(&e, REF_MAP_DELETE), vec![vec![map.addr(), 1]]);
        assert_eq!(
            calls_to(&e, NI_POINTER_DESTRUCT),
            vec![vec![stream.addr() + 0x5c8]]
        );
        assert_eq!(e.get(stream, BSStream::pObjectRefMap), Ptr::NULL);
    }

    #[test]
    fn stream_map_deleter_skips_a_missing_map() {
        let mut e = model_engine();
        let stream: Ptr<BSStream> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0043_d100, &args![stream]);
        assert!(calls_to(&e, REF_MAP_DELETE).is_empty());
        assert_eq!(e.get(stream, BSStream::pObjectRefMap), Ptr::NULL);
    }

    #[test]
    fn stream_scalar_deleting_destructor_frees_only_on_request() {
        let mut e = model_engine();
        let stream: Ptr<BSStream> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_d150, &args![stream, 0u32]);
        assert_eq!(back.ptr::<BSStream>(), stream);
        assert_eq!(calls_to(&e, NI_STREAM_DESTRUCT).len(), 1);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0043_d150, &args![stream, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![stream.addr()]]);
    }

    /// A model ready for `Finish`: the queued model holds a model whose 3D
    /// object (its table casts to `fade_node`, which has the world bound
    /// radius `radius`).
    struct FinishSetup {
        this: Ptr<QueuedModel>,
        model: u32,
        object: u32,
        fade_node: u32,
    }

    fn finishing_model(e: &mut Engine, radius: f32) -> FinishSetup {
        let this = queued_model(e);
        e.set(this, QueuedModel::eContext, 0x21);
        let name = text(e, "meshes\\a.nif");
        e.mem.set_u32(this.addr() + 0x28, name);
        e.set(this, QueuedModel::eLODFadeMult, 2);
        let model = e.mem.alloc(0x10);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(model + 0xc, object);
        e.set(this, QueuedModel::spModel, Ptr::new(model));
        // The fade node has a world bound with the radius at +0xC.
        let fade_node = e.mem.alloc(0x40);
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound + 0xc, radius);
        e.mem.set_u32(fade_node + 0x20, bound);
        e.register(FADE_CAST, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.mem.set_u32(object + 4, fade_node);
        // The settings: 64 cells, visual distance scale 10, fade scale 0.5.
        e.mem.set_u32(CELL_COUNT_SETTING + 4, 64);
        e.mem.set_f32(VISUAL_DISTANCE_SCALE_SETTING + 4, 10.0);
        e.mem.set_f32(FADE_RANGE_SCALE_SETTING + 4, 0.5);
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        FinishSetup {
            this,
            model,
            object,
            fade_node,
        }
    }

    /// The `(near, far)` floats `SetRange` received.
    fn set_range_args(e: &Engine) -> Vec<(f32, f32)> {
        calls_to(e, FADE_NODE_SET_RANGE)
            .iter()
            .map(|a| (f32::from_bits(a[1]), f32::from_bits(a[2])))
            .collect()
    }

    #[test]
    fn model_finish_sets_the_fade_range_from_the_radius() {
        let mut e = model_engine();
        let setup = finishing_model(&mut e, 100.0);
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        // 64 cells: limit = 262144 * 0.5 - (100 + 2048) = 128924; the radius
        // times 10 is smaller, and the node gets (range * 0.5, range).
        assert_eq!(set_range_args(&e), vec![(500.0, 1000.0)]);
        assert_eq!(
            calls_to(&e, FADE_NODE_SET_LOD_MULT_TYPE),
            vec![vec![setup.fade_node, 2]]
        );
        // The node was updated with the zeroed update record first.
        assert_eq!(calls_to(&e, NI_AV_OBJECT_UPDATE).len(), 1);
        assert_eq!(calls_to(&e, NI_AV_OBJECT_UPDATE)[0][0], setup.fade_node);
        // The model is added to the loader's map; the add succeeded.
        let name = e.mem.u32(setup.this.addr() + 0x28);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_ADD_MODEL),
            vec![vec![OWNER_OBJECT, name, setup.model]]
        );
        assert!(calls_to(&e, MODEL_LOADER_FIND_MODEL).is_empty());
        // The memory context of `Finish`, then `CheckFinished`.
        let enter = calls_to(&e, MEMORY_CONTEXT_ENTER);
        assert_eq!(&enter[0][1..], &[0x21, 1, MODEL_LOADER_SOURCE, 0x3e5]);
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![setup.this.addr()]]);
        assert!(calls_to(&e, ADD_REFERENCE).is_empty());
    }

    #[test]
    fn model_finish_limits_the_range_to_the_world() {
        let mut e = model_engine();
        let setup = finishing_model(&mut e, 100.0);
        // One cell: limit = 4096 * 0.5 - (100 + 2048) = -100.
        e.mem.set_u32(CELL_COUNT_SETTING + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        assert_eq!(set_range_args(&e), vec![(-50.0, -100.0)]);
    }

    #[test]
    fn model_finish_takes_a_tiny_radius_from_the_maximum_bound() {
        let mut e = model_engine();
        let setup = finishing_model(&mut e, 0.0005);
        e.register(GET_BOUND_MAX, |e, a| {
            e.mem.set_f32(a[0] + 8, 20.0);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        // The radius becomes 20 * 0.5 = 10, the range 10 * 10.
        let bound_calls = calls_to(&e, GET_BOUND_MAX);
        assert_eq!(bound_calls.len(), 1);
        assert_eq!(bound_calls[0][1], setup.object);
        assert_eq!(set_range_args(&e), vec![(50.0, 100.0)]);
        // A maximum bound of zero leaves radius 0: the limit is used.
        e.register(GET_BOUND_MAX, |e, a| {
            e.mem.set_f32(a[0] + 8, 0.0);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        assert_eq!(
            set_range_args(&e),
            vec![(65536.0 - 1024.0, 131072.0 - 2048.0)]
        );
    }

    #[test]
    fn model_finish_uses_the_overridden_visual_distance_when_set() {
        let mut e = model_engine();
        let setup = finishing_model(&mut e, 100.0);
        e.set(setup.this, QueuedModel::mfOverriddenVisualDistance, 77.0);
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        assert_eq!(set_range_args(&e), vec![(38.5, 77.0)]);
    }

    #[test]
    fn model_finish_branches_on_flags_missing_objects_and_the_map() {
        let mut e = model_engine();
        let setup = finishing_model(&mut e, 100.0);
        // Bit 2 adds a reference to the model.
        e.set(setup.this, QueuedModel::cFlags, 2);
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![setup.model]]);
        // Bit 8 skips everything but the final check.
        e.set(setup.this, QueuedModel::cFlags, 8);
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        assert!(calls_to(&e, NI_AV_OBJECT_UPDATE).is_empty());
        assert!(calls_to(&e, MODEL_LOADER_ADD_MODEL).is_empty());
        assert_eq!(calls_to(&e, CHECK_FINISHED).len(), 1);
        // No fade node: no range, but the model still goes into the map.
        e.set(setup.this, QueuedModel::cFlags, 0);
        e.mem.set_u32(setup.object + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        assert!(calls_to(&e, FADE_NODE_SET_RANGE).is_empty());
        assert_eq!(calls_to(&e, MODEL_LOADER_ADD_MODEL).len(), 1);
        // No 3D object: nothing is added.
        e.mem.set_u32(setup.model + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        assert!(calls_to(&e, MODEL_LOADER_ADD_MODEL).is_empty());
        assert_eq!(calls_to(&e, CHECK_FINISHED).len(), 1);
        // No model at all: only the final check.
        e.set(setup.this, QueuedModel::spModel, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        assert!(calls_to(&e, MODEL_LOADER_ADD_MODEL).is_empty());
        assert_eq!(calls_to(&e, CHECK_FINISHED).len(), 1);
    }

    #[test]
    fn model_finish_replaces_the_model_when_the_map_has_one() {
        let mut e = model_engine();
        let setup = finishing_model(&mut e, 100.0);
        e.register(MODEL_LOADER_ADD_MODEL, |_, _| 0u32.into_ret());
        let name = e.mem.u32(setup.this.addr() + 0x28);
        e.call_log = Some(vec![]);
        e.call(0x0043_d180, &args![setup.this]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_FIND_MODEL),
            vec![vec![OWNER_OBJECT, name, setup.this.addr() + 0x30]]
        );
    }

    #[test]
    fn update_data_constructor_stores_time_and_two_flags() {
        let mut e = model_engine();
        let data: Ptr<NiUpdateData> = e.new_object();
        for offset in 0..12 {
            e.mem.set_u8(data.addr() + offset, 0xee);
        }
        let back = e.call(0x0043_d410, &args![data, 1.5f32, 1u8, 0u8]);
        assert_eq!(back.ptr::<NiUpdateData>(), data);
        assert_eq!(e.get(data, NiUpdateData::fTime), 1.5);
        assert_eq!(e.get(data, NiUpdateData::bUpdateControllers), 1);
        assert_eq!(e.get(data, NiUpdateData::bParallelUpdate), 0);
        assert_eq!(e.get(data, NiUpdateData::bFoundParticles), 0);
        assert_eq!(e.get(data, NiUpdateData::bFoundMorphController), 0);
        assert_eq!(e.get(data, NiUpdateData::bSceneGraphChange), 0);
        // The padding after the five flags is not touched.
        assert_eq!(e.mem.u8(data.addr() + 9), 0xee);
    }

    #[test]
    fn world_bound_falls_back_to_the_empty_bound() {
        let mut e = model_engine();
        let object = Ptr::<()>::new(e.mem.alloc(0x40));
        assert_eq!(e.call(0x0043_d450, &args![object]).u32(), EMPTY_WORLD_BOUND);
        e.mem.set_u32(object.addr() + 0x20, 0x4321);
        assert_eq!(e.call(0x0043_d450, &args![object]).u32(), 0x4321);
    }

    #[test]
    fn setting_value_address_is_plus_4_or_a_zeroed_scratch() {
        let mut e = model_engine();
        e.mem.set_u32(SCRATCH_ZERO, 0x9999);
        assert_eq!(
            e.call(0x0043_d4d0, &args![Ptr::<()>::new(0x1000)]).u32(),
            0x1004
        );
        assert_eq!(e.mem.u32(SCRATCH_ZERO), 0x9999);
        assert_eq!(
            e.call(0x0043_d4d0, &args![Ptr::<()>::NULL]).u32(),
            SCRATCH_ZERO
        );
        assert_eq!(e.mem.u32(SCRATCH_ZERO), 0);
    }

    #[test]
    fn model_description_passes_the_word_model() {
        let mut e = model_engine();
        let this = queued_model(&mut e);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_d510, &args![this, 0x9000u32, 0x400u32]);
        assert!(back.bool());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_DESCRIPTION),
            vec![vec![this.addr(), 0x9000, 0x400, MODEL_WORD]]
        );
    }

    #[test]
    fn tree_billboard_constructor_builds_a_texture_with_the_data() {
        let mut e = model_engine();
        let this: Ptr<QueuedTreeBillboard> = e.new_object();
        let name = text(&mut e, "textures\\tree.dds");
        e.call_log = Some(vec![]);
        let back = e.call(
            0x0043_d540,
            &args![this, name, 0x44u32, Ptr::<()>::new(0x6000)],
        );
        assert_eq!(back.ptr::<QueuedTreeBillboard>(), this);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x44]]
        );
        // The billboard's table replaces the texture's.
        assert_eq!(e.mem.u32(this.addr()), QUEUED_TREE_BILLBOARD_VTABLE);
        assert_eq!(
            e.get(this, QueuedTreeBillboard::pBillboardData),
            Ptr::new(0x6000)
        );
        assert_eq!(calls_to(&e, QUEUED_FILE_ENTRY_FIND_FILE_ENTRY).len(), 1);
    }

    #[test]
    fn tree_billboard_destructor_deletes_the_data_then_the_texture_part() {
        let mut e = model_engine();
        let this: Ptr<QueuedTreeBillboard> = e.new_object();
        let data: Ptr<TreeBillboardData> = e.new_object();
        e.set(this, QueuedTreeBillboard::pBillboardData, data.cast());
        e.call_log = Some(vec![]);
        e.call(0x0043_d5b0, &args![this]);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_d5b0,
                TREE_BILLBOARD_DATA_DESTRUCT,
                MEMORY_FREE,
                NI_POINTER_DESTRUCT,
                QUEUED_FILE_ENTRY_DESTRUCT
            ]
        );
        // The data is freed (flag 1).
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![data.addr()]]);
        // The texture part ends with the texture's table.
        assert_eq!(e.mem.u32(this.addr()), QUEUED_TEXTURE_VTABLE);
        // Without data only the base runs.
        e.set(this, QueuedTreeBillboard::pBillboardData, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0043_d5b0, &args![this]);
        assert!(calls_to(&e, TREE_BILLBOARD_DATA_DESTRUCT).is_empty());
        assert_eq!(calls_to(&e, QUEUED_FILE_ENTRY_DESTRUCT).len(), 1);
    }

    #[test]
    fn tree_billboard_scalar_deleting_destructor_frees_only_on_request() {
        let mut e = model_engine();
        let this: Ptr<QueuedTreeBillboard> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_d580, &args![this, 0u32]);
        assert_eq!(back.ptr::<QueuedTreeBillboard>(), this);
        assert_eq!(calls_to(&e, QUEUED_FILE_ENTRY_DESTRUCT).len(), 1);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0043_d580, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn billboard_data_deleter_runs_the_destructor_and_frees_on_request() {
        let mut e = model_engine();
        let data: Ptr<TreeBillboardData> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_d640, &args![data, 0u32]);
        assert_eq!(back.ptr::<TreeBillboardData>(), data);
        assert_eq!(
            calls_to(&e, TREE_BILLBOARD_DATA_DESTRUCT),
            vec![vec![data.addr()]]
        );
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0043_d640, &args![data, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![data.addr()]]);
    }

    /// The IO manager of the tests: the table of queues at +0x64 is a block
    /// of dwords.
    const IO_MANAGER: u32 = 0x0ff5_0000;

    #[test]
    fn tree_billboard_finish_finishes_the_texture_then_posts_the_task() {
        let mut e = model_engine();
        e.map(IO_MANAGER, 0x1000);
        e.mem.set_u32(IO_MANAGER + 0x64, 0x7100);
        e.set_global(TASK_QUEUE, IO_MANAGER);
        e.set_global(FILE_MAP_OWNER, OWNER_OBJECT);
        let this: Ptr<QueuedTreeBillboard> = e.new_object();
        e.mem.set_u32(this.addr(), TASK_VTABLE);
        e.mem.set_u64(this.addr() + 0x10, 0x0004_0000);
        e.call_log = Some(vec![]);
        e.call(0x0043_d670, &args![this]);
        // The texture's `Finish` asks whether the task is finished first.
        let order = call_order(&e);
        let finish = order.iter().position(|a| *a == CHECK_FINISHED).unwrap();
        let post = order
            .iter()
            .position(|a| *a == TASK_POINTER_CONSTRUCT)
            .unwrap();
        assert!(finish < post);
        // The task is queued under its priority byte.
        let holder = calls_to(&e, TASK_POINTER_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, TASK_POINTER_CONSTRUCT),
            vec![vec![holder, this.addr()]]
        );
        assert_eq!(calls_to(&e, QUEUE_TABLE_ADD), vec![vec![0x7100, 4, holder]]);
    }

    #[test]
    fn post_process_task_is_added_through_the_io_manager_global() {
        let mut e = model_engine();
        e.map(IO_MANAGER, 0x1000);
        e.mem.set_u32(IO_MANAGER + 0x64, 0x7100);
        e.set_global(TASK_QUEUE, IO_MANAGER);
        let task = queued_model(&mut e);
        e.mem.set_u64(task.addr() + 0x10, 0x0003_0000);
        e.call_log = Some(vec![]);
        e.call(0x0043_d690, &args![task]);
        let holder = calls_to(&e, TASK_POINTER_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, TASK_POINTER_CONSTRUCT),
            vec![vec![holder, task.addr()]]
        );
        // The queue table of the manager is used.
        assert_eq!(calls_to(&e, QUEUE_TABLE_ADD), vec![vec![0x7100, 3, holder]]);
    }

    #[test]
    fn post_process_body_queues_under_the_priority_byte() {
        let mut e = model_engine();
        e.map(IO_MANAGER, 0x1000);
        e.mem.set_u32(IO_MANAGER + 0x64, 0x7200);
        let task = queued_model(&mut e);
        // Priority 0x12 in bits 16 to 23; higher bits do not matter.
        e.mem.set_u64(task.addr() + 0x10, 0x0000_0001_0012_0000);
        e.call_log = Some(vec![]);
        e.call(0x0043_d6b0, &args![Ptr::<()>::new(IO_MANAGER), task]);
        let holder = calls_to(&e, TASK_POINTER_CONSTRUCT)[0][0];
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_d6b0,
                TASK_POINTER_CONSTRUCT,
                QUEUE_TABLE_ADD,
                TASK_POINTER_DESTRUCT
            ]
        );
        assert_eq!(
            calls_to(&e, QUEUE_TABLE_ADD),
            vec![vec![0x7200, 0x12, holder]]
        );
        assert_eq!(calls_to(&e, TASK_POINTER_DESTRUCT), vec![vec![holder]]);
    }

    #[test]
    fn tree_billboard_post_process_calls_slot_0x30() {
        let mut e = model_engine();
        let this: Ptr<QueuedTreeBillboard> = e.new_object();
        e.mem.set_u32(this.addr(), MODEL_TASK_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x0043_d730, &args![this]);
        assert_eq!(calls_to(&e, QUEUE_ME_HOOK), vec![vec![this.addr()]]);
    }

    #[test]
    fn tree_billboard_creates_the_distant_node_and_deletes_it_again() {
        let mut e = model_engine();
        let this: Ptr<QueuedTreeBillboard> = e.new_object();
        let data: Ptr<TreeBillboardData> = e.new_object();
        e.set(this, QueuedTreeBillboard::pBillboardData, data.cast());
        let tree = e.mem.alloc(0x20);
        e.mem.set_u32(tree + 0xc, 9);
        e.set(data, TreeBillboardData::pTree, Ptr::new(tree));
        e.set(data, TreeBillboardData::iCellChunk, 11);
        e.set(data, TreeBillboardData::iCellKey, 12);
        e.set(data, TreeBillboardData::pInstancedNode, Ptr::new(13));
        // The game reads the low word of the size.
        e.set(data, TreeBillboardData::iArraySize, 0x1_0005);
        e.set(data, TreeBillboardData::pLocArray, Ptr::new(15));
        e.set(data, TreeBillboardData::pColorArray, Ptr::new(16));
        e.register(BUILD_DISTANT_3D, |_, _| 0x0ff6_0000u32.into_ret());
        e.map(0x0ff6_0000, 0x1000);
        e.mem.set_u32(0x0ff6_0000, OBJECT_VTABLE);
        e.register(CREATE_BILLBOARD_INSTANCES, |e, a| {
            // The record: 0, the node, the count.
            let record: Vec<u32> = (0..3).map(|i| e.mem.u32(a[3] + i * 4)).collect();
            e.mem.set_u32(SCRATCH, record[0]);
            e.mem.set_u32(SCRATCH + 4, record[1]);
            e.mem.set_u32(SCRATCH + 8, record[2]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0043_d750, &args![this]);
        assert_eq!(calls_to(&e, BUILD_DISTANT_3D), vec![vec![tree, 1]]);
        let built = calls_to(&e, CREATE_BILLBOARD_INSTANCES);
        assert_eq!(built.len(), 1);
        assert_eq!(&built[0][..3], &[11, 12, 13]);
        assert_eq!(&built[0][4..], &[15, 16, 5]);
        assert_eq!(
            [
                e.mem.u32(SCRATCH),
                e.mem.u32(SCRATCH + 4),
                e.mem.u32(SCRATCH + 8)
            ],
            [0, 0x0ff6_0000, 9]
        );
        // The node is deleted through its virtual destructor with flag 1.
        assert_eq!(calls_to(&e, OBJECT_DELETE), vec![vec![0x0ff6_0000, 1]]);
        // Without a node nothing else happens.
        e.register(BUILD_DISTANT_3D, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0043_d750, &args![this]);
        assert!(calls_to(&e, CREATE_BILLBOARD_INSTANCES).is_empty());
        assert!(calls_to(&e, OBJECT_DELETE).is_empty());
    }

    #[test]
    fn tree_billboard_description_passes_the_words() {
        let mut e = model_engine();
        let this: Ptr<QueuedTreeBillboard> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_d820, &args![this, 0x9000u32, 0x400u32]);
        assert!(back.bool());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_DESCRIPTION),
            vec![vec![this.addr(), 0x9000, 0x400, TREE_BILLBOARD_WORD]]
        );
    }

    #[test]
    fn tree_model_constructor_uses_the_trees_embedded_tes_model() {
        let mut e = model_engine();
        e.register(TES_NAME_SLOT, |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        // A tree with its TESModel (a table pointer) at +0x30.
        let tree = Ptr::<()>::new(e.mem.alloc(0x80));
        e.mem.set_u32(tree.addr() + 0x30, TES_MODEL_VTABLE);
        let name = text(&mut e, "trees\\oak.spt");
        e.mem.set_u32(tree.addr() + 0x30 + 0x38, name);
        let this: Ptr<QueuedTreeModel> = e.new_object();
        let reference = Ptr::<()>::new(0x8000);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_d850, &args![this, reference, tree, 0x31u32, 2u32]);
        assert_eq!(back.ptr::<QueuedTreeModel>(), this);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_TREE_MODEL_VTABLE);
        assert_eq!(e.get(this, QueuedTreeModel::pRef), reference);
        assert_eq!(e.get(this, QueuedTreeModel::pTree), tree);
        assert_eq!(e.get(this, QueuedTreeModel::eContext), 0x1e);
        // The `QueuedModel` part: the embedded TESModel, the context and
        // fade multiplier arguments, bit 1 set and bit 2 clear.
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x31]]
        );
        assert_eq!(
            e.get(this.cast::<QueuedModel>(), QueuedModel::pTESModel),
            Ptr::new(tree.addr() + 0x30)
        );
        assert_eq!(
            e.get(this.cast::<QueuedModel>(), QueuedModel::eLODFadeMult),
            2
        );
        assert_eq!(e.get(this.cast::<QueuedModel>(), QueuedModel::cFlags), 1);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME),
            vec![vec![this.addr(), name]]
        );
    }

    // --- The tree model, `QueuedMagicItem`, `QueuedKF` and its subclasses. ---

    /// A task's table with slot 0 deleting and slot 0x28 `CheckFinished`.
    const KF_TASK_VTABLE: u32 = 0x0ff6_0000;
    /// The icon part of a tree: slot 0x18 gives the directory text.
    const ICON_VTABLE: u32 = 0x0ff6_1000;
    const ICON_DIRECTORY_SLOT: u32 = 0x0ff0_0120;
    /// The anim idle map's table: slot 0x14 is the call `0043e810` makes.
    const ANIM_MAP_VTABLE: u32 = 0x0ff6_2000;
    const ANIM_MAP_SLOT: u32 = 0x0ff0_0121;
    /// The loader object (its anim idle map pointer is at +0x10) and the text
    /// the icon directory slot returns.
    const LOADER: u32 = 0x0ff6_3000;
    const ICON_DIRECTORY: u32 = LOADER + 0x800;
    /// The list `BuildDefaultModelList` gives, and the IO manager.
    const LIST_BLOCK: u32 = 0x0ff6_4000;
    const PART_IO_MANAGER: u32 = 0x0ff6_5000;
    /// Scratch dwords the doubles read: the answers of the KF model map.
    const FIND_KF_ANSWER: u32 = SCRATCH + 0x20;
    const ADD_KF_ANSWER: u32 = SCRATCH + 0x24;
    /// Where the texture-queue double leaves the path it was given.
    const QUEUED_PATH: u32 = SCRATCH + 0x40;
    /// The global counter `0040ab50` increments.
    const UNLOADED_ASSOCIATED_COUNT: u32 = 0x011c_3438;
    /// What the tree manager double answers for a tree node.
    const TREE_NODE: u32 = 0x7771;

    /// An engine with doubles for what this part calls, on top of the queued
    /// model ones.
    fn part_engine() -> Engine {
        let mut e = model_engine();
        for page in [0x011d_5000, LOADER, LIST_BLOCK, PART_IO_MANAGER] {
            e.map(page, 0x1000);
        }
        e.mem.set_cstr(ICON_DIRECTORY, b"textures\\trees\\");
        e.mem.set_u32(LOADER + 0x10, ANIM_MAP_OBJECT);
        e.mem.set_u32(PART_IO_MANAGER + 0x64, 0x7100);
        e.set_global(FILE_MAP_OWNER, LOADER);
        e.set_global(TASK_QUEUE, PART_IO_MANAGER);
        e.set_global(TREE_MANAGER, 0x7000_1000u32);
        e.register(TEXTURE_TEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(STRING_APPEND, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(ICON_DIRECTORY_SLOT, |_, _| ICON_DIRECTORY.into_ret());
        e.register(MODEL_LOADER_QUEUE_TEXTURE_BY_NAME, |e, a| {
            let path = e.mem.cstr(a[1]);
            e.mem.set_cstr(QUEUED_PATH, &path);
            Ret::default()
        });
        e.register(TREE_MANAGER_BUILD_TREE, |_, _| TREE_NODE.into_ret());
        for address in [
            TREE_MANAGER_STORE_TREE,
            TREE_MANAGER_REMOVE_BACKGROUND_TREE,
            MODEL_LOADER_QUEUE_MODEL,
            MODEL_LOADER_QUEUE_MODEL_LIST,
            MODEL_LOADER_QUEUE_ANIMATIONS,
            LIST_CLEAR,
            LIST_DELETE,
            MAGIC_ITEM_FINISHED_LOADING,
            QUEUED_FILE_CONSTRUCT,
            QUEUED_FILE_DESTRUCT,
            KF_POINTER_RELEASE,
            QUEUED_KF_QUEUE_ME,
            ANIM_IDLE_LOADED,
            ANIMATION_KF_LOADED,
            LOAD_KF_SEQUENCE,
            ANIM_MAP_SLOT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(GET_LOD_MULT, |_, _| 3u32.into_ret());
        e.register(GET_BIPED_TES_MODEL, |_, a| (0x6000 + a[1]).into_ret());
        e.register(BUILD_DEFAULT_MODEL_LIST, |_, _| LIST_BLOCK.into_ret());
        // The name length of a `TESModel`: a word at +8 in the tests.
        e.register(MODEL_NAME_LENGTH, |e, a| e.mem.u16(a[0] + 8).into_ret());
        e.register(EFFECT_SETTING_HAS_FLAG, |e, a| {
            (e.mem.u32(a[0] + 0x58) & a[1] != 0).into_ret()
        });
        e.register(EFFECT_IS_QUEUED, |e, a| {
            (e.mem.i32(a[0] + 0xa8) < 0).into_ret()
        });
        e.register(ASSOCIATED_ITEM_IS_QUEUED, |e, a| {
            (e.mem.i32(a[0] + 0xac) < 0).into_ret()
        });
        e.register(ASSOCIATED_ITEM_IS_LOADED, |e, a| {
            (e.mem.i32(a[0] + 0xac) > 0).into_ret()
        });
        e.register(EFFECT_COUNT_DOWN, |e, a| {
            let value = e.mem.i32(a[0] + 0xa8) - 1;
            e.mem.set_i32(a[0] + 0xa8, value);
            Ret::default()
        });
        e.register(ASSOCIATED_ITEM_COUNT_DOWN, |e, a| {
            let value = e.mem.i32(a[0] + 0xac) - 1;
            e.mem.set_i32(a[0] + 0xac, value);
            Ret::default()
        });
        e.register(EFFECT_ASSOCIATED_ITEM, |e, a| {
            e.mem.u32(a[0] + 0x60).into_ret()
        });
        e.register(FORM_TYPE, |e, a| e.mem.u8(a[0] + 4).into_ret());
        e.register(EFFECT_ITEM_SETTING, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(EFFECT_SETTING_IN_TABLE, |e, a| {
            (e.mem.u32(a[0] + 0x98) != 0).into_ret()
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(COUNT_UNLOADED_ASSOCIATED_ITEM, |e, _| {
            let value = e.mem.u32(UNLOADED_ASSOCIATED_COUNT) + 1;
            e.mem.set_u32(UNLOADED_ASSOCIATED_COUNT, value);
            Ret::default()
        });
        e.register(TASK_STATE_AT_LEAST_4, |e, a| {
            (e.mem.i32(a[0] + 0xc) >= 4).into_ret()
        });
        e.register(TASK_STATE_IS_6, |e, a| {
            (e.mem.i32(a[0] + 0xc) == 6).into_ret()
        });
        e.register(KF_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(KF_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(FIND_KF_MODEL, |e, _| e.mem.u32(FIND_KF_ANSWER).into_ret());
        e.register(ADD_KF_MODEL, |e, _| e.mem.u32(ADD_KF_ANSWER).into_ret());
        put_slots(
            &mut e,
            KF_TASK_VTABLE,
            &[(0, OBJECT_DELETE), (0x28, CHECK_FINISHED)],
        );
        put_slots(&mut e, ICON_VTABLE, &[(0x18, ICON_DIRECTORY_SLOT)]);
        put_slots(&mut e, ANIM_MAP_VTABLE, &[(0x14, ANIM_MAP_SLOT)]);
        e.mem.set_u32(ANIM_MAP_OBJECT, ANIM_MAP_VTABLE);
        e
    }

    /// The anim idle map object the loader at [`LOADER`] points to.
    const ANIM_MAP_OBJECT: u32 = LOADER + 0x400;

    /// A tree model whose tree has an icon part with the text `leaf`, a
    /// reference at 0x1111 and the priority byte 5 in its key.
    fn tree_model(e: &mut Engine, leaf: &str) -> Ptr<QueuedTreeModel> {
        let tree = Ptr::<()>::new(e.mem.alloc(0x80));
        e.mem.set_u32(tree.addr() + 0x48, ICON_VTABLE);
        let leaf = text(e, leaf);
        e.mem.set_u32(tree.addr() + 0x48 + 4, leaf);
        let this: Ptr<QueuedTreeModel> = e.new_object();
        e.mem.set_u32(this.addr(), MODEL_TASK_VTABLE);
        e.set(this, QueuedTreeModel::pRef, Ptr::new(0x1111));
        e.set(this, QueuedTreeModel::pTree, tree);
        e.mem.set_u64(this.addr() + 0x10, 5 << 16);
        this
    }

    #[test]
    fn tree_model_scalar_deleting_destructor_restores_the_table_and_frees() {
        let mut e = part_engine();
        let this: Ptr<QueuedTreeModel> = e.new_object();
        e.mem.set_u32(this.addr(), 0x1234);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_d8c0, &args![this, 0u32]);
        assert_eq!(back.ptr::<QueuedTreeModel>(), this);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_TREE_MODEL_VTABLE);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call(0x0043_d8c0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn tree_model_destructor_releases_the_model_then_the_base() {
        let mut e = part_engine();
        let this: Ptr<QueuedTreeModel> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0043_d8f0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_TREE_MODEL_VTABLE);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_d8f0,
                MODEL_POINTER_RELEASE,
                QUEUED_FILE_ENTRY_DESTRUCT
            ]
        );
        assert_eq!(
            calls_to(&e, MODEL_POINTER_RELEASE),
            vec![vec![this.addr() + 0x30]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_DESTRUCT),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn tree_model_queue_me_queues_the_leaf_texture_then_the_model() {
        let mut e = part_engine();
        let this = tree_model(&mut e, "leaf.dds");
        e.call_log = Some(vec![]);
        e.call(0x0043_d910, &args![this]);
        // directory + leaf text, copied into the second buffer.
        assert_eq!(e.mem.cstr(QUEUED_PATH), b"textures\\trees\\leaf.dds");
        let queued = calls_to(&e, MODEL_LOADER_QUEUE_TEXTURE_BY_NAME);
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0][0], LOADER);
        assert_eq!(&queued[0][2..], &[5, this.addr()]);
        // The path buffers are told their size (0x104).
        assert_eq!(calls_to(&e, STRING_COPY)[0][1], 0x104);
        assert_eq!(calls_to(&e, STRING_APPEND)[0][1], 0x104);
        assert_eq!(calls_to(&e, NORMALIZE_PATH)[0][2], 0x104);
        // `QueuedModel::QueueMe` follows: slot 0x30 of the task.
        let order = call_order(&e);
        let texture = order
            .iter()
            .position(|a| *a == MODEL_LOADER_QUEUE_TEXTURE_BY_NAME)
            .unwrap();
        let hook = order.iter().position(|a| *a == QUEUE_ME_HOOK).unwrap();
        assert!(texture < hook);
    }

    #[test]
    fn tree_model_queue_me_without_leaf_texture_only_queues_the_model() {
        let mut e = part_engine();
        let this = tree_model(&mut e, "");
        e.call_log = Some(vec![]);
        e.call(0x0043_d910, &args![this]);
        assert!(calls_to(&e, MODEL_LOADER_QUEUE_TEXTURE_BY_NAME).is_empty());
        assert!(calls_to(&e, STRING_COPY).is_empty());
        assert_eq!(calls_to(&e, QUEUE_ME_HOOK), vec![vec![this.addr(), 0]]);
    }

    #[test]
    fn tree_model_run_builds_the_tree_node_in_memory_context_0x1e() {
        let mut e = part_engine();
        let this = tree_model(&mut e, "");
        e.call_log = Some(vec![]);
        e.call(0x0043_da00, &args![this]);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_da00,
                MEMORY_CONTEXT_ENTER,
                TREE_MANAGER_BUILD_TREE,
                TREE_MANAGER_STORE_TREE,
                MEMORY_CONTEXT_LEAVE
            ]
        );
        let enter = &calls_to(&e, MEMORY_CONTEXT_ENTER)[0];
        assert_eq!(&enter[1..], &[0x1e, 1, MODEL_LOADER_SOURCE, 0x476]);
        let tree = e.get(this, QueuedTreeModel::pTree).addr();
        assert_eq!(
            calls_to(&e, TREE_MANAGER_BUILD_TREE),
            vec![vec![0x7000_1000, 0x1111, tree]]
        );
        assert_eq!(
            calls_to(&e, TREE_MANAGER_STORE_TREE),
            vec![vec![0x7000_1000, 0x1111, TREE_NODE]]
        );
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE), vec![vec![enter[0]]]);
    }

    #[test]
    fn tree_model_finish_calls_check_finished() {
        let mut e = part_engine();
        let this: Ptr<QueuedTreeModel> = e.new_object();
        e.mem.set_u32(this.addr(), KF_TASK_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x0043_daa0, &args![this]);
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![this.addr()]]);
    }

    #[test]
    fn tree_model_cancel_forgets_the_background_tree_from_state_4() {
        let mut e = part_engine();
        let this = tree_model(&mut e, "");
        for (first, forgotten) in [(3i32, false), (4, true), (6, true), (-1, false)] {
            e.call_log = Some(vec![]);
            e.call(0x0043_dac0, &args![this, first as u32, 9u32]);
            assert_eq!(
                calls_to(&e, QUEUED_FILE_CANCEL),
                vec![vec![this.addr(), first as u32, 9]]
            );
            let removed = calls_to(&e, TREE_MANAGER_REMOVE_BACKGROUND_TREE);
            if forgotten {
                assert_eq!(removed, vec![vec![0x7000_1000, 0x1111]]);
            } else {
                assert!(removed.is_empty());
            }
        }
    }

    #[test]
    fn tree_model_description_passes_the_words() {
        let mut e = part_engine();
        let this: Ptr<QueuedTreeModel> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_db00, &args![this, 0x9000u32, 0x400u32]);
        assert!(back.bool());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_DESCRIPTION),
            vec![vec![this.addr(), 0x9000, 0x400, TREE_MODEL_WORD]]
        );
    }

    #[test]
    fn magic_item_constructor_sets_table_item_and_effect() {
        let mut e = part_engine();
        let this: Ptr<QueuedMagicItem> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(
            0x0043_db30,
            &args![
                this,
                Ptr::<()>::new(0x5001),
                Ptr::<()>::new(0x5002),
                0x31u32
            ],
        );
        assert_eq!(back.ptr::<QueuedMagicItem>(), this);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CONSTRUCT),
            vec![vec![this.addr(), 0x31]]
        );
        assert_eq!(e.mem.u32(this.addr()), QUEUED_MAGIC_ITEM_VTABLE);
        assert_eq!(e.get(this, QueuedMagicItem::pMagicItem), Ptr::new(0x5001));
        assert_eq!(
            e.get(this, QueuedMagicItem::pEffectSetting),
            Ptr::new(0x5002)
        );
    }

    #[test]
    fn generate_key_packs_kind_counters_and_priority() {
        let mut e = part_engine();
        let this: Ptr<IOTask> = e.new_object();
        e.mem.set_u64(this.addr() + 0x10, 0x0000_0001_0007_1234);
        e.mem.set_u32(KEY_COUNTER_FIRST, 0x1_2344);
        e.mem.set_u32(KEY_COUNTER_SECOND, 0x00ab_cdef);
        e.call(0x0043_db70, &args![this]);
        // Counters advanced; the first one is cut to its low word.
        assert_eq!(e.mem.u32(KEY_COUNTER_FIRST), 0x1_2345);
        assert_eq!(e.mem.u32(KEY_COUNTER_SECOND), 0x00ab_cdf0);
        let expected = (3u64 << 56) | (0x00ab_cdf0u64 << 24) | (7 << 16) | 0x2345;
        assert_eq!(e.get(this, IOTask::Key), expected);
    }

    #[test]
    fn key_packing_takes_the_low_byte_and_word() {
        let mut e = part_engine();
        let this: Ptr<IOTask> = e.new_object();
        e.call(
            0x0043_dbb0,
            &args![this, 3u32, 0xdead_beefu32, 0x112u32, 0x1_abcdu32],
        );
        let expected = (3u64 << 56) | (0xdead_beefu64 << 24) | (0x12 << 16) | 0xabcd;
        assert_eq!(e.get(this, IOTask::Key), expected);
    }

    #[test]
    fn key_counters_return_the_incremented_values() {
        let mut e = part_engine();
        e.mem.set_u32(KEY_COUNTER_FIRST, 10);
        e.mem.set_u32(KEY_COUNTER_SECOND, 20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0043_dc10, &args![0x5000u32]).u32(), 11);
        assert_eq!(e.call(0x0043_dc30, &args![0x5000u32]).u32(), 21);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![KEY_COUNTER_FIRST], vec![KEY_COUNTER_SECOND]]
        );
    }

    #[test]
    fn magic_item_destructor_runs_the_queued_file_destructor_body() {
        let mut e = part_engine();
        let this: Ptr<QueuedMagicItem> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0043_dc50, &args![this]);
        assert_eq!(calls_to(&e, QUEUED_FILE_DESTRUCT), vec![vec![this.addr()]]);
    }

    /// An effect setting with `flags`, associated item `item` and a model of
    /// the given name length (0: none), whose test passes.
    fn effect_setting(e: &mut Engine, flags: u32, item: u32, name_length: u16) -> u32 {
        let setting = e.mem.alloc(0x100);
        e.mem.set_u32(setting + 0x58, flags);
        e.mem.set_u32(setting + 0x60, item);
        e.mem.set_u32(setting + 0x98, 1);
        e.mem.set_u16(setting + 0x18 + 8, name_length);
        setting
    }

    /// A form of type `kind` with a model name of `name_length` at +0x3c.
    fn associated_item(e: &mut Engine, kind: u8, name_length: u16) -> u32 {
        let item = e.mem.alloc(0x200);
        e.mem.set_u8(item + 4, kind);
        e.mem.set_u16(item + 0x3c + 8, name_length);
        item
    }

    /// A magic item whose effect list holds one effect item per setting.
    fn magic_item_with(e: &mut Engine, settings: &[u32]) -> Ptr {
        let magic = Ptr::new(e.mem.alloc(0x40));
        let mut node = magic.addr() + 0x10;
        for (i, setting) in settings.iter().enumerate() {
            let effect_item = e.mem.alloc(0x20);
            e.mem.set_u32(effect_item + 0x14, *setting);
            e.mem.set_u32(node, effect_item);
            if i + 1 < settings.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        magic
    }

    /// A queued magic item for `effect` and `magic` with the priority byte 4.
    fn queued_magic_item(e: &mut Engine, magic: Ptr, effect: u32) -> Ptr<QueuedMagicItem> {
        let this: Ptr<QueuedMagicItem> = e.new_object();
        e.mem.set_u32(this.addr(), KF_TASK_VTABLE);
        e.set(this, QueuedMagicItem::pMagicItem, magic);
        e.set(this, QueuedMagicItem::pEffectSetting, Ptr::new(effect));
        e.mem.set_u64(this.addr() + 0x10, 4 << 16);
        this
    }

    #[test]
    fn magic_item_queue_me_queues_the_effect_model_and_a_weapon_model() {
        let mut e = part_engine();
        // The task's effect has a model; the list's effect has a weapon.
        let weapon = associated_item(&mut e, 0x28, 2);
        let own = effect_setting(&mut e, 0, 0, 4);
        let listed = effect_setting(&mut e, 0x10000, weapon, 0);
        let magic = magic_item_with(&mut e, &[listed]);
        let this = queued_magic_item(&mut e, magic, own);
        e.call_log = Some(vec![]);
        e.call(0x0043_dc70, &args![this]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_MODEL),
            vec![
                vec![LOADER, own + 0x18, 4, this.addr(), 0, 1, 0, 0],
                vec![LOADER, weapon + 0x3c, 4, this.addr(), 3, 1, 0, 0],
            ]
        );
        // The effect's counter and the associated item's are counted down.
        assert_eq!(calls_to(&e, EFFECT_COUNT_DOWN), vec![vec![own]]);
        assert_eq!(calls_to(&e, ASSOCIATED_ITEM_COUNT_DOWN), vec![vec![listed]]);
        assert_eq!(e.mem.i32(own + 0xa8), -1);
        // Flag 0x40000 is clear: the global counter stays.
        assert!(calls_to(&e, COUNT_UNLOADED_ASSOCIATED_ITEM).is_empty());
        // The task ends in state 5 and calls `CheckFinished`.
        assert_eq!(calls_to(&e, TASK_SET_DONE), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![this.addr()]]);
        let order = call_order(&e);
        let done = order.iter().position(|a| *a == TASK_SET_DONE).unwrap();
        let finished = order.iter().position(|a| *a == CHECK_FINISHED).unwrap();
        assert!(done < finished);
    }

    #[test]
    fn magic_item_queue_me_queues_both_biped_models_of_an_armor() {
        let mut e = part_engine();
        let armor = associated_item(&mut e, 0x18, 0);
        // The task's effect has no model name: nothing is queued for it.
        let own = effect_setting(&mut e, 0, 0, 0);
        let listed = effect_setting(&mut e, 0x20000, armor, 0);
        let magic = magic_item_with(&mut e, &[listed]);
        let this = queued_magic_item(&mut e, magic, own);
        e.call_log = Some(vec![]);
        e.call(0x0043_dc70, &args![this]);
        assert_eq!(
            calls_to(&e, GET_BIPED_TES_MODEL),
            vec![vec![armor + 0x70, 0], vec![armor + 0x70, 1]]
        );
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_MODEL),
            vec![
                vec![LOADER, 0x6000, 4, this.addr(), 3, 1, 0, 0],
                vec![LOADER, 0x6001, 4, this.addr(), 3, 1, 0, 0],
            ]
        );
        // The effect's own counter is still counted down.
        assert_eq!(calls_to(&e, EFFECT_COUNT_DOWN), vec![vec![own]]);
    }

    #[test]
    fn magic_item_queue_me_queues_the_default_model_list_of_an_npc() {
        let mut e = part_engine();
        let npc = associated_item(&mut e, 0x2a, 0);
        let own = effect_setting(&mut e, 0, 0, 0);
        let listed = effect_setting(&mut e, 0x40000, npc, 0);
        let magic = magic_item_with(&mut e, &[listed]);
        let this = queued_magic_item(&mut e, magic, own);
        e.call_log = Some(vec![]);
        e.call(0x0043_dc70, &args![this]);
        assert_eq!(
            calls_to(&e, BUILD_DEFAULT_MODEL_LIST),
            vec![vec![npc, 1, 1]]
        );
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_MODEL_LIST),
            vec![vec![LOADER, LIST_BLOCK, 0, 4, this.addr(), 0]]
        );
        assert_eq!(
            call_order(&e)
                .into_iter()
                .filter(|a| [MODEL_LOADER_QUEUE_MODEL_LIST, LIST_CLEAR, LIST_DELETE].contains(a))
                .collect::<Vec<_>>(),
            vec![MODEL_LOADER_QUEUE_MODEL_LIST, LIST_CLEAR, LIST_DELETE]
        );
        assert_eq!(calls_to(&e, LIST_CLEAR), vec![vec![LIST_BLOCK]]);
        assert_eq!(calls_to(&e, LIST_DELETE), vec![vec![LIST_BLOCK, 1]]);
        // The item is neither loaded nor queued and the effect has flag
        // 0x40000: the global counter goes up.
        assert_eq!(e.mem.u32(UNLOADED_ASSOCIATED_COUNT), 1);
        // Without a list nothing is deleted.
        e.register(BUILD_DEFAULT_MODEL_LIST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0043_dc70, &args![this]);
        assert_eq!(calls_to(&e, LIST_CLEAR), vec![vec![0]]);
        assert!(calls_to(&e, LIST_DELETE).is_empty());
    }

    #[test]
    fn magic_item_queue_me_queues_the_animations_and_model_of_a_creature() {
        let mut e = part_engine();
        let creature = associated_item(&mut e, 0x2b, 0);
        let own = effect_setting(&mut e, 0, 0, 0);
        let listed = effect_setting(&mut e, 0x40000, creature, 0);
        // The item is already loaded: no counting of unloaded items.
        e.mem.set_i32(listed + 0xac, 1);
        let magic = magic_item_with(&mut e, &[listed]);
        let this = queued_magic_item(&mut e, magic, own);
        e.call_log = Some(vec![]);
        e.call(0x0043_dc70, &args![this]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_ANIMATIONS),
            vec![vec![LOADER, creature + 0xdc, 4, this.addr(), 0, 1, 1]]
        );
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_MODEL),
            vec![vec![LOADER, creature + 0xdc, 4, this.addr(), 3, 1, 0, 0]]
        );
        assert!(calls_to(&e, COUNT_UNLOADED_ASSOCIATED_ITEM).is_empty());
        assert_eq!(e.mem.i32(listed + 0xac), 0);
    }

    #[test]
    fn magic_item_queue_me_skips_what_the_rules_exclude() {
        let mut e = part_engine();
        let weapon = associated_item(&mut e, 0x28, 2);
        // The effect is already queued: its model is not queued again, and
        // the associated-item test is not consulted for the list's effects.
        let own = effect_setting(&mut e, 0, 0, 4);
        e.mem.set_i32(own + 0xa8, -1);
        // 1: fails the setting test. 2: no associated item. 3: flag 0x10000
        // with a form of another type. 4: flag 0x10000, weapon, no name.
        let failing = effect_setting(&mut e, 0x10000, weapon, 0);
        e.mem.set_u32(failing + 0x98, 0);
        let no_item = effect_setting(&mut e, 0x10000, 0, 0);
        let other = associated_item(&mut e, 0x18, 2);
        let wrong_type = effect_setting(&mut e, 0x10000, other, 0);
        let unnamed = associated_item(&mut e, 0x28, 0);
        let no_name = effect_setting(&mut e, 0x10000, unnamed, 0);
        let magic = magic_item_with(&mut e, &[failing, no_item, wrong_type, no_name]);
        let this = queued_magic_item(&mut e, magic, own);
        e.call_log = Some(vec![]);
        e.call(0x0043_dc70, &args![this]);
        assert!(calls_to(&e, MODEL_LOADER_QUEUE_MODEL).is_empty());
        // The effect itself is counted down; so is every accepted setting's
        // associated item (not the failing one).
        assert_eq!(calls_to(&e, EFFECT_COUNT_DOWN), vec![vec![own]]);
        assert_eq!(
            calls_to(&e, ASSOCIATED_ITEM_COUNT_DOWN),
            vec![vec![no_item], vec![wrong_type], vec![no_name]]
        );
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![this.addr()]]);
    }

    #[test]
    fn magic_item_queue_me_leaves_out_effects_whose_item_is_queued() {
        let mut e = part_engine();
        let weapon = associated_item(&mut e, 0x28, 2);
        // The task queues the effect's own model, and the listed effect's item
        // is already queued (counter negative): skipped entirely.
        let own = effect_setting(&mut e, 0, 0, 4);
        let listed = effect_setting(&mut e, 0x10000, weapon, 0);
        e.mem.set_i32(listed + 0xac, -1);
        let magic = magic_item_with(&mut e, &[listed]);
        let this = queued_magic_item(&mut e, magic, own);
        e.call_log = Some(vec![]);
        e.call(0x0043_dc70, &args![this]);
        assert_eq!(calls_to(&e, MODEL_LOADER_QUEUE_MODEL).len(), 1);
        assert!(calls_to(&e, ASSOCIATED_ITEM_COUNT_DOWN).is_empty());
        assert_eq!(e.mem.i32(listed + 0xac), -1);
        // With an empty effect list nothing else happens.
        let empty = Ptr::new(e.mem.alloc(0x40));
        e.set(this, QueuedMagicItem::pMagicItem, empty);
        e.call_log = Some(vec![]);
        e.call(0x0043_dc70, &args![this]);
        assert!(calls_to(&e, EFFECT_ITEM_SETTING).is_empty());
        assert_eq!(calls_to(&e, TASK_SET_DONE), vec![vec![this.addr()]]);
    }

    #[test]
    fn magic_item_queue_me_walks_every_effect_of_the_list() {
        let mut e = part_engine();
        let weapon = associated_item(&mut e, 0x28, 2);
        let own = effect_setting(&mut e, 0, 0, 0);
        let first = effect_setting(&mut e, 0x10000, weapon, 0);
        let second = effect_setting(&mut e, 0x10000, weapon, 0);
        let third = effect_setting(&mut e, 0, 0, 0);
        let magic = magic_item_with(&mut e, &[first, second, third]);
        let this = queued_magic_item(&mut e, magic, own);
        e.call_log = Some(vec![]);
        e.call(0x0043_dc70, &args![this]);
        assert_eq!(
            calls_to(&e, ASSOCIATED_ITEM_COUNT_DOWN),
            vec![vec![first], vec![second], vec![third]]
        );
        assert_eq!(calls_to(&e, MODEL_LOADER_QUEUE_MODEL).len(), 2);
    }

    #[test]
    fn magic_item_check_finished_waits_for_state_and_children() {
        let mut e = part_engine();
        let this: Ptr<QueuedMagicItem> = e.new_object();
        e.mem.set_u32(this.addr(), KF_TASK_VTABLE);
        e.set(this, QueuedMagicItem::pMagicItem, Ptr::new(0x5001));
        // Two children, one finished.
        let children = e.mem.alloc(0x20);
        e.mem.set_u32(children + 8, 2);
        e.mem.set_u32(children + 0x10, 1);
        e.mem.set_u32(this.addr() + 0x20, children);
        e.mem.set_i32(this.addr() + 0xc, 3);
        e.call_log = Some(vec![]);
        e.call(0x0043_e080, &args![this]);
        // State below 4: nothing.
        e.mem.set_i32(this.addr() + 0xc, 4);
        e.call(0x0043_e080, &args![this]);
        // Children unfinished: nothing.
        assert!(calls_to(&e, MAGIC_ITEM_FINISHED_LOADING).is_empty());
        // All finished with references left: the magic item is told, the task
        // stays.
        e.mem.set_u32(children + 0x10, 2);
        e.mem.set_i32(this.addr() + 8, 1);
        e.call(0x0043_e080, &args![this]);
        assert_eq!(
            calls_to(&e, MAGIC_ITEM_FINISHED_LOADING),
            vec![vec![0x5001]]
        );
        assert!(calls_to(&e, OBJECT_DELETE).is_empty());
        // Without references it deletes itself (virtual destructor, flag 1).
        e.mem.set_i32(this.addr() + 8, 0);
        e.call(0x0043_e080, &args![this]);
        assert_eq!(calls_to(&e, OBJECT_DELETE), vec![vec![this.addr(), 1]]);
    }

    /// A queued KF with the file name `name`, the table with `CheckFinished`
    /// and a stale flag byte.
    fn queued_kf(e: &mut Engine, name: &str) -> Ptr<QueuedKF> {
        let this: Ptr<QueuedKF> = e.new_object();
        e.mem.set_u32(this.addr(), KF_TASK_VTABLE);
        let name = text(e, name);
        e.set(this, QueuedKF::pFileName, name);
        this
    }

    #[test]
    fn kf_constructor_sets_up_the_entry_and_the_model_pointer() {
        let mut e = part_engine();
        let this: Ptr<QueuedKF> = e.new_object();
        e.set(this, QueuedKF::cFlags, 0xff);
        let name = text(&mut e, "a.kf");
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_e0f0, &args![this, name, 0x33u32]);
        assert_eq!(back.ptr::<QueuedKF>(), this);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_e0f0,
                QUEUED_FILE_ENTRY_CONSTRUCT,
                KF_POINTER_CONSTRUCT,
                QUEUED_FILE_ENTRY_SET_FILE_NAME,
                QUEUED_FILE_ENTRY_FIND_FILE_ENTRY
            ]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x33]]
        );
        assert_eq!(
            calls_to(&e, KF_POINTER_CONSTRUCT),
            vec![vec![this.addr() + 0x30, 0]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME),
            vec![vec![this.addr(), name]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_FIND_FILE_ENTRY),
            vec![vec![this.addr(), 0]]
        );
        assert_eq!(e.mem.u32(this.addr()), QUEUED_KF_VTABLE);
        assert_eq!(e.get(this, QueuedKF::cFlags), 0);
    }

    #[test]
    fn kf_scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = part_engine();
        let this = queued_kf(&mut e, "a.kf");
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_e180, &args![this, 0u32]);
        assert_eq!(back.ptr::<QueuedKF>(), this);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        assert_eq!(calls_to(&e, KF_POINTER_RELEASE).len(), 1);
        e.call(0x0043_e180, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn kf_destructor_releases_the_model_then_the_entry() {
        let mut e = part_engine();
        let this = queued_kf(&mut e, "a.kf");
        e.call_log = Some(vec![]);
        e.call(0x0043_e1b0, &args![this]);
        assert_eq!(
            call_order(&e),
            vec![0x0043_e1b0, KF_POINTER_RELEASE, QUEUED_FILE_ENTRY_DESTRUCT]
        );
        assert_eq!(
            calls_to(&e, KF_POINTER_RELEASE),
            vec![vec![this.addr() + 0x30]]
        );
        // The table is left alone.
        assert_eq!(e.mem.u32(this.addr()), KF_TASK_VTABLE);
    }

    #[test]
    fn kf_constructor_with_a_model_starts_done() {
        let mut e = part_engine();
        let this: Ptr<QueuedKF> = e.new_object();
        e.set(this, QueuedKF::cFlags, 0xff);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_e210, &args![this, Ptr::<()>::new(0x4b40), 0x33u32]);
        assert_eq!(back.ptr::<QueuedKF>(), this);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_e210,
                QUEUED_FILE_ENTRY_CONSTRUCT,
                KF_POINTER_CONSTRUCT,
                KF_POINTER_ASSIGN,
                TASK_SET_DONE
            ]
        );
        assert_eq!(
            calls_to(&e, KF_POINTER_ASSIGN),
            vec![vec![this.addr() + 0x30, 0x4b40]]
        );
        assert_eq!(e.get(this, QueuedKF::spKFModel), Ptr::new(0x4b40));
        assert_eq!(e.mem.u32(this.addr()), QUEUED_KF_VTABLE);
        assert_eq!(e.get(this, QueuedKF::cFlags), 0);
        assert_eq!(calls_to(&e, TASK_SET_DONE), vec![vec![this.addr()]]);
    }

    #[test]
    fn kf_run_takes_the_loaders_model_and_sets_bit_1() {
        let mut e = part_engine();
        let this = queued_kf(&mut e, "a.kf");
        e.mem.set_u32(FIND_KF_ANSWER, 0x4b40);
        e.call_log = Some(vec![]);
        e.call(0x0043_e2a0, &args![this]);
        let enter = &calls_to(&e, MEMORY_CONTEXT_ENTER)[0];
        assert_eq!(&enter[1..], &[0x33, 1, MODEL_LOADER_SOURCE, 0x529]);
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE), vec![vec![enter[0]]]);
        let name = e.get(this, QueuedKF::pFileName);
        assert_eq!(calls_to(&e, FIND_KF_MODEL), vec![vec![LOADER, name]]);
        assert_eq!(e.get(this, QueuedKF::spKFModel), Ptr::new(0x4b40));
        assert_eq!(e.get(this, QueuedKF::cFlags) & 1, 1);
        // Nothing is opened or read.
        assert!(calls_to(&e, QUEUED_FILE_ENTRY_GET_FILE).is_empty());
        assert!(calls_to(&e, BS_STREAM_LOAD).is_empty());
    }

    #[test]
    fn kf_run_reads_the_file_into_a_new_model() {
        let mut e = part_engine();
        let this = queued_kf(&mut e, "a.kf");
        e.call_log = Some(vec![]);
        e.call(0x0043_e2a0, &args![this]);
        let name = e.get(this, QueuedKF::pFileName);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_FILE),
            vec![vec![this.addr(), 0, 1]]
        );
        let load = &calls_to(&e, BS_STREAM_LOAD)[0];
        assert_eq!(&load[1..], &[name, OPEN_FILE]);
        // A 0x14-byte `KFModel` was built from the stream and stored.
        assert_eq!(calls_to(&e, MEMORY_ALLOC)[0], vec![0x14]);
        let model = e.get(this, QueuedKF::spKFModel);
        assert!(!model.is_null());
        assert_eq!(
            calls_to(&e, LOAD_KF_SEQUENCE),
            vec![vec![load[0], 0, model.addr() + 4]]
        );
        assert_eq!(e.get(this, QueuedKF::cFlags) & 1, 0);
        // The stream is destroyed again.
        assert_eq!(calls_to(&e, NI_STREAM_DESTRUCT).len(), 1);
    }

    #[test]
    fn kf_run_logs_a_file_that_cannot_be_opened_or_read() {
        let mut e = part_engine();
        let this = queued_kf(&mut e, "a.kf");
        let name = e.get(this, QueuedKF::pFileName);
        // No file: "find".
        e.register(QUEUED_FILE_ENTRY_GET_FILE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0043_e2a0, &args![this]);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![MODEL_ERROR_MESSAGE, FIND_WORD, name]]
        );
        assert!(calls_to(&e, BS_STREAM_LOAD).is_empty());
        assert!(e.get(this, QueuedKF::spKFModel).is_null());
        // A file the stream cannot read: "load".
        e.register(QUEUED_FILE_ENTRY_GET_FILE, |_, _| OPEN_FILE.into_ret());
        e.register(BS_STREAM_LOAD, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0043_e2a0, &args![this]);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![MODEL_ERROR_MESSAGE, LOAD_WORD, name]]
        );
        assert!(calls_to(&e, MEMORY_ALLOC).is_empty());
        assert!(e.get(this, QueuedKF::spKFModel).is_null());
    }

    #[test]
    fn kf_flag_bit_1_is_set_and_cleared() {
        let mut e = part_engine();
        let this = queued_kf(&mut e, "a.kf");
        e.set(this, QueuedKF::cFlags, 4);
        e.call_log = Some(vec![]);
        e.call(0x0043_e480, &args![this, 1u32]);
        assert_eq!(e.get(this, QueuedKF::cFlags), 5);
        assert_eq!(
            calls_to(&e, SET_FLAG_BIT_1),
            vec![vec![1, this.addr() + 0x34]]
        );
        e.call(0x0043_e480, &args![this, 0u32]);
        assert_eq!(e.get(this, QueuedKF::cFlags), 4);
    }

    #[test]
    fn kf_finish_adds_the_model_to_the_loader_or_takes_the_existing_one() {
        let mut e = part_engine();
        let this = queued_kf(&mut e, "a.kf");
        let name = e.get(this, QueuedKF::pFileName);
        // No model: only `CheckFinished`.
        e.call_log = Some(vec![]);
        e.call(0x0043_e4b0, &args![this]);
        assert!(calls_to(&e, ADD_KF_MODEL).is_empty());
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![this.addr()]]);
        // A model the loader had already (bit 1): not added.
        e.set(this, QueuedKF::spKFModel, Ptr::new(0x4b40));
        e.set(this, QueuedKF::cFlags, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_e4b0, &args![this]);
        assert!(calls_to(&e, ADD_KF_MODEL).is_empty());
        // A new model the loader accepts.
        e.set(this, QueuedKF::cFlags, 0);
        e.mem.set_u32(ADD_KF_ANSWER, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_e4b0, &args![this]);
        assert_eq!(calls_to(&e, ADD_KF_MODEL), vec![vec![LOADER, name, 0x4b40]]);
        assert!(calls_to(&e, FIND_KF_MODEL).is_empty());
        assert_eq!(e.get(this, QueuedKF::spKFModel), Ptr::new(0x4b40));
        // A name the loader has: its model replaces ours.
        e.mem.set_u32(ADD_KF_ANSWER, 0);
        e.mem.set_u32(FIND_KF_ANSWER, 0x4b99);
        e.call_log = Some(vec![]);
        e.call(0x0043_e4b0, &args![this]);
        assert_eq!(calls_to(&e, FIND_KF_MODEL), vec![vec![LOADER, name]]);
        assert_eq!(e.get(this, QueuedKF::spKFModel), Ptr::new(0x4b99));
        assert_eq!(calls_to(&e, CHECK_FINISHED).len(), 1);
    }

    #[test]
    fn kf_flag_bit_1_test() {
        let mut e = part_engine();
        let this = queued_kf(&mut e, "a.kf");
        for (flags, expected) in [(0, false), (2, false), (1, true), (7, true)] {
            e.set(this, QueuedKF::cFlags, flags);
            assert_eq!(e.call(0x0043_e530, &args![this]).bool(), expected);
        }
    }

    #[test]
    fn kf_description_passes_the_words() {
        let mut e = part_engine();
        let this = queued_kf(&mut e, "a.kf");
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_e550, &args![this, 0x9000u32, 0x400u32]);
        assert!(back.bool());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_DESCRIPTION),
            vec![vec![this.addr(), 0x9000, 0x400, KF_WORD]]
        );
    }

    /// A queued anim idle for `idle` whose KF part has the name `a.kf`.
    fn queued_anim_idle(e: &mut Engine, idle: u32) -> Ptr<QueuedAnimIdle> {
        let this: Ptr<QueuedAnimIdle> = e.new_object();
        e.mem.set_u32(this.addr(), KF_TASK_VTABLE);
        e.set(this, QueuedAnimIdle::spAnimIdle, Ptr::new(idle));
        e.mem.set_u64(this.addr() + 0x10, 6 << 16);
        this
    }

    #[test]
    fn anim_idle_constructor_builds_the_kf_part_then_its_own_fields() {
        let mut e = part_engine();
        let this: Ptr<QueuedAnimIdle> = e.new_object();
        let name = text(&mut e, "a.kf");
        e.call_log = Some(vec![]);
        let back = e.call(
            0x0043_e580,
            &args![
                this,
                name,
                0x33u32,
                Ptr::<()>::new(0x6100),
                Ptr::<()>::new(0x6200)
            ],
        );
        assert_eq!(back.ptr::<QueuedAnimIdle>(), this);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME),
            vec![vec![this.addr(), name]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x33]]
        );
        assert_eq!(e.mem.u32(this.addr()), QUEUED_ANIM_IDLE_VTABLE);
        assert_eq!(e.get(this, QueuedAnimIdle::pQueuedForRef), Ptr::new(0x6200));
        assert_eq!(e.get(this, QueuedAnimIdle::spAnimIdle), Ptr::new(0x6100));
        assert_eq!(
            calls_to(&e, NI_POINTER_CONSTRUCT),
            vec![vec![this.addr() + 0x3c, 0x6100]]
        );
    }

    #[test]
    fn anim_idle_scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = part_engine();
        let this = queued_anim_idle(&mut e, 0x6100);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_e600, &args![this, 0u32]);
        assert_eq!(back.ptr::<QueuedAnimIdle>(), this);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call(0x0043_e600, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn anim_idle_destructor_drops_the_idle_then_runs_the_kf_destructor() {
        let mut e = part_engine();
        let this = queued_anim_idle(&mut e, 0x6100);
        e.call_log = Some(vec![]);
        e.call(0x0043_e630, &args![this]);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_e630,
                NI_POINTER_DESTRUCT,
                KF_POINTER_RELEASE,
                QUEUED_FILE_ENTRY_DESTRUCT
            ]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_DESTRUCT),
            vec![vec![this.addr() + 0x3c]]
        );
    }

    #[test]
    fn anim_idle_queue_me_queues_the_models_of_the_existing_objects() {
        let mut e = part_engine();
        let idle = e.mem.alloc(0x40);
        let animation = e.mem.alloc(0x40);
        // Object 0 exists, object 1 does not.
        e.mem.set_u32(idle + 0x1c, animation);
        let this = queued_anim_idle(&mut e, idle);
        e.call_log = Some(vec![]);
        e.call(0x0043_e690, &args![this]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_MODEL),
            vec![vec![LOADER, animation + 0x18, 6, this.addr(), 2, 1, 0, 0]]
        );
        // `QueuedKF::QueueMe` closes it.
        assert_eq!(*call_order(&e).last().unwrap(), QUEUED_KF_QUEUE_ME);
        assert_eq!(calls_to(&e, QUEUED_KF_QUEUE_ME), vec![vec![this.addr()]]);
        // Both objects present: two models, in index order.
        let second = e.mem.alloc(0x40);
        e.mem.set_u32(idle + 0x20, second);
        e.call_log = Some(vec![]);
        e.call(0x0043_e690, &args![this]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_MODEL)
                .iter()
                .map(|call| call[1])
                .collect::<Vec<_>>(),
            vec![animation + 0x18, second + 0x18]
        );
        // Without an anim idle only the base runs.
        e.set(this, QueuedAnimIdle::spAnimIdle, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0043_e690, &args![this]);
        assert!(calls_to(&e, MODEL_LOADER_QUEUE_MODEL).is_empty());
        assert_eq!(calls_to(&e, QUEUED_KF_QUEUE_ME).len(), 1);
    }

    #[test]
    fn anim_idle_object_accessor_indexes_the_two_dwords_at_0x1c() {
        let mut e = part_engine();
        let idle = e.mem.alloc(0x40);
        e.mem.set_u32(idle + 0x1c, 0x11);
        e.mem.set_u32(idle + 0x20, 0x22);
        assert_eq!(
            e.call(0x0043_e750, &args![Ptr::<()>::new(idle), 0u32])
                .u32(),
            0x11
        );
        assert_eq!(
            e.call(0x0043_e750, &args![Ptr::<()>::new(idle), 1u32])
                .u32(),
            0x22
        );
    }

    #[test]
    fn anim_idle_check_finished_posts_when_state_and_children_allow() {
        let mut e = part_engine();
        let this = queued_anim_idle(&mut e, 0x6100);
        let children = e.mem.alloc(0x20);
        e.mem.set_u32(children + 8, 1);
        e.mem.set_u32(this.addr() + 0x20, children);
        // State below 4.
        e.mem.set_i32(this.addr() + 0xc, 3);
        e.call_log = Some(vec![]);
        e.call(0x0043_e770, &args![this]);
        assert!(calls_to(&e, TASK_POINTER_CONSTRUCT).is_empty());
        // State 4, one child unfinished.
        e.mem.set_i32(this.addr() + 0xc, 4);
        e.call(0x0043_e770, &args![this]);
        assert!(calls_to(&e, TASK_POINTER_CONSTRUCT).is_empty());
        // All finished: queued under the priority byte (6).
        e.mem.set_u32(children + 0x10, 1);
        e.call(0x0043_e770, &args![this]);
        let holder = calls_to(&e, TASK_POINTER_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, TASK_POINTER_CONSTRUCT),
            vec![vec![holder, this.addr()]]
        );
        assert_eq!(calls_to(&e, QUEUE_TABLE_ADD), vec![vec![0x7100, 6, holder]]);
    }

    #[test]
    fn anim_idle_post_process_tells_the_idle_and_the_loader() {
        let mut e = part_engine();
        let this = queued_anim_idle(&mut e, 0x6100);
        e.set(this, QueuedAnimIdle::spKFModel, Ptr::new(0x4b40));
        e.call_log = Some(vec![]);
        e.call(0x0043_e7b0, &args![this]);
        assert_eq!(calls_to(&e, ANIM_IDLE_LOADED), vec![vec![0x6100, 0x4b40]]);
        assert_eq!(
            calls_to(&e, ANIM_MAP_SLOT),
            vec![vec![ANIM_MAP_OBJECT, 0x6100]]
        );
        // State 6: the idle is not told, the loader still is.
        e.mem.set_i32(this.addr() + 0xc, 6);
        e.call_log = Some(vec![]);
        e.call(0x0043_e7b0, &args![this]);
        assert!(calls_to(&e, ANIM_IDLE_LOADED).is_empty());
        assert_eq!(calls_to(&e, ANIM_MAP_SLOT).len(), 1);
        // No model: the same.
        e.mem.set_i32(this.addr() + 0xc, 5);
        e.set(this, QueuedAnimIdle::spKFModel, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0043_e7b0, &args![this]);
        assert!(calls_to(&e, ANIM_IDLE_LOADED).is_empty());
        assert_eq!(calls_to(&e, ANIM_MAP_SLOT).len(), 1);
    }

    #[test]
    fn loader_anim_idle_map_slot_0x14_is_called_with_the_idle() {
        let mut e = part_engine();
        e.call_log = Some(vec![]);
        e.call(
            0x0043_e810,
            &args![Ptr::<()>::new(LOADER), Ptr::<()>::new(0x6100)],
        );
        assert_eq!(
            calls_to(&e, ANIM_MAP_SLOT),
            vec![vec![ANIM_MAP_OBJECT, 0x6100]]
        );
    }

    #[test]
    fn anim_idle_cancel_runs_the_base_then_tells_the_loader() {
        let mut e = part_engine();
        let this = queued_anim_idle(&mut e, 0x6100);
        e.call_log = Some(vec![]);
        e.call(0x0043_e840, &args![this, 5u32, 6u32]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CANCEL),
            vec![vec![this.addr(), 5, 6]]
        );
        assert_eq!(
            calls_to(&e, ANIM_MAP_SLOT),
            vec![vec![ANIM_MAP_OBJECT, 0x6100]]
        );
        let order = call_order(&e);
        let cancel = order.iter().position(|a| *a == QUEUED_FILE_CANCEL).unwrap();
        let slot = order.iter().position(|a| *a == ANIM_MAP_SLOT).unwrap();
        assert!(cancel < slot);
    }

    #[test]
    fn anim_idle_description_passes_the_words() {
        let mut e = part_engine();
        let this = queued_anim_idle(&mut e, 0x6100);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_e880, &args![this, 0x9000u32, 0x400u32]);
        assert!(back.bool());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_DESCRIPTION),
            vec![vec![this.addr(), 0x9000, 0x400, ANIM_IDLE_WORD]]
        );
    }

    /// A queued replacement KF in `state` for the list `owner` with the
    /// animation 0x6300 and, when `loaded`, a model.
    fn queued_replacement_kf(
        e: &mut Engine,
        owner: Ptr,
        state: i32,
        loaded: bool,
    ) -> Ptr<QueuedReplacementKF> {
        let this: Ptr<QueuedReplacementKF> = e.new_object();
        e.mem.set_u32(this.addr(), KF_TASK_VTABLE);
        e.mem.set_i32(this.addr() + 0xc, state);
        e.mem.set_u64(this.addr() + 0x10, 2 << 16);
        e.set(this, QueuedReplacementKF::pAnim, Ptr::new(0x6300));
        e.set(this, QueuedReplacementKF::pOwner, owner);
        if loaded {
            e.set(this, QueuedReplacementKF::spKFModel, Ptr::new(0x4b40));
            // Bit 1: the loader had the model already.
            e.set(this, QueuedReplacementKF::cFlags, 1);
        }
        this
    }

    #[test]
    fn replacement_kf_constructor_sets_animation_and_owner() {
        let mut e = part_engine();
        let this: Ptr<QueuedReplacementKF> = e.new_object();
        let name = text(&mut e, "a.kf");
        e.call_log = Some(vec![]);
        let back = e.call(
            0x0043_e8b0,
            &args![
                this,
                name,
                0x33u32,
                Ptr::<()>::new(0x6300),
                Ptr::<()>::new(0x6400)
            ],
        );
        assert_eq!(back.ptr::<QueuedReplacementKF>(), this);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME),
            vec![vec![this.addr(), name]]
        );
        assert_eq!(e.mem.u32(this.addr()), QUEUED_REPLACEMENT_KF_VTABLE);
        assert_eq!(e.get(this, QueuedReplacementKF::pAnim), Ptr::new(0x6300));
        assert_eq!(e.get(this, QueuedReplacementKF::pOwner), Ptr::new(0x6400));
    }

    #[test]
    fn replacement_kf_scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = part_engine();
        let this = queued_replacement_kf(&mut e, Ptr::NULL, 0, false);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_e8f0, &args![this, 0u32]);
        assert_eq!(back.ptr::<QueuedReplacementKF>(), this);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        assert_eq!(calls_to(&e, QUEUED_FILE_ENTRY_DESTRUCT).len(), 1);
        e.call(0x0043_e8f0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn replacement_kf_destructor_is_the_kf_destructor() {
        let mut e = part_engine();
        let this = queued_replacement_kf(&mut e, Ptr::NULL, 0, false);
        e.call_log = Some(vec![]);
        e.call(0x0043_e920, &args![this]);
        assert_eq!(
            call_order(&e),
            vec![0x0043_e920, KF_POINTER_RELEASE, QUEUED_FILE_ENTRY_DESTRUCT]
        );
    }

    #[test]
    fn replacement_kf_finish_counts_the_child_and_posts_the_task() {
        let mut e = part_engine();
        let list: Ptr<QueuedReplacementKFList> = e.new_object();
        e.set(list, QueuedReplacementKFList::iPostProcessingChildCount, 2);
        let this = queued_replacement_kf(&mut e, list.cast(), 4, true);
        e.call_log = Some(vec![]);
        e.call(0x0043_e940, &args![this]);
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessingChildCount),
            3
        );
        let holder = calls_to(&e, TASK_POINTER_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, QUEUE_TABLE_ADD), vec![vec![0x7100, 2, holder]]);
        // `QueuedKF::Finish` ran first: `CheckFinished` before the post.
        let order = call_order(&e);
        let finished = order.iter().position(|a| *a == CHECK_FINISHED).unwrap();
        let post = order
            .iter()
            .position(|a| *a == TASK_POINTER_CONSTRUCT)
            .unwrap();
        assert!(finished < post);
        // Without an owner the task is still posted.
        let orphan = queued_replacement_kf(&mut e, Ptr::NULL, 4, true);
        e.call_log = Some(vec![]);
        e.call(0x0043_e940, &args![orphan]);
        assert_eq!(calls_to(&e, TASK_POINTER_CONSTRUCT).len(), 1);
        // State 6 or no model: neither counted nor posted.
        let cancelled = queued_replacement_kf(&mut e, list.cast(), 6, true);
        let empty = queued_replacement_kf(&mut e, list.cast(), 4, false);
        e.call_log = Some(vec![]);
        e.call(0x0043_e940, &args![cancelled]);
        e.call(0x0043_e940, &args![empty]);
        assert!(calls_to(&e, TASK_POINTER_CONSTRUCT).is_empty());
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessingChildCount),
            3
        );
    }

    #[test]
    fn replacement_kf_list_counter_wraps() {
        let mut e = part_engine();
        let list: Ptr<QueuedReplacementKFList> = e.new_object();
        e.call(0x0043_e990, &args![list]);
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessingChildCount),
            1
        );
        e.set(
            list,
            QueuedReplacementKFList::iPostProcessingChildCount,
            u32::MAX,
        );
        e.call(0x0043_e990, &args![list]);
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessingChildCount),
            0
        );
    }

    #[test]
    fn replacement_kf_post_process_hands_the_model_to_the_animation() {
        let mut e = part_engine();
        let list: Ptr<QueuedReplacementKFList> = e.new_object();
        e.set(list, QueuedReplacementKFList::iPostProcessingChildCount, 2);
        let this = queued_replacement_kf(&mut e, list.cast(), 4, true);
        e.call_log = Some(vec![]);
        e.call(0x0043_e9b0, &args![this]);
        assert_eq!(
            calls_to(&e, ANIMATION_KF_LOADED),
            vec![vec![0x6300, 0x4b40]]
        );
        // The owning list counted the child (its function is in this file,
        // so the call is direct).
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessedChildCount),
            1
        );
        // No owner: the list is not told.
        let orphan = queued_replacement_kf(&mut e, Ptr::NULL, 4, true);
        e.call_log = Some(vec![]);
        e.call(0x0043_e9b0, &args![orphan]);
        assert_eq!(calls_to(&e, ANIMATION_KF_LOADED).len(), 1);
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessedChildCount),
            1
        );
        // State 6: nothing at all.
        let cancelled = queued_replacement_kf(&mut e, list.cast(), 6, true);
        e.call_log = Some(vec![]);
        e.call(0x0043_e9b0, &args![cancelled]);
        assert!(calls_to(&e, ANIMATION_KF_LOADED).is_empty());
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessedChildCount),
            1
        );
    }

    // --- The fifth part: the replacement KF list, `QueuedHead`,
    // `QueuedHelmet`, the distant-3D task and `QueuedReference`. ---

    /// Data the doubles of this part read: the answers of `CloneHelmet` and
    /// `LoadFaceGenModel` by slot, and the answer of the face coordinates
    /// test.
    const HELMET_DATA: u32 = 0x0ff7_0000;
    const CLONE_ANSWERS: u32 = HELMET_DATA;
    const FACE_ANSWERS: u32 = HELMET_DATA + 0x80;
    const APPLY_ANSWER: u32 = HELMET_DATA + 0x100;
    /// The tables of a child node (slot 0x18 answers the dword at +0xB0,
    /// slot 0xE4 takes the deep copy), of the loader's helmet map and
    /// replacement KF list map (slot 0x14), and of a reference task (slot
    /// 0x3c).
    const CHILD_VTABLE: u32 = 0x0ff7_1000;
    const CHILD_MARKED_SLOT: u32 = 0x0ff0_0130;
    const CHILD_SET_PART_SLOT: u32 = 0x0ff0_0131;
    const HELMET_MAP_VTABLE: u32 = 0x0ff7_2000;
    const HELMET_MAP_SLOT: u32 = 0x0ff0_0132;
    const KF_LIST_MAP_VTABLE: u32 = 0x0ff7_3000;
    const KF_LIST_MAP_SLOT: u32 = 0x0ff0_0133;
    const REFERENCE_TASK_VTABLE: u32 = 0x0ff7_4000;
    const ATTACH_HOOK: u32 = 0x0ff0_0134;
    const HELMET_MAP_OBJECT: u32 = LOADER + 0x500;
    const KF_LIST_MAP_OBJECT: u32 = LOADER + 0x600;
    /// The nodes `InitHead` gives, the player and the `TES` object.
    const BIPED_NODE: u32 = 0x6a01;
    const SKINNED_NODE: u32 = 0x6a02;
    const PLAYER: u32 = 0x7ee0_0001;
    const TES_OBJECT: u32 = 0x7ee0_0002;

    /// An engine with doubles for what this part calls, on top of the
    /// previous part's.
    fn helmet_engine() -> Engine {
        let mut e = part_engine();
        e.map(HELMET_DATA, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem
            .set_f32(HELMET_ROTATION_ANGLE, -std::f32::consts::FRAC_PI_2);
        e.mem.set_u32(PART_IO_MANAGER, QUEUE_VTABLE);
        e.mem.set_u32(HELMET_MAP_OBJECT, HELMET_MAP_VTABLE);
        e.mem.set_u32(KF_LIST_MAP_OBJECT, KF_LIST_MAP_VTABLE);
        e.mem.set_u32(LOADER + 0x14, KF_LIST_MAP_OBJECT);
        e.mem.set_u32(LOADER + 0x18, HELMET_MAP_OBJECT);
        e.set_global(PLAYER_CHARACTER, PLAYER);
        e.set_global(TES_GLOBAL, TES_OBJECT);
        e.mem
            .set_cstr(QUEUED_HEAD_FORMAT, b"Queued head for NPC %s");
        e.mem
            .set_cstr(QUEUED_HELMET_FORMAT, b"Queued helmet for biped anim %08X");
        e.register(FORMAT_STRING, |e, a| {
            let format = String::from_utf8(e.mem.cstr(a[2])).unwrap();
            let text = if format.contains("%s") {
                format.replace("%s", &String::from_utf8(e.mem.cstr(a[3])).unwrap())
            } else {
                format.replace("%08X", &format!("{:08X}", a[3]))
            };
            e.mem.set_cstr(a[0], text.as_bytes());
            Ret::default()
        });
        e.register(NAME_OBJECT_STRING, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FIELD_AT_C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(NPC_INIT_HEAD, |e, a| {
            e.mem.set_u32(a[1], BIPED_NODE);
            e.mem.set_u32(a[2], SKINNED_NODE);
            Ret::default()
        });
        e.register(REFERENCE_BASE_FORM, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        e.register(BIPED_ANIM_SLOT_IN_USE, |e, a| {
            (e.mem.u32(a[0] + 0x30 + 0x10 * a[1]) != 0).into_ret()
        });
        e.register(BIPED_ANIM_LOAD_FACE_GEN_MODEL, |e, a| {
            e.mem.u32(FACE_ANSWERS + 4 * a[1]).into_ret()
        });
        e.register(BIPED_ANIM_CLONE_HELMET, |e, a| {
            e.mem.u32(CLONE_ANSWERS + 4 * a[3]).into_ret()
        });
        e.register(QUEUED_MODEL_GET_MODEL, |e, a| {
            e.mem.u32(a[0] + 0x30).into_ret()
        });
        e.register(OBJECT_POINTER_AT_B8, |e, a| {
            e.mem.u32(a[0] + 0xb8).into_ret()
        });
        e.register(CREATE_DEEP_COPY, |e, a| {
            e.mem.set_u32(a[1], 0x7d00_0000 + a[0]);
            Ret::default()
        });
        e.register(FACE_GEN_MODEL_APPLY_COORDS, |e, _| {
            e.mem.u32(APPLY_ANSWER).into_ret()
        });
        e.register(SETTING_BYTE_POINTER, |_, a| (a[0] + 4).into_ret());
        e.register(LOCAL_ROTATE_ADDRESS, |_, a| (a[0] + 0x34).into_ret());
        e.register(MATRIX_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SIN_COS, |e, a| {
            let (sin, cos) = (f32::from_bits(a[0]) as f64).sin_cos();
            e.mem.set_f32(a[1], sin as f32);
            e.mem.set_f32(a[2], cos as f32);
            Ret::default()
        });
        e.register(REFERENCE_CELL, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(CHILD_MARKED_SLOT, |e, a| e.mem.u32(a[0] + 0xb0).into_ret());
        for address in [
            FACE_GEN_SECTION_ENTER,
            FACE_GEN_SECTION_LEAVE,
            KILL_EGT_DATA,
            NPC_SETUP_HEAD,
            QUEUE_NPC_FACE_GEN,
            SET_REFERENCE_ID_ON_SCENEGRAPH,
            VECTOR_CONSTRUCT,
            VECTOR_DESTRUCT,
            BIPED_ANIM_ATTACH_HELMET,
            MODEL_LOADER_QUEUE_HELMET_MODEL,
            MODEL_LOADER_QUEUE_EGM_FILE,
            NPC_GET_FACE_COORD,
            REFERENCE_SET_3D,
            SET_STARTS_DEAD,
            TES_LOAD_REFERENCE,
            CHILD_SET_PART_SLOT,
            HELMET_MAP_SLOT,
            KF_LIST_MAP_SLOT,
            ATTACH_HOOK,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        put_slots(
            &mut e,
            CHILD_VTABLE,
            &[(0x18, CHILD_MARKED_SLOT), (0xe4, CHILD_SET_PART_SLOT)],
        );
        put_slots(&mut e, HELMET_MAP_VTABLE, &[(0x14, HELMET_MAP_SLOT)]);
        put_slots(&mut e, KF_LIST_MAP_VTABLE, &[(0x14, KF_LIST_MAP_SLOT)]);
        put_slots(
            &mut e,
            REFERENCE_TASK_VTABLE,
            &[(0x28, CHECK_FINISHED), (0x3c, ATTACH_HOOK)],
        );
        e
    }

    /// A queued head for the NPC `npc`, in state `state`.
    fn queued_head(e: &mut Engine, npc: Ptr, state: i32) -> Ptr<QueuedHead> {
        let this: Ptr<QueuedHead> = e.new_object();
        e.mem.set_u32(this.addr(), TASK_VTABLE);
        e.mem.set_i32(this.addr() + 0xc, state);
        e.set(this, QueuedHead::pNPC, npc);
        this
    }

    /// An NPC with the form ID `form_id`, the race `race` in the object at
    /// +0x10C and the name string `name` in the name object at +0xD0.
    fn npc(e: &mut Engine, form_id: u32, race: u32, name: u32) -> Ptr {
        let npc = Ptr::new(e.mem.alloc(0x200));
        e.mem.set_u32(npc.addr() + 0xc, form_id);
        e.mem.set_u32(npc.addr() + 0x10c + 4, race);
        e.mem.set_u32(npc.addr() + 0xd0 + 4, name);
        npc
    }

    #[test]
    fn replacement_kf_get_description_names_the_kind() {
        let mut e = helmet_engine();
        let this = queued_replacement_kf(&mut e, Ptr::NULL, 4, false);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_ea00, &args![this, 0x1000u32, 0x40u32]);
        assert!(back.bool());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_DESCRIPTION),
            vec![vec![this.addr(), 0x1000, 0x40, REPLACEMENT_KF_WORD]]
        );
        // The result of the base function is passed on.
        e.register(QUEUED_FILE_ENTRY_GET_DESCRIPTION, |_, _| 0u32.into_ret());
        assert!(!e.call(0x0043_ea00, &args![this, 0x1000u32, 0x40u32]).bool());
    }

    /// A replacement KF list in state `state` with the animation 0x6300 and
    /// the two counters.
    fn replacement_list(
        e: &mut Engine,
        state: i32,
        processing: u32,
        processed: u32,
    ) -> Ptr<QueuedReplacementKFList> {
        let list: Ptr<QueuedReplacementKFList> = e.new_object();
        e.mem.set_u32(list.addr(), TASK_VTABLE);
        e.mem.set_i32(list.addr() + 0xc, state);
        e.set(list, QueuedReplacementKFList::pAnim, Ptr::new(0x6300));
        e.set(
            list,
            QueuedReplacementKFList::iPostProcessingChildCount,
            processing,
        );
        e.set(
            list,
            QueuedReplacementKFList::iPostProcessedChildCount,
            processed,
        );
        list
    }

    #[test]
    fn replacement_kf_list_child_post_processed_tells_the_loader_at_the_last_child() {
        let mut e = helmet_engine();
        let list = replacement_list(&mut e, 4, 2, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_ea30, &args![list]);
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessedChildCount),
            1
        );
        assert!(calls_to(&e, KF_LIST_MAP_SLOT).is_empty());
        // The second child is the last: the map is told about the animation.
        e.call(0x0043_ea30, &args![list]);
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessedChildCount),
            2
        );
        assert_eq!(
            calls_to(&e, KF_LIST_MAP_SLOT),
            vec![vec![KF_LIST_MAP_OBJECT, 0x6300]]
        );
        // A task that is not yet in state 4 only counts.
        let early = replacement_list(&mut e, 3, 1, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_ea30, &args![early]);
        assert_eq!(
            e.get(early, QueuedReplacementKFList::iPostProcessedChildCount),
            1
        );
        assert!(calls_to(&e, KF_LIST_MAP_SLOT).is_empty());
        // Unfinished children: the same.
        let busy = replacement_list(&mut e, 4, 1, 0);
        let children: Ptr<QueuedChildren> = e.new_object();
        e.mem.set_u32(children.addr() + 8, 2);
        e.set(children, QueuedChildren::iNumChildrenFinished, 1);
        e.mem.set_u32(busy.addr() + 0x20, children.addr());
        e.call_log = Some(vec![]);
        e.call(0x0043_ea30, &args![busy]);
        assert!(calls_to(&e, KF_LIST_MAP_SLOT).is_empty());
        // The count wraps like the 32-bit add.
        let wrapping = replacement_list(&mut e, 0, 0, u32::MAX);
        e.call(0x0043_ea30, &args![wrapping]);
        assert_eq!(
            e.get(wrapping, QueuedReplacementKFList::iPostProcessedChildCount),
            0
        );
    }

    #[test]
    fn loader_replacement_list_map_slot_0x14_is_called_with_the_animation() {
        let mut e = helmet_engine();
        e.call_log = Some(vec![]);
        e.call(
            0x0043_ea90,
            &args![Ptr::<()>::new(LOADER), Ptr::<()>::new(0x6300)],
        );
        assert_eq!(
            calls_to(&e, KF_LIST_MAP_SLOT),
            vec![vec![KF_LIST_MAP_OBJECT, 0x6300]]
        );
    }

    #[test]
    fn replacement_kf_list_cancel_runs_the_base_then_tells_the_loader() {
        let mut e = helmet_engine();
        let list = replacement_list(&mut e, 4, 2, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_eac0, &args![list, 5u32, 6u32]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CANCEL),
            vec![vec![list.addr(), 5, 6]]
        );
        assert_eq!(
            calls_to(&e, KF_LIST_MAP_SLOT),
            vec![vec![KF_LIST_MAP_OBJECT, 0x6300]]
        );
        let order = call_order(&e);
        let cancel = order.iter().position(|a| *a == QUEUED_FILE_CANCEL).unwrap();
        let slot = order.iter().position(|a| *a == KF_LIST_MAP_SLOT).unwrap();
        assert!(cancel < slot);
    }

    #[test]
    fn head_constructor_sets_table_npc_and_empty_node_pointers() {
        let mut e = helmet_engine();
        let this: Ptr<QueuedHead> = e.new_object();
        e.mem.set_u32(this.addr() + 0x2c, 0x1111);
        e.mem.set_u32(this.addr() + 0x30, 0x2222);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_eaf0, &args![this, 0x6500u32, 0x37u32]);
        assert_eq!(back.ptr::<QueuedHead>(), this);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CONSTRUCT),
            vec![vec![this.addr(), 0x37]]
        );
        assert_eq!(e.mem.u32(this.addr()), QUEUED_HEAD_VTABLE);
        assert_eq!(e.get(this, QueuedHead::pNPC), Ptr::new(0x6500));
        assert_eq!(
            calls_to(&e, NI_POINTER_CONSTRUCT),
            vec![vec![this.addr() + 0x2c, 0], vec![this.addr() + 0x30, 0]]
        );
        assert_eq!(e.get(this, QueuedHead::spBipedNode), Ptr::NULL);
        assert_eq!(e.get(this, QueuedHead::spSkinnedNode), Ptr::NULL);
    }

    #[test]
    fn head_scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = helmet_engine();
        let this = queued_head(&mut e, Ptr::NULL, 0);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_eb80, &args![this, 0u32]);
        assert_eq!(back.ptr::<QueuedHead>(), this);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        assert_eq!(calls_to(&e, QUEUED_FILE_DESTRUCT).len(), 1);
        e.call(0x0043_eb80, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn head_destructor_releases_the_skinned_node_first() {
        let mut e = helmet_engine();
        let this = queued_head(&mut e, Ptr::NULL, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_ebb0, &args![this]);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_ebb0,
                NI_POINTER_DESTRUCT,
                NI_POINTER_DESTRUCT,
                QUEUED_FILE_DESTRUCT
            ]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_DESTRUCT),
            vec![vec![this.addr() + 0x30], vec![this.addr() + 0x2c]]
        );
        // The virtual table is left as it was.
        assert_eq!(e.mem.u32(this.addr()), TASK_VTABLE);
    }

    #[test]
    fn head_queue_me_hands_the_npc_to_the_face_gen_manager_then_checks() {
        let mut e = helmet_engine();
        let this = queued_head(&mut e, Ptr::new(0x6500), 0);
        e.mem.set_u64(this.addr() + 0x10, 7 << 16);
        e.call_log = Some(vec![]);
        e.call(0x0043_ec20, &args![this]);
        assert_eq!(
            calls_to(&e, QUEUE_NPC_FACE_GEN),
            vec![vec![0x6500, 7, this.addr()]]
        );
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![this.addr()]]);
        let order = call_order(&e);
        let queue = order.iter().position(|a| *a == QUEUE_NPC_FACE_GEN).unwrap();
        let check = order.iter().position(|a| *a == CHECK_FINISHED).unwrap();
        assert!(queue < check);
    }

    #[test]
    fn head_check_finished_queues_a_new_task_or_checks_the_file() {
        let mut e = helmet_engine();
        // No children: a task still in state 0 goes to the task queue.
        let fresh = queued_head(&mut e, Ptr::NULL, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_ec60, &args![fresh]);
        assert_eq!(
            calls_to(&e, QUEUE_ADD),
            vec![vec![PART_IO_MANAGER, fresh.addr()]]
        );
        assert!(calls_to(&e, QUEUED_FILE_CHECK_FINISHED).is_empty());
        // Any other state goes through `QueuedFile::CheckFinished`.
        let running = queued_head(&mut e, Ptr::NULL, 3);
        e.call_log = Some(vec![]);
        e.call(0x0043_ec60, &args![running]);
        assert!(calls_to(&e, QUEUE_ADD).is_empty());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CHECK_FINISHED),
            vec![vec![running.addr()]]
        );
        // Unfinished children: nothing.
        let busy = queued_head(&mut e, Ptr::NULL, 0);
        let children: Ptr<QueuedChildren> = e.new_object();
        e.mem.set_u32(children.addr() + 8, 2);
        e.set(children, QueuedChildren::iNumChildrenFinished, 1);
        e.mem.set_u32(busy.addr() + 0x20, children.addr());
        e.call_log = Some(vec![]);
        e.call(0x0043_ec60, &args![busy]);
        assert!(calls_to(&e, QUEUE_ADD).is_empty());
        assert!(calls_to(&e, QUEUED_FILE_CHECK_FINISHED).is_empty());
    }

    #[test]
    fn head_run_builds_the_nodes_inside_the_face_gen_section() {
        let mut e = helmet_engine();
        let other = npc(&mut e, 0x14, 0x7ace, 0);
        let this = queued_head(&mut e, other, 3);
        e.call_log = Some(vec![]);
        e.call(0x0043_ecb0, &args![this]);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_ecb0,
                FACE_GEN_SECTION_ENTER,
                MEMORY_CONTEXT_ENTER,
                NPC_INIT_HEAD,
                NI_POINTER_ASSIGN,
                NI_POINTER_ASSIGN,
                FIELD_AT_C,
                FACE_GEN_SECTION_LEAVE,
                MEMORY_CONTEXT_LEAVE
            ]
        );
        let enter = calls_to(&e, MEMORY_CONTEXT_ENTER)[0].clone();
        assert_eq!(
            &enter[1..],
            &[HEAD_CONTEXT, 1, MODEL_LOADER_SOURCE, HEAD_RUN_SOURCE_LINE]
        );
        let init = &calls_to(&e, NPC_INIT_HEAD)[0];
        assert_eq!(init[0], other.addr());
        assert_eq!(
            calls_to(&e, NI_POINTER_ASSIGN),
            vec![
                vec![this.addr() + 0x2c, BIPED_NODE],
                vec![this.addr() + 0x30, SKINNED_NODE]
            ]
        );
        assert_eq!(e.get(this, QueuedHead::spBipedNode), Ptr::new(BIPED_NODE));
        assert_eq!(
            e.get(this, QueuedHead::spSkinnedNode),
            Ptr::new(SKINNED_NODE)
        );
        // The scope guard is the same block in the enter and the leave.
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE), vec![vec![enter[0]]]);
        // The NPC of form ID 7: the race's EGT data is killed.
        let player_npc = npc(&mut e, 7, 0x7ace, 0);
        let this = queued_head(&mut e, player_npc, 3);
        e.call_log = Some(vec![]);
        e.call(0x0043_ecb0, &args![this]);
        assert_eq!(
            calls_to(&e, FIELD_AT_4),
            vec![vec![player_npc.addr() + 0x10c]]
        );
        assert_eq!(calls_to(&e, KILL_EGT_DATA), vec![vec![0x7ace]]);
        let order = call_order(&e);
        let kill = order.iter().position(|a| *a == KILL_EGT_DATA).unwrap();
        let leave = order
            .iter()
            .position(|a| *a == FACE_GEN_SECTION_LEAVE)
            .unwrap();
        assert!(kill < leave);
    }

    /// A node with a dword at +0xE8.
    fn node_with_reference_slot(e: &mut Engine) -> Ptr {
        Ptr::new(e.mem.alloc(0x100))
    }

    /// A reference with the base form 0x8000 and the form ID 0x1234.
    fn reference(e: &mut Engine) -> Ptr {
        let reference = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(reference.addr() + 0xc, 0x1234);
        e.mem.set_u32(reference.addr() + 0x20, 0x8000);
        reference
    }

    #[test]
    fn head_setup_gives_the_nodes_the_reference() {
        let mut e = helmet_engine();
        let this = queued_head(&mut e, Ptr::NULL, 3);
        let biped = node_with_reference_slot(&mut e);
        let skinned = node_with_reference_slot(&mut e);
        e.set(this, QueuedHead::spBipedNode, biped);
        e.set(this, QueuedHead::spSkinnedNode, skinned);
        let subject = reference(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0043_ed90, &args![this, subject, 9u32]);
        assert_eq!(
            calls_to(&e, NPC_SETUP_HEAD),
            vec![vec![
                0x8000,
                subject.addr(),
                9,
                biped.addr(),
                skinned.addr()
            ]]
        );
        assert_eq!(
            calls_to(&e, SET_REFERENCE_ID_ON_SCENEGRAPH),
            vec![vec![biped.addr(), 0x1234], vec![skinned.addr(), 0x1234]]
        );
        assert_eq!(e.mem.u32(biped.addr() + 0xe8), subject.addr());
        assert_eq!(e.mem.u32(skinned.addr() + 0xe8), subject.addr());
        let enter = calls_to(&e, MEMORY_CONTEXT_ENTER)[0].clone();
        assert_eq!(
            &enter[1..],
            &[HEAD_CONTEXT, 1, MODEL_LOADER_SOURCE, HEAD_SETUP_SOURCE_LINE]
        );
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE), vec![vec![enter[0]]]);
        // The player reference: no form ID on the scene graph, same stores.
        let biped2 = node_with_reference_slot(&mut e);
        let skinned2 = node_with_reference_slot(&mut e);
        let this2 = queued_head(&mut e, Ptr::NULL, 3);
        e.set(this2, QueuedHead::spBipedNode, biped2);
        e.set(this2, QueuedHead::spSkinnedNode, skinned2);
        e.set_global(PLAYER_CHARACTER, subject.addr());
        e.call_log = Some(vec![]);
        e.call(0x0043_ed90, &args![this2, subject, 9u32]);
        assert!(calls_to(&e, SET_REFERENCE_ID_ON_SCENEGRAPH).is_empty());
        assert_eq!(e.mem.u32(biped2.addr() + 0xe8), subject.addr());
        assert_eq!(e.mem.u32(skinned2.addr() + 0xe8), subject.addr());
        // No skinned node: only the biped one is touched.
        e.set_global(PLAYER_CHARACTER, PLAYER);
        let biped3 = node_with_reference_slot(&mut e);
        let this3 = queued_head(&mut e, Ptr::NULL, 3);
        e.set(this3, QueuedHead::spBipedNode, biped3);
        e.call_log = Some(vec![]);
        e.call(0x0043_ed90, &args![this3, subject, 9u32]);
        assert_eq!(
            calls_to(&e, SET_REFERENCE_ID_ON_SCENEGRAPH),
            vec![vec![biped3.addr(), 0x1234]]
        );
        assert_eq!(e.mem.u32(biped3.addr() + 0xe8), subject.addr());
    }

    #[test]
    fn head_get_description_names_the_npc() {
        let mut e = helmet_engine();
        let name = text(&mut e, "Doc Mitchell");
        let doc = npc(&mut e, 0x14, 0, name);
        let this = queued_head(&mut e, doc, 0);
        let buffer = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_eed0, &args![this, buffer, 0x80u32]);
        assert!(back.bool());
        assert_eq!(
            calls_to(&e, NAME_OBJECT_STRING),
            vec![vec![doc.addr() + 0xd0]]
        );
        assert_eq!(
            calls_to(&e, FORMAT_STRING),
            vec![vec![buffer, 0x80, QUEUED_HEAD_FORMAT, name]]
        );
        assert_eq!(e.mem.cstr(buffer), b"Queued head for NPC Doc Mitchell");
    }

    /// A biped animation for `reference` whose slots `used` hold model data
    /// (`0x9000 + slot`).
    fn biped_anim(e: &mut Engine, reference: Ptr, used: &[u32]) -> Ptr {
        let biped = Ptr::new(e.mem.alloc(0x300));
        e.mem.set_u32(biped.addr() + 0x2b0, reference.addr());
        for slot in used {
            e.mem
                .set_u32(biped.addr() + 0x30 + 0x10 * slot, 0x9000 + slot);
        }
        biped
    }

    /// A queued helmet task for `biped` and `reference` (in state `state`,
    /// flags `flags`) whose table makes slot 0x28 `CheckFinished`.
    fn queued_helmet(
        e: &mut Engine,
        biped: Ptr,
        reference: Ptr,
        state: i32,
        flags: u8,
    ) -> Ptr<QueuedHelmet> {
        let this: Ptr<QueuedHelmet> = e.new_object();
        e.mem.set_u32(this.addr(), TASK_VTABLE);
        e.mem.set_i32(this.addr() + 0xc, state);
        e.set(this, QueuedHelmet::pBipedAnim, biped);
        e.set(this, QueuedHelmet::pRef, reference);
        e.set(this, QueuedHelmet::cFlags, flags);
        this
    }

    #[test]
    fn helmet_constructor_builds_the_three_arrays() {
        let mut e = helmet_engine();
        let subject = reference(&mut e);
        let biped = biped_anim(&mut e, subject, &[]);
        let this: Ptr<QueuedHelmet> = e.new_object();
        e.mem.set_u8(this.addr() + 0x120, 0xff);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_ef10, &args![this, biped, 0x31u32, 0xdead_beefu32]);
        assert_eq!(back.ptr::<QueuedHelmet>(), this);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CONSTRUCT),
            vec![vec![this.addr(), 0x31]]
        );
        assert_eq!(e.mem.u32(this.addr()), QUEUED_HELMET_VTABLE);
        assert_eq!(e.get(this, QueuedHelmet::pBipedAnim), biped);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![
                vec![
                    this.addr() + 0x2c,
                    4,
                    0x14,
                    HELMET_MODEL_POINTER_CONSTRUCT,
                    TASK_POINTER_DESTRUCT
                ],
                vec![
                    this.addr() + 0x7c,
                    4,
                    0x14,
                    NI_POINTER_DEFAULT_CONSTRUCT,
                    NI_POINTER_DESTRUCT
                ],
                vec![
                    this.addr() + 0xcc,
                    4,
                    0x14,
                    NI_POINTER_DEFAULT_CONSTRUCT,
                    NI_POINTER_DESTRUCT
                ],
            ]
        );
        assert_eq!(e.get(this, QueuedHelmet::cFlags), 0);
        // The reference comes from the biped animation.
        assert_eq!(e.get(this, QueuedHelmet::pRef), subject);
    }

    #[test]
    fn helmet_post_process_attaches() {
        let mut e = helmet_engine();
        let subject = reference(&mut e);
        let biped = biped_anim(&mut e, subject, &[]);
        let this = queued_helmet(&mut e, biped, subject, 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_eff0, &args![this]);
        // With no cloned nodes only the loader's helmet map is told.
        assert!(calls_to(&e, BIPED_ANIM_ATTACH_HELMET).is_empty());
        assert_eq!(
            calls_to(&e, HELMET_MAP_SLOT),
            vec![vec![HELMET_MAP_OBJECT, subject.addr()]]
        );
    }

    #[test]
    fn biped_reference_getter_reads_offset_0x2b0() {
        let mut e = helmet_engine();
        let block = Ptr::<()>::new(e.mem.alloc(0x300));
        e.mem.set_u32(block.addr() + 0x2b0, 0x4321);
        assert_eq!(e.call(0x0043_f010, &args![block]).u32(), 0x4321);
    }

    #[test]
    fn helmet_scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = helmet_engine();
        let this = queued_helmet(&mut e, Ptr::NULL, Ptr::NULL, 0, 0);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_f030, &args![this, 0u32]);
        assert_eq!(back.ptr::<QueuedHelmet>(), this);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        assert_eq!(calls_to(&e, QUEUED_FILE_DESTRUCT).len(), 1);
        e.call(0x0043_f030, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn helmet_model_pointer_constructor_is_the_task_pointer_with_null() {
        let mut e = helmet_engine();
        let slot = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(slot.addr(), 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0043_f060, &args![slot]);
        assert_eq!(
            calls_to(&e, TASK_POINTER_CONSTRUCT),
            vec![vec![slot.addr(), 0]]
        );
        assert_eq!(e.mem.u32(slot.addr()), 0);
    }

    #[test]
    fn helmet_destructor_destroys_the_arrays_last_first() {
        let mut e = helmet_engine();
        let this = queued_helmet(&mut e, Ptr::NULL, Ptr::NULL, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_f080, &args![this]);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![
                vec![this.addr() + 0xcc, 4, 0x14, NI_POINTER_DESTRUCT],
                vec![this.addr() + 0x7c, 4, 0x14, NI_POINTER_DESTRUCT],
                vec![this.addr() + 0x2c, 4, 0x14, TASK_POINTER_DESTRUCT],
            ]
        );
        let order = call_order(&e);
        assert_eq!(*order.last().unwrap(), QUEUED_FILE_DESTRUCT);
        assert_eq!(e.mem.u32(this.addr()), TASK_VTABLE);
    }

    #[test]
    fn helmet_queue_me_queues_each_slot_in_use() {
        let mut e = helmet_engine();
        let subject = reference(&mut e);
        let biped = biped_anim(&mut e, subject, &[2, 5]);
        let this = queued_helmet(&mut e, biped, subject, 0, 0);
        e.mem.set_u64(this.addr() + 0x10, 6 << 16);
        e.call_log = Some(vec![]);
        e.call(0x0043_f120, &args![this]);
        assert_eq!(
            calls_to(&e, BIPED_ANIM_SLOT_IN_USE).len(),
            0x14,
            "every slot is asked"
        );
        // Model data is the dword at +4 of the slot's entry.
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_HELMET_MODEL),
            vec![
                vec![
                    LOADER,
                    0x9002,
                    this.addr() + 0x2c + 8,
                    6,
                    this.addr(),
                    3,
                    1,
                    1,
                    1,
                    0
                ],
                vec![
                    LOADER,
                    0x9005,
                    this.addr() + 0x2c + 20,
                    6,
                    this.addr(),
                    3,
                    1,
                    1,
                    1,
                    0
                ],
            ]
        );
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_EGM_FILE),
            vec![
                vec![LOADER, 0x9002, 6, this.addr(), 0xffff_ffff],
                vec![LOADER, 0x9005, 6, this.addr(), 0xffff_ffff],
            ]
        );
        // Flag bit 1 is set before `CheckFinished` is called.
        assert_eq!(e.get(this, QueuedHelmet::cFlags), 1);
        let order = call_order(&e);
        let flag = order.iter().position(|a| *a == SET_FLAG_BIT_1).unwrap();
        let check = order.iter().position(|a| *a == CHECK_FINISHED).unwrap();
        assert!(flag < check);
        assert_eq!(calls_to(&e, CHECK_FINISHED), vec![vec![this.addr()]]);
    }

    #[test]
    fn helmet_flag_setter_sets_and_clears_bit_1() {
        let mut e = helmet_engine();
        let this = queued_helmet(&mut e, Ptr::NULL, Ptr::NULL, 0, 0xf0);
        e.call(0x0043_f1f0, &args![this, 1u32]);
        assert_eq!(e.get(this, QueuedHelmet::cFlags), 0xf1);
        e.call(0x0043_f1f0, &args![this, 0u32]);
        assert_eq!(e.get(this, QueuedHelmet::cFlags), 0xf0);
    }

    #[test]
    fn biped_slot_entry_address_is_0x2c_plus_0x10_per_slot() {
        let mut e = helmet_engine();
        let block = Ptr::<()>::new(0x5000);
        assert_eq!(e.call(0x0043_f220, &args![block, 0u32]).u32(), 0x502c);
        assert_eq!(e.call(0x0043_f220, &args![block, 3u32]).u32(), 0x505c);
    }

    #[test]
    fn helmet_run_loads_the_face_gen_model_of_each_slot_in_use() {
        let mut e = helmet_engine();
        let subject = reference(&mut e);
        let biped = biped_anim(&mut e, subject, &[1, 4]);
        e.mem.set_u32(FACE_ANSWERS + 4, 0x5001);
        e.mem.set_u32(FACE_ANSWERS + 16, 0x5004);
        let this = queued_helmet(&mut e, biped, subject, 3, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_f240, &args![this]);
        assert_eq!(
            calls_to(&e, BIPED_ANIM_LOAD_FACE_GEN_MODEL),
            vec![vec![biped.addr(), 1], vec![biped.addr(), 4]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x7c + 4), 0x5001);
        assert_eq!(e.mem.u32(this.addr() + 0x7c + 16), 0x5004);
        assert_eq!(e.mem.u32(this.addr() + 0x7c), 0);
        // A cancelled task (state 6) loads nothing.
        let cancelled = queued_helmet(&mut e, biped, subject, 6, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_f240, &args![cancelled]);
        assert!(calls_to(&e, BIPED_ANIM_LOAD_FACE_GEN_MODEL).is_empty());
        assert_eq!(e.mem.u32(cancelled.addr() + 0x7c + 4), 0);
    }

    #[test]
    fn helmet_finish_posts_a_parentless_task_then_checks() {
        let mut e = helmet_engine();
        let this = queued_helmet(&mut e, Ptr::NULL, Ptr::NULL, 4, 0);
        e.mem.set_u64(this.addr() + 0x10, 2 << 16);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2b0, &args![this]);
        let holder = calls_to(&e, TASK_POINTER_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, QUEUE_TABLE_ADD), vec![vec![0x7100, 2, holder]]);
        let order = call_order(&e);
        let post = order
            .iter()
            .position(|a| *a == TASK_POINTER_CONSTRUCT)
            .unwrap();
        let check = order.iter().position(|a| *a == CHECK_FINISHED).unwrap();
        assert!(post < check);
        // With a parent: only the check.
        e.mem.set_u32(this.addr() + 0x1c, 0x6600);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2b0, &args![this]);
        assert!(calls_to(&e, QUEUE_TABLE_ADD).is_empty());
        assert_eq!(calls_to(&e, CHECK_FINISHED).len(), 1);
        // State 6: the same.
        let cancelled = queued_helmet(&mut e, Ptr::NULL, Ptr::NULL, 6, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2b0, &args![cancelled]);
        assert!(calls_to(&e, QUEUE_TABLE_ADD).is_empty());
        assert_eq!(calls_to(&e, CHECK_FINISHED).len(), 1);
    }

    /// A child node of a cloned helmet: its virtual function 0x18 answers
    /// `answer`, the `NiPointer` at +0xBC is `marker`, the part at +0xB8 is
    /// `part`, and the local rotation is the identity.
    fn helmet_child(e: &mut Engine, answer: u32, marker: u32, part: u32) -> Ptr {
        let child = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(child.addr(), CHILD_VTABLE);
        e.mem.set_u32(child.addr() + 0xb0, answer);
        e.mem.set_u32(child.addr() + 0xb8, part);
        e.mem.set_u32(child.addr() + 0xbc, marker);
        for (i, value) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(child.addr() + 0x34 + 4 * i as u32, *value);
        }
        child
    }

    /// A node whose child array (at +0x9C) holds `children`.
    fn node_with_children(e: &mut Engine, children: &[Ptr]) -> Ptr {
        let node = Ptr::new(e.mem.alloc(0x100));
        let base = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, child.addr());
        }
        e.mem.set_u32(node.addr() + 0x9c + 4, base);
        e.mem
            .set_u16(node.addr() + 0x9c + 0xa, children.len() as u16);
        node
    }

    /// The rotation matrix of a child, as floats.
    fn child_rotation(e: &Engine, child: Ptr) -> Vec<f32> {
        (0..9)
            .map(|i| e.mem.f32(child.addr() + 0x34 + 4 * i))
            .collect()
    }

    /// Queues a loaded model into `slot` of `this`: the queued model has a
    /// model whose 3D object is `node_3d`.
    fn give_loaded_model(e: &mut Engine, this: Ptr<QueuedHelmet>, slot: u32, node_3d: u32) {
        let queued = e.mem.alloc(0x80);
        let model = e.mem.alloc(0x20);
        e.mem.set_u32(model + 0xc, node_3d);
        e.mem.set_u32(queued + 0x30, model);
        e.mem.set_u32(this.addr() + 0x2c + 4 * slot, queued);
    }

    /// An NPC reference (base form of type 0x2a) and its helmet task in
    /// state 4 with bit 1 set, for a biped with `used` slots.
    fn helmet_scene(e: &mut Engine, used: &[u32]) -> (Ptr<QueuedHelmet>, Ptr, Ptr) {
        let form = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_u8(form.addr() + 4, 0x2a);
        let subject = reference(e);
        e.mem.set_u32(subject.addr() + 0x20, form.addr());
        let biped = biped_anim(e, subject, used);
        (queued_helmet(e, biped, subject, 4, 1), form, subject)
    }

    #[test]
    fn helmet_check_finished_without_bit_1_is_the_file_check() {
        let mut e = helmet_engine();
        let this = queued_helmet(&mut e, Ptr::NULL, Ptr::NULL, 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert_eq!(
            call_order(&e),
            vec![0x0043_f2f0, TEST_FLAG_BIT_1, QUEUED_FILE_CHECK_FINISHED]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CHECK_FINISHED),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn helmet_check_finished_queues_a_new_task() {
        let mut e = helmet_engine();
        let this = queued_helmet(&mut e, Ptr::NULL, Ptr::NULL, 0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert_eq!(
            calls_to(&e, QUEUE_ADD),
            vec![vec![PART_IO_MANAGER, this.addr()]]
        );
        assert!(calls_to(&e, MEMORY_CONTEXT_ENTER).is_empty());
        assert!(calls_to(&e, QUEUED_FILE_CHECK_FINISHED).is_empty());
    }

    #[test]
    fn helmet_check_finished_does_not_clone_for_the_player_or_early() {
        let mut e = helmet_engine();
        let (this, _, subject) = helmet_scene(&mut e, &[1]);
        // Unfinished children: the task is not queued but the context is
        // entered, and nothing is cloned.
        let children: Ptr<QueuedChildren> = e.new_object();
        e.mem.set_u32(children.addr() + 8, 2);
        e.mem.set_u32(this.addr() + 0x20, children.addr());
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert!(calls_to(&e, QUEUE_ADD).is_empty());
        let enter = calls_to(&e, MEMORY_CONTEXT_ENTER)[0].clone();
        assert_eq!(
            &enter[1..],
            &[
                HELMET_CONTEXT,
                1,
                MODEL_LOADER_SOURCE,
                HELMET_CHECK_FINISHED_SOURCE_LINE
            ]
        );
        assert!(calls_to(&e, BIPED_ANIM_CLONE_HELMET).is_empty());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CHECK_FINISHED),
            vec![vec![this.addr()]]
        );
        // The spare `NiPointer` is built first and destroyed last, and the
        // context is left with the same guard.
        let spare = calls_to(&e, NI_POINTER_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, NI_POINTER_DESTRUCT), vec![vec![spare]]);
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE), vec![vec![enter[0]]]);
        // The player's helmet is never cloned.
        e.mem.set_u32(this.addr() + 0x20, 0);
        give_loaded_model(&mut e, this, 1, 0x3d00);
        e.set_global(PLAYER_CHARACTER, subject.addr());
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert!(calls_to(&e, BIPED_ANIM_CLONE_HELMET).is_empty());
        // State 3 (before 4) and state 6: the same.
        e.set_global(PLAYER_CHARACTER, PLAYER);
        for state in [3, 6] {
            e.mem.set_i32(this.addr() + 0xc, state);
            e.call_log = Some(vec![]);
            e.call(0x0043_f2f0, &args![this]);
            assert!(calls_to(&e, BIPED_ANIM_CLONE_HELMET).is_empty());
        }
    }

    #[test]
    fn helmet_check_finished_clones_and_turns_the_child() {
        let mut e = helmet_engine();
        let (this, form, _) = helmet_scene(&mut e, &[1]);
        let biped = e.get(this, QueuedHelmet::pBipedAnim);
        give_loaded_model(&mut e, this, 1, 0x3d00);
        let child = helmet_child(&mut e, 0, 0, 0x1111);
        let clone = node_with_children(&mut e, &[child]);
        e.mem.set_u32(CLONE_ANSWERS + 4, clone.addr());
        e.mem.set_u32(FACE_ANSWERS + 4, 0x5001);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        // The clone is made from the model's 3D, and kept in the array.
        assert_eq!(
            calls_to(&e, BIPED_ANIM_CLONE_HELMET),
            vec![vec![biped.addr(), 0, 0x3d00, 1]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0xcc + 4), clone.addr());
        // The face generation model was missing: it is loaded.
        assert_eq!(
            calls_to(&e, BIPED_ANIM_LOAD_FACE_GEN_MODEL),
            vec![vec![biped.addr(), 1]]
        );
        // The child's part is deep-copied and given to the child.
        let spare = calls_to(&e, NI_POINTER_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, CREATE_DEEP_COPY), vec![vec![0x1111, spare]]);
        assert_eq!(
            calls_to(&e, CHILD_SET_PART_SLOT),
            vec![vec![child.addr(), 0x7d00_1111]]
        );
        // The NPC's face coordinates go into four 0x20-byte elements.
        let coords = calls_to(&e, VECTOR_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![
                coords,
                0x20,
                4,
                FACE_COORD_CONSTRUCT,
                FACE_COORD_DESTRUCT
            ]]
        );
        assert_eq!(
            calls_to(&e, NPC_GET_FACE_COORD),
            vec![vec![form.addr(), coords]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![vec![coords, 0x20, 4, FACE_COORD_DESTRUCT]]
        );
        // The setting byte is clear: the child is turned by -pi/2 around Y
        // without asking the face generation model.
        assert!(calls_to(&e, FACE_GEN_MODEL_APPLY_COORDS).is_empty());
        let (sin, cos) = (-std::f32::consts::FRAC_PI_2 as f64).sin_cos();
        let expected = [
            cos as f32,
            0.0,
            -sin as f32,
            0.0,
            1.0,
            0.0,
            sin as f32,
            0.0,
            cos as f32,
        ];
        assert_eq!(child_rotation(&e, child), expected);
        // The context is left last, after the file check.
        let order = call_order(&e);
        assert_eq!(*order.last().unwrap(), MEMORY_CONTEXT_LEAVE);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CHECK_FINISHED),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn helmet_check_finished_asks_the_face_gen_model_when_the_setting_is_set() {
        let mut e = helmet_engine();
        let (this, _, _) = helmet_scene(&mut e, &[1]);
        give_loaded_model(&mut e, this, 1, 0x3d00);
        let child = helmet_child(&mut e, 0, 0, 0x1111);
        let clone = node_with_children(&mut e, &[child]);
        e.mem.set_u32(CLONE_ANSWERS + 4, clone.addr());
        // The face generation model is there already.
        e.mem.set_u32(this.addr() + 0x7c + 4, 0x5001);
        e.mem.set_u8(HELMET_ROTATION_SETTING + 4, 1);
        // It says no: the child keeps its rotation.
        e.mem.set_u32(APPLY_ANSWER, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert!(calls_to(&e, BIPED_ANIM_LOAD_FACE_GEN_MODEL).is_empty());
        let coords = calls_to(&e, VECTOR_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, FACE_GEN_MODEL_APPLY_COORDS),
            vec![vec![0x5001, coords, child.addr(), 0]]
        );
        assert_eq!(
            child_rotation(&e, child),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
        // It says yes: the child is turned.
        e.mem.set_u32(APPLY_ANSWER, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        let turned = child_rotation(&e, child);
        assert_eq!(turned[4], 1.0);
        assert_eq!(turned[2], 1.0, "-sin(-pi/2) is 1");
        assert_eq!(turned[6], -1.0);
    }

    #[test]
    fn helmet_check_finished_skips_what_cannot_be_cloned() {
        let mut e = helmet_engine();
        let (this, form, _) = helmet_scene(&mut e, &[1, 2, 3, 4]);
        // Slot 1 has no queued model; slot 2 a model without a 3D; slot 3 a
        // clone that is null; slot 4 has no queued model either.
        give_loaded_model(&mut e, this, 2, 0);
        give_loaded_model(&mut e, this, 3, 0x3d03);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert_eq!(
            calls_to(&e, BIPED_ANIM_CLONE_HELMET).len(),
            1,
            "only slot 3 reaches the clone call"
        );
        assert!(calls_to(&e, NPC_GET_FACE_COORD).is_empty());
        // A clone with a marked child (answers non-zero and has a pointer at
        // +0xBC) is left alone: no face model, no turn.
        let marked = helmet_child(&mut e, 1, 0x6666, 0x1111);
        let clone = node_with_children(&mut e, &[marked]);
        e.mem.set_u32(CLONE_ANSWERS + 12, clone.addr());
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert!(calls_to(&e, BIPED_ANIM_LOAD_FACE_GEN_MODEL).is_empty());
        assert!(calls_to(&e, NPC_GET_FACE_COORD).is_empty());
        // A child that answers non-zero but has no pointer is not marked.
        let unmarked = helmet_child(&mut e, 1, 0, 0x1111);
        let clone = node_with_children(&mut e, &[unmarked]);
        e.mem.set_u32(CLONE_ANSWERS + 12, clone.addr());
        e.mem.set_u32(FACE_ANSWERS + 12, 0x5003);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert_eq!(calls_to(&e, NPC_GET_FACE_COORD).len(), 1);
        // A face generation model that cannot be loaded: nothing is turned.
        let child = helmet_child(&mut e, 0, 0, 0x1111);
        let clone = node_with_children(&mut e, &[child]);
        e.mem.set_u32(CLONE_ANSWERS + 12, clone.addr());
        e.mem.set_u32(FACE_ANSWERS + 12, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert!(calls_to(&e, NPC_GET_FACE_COORD).is_empty());
        // A child without a part, or a base form that is not an NPC (type
        // 0x2a): skipped.
        let no_part = helmet_child(&mut e, 0, 0, 0);
        let clone = node_with_children(&mut e, &[no_part]);
        e.mem.set_u32(CLONE_ANSWERS + 12, clone.addr());
        e.mem.set_u32(FACE_ANSWERS + 12, 0x5003);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert!(calls_to(&e, NPC_GET_FACE_COORD).is_empty());
        let child = helmet_child(&mut e, 0, 0, 0x1111);
        let clone = node_with_children(&mut e, &[child]);
        e.mem.set_u32(CLONE_ANSWERS + 12, clone.addr());
        e.mem.set_u8(form.addr() + 4, 0x2b);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert!(calls_to(&e, NPC_GET_FACE_COORD).is_empty());
        // A biped animation without a reference: skipped too.
        e.mem.set_u8(form.addr() + 4, 0x2a);
        let biped = e.get(this, QueuedHelmet::pBipedAnim);
        e.mem.set_u32(biped.addr() + 0x2b0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_f2f0, &args![this]);
        assert!(calls_to(&e, NPC_GET_FACE_COORD).is_empty());
    }

    #[test]
    fn matrix_y_rotation_fills_cos_sin() {
        let mut e = helmet_engine();
        let matrix = Ptr::<()>::new(e.mem.alloc(36));
        for i in 0..9 {
            e.mem.set_f32(matrix.addr() + 4 * i, 7.0);
        }
        e.call_log = Some(vec![]);
        e.call(0x0043_f850, &args![matrix, 0.5f32]);
        let call = calls_to(&e, SIN_COS)[0].clone();
        assert_eq!(call[0], 0.5f32.to_bits());
        let (sin, cos) = 0.5f64.sin_cos();
        let (sin, cos) = (sin as f32, cos as f32);
        let values: Vec<f32> = (0..9).map(|i| e.mem.f32(matrix.addr() + 4 * i)).collect();
        assert_eq!(values, vec![cos, 0.0, -sin, 0.0, 1.0, 0.0, sin, 0.0, cos]);
        // The sine goes to the first output, the cosine to the second.
        assert_eq!(call[2], call[1] + 4);
    }

    fn write_matrix(e: &mut Engine, values: [f32; 9]) -> Ptr {
        let matrix = Ptr::new(e.mem.alloc(36));
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(matrix.addr() + 4 * i as u32, *value);
        }
        matrix
    }

    fn read_matrix(e: &Engine, matrix: Ptr) -> Vec<f32> {
        (0..9).map(|i| e.mem.f32(matrix.addr() + 4 * i)).collect()
    }

    #[test]
    fn matrix_multiply_is_the_row_by_column_product() {
        let mut e = helmet_engine();
        let left = write_matrix(&mut e, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let right = write_matrix(&mut e, [9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        let out = write_matrix(&mut e, [0.0; 9]);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_f8d0, &args![left, out, right]);
        assert_eq!(back.ptr::<()>(), out);
        assert_eq!(
            read_matrix(&e, out),
            vec![30.0, 24.0, 18.0, 84.0, 69.0, 54.0, 138.0, 114.0, 90.0]
        );
        // The inputs are untouched; the constructor is called on a stack
        // block, not on the output.
        assert_eq!(
            read_matrix(&e, left),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );
        assert_eq!(calls_to(&e, MATRIX_CONSTRUCT).len(), 1);
        assert_ne!(calls_to(&e, MATRIX_CONSTRUCT)[0][0], out.addr());
        // The output may be the left operand.
        e.call(0x0043_f8d0, &args![left, left, right]);
        assert_eq!(
            read_matrix(&e, left),
            vec![30.0, 24.0, 18.0, 84.0, 69.0, 54.0, 138.0, 114.0, 90.0]
        );
    }

    #[test]
    fn matrix_multiply_rounds_each_sum_once() {
        let mut e = helmet_engine();
        // 1 + 2^-30 is not a float, but the sum is rounded to a float only at
        // the store: (1 + 2^-30) + (-1) is 2^-30, where rounding each partial
        // sum to float would give 0.
        let tiny = 2.0f32.powi(-30);
        let left = write_matrix(&mut e, [1.0, tiny, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let right = write_matrix(&mut e, [1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        let out = write_matrix(&mut e, [0.0; 9]);
        e.call(0x0043_f8d0, &args![left, out, right]);
        // Row 0, column 0: 1 * 1 + tiny * 1 + (-1) * 1.
        assert_eq!(e.mem.f32(out.addr()), tiny);
    }

    #[test]
    fn local_rotation_copy_writes_nine_dwords_at_0x34() {
        let mut e = helmet_engine();
        let object = Ptr::<()>::new(e.mem.alloc(0x80));
        e.mem.set_u32(object.addr() + 0x34 + 36, 0xaaaa);
        let matrix = write_matrix(&mut e, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        e.call(0x0043_fa80, &args![object, matrix]);
        let copied: Vec<f32> = (0..9)
            .map(|i| e.mem.f32(object.addr() + 0x34 + 4 * i))
            .collect();
        assert_eq!(copied, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        // The word after the matrix is not touched.
        assert_eq!(e.mem.u32(object.addr() + 0x34 + 36), 0xaaaa);
    }

    #[test]
    fn helmet_bit_1_test_reads_the_flags() {
        let mut e = helmet_engine();
        let on = queued_helmet(&mut e, Ptr::NULL, Ptr::NULL, 0, 0x03);
        let off = queued_helmet(&mut e, Ptr::NULL, Ptr::NULL, 0, 0x02);
        assert!(e.call(0x0043_fab0, &args![on]).bool());
        assert!(!e.call(0x0043_fab0, &args![off]).bool());
    }

    #[test]
    fn node_pointer_at_0xbc_is_dereferenced() {
        let mut e = helmet_engine();
        let block = Ptr::<()>::new(e.mem.alloc(0x100));
        e.mem.set_u32(block.addr() + 0xbc, 0x7777);
        assert_eq!(e.call(0x0043_fad0, &args![block]).u32(), 0x7777);
    }

    #[test]
    fn setting_byte_is_read_through_the_pointer_getter() {
        let mut e = helmet_engine();
        e.call_log = Some(vec![]);
        e.mem.set_u8(HELMET_ROTATION_SETTING + 4, 0);
        assert_eq!(e.call(0x0043_faf0, &args![]).u8(), 0);
        e.mem.set_u8(HELMET_ROTATION_SETTING + 4, 1);
        assert_eq!(e.call(0x0043_faf0, &args![]).u8(), 1);
        assert_eq!(
            calls_to(&e, SETTING_BYTE_POINTER),
            vec![vec![HELMET_ROTATION_SETTING], vec![HELMET_ROTATION_SETTING]]
        );
    }

    #[test]
    fn helmet_cancel_runs_the_base_then_tells_the_loader() {
        let mut e = helmet_engine();
        let subject = reference(&mut e);
        let this = queued_helmet(&mut e, Ptr::NULL, subject, 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0043_fb10, &args![this, 5u32, 6u32]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CANCEL),
            vec![vec![this.addr(), 5, 6]]
        );
        assert_eq!(
            calls_to(&e, HELMET_MAP_SLOT),
            vec![vec![HELMET_MAP_OBJECT, subject.addr()]]
        );
        let order = call_order(&e);
        let cancel = order.iter().position(|a| *a == QUEUED_FILE_CANCEL).unwrap();
        let slot = order.iter().position(|a| *a == HELMET_MAP_SLOT).unwrap();
        assert!(cancel < slot);
    }

    #[test]
    fn loader_helmet_map_slot_0x14_is_called_with_the_reference() {
        let mut e = helmet_engine();
        e.call_log = Some(vec![]);
        e.call(
            0x0043_fb50,
            &args![Ptr::<()>::new(LOADER), Ptr::<()>::new(0x6300)],
        );
        assert_eq!(
            calls_to(&e, HELMET_MAP_SLOT),
            vec![vec![HELMET_MAP_OBJECT, 0x6300]]
        );
    }

    #[test]
    fn helmet_attach_attaches_each_cloned_node() {
        let mut e = helmet_engine();
        let subject = reference(&mut e);
        let biped = biped_anim(&mut e, subject, &[]);
        let this = queued_helmet(&mut e, biped, subject, 4, 1);
        e.mem.set_u32(this.addr() + 0xcc + 8, 0x3c02);
        e.mem.set_u32(this.addr() + 0x7c + 8, 0x5002);
        e.mem.set_u32(this.addr() + 0xcc + 12 * 4, 0x3c0c);
        e.call_log = Some(vec![]);
        e.call(0x0043_fb80, &args![this]);
        assert_eq!(
            calls_to(&e, BIPED_ANIM_ATTACH_HELMET),
            vec![
                vec![biped.addr(), 0x5002, 0x3c02, 2],
                vec![biped.addr(), 0, 0x3c0c, 12]
            ]
        );
        assert_eq!(
            calls_to(&e, HELMET_MAP_SLOT),
            vec![vec![HELMET_MAP_OBJECT, subject.addr()]]
        );
    }

    #[test]
    fn helmet_get_description_names_the_biped_anim() {
        let mut e = helmet_engine();
        let subject = reference(&mut e);
        let biped = biped_anim(&mut e, subject, &[]);
        let this = queued_helmet(&mut e, biped, subject, 4, 1);
        let buffer = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_fc10, &args![this, buffer, 0x80u32]);
        assert!(back.bool());
        assert_eq!(
            calls_to(&e, FORMAT_STRING),
            vec![vec![buffer, 0x80, QUEUED_HELMET_FORMAT, biped.addr()]]
        );
        let expected = format!("Queued helmet for biped anim {:08X}", biped.addr());
        assert_eq!(e.mem.cstr(buffer), expected.as_bytes());
    }

    /// A distant 3D task for `reference` holding the node 0x6d00 in
    /// `spNode`, in `state`.
    fn distant_task(e: &mut Engine, reference: Ptr, state: i32) -> Ptr<AttachDistant3DTask> {
        let this: Ptr<AttachDistant3DTask> = e.new_object();
        e.mem.set_u32(this.addr(), TASK_VTABLE);
        e.mem.set_i32(this.addr() + 0xc, state);
        e.set(this, AttachDistant3DTask::pRef, reference);
        e.set(this, AttachDistant3DTask::spNode, Ptr::new(0x6d00));
        this
    }

    #[test]
    fn distant_3d_task_post_process_attaches_the_node_to_the_reference() {
        let mut e = helmet_engine();
        let subject = reference(&mut e);
        e.mem.set_u32(subject.addr() + 0x40, 0xce11);
        let this = distant_task(&mut e, subject, 4);
        e.call_log = Some(vec![]);
        e.call(0x0043_fc40, &args![this]);
        assert_eq!(
            calls_to(&e, REFERENCE_SET_3D),
            vec![vec![subject.addr(), 0x6d00]]
        );
        assert_eq!(
            calls_to(&e, TES_LOAD_REFERENCE),
            vec![vec![TES_OBJECT, subject.addr(), 0xce11, 0, 0]]
        );
        assert_eq!(calls_to(&e, SET_STARTS_DEAD), vec![vec![subject.addr(), 1]]);
        assert!(calls_to(&e, NI_POINTER_ASSIGN).is_empty());
        assert_eq!(e.get(this, AttachDistant3DTask::spNode), Ptr::new(0x6d00));
        // A cancelled task (state 6) clears the node and does nothing else.
        let cancelled = distant_task(&mut e, subject, 6);
        e.call_log = Some(vec![]);
        e.call(0x0043_fc40, &args![cancelled]);
        assert!(calls_to(&e, REFERENCE_SET_3D).is_empty());
        assert_eq!(
            calls_to(&e, NI_POINTER_ASSIGN),
            vec![vec![cancelled.addr() + 0x1c, 0]]
        );
        assert_eq!(e.get(cancelled, AttachDistant3DTask::spNode), Ptr::NULL);
        // No reference: the same.
        let orphan = distant_task(&mut e, Ptr::NULL, 4);
        e.call_log = Some(vec![]);
        e.call(0x0043_fc40, &args![orphan]);
        assert!(calls_to(&e, TES_LOAD_REFERENCE).is_empty());
        assert_eq!(e.get(orphan, AttachDistant3DTask::spNode), Ptr::NULL);
        // A reference that already has a 3D (`0043fcd0` non-zero): the same.
        let loaded = reference(&mut e);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + 0x14, 0x7001);
        e.mem.set_u32(loaded.addr() + 0x64, data);
        let busy = distant_task(&mut e, loaded, 4);
        e.call_log = Some(vec![]);
        e.call(0x0043_fc40, &args![busy]);
        assert!(calls_to(&e, REFERENCE_SET_3D).is_empty());
        assert_eq!(e.get(busy, AttachDistant3DTask::spNode), Ptr::NULL);
    }

    #[test]
    fn reference_3d_getter_checks_the_thread_word_then_the_data() {
        let mut e = helmet_engine();
        let subject = reference(&mut e);
        // No data: 0.
        assert_eq!(e.call(0x0043_fcd0, &args![subject]).u32(), 0);
        // Data: the NiPointer at +0x14 of it.
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + 0x14, 0x7001);
        e.mem.set_u32(subject.addr() + 0x64, data);
        assert_eq!(e.call(0x0043_fcd0, &args![subject]).u32(), 0x7001);
        // The thread's reference word is this reference: the thread's 3D
        // word, whatever the data is.
        let tls = e.tls();
        e.mem.set_u32(tls + 0x264, subject.addr());
        e.mem.set_u32(tls + 0x260, 0x7002);
        assert_eq!(e.call(0x0043_fcd0, &args![subject]).u32(), 0x7002);
        // Another reference does not match.
        let other = reference(&mut e);
        assert_eq!(e.call(0x0043_fcd0, &args![other]).u32(), 0);
    }

    #[test]
    fn reference_task_constructor_builds_the_four_pointers() {
        let mut e = helmet_engine();
        let this: Ptr<QueuedReference> = e.new_object();
        e.mem.set_u32(this.addr() + 0x34, 0x1111);
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_fd40, &args![this, 0x6500u32, 0x31u32]);
        assert_eq!(back.ptr::<QueuedReference>(), this);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CONSTRUCT),
            vec![vec![this.addr(), 0x31]]
        );
        assert_eq!(e.mem.u32(this.addr()), QUEUED_REFERENCE_VTABLE);
        assert_eq!(e.get(this, QueuedReference::pRef), Ptr::new(0x6500));
        assert_eq!(
            calls_to(&e, TASK_POINTER_CONSTRUCT),
            vec![vec![this.addr() + 0x2c, 0], vec![this.addr() + 0x38, 0]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_CONSTRUCT),
            vec![vec![this.addr() + 0x30, 0], vec![this.addr() + 0x34, 0]]
        );
        assert_eq!(e.get(this, QueuedReference::spCloned3D), Ptr::NULL);
    }

    #[test]
    fn reference_task_post_process_calls_slot_0x3c() {
        let mut e = helmet_engine();
        let this: Ptr<QueuedReference> = e.new_object();
        e.mem.set_u32(this.addr(), REFERENCE_TASK_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x0043_fdf0, &args![this]);
        assert_eq!(calls_to(&e, ATTACH_HOOK), vec![vec![this.addr()]]);
    }

    #[test]
    fn reference_task_scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = helmet_engine();
        let this: Ptr<QueuedReference> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0043_fe10, &args![this, 0u32]);
        assert_eq!(back.ptr::<QueuedReference>(), this);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        assert_eq!(calls_to(&e, QUEUED_FILE_DESTRUCT).len(), 1);
        e.call(0x0043_fe10, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn reference_task_destructor_releases_the_pointers_last_first() {
        let mut e = helmet_engine();
        let this: Ptr<QueuedReference> = e.new_object();
        e.mem.set_u32(this.addr(), 0x2222);
        e.call_log = Some(vec![]);
        e.call(0x0043_fe40, &args![this]);
        assert_eq!(
            call_order(&e),
            vec![
                0x0043_fe40,
                TASK_POINTER_DESTRUCT,
                NI_POINTER_DESTRUCT,
                MODEL_POINTER_RELEASE,
                TASK_POINTER_DESTRUCT,
                QUEUED_FILE_DESTRUCT
            ]
        );
        assert_eq!(
            calls_to(&e, TASK_POINTER_DESTRUCT),
            vec![vec![this.addr() + 0x38], vec![this.addr() + 0x2c]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_DESTRUCT),
            vec![vec![this.addr() + 0x34]]
        );
        assert_eq!(
            calls_to(&e, MODEL_POINTER_RELEASE),
            vec![vec![this.addr() + 0x30]]
        );
        // The virtual table is left as it was.
        assert_eq!(e.mem.u32(this.addr()), 0x2222);
    }
}
