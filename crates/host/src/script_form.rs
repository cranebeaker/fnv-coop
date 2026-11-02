//! A script record in the game's memory, and its variables bridged to the
//! world's.
//!
//! The world keeps a script instance's variables by name
//! ([`script::interp::Locals`], in `GameState::variables`); the game keeps
//! them in a `ScriptLocals` (Xbox PDB) by the variable's id (`SLSD`), read
//! and written with `ScriptLocals::GetVariable` (`005a9140`) and
//! `SetVariable` (`005a9290`). [`write_locals`] and [`read_locals`] copy one
//! way and the other, by the variable's name (`SCVR`, matched without case,
//! as the world's names are).
//!
//! A reference variable holds a form id in the low 32 bits of its 8-byte
//! slot, as `Script::PutNumericIDInDouble` (`005acc70`) leaves it and
//! `ScriptLocals::LoadGame` (`005a9f20`) restores it (the high word 0); the
//! world holds the form id as a number. A variable is a reference when the
//! world's script declares it `ref` or when the game's own test
//! (`005a9480`: a referenced object of the script names the variable)
//! says so.

use engine::Engine;
use esm::{FormId, FourCC, Record};
use script::interp::Locals;
use script::VarKind;

/// `TESDataHandler::CreateFormOfType` (Xbox PDB, `00465110`).
const CREATE_FORM_OF_TYPE: u32 = 0x0046_5110;
/// Form type of `Script` (`SCPT`).
const FORM_TYPE_SCRIPT: u32 = 0x11;
/// Makes the `ScriptLocals` of a script instance (`005abf60`).
const NEW_LOCALS: u32 = 0x005a_bf60;
/// `ScriptLocals::GetVariable(id, script)` and `SetVariable(id, value)`
/// (Xbox PDB), `thiscall`.
const GET_VARIABLE: u32 = 0x005a_9140;
const SET_VARIABLE: u32 = 0x005a_9290;
/// Whether a referenced object of the locals' script names the variable.
const IS_REFERENCE_VARIABLE: u32 = 0x005a_9480;
/// `operator new` (`00401000`).
const NEW: u32 = 0x0040_1000;
/// `BSSimpleList::AddTail` (`00905820`), `thiscall` (`&item`).
const LIST_ADD_TAIL: u32 = 0x0090_5820;

/// `Script` (Xbox PDB) fields `Script::Load` (`005ab9f0`) fills, PC
/// offsets: `m_header` (`SCRIPT_HEADER`, 0x14 bytes), `m_data`,
/// `listRefObjects` and `listVariables` (both `BSSimpleList`, the first
/// node embedded).
const HEADER: u32 = 0x18;
const HEADER_SIZE: usize = 0x14;
const DATA: u32 = 0x30;
const REFERENCED_OBJECTS: u32 = 0x44;
const VARIABLES: u32 = 0x4c;
/// `TESForm::SetFormID(id, remove from the data handler)` (Xbox PDB,
/// `00485c10`).
const SET_FORM_ID: u32 = 0x0048_5c10;

/// A script form made in the game's memory from its record.
#[derive(Debug, Clone)]
pub struct ScriptForm {
    /// Its address.
    pub form: u32,
    pub id: FormId,
    /// Its variables, in the record's order.
    pub variables: Vec<Variable>,
    /// Its referenced objects, in order: the form (`SCRO`, load-order id,
    /// not made yet) or the variable (`SCRV`).
    pub referenced: Vec<Referenced>,
}

/// One variable of a script: its id (`SLSD` +0), name (`SCVR`) and
/// whether it holds a whole number (`SLSD` +0x10, flag 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub id: u32,
    pub name: String,
    pub integer: bool,
}

/// One referenced object of a script (`SCRIPT_REFERENCED_OBJECT`, Xbox PDB).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Referenced {
    Form(FormId),
    Variable(u32),
}

fn chunk<'r>(record: &'r Record, kind: &[u8; 4]) -> Option<&'r [u8]> {
    record.get(FourCC(*kind)).map(|s| s.data.as_slice())
}

/// `BSSimpleList::AddTail(&item)` (`00905820`), as `Script::Load` adds
/// each variable and referenced object.
fn add_tail(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_ADD_TAIL, &[list, slot.addr()]);
    });
}

impl ScriptForm {
    /// Makes the script `id` from its `record`: `CreateFormOfType(0x11)`
    /// (the `Script` constructor, `005aa0f0`), its form id, then what
    /// `Script::Load` (`005ab9f0`) reads from the record, in its order:
    /// the header (`SCHR`), the compiled code (`SCDA`, a new block), the
    /// variables (`SLSD`, a 0x20-byte `ScriptVariable`: `SCRIPT_LOCAL`'s
    /// 0x18 bytes, then the name `SCVR` as a `BSStringT`) and the
    /// referenced objects (`SCRO`/`SCRV`, 0x10 bytes: the form at +8 or the
    /// variable id at +0xC).
    ///
    /// Bridged, not translated: `Script::Load` itself needs the game's file
    /// reading (`TESFile`, with `004726f0`, `00401660` and
    /// `ScriptVariable::Load` `005e2a20` untranslated), so the record comes
    /// from the world's reader and its bytes are put where `Script::Load`
    /// puts them. The referenced forms are left unresolved (+8 null):
    /// `Script::InitItem` (`005ab400`) resolves them through the form map,
    /// and the forms of most types can't be made yet (their constructors
    /// are open). A line that uses one fails as the game's interpreter fails
    /// on an unresolved object ("failed to evaluate expression").
    pub fn load(
        e: &mut Engine,
        id: FormId,
        record: &Record,
        to_global: impl Fn(FormId) -> FormId,
    ) -> Result<ScriptForm, String> {
        let (Some(header), Some(code)) = (chunk(record, b"SCHR"), chunk(record, b"SCDA")) else {
            return Err(format!("script {id}: no compiled code (SCHR/SCDA)"));
        };
        let form = e.call(CREATE_FORM_OF_TYPE, &[FORM_TYPE_SCRIPT]).u32();
        if form == 0 {
            return Err(format!("script {id}: CreateFormOfType gave no form"));
        }
        // Its id, and into the game's form map (`TESForm::SetFormID`,
        // `00485c10`), where `InitItem`s look scripts up.
        e.call(SET_FORM_ID, &[form, id.0, 0]);
        e.mem
            .write(form + HEADER, &header[..header.len().min(HEADER_SIZE)]);
        let data = e.call(NEW, &[code.len() as u32]).u32();
        e.mem.write(data, code);
        e.mem.set_u32(form + DATA, data);
        let mut referenced = Vec::new();
        let mut variables = Vec::new();
        let mut pending: Option<(u32, u32, bool)> = None;
        for sub in &record.subrecords {
            match &sub.kind.0 {
                b"SCRO" | b"SCRV" if sub.data.len() >= 4 => {
                    let word = u32::from_le_bytes(sub.data[..4].try_into().unwrap());
                    let object = e.call(NEW, &[0x10]).u32();
                    e.mem.write(object, &[0; 0x10]);
                    if &sub.kind.0 == b"SCRO" {
                        referenced.push(Referenced::Form(to_global(FormId(word))));
                    } else {
                        e.mem.set_u32(object + 0xc, word);
                        referenced.push(Referenced::Variable(word));
                    }
                    add_tail(e, form + REFERENCED_OBJECTS, object);
                }
                b"SLSD" if sub.data.len() >= 4 => {
                    let variable = e.call(NEW, &[0x20]).u32();
                    e.mem.write(variable, &[0; 0x20]);
                    e.mem.write(variable, &sub.data[..sub.data.len().min(0x18)]);
                    let index = u32::from_le_bytes(sub.data[..4].try_into().unwrap());
                    let integer = sub.data.get(0x10).is_some_and(|f| f & 1 != 0);
                    add_tail(e, form + VARIABLES, variable);
                    pending = Some((variable, index, integer));
                }
                b"SCVR" => {
                    if let Some((variable, id, integer)) = pending.take() {
                        let length = sub
                            .data
                            .iter()
                            .position(|b| *b == 0)
                            .unwrap_or(sub.data.len());
                        let text = e.call(NEW, &[length as u32 + 1]).u32();
                        e.mem.write(text, &sub.data[..length]);
                        e.mem.set_u8(text + length as u32, 0);
                        // BSStringT (Xbox PDB): text, length, capacity.
                        e.mem.set_u32(variable + 0x18, text);
                        e.mem.set_u16(variable + 0x1c, length as u16);
                        e.mem.set_u16(variable + 0x1e, length as u16 + 1);
                        let name = String::from_utf8_lossy(&sub.data[..length]).into_owned();
                        variables.push(Variable { id, name, integer });
                    }
                }
                _ => {}
            }
        }
        Ok(ScriptForm {
            form,
            id,
            variables,
            referenced,
        })
    }

    /// A new `ScriptLocals` of this script (`005abf60`): every variable 0,
    /// the script its master.
    pub fn new_locals(&self, e: &mut Engine) -> u32 {
        e.call(NEW_LOCALS, &[self.form]).u32()
    }

    fn is_reference(
        &self,
        e: &mut Engine,
        locals: u32,
        id: u32,
        world: &Locals,
        name: &str,
    ) -> bool {
        let declared = world
            .iter()
            .any(|(n, kind, _)| kind == VarKind::Ref && n.eq_ignore_ascii_case(name));
        declared || e.call(IS_REFERENCE_VARIABLE, &[locals, id]).bool()
    }
}

/// The world's values of the script's variables into the game's `locals`
/// (`ScriptLocals::SetVariable`). A variable the world doesn't have is 0.
pub fn write_locals(e: &mut Engine, script: &ScriptForm, locals: u32, world: &Locals) {
    for v in &script.variables {
        let (id, name) = (&v.id, &v.name);
        let value = world.get(name).unwrap_or(0.0);
        let value = if script.is_reference(e, locals, *id, world, name) {
            f64::from_bits(u64::from(value as u32))
        } else {
            value
        };
        let mut words = vec![locals, *id];
        words.extend_from_slice(&split(value));
        e.call(SET_VARIABLE, &words);
    }
}

/// The game's `locals` back into the world's (`ScriptLocals::GetVariable`):
/// each variable the world has is set (a whole-number one as the world
/// keeps it); one it doesn't have is added with the kind the game gives it
/// (a reference, or a number).
pub fn read_locals(e: &mut Engine, script: &ScriptForm, locals: u32, world: &mut Locals) {
    for v in &script.variables {
        let (id, name) = (&v.id, &v.name);
        let raw = e.call(GET_VARIABLE, &[locals, *id, script.form]).f64();
        let reference = script.is_reference(e, locals, *id, world, name);
        let value = if reference {
            f64::from(raw.to_bits() as u32)
        } else {
            raw
        };
        if !world.set(name, value) {
            let kind = if reference {
                VarKind::Ref
            } else if v.integer {
                VarKind::Integer
            } else {
                VarKind::Float
            };
            world.insert(name, kind, value);
        }
    }
}

/// A double as two argument words (low first).
fn split(value: f64) -> [u32; 2] {
    let bits = value.to_bits();
    [bits as u32, (bits >> 32) as u32]
}

#[cfg(test)]
mod tests {
    use super::*;
    use esm::{RecordHeader, Subrecord};

    /// The game's memory for a test, with `NV_EXE` set to the installed
    /// executable: its data sections mapped and the allocator started, as
    /// [`crate::Host::start`] does. These functions read the executable's
    /// data (with those sections mapped zeroed instead, the script
    /// constructor's callees recurse until the stack overflows), so without
    /// it the tests have nothing to run on and pass without checking.
    fn game() -> Option<Engine> {
        let path = std::env::var("NV_EXE").ok().filter(|p| !p.is_empty())?;
        let mut e = Engine::with_exe(std::path::Path::new(&path)).unwrap();
        crate::start_allocator(&mut e);
        Some(e)
    }

    fn sub(kind: &[u8; 4], data: Vec<u8>) -> Subrecord {
        Subrecord {
            kind: FourCC(*kind),
            data,
        }
    }

    /// `SLSD`: the index, 12 bytes, the flags byte (1: a whole number), 7.
    fn slsd(index: u32, integer: bool) -> Subrecord {
        let mut data = index.to_le_bytes().to_vec();
        data.extend_from_slice(&[0; 12]);
        data.push(u8::from(integer));
        data.extend_from_slice(&[0; 7]);
        sub(b"SLSD", data)
    }

    fn name(text: &str) -> Subrecord {
        let mut data = text.as_bytes().to_vec();
        data.push(0);
        sub(b"SCVR", data)
    }

    /// A quest script with a float, a whole number, a `ref` used as a call
    /// target (its `SCRV`) and a `ref` only stored (no `SCRV`).
    fn record() -> Record {
        let mut header = vec![0u8; 0x14];
        header[8..12].copy_from_slice(&4u32.to_le_bytes());
        header[0x10] = 1;
        Record {
            header: RecordHeader {
                kind: FourCC(*b"SCPT"),
                data_size: 0,
                flags: 0,
                form_id: FormId(0x0001_2345),
                revision: 0,
                version: 15,
                unknown: 0,
            },
            subrecords: vec![
                sub(b"SCHR", header),
                sub(b"SCDA", vec![0x1d, 0, 0, 0]),
                slsd(1, false),
                name("fTimer"),
                slsd(2, true),
                name("iStep"),
                slsd(3, false),
                name("rTarget"),
                slsd(4, false),
                name("rKept"),
                sub(b"SCRO", 0x0100_0014u32.to_le_bytes().to_vec()),
                sub(b"SCRV", 3u32.to_le_bytes().to_vec()),
            ],
        }
    }

    fn world() -> Locals {
        let mut world = Locals::default();
        world.insert("ftimer", VarKind::Float, 2.8);
        world.insert("istep", VarKind::Integer, 7.0);
        world.insert("rtarget", VarKind::Ref, f64::from(0x0010_4c0cu32));
        world.insert("rkept", VarKind::Ref, f64::from(0x0001_0f2au32));
        world
    }

    #[test]
    fn a_record_becomes_a_script_with_its_variables_and_objects() {
        let Some(mut e) = game() else {
            return;
        };
        let script = ScriptForm::load(&mut e, FormId(0x0001_2345), &record(), |id| {
            FormId(id.0 & 0x00ff_ffff)
        })
        .unwrap();
        assert_eq!(e.mem.u32(script.form + 0x0c), 0x0001_2345);
        assert_eq!(e.mem.u8(script.form + 4), FORM_TYPE_SCRIPT as u8);
        let names: Vec<(u32, &str, bool)> = script
            .variables
            .iter()
            .map(|v| (v.id, v.name.as_str(), v.integer))
            .collect();
        assert_eq!(
            names,
            [
                (1, "fTimer", false),
                (2, "iStep", true),
                (3, "rTarget", false),
                (4, "rKept", false)
            ]
        );
        assert_eq!(
            script.referenced,
            [Referenced::Form(FormId(0x14)), Referenced::Variable(3)]
        );
        let code = e.mem.u32(script.form + DATA);
        assert_eq!(e.mem.bytes(code, 4), [0x1d, 0, 0, 0]);
        assert_eq!(e.mem.u32(script.form + HEADER + 8), 4);
    }

    #[test]
    fn variables_go_to_the_game_and_back_unchanged() {
        let Some(mut e) = game() else {
            return;
        };
        let script = ScriptForm::load(&mut e, FormId(0x0001_2345), &record(), |id| id).unwrap();
        let locals = script.new_locals(&mut e);
        let world = world();
        write_locals(&mut e, &script, locals, &world);
        let raw = |e: &mut Engine, id: u32| e.call(GET_VARIABLE, &[locals, id, script.form]).f64();
        assert_eq!(raw(&mut e, 1), 2.8);
        assert_eq!(raw(&mut e, 2), 7.0);
        // References: the form id in the low word, as the game keeps it.
        assert_eq!(raw(&mut e, 3).to_bits(), 0x0010_4c0c);
        assert_eq!(raw(&mut e, 4).to_bits(), 0x0001_0f2a);
        let mut back = Locals::default();
        back.insert("ftimer", VarKind::Float, 0.0);
        back.insert("istep", VarKind::Integer, 0.0);
        back.insert("rtarget", VarKind::Ref, 0.0);
        back.insert("rkept", VarKind::Ref, 0.0);
        read_locals(&mut e, &script, locals, &mut back);
        assert_eq!(back, world);
    }

    #[test]
    fn the_games_changes_come_back_and_unknown_names_are_added() {
        let Some(mut e) = game() else {
            return;
        };
        let script = ScriptForm::load(&mut e, FormId(0x0001_2345), &record(), |id| id).unwrap();
        let locals = script.new_locals(&mut e);
        // As `set iStep to 9` and a reference stored by the game (its high
        // word not cleared) leave them.
        e.call(SET_VARIABLE, &[locals, 2, 0, 0x4022_0000]);
        e.call(SET_VARIABLE, &[locals, 3, 0x0000_0014, 0x1234_5678]);
        let mut back = Locals::default();
        back.insert("rtarget", VarKind::Ref, 0.0);
        read_locals(&mut e, &script, locals, &mut back);
        assert_eq!(back.get("istep"), Some(9.0));
        assert_eq!(back.get("rtarget"), Some(20.0));
        // Not in the world's script: added, at the game's value.
        assert_eq!(back.get("ftimer"), Some(0.0));
        // A reference by the game's own test (its SCRV) though the world
        // didn't declare it.
        let mut fresh = Locals::default();
        read_locals(&mut e, &script, locals, &mut fresh);
        assert_eq!(fresh.get("rtarget"), Some(20.0));
        assert!(fresh
            .iter()
            .any(|(n, k, _)| n == "rtarget" && k == VarKind::Ref));
    }
}
