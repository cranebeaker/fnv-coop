//! Global variables in the game's memory, where the interpreter reads them.
//!
//! A script reads a global through its referenced object (the `G` operand
//! of `005ac7a0`): the float at `TESGlobal` +0x24, through `00526ac0`. The
//! world keeps the value by form id (`GameState::globals`); [`write_global`]
//! and [`read_global`] carry it one way and the other.

use engine::units::fallout_shared::run_005a6150::TESGlobal;
use engine::{Engine, Ptr};
use esm::{FormId, FourCC, Record};

/// `TESDataHandler::CreateFormOfType` (Xbox PDB, `00465110`).
const CREATE_FORM_OF_TYPE: u32 = 0x0046_5110;
const FORM_TYPE_GLOBAL: u32 = 0x06;
/// The float at +0x24 (`00526ac0`): a global's value as the interpreter
/// reads it.
const GLOBAL_VALUE_OF: u32 = 0x0052_6ac0;
/// `TESForm::iFormID` (Xbox PDB) +0xC.
const FORM_ID: u32 = 0x0c;

fn chunk<'r>(record: &'r Record, kind: &[u8; 4]) -> Option<&'r [u8]> {
    record.get(FourCC(*kind)).map(|s| s.data.as_slice())
}

/// A global variable made in the game's memory.
#[derive(Debug, Clone, Copy)]
pub struct GlobalForm {
    pub form: u32,
    pub id: FormId,
}

impl GlobalForm {
    /// Makes the global `id` from its `record`: `CreateFormOfType(6)` (the
    /// `TESGlobal` constructor `005a6320`: a short of value 0) and its form
    /// id, then what `TESGlobal::Load` keeps of `FNAM` (the type, `'s'`,
    /// `'l'` or `'f'`, at +0x20) and `FLTV` (the value, +0x24). Bridged:
    /// `TESGlobal::Load` reads through `TESFile`, whose reading isn't
    /// translated; the record's values are put where it puts them.
    pub fn load(e: &mut Engine, id: FormId, record: &Record) -> Result<GlobalForm, String> {
        let form = e.call(CREATE_FORM_OF_TYPE, &[FORM_TYPE_GLOBAL]).u32();
        if form == 0 {
            return Err(format!("global {id}: CreateFormOfType gave no form"));
        }
        e.mem.set_u32(form + FORM_ID, id.0);
        let global = Ptr::<TESGlobal>::new(form);
        if let Some(&kind) = chunk(record, b"FNAM").and_then(|d| d.first()) {
            e.set(global, TESGlobal::cType, kind);
        }
        if let Some(value) = chunk(record, b"FLTV").filter(|d| d.len() >= 4) {
            let value = f32::from_le_bytes(value[..4].try_into().unwrap());
            e.set(global, TESGlobal::fValue, value);
        }
        Ok(GlobalForm { form, id })
    }
}

/// The world's value of a global into the game's (`TESGlobal::fValue`).
pub fn write_global(e: &mut Engine, global: &GlobalForm, value: f32) {
    e.set(Ptr::<TESGlobal>::new(global.form), TESGlobal::fValue, value);
}

/// The game's value of a global, as the interpreter reads it (`00526ac0`).
pub fn read_global(e: &mut Engine, global: &GlobalForm) -> f32 {
    e.call(GLOBAL_VALUE_OF, &[global.form]).f64() as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use esm::{RecordHeader, Subrecord};

    /// See `script_form::tests::game`: these need the executable's data.
    fn game() -> Option<Engine> {
        let path = std::env::var("NV_EXE").ok().filter(|p| !p.is_empty())?;
        let mut e = Engine::with_exe(std::path::Path::new(&path)).unwrap();
        crate::start_allocator(&mut e);
        Some(e)
    }

    fn record(id: u32, subrecords: Vec<(&[u8; 4], Vec<u8>)>) -> Record {
        Record {
            header: RecordHeader {
                kind: FourCC(*b"GLOB"),
                data_size: 0,
                flags: 0,
                form_id: FormId(id),
                revision: 0,
                version: 15,
                unknown: 0,
            },
            subrecords: subrecords
                .into_iter()
                .map(|(k, data)| Subrecord {
                    kind: FourCC(*k),
                    data,
                })
                .collect(),
        }
    }

    #[test]
    fn a_global_holds_its_type_and_value_where_the_interpreter_reads_them() {
        let Some(mut e) = game() else {
            return;
        };
        let global = GlobalForm::load(
            &mut e,
            FormId(0x38),
            &record(
                0x38,
                vec![
                    (b"FNAM", vec![b'f']),
                    (b"FLTV", 12.0f32.to_le_bytes().to_vec()),
                ],
            ),
        )
        .unwrap();
        assert_eq!(e.mem.u8(global.form + 4), 0x06);
        assert_eq!(e.mem.u32(global.form + FORM_ID), 0x38);
        assert_eq!(e.mem.u8(global.form + 0x20), b'f');
        assert_eq!(read_global(&mut e, &global), 12.0);
        write_global(&mut e, &global, 23.5);
        assert_eq!(read_global(&mut e, &global), 23.5);
        // Without FNAM/FLTV: the constructor's short of value 0.
        let short = GlobalForm::load(&mut e, FormId(0x39), &record(0x39, vec![])).unwrap();
        assert_eq!(e.mem.u8(short.form + 0x20), b's');
        assert_eq!(read_global(&mut e, &short), 0.0);
    }
}
