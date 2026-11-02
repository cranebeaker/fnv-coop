"""Repack an MSF (PDB) file into 4096-byte pages so that llvm-pdbutil can read it.

The Xbox 360 prototype PDBs use 1024-byte pages and a stream directory too
large for one block-map page (the superblock then lists several block-map
pages). LLVM only accepts a single block-map page. The streams' contents are
copied unchanged; only the container is rewritten.

Usage: python msf_repack.py <in.pdb> <out.pdb>
"""
import struct
import sys

MAGIC = b"Microsoft C/C++ MSF 7.00\r\n\x1aDS\0\0\0"


def read_msf(data):
    assert data[:32] == MAGIC, "not an MSF 7.00 file"
    bs, fpm, nblocks, dir_bytes, _unk = struct.unpack_from("<5I", data, 32)
    n_dir_blocks = -(-dir_bytes // bs)
    n_map_blocks = -(-(n_dir_blocks * 4) // bs)
    map_blocks = struct.unpack_from(f"<{n_map_blocks}I", data, 52)
    dir_block_list = []
    for mb in map_blocks:
        dir_block_list += struct.unpack_from(f"<{bs // 4}I", data, mb * bs)
    dir_block_list = dir_block_list[:n_dir_blocks]
    directory = b"".join(data[b * bs:(b + 1) * bs] for b in dir_block_list)[:dir_bytes]
    n_streams = struct.unpack_from("<I", directory, 0)[0]
    sizes = struct.unpack_from(f"<{n_streams}I", directory, 4)
    pos = 4 + 4 * n_streams
    streams = []
    for size in sizes:
        if size == 0xFFFFFFFF:
            streams.append(None)
            continue
        nb = -(-size // bs)
        blocks = struct.unpack_from(f"<{nb}I", directory, pos)
        pos += 4 * nb
        streams.append(b"".join(data[b * bs:(b + 1) * bs] for b in blocks)[:size])
    return bs, streams


def write_msf(streams, bs=4096):
    out = bytearray()
    # Page 0 superblock, pages 1 and 2 free-page maps; every `bs` pages
    # later another pair of FPM pages sits at offsets 1 and 2 of the interval.
    next_block = 3
    pages = {}

    def alloc():
        nonlocal next_block
        while next_block % bs in (1, 2):
            next_block += 1
        b = next_block
        next_block += 1
        return b

    stream_blocks = []
    for s in streams:
        if s is None:
            stream_blocks.append(None)
            continue
        blocks = []
        for i in range(0, len(s), bs):
            b = alloc()
            pages[b] = s[i:i + bs].ljust(bs, b"\0")
            blocks.append(b)
        stream_blocks.append(blocks)
    directory = bytearray(struct.pack("<I", len(streams)))
    for s in streams:
        directory += struct.pack("<I", 0xFFFFFFFF if s is None else len(s))
    for blocks in stream_blocks:
        if blocks:
            directory += struct.pack(f"<{len(blocks)}I", *blocks)
    dir_blocks = []
    for i in range(0, len(directory), bs):
        b = alloc()
        pages[b] = bytes(directory[i:i + bs]).ljust(bs, b"\0")
        dir_blocks.append(b)
    assert len(dir_blocks) * 4 <= bs, "directory still too large for one map page"
    map_block = alloc()
    pages[map_block] = struct.pack(f"<{len(dir_blocks)}I", *dir_blocks).ljust(bs, b"\0")
    nblocks = next_block
    # Free page maps: one bit per page, 1 = free. Mark everything used.
    for interval in range(0, -(-nblocks // bs)):
        for k in (1, 2):
            b = interval * bs + k
            if b < nblocks:
                pages[b] = b"\0" * bs
    super_block = MAGIC + struct.pack("<6I", bs, 1, nblocks, len(directory), 0, map_block)
    pages[0] = super_block.ljust(bs, b"\0")
    for b in range(nblocks):
        out += pages.get(b, b"\0" * bs)
    return bytes(out)


def main():
    data = open(sys.argv[1], "rb").read()
    bs, streams = read_msf(data)
    print(f"read {len(streams)} streams from {bs}-byte pages")
    open(sys.argv[2], "wb").write(write_msf(streams))
    print("wrote", sys.argv[2])


if __name__ == "__main__":
    main()
