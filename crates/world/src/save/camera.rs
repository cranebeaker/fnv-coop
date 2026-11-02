//! Versioned, deterministic encoding of the camera/animation snapshot stored
//! in an nv-rs save. This is a compact binary payload represented as hex so
//! it can live on one line in the text save format.

use std::collections::BTreeMap;

use esm::FormId;

use crate::animation::{self, snapshot};

const MAGIC: &[u8; 4] = b"NVCM";
const VERSION: u8 = 1;
const MAX_BYTES: usize = 16 * 1024 * 1024;
const MAX_ITEMS: usize = 262_144;
const MAX_STRING: usize = 1024 * 1024;

/// Camera view state preserved by an nv-rs save.
#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub animation: Option<snapshot::Snapshot>,
    pub requests: Vec<FormId>,
    pub npc_requests: BTreeMap<FormId, FormId>,
    pub package: Option<FormId>,
    pub hand_follow: f32,
    pub pitch: f32,
}

/// Encode to lowercase hexadecimal with a magic and explicit format version.
pub fn encode(camera: &Camera) -> String {
    let mut out = Writer::default();
    out.bytes.extend_from_slice(MAGIC);
    out.u8(VERSION);
    out.option(camera.animation.as_ref(), |w, value| {
        write_snapshot(w, value)
    });
    out.vec(&camera.requests, |w, id| w.u32(id.0));
    out.len(camera.npc_requests.len());
    for (from, to) in &camera.npc_requests {
        out.u32(from.0);
        out.u32(to.0);
    }
    out.option(camera.package, |w, id| w.u32(id.0));
    out.f32(camera.hand_follow);
    out.f32(camera.pitch);
    let mut hex = String::with_capacity(out.bytes.len() * 2);
    for byte in out.bytes {
        use std::fmt::Write as _;
        write!(&mut hex, "{byte:02x}").expect("writing to String cannot fail");
    }
    hex
}

/// Decode a hex camera payload, rejecting unsupported or malformed data.
pub fn decode(text: &str) -> Result<Camera, String> {
    if text.len() % 2 != 0 || text.len() / 2 > MAX_BYTES {
        return Err("camera payload has invalid or excessive length".into());
    }
    let bytes = decode_hex(text)?;
    let mut r = Reader {
        bytes: &bytes,
        at: 0,
    };
    if r.take(4)? != MAGIC {
        return Err("camera payload has invalid magic".into());
    }
    let version = r.u8()?;
    if version != VERSION {
        return Err(format!("unsupported camera payload version {version}"));
    }
    let animation = r.option(read_snapshot)?;
    let requests = r.vec(|r| Ok(FormId(r.u32()?)))?;
    let count = r.len()?;
    let mut npc_requests = BTreeMap::new();
    for _ in 0..count {
        let key = FormId(r.u32()?);
        let value = FormId(r.u32()?);
        if npc_requests.insert(key, value).is_some() {
            return Err("camera payload has duplicate NPC request keys".into());
        }
    }
    let package = r.option(|r| Ok(FormId(r.u32()?)))?;
    let hand_follow = r.f32()?;
    let pitch = r.f32()?;
    r.finish()?;
    Ok(Camera {
        animation,
        requests,
        npc_requests,
        package,
        hand_follow,
        pitch,
    })
}

#[derive(Default)]
struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    fn u8(&mut self, v: u8) {
        self.bytes.push(v);
    }
    fn u32(&mut self, v: u32) {
        self.bytes.extend_from_slice(&v.to_le_bytes());
    }
    fn i32(&mut self, v: i32) {
        self.bytes.extend_from_slice(&v.to_le_bytes());
    }
    fn f32(&mut self, v: f32) {
        self.u32(v.to_bits());
    }
    fn len(&mut self, v: usize) {
        self.u32(u32::try_from(v).expect("camera collection exceeds u32"));
    }
    fn string(&mut self, s: &str) {
        self.len(s.len());
        self.bytes.extend_from_slice(s.as_bytes());
    }
    fn option<T>(&mut self, value: Option<T>, f: impl FnOnce(&mut Self, T)) {
        match value {
            Some(v) => {
                self.u8(1);
                f(self, v);
            }
            None => self.u8(0),
        }
    }
    fn vec<T>(&mut self, values: &[T], mut f: impl FnMut(&mut Self, &T)) {
        self.len(values.len());
        for value in values {
            f(self, value);
        }
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self
            .at
            .checked_add(n)
            .ok_or("camera payload length overflow")?;
        let result = self
            .bytes
            .get(self.at..end)
            .ok_or("truncated camera payload")?;
        self.at = end;
        Ok(result)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn i32(&mut self) -> Result<i32, String> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn f32(&mut self) -> Result<f32, String> {
        let v = f32::from_bits(self.u32()?);
        if !v.is_finite() {
            return Err("camera payload contains a non-finite float".into());
        }
        Ok(v)
    }
    fn len(&mut self) -> Result<usize, String> {
        let n = usize::try_from(self.u32()?).map_err(|_| "camera collection length overflow")?;
        if n > MAX_ITEMS {
            return Err("camera payload collection is too large".into());
        }
        Ok(n)
    }
    fn string(&mut self) -> Result<String, String> {
        let n = usize::try_from(self.u32()?).map_err(|_| "camera string length overflow")?;
        if n > MAX_STRING {
            return Err("camera payload string is too large".into());
        }
        String::from_utf8(self.take(n)?.to_vec())
            .map_err(|_| "camera payload string is not UTF-8".into())
    }
    fn option<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, String>,
    ) -> Result<Option<T>, String> {
        match self.u8()? {
            0 => Ok(None),
            1 => Ok(Some(f(self)?)),
            _ => Err("invalid camera option tag".into()),
        }
    }
    fn vec<T>(
        &mut self,
        mut f: impl FnMut(&mut Self) -> Result<T, String>,
    ) -> Result<Vec<T>, String> {
        let n = self.len()?;
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(f(self)?);
        }
        Ok(result)
    }
    fn finish(&self) -> Result<(), String> {
        if self.at == self.bytes.len() {
            Ok(())
        } else {
            Err("trailing bytes in camera payload".into())
        }
    }
}

fn decode_hex(text: &str) -> Result<Vec<u8>, String> {
    fn nibble(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    }
    let mut bytes = Vec::with_capacity(text.len() / 2);
    for pair in text.as_bytes().chunks_exact(2) {
        let hi = nibble(pair[0]).ok_or("invalid hex camera payload")?;
        let lo = nibble(pair[1]).ok_or("invalid hex camera payload")?;
        bytes.push((hi << 4) | lo);
    }
    Ok(bytes)
}

fn write_snapshot(w: &mut Writer, s: &snapshot::Snapshot) {
    w.vec(&s.bones, |w, b| {
        w.string(&b.name);
        w.option(
            b.parent
                .map(|v| u32::try_from(v).expect("bone parent exceeds u32")),
            |w, v| w.u32(v),
        );
    });
    w.f32(s.settings.default_blend);
    w.f32(s.settings.mult);
    w.f32(s.movement_rate);
    w.f32(s.weapon_rate);
    w.vec(&s.active, write_active);
}

fn read_snapshot(r: &mut Reader<'_>) -> Result<snapshot::Snapshot, String> {
    let bones = r.vec(|r| {
        Ok(snapshot::BoneIdentity {
            name: r.string()?,
            parent: r.option(|r| {
                usize::try_from(r.u32()?).map_err(|_| "bone parent index overflow".into())
            })?,
        })
    })?;
    let settings = animation::Settings {
        default_blend: r.f32()?,
        mult: r.f32()?,
    };
    let movement_rate = r.f32()?;
    let weapon_rate = r.f32()?;
    let active = r.vec(read_active)?;
    if bones
        .iter()
        .any(|bone| bone.parent.is_some_and(|parent| parent >= bones.len()))
    {
        return Err("camera snapshot has an invalid bone parent".into());
    }
    for entry in &active {
        let expected_frozen = if entry.sequence_key.is_some() {
            0
        } else {
            bones.len()
        };
        if entry.section as usize >= animation::section::COUNT
            || entry.frozen.len() != expected_frozen
        {
            return Err("camera snapshot has an invalid animation entry".into());
        }
    }
    Ok(snapshot::Snapshot {
        bones,
        settings,
        movement_rate,
        weapon_rate,
        active,
    })
}

fn write_active(w: &mut Writer, a: &snapshot::ActiveSnapshot) {
    w.option(a.sequence_key.as_ref(), |w, s| w.string(s));
    w.option(a.sequence_signature.as_ref(), write_signature);
    w.vec(&a.frozen, |w, pose| {
        w.option(*pose, |w, (priority, pose)| {
            w.u8(priority);
            write_pose(w, pose);
        });
    });
    w.u8(a.group);
    w.u8(a.section);
    write_group_data(w, a.data);
    w.u8(match a.state {
        animation::State::Animating => 0,
        animation::State::EaseIn => 1,
        animation::State::EaseOut => 2,
        animation::State::TransSource => 3,
        animation::State::TransDest => 4,
    });
    for f in [a.ease, a.ease_length, a.time] {
        w.f32(f);
    }
    w.i32(a.loops_left);
    w.f32(a.weight);
}

fn read_active(r: &mut Reader<'_>) -> Result<snapshot::ActiveSnapshot, String> {
    let sequence_key = r.option(|r| r.string())?;
    if sequence_key.as_deref() == Some("") {
        return Err("empty camera animation sequence key".into());
    }
    let sequence_signature = r.option(read_signature)?;
    if sequence_key.is_some() != sequence_signature.is_some() {
        return Err("camera animation key and signature do not match".into());
    }
    let frozen = r.vec(|r| r.option(|r| Ok((r.u8()?, read_pose(r)?))))?;
    let group = r.u8()?;
    let section = r.u8()?;
    let data = read_group_data(r)?;
    let state = match r.u8()? {
        0 => animation::State::Animating,
        1 => animation::State::EaseIn,
        2 => animation::State::EaseOut,
        3 => animation::State::TransSource,
        4 => animation::State::TransDest,
        _ => return Err("invalid camera animation state".into()),
    };
    let ease = r.f32()?;
    let ease_length = r.f32()?;
    let time = r.f32()?;
    let loops_left = r.i32()?;
    let weight = r.f32()?;
    Ok(snapshot::ActiveSnapshot {
        sequence_key,
        sequence_signature,
        frozen,
        group,
        section,
        data,
        state,
        ease,
        ease_length,
        time,
        loops_left,
        weight,
    })
}

fn write_signature(w: &mut Writer, s: &snapshot::SequenceSignature) {
    w.string(&s.name);
    w.f32(s.start);
    w.f32(s.stop);
    w.u8(u8::from(s.looping));
    w.vec(&s.tracks, |w, track| {
        w.string(&track.node);
        w.u8(track.priority);
    });
    write_group_data(w, s.data);
}

fn read_signature(r: &mut Reader<'_>) -> Result<snapshot::SequenceSignature, String> {
    let name = r.string()?;
    let start = r.f32()?;
    let stop = r.f32()?;
    let looping = match r.u8()? {
        0 => false,
        1 => true,
        _ => return Err("invalid camera sequence looping flag".into()),
    };
    let tracks = r.vec(|r| {
        Ok(snapshot::TrackIdentity {
            node: r.string()?,
            priority: r.u8()?,
        })
    })?;
    let data = read_group_data(r)?;
    Ok(snapshot::SequenceSignature {
        name,
        start,
        stop,
        looping,
        tracks,
        data,
    })
}

fn write_group_data(w: &mut Writer, data: animation::GroupData) {
    w.u8(data.blend);
    w.u8(data.blend_in);
    w.u8(data.blend_out);
    for f in [data.start, data.end, data.loop_start, data.loop_end] {
        w.f32(f);
    }
    for f in data.travel {
        w.f32(f);
    }
}

fn read_group_data(r: &mut Reader<'_>) -> Result<animation::GroupData, String> {
    Ok(animation::GroupData {
        blend: r.u8()?,
        blend_in: r.u8()?,
        blend_out: r.u8()?,
        start: r.f32()?,
        end: r.f32()?,
        loop_start: r.f32()?,
        loop_end: r.f32()?,
        travel: [r.f32()?, r.f32()?, r.f32()?],
        // Not saved: the animation snapshot reads it from the file.
        attach: 0.0,
    })
}

fn write_pose(w: &mut Writer, pose: nif::anim::Pose) {
    w.option(pose.translation, |w, v| {
        for f in v {
            w.f32(f);
        }
    });
    w.option(pose.rotation, |w, v| {
        for f in v {
            w.f32(f);
        }
    });
    w.option(pose.scale, |w, v| w.f32(v));
}

fn read_pose(r: &mut Reader<'_>) -> Result<nif::anim::Pose, String> {
    Ok(nif::anim::Pose {
        translation: r.option(|r| Ok([r.f32()?, r.f32()?, r.f32()?]))?,
        rotation: r.option(|r| Ok([r.f32()?, r.f32()?, r.f32()?, r.f32()?]))?,
        scale: r.option(|r| r.f32())?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn camera() -> Camera {
        let pose = nif::anim::Pose {
            translation: Some([1.25, -0.0, 3.5]),
            rotation: Some([0.0, 0.25, -0.5, 1.0]),
            scale: Some(1.0),
        };
        Camera {
            animation: Some(snapshot::Snapshot {
                bones: vec![
                    snapshot::BoneIdentity {
                        name: "Bip01".into(),
                        parent: None,
                    },
                    snapshot::BoneIdentity {
                        name: "Camera1st".into(),
                        parent: Some(0),
                    },
                ],
                settings: animation::Settings {
                    default_blend: 0.2,
                    mult: 1.0,
                },
                movement_rate: 77.0,
                weapon_rate: 1.5,
                active: vec![
                    snapshot::ActiveSnapshot {
                        sequence_key: Some(r"meshes\actors\male\idleanims\camera.kf".into()),
                        sequence_signature: Some(snapshot::SequenceSignature {
                            name: "CameraIdle".into(),
                            start: 0.0,
                            stop: 1.0,
                            looping: true,
                            tracks: vec![snapshot::TrackIdentity {
                                node: "Camera1st".into(),
                                priority: 10,
                            }],
                            data: animation::GroupData {
                                blend: 6,
                                blend_in: 4,
                                blend_out: 3,
                                start: 0.0,
                                end: 1.0,
                                loop_start: 0.125,
                                loop_end: 0.875,
                                travel: [0.0, 1.0, -2.0],
                                attach: 0.0,
                            },
                        }),
                        frozen: Vec::new(),
                        group: 0,
                        section: 0,
                        data: animation::GroupData {
                            blend: 6,
                            blend_in: 4,
                            blend_out: 3,
                            start: 0.0,
                            end: 1.0,
                            loop_start: 0.125,
                            loop_end: 0.875,
                            travel: [0.0, 1.0, -2.0],
                            attach: 0.0,
                        },
                        state: animation::State::EaseIn,
                        ease: 0.05,
                        ease_length: 0.2,
                        time: 0.25,
                        loops_left: -1,
                        weight: 0.75,
                    },
                    snapshot::ActiveSnapshot {
                        sequence_key: None,
                        sequence_signature: None,
                        frozen: vec![Some((10, pose)), None],
                        group: 1,
                        section: 1,
                        data: animation::GroupData {
                            blend: 0,
                            blend_in: 0,
                            blend_out: 0,
                            start: 0.0,
                            end: 1.0,
                            loop_start: 0.0,
                            loop_end: 1.0,
                            travel: [0.0; 3],
                            attach: 0.0,
                        },
                        state: animation::State::TransSource,
                        ease: 0.1,
                        ease_length: 0.2,
                        time: 0.3,
                        loops_left: 0,
                        weight: 1.0,
                    },
                ],
            }),
            requests: vec![FormId(0x12345678), FormId(0x87654321)],
            npc_requests: BTreeMap::from([(FormId(4), FormId(8)), (FormId(2), FormId(3))]),
            package: Some(FormId(0x10203040)),
            hand_follow: 0.85,
            pitch: -0.125,
        }
    }

    #[test]
    fn round_trips_full_camera_snapshot_and_is_deterministic() {
        let value = camera();
        let encoded = encode(&value);
        assert!(encoded
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
        assert_eq!(encoded, encode(&value));
        assert_eq!(decode(&encoded).unwrap(), value);
        assert_eq!(encode(&decode(&encoded).unwrap()), encoded);
    }

    #[test]
    fn rejects_malformed_and_unsupported_payloads() {
        let base = encode(&Camera {
            animation: None,
            requests: Vec::new(),
            npc_requests: BTreeMap::new(),
            package: None,
            hand_follow: 0.85,
            pitch: 0.0,
        });
        assert!(decode("0").is_err());
        assert!(decode("zz").is_err());
        assert!(decode(&format!("{base}00"))
            .unwrap_err()
            .contains("trailing"));
        let mut unknown = decode_hex(&base).unwrap();
        unknown[4] = VERSION + 1;
        let unknown: String = unknown.iter().map(|b| format!("{b:02x}")).collect();
        assert!(decode(&unknown).unwrap_err().contains("version"));
        let mut non_finite = decode_hex(&base).unwrap();
        non_finite[15..19].copy_from_slice(&f32::NAN.to_bits().to_le_bytes());
        let non_finite: String = non_finite.iter().map(|b| format!("{b:02x}")).collect();
        assert!(decode(&non_finite).unwrap_err().contains("non-finite"));
        assert!(decode(&"00".repeat(MAX_BYTES + 1))
            .unwrap_err()
            .contains("length"));
        let mut invalid_snapshot = camera();
        invalid_snapshot.animation.as_mut().unwrap().active[0].section = u8::MAX;
        assert!(decode(&encode(&invalid_snapshot))
            .unwrap_err()
            .contains("invalid animation entry"));
        let mut missing_signature = camera();
        missing_signature.animation.as_mut().unwrap().active[0].sequence_signature = None;
        assert!(decode(&encode(&missing_signature))
            .unwrap_err()
            .contains("key and signature"));
    }
}
