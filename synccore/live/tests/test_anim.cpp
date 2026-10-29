// live-core: schema v2 animation extension (one-shot action group + counter + dead / knocked / swim / talk / block / ragdoll bits).
// The scenarios live in scenarios.h so the mutation twins (test_twins.cpp) run the very same checks against a broken codec.
#include "minitest.h"
#include "scenarios.h"

using namespace live_test;

TEST(anim, schema_version_is_three) { CHECK(kLiveSchemaVersion == 3); }

TEST(anim, extension_round_trips_in_full_and_delta_snapshots) { CHECK(animExtRoundTrips()); }

TEST(anim, a_lone_one_shot_counter_bump_is_sent) { CHECK(animSeqOnlyChangeSent()); }

TEST(anim, a_one_shot_in_a_lost_datagram_arrives_with_the_next_one) { CHECK(animOneShotSurvivesLoss()); }

TEST(anim, decoder_accepts_only_canonical_forms) { CHECK(animExtMalformedRejected()); }

TEST(anim, no_extension_means_the_v1_wire_form) {
    // A snapshot whose extension is all zero is byte-identical in size to a v1 snapshot; with an extension it is exactly 3 bytes longer.
    Pair a, b;
    ActorSnapshot plain = mk(44, 1, 1, 1000, 1, 2, 3);
    plain.anim.locomotion = 3; plain.anim.weaponDrawn = true;
    ActorSnapshot ext = plain;
    ext.anim.action = 32; ext.anim.actionSeq = 1;
    a.tx.queue(kHost, plain, plain.tick);
    b.tx.queue(kHost, ext, ext.tick);
    auto pa = a.tx.flush(kHost, plain.tick), pb = b.tx.flush(kHost, ext.tick);
    REQUIRE(pa.size() == 1 && pb.size() == 1);
    CHECK(pb[0].size() == pa[0].size() + 3);
    // the flags byte of the plain one has no extension bit
    CHECK(animHasExt(plain.anim) == false && animHasExt(ext.anim) == true);
    CHECK((packAnimFlags(ext.anim) & kAnimFlagExt) == 0);
}

TEST(anim, action_counter_compare_wraps) {
    CHECK(actionSeqNewer(1, 2));
    CHECK(!actionSeqNewer(2, 1));
    CHECK(!actionSeqNewer(7, 7));
    CHECK(actionSeqNewer(255, 0));      // wrap
    CHECK(actionSeqNewer(250, 5));      // wrap
    CHECK(!actionSeqNewer(5, 250));
    CHECK(actionSeqNewer(0, 127));
    CHECK(!actionSeqNewer(0, 128));     // exactly half the range is "older"
}

TEST(anim, interpolation_buffer_carries_the_extension_of_the_older_snapshot) {
    // The anim state is a zero order hold of the snapshot the render time has passed, so a one-shot shows at the same moment the
    // interpolated position reaches the snapshot that announced it.
    InterpBuffer ib;
    for (uint32_t k = 0; k < 6; ++k) {
        ActorSnapshot s = mk(45, 1, k + 1, 1000 + k * 50, k * 10.0f);
        if (k >= 3) { s.anim.action = 223; s.anim.actionSeq = 1; }
        ib.push(s, s.tick + 20);
    }
    // render time = now - 20 (offset) - 100 (delay)
    InterpOut early = ib.sample(1000 + 100 + 20 + 90);    // render at tick 1090: before the snapshot at 1150
    InterpOut late = ib.sample(1000 + 100 + 20 + 160);    // render at tick 1160: after it
    CHECK(early.anim.action == 0 && early.anim.actionSeq == 0);
    CHECK(late.anim.action == 223 && late.anim.actionSeq == 1);
}

// ---- schema v3: the furniture block (sit state, bed flag, furniture reference) ----------------------------------------------------------
TEST(anim, furniture_block_round_trips_in_full_and_delta_snapshots) { CHECK(sitRoundTrips()); }

TEST(anim, a_furniture_only_change_is_sent) { CHECK(sitOnlyChangeSent()); }

TEST(anim, a_sit_in_a_lost_datagram_arrives_with_the_next_one) { CHECK(sitSurvivesLoss()); }

TEST(anim, no_furniture_means_the_v2_wire_form) { CHECK(sitWireSizes()); }

TEST(anim, furniture_block_has_one_canonical_form) { CHECK(sitCanonicalForms()); }

TEST(anim, decoder_accepts_only_canonical_furniture_blocks) { CHECK(sitMalformedRejected()); }

TEST(anim, furniture_block_helpers_pack_and_reject) {
    AnimState a;
    CHECK(packAnimSit(a) == 0);
    a.sit = SitState::Using; a.furniture = 5;
    CHECK(packAnimSit(a) == 2);
    a.sleep = true;
    CHECK(packAnimSit(a) == 6);
    a.sit = SitState::None;
    CHECK(packAnimSit(a) == 0);                      // a bed flag alone is nothing
    AnimState b;
    CHECK(unpackAnimSit(b, 7, 9) && b.sit == SitState::Leaving && b.sleep && b.furniture == 9);
    CHECK(unpackAnimSit(b, 0, 0) && b.sit == SitState::None && !b.sleep && b.furniture == 0);
    CHECK(!unpackAnimSit(b, 4, 0) && !unpackAnimSit(b, 0x0A, 0) && !unpackAnimSit(b, 0, 3));
}

TEST(anim, interpolation_buffer_carries_the_furniture_block_of_the_older_snapshot) {
    // Like the v2 extension, the sit state is a zero order hold: the copy sits down when the interpolated body reaches the snapshot that says so.
    InterpBuffer ib;
    for (uint32_t k = 0; k < 6; ++k) {
        ActorSnapshot s = mk(55, 1, k + 1, 1000 + k * 50, 0);
        if (k >= 3) s.anim = sittingAnim(SitState::Using, 0x555);
        ib.push(s, s.tick + 20);
    }
    InterpOut early = ib.sample(1000 + 100 + 20 + 90);
    InterpOut late = ib.sample(1000 + 100 + 20 + 160);
    CHECK(early.anim.sit == SitState::None && early.anim.furniture == 0);
    CHECK(late.anim.sit == SitState::Using && late.anim.furniture == 0x555);
}
