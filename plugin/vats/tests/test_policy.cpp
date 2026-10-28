// plugin/vats: the real-time policy (what the three engine hooks decide) and the shared types.
#include <limits>
#include "vats/vats_policy.h"
#include "minitest.h"

using namespace nvcoop::vats;

namespace {

PolicyInput session(Mode mode = Mode::TargetSelect, bool menu = true) {
    PolicyInput in;
    in.sessionActive = true;
    in.mode = mode;
    in.vatsMenuOpen = menu;
    return in;
}

TEST(vats_policy, without_a_session_vats_is_the_vanilla_game) {
    PolicyInput in = session();
    in.sessionActive = false;
    CHECK(!realtimeActive(in));
    CHECK(menuModeForWorld(in, true) == true);          // the world pauses as in the single-player game
    CHECK(scaleForVatsRequest(in, 0.0001f) == 0.0001f);  // the engine's freeze request goes through
    CHECK(timeMultFor(in, 3.0) == 3.0);                  // the player still runs at 3x in playback
}

TEST(vats_policy, with_realtime_switched_off_a_session_does_not_change_vats) {
    PolicyInput in = session();
    in.settings.realtime = false;
    CHECK(!realtimeActive(in));
    CHECK(menuModeForWorld(in, true));
    CHECK(scaleForVatsRequest(in, 0.3f) == 0.3f);
    CHECK(timeMultFor(in, 3.0) == 3.0);
}

TEST(vats_policy, the_menu_pause_is_lifted_only_for_the_vats_menu) {
    for (Mode m : {Mode::TargetSelect, Mode::LimbSelect, Mode::Mode3, Mode::Playback}) {
        PolicyInput in = session(m, true);
        CHECK_MSG(!menuModeForWorld(in, true), toString(m));      // the world keeps running while VATS is open
    }
    PolicyInput other = session(Mode::None, false);               // Pip-Boy, dialogue, barter: not VATS
    CHECK(menuModeForWorld(other, true));
    PolicyInput menuNoMode = session(Mode::None, true);           // the menu is closing: this frame is still paused
    CHECK(menuModeForWorld(menuNoMode, true));
    PolicyInput odd = session(Mode::TargetSelect, false);         // VATS mode set but another menu on top
    CHECK(menuModeForWorld(odd, true));
    CHECK(!menuModeForWorld(session(), false));                   // the engine's own "not paused" is never turned into "paused"
}

TEST(vats_policy, the_interface_update_moves_to_the_main_thread_only_while_a_menu_is_open_in_a_realtime_session) {
    // regression (FalloutNV.exe+400F66, 2026-10-27): the worker thread cleared the VATS highlight target while the main thread drew it
    for (Mode m : {Mode::TargetSelect, Mode::LimbSelect, Mode::Mode3, Mode::Playback})
        CHECK_MSG(interfaceUpdateOnMain(session(m, true), true), toString(m));   // the world runs under the open VATS menu: the main thread updates
    CHECK(!interfaceUpdateOnMain(session(Mode::None, false), true));             // another menu (opened mid-frame): as the game does it
    CHECK(!interfaceUpdateOnMain(session(Mode::None, true), true));              // the VATS menu is closing: the world is paused anyway
    CHECK(!interfaceUpdateOnMain(session(), false));                              // no menu: the worker keeps its stage 0, as in the game
    PolicyInput solo = session();
    solo.sessionActive = false;
    CHECK(!interfaceUpdateOnMain(solo, true));                                    // solo: vanilla never kicks the workers with a menu open anyway
    PolicyInput off = session();
    off.settings.realtime = false;
    CHECK(!interfaceUpdateOnMain(off, true));
}

TEST(vats_policy, the_interface_update_runs_exactly_once_per_kicked_frame) {
    InterfaceHandoff h;
    CHECK(h.workerRuns());                          // before any kick: the game's own thread
    CHECK(h.atKick(true, true));                    // a menu is open: the main thread runs it ...
    CHECK(!h.workerRuns());                         // ... and the worker skips it
    CHECK(!h.atKick(false, true));                  // the next frame without a menu: the worker again
    CHECK(h.workerRuns());
    CHECK(!h.atKick(true, false));                  // a worker site is not ours (another mod, a failed patch): never twice, the worker keeps it
    CHECK(h.workerRuns());
    CHECK(h.atKick(true, true));
    h.reset();                                      // hooks removed (hot reload): the worker runs it
    CHECK(h.workerRuns());
}

TEST(vats_policy, the_world_scale_request_becomes_one_or_the_party_scale) {
    PolicyInput in = session();
    CHECK(scaleForVatsRequest(in, 0.0001f) == 1.0f);   // the freeze on entry
    CHECK(scaleForVatsRequest(in, 0.3f) == 1.0f);      // the kill-shot slow motion
    CHECK(scaleForVatsRequest(in, 1.0f) == 1.0f);      // the reset on exit
    in.settings.partySlowMo = true;
    in.partySlowActive = true;
    in.partyScale = 0.35f;
    CHECK(scaleForVatsRequest(in, 0.0001f) == 0.35f);  // inside a host-granted window everybody is slowed together
    CHECK(desiredWorldScale(in) == 0.35f);
    in.partySlowActive = false;
    CHECK(desiredWorldScale(in) == 1.0f);
}

TEST(vats_policy, a_window_of_a_host_with_slow_mo_off_does_not_slow_anybody) {
    PolicyInput in = session();
    in.settings.partySlowMo = false;
    in.partySlowActive = true;
    in.partyScale = 0.2f;
    CHECK(desiredWorldScale(in) == 1.0f);
}

TEST(vats_policy, nobody_is_sped_up_to_compensate_a_slow_down_that_does_not_happen) {
    PolicyInput in = session(Mode::Playback);
    CHECK(timeMultFor(in, 3.0) == 1.0);
    CHECK(timeMultFor(in, 0.5) == 1.0);
}

TEST(vats_types, limb_names_codes_and_the_combat_mapping) {
    CHECK(limbFromName("Head") == LimbId::Head);
    CHECK(limbFromName("left arm") == LimbId::LeftArm);
    CHECK(limbFromName("right_leg") == LimbId::RightLeg);
    CHECK(limbFromName("RightArm") == LimbId::RightArm);
    CHECK(limbFromName("torso") == LimbId::Torso);
    CHECK(limbFromName("26") == LimbId::Torso);
    CHECK(limbFromName("99") == LimbId::None);
    CHECK(limbFromName("") == LimbId::None);
    CHECK(limbFromName("elbow") == LimbId::None);
    CHECK(limbFromAv(24) == LimbId::None);
    CHECK(limbFromAv(31) == LimbId::Brain);
    CHECK(combatLimbOf(LimbId::Torso) == 0);
    CHECK(combatLimbOf(LimbId::Head) == 1);
    CHECK(combatLimbOf(LimbId::LeftArm) == 2);
    CHECK(combatLimbOf(LimbId::RightLeg) == 5);
    CHECK(combatLimbOf(LimbId::Brain) == 255);
    for (int v = 25; v <= 31; ++v) CHECK(limbFromName(toString(limbFromAv(v))) == limbFromAv(v));
    CHECK(modeFromEngine(4) == Mode::Playback);
    CHECK(modeFromEngine(9) == Mode::None);
}

TEST(vats_types, weapon_classes) {
    bool ok = false;
    CHECK(weaponClassFromName("Gun", &ok) == WeaponClass::Gun && ok);
    CHECK(weaponClassFromName("laser", &ok) == WeaponClass::Energy && ok);
    CHECK(weaponClassFromName("grenade", &ok) == WeaponClass::Explosive && ok);
    weaponClassFromName("bow", &ok);
    CHECK(!ok);
}

TEST(vats_types, settings_are_clamped_wherever_they_come_from) {
    Settings s;
    s.slowScale = 0.0f;           // would freeze the world
    s.slowMaxMs = 3600000;        // an hour
    s.slowCooldownMs = 4000000000u;
    s.autoFireDelayMs = 99999;
    Settings c = sanitized(s);
    CHECK(c.slowScale >= 0.10f);
    CHECK(c.slowMaxMs <= 10000);
    CHECK(c.slowCooldownMs <= 120000);
    CHECK(c.autoFireDelayMs <= 3000);
    s.slowScale = 5.0f;
    CHECK(sanitized(s).slowScale <= 0.90f);
    s.slowScale = std::numeric_limits<float>::quiet_NaN();
    const float f = sanitized(s).slowScale;
    CHECK(f >= 0.10f && f <= 0.90f);
    s.slowMaxMs = 1;
    CHECK(sanitized(s).slowMaxMs >= 500);
    CHECK(sanitized(Settings{}) == Settings{});   // the defaults are inside their own ranges
}

}  // namespace
