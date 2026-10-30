// The one datagram of the parity channel (first byte 0x9E): a guest's compact sample, guest -> host, reliable. Game-free.
//
//   u8 0x9E  u8 version(1)  u8 kind(1 = Sample)
//   u8 slot  varu seq  varu cell  varu ws  varu cov  u8 flags (bit 0 menu)  u8 n (<= 64)
//   n x entity:
//     u8 head  bits 0..1 kind (0 Npc, 1 Puppet, 2 Self), bit 2 seated, bit 3 dead, bit 4 disabled, bit 5 look follows, bit 6 acts follow (Puppet / Self)
//     [u8 slot]                       Puppet / Self only
//     varu ref   vari x  vari y  vari z   u8 speed16   u8 hp   u8 group
//     [varu acts]                     when bit 6
//     [varu race  u8 female  varu hair  varu eyes  u32 faceHash]    when bit 5
//   u32 crc32 LE over everything before it
// A sample of 30 actors and 2 players is about 600 bytes. The channel is not part of the Hello handshake (like 0xAC, 0xCB, 0xDA): a build without the module drops it.
#pragma once
#include <cstddef>
#include <cstdint>

#include "fnvcoop/transport.h"
#include "parity/parity_types.h"

namespace nvcoop::parity {

constexpr size_t kMaxDatagramBytes = 1800;

// Writes the sample as it is (at most kMaxEnts entities): the sender trims it first with netSample() in parity_sampler.h, which also narrows `cov`.
fnvcoop::Bytes encodeSample(const Sample& s);
bool decodeSample(const uint8_t* data, size_t size, Sample& out);

}  // namespace nvcoop::parity
