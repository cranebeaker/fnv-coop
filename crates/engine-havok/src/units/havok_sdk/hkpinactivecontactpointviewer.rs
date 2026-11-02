//! `Havok SDK/hkpinactivecontactpointviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The process id `registerViewer` stores (`01269008`), the process factory object
/// (`01269cd0`), `hkProcessFactory::registerProcess` (Xbox PDB, `00dd9380`, `this` the factory,
/// then the name and the creation function), the string `"Contact Points (Inactive)"` at
/// `010d1f50` and the creation function at `00d687c0`.
const PROCESS_ID: u32 = 0x0126_9008;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_NAME: u32 = 0x010d_1f50;
const PROCESS_CREATE: u32 = 0x00d6_87c0;

// Translated from 00d68800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInactiveContactPointViewer::registerViewer` (Xbox PDB): registers the process
/// `"Contact Points (Inactive)"` with the process factory at `01269cd0` (creation function
/// `00d687c0`) and stores its id at `01269008`.
pub fn hkp_inactive_contact_point_viewer_register_viewer(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, PROCESS_NAME, PROCESS_CREATE],
        )
        .u32();
    e.set_global(PROCESS_ID, id);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d6_8800,
        hkp_inactive_contact_point_viewer_register_viewer()
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn register_viewer_registers_the_process_and_stores_its_id() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(REGISTER_PROCESS, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: 0x4321,
                ..Ret::default()
            }
        });
        e.set_global(PROCESS_FACTORY, 0x5555u32);
        e.call(0x00d6_8800, &args![]);
        assert_eq!(*seen.borrow(), vec![vec![0x5555, 0x010d_1f50, 0x00d6_87c0]]);
        assert_eq!(e.global::<u32>(PROCESS_ID), 0x4321);
    }
}
