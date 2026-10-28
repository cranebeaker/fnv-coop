// plugin/vats: the call-site patcher on a fake image that holds the pinned bytes of FalloutNV.exe 1.4.0.525 at every site and function entry,
// and the consistency of the address tables (vats_addrs.h). plugin/vats/tools/verify_pins.py checks the same tables against the real image.
#include <cstring>
#include <map>
#include <set>
#include "vats/vats_addrs.h"
#include "vats/vats_patch.h"
#include "minitest.h"

using namespace nvcoop::vats;
namespace ad = nvcoop::vats::addr;

namespace {

struct FakeImage : nvhooks::MemoryOps {
    std::map<uint32_t, uint8_t> mem;
    uint32_t failAddr = 0;
    void put(uint32_t a, const uint8_t* b, size_t n) { for (size_t i = 0; i < n; ++i) mem[a + static_cast<uint32_t>(i)] = b[i]; }
    bool read(nvhooks::Addr a, void* out, size_t n) override {
        for (size_t i = 0; i < n; ++i) {
            auto it = mem.find(a + static_cast<uint32_t>(i));
            if (it == mem.end()) return false;
            static_cast<uint8_t*>(out)[i] = it->second;
        }
        return true;
    }
    bool write(nvhooks::Addr a, const void* in, size_t n) override {
        if (failAddr && a <= failAddr && failAddr < a + n) return false;
        for (size_t i = 0; i < n; ++i) mem[a + static_cast<uint32_t>(i)] = static_cast<const uint8_t*>(in)[i];
        return true;
    }
    nvhooks::Addr allocExec(size_t) override { return 0; }
    void freeExec(nvhooks::Addr, size_t) override {}
    void call(uint32_t site, uint32_t target) {
        uint8_t b[5] = {0xE8};
        const uint32_t rel = ad::relTo(site, target);
        std::memcpy(b + 1, &rel, 4);
        put(site, b, 5);
    }
    uint32_t rel(uint32_t site) { uint8_t b[5] = {}; read(site, b, 5); uint32_t r; std::memcpy(&r, b + 1, 4); return r; }
};

struct Group { const ad::Site* sites; size_t n; uint32_t target; const uint8_t* prologue; };
std::vector<Group> groups() {
    return {
        {ad::kTimeScaleSites, ad::kTimeScaleSiteCount, ad::kSetTimeScale, ad::kSetTimeScalePrologue},
        {ad::kPlayerMultSites, ad::kPlayerMultSiteCount, ad::kGetPlayerTimeMult, ad::kGetTimeMultPrologue},
        {ad::kTargetMultSites, ad::kTargetMultSiteCount, ad::kGetTargetTimeMult, ad::kGetTimeMultPrologue},
        {ad::kPauseSitesMain, ad::kPauseSiteMainCount, ad::kMenuMode, ad::kMenuModePrologue},
        {ad::kPauseSitesExtended, ad::kPauseSiteExtendedCount, ad::kMenuMode, ad::kMenuModePrologue},
        {ad::kWorkerInterfaceSites, ad::kWorkerInterfaceSiteCount, ad::kInterfaceUpdate, ad::kInterfaceUpdatePrologue},
        {ad::kWorkerKickSites, ad::kWorkerKickSiteCount, ad::kWorkerKick, ad::kWorkerKickPrologue},
    };
}

FakeImage realImage() {
    FakeImage m;
    for (const Group& g : groups()) {
        m.put(g.target, g.prologue, 16);
        for (size_t i = 0; i < g.n; ++i) m.call(g.sites[i].site, g.target);
    }
    return m;
}
constexpr uint32_t kThunk = 0x10001000;

TEST(vats_patch, the_tables_have_the_sizes_the_re_notes_state_and_no_site_twice) {
    CHECK(ad::kTimeScaleSiteCount == 11);
    CHECK(ad::kPlayerMultSiteCount == 5);
    CHECK(ad::kTargetMultSiteCount == 9);
    CHECK(ad::kPauseSiteMainCount == 2);
    CHECK(ad::kPauseSiteExtendedCount == 9);
    // the worker threads' stage-0 interface update (both thread variants) and the main loop's one kick (docs: vats_addrs.h, the 0x800F66 crash)
    CHECK(ad::kWorkerInterfaceSiteCount == 2 && ad::kWorkerInterfaceSites[0].site == 0x008C7DB9 && ad::kWorkerInterfaceSites[1].site == 0x008C7BE9);
    CHECK(ad::kWorkerKickSiteCount == 1 && ad::kWorkerKickSites[0].site == 0x0086EC87);
    std::set<uint32_t> seen;
    for (const Group& g : groups())
        for (size_t i = 0; i < g.n; ++i) CHECK_MSG(seen.insert(g.sites[i].site).second, g.sites[i].name);
}

TEST(vats_patch, every_pinned_function_starts_like_a_standard_or_aligned_frame_and_every_site_is_inside_the_code_section) {
    for (const Group& g : groups()) {
        CHECK(g.prologue[0] == 0x55 && g.prologue[1] == 0x8B && g.prologue[2] == 0xEC);
        for (size_t i = 0; i < g.n; ++i) CHECK_MSG(g.sites[i].site >= 0x00401000u && g.sites[i].site < 0x00FDF000u, g.sites[i].name);
    }
    CHECK(ad::kVatsCamera == 0x011F2250);
    CHECK(ad::kVatsMenuId == 0x420);
    CHECK(ad::kMenuVtable + 4 * ad::kMenuVtblHandleClick == 0x010700E0);   // slot 3 of the VATSMenu vtable
}

TEST(vats_patch, relTo_is_the_x86_call_displacement) {
    CHECK(ad::relTo(0x1000, 0x1005) == 0);
    CHECK(ad::relTo(0x1000, 0x1000) == 0xFFFFFFFBu);
    CHECK(ad::relTo(0x9C6F3A, ad::kSetTimeScale) == 0x00AA4DB0u - (0x9C6F3Au + 5));
}

TEST(vats_patch, all_sites_of_every_group_are_redirected_and_restored_byte_for_byte) {
    for (const Group& g : groups()) {
        FakeImage m = realImage();
        FakeImage before = m;
        {
            SitePatcher p(m);
            const int n = p.install(g.sites, g.n, g.target, g.prologue, 16, kThunk);
            CHECK(n == static_cast<int>(g.n));
            CHECK(p.prologueOk());
            for (size_t i = 0; i < g.n; ++i) CHECK(m.rel(g.sites[i].site) == ad::relTo(g.sites[i].site, kThunk));
            CHECK(p.audit() == 0);
            CHECK(p.uninstall() == static_cast<int>(g.n));
        }
        CHECK(m.mem == before.mem);
    }
}

TEST(vats_patch, a_different_exe_build_is_refused_and_nothing_is_written) {
    const Group g = groups()[0];
    FakeImage m = realImage();
    m.mem[g.target + 3] ^= 0xFF;
    FakeImage before = m;
    SitePatcher p(m);
    CHECK(p.install(g.sites, g.n, g.target, g.prologue, 16, kThunk) == 0);
    CHECK(!p.prologueOk());
    CHECK(!p.lastError().empty());
    CHECK(m.mem == before.mem);
    for (const SiteResult& r : p.results()) CHECK(r.status == SiteStatus::Refused);
}

TEST(vats_patch, a_site_that_is_not_the_expected_call_is_left_alone_while_the_others_are_patched) {
    const Group g = groups()[0];
    FakeImage m = realImage();
    m.mem[g.sites[4].site + 2] ^= 0x01;                 // some other mod already moved this call
    const uint32_t other = m.rel(g.sites[4].site);
    SitePatcher p(m);
    CHECK(p.install(g.sites, g.n, g.target, g.prologue, 16, kThunk) == static_cast<int>(g.n) - 1);
    CHECK(m.rel(g.sites[4].site) == other);
    CHECK(p.results()[4].status == SiteStatus::WrongBytes);
    CHECK(p.results()[0].status == SiteStatus::Patched);
}

TEST(vats_patch, a_site_that_is_not_a_call_at_all_is_left_alone) {
    const Group g = groups()[1];
    FakeImage m = realImage();
    m.mem[g.sites[0].site] = 0xE9;                      // a jmp
    SitePatcher p(m);
    p.install(g.sites, g.n, g.target, g.prologue, 16, kThunk);
    CHECK(m.mem[g.sites[0].site] == 0xE9);
    CHECK(p.results()[0].status == SiteStatus::WrongBytes);
}

TEST(vats_patch, restoring_never_overwrites_somebody_elses_chained_call) {
    const Group g = groups()[3];
    FakeImage m = realImage();
    SitePatcher p(m);
    p.install(g.sites, g.n, g.target, g.prologue, 16, kThunk);
    const uint32_t chained = 0xDEADBEEF;
    for (int i = 0; i < 4; ++i) m.mem[g.sites[0].site + 1 + static_cast<uint32_t>(i)] = static_cast<uint8_t>(chained >> (8 * i));
    CHECK(p.audit() == 1);
    CHECK(p.uninstall() == static_cast<int>(g.n) - 1);
    CHECK(p.tampered() == 1);
    CHECK(m.rel(g.sites[0].site) == chained);           // left exactly as the other patcher wrote it
}

TEST(vats_patch, installing_twice_is_idempotent_and_a_write_failure_is_reported) {
    const Group g = groups()[2];
    FakeImage m = realImage();
    SitePatcher p(m);
    CHECK(p.install(g.sites, g.n, g.target, g.prologue, 16, kThunk) == static_cast<int>(g.n));
    CHECK(p.install(g.sites, g.n, g.target, g.prologue, 16, kThunk) == static_cast<int>(g.n));   // (uninstalls first, then patches again)
    p.uninstall();
    m.failAddr = g.sites[2].site + 2;
    CHECK(p.install(g.sites, g.n, g.target, g.prologue, 16, kThunk) == static_cast<int>(g.n) - 1);
    CHECK(p.results()[2].status == SiteStatus::WriteFailed);
}

TEST(vats_patch, an_unreadable_site_or_function_is_a_refusal_not_a_crash) {
    const Group g = groups()[0];
    FakeImage m;                                        // empty image: nothing is readable
    SitePatcher p(m);
    CHECK(p.install(g.sites, g.n, g.target, g.prologue, 16, kThunk) == 0);
    CHECK(!p.prologueOk());
    FakeImage m2 = realImage();
    for (int i = 0; i < 5; ++i) m2.mem.erase(g.sites[1].site + static_cast<uint32_t>(i));
    SitePatcher q(m2);
    CHECK(q.install(g.sites, g.n, g.target, g.prologue, 16, kThunk) == static_cast<int>(g.n) - 1);
    CHECK(q.results()[1].status == SiteStatus::ReadFailed);
}

TEST(vats_patch, game_settings_are_the_documented_objects) {
    CHECK(ad::kSettingPlayerTimeMult.object == 0x011CF374 && ad::kSettingPlayerTimeMult.defaultValue == 3.0f);
    CHECK(ad::kSettingTargetTimeMult.object == 0x011CFCEC && ad::kSettingTargetTimeMult.defaultValue == 1.0f);
    CHECK(ad::kSettingValueOffset == 4);
    CHECK(std::strcmp(ad::kSettingCamMaxTime.name, "fVATSCameraMaxTime") == 0);
}

}  // namespace
