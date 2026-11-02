//! `Havok SDK/hkpagent1nmachineprocess.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The two process functions rebuild an agent track (see `hkpagent1nmachine.rs` for its layout:
//! sectors of `0x200` bytes holding entries of 16-byte units) entry by entry. The input record
//! (the "input" of both functions) is 32 words; the process functions keep a copy of it in their
//! frame (the "context", whose words `+0`/`+4` end up pointing at records of
//! `(body, key, collidable word +8, collidable)`, word `+0xc` at the collision input with the
//! dispatcher, word `+0x10` at an argument passed on) and hand a pointer to that copy to the
//! agent functions of the dispatcher. The dispatcher keeps one `0x50`-byte record per agent type
//! (the entry's byte at `+1`) with function pointers at `+0x16b0`, `+0x16b4` (destroy), `+0x16b8`,
//! `+0x16d8` and `+0x16dc` and a flag word at `+0x16f0` (2: the agent wants the two bodies
//! swapped); their purposes are not identified, they are named by their offsets.
//!
//! Entry types handled: 0 and 1 (kept, 0x10 bytes), 2, 3, 6, 10, 11 and 14 (agent data at `+0x10`),
//! 4, 5, 12 and 13 (agent data at `+0x20`; types 10 to 14 are the same agents after the transform
//! shapes were resolved: their type with bit 3 cleared is used); types 7, 8, 9 and above 14 are
//! dropped (the write position does not move on, so the next entry overwrites them). The
//! exe's stack frame is mirrored with the offsets of the disassembly (`s + 0x..` are offsets from
//! the stack pointer after the prologue).
//!
//! SSE code computes lane by lane in `f32`; x87 code computes in `f64` and rounds to `f32` at each
//! `float` store. Floats copied through `FLD`/`FSTP` go through [`x87_quiet`] and
//! [`x87_quiet_bits`]. The profiling record at the end (`MinumTim`, the number of entries that
//! were advanced past) is appended to the thread's monitor stream.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpagent1nmachine::{
    copy_units, dispatcher_function, router_allocator, router_free, take_sector, ARRAY_RESERVE,
    MEM_MOVE, MONITOR_SLOT, PROCESS_TRANSFORMED_SHAPES, SECTOR_SIZE, TLS_GET_VALUE,
    WARP_MISMATCH_TIME,
};

/// `hkTransform::setInverse` (`00cb21e0`, thiscall `(this, source)`), `hkTransform::
/// setMulInverseMul` (`00cb22f0`, thiscall `(this, a, b)`) and `hkSweptTransformUtil::lerp2`
/// (`00cf1510`, cdecl `(motion state transform, time, result)`).
pub(super) const TRANSFORM_SET_INVERSE: u32 = 0x00cb_21e0;
pub(super) const TRANSFORM_SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
pub(super) const SWEPT_LERP2: u32 = 0x00cf_1510;
/// `hkAgent1nMachine_Weld` (`00da3930`, cdecl `(context, provider, state)`).
const WELD: u32 = 0x00da_3930;
/// Unnamed (`00d2eb50`, fastcall on the object at the state's `+0x3040`): swaps bytes of the
/// object after the state's direction vector was mirrored.
const STATE_REORDER: u32 = 0x00d2_eb50;
/// The sign mask `(0x80000000, 0x80000000, 0x80000000, 0)` XORed into vectors.
const SIGN_MASK: u32 = 0x010c_4c00;
/// `double` constants: 0.5 and (the `float` 0.1 as a `double`) 0.1.
const HALF: u32 = 0x0101_1588;
const TENTH: u32 = 0x0101_ffa0;
/// Label of the profiling record of the process functions ("MinumTim").
pub(super) const MINIMUM_TIME_LABEL: u32 = 0x010c_c9d0;
/// A global the create step compares with 1 (0 in the exe image).
pub(super) const SKIP_CREATE: u32 = 0x0126_9200;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// A 16-byte vector XORed with the sign mask (`XORPS`), stored at `to`.
fn store_flipped(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane) ^ e.mem.u32(SIGN_MASK + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// `FLD float [from]` / `FSTP float [to]`.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

// Translated from 00da4920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgent1nMachine_flipInput` (named in the engine map), register input: `destination` is EAX
/// and `source` ECX. Swaps the two body pointers (words `+0` and `+4`), copies words `+0xc` and
/// `+0x10`, stores the inverse of the transform at `source + 0x20` (`hkTransform::setInverse`) at
/// `destination + 0x20` and the vector at `source + 0x70` with its first three lanes negated at
/// `destination + 0x70`.
pub fn hk_agent1n_machine_flip_input(e: &mut Engine, destination: Ptr, source: Ptr) {
    let (dst, src) = (destination.addr(), source.addr());
    let word = e.mem.u32(src + 4);
    e.mem.set_u32(dst, word);
    let word = e.mem.u32(src);
    e.mem.set_u32(dst + 4, word);
    let word = e.mem.u32(src + 0xc);
    e.mem.set_u32(dst + 0xc, word);
    let word = e.mem.u32(src + 0x10);
    e.mem.set_u32(dst + 0x10, word);
    store_flipped(e, src + 0x70, dst + 0x70);
    e.call(TRANSFORM_SET_INVERSE, &args![dst + 0x20, src + 0x20]);
}

// Translated from 00da49b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgent1nMachine_initInputAtTime` (named in the engine map), register input: `source` is ESI
/// (a context: words `+0` and `+4` point at the two bodies, `+0xc` at the collision input whose
/// float at `+0x50` is the time, `+0x10` is passed on), `block` is EDI (a 0x80-byte block) and
/// `output` is the one stack argument (the caller removes it); ECX is not used. Interpolates the
/// two bodies' motion states to the time (`hkSweptTransformUtil::lerp2` into `block + 0x20` and
/// `block + 0x60`), fills `block` (two records at `+0` and `+0x10`) and `output` (pointers to
/// them, the words `+0xc` and `+0x10` of the source) and stores the relative transform of the two
/// results at `output + 0x20`.
pub fn hk_agent1n_machine_init_input_at_time(e: &mut Engine, source: Ptr, block: Ptr, output: Ptr) {
    let (src, block, out) = (source.addr(), block.addr(), output.addr());
    let first = e.mem.u32(e.mem.u32(src) + 8) + 0x40;
    let time = x87_quiet_bits(e.mem.u32(e.mem.u32(src + 0xc) + 0x50));
    e.call(SWEPT_LERP2, &args![first, time, block + 0x20]);
    let second = e.mem.u32(e.mem.u32(src + 4) + 8) + 0x40;
    let time = x87_quiet_bits(e.mem.u32(e.mem.u32(src + 0xc) + 0x50));
    e.call(SWEPT_LERP2, &args![second, time, block + 0x60]);
    e.mem.set_u32(out, block);
    let record = block.wrapping_add(0x10);
    e.mem.set_u32(out + 4, record);
    let word = e.mem.u32(src + 0x10);
    e.mem.set_u32(out + 0x10, word);
    let word = e.mem.u32(src + 0xc);
    e.mem.set_u32(out + 0xc, word);
    let body = e.mem.u32(src);
    let low = e.mem.u32(body + 4);
    let high = e.mem.u32(body);
    e.mem.set_u32(block, high);
    e.mem.set_u32(block + 4, low);
    let word = e.mem.u32(src);
    e.mem.set_u32(block + 0xc, word);
    e.mem.set_u32(block + 8, block + 0x20);
    if record != 0 {
        let word = e.mem.u32(src + 4);
        e.mem.set_u32(record + 0xc, word);
        e.mem.set_u32(record + 8, block + 0x60);
    }
    e.call(
        TRANSFORM_SET_MUL_INVERSE_MUL,
        &args![out + 0x20, block + 0x20, block + 0x60],
    );
}

/// What follows an entry.
enum Flow {
    Continue,
    Finished,
}

/// The rebuild of the track: the entries are read from the input sectors and written to output
/// sectors, a 0x80-byte scratch area collecting entries when the output sector is nearly full
/// (the exe's `s + 0x100` / `s + 0x1a0` areas).
struct Rebuild {
    track: u32,
    /// The state of the 1n function (0 for the Nm function, which has no pointer stream).
    state: u32,
    /// Address of the context's word `+0xc` (the collision input).
    collision_input_at: u32,
    sector: u32,
    write: u32,
    write_limit: u32,
    read: u32,
    read_limit: u32,
    previous: u32,
    out_index: u32,
    in_index: i32,
    used: u32,
    in_temp: bool,
    temp: u32,
    temp_limit: u32,
    count: u32,
}

impl Rebuild {
    /// The set-up: the first input sector is read, an output sector is taken (the collision
    /// input's cached sector when there is one).
    fn start(
        e: &mut Engine,
        track: u32,
        collision_input_at: u32,
        temp: u32,
        state: u32,
    ) -> Rebuild {
        let collision_input = e.mem.u32(collision_input_at);
        let first = e.mem.u32(e.mem.u32(track));
        let mut sector = e.mem.u32(collision_input + 0x64);
        if sector == 0 {
            sector = take_sector(e);
        }
        e.mem.set_u32(collision_input + 0x64, 0);
        let first_used = e.mem.u32(first);
        Rebuild {
            track,
            state,
            collision_input_at,
            sector,
            write: sector.wrapping_add(0x10),
            write_limit: sector.wrapping_add(0x200),
            read: first + 0x10,
            read_limit: first.wrapping_add(first_used).wrapping_add(0x10),
            previous: first,
            out_index: 0,
            in_index: 1,
            used: 0,
            in_temp: false,
            temp,
            temp_limit: temp + 0x80,
            count: 0,
        }
    }

    /// The pointer stream's records whose pointer at `+4` lies in `bytes` bytes at `temp` are
    /// moved to `dest` (both words).
    fn move_records(&self, e: &mut Engine, dest: u32, bytes: i32) {
        let stream = e.mem.u32(self.state + 0x3080);
        if stream == 0 {
            return;
        }
        let mut record = stream.wrapping_add(0x408);
        if record < e.mem.u32(stream) {
            loop {
                let delta = e.mem.u32(record + 4).wrapping_sub(self.temp) as i32;
                if delta >= 0 && delta < bytes {
                    e.mem.set_u32(record + 4, (delta as u32).wrapping_add(dest));
                    let word = e
                        .mem
                        .u32(record + 8)
                        .wrapping_add(dest.wrapping_sub(self.temp));
                    e.mem.set_u32(record + 8, word);
                }
                record += 0xc;
                if record >= e.mem.u32(stream) {
                    break;
                }
            }
        }
    }

    /// Appends the entries collected in the scratch area to the output sector (`bytes` of them).
    fn flush_to_sector(&mut self, e: &mut Engine, bytes: u32) {
        let dest = self.sector.wrapping_add(self.used).wrapping_add(0x10);
        copy_units(e, self.temp, dest, (bytes as i32) >> 4);
        self.used = self.used.wrapping_add(bytes);
        if self.state != 0 {
            self.move_records(e, dest, bytes as i32);
        }
    }

    /// The end of an entry's handling: moves on to the next input sector when this one is used
    /// up (finishing the track when there are none), and makes room in the output sector.
    fn end_entry(&mut self, e: &mut Engine) -> Flow {
        if self.read == self.read_limit {
            let collision_input = e.mem.u32(self.collision_input_at);
            if e.mem.u32(collision_input + 0x64) == 0 {
                e.mem.set_u32(collision_input + 0x64, self.previous);
            } else {
                router_free(e, self.previous, SECTOR_SIZE);
            }
            if self.in_index >= e.mem.i32(self.track + 4) {
                self.finish(e);
                return Flow::Finished;
            }
            let sector = e
                .mem
                .u32(e.mem.u32(self.track).wrapping_add(4 * self.in_index as u32));
            self.read = sector + 0x10;
            self.read_limit = sector.wrapping_add(e.mem.u32(sector)).wrapping_add(0x10);
            self.previous = sector;
            self.in_index += 1;
        }
        if self.write.wrapping_add(0x80) <= self.write_limit {
            return Flow::Continue;
        }
        if !self.in_temp {
            self.used = self.write.wrapping_sub(self.sector).wrapping_sub(0x10);
            self.in_temp = true;
            self.write = self.temp;
            self.write_limit = self.temp_limit;
            return Flow::Continue;
        }
        let bytes = self.write.wrapping_sub(self.temp);
        if (bytes as i32) <= 0x1f0 - self.used as i32 {
            self.flush_to_sector(e, bytes);
            if (self.used as i32) < 0x1f0 {
                self.write = self.temp;
                return Flow::Continue;
            }
            self.in_temp = false;
        }
        // The output sector is full: store it and start the next one.
        e.mem.set_u32(self.sector, self.used);
        if (self.out_index as i32) >= self.in_index {
            let allocator = router_allocator(e);
            let size = e.mem.u32(self.track + 4);
            let new_size = size.wrapping_add(1);
            let shift = size.wrapping_sub(self.in_index as u32);
            let capacity = e.mem.u32(self.track + 8) & 0x3fff_ffff;
            if (capacity as i32) < new_size as i32 {
                let mut wanted = capacity.wrapping_mul(2);
                if (new_size as i32) >= wanted as i32 {
                    wanted = new_size;
                }
                e.call(ARRAY_RESERVE, &args![allocator, self.track, wanted, 4u32]);
            }
            let source = e.mem.u32(self.track) + 4 * self.in_index as u32;
            e.call(MEM_MOVE, &args![source + 4, source, shift * 4]);
            self.in_index += 1;
            e.mem.set_u32(self.track + 4, new_size);
        }
        let data = e.mem.u32(self.track);
        e.mem.set_u32(data + 4 * self.out_index, self.sector);
        self.out_index += 1;
        let collision_input = e.mem.u32(self.collision_input_at);
        let mut sector = e.mem.u32(collision_input + 0x64);
        if sector == 0 {
            sector = take_sector(e);
        }
        self.sector = sector;
        self.write = sector.wrapping_add(0x10);
        e.mem.set_u32(collision_input + 0x64, 0);
        self.write_limit = sector.wrapping_add(0x200);
        if self.in_temp {
            copy_units(e, self.temp, self.write, (bytes as i32) >> 4);
            if self.state != 0 {
                self.move_records(e, self.write, bytes as i32);
            }
            self.write = self.write.wrapping_add(bytes);
            self.in_temp = false;
        }
        Flow::Continue
    }

    /// The last output sector is stored in the track and the profiling record is written.
    fn finish(&mut self, e: &mut Engine) {
        let used;
        if self.in_temp {
            let bytes = self.write.wrapping_sub(self.temp);
            let dest = self.sector.wrapping_add(self.used).wrapping_add(0x10);
            copy_units(e, self.temp, dest, (bytes as i32) >> 4);
            if self.state != 0 {
                let stream = e.mem.u32(self.state + 0x3080);
                if stream != 0 {
                    let cursor = e.mem.u32(stream);
                    if cursor > stream.wrapping_add(0x408) {
                        let delta = e.mem.u32(cursor - 8).wrapping_sub(self.temp) as i32;
                        if delta >= 0 && delta < bytes as i32 {
                            e.mem.set_u32(cursor - 8, (delta as u32).wrapping_add(dest));
                            let word = e
                                .mem
                                .u32(cursor - 4)
                                .wrapping_add(dest.wrapping_sub(self.temp));
                            e.mem.set_u32(cursor - 4, word);
                        }
                    }
                }
            }
            used = self.used.wrapping_add(bytes);
        } else {
            used = self.write.wrapping_sub(self.sector).wrapping_sub(0x10);
        }
        e.mem.set_u32(self.sector, used);
        let size = self.out_index + 1;
        let allocator = router_allocator(e);
        let capacity = e.mem.u32(self.track + 8) & 0x3fff_ffff;
        if (capacity as i32) < size as i32 {
            let mut wanted = capacity.wrapping_mul(2);
            if (size as i32) >= wanted as i32 {
                wanted = size;
            }
            e.call(ARRAY_RESERVE, &args![allocator, self.track, wanted, 4u32]);
        }
        let data = e.mem.u32(self.track);
        e.mem.set_u32(self.track + 4, size);
        e.mem.set_u32(data + 4 * self.out_index, self.sector);
        let index: u32 = e.global(MONITOR_SLOT);
        let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
        let cursor = e.mem.u32(stream + 4);
        if cursor < e.mem.u32(stream + 0xc) {
            e.mem.set_u32(cursor, MINIMUM_TIME_LABEL);
            e.mem
                .set_f32(cursor + 4, f64::from(self.count as i32) as f32);
            e.mem.set_u32(stream + 4, cursor + 8);
        }
    }
}

/// The velocity-corrected offset of the two motion states over `step` (the exe's SSE/x87 mix):
/// `(x[0x40..] - x[0x50..]) * a + (y[0x50..] - y[0x40..]) * b` per lane with `a = step * x.f[0x5c]`
/// and `b = step * y.f[0x5c]` (both rounded to `float`), and a fourth lane of
/// `x.f[0x9c] * a * x.f[0xa0] + y.f[0x9c] * b * y.f[0xa0]` (computed in `f64`).
pub(super) fn swept_offset(e: &Engine, x: u32, y: u32, step: f32) -> Vec4 {
    let a = (f64::from(e.mem.f32(x + 0x5c)) * f64::from(step)) as f32;
    let b = (f64::from(step) * f64::from(e.mem.f32(y + 0x5c))) as f32;
    let (x40, x50) = (load_vec(e, x + 0x40), load_vec(e, x + 0x50));
    let (y40, y50) = (load_vec(e, y + 0x40), load_vec(e, y + 0x50));
    let mut vector: Vec4 =
        std::array::from_fn(|lane| (x40[lane] - x50[lane]) * a + (y50[lane] - y40[lane]) * b);
    vector[3] = (f64::from(e.mem.f32(x + 0x9c)) * f64::from(a) * f64::from(e.mem.f32(x + 0xa0))
        + f64::from(e.mem.f32(y + 0x9c)) * f64::from(b) * f64::from(e.mem.f32(y + 0xa0)))
        as f32;
    vector
}

/// The dot product of the entry's vector at `vec` and the vector at `vector` the exe's way
/// (`MULPS`, then `(m2 + w) + (m1 + m0)` with `w` the vector's fourth lane).
pub(super) fn entry_dot(e: &Engine, vec: u32, vector: u32) -> f32 {
    let a = load_vec(e, vec);
    let v = load_vec(e, vector);
    (a[2] * v[2] + v[3]) + (a[1] * v[1] + a[0] * v[0])
}

/// The sweep records the agent functions appended to the state's list (0x30 bytes each, from
/// `cursor` up to the list's end `[state]`): the position moves along the velocity by its fourth
/// lane and the velocity is mirrored.
fn advance_records(e: &mut Engine, cursor: u32, state: u32) {
    let mut record = cursor;
    if record < e.mem.u32(state) {
        loop {
            let velocity = load_vec(e, record + 0x10);
            let position = load_vec(e, record);
            let weight = velocity[3];
            let moved: Vec4 = std::array::from_fn(|lane| weight * velocity[lane] + position[lane]);
            store_vec(e, record, moved);
            store_flipped(e, record + 0x10, record + 0x10);
            record += 0x30;
            if record >= e.mem.u32(state) {
                break;
            }
        }
    }
}

/// When the state's float at `+0x3030` no longer equals the saved bits `saved` (an unordered
/// comparison counts as different), mirrors the state's vector at `+0x3020` and calls `00d2eb50`.
fn toggle_state(e: &mut Engine, state: u32, saved: u32) {
    if f32::from_bits(saved) == e.mem.f32(state + 0x3030) {
        return;
    }
    store_flipped(e, state + 0x3020, state + 0x3020);
    e.call(STATE_REORDER, &args![state + 0x3040]);
}

/// The frame of `hkAgent1nMachine_Process`.
struct One {
    s: u32,
    provider: u32,
    state: u32,
    /// The dispatcher (the first word of the collision input; the exe's `s + 0x24`).
    dispatcher: u32,
    /// The key of the entry being handled (`s + 0x28`).
    key: u32,
    rb: Rebuild,
}

impl One {
    /// The context copy at `s + 0x70`.
    fn ctx(&self) -> u32 {
        self.s + 0x70
    }

    /// The body for the entry's key from the provider (slot `0x14`, into the buffer at
    /// `s + 0x540`), stored with the key in the record at `s + 0x44`.
    fn fetch_body(&mut self, e: &mut Engine) {
        let body = e
            .vcall(self.provider, 0x14, &args![self.key, self.s + 0x540])
            .u32();
        e.mem.set_u32(self.s + 0x44, body);
        e.mem.set_u32(self.s + 0x48, self.key);
    }

    /// Records where the agent function that returned `end` left off in the entry's size byte.
    fn set_size(&mut self, e: &mut Engine, entry: u32, end: u32) {
        self.rb.write = end;
        e.mem
            .set_u8(entry + 3, (end as u8).wrapping_sub(entry as u8));
    }

    /// Type 2: the agent function at `+0x16dc` is called with the context, the entry, its data
    /// (`+0x10`), 0 and the state.
    fn type2(&mut self, e: &mut Engine) {
        let entry = self.rb.write;
        self.fetch_body(e);
        let function = dispatcher_function(e, entry, self.dispatcher, 0x16dc);
        let end = e
            .call(
                function,
                &args![self.ctx(), entry, entry + 0x10, 0u32, self.state],
            )
            .u32();
        self.set_size(e, entry, end);
    }

    /// Type 3: like type 2 with the mirrored context (at `s + 0x420`), after which the sweep
    /// records the function appended are advanced and the state's direction is mirrored when its
    /// float at `+0x3030` changed.
    fn type3(&mut self, e: &mut Engine) {
        let (s, entry, state) = (self.s, self.rb.write, self.state);
        e.mem.set_u32(s + 0x428, 0);
        hk_agent1n_machine_flip_input(e, Ptr::new(s + 0x420), Ptr::new(self.ctx()));
        self.fetch_body(e);
        copy_float(e, state + 0x3030, s + 0x1c);
        let cursor = e.mem.u32(state);
        let function = dispatcher_function(e, entry, self.dispatcher, 0x16dc);
        let end = e
            .call(
                function,
                &args![s + 0x420, entry, entry + 0x10, 0u32, state],
            )
            .u32();
        self.set_size(e, entry, end);
        advance_records(e, cursor, state);
        let saved = e.mem.u32(s + 0x1c);
        toggle_state(e, state, saved);
    }

    /// The entry is complete: it is passed over when its byte `+2` is 0, otherwise the agent
    /// function at `+0x16b8` is called with the entry, its data (`+0x20`), `fourth` and the
    /// state's word `+4`.
    fn pass_entry(&mut self, e: &mut Engine, fourth: u32) {
        let entry = self.rb.write;
        if e.mem.u8(entry + 2) == 0 {
            self.rb.write = entry + u32::from(e.mem.u8(entry + 3));
            self.rb.count += 1;
        } else {
            let state_word = e.mem.u32(self.state + 4);
            let function = dispatcher_function(e, entry, self.dispatcher, 0x16b8);
            let end = e
                .call(function, &args![entry, entry + 0x20, fourth, state_word])
                .u32();
            self.rb.count += 1;
            self.set_size(e, entry, end);
        }
    }

    /// Type 4 (agent data at `+0x20`): the entry's time (float at `+0xc`) is compared with the
    /// collision input's times (floats at `+0x50` and `+0x54`) and the entry's vector (`+0x10`)
    /// is brought up to the later time.
    fn type4(&mut self, e: &mut Engine) {
        let (s, entry) = (self.s, self.rb.write);
        let ctx = self.ctx();
        let mut collision_input = e.mem.u32(ctx + 0xc);
        let time = e.mem.f32(entry + 0xc);
        let mut pending: Option<f32> = None;
        let mut update = false;
        if time == e.mem.f32(collision_input + 0x50) {
            update = true;
        } else if e.mem.f32(collision_input + 0x54) <= time {
            let step = (f64::from(time) - f64::from(e.mem.f32(collision_input + 0x54))) as f32;
            let x = e.mem.u32(e.mem.u32(ctx) + 8);
            let y = e.mem.u32(e.mem.u32(ctx + 4) + 8);
            let vector = swept_offset(e, x, y, step);
            store_vec(e, s + 0xf0, vector);
            update = true;
        } else {
            let object = e.mem.u32(collision_input + 0x60);
            if e.mem.u32(object + 0x10) == 0 {
                copy_float(e, collision_input + 0x54, entry + 0xc);
                clear_vec(e, entry + 0x10);
                let scaled = (f64::from(e.mem.f32(object + 0x18)) * e.mem.f64(TENTH)) as f32;
                e.mem.set_f32(entry + 0x1c, scaled);
                pending = Some(scaled);
            } else {
                if e.mem.u8(s + 0x1a) == 0 {
                    let transformed = e.mem.u8(s + 0x18);
                    e.mem.set_u8(s + 0x1a, u8::from(transformed == 0));
                    hk_agent1n_machine_init_input_at_time(
                        e,
                        Ptr::new(ctx),
                        Ptr::new(s + 0x4a0),
                        Ptr::new(s + 0x300),
                    );
                }
                self.fetch_body(e);
                let record = e.mem.u32(ctx + 4);
                copy_words(e, record, s + 0x4b0, 2);
                let function = dispatcher_function(e, entry, self.dispatcher, 0x16d8);
                e.call(
                    function,
                    &args![s + 0x300, entry, entry + 0x20, entry + 0x10],
                );
                collision_input = e.mem.u32(ctx + 0xc);
                update = true;
            }
        }
        if update {
            copy_float(e, collision_input + 0x54, entry + 0xc);
            let dot = entry_dot(e, entry + 0x10, ctx + 0x70);
            let remaining = (f64::from(e.mem.f32(entry + 0x1c)) - f64::from(dot)) as f32;
            let collision_input = e.mem.u32(ctx + 0xc);
            if e.mem.f32(collision_input + 8) <= remaining {
                e.mem.set_f32(entry + 0x1c, remaining);
                // The contact manager word of the input (00da5670 `MOV ECX,[ESP+0x84]`).
                let fourth = e.mem.u32(ctx + 0x10);
                self.pass_entry(e, fourth);
                return;
            }
            pending = Some(remaining);
        }
        let Some(value) = pending else { return };
        e.mem.set_f32(s + 0xd0, value);
        self.fetch_body(e);
        let function = dispatcher_function(e, entry, self.dispatcher, 0x16dc);
        let end = e
            .call(
                function,
                &args![ctx, entry, entry + 0x20, entry + 0x10, self.state],
            )
            .u32();
        self.set_size(e, entry, end);
    }

    /// Type 5 (agent data at `+0x20`): type 4 for the mirrored context (at `s + 0x180`), with the
    /// sweep records and the state's direction handled like type 3.
    fn type5(&mut self, e: &mut Engine) {
        let (s, entry, state) = (self.s, self.rb.write, self.state);
        let ctx = self.ctx();
        e.mem.set_u32(s + 0x188, 0);
        hk_agent1n_machine_flip_input(e, Ptr::new(s + 0x180), Ptr::new(ctx));
        let time = e.mem.f32(entry + 0xc);
        let mut collision_input = e.mem.u32(s + 0x18c);
        let mut pending: Option<f32> = None;
        let mut update = false;
        if time == e.mem.f32(collision_input + 0x50) {
            update = true;
        } else {
            let object = e.mem.u32(collision_input + 0x60);
            if e.mem.u32(object + 0x10) != 0 {
                if e.mem.u8(s + 0x19) == 0 {
                    let transformed = e.mem.u8(s + 0x18);
                    e.mem.set_u8(s + 0x19, u8::from(transformed == 0));
                    hk_agent1n_machine_init_input_at_time(
                        e,
                        Ptr::new(s + 0x180),
                        Ptr::new(s + 0x200),
                        Ptr::new(s + 0x2a0),
                    );
                    let record = e.mem.u32(s + 0x184);
                    copy_words(e, record, s + 0x210, 2);
                }
                self.fetch_body(e);
                let record = e.mem.u32(ctx + 4);
                copy_words(e, record, s + 0x200, 2);
                let function = dispatcher_function(e, entry, self.dispatcher, 0x16d8);
                e.call(
                    function,
                    &args![s + 0x2a0, entry, entry + 0x20, entry + 0x10],
                );
                collision_input = e.mem.u32(s + 0x18c);
                update = true;
            } else {
                copy_float(e, collision_input + 0x54, entry + 0xc);
                clear_vec(e, entry + 0x10);
                let scaled = (f64::from(e.mem.f32(object + 0x18)) * e.mem.f64(HALF)) as f32;
                e.mem.set_f32(entry + 0x1c, scaled);
                pending = Some(scaled);
            }
        }
        if update {
            copy_float(e, collision_input + 0x54, entry + 0xc);
            let dot = entry_dot(e, entry + 0x10, s + 0x1f0);
            let remaining = (f64::from(e.mem.f32(entry + 0x1c)) - f64::from(dot)) as f32;
            let collision_input = e.mem.u32(s + 0x18c);
            if e.mem.f32(collision_input + 8) <= remaining {
                e.mem.set_f32(entry + 0x1c, remaining);
                let fourth = e.mem.u32(s + 0x190);
                self.pass_entry(e, fourth);
                return;
            }
            pending = Some(remaining);
        }
        let Some(value) = pending else { return };
        e.mem.set_f32(s + 0x1e0, value);
        self.fetch_body(e);
        copy_float(e, state + 0x3030, s + 0x10);
        let cursor = e.mem.u32(state);
        let function = dispatcher_function(e, entry, self.dispatcher, 0x16dc);
        let end = e
            .call(
                function,
                &args![s + 0x180, entry, entry + 0x20, entry + 0x10, state],
            )
            .u32();
        self.set_size(e, entry, end);
        advance_records(e, cursor, state);
        let saved = e.mem.u32(s + 0x10);
        toggle_state(e, state, saved);
    }

    /// Type 6: the object at the entry's `+0x10` is asked (slot `0x18`) with the context's words
    /// `+0`, `+4` and `+0xc` and the state; the entry takes 0x20 bytes.
    fn type6(&mut self, e: &mut Engine) {
        let entry = self.rb.write;
        let ctx = self.ctx();
        self.fetch_body(e);
        let object = e.mem.u32(entry + 0x10);
        let (ctx0, ctx1, ctx3) = (e.mem.u32(ctx), e.mem.u32(ctx + 4), e.mem.u32(ctx + 0xc));
        e.vcall(object, 0x18, &args![ctx0, ctx1, ctx3, self.state]);
        self.rb.write = entry + 0x20;
    }

    /// Types 10 to 14: the transform shapes of the bodies are resolved (the record at `s + 0x44`
    /// is replaced by the result) and the relative transform is stored at `ctx + 0x20`.
    fn transformed(&mut self, e: &mut Engine) {
        let s = self.s;
        let ctx = self.ctx();
        self.fetch_body(e);
        e.mem.set_u8(s + 0x33, 0);
        let record = e.mem.u32(ctx + 4);
        let result = e
            .call(
                PROCESS_TRANSFORMED_SHAPES,
                &args![record, s + 0x360, s + 0x740, 4u32, s + 0x33],
            )
            .u32();
        e.mem.set_u32(ctx + 4, result);
        let first = e.mem.u32(e.mem.u32(ctx) + 8);
        let second = e.mem.u32(result + 8);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![ctx + 0x20, first, second],
        );
    }

    /// Handles the entry at the write position by its type.
    fn dispatch(&mut self, e: &mut Engine) {
        let s = self.s;
        let mut kind = u32::from(e.mem.u8(self.rb.write));
        e.mem.set_u8(s + 0x18, 0);
        loop {
            if kind > 0xe {
                return;
            }
            match kind {
                0 | 1 => self.rb.write += 0x10,
                2 => self.type2(e),
                3 => self.type3(e),
                4 => self.type4(e),
                5 => self.type5(e),
                6 => self.type6(e),
                7..=9 => {}
                _ => {
                    self.transformed(e);
                    kind &= 0xf7;
                    e.mem.set_u8(s + 0x18, 1);
                    e.mem.set_u8(s + 0x19, 0);
                    e.mem.set_u8(s + 0x1a, 0);
                    continue;
                }
            }
            return;
        }
    }

    /// A new entry for the key (the keys list has a key the track has no entry for): the
    /// provider (slot 4 of the object at `[collision input + 0xc] + 0xc`) says whether one is
    /// wanted; it is created with the agent function at `+0x16b0` of the agent type picked from
    /// the dispatcher's table. Returns false when the entry is skipped.
    fn create(&mut self, e: &mut Engine, input: u32, keys: &mut u32) -> bool {
        let (s, entry) = (self.s, self.rb.write);
        let ctx = self.ctx();
        let collision_input = e.mem.u32(ctx + 0xc);
        let holder = e.mem.u32(collision_input + 0xc) + 0xc;
        let (input0, input1) = (e.mem.u32(input), e.mem.u32(input + 4));
        let wanted = e
            .vcall(
                holder,
                4,
                &args![
                    s + 0x67,
                    collision_input,
                    input0,
                    input1,
                    self.provider,
                    self.key
                ],
            )
            .u32();
        if e.mem.u8(wanted) == 0 {
            e.mem.set_u32(entry, 0);
            e.mem.set_u32(entry + 4, 0xffff_ffff);
            e.mem.set_u32(entry + 8, self.key);
            e.mem.set_u8(entry + 3, 0x10);
            *keys += 4;
            return true;
        }
        if e.mem.u32(SKIP_CREATE) == 1 {
            *keys += 4;
            return false;
        }
        self.fetch_body(e);
        let ctx0 = e.mem.u32(ctx);
        let ctx0_shape = e.mem.u32(ctx0);
        e.mem.set_u8(s + 0x1b, 0);
        let record = e.mem.u32(ctx + 4);
        let result = e
            .call(
                PROCESS_TRANSFORMED_SHAPES,
                &args![record, s + 0x360, s + 0x740, 4u32, s + 0x1b],
            )
            .u32();
        e.mem.set_u32(ctx + 4, result);
        let result_shape = e.mem.u32(result);
        let first = e.mem.u32(e.mem.u32(ctx) + 8);
        let second = e.mem.u32(result + 8);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![ctx + 0x20, first, second],
        );
        e.mem.set_u32(entry + 4, 0xffff_ffff);
        e.mem.set_u32(entry + 8, self.key);
        let collision_input = e.mem.u32(ctx + 0xc);
        let index = (e.mem.u32(ctx0_shape + 0xc) << 5).wrapping_add(e.mem.u32(result_shape + 0xc));
        let table = if e.mem.u32(collision_input + 0x14) == 0 {
            0xeb0
        } else {
            0x12b0
        };
        let agent = e
            .mem
            .u8(self.dispatcher.wrapping_add(index).wrapping_add(table));
        e.mem.set_u8(entry + 1, agent);
        e.mem.set_u8(entry + 2, 0);
        e.mem.set_u32(s + 0x3a8, 0);
        let record = self.dispatcher.wrapping_add(u32::from(agent) * 0x50);
        let (mut agent_input, mut flipped) = (ctx, 0u8);
        if e.mem.u32(record + 0x16f0) == 2 {
            agent_input = s + 0x3a0;
            flipped = 1;
            hk_agent1n_machine_flip_input(e, Ptr::new(s + 0x3a0), Ptr::new(ctx));
        }
        let changed = e.mem.u8(s + 0x1b);
        let end = if e.mem.u32(record + 0x16d8) != 0 {
            copy_float(e, WARP_MISMATCH_TIME, entry + 0xc);
            e.mem.set_u8(entry, (flipped + 4) | changed);
            clear_vec(e, entry + 0x10);
            let function = e.mem.u32(record + 0x16b0);
            e.call(function, &args![agent_input, entry, entry + 0x20])
                .u32()
        } else {
            e.mem.set_u8(entry, (flipped + 2) | changed);
            let function = e.mem.u32(record + 0x16b0);
            e.call(function, &args![agent_input, entry, entry + 0x10])
                .u32()
        };
        e.mem
            .set_u8(entry + 3, (end as u8).wrapping_sub(entry as u8));
        *keys += 4;
        true
    }

    /// Reads the entry at the read position into the write position (the keys list tells which
    /// entry is wanted) and handles it.
    fn entry(&mut self, e: &mut Engine, input: u32, keys: &mut u32) {
        let ctx = self.ctx();
        let read = self.rb.read;
        let write = self.rb.write;
        if *keys == 0 {
            let size = u32::from(e.mem.u8(read + 3));
            copy_units(e, read, write, (size >> 4) as i32);
            self.key = e.mem.u32(read + 8);
            self.rb.read = read + size;
        } else {
            let wanted = e.mem.u32(*keys);
            self.key = wanted;
            let found = e.mem.u32(read + 8);
            if wanted == found {
                let size = u32::from(e.mem.u8(read + 3));
                copy_units(e, read, write, (size >> 4) as i32);
                self.rb.read = read + size;
                *keys += 4;
            } else if wanted > found {
                // An entry nobody asked for: destroyed.
                let data = if e.mem.u8(read) & 6 == 4 {
                    read + 0x20
                } else {
                    read + 0x10
                };
                self.rb.read = read + u32::from(e.mem.u8(read + 3));
                let state_word = e.mem.u32(self.state + 4);
                let function = dispatcher_function(e, read, self.dispatcher, 0x16b4);
                let fourth = e.mem.u32(ctx + 0x10);
                e.call(
                    function,
                    &args![read, data, fourth, state_word, self.dispatcher],
                );
                return;
            } else if !self.create(e, input, keys) {
                return;
            }
        }
        self.dispatch(e);
    }
}

/// Clears the 16 bytes at `at` (`MOVAPS` of a zeroed register).
fn clear_vec(e: &mut Engine, at: u32) {
    for word in 0..4 {
        e.mem.set_u32(at + 4 * word, 0);
    }
}

// Translated from 00da4aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgent1nMachine_Process` (named in the engine map, cdecl): rebuilds `track` (an array of
/// sector pointers) from its entries. `input` is the 32-word input record (copied into the
/// frame), `provider` an object whose slot `0x14` gives the body for a key (into a `0x200`-byte
/// buffer) and whose slot `0x18` says whether a pointer stream is wanted, `keys` a sorted list of
/// keys (or null: every entry is kept) and `state` the output state (the pointer stream at
/// `+0x3080`, the sweep list at `+0`, the direction at `+0x3020` and its float at `+0x3030`).
///
/// Every entry is copied to the output; with a key list entries the list lacks are destroyed
/// (dispatcher function `+0x16b4`) and keys without an entry get a new one (asked of slot 4 of
/// the collision input's object, built by function `+0x16b0`). The agent functions at `+0x16dc`
/// (and `+0x16d8`, `+0x16b8` for the swept types 4 and 5) update the entries and may resize
/// them. When the pointer stream holds records afterwards the weld function (`00da3930`) runs.
pub fn hk_agent1n_machine_process(
    e: &mut Engine,
    track: Ptr,
    input: Ptr,
    provider: Ptr,
    keys: Ptr,
    state: Ptr,
) {
    let (track, input, provider, state) =
        (track.addr(), input.addr(), provider.addr(), state.addr());
    let mut keys = keys.addr();
    e.with_stack(0x1a10, |e, frame| {
        let s = frame.addr();
        let old_stream = e.mem.u32(state + 0x3080);
        e.mem.set_u32(s + 0x68, old_stream);
        let wants_stream = old_stream == 0 && e.vcall(provider, 0x18, &args![]).u32() as u8 != 0;
        if wants_stream {
            e.mem.set_u32(s + 0xa08, s + 0xe10);
            e.mem.set_u32(s + 0xa0c, s + 0xa10);
            e.mem.set_u32(state + 0x3080, s + 0xa08);
        } else {
            e.mem.set_u32(state + 0x3080, 0);
        }
        let ctx = s + 0x70;
        copy_words(e, input, ctx, 32);
        let collision_input = e.mem.u32(ctx + 0xc);
        let dispatcher = e.mem.u32(collision_input);
        let second = e.mem.u32(input + 4);
        e.mem.set_u32(s + 0x50, second);
        let second_word = e.mem.u32(second + 8);
        e.mem.set_u32(s + 0x4c, second_word);
        e.mem.set_u32(ctx + 4, s + 0x44);
        let rb = Rebuild::start(e, track, ctx + 0xc, s + 0x100, state);
        e.mem.set_u32(s + 0x308, 0);
        e.mem.set_u32(s + 0x2a8, 0);
        e.mem.set_u8(s + 0x1a, 0);
        e.mem.set_u8(s + 0x19, 0);
        let mut run = One {
            s,
            provider,
            state,
            dispatcher,
            key: 0,
            rb,
        };
        loop {
            e.mem.set_u32(ctx + 4, s + 0x44);
            copy_words(e, input + 0x20, ctx + 0x20, 16);
            run.entry(e, input, &mut keys);
            if matches!(run.rb.end_entry(e), Flow::Finished) {
                break;
            }
        }
        copy_words(e, input + 0x20, ctx + 0x20, 16);
        let second = e.mem.u32(input + 4);
        e.mem.set_u32(ctx + 4, second);
        let stream = e.mem.u32(state + 0x3080);
        if stream != 0 && e.mem.u32(stream) > stream.wrapping_add(0x408) {
            e.call(WELD, &args![ctx, provider, state]);
        }
        let saved = e.mem.u32(s + 0x68);
        e.mem.set_u32(state + 0x3080, saved);
    });
}

/// The frame of `hkAgentNmMachine_Process`: the 1n function with two body providers and keys of
/// two words (`key0` for provider `b` and the entry's word `+4`, `key1` for provider `a` and the
/// entry's word `+8`). The bodies are kept in two records of the frame (`s + 0x4c` for `a` and
/// `s + 0x5c` for `b`, each `(body, key, collidable word +8, collidable)`) and fetched again
/// only when the key changes.
struct Many {
    s: u32,
    provider_b: u32,
    provider_a: u32,
    state: u32,
    /// The dispatcher (the first word of the collision input; the exe's `s + 0x38`).
    dispatcher: u32,
    key0: u32,
    key1: u32,
    rb: Rebuild,
}

impl Many {
    /// The context copy at `s + 0x90`.
    fn ctx(&self) -> u32 {
        self.s + 0x90
    }

    /// Fetches the bodies for the entry's two keys when they differ from the cached ones
    /// (provider `a`'s buffer is at `s + 0x7a0`, provider `b`'s at `s + 0x5a0`).
    fn refresh(&mut self, e: &mut Engine) {
        let s = self.s;
        if e.mem.u32(s + 0x24) != self.key1 {
            e.mem.set_u32(s + 0x24, self.key1);
            let body = e
                .vcall(self.provider_a, 0x14, &args![self.key1, s + 0x7a0])
                .u32();
            e.mem.set_u32(s + 0x4c, body);
            e.mem.set_u32(s + 0x50, self.key1);
        }
        if e.mem.u32(s + 0x20) != self.key0 {
            e.mem.set_u32(s + 0x20, self.key0);
            let body = e
                .vcall(self.provider_b, 0x14, &args![self.key0, s + 0x5a0])
                .u32();
            e.mem.set_u32(s + 0x5c, body);
            e.mem.set_u32(s + 0x60, self.key0);
        }
    }

    /// Records where the agent function that returned `end` left off in the entry's size byte.
    fn set_size(&mut self, e: &mut Engine, entry: u32, end: u32) {
        self.rb.write = end;
        e.mem
            .set_u8(entry + 3, (end as u8).wrapping_sub(entry as u8));
    }

    /// Type 2: see [`One::type2`].
    fn type2(&mut self, e: &mut Engine) {
        let entry = self.rb.write;
        self.refresh(e);
        let function = dispatcher_function(e, entry, self.dispatcher, 0x16dc);
        let end = e
            .call(
                function,
                &args![self.ctx(), entry, entry + 0x10, 0u32, self.state],
            )
            .u32();
        self.set_size(e, entry, end);
    }

    /// Type 3: see [`One::type3`] (the mirrored context is at `s + 0x4a0`).
    fn type3(&mut self, e: &mut Engine) {
        let (s, entry, state) = (self.s, self.rb.write, self.state);
        e.mem.set_u32(s + 0x4a8, 0);
        hk_agent1n_machine_flip_input(e, Ptr::new(s + 0x4a0), Ptr::new(self.ctx()));
        self.refresh(e);
        copy_float(e, state + 0x3030, s + 0x14);
        let cursor = e.mem.u32(state);
        let function = dispatcher_function(e, entry, self.dispatcher, 0x16dc);
        let end = e
            .call(
                function,
                &args![s + 0x4a0, entry, entry + 0x10, 0u32, state],
            )
            .u32();
        self.set_size(e, entry, end);
        advance_records(e, cursor, state);
        let saved = e.mem.u32(s + 0x14);
        toggle_state(e, state, saved);
    }

    /// The entry is complete: see [`One::pass_entry`].
    fn pass_entry(&mut self, e: &mut Engine, fourth: u32) {
        let entry = self.rb.write;
        if e.mem.u8(entry + 2) == 0 {
            self.rb.write = entry + u32::from(e.mem.u8(entry + 3));
            self.rb.count += 1;
        } else {
            let state_word = e.mem.u32(self.state + 4);
            let function = dispatcher_function(e, entry, self.dispatcher, 0x16b8);
            let end = e
                .call(function, &args![entry, entry + 0x20, fourth, state_word])
                .u32();
            self.rb.count += 1;
            self.set_size(e, entry, end);
        }
    }

    /// Type 4: see [`One::type4`].
    fn type4(&mut self, e: &mut Engine) {
        let (s, entry) = (self.s, self.rb.write);
        let ctx = self.ctx();
        let mut collision_input = e.mem.u32(ctx + 0xc);
        let time = e.mem.f32(entry + 0xc);
        let mut pending: Option<f32> = None;
        let mut update = false;
        if time == e.mem.f32(collision_input + 0x50) {
            update = true;
        } else if e.mem.f32(collision_input + 0x54) <= time {
            let step = (f64::from(time) - f64::from(e.mem.f32(collision_input + 0x54))) as f32;
            let x = e.mem.u32(e.mem.u32(ctx) + 8);
            let y = e.mem.u32(e.mem.u32(ctx + 4) + 8);
            let vector = swept_offset(e, x, y, step);
            store_vec(e, s + 0x110, vector);
            update = true;
        } else {
            let object = e.mem.u32(collision_input + 0x60);
            if e.mem.u32(object + 0x10) == 0 {
                copy_float(e, collision_input + 0x54, entry + 0xc);
                clear_vec(e, entry + 0x10);
                let scaled = (f64::from(e.mem.f32(object + 0x18)) * e.mem.f64(TENTH)) as f32;
                e.mem.set_f32(entry + 0x1c, scaled);
                pending = Some(scaled);
            } else {
                if e.mem.u8(s + 0x2d) == 0 {
                    let transformed = e.mem.u8(s + 0x2e);
                    e.mem.set_u8(s + 0x2d, u8::from(transformed == 0));
                    hk_agent1n_machine_init_input_at_time(
                        e,
                        Ptr::new(ctx),
                        Ptr::new(s + 0x3c0),
                        Ptr::new(s + 0x360),
                    );
                }
                self.refresh(e);
                let (first, second) = (e.mem.u32(ctx + 4), e.mem.u32(ctx));
                copy_words(e, first, s + 0x3d0, 2);
                copy_words(e, second, s + 0x3c0, 2);
                let function = dispatcher_function(e, entry, self.dispatcher, 0x16d8);
                e.call(
                    function,
                    &args![s + 0x360, entry, entry + 0x20, entry + 0x10],
                );
                collision_input = e.mem.u32(ctx + 0xc);
                update = true;
            }
        }
        if update {
            copy_float(e, collision_input + 0x54, entry + 0xc);
            let dot = entry_dot(e, entry + 0x10, s + 0x100);
            let remaining = (f64::from(e.mem.f32(entry + 0x1c)) - f64::from(dot)) as f32;
            let collision_input = e.mem.u32(ctx + 0xc);
            if e.mem.f32(collision_input + 8) <= remaining {
                e.mem.set_f32(entry + 0x1c, remaining);
                // The contact manager word of the input (00da5670 `MOV ECX,[ESP+0x84]`).
                let fourth = e.mem.u32(ctx + 0x10);
                self.pass_entry(e, fourth);
                return;
            }
            pending = Some(remaining);
        }
        let Some(value) = pending else { return };
        e.mem.set_f32(s + 0xf0, value);
        self.refresh(e);
        let function = dispatcher_function(e, entry, self.dispatcher, 0x16dc);
        let end = e
            .call(
                function,
                &args![ctx, entry, entry + 0x20, entry + 0x10, self.state],
            )
            .u32();
        self.set_size(e, entry, end);
    }

    /// Type 5: see [`One::type5`] (the mirrored context is at `s + 0x120`).
    fn type5(&mut self, e: &mut Engine) {
        let (s, entry, state) = (self.s, self.rb.write, self.state);
        let ctx = self.ctx();
        e.mem.set_u32(s + 0x128, 0);
        hk_agent1n_machine_flip_input(e, Ptr::new(s + 0x120), Ptr::new(ctx));
        let time = e.mem.f32(entry + 0xc);
        let mut collision_input = e.mem.u32(s + 0x12c);
        let mut pending: Option<f32> = None;
        let mut update = false;
        if time == e.mem.f32(collision_input + 0x50) {
            update = true;
        } else {
            let object = e.mem.u32(collision_input + 0x60);
            if e.mem.u32(object + 0x10) != 0 {
                if e.mem.u8(s + 0x2f) == 0 {
                    let transformed = e.mem.u8(s + 0x2e);
                    e.mem.set_u8(s + 0x2f, u8::from(transformed == 0));
                    hk_agent1n_machine_init_input_at_time(
                        e,
                        Ptr::new(s + 0x120),
                        Ptr::new(s + 0x220),
                        Ptr::new(s + 0x300),
                    );
                    let record = e.mem.u32(s + 0x124);
                    copy_words(e, record, s + 0x230, 2);
                }
                self.refresh(e);
                let (first, second) = (e.mem.u32(ctx + 4), e.mem.u32(ctx));
                copy_words(e, first, s + 0x220, 2);
                copy_words(e, second, s + 0x230, 2);
                let function = dispatcher_function(e, entry, self.dispatcher, 0x16d8);
                e.call(
                    function,
                    &args![s + 0x300, entry, entry + 0x20, entry + 0x10],
                );
                collision_input = e.mem.u32(s + 0x12c);
                update = true;
            } else {
                copy_float(e, collision_input + 0x54, entry + 0xc);
                clear_vec(e, entry + 0x10);
                let scaled = (f64::from(e.mem.f32(object + 0x18)) * e.mem.f64(HALF)) as f32;
                e.mem.set_f32(entry + 0x1c, scaled);
                pending = Some(scaled);
            }
        }
        if update {
            copy_float(e, collision_input + 0x54, entry + 0xc);
            let dot = entry_dot(e, entry + 0x10, s + 0x190);
            let remaining = (f64::from(e.mem.f32(entry + 0x1c)) - f64::from(dot)) as f32;
            let collision_input = e.mem.u32(s + 0x12c);
            if e.mem.f32(collision_input + 8) <= remaining {
                e.mem.set_f32(entry + 0x1c, remaining);
                let fourth = e.mem.u32(s + 0x130);
                self.pass_entry(e, fourth);
                return;
            }
            pending = Some(remaining);
        }
        let Some(value) = pending else { return };
        e.mem.set_f32(s + 0x180, value);
        self.refresh(e);
        copy_float(e, state + 0x3030, s + 0x18);
        let cursor = e.mem.u32(state);
        let function = dispatcher_function(e, entry, self.dispatcher, 0x16dc);
        let end = e
            .call(
                function,
                &args![s + 0x120, entry, entry + 0x20, entry + 0x10, state],
            )
            .u32();
        self.set_size(e, entry, end);
        advance_records(e, cursor, state);
        let saved = e.mem.u32(s + 0x18);
        toggle_state(e, state, saved);
    }

    /// Type 6: see [`One::type6`].
    fn type6(&mut self, e: &mut Engine) {
        let entry = self.rb.write;
        let ctx = self.ctx();
        self.refresh(e);
        let object = e.mem.u32(entry + 0x10);
        let (ctx0, ctx1, ctx3) = (e.mem.u32(ctx), e.mem.u32(ctx + 4), e.mem.u32(ctx + 0xc));
        e.vcall(object, 0x18, &args![ctx0, ctx1, ctx3, self.state]);
        self.rb.write = entry + 0x20;
    }

    /// Types 10 to 14: the transform shapes of both bodies are resolved (the records the
    /// context points at are replaced by the results) and the relative transform is stored at
    /// `ctx + 0x20`.
    fn transformed(&mut self, e: &mut Engine) {
        let s = self.s;
        let ctx = self.ctx();
        self.refresh(e);
        e.mem.set_u8(s + 0x37, 0);
        let record = e.mem.u32(ctx + 4);
        let result_a = e
            .call(
                PROCESS_TRANSFORMED_SHAPES,
                &args![record, s + 0x460, s + 0x9a0, 4u32, s + 0x37],
            )
            .u32();
        e.mem.set_u32(ctx + 4, result_a);
        let record = e.mem.u32(ctx);
        let result_b = e
            .call(
                PROCESS_TRANSFORMED_SHAPES,
                &args![record, s + 0x2c0, s + 0xc60, 4u32, s + 0x37],
            )
            .u32();
        e.mem.set_u32(ctx, result_b);
        let first = e.mem.u32(result_b + 8);
        let second = e.mem.u32(e.mem.u32(ctx + 4) + 8);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![ctx + 0x20, first, second],
        );
    }

    /// Handles the entry at the write position by its type.
    fn dispatch(&mut self, e: &mut Engine) {
        let s = self.s;
        let mut kind = u32::from(e.mem.u8(self.rb.write));
        e.mem.set_u8(s + 0x2e, 0);
        loop {
            if kind > 0xe {
                return;
            }
            match kind {
                0 | 1 => self.rb.write += 0x10,
                2 => self.type2(e),
                3 => self.type3(e),
                4 => self.type4(e),
                5 => self.type5(e),
                6 => self.type6(e),
                7..=9 => {}
                _ => {
                    self.transformed(e);
                    kind &= 0xf7;
                    e.mem.set_u8(s + 0x2e, 1);
                    e.mem.set_u8(s + 0x2f, 0);
                    e.mem.set_u8(s + 0x2d, 0);
                    continue;
                }
            }
            return;
        }
    }

    /// A new entry for the key pair: see [`One::create`] (slot 0 of the object is asked with
    /// both providers and both keys). Returns false when the entry is skipped.
    fn create(&mut self, e: &mut Engine, input: u32, keys: &mut u32) -> bool {
        let (s, entry) = (self.s, self.rb.write);
        let ctx = self.ctx();
        let collision_input = e.mem.u32(ctx + 0xc);
        let holder = e.mem.u32(collision_input + 0xc) + 0xc;
        let (input0, input1) = (e.mem.u32(input), e.mem.u32(input + 4));
        let wanted = e
            .vcall(
                holder,
                0,
                &args![
                    s + 0x8b,
                    collision_input,
                    input0,
                    input1,
                    self.provider_b,
                    self.provider_a,
                    self.key0,
                    self.key1
                ],
            )
            .u32();
        if e.mem.u8(wanted) == 0 {
            e.mem.set_u32(entry, 0);
            e.mem.set_u32(entry + 8, self.key1);
            e.mem.set_u32(entry + 4, self.key0);
            e.mem.set_u8(entry + 3, 0x10);
            *keys += 8;
            return true;
        }
        if e.mem.u32(SKIP_CREATE) == 1 {
            *keys += 8;
            return false;
        }
        self.refresh(e);
        e.mem.set_u8(s + 0x1f, 0);
        let record = e.mem.u32(ctx + 4);
        let result_a = e
            .call(
                PROCESS_TRANSFORMED_SHAPES,
                &args![record, s + 0x460, s + 0x9a0, 4u32, s + 0x1f],
            )
            .u32();
        e.mem.set_u32(ctx + 4, result_a);
        let shape_a = e.mem.u32(result_a);
        let record = e.mem.u32(ctx);
        let result_b = e
            .call(
                PROCESS_TRANSFORMED_SHAPES,
                &args![record, s + 0x2c0, s + 0xc60, 4u32, s + 0x1f],
            )
            .u32();
        e.mem.set_u32(ctx, result_b);
        let shape_b = e.mem.u32(result_b);
        let first = e.mem.u32(result_b + 8);
        let second = e.mem.u32(e.mem.u32(ctx + 4) + 8);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![ctx + 0x20, first, second],
        );
        e.mem.set_u32(entry + 8, self.key1);
        e.mem.set_u32(entry + 4, self.key0);
        let collision_input = e.mem.u32(ctx + 0xc);
        let index = (e.mem.u32(shape_b + 0xc) << 5).wrapping_add(e.mem.u32(shape_a + 0xc));
        let table = if e.mem.u32(collision_input + 0x14) == 0 {
            0xeb0
        } else {
            0x12b0
        };
        let agent = e
            .mem
            .u8(self.dispatcher.wrapping_add(index).wrapping_add(table));
        e.mem.set_u8(entry + 1, agent);
        e.mem.set_u8(entry + 2, 0);
        e.mem.set_u32(s + 0x528, 0);
        let record = self.dispatcher.wrapping_add(u32::from(agent) * 0x50);
        let (mut agent_input, mut flipped) = (ctx, 0u8);
        if e.mem.u32(record + 0x16f0) == 2 {
            agent_input = s + 0x520;
            flipped = 1;
            hk_agent1n_machine_flip_input(e, Ptr::new(s + 0x520), Ptr::new(ctx));
        }
        let changed = e.mem.u8(s + 0x1f);
        let end = if e.mem.u32(record + 0x16d8) != 0 {
            copy_float(e, WARP_MISMATCH_TIME, entry + 0xc);
            e.mem.set_u8(entry, (flipped + 4) | changed);
            clear_vec(e, entry + 0x10);
            let function = e.mem.u32(record + 0x16b0);
            e.call(function, &args![agent_input, entry, entry + 0x20])
                .u32()
        } else {
            e.mem.set_u8(entry, (flipped + 2) | changed);
            let function = e.mem.u32(record + 0x16b0);
            e.call(function, &args![agent_input, entry, entry + 0x10])
                .u32()
        };
        e.mem
            .set_u8(entry + 3, (end as u8).wrapping_sub(entry as u8));
        *keys += 8;
        true
    }

    /// Reads the entry at the read position into the write position (the keys list tells which
    /// entry is wanted) and handles it.
    fn entry(&mut self, e: &mut Engine, input: u32, keys: &mut u32) {
        let ctx = self.ctx();
        let read = self.rb.read;
        let write = self.rb.write;
        if *keys == 0 {
            let size = u32::from(e.mem.u8(read + 3));
            copy_units(e, read, write, (size >> 4) as i32);
            self.key0 = e.mem.u32(read + 4);
            self.key1 = e.mem.u32(read + 8);
            self.rb.read = read + size;
        } else {
            self.key0 = e.mem.u32(*keys);
            self.key1 = e.mem.u32(*keys + 4);
            let found0 = e.mem.u32(read + 4);
            let found1 = e.mem.u32(read + 8);
            if self.key1 == found1 && self.key0 == found0 {
                let size = u32::from(e.mem.u8(read + 3));
                copy_units(e, read, write, (size >> 4) as i32);
                self.rb.read = read + size;
                *keys += 8;
            } else if self.key1 > found1 || (self.key1 == found1 && self.key0 > found0) {
                // An entry nobody asked for: destroyed.
                let data = if e.mem.u8(read) & 6 == 4 {
                    read + 0x20
                } else {
                    read + 0x10
                };
                self.rb.read = read + u32::from(e.mem.u8(read + 3));
                let state_word = e.mem.u32(self.state + 4);
                let function = dispatcher_function(e, read, self.dispatcher, 0x16b4);
                let fourth = e.mem.u32(ctx + 0x10);
                e.call(
                    function,
                    &args![read, data, fourth, state_word, self.dispatcher],
                );
                return;
            } else if !self.create(e, input, keys) {
                return;
            }
        }
        self.dispatch(e);
    }
}

// Translated from 00da5b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNmMachine_Process` (named in the engine map, cdecl): `hkAgent1nMachine_Process` for
/// two body providers (`provider_b` for the entry's key word `+4`, `provider_a` for word `+8`)
/// and a list of key pairs (`keys`, 8 bytes each: the word for `+4`, then the word for `+8`;
/// compared with the entry's words with `+8` the more significant). It has no pointer stream and
/// no weld step; the rest follows [`hk_agent1n_machine_process`] (the context is at `s + 0x90`,
/// its words `+0` and `+4` point at the records of the two bodies).
pub fn hk_agent_nm_machine_process(
    e: &mut Engine,
    track: Ptr,
    input: Ptr,
    provider_b: Ptr,
    provider_a: Ptr,
    keys: Ptr,
    state: Ptr,
) {
    let (track, input, state) = (track.addr(), input.addr(), state.addr());
    let (provider_b, provider_a) = (provider_b.addr(), provider_a.addr());
    let mut keys = keys.addr();
    e.with_stack(0xf20, |e, frame| {
        let s = frame.addr();
        let ctx = s + 0x90;
        copy_words(e, input, ctx, 32);
        e.mem.set_u32(s + 0x20, 0xffff_ffff);
        e.mem.set_u32(s + 0x24, 0xffff_ffff);
        let collision_input = e.mem.u32(ctx + 0xc);
        let dispatcher = e.mem.u32(collision_input);
        let second = e.mem.u32(input + 4);
        e.mem.set_u32(s + 0x58, second);
        let word = e.mem.u32(second + 8);
        e.mem.set_u32(s + 0x54, word);
        e.mem.set_u32(ctx + 4, s + 0x4c);
        let first = e.mem.u32(input);
        e.mem.set_u32(s + 0x68, first);
        let word = e.mem.u32(first + 8);
        e.mem.set_u32(s + 0x64, word);
        e.mem.set_u32(ctx, s + 0x5c);
        let rb = Rebuild::start(e, track, ctx + 0xc, s + 0x1a0, 0);
        e.mem.set_u32(s + 0x368, 0);
        e.mem.set_u32(s + 0x308, 0);
        e.mem.set_u8(s + 0x2d, 0);
        e.mem.set_u8(s + 0x2f, 0);
        let mut run = Many {
            s,
            provider_b,
            provider_a,
            state,
            dispatcher,
            key0: 0,
            key1: 0,
            rb,
        };
        loop {
            e.mem.set_u32(ctx + 4, s + 0x4c);
            copy_words(e, input + 0x20, ctx + 0x20, 16);
            e.mem.set_u32(ctx, s + 0x5c);
            run.entry(e, input, &mut keys);
            if matches!(run.rb.end_entry(e), Flow::Finished) {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_4920, hk_agent1n_machine_flip_input(Ptr, Ptr)),
        entry!(
            0x00da_49b0,
            hk_agent1n_machine_init_input_at_time(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00da_4aa0,
            hk_agent1n_machine_process(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00da_5b80,
            hk_agent_nm_machine_process(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::super::hkpagent1nmachine::MEMORY_ROUTER;

    /// Doubles standing in for dispatcher functions, object slots and callees outside the file.
    const AGENT_A: u32 = 0x0900_0101;
    const AGENT_B: u32 = 0x0900_0102;
    const CREATE_FUNCTION: u32 = 0x0900_0103;
    const DESTROY_FUNCTION: u32 = 0x0900_0104;
    const PASS_FUNCTION: u32 = 0x0900_0105;
    const PREPARE_FUNCTION: u32 = 0x0900_0106;
    const BODY_SLOT_B: u32 = 0x0900_0110;
    const BODY_SLOT_A: u32 = 0x0900_0111;
    const STREAM_SLOT: u32 = 0x0900_0112;
    const CREATE_SLOT: u32 = 0x0900_0113;
    const OBJECT_SLOT: u32 = 0x0900_0114;
    const TAKE_DOUBLE: u32 = 0x0900_0120;
    const FREE_BLOCK_DOUBLE: u32 = 0x0900_0121;
    const FREE_DATA_DOUBLE: u32 = 0x0900_0122;

    /// The dispatcher's record of an agent type holds the functions at these offsets.
    const CREATE_AT: u32 = 0x16b0;
    const DESTROY_AT: u32 = 0x16b4;
    const PASS_AT: u32 = 0x16b8;
    const PREPARE_AT: u32 = 0x16d8;
    const AGENT_AT: u32 = 0x16dc;
    const FLIP_AT: u32 = 0x16f0;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.map(0x0126_8000, 0x1000);
        e
    }

    /// An engine that logs its calls.
    fn logged() -> Engine {
        let mut e = engine();
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A sector with entries `(type, agent type, size)` one after the other from `+0x10`; the
    /// used bytes are their sizes. Returns the sector and the entries' addresses.
    fn sector(e: &mut Engine, entries: &[(u8, u8, u8)]) -> (u32, Vec<u32>) {
        let at = e.mem.alloc(0x200);
        let mut offset = 0x10;
        let mut addresses = Vec::new();
        for &(kind, agent, size) in entries {
            let entry = at + offset;
            e.mem.set_u8(entry, kind);
            e.mem.set_u8(entry + 1, agent);
            e.mem.set_u8(entry + 3, size);
            addresses.push(entry);
            offset += u32::from(size);
        }
        e.mem.set_u32(at, offset - 0x10);
        (at, addresses)
    }

    /// A track (data, size, capacity word) holding `sectors`, with room for four more.
    fn track(e: &mut Engine, sectors: &[u32]) -> u32 {
        let data = e.mem.alloc(4 * sectors.len() as u32 + 32);
        for (i, sector) in sectors.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *sector);
        }
        let at = e.mem.alloc(0x10);
        e.mem.set_u32(at, data);
        e.mem.set_u32(at + 4, sectors.len() as u32);
        e.mem.set_u32(at + 8, sectors.len() as u32 + 8);
        at
    }

    /// An object whose vtable has `slots` `(byte offset, function)`.
    fn make_object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for &(offset, function) in slots {
            e.mem.set_u32(vtable + offset, function);
        }
        let at = e.mem.alloc(0x30);
        e.mem.set_u32(at, vtable);
        at
    }

    /// Doubles for the memory router: its allocator hands out zeroed blocks (slot 4) and takes
    /// blocks back (slot 8). Returns the allocator.
    fn install_allocator(e: &mut Engine) -> u32 {
        e.register_double(TAKE_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register_double(FREE_BLOCK_DOUBLE, |_, _| Ret::default());
        e.register_double(FREE_DATA_DOUBLE, |_, _| Ret::default());
        let allocator = make_object(
            e,
            &[
                (4, TAKE_DOUBLE),
                (8, FREE_BLOCK_DOUBLE),
                (0x10, FREE_DATA_DOUBLE),
            ],
        );
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        allocator
    }

    /// A double for `_reserve`: the array moves to a block of the new capacity, keeping its
    /// elements.
    fn install_reserve(e: &mut Engine) {
        e.register_double(ARRAY_RESERVE, |e, a| {
            let (array, capacity, element) = (a[1], a[2], a[3]);
            let size = e.mem.u32(array + 4);
            let block = e.mem.alloc(capacity * element);
            let old = e.mem.u32(array);
            for offset in 0..size * element {
                let byte = e.mem.u8(old + offset);
                e.mem.set_u8(block + offset, byte);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
    }

    /// The monitor stream (cursor at `+4`, limit at `+0xc`) and the TLS slot.
    fn monitor_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0xf0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        stream
    }

    /// The `(label, float)` profiling records written since the stream was made.
    fn minimum_times(e: &Engine, stream: u32) -> Vec<(u32, f32)> {
        let end = e.mem.u32(stream + 4);
        (stream + 0x40..end)
            .step_by(8)
            .map(|at| (e.mem.u32(at), e.mem.f32(at + 4)))
            .collect()
    }

    /// The entries `(type, size)` of the sector stored at `index` in the track, and its used
    /// bytes.
    fn stored_entries(e: &Engine, walked: u32, index: u32) -> (Vec<(u8, u8)>, u32) {
        let sector = e.mem.u32(e.mem.u32(walked) + 4 * index);
        let used = e.mem.u32(sector);
        let mut entries = Vec::new();
        let mut at = sector + 0x10;
        while at < sector + 0x10 + used {
            let size = e.mem.u8(at + 3);
            entries.push((e.mem.u8(at), size));
            at += u32::from(size);
        }
        (entries, used)
    }

    /// The address of the first entry of the sector stored at `index` in the track.
    fn stored_sector(e: &Engine, walked: u32, index: u32) -> u32 {
        e.mem.u32(e.mem.u32(walked) + 4 * index)
    }

    /// The constants the process functions read from the exe image.
    fn constants(e: &mut Engine) {
        for lane in 0..3 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e.mem.set_u32(SIGN_MASK + 12, 0);
        e.mem.set_f64(HALF, 0.5);
        e.mem.set_f64(TENTH, f64::from(0.1f32));
        e.mem.set_f32(WARP_MISMATCH_TIME, -1.0);
        e.mem.set_u32(SKIP_CREATE, 0);
    }

    /// What the tests build the process functions' surroundings from.
    struct World {
        allocator: u32,
        stream: u32,
        dispatcher: u32,
        collision_input: u32,
        input: u32,
        state: u32,
        list: u32,
        provider_b: u32,
        provider_a: u32,
        holder: u32,
        flags: u32,
        body_b: u32,
        body_a: u32,
        object: u32,
        shapes: [u32; 2],
        collidables: [u32; 2],
        motions: [u32; 2],
    }

    /// A world with doubles for everything outside the file: the providers hand out one body each
    /// (`body_b`, `body_a`: a shape whose type is at `+0xc`), the stream is not wanted, the object
    /// that decides on new entries says no (its flag byte `flags` is 0) and the shape resolver hands
    /// its first argument back.
    fn world(e: &mut Engine) -> World {
        let allocator = install_allocator(e);
        let stream = monitor_stream(e);
        constants(e);
        let body_b = e.mem.alloc(0x20);
        let body_a = e.mem.alloc(0x20);
        e.mem.set_u32(body_b + 0xc, 2);
        e.mem.set_u32(body_a + 0xc, 3);
        e.register_double(BODY_SLOT_B, move |_, _| Ret {
            eax: body_b,
            ..Ret::default()
        });
        e.register_double(BODY_SLOT_A, move |_, _| Ret {
            eax: body_a,
            ..Ret::default()
        });
        e.register_double(STREAM_SLOT, |_, _| Ret::default());
        let flags = e.mem.alloc(8);
        e.register_double(CREATE_SLOT, move |_, _| Ret {
            eax: flags,
            ..Ret::default()
        });
        e.register_double(PROCESS_TRANSFORMED_SHAPES, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        for double in [
            TRANSFORM_SET_INVERSE,
            TRANSFORM_SET_MUL_INVERSE_MUL,
            SWEPT_LERP2,
            WELD,
            STATE_REORDER,
        ] {
            e.register_double(double, |_, _| Ret::default());
        }
        let dispatcher = e.mem.alloc(0x2000);
        let shapes = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        let motions = [e.mem.alloc(0x200), e.mem.alloc(0x200)];
        let collidables = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        for side in 0..2 {
            e.mem.set_u32(shapes[side] + 0xc, 1);
            e.mem.set_u32(collidables[side], shapes[side]);
            e.mem.set_u32(collidables[side] + 8, motions[side]);
        }
        // The object whose word +0xc is the vtable (slot 0 and slot 4 decide on new entries).
        let holder = e.mem.alloc(0x30);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable, CREATE_SLOT);
        e.mem.set_u32(vtable + 4, CREATE_SLOT);
        e.mem.set_u32(holder + 0xc, vtable);
        let object = e.mem.alloc(0x40);
        e.mem.set_f32(object + 0x18, 4.0);
        let collision_input = e.mem.alloc(0x100);
        e.mem.set_u32(collision_input, dispatcher);
        e.mem.set_f32(collision_input + 8, 1.0);
        e.mem.set_u32(collision_input + 0xc, holder);
        e.mem.set_f32(collision_input + 0x50, 1.0);
        e.mem.set_f32(collision_input + 0x54, 0.5);
        e.mem.set_u32(collision_input + 0x60, object);
        let input = e.mem.alloc(0x80);
        e.mem.set_u32(input, collidables[0]);
        e.mem.set_u32(input + 4, collidables[1]);
        e.mem.set_u32(input + 0xc, collision_input);
        e.mem.set_u32(input + 0x10, 0x4444);
        for word in 8..0x18 {
            e.mem.set_u32(input + 4 * word, 0x2000 + word);
        }
        for (lane, value) in [1.0f32, 0.0, 0.0, 0.5].into_iter().enumerate() {
            e.mem.set_f32(input + 0x70 + 4 * lane as u32, value);
        }
        let state = e.mem.alloc(0x3100);
        let list = e.mem.alloc(0x100);
        e.mem.set_u32(state, list);
        e.mem.set_u32(state + 4, 0x5757);
        e.mem.set_f32(state + 0x3030, 2.0);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(state + 0x3020 + 4 * lane as u32, value);
        }
        let provider_b = make_object(e, &[(0x14, BODY_SLOT_B), (0x18, STREAM_SLOT)]);
        let provider_a = make_object(e, &[(0x14, BODY_SLOT_A)]);
        World {
            allocator,
            stream,
            dispatcher,
            collision_input,
            input,
            state,
            list,
            provider_b,
            provider_a,
            holder,
            flags,
            body_b,
            body_a,
            object,
            shapes,
            collidables,
            motions,
        }
    }

    /// Sets the function of the dispatcher's record for `agent` at `table`.
    fn set_function(e: &mut Engine, w: &World, agent: u32, table: u32, function: u32) {
        e.mem.set_u32(w.dispatcher + agent * 0x50 + table, function);
    }

    /// Calls `hkAgent1nMachine_Process` (no key list when `keys` is 0).
    fn run_one(e: &mut Engine, w: &World, walked: u32, keys: u32) {
        e.call(
            0x00da_4aa0,
            &args![walked, w.input, w.provider_b, keys, w.state],
        );
    }

    /// Calls `hkAgentNmMachine_Process`.
    fn run_many(e: &mut Engine, w: &World, walked: u32, keys: u32) {
        e.call(
            0x00da_5b80,
            &args![walked, w.input, w.provider_b, w.provider_a, keys, w.state],
        );
    }

    /// What an agent function double saw: its arguments, the 32 words at its first argument (the
    /// context) and the four words of the records the context's first two words point at.
    #[derive(Clone, Debug)]
    struct Seen {
        args: Vec<u32>,
        ctx: Vec<u32>,
        first: Vec<u32>,
        second: Vec<u32>,
    }

    type Agent = Rc<RefCell<Vec<Seen>>>;

    /// Registers an agent function double that logs what it saw and returns the entry plus
    /// `size`.
    fn install_agent(e: &mut Engine, address: u32, size: u32) -> Agent {
        install_sweeper(e, address, size, false, None)
    }

    /// Like [`install_agent`]; with `sweep` the function also appends a sweep record (position
    /// `(1, 2, 3, 4)`, velocity `(0.5, 1, 2, 2)`) to the state's list and with `float` it sets the
    /// state's float at `+0x3030`.
    fn install_sweeper(
        e: &mut Engine,
        address: u32,
        size: u32,
        sweep: bool,
        float: Option<f32>,
    ) -> Agent {
        let seen: Agent = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(address, move |e, a| {
            let ctx: Vec<u32> = (0..32).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            let first = (0..4).map(|i| e.mem.u32(ctx[0] + 4 * i)).collect();
            let second = (0..4).map(|i| e.mem.u32(ctx[1] + 4 * i)).collect();
            log.borrow_mut().push(Seen {
                args: a.to_vec(),
                ctx,
                first,
                second,
            });
            if sweep {
                let list = e.mem.u32(a[4]);
                for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                    e.mem.set_f32(list + 4 * lane as u32, value);
                }
                for (lane, value) in [0.5f32, 1.0, 2.0, 2.0].into_iter().enumerate() {
                    e.mem.set_f32(list + 0x10 + 4 * lane as u32, value);
                }
                e.mem.set_u32(a[4], list + 0x30);
            }
            if let Some(value) = float {
                e.mem.set_f32(a[4] + 0x3030, value);
            }
            Ret {
                eax: a[1] + size,
                ..Ret::default()
            }
        });
        seen
    }

    /// Sets the key words of the entry at `at` (`+4` and `+8`).
    fn set_keys(e: &mut Engine, at: u32, word4: u32, word8: u32) {
        e.mem.set_u32(at + 4, word4);
        e.mem.set_u32(at + 8, word8);
    }

    // ---- flipInput and initInputAtTime ----

    #[test]
    fn flip_input_swaps_the_bodies_and_mirrors_the_vector() {
        let mut e = logged();
        constants(&mut e);
        e.register_double(TRANSFORM_SET_INVERSE, |_, _| Ret::default());
        let source = e.mem.alloc(0x100);
        let destination = e.mem.alloc(0x100);
        e.mem.set_u32(source, 0x11);
        e.mem.set_u32(source + 4, 0x22);
        e.mem.set_u32(source + 0xc, 0x33);
        e.mem.set_u32(source + 0x10, 0x44);
        for (lane, value) in [1.0f32, -2.0, 0.5, 8.0].into_iter().enumerate() {
            e.mem.set_f32(source + 0x70 + 4 * lane as u32, value);
        }
        e.call(0x00da_4920, &args![destination, source]);
        assert_eq!(e.mem.u32(destination), 0x22);
        assert_eq!(e.mem.u32(destination + 4), 0x11);
        assert_eq!(e.mem.u32(destination + 0xc), 0x33);
        assert_eq!(e.mem.u32(destination + 0x10), 0x44);
        let mirrored: Vec<f32> = (0..4)
            .map(|lane| e.mem.f32(destination + 0x70 + 4 * lane))
            .collect();
        assert_eq!(mirrored, vec![-1.0, 2.0, -0.5, 8.0]);
        assert_eq!(
            calls(&e, TRANSFORM_SET_INVERSE),
            vec![vec![destination + 0x20, source + 0x20]]
        );
    }

    #[test]
    fn init_input_at_time_interpolates_both_motions_and_fills_the_block() {
        let mut e = logged();
        e.register_double(SWEPT_LERP2, |_, _| Ret::default());
        e.register_double(TRANSFORM_SET_MUL_INVERSE_MUL, |_, _| Ret::default());
        let motions = [e.mem.alloc(0x100), e.mem.alloc(0x100)];
        let bodies = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        e.mem.set_u32(bodies[0], 0xaaaa);
        e.mem.set_u32(bodies[0] + 4, 0xbbbb);
        e.mem.set_u32(bodies[0] + 8, motions[0]);
        e.mem.set_u32(bodies[1] + 8, motions[1]);
        let collision_input = e.mem.alloc(0x100);
        // The time is a signalling NaN: copied through the x87 stack it is quieted.
        e.mem.set_u32(collision_input + 0x50, 0x7f80_0001);
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source, bodies[0]);
        e.mem.set_u32(source + 4, bodies[1]);
        e.mem.set_u32(source + 0xc, collision_input);
        e.mem.set_u32(source + 0x10, 0x1234);
        let block = e.mem.alloc(0x80);
        let output = e.mem.alloc(0x40);
        e.call(0x00da_49b0, &args![source, block, output]);
        assert_eq!(
            calls(&e, SWEPT_LERP2),
            vec![
                vec![motions[0] + 0x40, 0x7fc0_0001, block + 0x20],
                vec![motions[1] + 0x40, 0x7fc0_0001, block + 0x60],
            ]
        );
        assert_eq!(e.mem.u32(output), block);
        assert_eq!(e.mem.u32(output + 4), block + 0x10);
        assert_eq!(e.mem.u32(output + 0xc), collision_input);
        assert_eq!(e.mem.u32(output + 0x10), 0x1234);
        assert_eq!(e.mem.u32(block), 0xaaaa);
        assert_eq!(e.mem.u32(block + 4), 0xbbbb);
        assert_eq!(e.mem.u32(block + 8), block + 0x20);
        assert_eq!(e.mem.u32(block + 0xc), bodies[0]);
        assert_eq!(e.mem.u32(block + 0x10 + 8), block + 0x60);
        assert_eq!(e.mem.u32(block + 0x10 + 0xc), bodies[1]);
        assert_eq!(
            calls(&e, TRANSFORM_SET_MUL_INVERSE_MUL),
            vec![vec![output + 0x20, block + 0x20, block + 0x60]]
        );
    }

    // ---- hkAgent1nMachine_Process ----

    /// A sector (see [`sector`]) whose entries have the key words `+4` and `+8` of `keys`.
    fn keyed(e: &mut Engine, entries: &[(u8, u8, u8)], keys: &[(u32, u32)]) -> (u32, Vec<u32>) {
        let (at, addresses) = sector(e, entries);
        for (entry, (word4, word8)) in addresses.iter().zip(keys) {
            set_keys(e, *entry, *word4, *word8);
        }
        (at, addresses)
    }

    /// A list of keys in memory.
    fn key_list(e: &mut Engine, words: &[u32]) -> u32 {
        let at = e.mem.alloc(4 * words.len() as u32);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, *word);
        }
        at
    }

    /// The first entry of the rebuilt track's sector `index`.
    fn first_entry(e: &Engine, walked: u32, index: u32) -> u32 {
        stored_sector(e, walked, index) + 0x10
    }

    /// The float lanes at `at`.
    fn lanes(e: &Engine, at: u32) -> Vec<f32> {
        (0..4).map(|lane| e.mem.f32(at + 4 * lane)).collect()
    }

    /// Makes the entry at `entry` a swept entry with the pass-on flag `flag`, time `time` and
    /// vector `(2, 3, 4, 10)`.
    fn fill_swept(e: &mut Engine, entry: u32, flag: u8, time: f32) {
        e.mem.set_u8(entry + 2, flag);
        e.mem.set_f32(entry + 0xc, time);
        for (lane, value) in [2.0f32, 3.0, 4.0, 10.0].into_iter().enumerate() {
            e.mem.set_f32(entry + 0x10 + 4 * lane as u32, value);
        }
    }

    /// Sets up agent 5 for swept entries: the process function is the recording agent, the
    /// functions at `+0x16b8` and `+0x16d8` doubles returning the entry plus `0x30`/nothing.
    fn swept_agent(e: &mut Engine, w: &World) -> Agent {
        e.register_double(PASS_FUNCTION, |_, a| Ret {
            eax: a[0] + 0x30,
            ..Ret::default()
        });
        e.register_double(PREPARE_FUNCTION, |_, _| Ret::default());
        set_function(e, w, 5, PASS_AT, PASS_FUNCTION);
        set_function(e, w, 5, PREPARE_AT, PREPARE_FUNCTION);
        set_function(e, w, 5, AGENT_AT, AGENT_A);
        install_agent(e, AGENT_A, 0x30)
    }

    /// A track with one swept entry of `kind` (flag 1, `time`) and the end entry.
    fn swept_track(e: &mut Engine, kind: u8, time: f32) -> u32 {
        let (first, entries) = sector(e, &[(kind, 5, 0x30), (1, 0, 0x10)]);
        fill_swept(e, entries[0], 1, time);
        track(e, &[first])
    }

    #[test]
    fn process_keeps_the_entries_and_hands_the_input_sectors_back() {
        let mut e = logged();
        let w = world(&mut e);
        let (first, _) = sector(&mut e, &[(0, 0, 0x10)]);
        let (second, _) = sector(&mut e, &[(1, 0, 0x10)]);
        let walked = track(&mut e, &[first, second]);
        run_one(&mut e, &w, walked, 0);
        assert_eq!(e.mem.u32(walked + 4), 1);
        let (entries, used) = stored_entries(&e, walked, 0);
        assert_eq!(entries, vec![(0, 0x10), (1, 0x10)]);
        assert_eq!(used, 0x20);
        // The first input sector went to the collision input's cache, the second was freed.
        assert_eq!(e.mem.u32(w.collision_input + 0x64), first);
        assert_eq!(
            calls(&e, FREE_BLOCK_DOUBLE),
            vec![vec![w.allocator, second, 0x200]]
        );
        assert_eq!(minimum_times(&e, w.stream), vec![(MINIMUM_TIME_LABEL, 0.0)]);
        assert_eq!(e.mem.u32(w.state + 0x3080), 0);
    }

    #[test]
    fn process_hands_a_type_two_entry_to_its_agent() {
        let mut e = logged();
        let w = world(&mut e);
        let seen = install_agent(&mut e, AGENT_A, 0x40);
        set_function(&mut e, &w, 3, AGENT_AT, AGENT_A);
        let (first, _) = keyed(
            &mut e,
            &[(2, 3, 0x30), (1, 0, 0x10)],
            &[(0, 7), (0, 0xffff_ffff)],
        );
        let walked = track(&mut e, &[first]);
        run_one(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let call = &seen[0];
        assert_eq!(call.args[1..], [kept, kept + 0x10, 0, w.state]);
        assert_eq!(call.ctx[0], w.collidables[0]);
        assert_eq!(call.ctx[3], w.collision_input);
        assert_eq!(call.ctx[4], 0x4444);
        assert_eq!(call.ctx[8], 0x2008);
        assert_eq!(call.ctx[0x17], 0x2017);
        assert_eq!(call.ctx[0x1c], 1.0f32.to_bits());
        assert_eq!(
            call.second,
            vec![w.body_b, 7, w.motions[1], w.collidables[1]]
        );
        let bodies = calls(&e, BODY_SLOT_B);
        assert_eq!(bodies.len(), 1);
        assert_eq!(bodies[0][..2], [w.provider_b, 7]);
        let (entries, used) = stored_entries(&e, walked, 0);
        assert_eq!(entries, vec![(2, 0x40), (1, 0x10)]);
        assert_eq!(used, 0x50);
    }

    #[test]
    fn process_drops_the_entry_types_it_does_not_handle() {
        let mut e = logged();
        let w = world(&mut e);
        let (first, _) = sector(
            &mut e,
            &[
                (7, 0, 0x10),
                (0, 0, 0x10),
                (9, 0, 0x20),
                (15, 0, 0x10),
                (1, 0, 0x10),
            ],
        );
        let walked = track(&mut e, &[first]);
        run_one(&mut e, &w, walked, 0);
        let (entries, used) = stored_entries(&e, walked, 0);
        assert_eq!(entries, vec![(0, 0x10), (1, 0x10)]);
        assert_eq!(used, 0x20);
    }

    #[test]
    fn process_destroys_unwanted_entries_and_adds_a_placeholder_for_an_unwanted_key() {
        let mut e = logged();
        let w = world(&mut e);
        install_agent(&mut e, AGENT_A, 0x30);
        e.register_double(DESTROY_FUNCTION, |_, _| Ret::default());
        set_function(&mut e, &w, 3, AGENT_AT, AGENT_A);
        set_function(&mut e, &w, 3, DESTROY_AT, DESTROY_FUNCTION);
        let (first, entries) = keyed(
            &mut e,
            &[(2, 3, 0x30), (2, 3, 0x30), (1, 0, 0x10)],
            &[(0, 5), (0, 8), (0, 0xffff_ffff)],
        );
        let walked = track(&mut e, &[first]);
        let keys = key_list(&mut e, &[5, 6, 0xffff_ffff]);
        run_one(&mut e, &w, walked, keys);
        let (kept, used) = stored_entries(&e, walked, 0);
        assert_eq!(kept, vec![(2, 0x30), (0, 0x10), (1, 0x10)]);
        assert_eq!(used, 0x50);
        let placeholder = first_entry(&e, walked, 0) + 0x30;
        assert_eq!(e.mem.u32(placeholder + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(placeholder + 8), 6);
        assert_eq!(
            calls(&e, DESTROY_FUNCTION),
            vec![vec![
                entries[1],
                entries[1] + 0x10,
                0x4444,
                0x5757,
                w.dispatcher
            ]]
        );
        let asked = calls(&e, CREATE_SLOT);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][0], w.holder + 0xc);
        assert_eq!(
            asked[0][2..],
            [
                w.collision_input,
                w.collidables[0],
                w.collidables[1],
                w.provider_b,
                6
            ]
        );
    }

    #[test]
    fn process_builds_a_wanted_entry_with_the_agent_of_the_shape_pair() {
        let mut e = logged();
        let w = world(&mut e);
        e.mem.set_u8(w.flags, 1);
        e.mem.set_u32(w.shapes[0] + 0xc, 4);
        // The shape types (4 and the body's 2) pick agent 3 from the table.
        e.mem.set_u8(w.dispatcher + (4 << 5) + 2 + 0xeb0, 3);
        e.register_double(CREATE_FUNCTION, |_, a| Ret {
            eax: a[1] + 0x30,
            ..Ret::default()
        });
        let seen = install_agent(&mut e, AGENT_A, 0x30);
        set_function(&mut e, &w, 3, CREATE_AT, CREATE_FUNCTION);
        set_function(&mut e, &w, 3, AGENT_AT, AGENT_A);
        let (first, _) = keyed(&mut e, &[(1, 0, 0x10)], &[(0, 0xffff_ffff)]);
        let walked = track(&mut e, &[first]);
        let keys = key_list(&mut e, &[4, 0xffff_ffff]);
        run_one(&mut e, &w, walked, keys);
        let kept = first_entry(&e, walked, 0);
        let (entries, used) = stored_entries(&e, walked, 0);
        assert_eq!(entries, vec![(2, 0x30), (1, 0x10)]);
        assert_eq!(used, 0x40);
        assert_eq!(e.mem.u8(kept + 1), 3);
        assert_eq!(e.mem.u32(kept + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(kept + 8), 4);
        let built = calls(&e, CREATE_FUNCTION);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1..], [kept, kept + 0x10]);
        let resolved = calls(&e, PROCESS_TRANSFORMED_SHAPES);
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0][3], 4);
        let relative = calls(&e, TRANSFORM_SET_MUL_INVERSE_MUL);
        assert_eq!(relative.len(), 1);
        assert_eq!(relative[0][1..], [w.motions[0], w.motions[1]]);
        // The new entry is then handled as a type 2 entry.
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(calls(&e, BODY_SLOT_B).len(), 2);
    }

    #[test]
    fn process_builds_a_flipped_swept_entry_and_handles_it_as_type_five() {
        let mut e = logged();
        let w = world(&mut e);
        e.mem.set_u8(w.flags, 1);
        e.mem.set_u8(w.dispatcher + (1 << 5) + 2 + 0xeb0, 4);
        e.register_double(CREATE_FUNCTION, |_, a| Ret {
            eax: a[1] + 0x40,
            ..Ret::default()
        });
        e.register_double(PREPARE_FUNCTION, |_, _| Ret::default());
        let seen = install_agent(&mut e, AGENT_B, 0x40);
        set_function(&mut e, &w, 4, CREATE_AT, CREATE_FUNCTION);
        set_function(&mut e, &w, 4, PREPARE_AT, PREPARE_FUNCTION);
        set_function(&mut e, &w, 4, AGENT_AT, AGENT_B);
        e.mem.set_u32(w.dispatcher + 4 * 0x50 + FLIP_AT, 2);
        let (first, _) = keyed(&mut e, &[(1, 0, 0x10)], &[(0, 0xffff_ffff)]);
        let walked = track(&mut e, &[first]);
        let keys = key_list(&mut e, &[4, 0xffff_ffff]);
        run_one(&mut e, &w, walked, keys);
        let kept = first_entry(&e, walked, 0);
        let (entries, used) = stored_entries(&e, walked, 0);
        assert_eq!(entries, vec![(5, 0x40), (1, 0x10)]);
        assert_eq!(used, 0x50);
        assert_eq!(e.mem.u8(kept + 1), 4);
        let built = calls(&e, CREATE_FUNCTION);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1..], [kept, kept + 0x20]);
        // The time is the mismatch marker (-1) until the zero path of type 5 replaces it by the
        // collision input's second time, with the vector cleared and the object's float scaled.
        assert_eq!(e.mem.f32(kept + 0xc), 0.5);
        assert_eq!(lanes(&e, kept + 0x10), vec![0.0, 0.0, 0.0, 2.0]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].args[1..4], [kept, kept + 0x20, kept + 0x10]);
        assert_eq!(seen[0].ctx[0x18], 2.0f32.to_bits());
        assert_eq!(
            seen[0].first,
            vec![w.body_b, 4, w.motions[1], w.collidables[1]]
        );
        // One mirrored input for the new entry and one for the type 5 handling.
        assert_eq!(calls(&e, TRANSFORM_SET_INVERSE).len(), 2);
    }

    #[test]
    fn process_passes_a_type_four_entry_of_equal_time_on() {
        let mut e = logged();
        let w = world(&mut e);
        let seen = swept_agent(&mut e, &w);
        let (first, entries) = sector(&mut e, &[(4, 5, 0x30), (4, 5, 0x30), (1, 0, 0x10)]);
        fill_swept(&mut e, entries[0], 1, 1.0);
        fill_swept(&mut e, entries[1], 0, 1.0);
        let walked = track(&mut e, &[first]);
        run_one(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        let (found, used) = stored_entries(&e, walked, 0);
        assert_eq!(found, vec![(4, 0x30), (4, 0x30), (1, 0x10)]);
        assert_eq!(used, 0x70);
        // Dot product (4 * 0 + 0.5) + (3 * 0 + 2 * 1) = 2.5; 10 - 2.5 remains and the time moves
        // to the collision input's second time.
        for entry in [kept, kept + 0x30] {
            assert_eq!(e.mem.f32(entry + 0xc), 0.5);
            assert_eq!(e.mem.f32(entry + 0x1c), 7.5);
        }
        let passed = calls(&e, PASS_FUNCTION);
        assert_eq!(passed.len(), 1);
        assert_eq!(passed[0][0], kept);
        assert_eq!(passed[0][1], kept + 0x20);
        assert_eq!(passed[0][2], 0x4444);
        assert_eq!(passed[0][3], 0x5757);
        assert!(seen.borrow().is_empty());
        assert_eq!(minimum_times(&e, w.stream), vec![(MINIMUM_TIME_LABEL, 2.0)]);
    }

    #[test]
    fn process_keeps_a_type_four_entry_with_much_remaining_distance_for_its_agent() {
        let mut e = logged();
        let w = world(&mut e);
        e.mem.set_f32(w.collision_input + 8, 100.0);
        let seen = swept_agent(&mut e, &w);
        let walked = swept_track(&mut e, 4, 1.0);
        run_one(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].args[1..], [kept, kept + 0x20, kept + 0x10, w.state]);
        assert_eq!(seen[0].ctx[0x18], 7.5f32.to_bits());
        assert!(calls(&e, PASS_FUNCTION).is_empty());
        assert_eq!(e.mem.f32(kept + 0xc), 0.5);
        assert_eq!(e.mem.f32(kept + 0x1c), 10.0);
        assert_eq!(stored_entries(&e, walked, 0).0, vec![(4, 0x30), (1, 0x10)]);
    }

    #[test]
    fn process_restarts_a_type_four_entry_before_the_second_time_without_an_object_state() {
        let mut e = logged();
        let w = world(&mut e);
        let seen = swept_agent(&mut e, &w);
        let walked = swept_track(&mut e, 4, 0.25);
        run_one(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        let scaled = (4.0f64 * f64::from(0.1f32)) as f32;
        assert_eq!(e.mem.f32(kept + 0xc), 0.5);
        assert_eq!(lanes(&e, kept + 0x10), vec![0.0, 0.0, 0.0, scaled]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].ctx[0x18], scaled.to_bits());
        assert!(calls(&e, PASS_FUNCTION).is_empty());
    }

    #[test]
    fn process_brings_a_later_type_four_entry_back_to_the_second_time() {
        let mut e = logged();
        let w = world(&mut e);
        swept_agent(&mut e, &w);
        let walked = swept_track(&mut e, 4, 0.75);
        run_one(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        assert_eq!(e.mem.f32(kept + 0xc), 0.5);
        assert_eq!(e.mem.f32(kept + 0x1c), 7.5);
        assert_eq!(calls(&e, PASS_FUNCTION).len(), 1);
    }

    #[test]
    fn process_initialises_the_input_at_the_time_for_a_type_four_entry_with_an_object() {
        let mut e = logged();
        let w = world(&mut e);
        e.mem.set_u32(w.object + 0x10, 1);
        swept_agent(&mut e, &w);
        let walked = swept_track(&mut e, 4, 0.25);
        run_one(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        let lerps = calls(&e, SWEPT_LERP2);
        assert_eq!(lerps.len(), 2);
        assert_eq!(lerps[0][..2], [w.motions[0] + 0x40, 1.0f32.to_bits()]);
        assert_eq!(lerps[1][..2], [w.motions[1] + 0x40, 1.0f32.to_bits()]);
        assert_eq!(calls(&e, TRANSFORM_SET_MUL_INVERSE_MUL).len(), 1);
        let prepared = calls(&e, PREPARE_FUNCTION);
        assert_eq!(prepared.len(), 1);
        assert_eq!(prepared[0][1..], [kept, kept + 0x20, kept + 0x10]);
        // Then the entry is brought to the second time and passed on.
        assert_eq!(e.mem.f32(kept + 0xc), 0.5);
        assert_eq!(e.mem.f32(kept + 0x1c), 7.5);
        assert_eq!(calls(&e, PASS_FUNCTION).len(), 1);
    }

    #[test]
    fn process_handles_a_type_five_entry_with_the_mirrored_input() {
        let mut e = logged();
        let w = world(&mut e);
        swept_agent(&mut e, &w);
        let walked = swept_track(&mut e, 5, 1.0);
        run_one(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        // The mirrored vector is (-1, -0, -0, 0.5): the dot product is -1.5.
        assert_eq!(e.mem.f32(kept + 0x1c), 11.5);
        assert_eq!(e.mem.f32(kept + 0xc), 0.5);
        let passed = calls(&e, PASS_FUNCTION);
        assert_eq!(passed.len(), 1);
        assert_eq!(passed[0], vec![kept, kept + 0x20, 0x4444, 0x5757]);
        assert_eq!(calls(&e, TRANSFORM_SET_INVERSE).len(), 1);
    }

    #[test]
    fn process_advances_the_sweep_records_of_a_type_five_agent_and_mirrors_the_state() {
        let mut e = logged();
        let w = world(&mut e);
        e.mem.set_f32(w.collision_input + 8, 100.0);
        swept_agent(&mut e, &w);
        let seen = install_sweeper(&mut e, AGENT_A, 0x30, true, Some(3.0));
        let walked = swept_track(&mut e, 5, 1.0);
        run_one(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].args[1..4], [kept, kept + 0x20, kept + 0x10]);
        assert_eq!(seen[0].ctx[0x18], 11.5f32.to_bits());
        // The mirrored context: the bodies' records swapped, the vector's first lanes negated.
        assert_eq!(
            seen[0].first,
            vec![w.body_b, 0, w.motions[1], w.collidables[1]]
        );
        assert_eq!(seen[0].ctx[3], w.collision_input);
        // The record the agent appended: the position moved along the velocity by its last
        // lane, the velocity mirrored.
        assert_eq!(lanes(&e, w.list), vec![2.0, 4.0, 7.0, 8.0]);
        assert_eq!(lanes(&e, w.list + 0x10), vec![-0.5, -1.0, -2.0, 2.0]);
        // The state's float changed, so its vector is mirrored and the reorder function runs.
        assert_eq!(lanes(&e, w.state + 0x3020), vec![-1.0, -2.0, -3.0, 4.0]);
        assert_eq!(calls(&e, STATE_REORDER), vec![vec![w.state + 0x3040]]);
    }

    #[test]
    fn process_hands_a_type_three_entry_the_mirrored_input_and_advances_its_records() {
        let mut e = logged();
        let w = world(&mut e);
        let seen = install_sweeper(&mut e, AGENT_A, 0x30, true, None);
        set_function(&mut e, &w, 6, AGENT_AT, AGENT_A);
        let (first, _) = sector(&mut e, &[(3, 6, 0x30), (1, 0, 0x10)]);
        let walked = track(&mut e, &[first]);
        run_one(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].args[1..], [kept, kept + 0x10, 0, w.state]);
        let mirrored: Vec<u32> = [-1.0f32, -0.0, -0.0, 0.5]
            .iter()
            .map(|value| value.to_bits())
            .collect();
        assert_eq!(seen[0].ctx[0x1c..0x20], mirrored[..]);
        assert_eq!(lanes(&e, w.list), vec![2.0, 4.0, 7.0, 8.0]);
        assert_eq!(lanes(&e, w.list + 0x10), vec![-0.5, -1.0, -2.0, 2.0]);
        // The state's float did not change.
        assert!(calls(&e, STATE_REORDER).is_empty());
        assert_eq!(lanes(&e, w.state + 0x3020), vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn process_asks_the_object_of_a_type_six_entry() {
        let mut e = logged();
        let w = world(&mut e);
        e.register_double(OBJECT_SLOT, |_, _| Ret::default());
        let asked = make_object(&mut e, &[(0x18, OBJECT_SLOT)]);
        let (first, entries) = sector(&mut e, &[(6, 0, 0x20), (1, 0, 0x10)]);
        e.mem.set_u32(entries[0] + 0x10, asked);
        let walked = track(&mut e, &[first]);
        run_one(&mut e, &w, walked, 0);
        let slot = calls(&e, OBJECT_SLOT);
        assert_eq!(slot.len(), 1);
        assert_eq!(slot[0][0], asked);
        assert_eq!(slot[0][1], w.collidables[0]);
        assert_eq!(slot[0][3..], [w.collision_input, w.state]);
        assert_eq!(stored_entries(&e, walked, 0).0, vec![(6, 0x20), (1, 0x10)]);
    }

    #[test]
    fn process_resolves_the_transform_shapes_of_a_type_twelve_entry() {
        let mut e = logged();
        let w = world(&mut e);
        swept_agent(&mut e, &w);
        let walked = swept_track(&mut e, 12, 1.0);
        run_one(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        let resolved = calls(&e, PROCESS_TRANSFORMED_SHAPES);
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0][3], 4);
        let relative = calls(&e, TRANSFORM_SET_MUL_INVERSE_MUL);
        assert_eq!(relative.len(), 1);
        assert_eq!(relative[0][1..], [w.motions[0], w.motions[1]]);
        // Then it is handled as type 4; its own type byte stays.
        assert_eq!(stored_entries(&e, walked, 0).0, vec![(12, 0x30), (1, 0x10)]);
        assert_eq!(e.mem.f32(kept + 0x1c), 7.5);
        assert_eq!(calls(&e, PASS_FUNCTION).len(), 1);
    }

    /// Sets up the pointer stream and an agent that adds a record `(0, entry, entry + 0x10)` to it
    /// for every entry; the weld double notes the records it finds. Runs `count` entries of type
    /// 2 (size 0x30, which the agent grows to 0x40) and returns the track and the records.
    fn stream_scenario(
        e: &mut Engine,
        w: &World,
        count: usize,
    ) -> (u32, Rc<RefCell<Vec<(u32, u32)>>>) {
        e.register_double(STREAM_SLOT, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(AGENT_A, |e, a| {
            let stream = e.mem.u32(a[4] + 0x3080);
            let cursor = e.mem.u32(stream);
            e.mem.set_u32(cursor + 4, a[1]);
            e.mem.set_u32(cursor + 8, a[1] + 0x10);
            e.mem.set_u32(stream, cursor + 0xc);
            Ret {
                eax: a[1] + 0x40,
                ..Ret::default()
            }
        });
        let found: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let log = Rc::clone(&found);
        e.register_double(WELD, move |e, a| {
            let stream = e.mem.u32(a[2] + 0x3080);
            let end = e.mem.u32(stream);
            log.borrow_mut().extend(
                (stream + 0x408..end)
                    .step_by(12)
                    .map(|at| (e.mem.u32(at + 4), e.mem.u32(at + 8))),
            );
            Ret::default()
        });
        set_function(e, w, 1, AGENT_AT, AGENT_A);
        let entries = vec![(2u8, 1u8, 0x30u8); count];
        let (first, _) = sector(e, &entries);
        (track(e, &[first]), found)
    }

    #[test]
    fn process_moves_a_sector_that_grew_through_the_scratch_area_and_its_stream_records() {
        let mut e = logged();
        let w = world(&mut e);
        let (walked, found) = stream_scenario(&mut e, &w, 9);
        run_one(&mut e, &w, walked, 0);
        assert_eq!(e.mem.u32(walked + 4), 2);
        let (one, one_used) = stored_entries(&e, walked, 0);
        let (two, two_used) = stored_entries(&e, walked, 1);
        assert_eq!(one, vec![(2, 0x40); 7]);
        assert_eq!(one_used, 0x1c0);
        assert_eq!(two, vec![(2, 0x40); 2]);
        assert_eq!(two_used, 0x80);
        // The weld function saw the records of all nine entries at their final places.
        let (low, high) = (first_entry(&e, walked, 0), first_entry(&e, walked, 1));
        let places: Vec<(u32, u32)> = (0..9)
            .map(|i| {
                let at = if i < 7 {
                    low + 0x40 * i
                } else {
                    high + 0x40 * (i - 7)
                };
                (at, at + 0x10)
            })
            .collect();
        assert_eq!(*found.borrow(), places);
        assert_eq!(calls(&e, WELD).len(), 1);
        assert_eq!(e.mem.u32(w.state + 0x3080), 0);
    }

    #[test]
    fn process_moves_the_scratch_entries_into_the_last_sector_and_the_last_stream_record() {
        let mut e = logged();
        let w = world(&mut e);
        let (walked, found) = stream_scenario(&mut e, &w, 7);
        run_one(&mut e, &w, walked, 0);
        assert_eq!(e.mem.u32(walked + 4), 1);
        let (entries, used) = stored_entries(&e, walked, 0);
        assert_eq!(entries, vec![(2, 0x40); 7]);
        assert_eq!(used, 0x1c0);
        // Only the last record belongs to an entry that was in the scratch area: it is moved.
        let low = first_entry(&e, walked, 0);
        let places: Vec<(u32, u32)> = (0..7)
            .map(|i| (low + 0x40 * i, low + 0x40 * i + 0x10))
            .collect();
        assert_eq!(*found.borrow(), places);
    }

    #[test]
    fn process_grows_the_track_array_when_the_rebuilt_track_needs_more_sectors() {
        let mut e = logged();
        let w = world(&mut e);
        install_reserve(&mut e);
        install_agent(&mut e, AGENT_A, 0x40);
        set_function(&mut e, &w, 1, AGENT_AT, AGENT_A);
        let (first, _) = sector(&mut e, &[(2u8, 1u8, 0x30u8); 9]);
        let walked = track(&mut e, &[first]);
        // No spare capacity: the capacity word equals the size.
        e.mem.set_u32(walked + 8, 1);
        run_one(&mut e, &w, walked, 0);
        assert_eq!(e.mem.u32(walked + 4), 2);
        let reserves = calls(&e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0][1..], [walked, 2, 4]);
        assert_eq!(stored_entries(&e, walked, 1).1, 0x80);
    }

    // ---- hkAgentNmMachine_Process ----

    #[test]
    fn nm_process_fetches_a_body_only_when_its_key_changes() {
        let mut e = logged();
        let w = world(&mut e);
        let seen = install_agent(&mut e, AGENT_A, 0x30);
        set_function(&mut e, &w, 3, AGENT_AT, AGENT_A);
        let (first, _) = keyed(
            &mut e,
            &[(2, 3, 0x30), (2, 3, 0x30), (2, 3, 0x30), (1, 0, 0x10)],
            &[(1, 2), (1, 2), (3, 2), (0xffff_ffff, 0xffff_ffff)],
        );
        let walked = track(&mut e, &[first]);
        run_many(&mut e, &w, walked, 0);
        let (entries, used) = stored_entries(&e, walked, 0);
        assert_eq!(entries, vec![(2, 0x30), (2, 0x30), (2, 0x30), (1, 0x10)]);
        assert_eq!(used, 0xa0);
        let kept = first_entry(&e, walked, 0);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 3);
        assert_eq!(seen[0].args[1..], [kept, kept + 0x10, 0, w.state]);
        assert_eq!(
            seen[0].first,
            vec![w.body_b, 1, w.motions[0], w.collidables[0]]
        );
        assert_eq!(
            seen[0].second,
            vec![w.body_a, 2, w.motions[1], w.collidables[1]]
        );
        assert_eq!(seen[2].first[1], 3);
        assert_eq!(seen[2].second[1], 2);
        let from_a = calls(&e, BODY_SLOT_A);
        assert_eq!(from_a.len(), 1);
        assert_eq!(from_a[0][..2], [w.provider_a, 2]);
        let from_b: Vec<u32> = calls(&e, BODY_SLOT_B).iter().map(|call| call[1]).collect();
        assert_eq!(from_b, vec![1, 3]);
        assert_eq!(minimum_times(&e, w.stream), vec![(MINIMUM_TIME_LABEL, 0.0)]);
    }

    #[test]
    fn nm_process_orders_the_key_pairs_by_their_second_word_first() {
        let mut e = logged();
        let w = world(&mut e);
        install_agent(&mut e, AGENT_A, 0x30);
        e.register_double(DESTROY_FUNCTION, |_, _| Ret::default());
        set_function(&mut e, &w, 3, AGENT_AT, AGENT_A);
        set_function(&mut e, &w, 3, DESTROY_AT, DESTROY_FUNCTION);
        let (first, entries) = keyed(
            &mut e,
            &[(2, 3, 0x30), (2, 3, 0x30), (1, 0, 0x10)],
            &[(9, 1), (2, 2), (0xffff_ffff, 0xffff_ffff)],
        );
        let walked = track(&mut e, &[first]);
        let keys = key_list(&mut e, &[1, 2, 0xffff_ffff, 0xffff_ffff]);
        run_many(&mut e, &w, walked, keys);
        // (9, 1) is passed by the key (1, 2) although 9 > 1; (2, 2) is behind it and gets a
        // placeholder for (1, 2) first; then (-1, -1) passes it.
        let (kept, used) = stored_entries(&e, walked, 0);
        assert_eq!(kept, vec![(0, 0x10), (1, 0x10)]);
        assert_eq!(used, 0x20);
        let placeholder = first_entry(&e, walked, 0);
        assert_eq!(e.mem.u32(placeholder + 4), 1);
        assert_eq!(e.mem.u32(placeholder + 8), 2);
        assert_eq!(
            calls(&e, DESTROY_FUNCTION),
            vec![
                vec![entries[0], entries[0] + 0x10, 0x4444, 0x5757, w.dispatcher],
                vec![entries[1], entries[1] + 0x10, 0x4444, 0x5757, w.dispatcher]
            ]
        );
        let asked = calls(&e, CREATE_SLOT);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][0], w.holder + 0xc);
        assert_eq!(
            asked[0][2..],
            [
                w.collision_input,
                w.collidables[0],
                w.collidables[1],
                w.provider_b,
                w.provider_a,
                1,
                2
            ]
        );
    }

    #[test]
    fn nm_process_passes_swept_entries_of_equal_time_on() {
        let mut e = logged();
        let w = world(&mut e);
        swept_agent(&mut e, &w);
        let (first, entries) = sector(&mut e, &[(4, 5, 0x30), (5, 5, 0x30), (1, 0, 0x10)]);
        fill_swept(&mut e, entries[0], 1, 1.0);
        fill_swept(&mut e, entries[1], 1, 1.0);
        let walked = track(&mut e, &[first]);
        run_many(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        assert_eq!(e.mem.f32(kept + 0x1c), 7.5);
        assert_eq!(e.mem.f32(kept + 0x30 + 0x1c), 11.5);
        let passed = calls(&e, PASS_FUNCTION);
        assert_eq!(passed.len(), 2);
        assert_eq!(passed[0][0], kept);
        assert_eq!(passed[0][2], 0x4444);
        assert_eq!(passed[1], vec![kept + 0x30, kept + 0x50, 0x4444, 0x5757]);
        assert_eq!(minimum_times(&e, w.stream), vec![(MINIMUM_TIME_LABEL, 2.0)]);
    }

    #[test]
    fn nm_process_resolves_the_transform_shapes_of_both_bodies_for_a_type_twelve_entry() {
        let mut e = logged();
        let w = world(&mut e);
        swept_agent(&mut e, &w);
        let walked = swept_track(&mut e, 12, 1.0);
        run_many(&mut e, &w, walked, 0);
        let resolved = calls(&e, PROCESS_TRANSFORMED_SHAPES);
        assert_eq!(resolved.len(), 2);
        assert_eq!(resolved[0][3], 4);
        assert_eq!(resolved[1][3], 4);
        assert_ne!(resolved[0][0], resolved[1][0]);
        let relative = calls(&e, TRANSFORM_SET_MUL_INVERSE_MUL);
        assert_eq!(relative.len(), 1);
        assert_eq!(relative[0][1..], [w.motions[0], w.motions[1]]);
        assert_eq!(calls(&e, BODY_SLOT_A).len(), 1);
        assert_eq!(calls(&e, BODY_SLOT_B).len(), 1);
        let kept = first_entry(&e, walked, 0);
        assert_eq!(stored_entries(&e, walked, 0).0, vec![(12, 0x30), (1, 0x10)]);
        assert_eq!(e.mem.f32(kept + 0x1c), 7.5);
        assert_eq!(calls(&e, PASS_FUNCTION).len(), 1);
    }

    #[test]
    fn nm_process_asks_a_type_six_object_and_mirrors_the_input_for_type_three() {
        let mut e = logged();
        let w = world(&mut e);
        e.register_double(OBJECT_SLOT, |_, _| Ret::default());
        let asked = make_object(&mut e, &[(0x18, OBJECT_SLOT)]);
        let seen = install_sweeper(&mut e, AGENT_A, 0x30, true, None);
        set_function(&mut e, &w, 6, AGENT_AT, AGENT_A);
        let (first, entries) = sector(&mut e, &[(6, 0, 0x20), (3, 6, 0x30), (1, 0, 0x10)]);
        e.mem.set_u32(entries[0] + 0x10, asked);
        let walked = track(&mut e, &[first]);
        run_many(&mut e, &w, walked, 0);
        let slot = calls(&e, OBJECT_SLOT);
        assert_eq!(slot.len(), 1);
        assert_eq!(slot[0][0], asked);
        assert_eq!(slot[0][3..], [w.collision_input, w.state]);
        let kept = first_entry(&e, walked, 0);
        assert_eq!(
            stored_entries(&e, walked, 0).0,
            vec![(6, 0x20), (3, 0x30), (1, 0x10)]
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].args[1..], [kept + 0x20, kept + 0x30, 0, w.state]);
        // The mirrored context swaps the records of the two bodies.
        assert_eq!(
            seen[0].first,
            vec![w.body_a, 0, w.motions[1], w.collidables[1]]
        );
        assert_eq!(
            seen[0].second,
            vec![w.body_b, 0, w.motions[0], w.collidables[0]]
        );
        assert_eq!(lanes(&e, w.list), vec![2.0, 4.0, 7.0, 8.0]);
        assert_eq!(lanes(&e, w.list + 0x10), vec![-0.5, -1.0, -2.0, 2.0]);
        assert!(calls(&e, STATE_REORDER).is_empty());
    }

    #[test]
    fn nm_process_mirrors_the_state_when_a_type_five_agent_changes_its_float() {
        let mut e = logged();
        let w = world(&mut e);
        e.mem.set_f32(w.collision_input + 8, 100.0);
        swept_agent(&mut e, &w);
        let seen = install_sweeper(&mut e, AGENT_A, 0x30, false, Some(3.0));
        let walked = swept_track(&mut e, 5, 1.0);
        run_many(&mut e, &w, walked, 0);
        let kept = first_entry(&e, walked, 0);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].args[1..4], [kept, kept + 0x20, kept + 0x10]);
        assert_eq!(seen[0].ctx[0x18], 11.5f32.to_bits());
        assert_eq!(lanes(&e, w.state + 0x3020), vec![-1.0, -2.0, -3.0, 4.0]);
        assert_eq!(calls(&e, STATE_REORDER), vec![vec![w.state + 0x3040]]);
    }
}
