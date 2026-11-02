//! Havok's contact manager (`hkpSimpleConstraintContactMgr`, Xbox PDB;
//! B1 PR 6, docs/PHYSICS.md "Contact points"): the contact points one
//! agent (a pair of bodies the broadphase paired) holds, in a
//! `hkpSimpleContactConstraintAtom`, with their properties, and the
//! events the game hears when a point is added. Translated from
//! FalloutNV.exe 1.4.0.525; names are the Xbox prototype's (ADR-0002),
//! addresses the PC's (ADR-0003). Positions and distances are in world
//! (game) units; the normal points from the second body to the first.

use crate::Vec3;

/// A point's flags (`hkpContactPointProperties::m_flags`, `+0xf`): new,
/// waiting for the solver's callbacks (`hkSimpleContactConstraintData_
/// fireCallbacks` `00d92900`, before the next solve).
pub const POINT_NEW: u8 = 1;
/// Solved together with the point before it (`00d92df0` sets it on a point
/// added after one without it; the Jacobian builder `00d72190` makes the
/// two one 2 × 2 block).
pub const POINT_PAIRED: u8 = 2;
/// Disabled (`00d92900` leaves it out).
pub const POINT_DISABLED: u8 = 8;

/// The info's flag asking the Jacobian builder to work out the contact
/// radius for the angular friction again (`hkpSimpleContactConstraintDataInfo
/// ::m_flags` bit 2: set when a point is added, `00d92df0`, or removed,
/// `00cfd200`).
pub const INFO_RADIUS_DIRTY: u16 = 4;

/// `hkpContactPointProperties` (0x14 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PointProperties {
    /// `m_impulseApplied` (`+0x0`): the normal impulse the last solve gave
    /// (Havok units), or the first guess for a new point.
    pub impulse_applied: f32,
    /// `m_internalSolverData` (`+0x4`): what the solver predicted the
    /// distance to become, negated (Havok units).
    pub internal_solver_data: f32,
    /// `m_friction` (hkUFloat8, `+0xc`).
    pub friction: u8,
    /// `m_restitution` (`+0xd`): × 128.
    pub restitution: u8,
    /// `m_maxImpulse` (hkUFloat8, `+0xe`; 0: none).
    pub max_impulse: u8,
    /// `m_flags` (`+0xf`).
    pub flags: u8,
    /// `m_internalDataA` (`+0x10`): the penetration the solver allows this
    /// point (negative, Havok units).
    pub internal_data_a: f32,
}

/// A contact point (`hkContactPoint`, 0x20 bytes) with its properties and
/// the feature of the two shapes it came from (this generator's: Havok's
/// agents keep their own point ids).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContactPoint {
    pub key: u64,
    /// `m_position` (on the second body's surface).
    pub position: Vec3,
    /// `m_separatingNormal`: from the second body toward the first.
    pub normal: Vec3,
    /// `m_separatingNormal.w`: apart (positive) or into each other.
    pub distance: f32,
    pub props: PointProperties,
}

/// `hkpSimpleContactConstraintDataInfo` (0x20 bytes, atom `+0x10`).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ContactInfo {
    /// `m_flags` (`+0x10`).
    pub flags: u16,
    /// `m_index` (`+0x12`): the friction's axis (0, 1 or 2).
    pub index: u16,
    /// `m_data` (`+0x14`): the contact radius (`+0x14`), the friction's
    /// impulses and drifts (`+0x18`…`+0x2c`, written by the solver's
    /// export `00def570`).
    pub data: [f32; 7],
}

/// One agent's contact points (the atom of its contact manager).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Manifold {
    pub points: Vec<ContactPoint>,
    pub info: ContactInfo,
}

/// A point the narrowphase found, before the contact manager takes it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NewPoint {
    pub key: u64,
    pub position: Vec3,
    pub normal: Vec3,
    pub distance: f32,
}

/// The contact properties of a new point (`hkpSimpleConstraintContactMgr
/// ::setContactPointProperties` `00cfd800`): friction √(f₁f₂) as hkUFloat8
/// (`00ca9360`), restitution √(r₁r₂) × 128 rounded into a byte, no most
/// impulse.
// Translated from 00cfd800 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn point_properties(friction: (f32, f32), restitution: (f32, f32)) -> PointProperties {
    let f = (friction.0 * friction.1).max(0.0).sqrt();
    let r = (restitution.0 * restitution.1).max(0.0).sqrt();
    PointProperties {
        friction: crate::havok::ufloat8(f),
        restitution: (r * 128.0).round() as i32 as u8,
        max_impulse: 0,
        ..PointProperties::default()
    }
}

impl Manifold {
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Adds a point (`addContactPointImpl` `00cfcf80` → `00d92df0`): the
    /// info asks for the contact radius again; the properties start with
    /// no impulse and flagged new (the solver's callbacks finish them), and
    /// paired with the point before when that one isn't paired itself and
    /// has no most impulse.
    // Translated from 00cfcf80 and 00d92df0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn add(&mut self, point: NewPoint, mut props: PointProperties) -> usize {
        self.info.flags |= INFO_RADIUS_DIRTY;
        props.impulse_applied = 0.0;
        props.internal_solver_data = 0.0;
        props.flags = POINT_NEW;
        if let Some(last) = self.points.last() {
            if last.props.flags & POINT_PAIRED == 0 && last.props.max_impulse == 0 {
                props.flags = POINT_NEW | POINT_PAIRED;
            }
        }
        self.points.push(ContactPoint {
            key: point.key,
            position: point.position,
            normal: point.normal,
            distance: point.distance,
            props,
        });
        self.points.len() - 1
    }

    /// Removes point `i` (`removeContactPointImpl` `00cfd320` → `00cfd200`):
    /// the later points move down, the one now at `i` is no longer paired;
    /// the info's flags 1 and 4 are set.
    // Translated from 00cfd200 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn remove(&mut self, i: usize) {
        self.points.remove(i);
        if let Some(p) = self.points.get_mut(i) {
            p.props.flags &= !POINT_PAIRED;
        }
        self.info.flags |= 1 | INFO_RADIUS_DIRTY;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(key: u64) -> NewPoint {
        NewPoint {
            key,
            position: [0.0; 3],
            normal: [0.0, 0.0, 1.0],
            distance: 0.0,
        }
    }

    #[test]
    fn points_pair_up_as_they_come_and_part_when_one_goes() {
        let mut m = Manifold::default();
        let p = point_properties((0.5, 2.5), (0.4, 0.4));
        for k in 0..4 {
            m.add(point(k), p);
        }
        let flags: Vec<u8> = m.points.iter().map(|p| p.props.flags).collect();
        assert_eq!(flags, vec![1, 3, 1, 3]);
        assert_ne!(m.info.flags & INFO_RADIUS_DIRTY, 0);
        // The first goes: the second, now first, is single.
        m.info.flags = 0;
        m.remove(0);
        let flags: Vec<u8> = m.points.iter().map(|p| p.props.flags).collect();
        assert_eq!(flags, vec![1, 1, 3]);
        assert_eq!(m.info.flags, 5);
    }

    #[test]
    fn properties_combine_as_the_contact_manager_does() {
        // A bottle (0.5, 0.4) on the ground (2.5, 0.4): √1.25 = 1.118 kept
        // as the table's next entry, 1.14; √0.16 × 128 = 51.2 → 51.
        let p = point_properties((0.5, 2.5), (0.4, 0.4));
        assert_eq!(crate::havok::UFLOAT8[p.friction as usize], 1.14);
        assert_eq!(p.restitution, 51);
        assert_eq!(p.max_impulse, 0);
    }
}
