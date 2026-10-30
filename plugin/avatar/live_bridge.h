// Glue between synccore/live (snapshots, InterpBuffer) and the puppet driver. Header only; include dirs needed:
// <repo>/synccore and <repo>/synccore/include. Kept apart from puppet_manager.h so the driver itself stays free of the
// network layer.
//
//   receiver machine, each frame:
//     InterpOut o = buf.sample(now, actual ? &actualPose : nullptr);     // buf = the InterpBuffer of that player's netId
//     pm.SetPose(slot, avatar::FromInterp(o));  pm.Tick(now);
//   sender machine (the local player), at 20 Hz:
//     ActorSnapshot s = avatar::MakeLocalSnapshot(netId, epoch, seq, tick, cell, pos, rot, vel, anim);
#pragma once
#include "avatar_faults.h"
#include "avatar_types.h"
#include "loco.h"
#include "live/live.h"

namespace nvcoop::avatar {

namespace live = ::fnvcoop::live;

inline Anim FromLive(const live::AnimState& a) {
    Anim o;
    o.loco = a.locomotion;
    o.weaponDrawn = a.weaponDrawn;
    o.sneak = a.sneak;
    o.jump = a.jump;
    o.reload = a.reload;
    o.aim = a.aim;
    if (!avatarFaults().bridgeDropsAction) {
        o.action = a.action;
        o.actionSeq = a.actionSeq;
        o.block = a.block;
    }
    if (!avatarFaults().bridgeDropsSit) {
        o.sit = static_cast<uint8_t>(static_cast<uint8_t>(a.sit) & 3);
        o.sleep = a.sleep && o.sit != 0;
        o.furniture = o.sit ? a.furniture : 0;
    }
    return o;
}

inline live::AnimState ToLive(const Anim& a) {
    live::AnimState o;
    o.locomotion = a.loco;
    o.weaponDrawn = a.weaponDrawn;
    o.sneak = a.sneak;
    o.jump = a.jump;
    o.reload = a.reload;
    o.aim = a.aim;
    if (!avatarFaults().bridgeDropsAction) {
        o.action = a.action;
        o.actionSeq = a.actionSeq;
        o.block = a.block;
    }
    if (!avatarFaults().bridgeDropsSit) {
        o.sit = static_cast<live::SitState>(a.sit & 3);
        o.sleep = a.sleep && o.sit != live::SitState::None;
        o.furniture = o.sit != live::SitState::None ? a.furniture : 0;
    }
    return o;
}

inline Vec3 FromLive(const live::Vec3& v) { return Vec3{v.x, v.y, v.z}; }
inline live::Vec3 ToLive(const Vec3& v) { live::Vec3 o; o.x = v.x; o.y = v.y; o.z = v.z; return o; }

// InterpOut -> pose. Holding / Empty (the buffer has run dry) is `frozen`; Snapped marks a jump.
inline PuppetPose FromInterp(const live::InterpOut& o) {
    PuppetPose p;
    p.valid = o.state != live::InterpState::Empty;
    p.cell = o.cell;
    p.pos = FromLive(o.pos);
    p.rot = FromLive(o.rot);
    p.vel = FromLive(o.vel);
    p.anim = FromLive(o.anim);
    p.snapped = o.snapped || o.state == live::InterpState::Snapped;
    p.frozen = o.state == live::InterpState::Holding || o.state == live::InterpState::Empty;
    return p;
}

// Sender side: the local player's reads -> the snapshot that goes on the wire. `rot` = (pitch, yaw, roll) with yaw the
// REFR rotZ heading in radians; `read` supplies the mover flags for the locomotion code.
inline live::ActorSnapshot MakeLocalSnapshot(live::NetId netId, live::OwnerEpoch epoch, uint32_t seq, uint32_t tick, live::CellId cell,
                                             const Vec3& pos, const Vec3& rot, const Vec3& vel, Anim anim, const LocalRead* read = nullptr) {
    if (read) {
        LocalRead r = *read;
        r.vel = vel;
        r.yaw = rot.y;
        anim.loco = CaptureLoco(r);
    }
    live::ActorSnapshot s;
    s.netId = netId;
    s.ownerEpoch = epoch;
    s.seq = seq;
    s.tick = tick;
    s.cell = cell;
    s.pos = ToLive(pos);
    s.rot = ToLive(rot);
    s.vel = ToLive(vel);
    s.anim = ToLive(anim);
    return s;
}

}  // namespace nvcoop::avatar
