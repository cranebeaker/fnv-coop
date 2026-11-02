//! `Havok SDK/hkpactivecontactpointviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`:
//! its functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word holding the object the registration is run on.
const REGISTRY: u32 = 0x0126_9cd0;
/// The word that receives the answer.
const VIEWER_ID: u32 = 0x0126_900c;
/// The registration routine (`00dd9380`, `this` plus two words).
const REGISTER: u32 = 0x00dd_9380;
/// The first word handed to it, and the address of the routine handed as the second.
const VIEWER_TAG: u32 = 0x010d_1fe0;
const VIEWER_CREATE: u32 = 0x00d6_8890;

// Translated from 00d688d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpActiveContactPointViewer::registerViewer` (Xbox PDB): takes no arguments. Runs
/// `00dd9380` on the object held in the word at 0x01269cd0 with the words 0x010d1fe0 and
/// 0x00d68890, and stores the answer in the word at 0x0126900c.
pub fn hkp_active_contact_point_viewer_register_viewer(e: &mut Engine) {
    let registry = e.mem.u32(REGISTRY);
    let id = e
        .call(REGISTER, &args![registry, VIEWER_TAG, VIEWER_CREATE])
        .u32();
    e.mem.set_u32(VIEWER_ID, id);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d688d0,
        hkp_active_contact_point_viewer_register_viewer()
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};

    #[test]
    fn registers_the_viewer_and_keeps_the_answer() {
        let mut e = logging_engine();
        for address in [REGISTRY, VIEWER_ID] {
            e.map(address, 4);
        }
        e.mem.set_u32(REGISTRY, 0x5000);
        e.register(REGISTER, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        e.call(0x00d6_88d0, &args![]);
        assert_eq!(
            calls_to(&e, REGISTER),
            [[0x5000, VIEWER_TAG, VIEWER_CREATE]]
        );
        assert_eq!(e.mem.u32(VIEWER_ID), 0x77);
    }
}
