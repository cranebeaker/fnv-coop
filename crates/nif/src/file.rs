//! A parsed NIF file: header, block table and typed access to blocks.

use std::collections::BTreeMap;
use std::ops::Range;
use std::path::Path;

use crate::blocks::{self, kind_of, Block, Ctx, Kind, Layout};
use crate::error::{Error, Result};
use crate::header::{parse_header, Header};
use crate::reader::Reader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockInfo {
    pub type_index: u16,
    /// File offset of the block's first byte.
    pub offset: usize,
    pub size: usize,
}

pub struct Nif {
    bytes: Vec<u8>,
    header: Header,
    blocks: Vec<BlockInfo>,
    roots: Vec<i32>,
    layout: Layout,
}

impl Nif {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::parse(std::fs::read(path)?)
    }

    pub fn parse(bytes: Vec<u8>) -> Result<Self> {
        let header = parse_header(&bytes)?;
        let mut blocks = Vec::with_capacity(header.block_sizes.len());
        let mut offset = header.data_start;
        for (i, (&size, &type_index)) in header
            .block_sizes
            .iter()
            .zip(&header.block_type_index)
            .enumerate()
        {
            let size = size as usize;
            if offset.saturating_add(size) > bytes.len() {
                return Err(Error::Malformed {
                    offset,
                    reason: format!(
                        "block {i} ({}) is {size} bytes, past the end of the file",
                        header.block_types[usize::from(type_index)]
                    ),
                });
            }
            blocks.push(BlockInfo {
                type_index,
                offset,
                size,
            });
            offset += size;
        }

        // Footer: the root blocks. Old or truncated footers default to block 0.
        let mut r = Reader::at(&bytes, offset);
        let roots = match r.u32("the root count") {
            Ok(n) => r
                .counted(n as usize, 4, "roots", |r| r.i32("a root"))
                .unwrap_or_default(),
            Err(_) => Vec::new(),
        };
        let roots = if roots.is_empty() && !blocks.is_empty() {
            vec![0]
        } else {
            roots
        };

        let mut nif = Self {
            bytes,
            layout: Layout {
                av_flags_u32: header.bs_version > 26,
                material_crc: false,
            },
            header,
            blocks,
            roots,
        };
        nif.layout = nif.detect_layout();
        Ok(nif)
    }

    /// Picks the layout variants that decode sample blocks exactly.
    fn detect_layout(&self) -> Layout {
        let mut layout = self.layout;
        let exact = |i: usize, layout: Layout| -> bool {
            let ctx = Ctx {
                strings: &self.header.strings,
                bs_version: self.header.bs_version,
                layout,
            };
            let mut r = Reader::new(self.block_bytes(i), self.blocks[i].offset);
            let ok = match self.block_type(i) {
                t @ ("NiNode" | "BSFadeNode") => blocks::read_node(&mut r, &ctx, t).is_ok(),
                "NiTriShapeData" => blocks::read_geometry_data(&mut r, &ctx, false).is_ok(),
                _ => false,
            };
            ok && r.remaining() == 0
        };

        let first =
            |names: &[&str]| (0..self.blocks.len()).find(|&i| names.contains(&self.block_type(i)));
        if let Some(i) = first(&["NiNode", "BSFadeNode"]) {
            let flipped = Layout {
                av_flags_u32: !layout.av_flags_u32,
                ..layout
            };
            if !exact(i, layout) && exact(i, flipped) {
                layout = flipped;
            }
        }
        if let Some(i) = first(&["NiTriShapeData"]) {
            let with_crc = Layout {
                material_crc: true,
                ..layout
            };
            if !exact(i, layout) && exact(i, with_crc) {
                layout = with_crc;
            }
        }
        layout
    }

    /// What decoding a block of this file needs (its strings, version and
    /// layout).
    pub(crate) fn ctx(&self) -> Ctx<'_> {
        Ctx {
            strings: &self.header.strings,
            bs_version: self.header.bs_version,
            layout: self.layout,
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn blocks(&self) -> &[BlockInfo] {
        &self.blocks
    }

    pub fn roots(&self) -> &[i32] {
        &self.roots
    }

    pub fn file_size(&self) -> usize {
        self.bytes.len()
    }

    pub fn block_type(&self, index: usize) -> &str {
        &self.header.block_types[usize::from(self.blocks[index].type_index)]
    }

    pub fn block_range(&self, index: usize) -> Range<usize> {
        let b = &self.blocks[index];
        b.offset..b.offset + b.size
    }

    pub fn block_bytes(&self, index: usize) -> &[u8] {
        &self.bytes[self.block_range(index)]
    }

    /// Number of blocks of each type, most common first.
    pub fn type_counts(&self) -> Vec<(String, usize)> {
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for i in 0..self.blocks.len() {
            *counts.entry(self.block_type(i)).or_default() += 1;
        }
        let mut counts: Vec<(String, usize)> = counts
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        counts
    }

    /// The name of a block, for types that have one (cameras and lights
    /// too, read from their `NiAVObject` fields).
    pub fn block_name(&self, index: usize) -> Option<String> {
        match self.block(index).ok()? {
            Block::Node(n) => Some(n.av.net.name),
            Block::Geometry(g) => Some(g.av.net.name),
            Block::Shader(s) => Some(s.net.name),
            _ => match self.block_type(index) {
                "NiCamera" | "NiPointLight" | "NiSpotLight" | "NiAmbientLight"
                | "NiDirectionalLight" => self.av_name(index),
                _ => None,
            },
        }
        .filter(|n| !n.is_empty())
    }

    /// Decodes one block. Types this reader doesn't handle come back as
    /// [`Block::Other`].
    pub fn block(&self, index: usize) -> Result<Block> {
        if index >= self.blocks.len() {
            return Err(Error::Malformed {
                offset: 0,
                reason: format!(
                    "reference to block {index}, but the file has {}",
                    self.blocks.len()
                ),
            });
        }
        if self.header.bs_version > 34 {
            return Err(Error::Unsupported(format!(
                "Bethesda version {} (Skyrim or later); Fallout 3 and New Vegas use 34",
                self.header.bs_version
            )));
        }
        let ctx = Ctx {
            strings: &self.header.strings,
            bs_version: self.header.bs_version,
            layout: self.layout,
        };
        let type_name = self.block_type(index);
        let mut r = Reader::new(self.block_bytes(index), self.blocks[index].offset);
        let decoded = match kind_of(type_name) {
            Kind::Node => blocks::read_node(&mut r, &ctx, type_name).map(Block::Node),
            Kind::Geometry => blocks::read_geometry(&mut r, &ctx).map(Block::Geometry),
            Kind::TriShapeData => {
                blocks::read_geometry_data(&mut r, &ctx, false).map(Block::GeometryData)
            }
            Kind::TriStripsData => {
                blocks::read_geometry_data(&mut r, &ctx, true).map(Block::GeometryData)
            }
            Kind::Shader(layout) => {
                blocks::read_shader(&mut r, &ctx, layout, type_name).map(Block::Shader)
            }
            Kind::TextureSet => blocks::read_texture_set(&mut r).map(Block::TextureSet),
            Kind::Texturing => blocks::read_texturing(&mut r, &ctx).map(Block::Texturing),
            Kind::SourceTexture => {
                blocks::read_source_texture(&mut r, &ctx).map(Block::SourceTexture)
            }
            Kind::Material => blocks::read_material(&mut r, &ctx).map(Block::Material),
            Kind::Alpha => blocks::read_alpha(&mut r, &ctx).map(Block::Alpha),
            Kind::Stencil => blocks::read_stencil(&mut r, &ctx).map(Block::Stencil),
            Kind::ZBuffer => blocks::read_zbuffer(&mut r, &ctx).map(Block::ZBuffer),
            Kind::Other => Ok(Block::Other),
        };
        decoded.map_err(|source| Error::InBlock {
            index,
            type_name: type_name.to_string(),
            source: Box::new(source),
        })
    }
}
