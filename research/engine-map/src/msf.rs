//! In-memory MSF 7.00 repack into 4096-byte pages, the same container
//! rewrite as `research/pdb/msf_repack.py`. The Xbox prototype PDBs use
//! 1024-byte pages and a stream directory that needs several block-map
//! pages, which the `pdb` crate does not read. Stream contents are copied
//! unchanged.

const MAGIC: &[u8; 32] = b"Microsoft C/C++ MSF 7.00\r\n\x1aDS\0\0\0";

fn u32_at(d: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(d[o..o + 4].try_into().unwrap())
}

pub fn repack(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 56 || &data[..32] != MAGIC {
        return Err("not an MSF 7.00 file".into());
    }
    let bs = u32_at(data, 32) as usize;
    if bs == 4096 {
        return Ok(data.to_vec());
    }
    let dir_bytes = u32_at(data, 44) as usize;
    let n_dir_blocks = dir_bytes.div_ceil(bs);
    let n_map_blocks = (n_dir_blocks * 4).div_ceil(bs);
    let mut dir_list = Vec::new();
    for i in 0..n_map_blocks {
        let mb = u32_at(data, 52 + 4 * i) as usize;
        for k in 0..bs / 4 {
            dir_list.push(u32_at(data, mb * bs + 4 * k) as usize);
        }
    }
    dir_list.truncate(n_dir_blocks);
    let mut dir = Vec::with_capacity(n_dir_blocks * bs);
    for b in dir_list {
        dir.extend_from_slice(&data[b * bs..(b + 1) * bs]);
    }
    dir.truncate(dir_bytes);
    let n_streams = u32_at(&dir, 0) as usize;
    let mut pos = 4 + 4 * n_streams;
    let mut streams: Vec<Option<Vec<u8>>> = Vec::with_capacity(n_streams);
    for i in 0..n_streams {
        let size = u32_at(&dir, 4 + 4 * i);
        if size == u32::MAX {
            streams.push(None);
            continue;
        }
        let size = size as usize;
        let mut s = Vec::with_capacity(size);
        for _ in 0..size.div_ceil(bs) {
            let b = u32_at(&dir, pos) as usize;
            pos += 4;
            s.extend_from_slice(&data[b * bs..(b + 1) * bs]);
        }
        s.truncate(size);
        streams.push(Some(s));
    }

    const BS: usize = 4096;
    let mut next = 3usize;
    let mut alloc = || {
        while next % BS == 1 || next % BS == 2 {
            next += 1;
        }
        next += 1;
        next - 1
    };
    let mut pages: Vec<(usize, Vec<u8>)> = Vec::new();
    let mut stream_blocks = Vec::new();
    for s in &streams {
        let mut blocks = Vec::new();
        if let Some(s) = s {
            for c in s.chunks(BS) {
                let b = alloc();
                let mut p = c.to_vec();
                p.resize(BS, 0);
                pages.push((b, p));
                blocks.push(b as u32);
            }
        }
        stream_blocks.push(blocks);
    }
    let mut d = Vec::new();
    d.extend_from_slice(&(streams.len() as u32).to_le_bytes());
    for s in &streams {
        let n = s.as_ref().map(|s| s.len() as u32).unwrap_or(u32::MAX);
        d.extend_from_slice(&n.to_le_bytes());
    }
    for bl in &stream_blocks {
        for b in bl {
            d.extend_from_slice(&b.to_le_bytes());
        }
    }
    let mut dir_blocks = Vec::new();
    for c in d.chunks(BS) {
        let b = alloc();
        let mut p = c.to_vec();
        p.resize(BS, 0);
        pages.push((b, p));
        dir_blocks.push(b as u32);
    }
    if dir_blocks.len() * 4 > BS {
        return Err("directory still too large for one map page".into());
    }
    let map_block = alloc();
    let mut mp = Vec::new();
    for b in &dir_blocks {
        mp.extend_from_slice(&b.to_le_bytes());
    }
    mp.resize(BS, 0);
    pages.push((map_block, mp));
    let nblocks = next;
    let mut out = vec![0u8; nblocks * BS];
    for (b, p) in pages {
        out[b * BS..(b + 1) * BS].copy_from_slice(&p);
    }
    let mut sb = MAGIC.to_vec();
    for v in [BS, 1, nblocks, d.len(), 0, map_block] {
        sb.extend_from_slice(&(v as u32).to_le_bytes());
    }
    out[..sb.len()].copy_from_slice(&sb);
    Ok(out)
}
