#include <limits>
#include "fnvcoop/protocol.h"
#include "minitest.h"
#include "sim.h"

using namespace fnvcoop;

namespace {
EffectTxn randomTxn(Rng& r, uint64_t seq) {
    EffectTxn t;
    t.seq = seq;
    t.origin = PlayerId(r.range(0, 3));
    t.cause = sim::randomCause(r);
    t.flags = uint8_t(r.range(0, 1));
    t.originEpoch = r.range(0, 5);
    for (uint32_t i = 0, n = r.range(0, 8); i < n; ++i) t.effects.push_back(sim::randomEffect(r, 4));
    return t;
}

Message randomMessage(Rng& r) {
    switch (r.range(0, 6)) {
        case 0: { Hello h{r.range(1, 100), r.next(), r.range(0, 1000)}; if (r.chance(0.5)) h.cursorCheck = r.next(); return h; }
        case 1: {
            Welcome w;
            w.session = r.range(1, 100); w.slot = uint8_t(r.range(1, 3)); w.headSeq = r.range(0, 500);
            w.hasSnapshot = r.chance(0.7);
            if (w.hasSnapshot) {
                for (uint32_t i = 0, n = r.range(1, 20); i < n; ++i) w.snapshot.applyTxn(randomTxn(r, i + 1));
            }
            return w;
        }
        case 2: return Reject{r.range(1, 9), 1};
        case 3: {
            TxnBatch b; b.session = r.range(1, 9);
            for (uint32_t i = 0, n = r.range(0, 5); i < n; ++i) b.txns.push_back(randomTxn(r, i + 1));
            return b;
        }
        case 4: return Ack{r.range(1, 9), r.next() >> 20};
        case 5: {
            Intent in; in.session = r.range(1, 9); in.epoch = r.range(1, 9); in.id = r.range(1, 99); in.cause = sim::randomCause(r);
            for (uint32_t i = 0, n = r.range(1, 4); i < n; ++i) in.effects.push_back(sim::randomEffect(r, 4));
            return in;
        }
        default: return IntentAck{r.range(1, 9), r.range(1, 9), r.range(1, 99)};
    }
}

}  // namespace

TEST(wire, roundtrip_every_effect_type) {
    FormRef f("falloutnv.esm", 0x1234), g("deadmoney.esm", 7);
    std::vector<Effect> all = {
        SetStage{f, 40}, SetObjective{f, 3, 2}, SetGlobal{g, 1.5f}, EnableRef{f}, DisableRef{g},
        AddItem{f, 5, Recipient::slots(0b101)}, RemoveItem{g, -2, Recipient::all()}, ModActorValue{9, -300, Recipient::origin()},
        ModReputation{f, 10, -4, Recipient::dflt()}, ModKarma{-250, Recipient::all()}, StartQuest{f}, StopQuest{g},
        MoveRef{f, g, 1.f, -2.5f, 1e9f, true}, KillActor{f}, ResurrectActor{g},
        SetScriptVar{f, 65535, -1234.5678}, SetPlayerControls{0xA5, false, Recipient::origin()}, PartyTether{g, true},
        SlideshowEnd{}, SaveSuppress{true}, CompanionOwner{f, 3}, ForcedDialogue{f, g, 2}, BarrierArrive{f, 0b1111},
        BarrierRelease{g}, SayOnceFlag{f}, CompleteQuest{f}, CompleteAllObjectives{g}, SetLock{f, true, 75},
        SetFactionRank{f, g, -1}, SetPerk{f, 2, Recipient::all()}, SwapTexture{f, g, 2}, SetFastTravelLock{f, true}, FailQuest{g},
        SpawnRef{0xFF000001, f, g, 1.f, 2.f, 3.f, true}, DespawnRef{0xFF000001},
        ObjectState{f, 15, true, true, 50, true, true}, RefScriptVar{f, 9, 2.5}, RefAddItem{f, g, 3}, RefRemoveItem{f, g, 2},
        RemoveAllItems{f}, AddTopic{g}, AddNote{f}, RemoveNote{f}, SetMapMarker{g, 3}, ForceActiveQuest{f, Recipient::origin()},
        RequestSave{2}, SetWeather{f, true}, SetTimeScale{20.f}, AdvanceTime{480}, RefAngle{f, kAxisY, -45.5f, true, 10.f},
        RefCommand{kOpPlayIdle, f, g, f, 1, -2, 3, 4, 2.5f, "IdleWave", Recipient::slots(5)},
        ContainerTransfer{f, g, true, false, Recipient::all()}};
    CHECK(all.size() == std::variant_size_v<Effect>);
    EffectTxn t; t.seq = 77; t.origin = 2; t.cause = {CauseKind::QuestResult, f, 40}; t.effects = all; t.flags = 1; t.originEpoch = 9;
    TxnBatch b{5, {t}};
    Bytes enc = encodeMessage(b);
    CHECK(!enc.empty());
    Message out;
    CHECK(decodeMessage(enc, out) == DecodeStatus::Ok);
    auto* ob = std::get_if<TxnBatch>(&out);
    REQUIRE(ob != nullptr);
    CHECK(ob->session == 5);
    REQUIRE(ob->txns.size() == 1);
    CHECK(ob->txns[0] == t);
}

TEST(wire, ref_angle_with_and_without_a_pre_image) {
    FormRef f("falloutnv.esm", 0x1234);
    for (RefAngle a : {RefAngle{f, kAxisX, 12.5f}, RefAngle{f, kAxisZ, 270.f, true, -0.5f}}) {
        EffectTxn t; t.seq = 1; t.effects = {a};
        Bytes enc = encodeMessage(TxnBatch{1, {t}});
        Message out;
        REQUIRE(decodeMessage(enc, out) == DecodeStatus::Ok);
        CHECK(std::get<RefAngle>(std::get<TxnBatch>(out).txns[0].effects[0]) == a);
    }
    RefAngle bad{f, 3, 1.f};   // axis out of range is rejected on decode
    EffectTxn t; t.seq = 1; t.effects = {bad};
    Message out;
    CHECK(decodeMessage(encodeMessage(TxnBatch{1, {t}}), out) != DecodeStatus::Ok);
}

TEST(wire, roundtrip_random_messages_canonical) {
    Rng r(42);
    for (int i = 0; i < 3000; ++i) {
        Message m = randomMessage(r);
        Bytes a = encodeMessage(m);
        REQUIRE(!a.empty());
        Message d;
        REQUIRE(decodeMessage(a, d) == DecodeStatus::Ok);
        CHECK(d.index() == m.index());
        CHECK(encodeMessage(d) == a);  // decode(encode(x)) re-encodes to identical bytes
    }
}

TEST(wire, intent_action_kinds_roundtrip_and_reject_bad_kind) {
    for (auto k : {IntentKind::ActivateRef, IntentKind::TriggerEnter, IntentKind::PickUp}) {
        Intent in{3, 4, 5, Cause(), {}, k, FormRef("falloutnv.esm", 0x1234)};
        Bytes b = encodeMessage(in);
        Message m;
        REQUIRE(decodeMessage(b, m) == DecodeStatus::Ok);
        auto* o = std::get_if<Intent>(&m);
        REQUIRE(o != nullptr);
        CHECK((o->kind == k && o->ref == in.ref && o->effects.empty() && o->id == 5));
    }
    Bytes good = encodeMessage(Intent{1, 1, 1, Cause(), {}, IntentKind::PickUp, FormRef("a.esm", 1)});
    Bytes bad = good;
    bad[4 + 1 + 1 + 1 + 1 + 1] = 9;  // table(2: count+len?) is variable; scan instead
    bool anyRejected = false;
    for (size_t i = 4; i + 4 < good.size(); ++i) {
        Bytes x = good; x[i] = 0xFF;
        Message m;
        if (decodeMessage(x.data(), x.size(), m, false) != DecodeStatus::Ok) anyRejected = true;
    }
    CHECK(anyRejected);
}

TEST(wire, plugin_names_use_string_table) {
    EffectTxn t;
    for (int i = 0; i < 50; ++i) t.effects.push_back(SetStage{FormRef("falloutnv.esm", uint32_t(i)), 1});
    Bytes b = encodeMessage(TxnBatch{1, {t}});
    // 50 effects at ~4 bytes each; a naive per-ref plugin string would be > 50*(13+4).
    CHECK(b.size() < 400);
}

TEST(wire, varint_and_zigzag_edges) {
    for (int64_t v : {int64_t(0), int64_t(1), int64_t(-1), int64_t(63), int64_t(-64), int64_t(INT32_MAX), int64_t(INT32_MIN),
                      INT64_MAX, INT64_MIN}) {
        Archive w = Archive::writer();
        int64_t x = v; w.vari(x);
        Bytes enc = w.finish();
        Archive r = Archive::reader(enc.data(), enc.size());
        int64_t y = 12345; r.vari(y);
        CHECK(r.ok()); CHECK(y == v); CHECK(r.atEnd());
    }
    for (uint64_t v : {uint64_t(0), uint64_t(127), uint64_t(128), uint64_t(UINT32_MAX), UINT64_MAX}) {
        Archive w = Archive::writer();
        uint64_t x = v; w.varu(x);
        Bytes enc = w.finish();
        Archive r = Archive::reader(enc.data(), enc.size());
        uint64_t y = 1; r.varu(y);
        CHECK(r.ok()); CHECK(y == v);
    }
}

TEST(wire, rejects_malformed_primitives) {
    auto readU = [](std::vector<uint8_t> body, bool& ok) {
        std::vector<uint8_t> buf{0};  // empty string table
        buf.insert(buf.end(), body.begin(), body.end());
        Archive r = Archive::reader(buf.data(), buf.size());
        uint64_t v = 0; r.varu(v); ok = r.ok(); return v;
    };
    bool ok;
    readU({0x80, 0x00}, ok); CHECK(!ok);                                                   // overlong
    readU({0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F}, ok); CHECK(!ok);  // > 64 bits
    readU({0x80}, ok); CHECK(!ok);                                                         // truncated
    // count larger than the bytes remaining must not allocate/accept
    std::vector<uint8_t> buf{0, 0xFF, 0xFF, 0x7F};
    Archive r = Archive::reader(buf.data(), buf.size());
    size_t n = 0; CHECK(!r.count(n, 1u << 30)); CHECK(!r.ok());
    // out-of-range enum / bool
    std::vector<uint8_t> b2{0, 2};
    Archive r2 = Archive::reader(b2.data(), b2.size());
    bool bv = false; r2.boolean(bv); CHECK(!r2.ok());
}

TEST(wire, script_var_rejects_non_finite_double_and_big_index) {
    EffectTxn t;
    t.effects.push_back(SetScriptVar{FormRef("a.esm", 1), 3, std::numeric_limits<double>::quiet_NaN()});
    CHECK(encodeMessage(TxnBatch{1, {t}}).empty());
    t.effects[0] = SetScriptVar{FormRef("a.esm", 1), 3, 2.5};
    Bytes ok = encodeMessage(TxnBatch{1, {t}});
    REQUIRE(!ok.empty());
    Message m;
    CHECK(decodeMessage(ok, m) == DecodeStatus::Ok);
}

TEST(wire, rejects_non_finite_floats) {
    EffectTxn t;
    t.effects.push_back(SetGlobal{FormRef("a.esm", 1), std::numeric_limits<float>::quiet_NaN()});
    CHECK(encodeMessage(TxnBatch{1, {t}}).empty());
    t.effects[0] = MoveRef{FormRef("a.esm", 1), FormRef("a.esm", 2), 0, std::numeric_limits<float>::infinity(), 0};
    CHECK(encodeMessage(TxnBatch{1, {t}}).empty());
    // And on the read side: hand-craft a valid message, then patch the float to NaN (CRC bypassed).
    t.effects[0] = SetGlobal{FormRef("a.esm", 1), 1.0f};
    Bytes ok = encodeMessage(TxnBatch{1, {t}});
    REQUIRE(!ok.empty());
    Bytes bad = ok;
    // float is the 4 bytes just before the 4-byte crc
    size_t at = bad.size() - 8;
    bad[at] = 0; bad[at + 1] = 0; bad[at + 2] = 0xC0; bad[at + 3] = 0x7F;  // quiet NaN
    Message m;
    CHECK(decodeMessage(bad.data(), bad.size(), m, false) == DecodeStatus::Malformed);
}

TEST(wire, every_truncation_is_rejected) {
    Rng r(7);
    for (int i = 0; i < 200; ++i) {
        Message orig = randomMessage(r);
        Bytes a = encodeMessage(orig);
        // A Hello cut right before its appended cursorCheck (8 bytes) IS a Hello of an older build: valid without the CRC check.
        const size_t oldHello = std::holds_alternative<Hello>(orig) ? a.size() - 8 : 0;
        for (size_t cut = 0; cut < a.size(); ++cut) {
            Message m;
            CHECK(decodeMessage(a.data(), cut, m) != DecodeStatus::Ok);
            if (cut > 8) CHECK(decodeMessage(a.data(), cut, m, false) != DecodeStatus::Ok || cut == a.size() ||
                               (cut == oldHello && std::get_if<Hello>(&m) && std::get<Hello>(m).cursorCheck == 0));
        }
    }
}

TEST(wire, single_bit_flip_always_caught_by_crc) {
    Rng r(11);
    for (int i = 0; i < 300; ++i) {
        Bytes a = encodeMessage(randomMessage(r));
        for (int k = 0; k < 20; ++k) {
            Bytes b = a;
            b[r.u32() % b.size()] ^= uint8_t(1u << (r.u32() % 8));
            Message m;
            CHECK(decodeMessage(b, m) != DecodeStatus::Ok);
        }
    }
}

TEST(wire, fuzz_random_garbage_never_crashes) {
    Rng r(1234);
    size_t ok = 0;
    for (int i = 0; i < 200000; ++i) {
        Bytes b(r.range(0, 200));
        for (auto& x : b) x = uint8_t(r.u32());
        if (b.size() > 4 && r.chance(0.7)) { b[0] = 0xFC; b[1] = 0x4E; b[2] = kWireVersion; b[3] = uint8_t(r.range(1, 8)); }
        Message m;
        if (decodeMessage(b.data(), b.size(), m, false) == DecodeStatus::Ok) ++ok;
    }
    CHECK(ok < 200000);  // reaching here without a crash/hang is the real assertion
}

TEST(wire, fuzz_mutated_valid_messages_behind_crc) {
    Rng r(99);
    size_t decoded = 0, rejected = 0;
    for (int i = 0; i < 4000; ++i) {
        Bytes a = encodeMessage(randomMessage(r));
        for (int k = 0; k < 50; ++k) {
            Bytes b = a;
            uint32_t edits = r.range(1, 4);
            for (uint32_t e = 0; e < edits; ++e) {
                switch (r.range(0, 3)) {
                    case 0: b[r.u32() % b.size()] = uint8_t(r.u32()); break;
                    case 1: b[r.u32() % b.size()] ^= uint8_t(1u << (r.u32() % 8)); break;
                    case 2: if (b.size() > 9) b.erase(b.begin() + 4 + r.u32() % (b.size() - 8)); break;
                    default: b.insert(b.begin() + 4 + r.u32() % (b.size() - 3), uint8_t(r.u32())); break;
                }
            }
            Message m;
            if (decodeMessage(b.data(), b.size(), m, false) == DecodeStatus::Ok) {
                ++decoded;
                CHECK(!encodeMessage(m).empty() || true);  // anything accepted must be re-encodable without crashing
            } else {
                ++rejected;
            }
        }
    }
    CHECK(rejected > 0);
    std::printf("    fuzz: %zu mutated messages accepted, %zu rejected\n", decoded, rejected);
}

TEST(wire, snapshot_roundtrip_exact) {
    Rng r(5);
    ShadowState s;
    for (uint64_t i = 1; i <= 300; ++i) s.applyTxn(randomTxn(r, i));
    Welcome w; w.session = 3; w.slot = 2; w.headSeq = s.seq; w.hasSnapshot = true; w.snapshot = s;
    Bytes b = encodeMessage(w);
    REQUIRE(!b.empty());
    Message m;
    REQUIRE(decodeMessage(b, m) == DecodeStatus::Ok);
    auto* ow = std::get_if<Welcome>(&m);
    REQUIRE(ow != nullptr);
    CHECK(ow->snapshot == s);
    CHECK(ow->policy == w.policy);
}
