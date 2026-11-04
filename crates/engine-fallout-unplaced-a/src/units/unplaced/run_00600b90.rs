//! `(unplaced)/run_00600b90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two functions of the `NiTArray` instance whose vtable is `0104a060`
//! (RTTI `.?AV?$NiTArray@PAVTESIdleForm@@...`); the engine map gives them no
//! names.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// The instance's vtable (RTTI `.?AV?$NiTArray@PAVTESIdleForm@@...`).
const VTABLE: u32 = 0x0104_a060;
/// `004ede70` (`cdecl`, one word): frees an element block.
const FREE_BLOCK_004EDE70: u32 = 0x004e_de70;
/// `0096afc0` (`cdecl`, one word: the element count): allocates an element
/// block.
const ALLOCATE_BLOCK_0096AFC0: u32 = 0x0096_afc0;

// Translated from 00600b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The array's destructor (`thiscall`): stores the vtable and frees the
/// elements (`004ede70(m_pBase)`).
pub fn fn_00600b90(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), VTABLE);
    let base = e.get(this, NiTArray::m_pBase);
    e.call(FREE_BLOCK_004EDE70, &args![base]);
}

// Translated from 00600bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compacts the array (`thiscall`): unless `m_usESize` already equals
/// `m_usSize`, moves the non-null elements to the front (when there are
/// any), sets `m_usSize` and `m_usMaxSize` to `m_usESize`, copies the first
/// `m_usSize` elements into a new block (`0096afc0`; none and a null
/// `m_pBase` when the size is 0) and frees the old block (`004ede70`).
pub fn fn_00600bc0(e: &mut Engine, this: Ptr<NiTArray>) {
    let used = e.get(this, NiTArray::m_usESize);
    if used == e.get(this, NiTArray::m_usSize) {
        return;
    }
    if used != 0 {
        let mut to: u16 = 0;
        let mut from: u16 = 0;
        while from < e.get(this, NiTArray::m_usSize) {
            let base = e.get(this, NiTArray::m_pBase);
            let element = e.mem.u32(base + u32::from(from) * 4);
            if element != 0 {
                let target = base + u32::from(to) * 4;
                if e.mem.u32(target) != element {
                    e.mem.set_u32(target, element);
                }
                to = to.wrapping_add(1);
            }
            from = from.wrapping_add(1);
        }
    }
    let old = e.get(this, NiTArray::m_pBase);
    let used = e.get(this, NiTArray::m_usESize);
    e.set(this, NiTArray::m_usSize, used);
    let size = e.get(this, NiTArray::m_usSize);
    e.set(this, NiTArray::m_usMaxSize, size);
    let capacity = e.get(this, NiTArray::m_usMaxSize);
    if capacity > 0 {
        let block = e
            .call(ALLOCATE_BLOCK_0096AFC0, &args![u32::from(capacity)])
            .u32();
        e.set(this, NiTArray::m_pBase, block);
        let mut index: u16 = 0;
        while index < e.get(this, NiTArray::m_usSize) {
            let element = e.mem.u32(old + u32::from(index) * 4);
            let base = e.get(this, NiTArray::m_pBase);
            e.mem.set_u32(base + u32::from(index) * 4, element);
            index = index.wrapping_add(1);
        }
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    e.call(FREE_BLOCK_004EDE70, &args![old]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060_0b90, fn_00600b90(Ptr<NiTArray>)),
        entry!(0x0060_0bc0, fn_00600bc0(Ptr<NiTArray>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn array(e: &mut Engine, elements: &[u32], used: u16) -> Ptr<NiTArray> {
        let this = e.new_object::<NiTArray>();
        let base = e.mem.alloc(4 * elements.len() as u32);
        for (i, element) in elements.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *element);
        }
        e.set(this, NiTArray::m_pBase, base);
        e.set(this, NiTArray::m_usMaxSize, elements.len() as u16);
        e.set(this, NiTArray::m_usSize, elements.len() as u16);
        e.set(this, NiTArray::m_usESize, used);
        this
    }

    #[test]
    fn fn_00600b90_frees_the_elements() {
        let mut e = Engine::new();
        let this = array(&mut e, &[1, 2], 2);
        let base = e.get(this, NiTArray::m_pBase);
        e.register(FREE_BLOCK_004EDE70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0060_0b90, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (FREE_BLOCK_004EDE70, vec![base])
        );
    }

    #[test]
    fn fn_00600bc0_moves_the_elements_into_a_smaller_block() {
        let mut e = Engine::new();
        let this = array(&mut e, &[0, 0x11, 0, 0x22, 0x33], 3);
        let old = e.get(this, NiTArray::m_pBase);
        e.register(ALLOCATE_BLOCK_0096AFC0, |e, a| word(e.mem.alloc(4 * a[0])));
        e.register(FREE_BLOCK_004EDE70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0060_0bc0, &args![this]);
        let log = e.call_log.take().unwrap();
        let block = e.get(this, NiTArray::m_pBase);
        assert_ne!(block, old);
        assert_eq!(e.mem.u32(block), 0x11);
        assert_eq!(e.mem.u32(block + 4), 0x22);
        assert_eq!(e.mem.u32(block + 8), 0x33);
        assert_eq!(e.get(this, NiTArray::m_usSize), 3);
        assert_eq!(e.get(this, NiTArray::m_usMaxSize), 3);
        assert_eq!(log[1], (ALLOCATE_BLOCK_0096AFC0, vec![3]));
        assert_eq!(log[2], (FREE_BLOCK_004EDE70, vec![old]));
        // The old block was compacted in place first.
        assert_eq!(e.mem.u32(old), 0x11);
    }

    #[test]
    fn fn_00600bc0_empties_or_keeps_the_array() {
        let mut e = Engine::new();
        let this = array(&mut e, &[0, 0], 0);
        let old = e.get(this, NiTArray::m_pBase);
        e.register(FREE_BLOCK_004EDE70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0060_0bc0, &args![this]);
        assert_eq!(e.get(this, NiTArray::m_pBase), 0);
        assert_eq!(e.get(this, NiTArray::m_usMaxSize), 0);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (FREE_BLOCK_004EDE70, vec![old])
        );

        let this = array(&mut e, &[1, 2], 2);
        e.call_log = Some(vec![]);
        e.call(0x0060_0bc0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
