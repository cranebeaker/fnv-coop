//! `NiAdditionalGeometryData`: extra per-vertex channels a shape's
//! geometry carries beside its positions, normals and texture coordinates.
//! The distant-land chunks (`meshes\landscape\lod\<world>\<world>.level<L>
//! .x<X>.y<Y>.nif`) keep one: a float per vertex that the game's
//! distant-land vertex shader (`SLS2002.vso`) receives as texcoord1 and
//! geomorphs the vertex's height toward (see `world::lod`).
//!
//! Layout (version 20.2.0.7, checked on `wastelandnv.level8.x-8.y8.nif`:
//! 798 vertices, one 4-byte channel, the block exactly used up): vertex
//! count u16; channel count u32, then per channel its type u32, bytes per
//! vertex u32, total bytes u32, bytes per vertex over all channels u32,
//! data block number u32, offset in the block's vertex u32, a byte (2);
//! data block count u32, then per block a has-data byte and, with data, the
//! block's size u32, its sub-block count u32 and their offsets u32 each,
//! a data count u32 and their sizes u32 each, then size × data count bytes.

use crate::file::Nif;
use crate::reader::Reader;

/// One channel: what it holds and its bytes, vertex after vertex.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Channel {
    pub data_type: u32,
    pub bytes_per_vertex: u32,
    pub data: Vec<u8>,
}

impl Channel {
    /// The channel read as one float per vertex (`None` unless it's 4 bytes
    /// a vertex).
    pub fn floats(&self) -> Option<Vec<f32>> {
        (self.bytes_per_vertex == 4).then(|| {
            self.data
                .chunks_exact(4)
                .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                .collect()
        })
    }
}

/// A decoded `NiAdditionalGeometryData` block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionalGeometry {
    pub vertices: usize,
    pub channels: Vec<Channel>,
}

struct ChannelInfo {
    data_type: u32,
    bytes_per_vertex: u32,
    total_bytes: u32,
    stride: u32,
    block: u32,
    offset: u32,
}

/// Decodes one block's bytes; `None` when they don't add up.
pub fn parse(bytes: &[u8]) -> Option<AdditionalGeometry> {
    let mut r = Reader::new(bytes, 0);
    let vertices = usize::from(r.u16("vertex count").ok()?);
    let count = r.u32("channel count").ok()? as usize;
    let mut infos = Vec::new();
    for _ in 0..count.min(64) {
        infos.push(ChannelInfo {
            data_type: r.u32("type").ok()?,
            bytes_per_vertex: r.u32("bytes per vertex").ok()?,
            total_bytes: r.u32("total bytes").ok()?,
            stride: r.u32("stride").ok()?,
            block: r.u32("block").ok()?,
            offset: r.u32("offset").ok()?,
        });
        r.u8("unknown byte").ok()?;
    }
    if infos.len() != count {
        return None;
    }
    let blocks = r.u32("block count").ok()? as usize;
    let mut data_blocks: Vec<Option<Vec<u8>>> = Vec::new();
    for _ in 0..blocks.min(64) {
        if r.u8("has data").ok()? == 0 {
            data_blocks.push(None);
            continue;
        }
        let size = r.u32("block size").ok()? as usize;
        let subs = r.u32("sub-blocks").ok()? as usize;
        for _ in 0..subs.min(1 << 16) {
            r.u32("sub-block offset").ok()?;
        }
        let datas = r.u32("data count").ok()? as usize;
        for _ in 0..datas.min(1 << 16) {
            r.u32("data size").ok()?;
        }
        let total = size.checked_mul(datas)?;
        data_blocks.push(Some(r.take(total, "data").ok()?.to_vec()));
    }
    let channels = infos
        .iter()
        .map(|info| {
            let block = data_blocks.get(info.block as usize)?.as_ref()?;
            let (per, stride) = (info.bytes_per_vertex as usize, info.stride as usize);
            let mut data = Vec::with_capacity(info.total_bytes as usize);
            for v in 0..vertices {
                let at = v * stride + info.offset as usize;
                data.extend_from_slice(block.get(at..at + per)?);
            }
            Some(Channel {
                data_type: info.data_type,
                bytes_per_vertex: info.bytes_per_vertex,
                data,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    Some(AdditionalGeometry { vertices, channels })
}

impl Nif {
    /// The file's `NiAdditionalGeometryData` blocks that decode, by block
    /// number.
    pub fn additional_geometry(&self) -> Vec<(usize, AdditionalGeometry)> {
        (0..self.blocks().len())
            .filter(|&i| self.block_type(i) == "NiAdditionalGeometryData")
            .filter_map(|i| parse(self.block_bytes(i)).map(|g| (i, g)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A block laid out as the game's distant-land chunks have it.
    fn block(heights: &[f32]) -> Vec<u8> {
        let n = heights.len() as u32;
        let mut b = (n as u16).to_le_bytes().to_vec();
        b.extend(1u32.to_le_bytes());
        for v in [1u32, 4, 4 * n, 4, 0, 0] {
            b.extend(v.to_le_bytes());
        }
        b.push(2);
        b.extend(1u32.to_le_bytes());
        b.push(1);
        for v in [4 * n, 1, 0, 1, 4] {
            b.extend(v.to_le_bytes());
        }
        for h in heights {
            b.extend(h.to_le_bytes());
        }
        b
    }

    #[test]
    fn reads_the_distant_lands_morph_heights() {
        let g = parse(&block(&[4280.0, 4240.0, 4320.0])).unwrap();
        assert_eq!(g.vertices, 3);
        assert_eq!(g.channels.len(), 1);
        assert_eq!(
            g.channels[0].floats().unwrap(),
            vec![4280.0, 4240.0, 4320.0]
        );
        // Cut short: nothing.
        let mut short = block(&[1.0, 2.0]);
        short.truncate(short.len() - 2);
        assert_eq!(parse(&short), None);
    }
}
