//! `Havok SDK/run_00da3400` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkJobQueue` job-type registration (`00d044c0`, thiscall `(job type, table, count, a, b)`).
const JOB_TYPE_REGISTER: u32 = 0x00d0_44c0;

// Translated from 00da3400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the dispatch table of seven job functions at `01269af0` (`hkCpuMoppAabbQueryJob` at
/// `+0`, the code at `00df6f80` at `+4`, `hkCpuWorldGetClosestPointsJob` at `+8`, the code at
/// `00df79d0` at `+0xc`, `hkCpuWorldLinearCastJob` at `+0x10`, the code at `00df6950` at
/// `+0x14` and the code at `00df6850` at `+0x18`) and registers it with the job queue `queue`
/// as job type 2 with the two functions `00df7ad0` and `00df7a90`: cdecl `(queue)`, called once
/// by `00d2cca0` (the job queue's constructor).
pub fn fn_00da3400(e: &mut Engine, queue: u32) {
    e.mem.set_u32(0x0126_9afc, 0x00df_79d0);
    e.mem.set_u32(0x0126_9b00, 0x00df_7340);
    e.mem.set_u32(0x0126_9af0, 0x00df_7010);
    e.mem.set_u32(0x0126_9af4, 0x00df_6f80);
    e.mem.set_u32(0x0126_9af8, 0x00df_6b50);
    e.mem.set_u32(0x0126_9b04, 0x00df_6950);
    e.mem.set_u32(0x0126_9b08, 0x00df_6850);
    e.call(
        JOB_TYPE_REGISTER,
        &args![
            queue,
            2u32,
            0x0126_9af0u32,
            7u32,
            0x00df_7ad0u32,
            0x00df_7a90u32
        ],
    );
}

// Translated from 00da3480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00da3400`, for the eight job functions at `01269ad0` (`hkCpuShapeRayCastJob`,
/// `hkCpuWorldRayCastJob`, the code at `00df7d60`, `hkCpuKdTreeBuildFastDistributedJob`,
/// `hkCpuKdTreeBuildFastDistributedSubJob`, the code at `00df8800`,
/// `hkCpuKdTreeBuildCoordinatorJob` and the code at `00df7c00`, in that order), registered as
/// job type 3 with the functions `00df9880` and `00df9920`: cdecl `(queue)`.
pub fn fn_00da3480(e: &mut Engine, queue: u32) {
    e.mem.set_u32(0x0126_9ad0, 0x00df_94b0);
    e.mem.set_u32(0x0126_9ad4, 0x00df_8df0);
    e.mem.set_u32(0x0126_9ad8, 0x00df_7d60);
    e.mem.set_u32(0x0126_9adc, 0x00df_7e20);
    e.mem.set_u32(0x0126_9ae0, 0x00df_8440);
    e.mem.set_u32(0x0126_9ae4, 0x00df_8800);
    e.mem.set_u32(0x0126_9ae8, 0x00df_8570);
    e.mem.set_u32(0x0126_9aec, 0x00df_7c00);
    e.call(
        JOB_TYPE_REGISTER,
        &args![
            queue,
            3u32,
            0x0126_9ad0u32,
            8u32,
            0x00df_9880u32,
            0x00df_9920u32
        ],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_3400, fn_00da3400(u32)),
        entry!(0x00da_3480, fn_00da3480(u32)),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    fn registered() -> (Engine, std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(JOB_TYPE_REGISTER, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (e, seen)
    }

    fn words(e: &Engine, table: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(table + 4 * i)).collect()
    }

    #[test]
    fn the_first_table_is_filled_and_registered_as_job_type_two() {
        let (mut e, seen) = registered();
        e.call(0x00da_3400, &args![0x5000u32]);
        assert_eq!(
            words(&e, 0x0126_9af0, 7),
            [
                0x00df_7010,
                0x00df_6f80,
                0x00df_6b50,
                0x00df_79d0,
                0x00df_7340,
                0x00df_6950,
                0x00df_6850
            ]
        );
        assert_eq!(
            *seen.borrow(),
            vec![vec![0x5000, 2, 0x0126_9af0, 7, 0x00df_7ad0, 0x00df_7a90]]
        );
        // Nothing next to the table was touched.
        assert_eq!(e.mem.u32(0x0126_9aec), 0);
        assert_eq!(e.mem.u32(0x0126_9b0c), 0);
    }

    #[test]
    fn the_second_table_is_filled_and_registered_as_job_type_three() {
        let (mut e, seen) = registered();
        e.call(0x00da_3480, &args![0x6000u32]);
        assert_eq!(
            words(&e, 0x0126_9ad0, 8),
            [
                0x00df_94b0,
                0x00df_8df0,
                0x00df_7d60,
                0x00df_7e20,
                0x00df_8440,
                0x00df_8800,
                0x00df_8570,
                0x00df_7c00
            ]
        );
        assert_eq!(
            *seen.borrow(),
            vec![vec![0x6000, 3, 0x0126_9ad0, 8, 0x00df_9880, 0x00df_9920]]
        );
        assert_eq!(e.mem.u32(0x0126_9af0), 0);
    }
}
