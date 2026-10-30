// The opening scene's watcher (game-free): turns what the game says about Doc Mitchell's house (the two quests' stages, the open menu, whether this
// window is the one that gets the keyboard) into the log lines and the one notice a player needs when the scene waits for HIM.
//
// Why. The 2026-10-30 playtest (host and guest copies on one PC): "the host is just staring at the fan and doesn't get up". The scene is the stock one: the
// player lies in bed through Doc's first lines (VCG01 stages 3 to 8), is asked for his name (stage 10, menu 1051), picks his face in the reflectron
// (stage 36, menu 1036) and only then is stood up (stage 45 to 50); Doc's scene never goes on by itself while a menu waits. With two copies on one
// PC only the foreground window gets keyboard and mouse (DirectInput foreground level, plugin/splitscreen/focus_fake.h), so the copy the player is not
// looking at sits at the fan for good. Nothing in the old log said where it was waiting: the log had no line for the host's scene at all.
//
//   * every stage change of VCG00 / VCG01 is a log line (with the menu and the foreground state), so the next log shows exactly where a copy stood;
//   * the wake-up steps are named: the black screen ends (VCG00 100), the player is stood up (VCG01 45), the controls come back (VCG01 55);
//   * a stage that does not change for a while is a "stalled" line that says WHAT it waits for, and a menu that waits in a window that is not in
//     front puts one notice on that window's screen.
//
// Mutation twins (FNVCOOP_MUT, tests/test_intro_watch.cpp must fail for each): 1600 no line for a stage change   1601 no stall line
// 1602 the stall line does not say that a menu waits   1603 no notice for a background window   1604 the notice repeats every call
// 1605 the wake-up steps are not logged   1606 a menu opening is not logged
#pragma once
#include <cstdint>
#include <cstdio>
#include <string>
#include <vector>

#include "fnvcoop_mut.h"

namespace nvcoop::coop::introwatch {

struct Config {
    uint64_t stallMs = 20000;     // a stage unchanged this long while the scene is running is "stalled"
    uint64_t repeatMs = 60000;    // ... and said again this often
};

struct Sample {
    uint64_t nowMs = 0;
    int vcg00 = -1, vcg01 = -1;   // quest stages (-1 = not readable / quest has none yet)
    bool foreground = true;       // this copy's window is the foreground window (the only one that gets keyboard and mouse)
    int menu = 0;                 // the first open menu that is neither the HUD nor the pause menu, 0 = none
};

struct Out {
    std::vector<std::string> log;
    std::string notice;           // shown on this copy's screen (empty = nothing)
};

inline const char* menuName(int id) {
    switch (id) {
        case 1001: return "dialogue";
        case 1036: return "face (reflectron)";
        case 1048: return "SPECIAL / tag skills";
        case 1051: return "name";
        case 1084: return "traits";
        default: return "";
    }
}

class IntroWatch {
public:
    explicit IntroWatch(Config c = {}) : cfg_(c) {}

    Out step(const Sample& s) {
        Out o;
        if (!started_) {
            started_ = true;
            t0_ = s.nowMs;
            v0_ = s.vcg00;
            v1_ = s.vcg01;
            since_ = s.nowMs;
            menu_ = s.menu;
            o.log.push_back("coop: intro scene: starts watching at VCG00 stage " + std::to_string(s.vcg00) + ", VCG01 stage " + std::to_string(s.vcg01) + fg(s));
            return o;
        }
        const bool changed = s.vcg00 != v0_ || s.vcg01 != v1_;
        if (changed) {
            if (!FNV_MUT(1600))
                o.log.push_back("coop: intro scene: " + at(s) + ": VCG00 stage " + std::to_string(v0_) + " -> " + std::to_string(s.vcg00) + ", VCG01 stage " + std::to_string(v1_) + " -> " +
                                std::to_string(s.vcg01) + " (waited " + secs(s.nowMs - since_) + ")" + fg(s) + menuText(s.menu));
            if (!FNV_MUT(1605)) wake(s, o);
            v0_ = s.vcg00;
            v1_ = s.vcg01;
            since_ = s.nowMs;
            stallSaid_ = false;
            noticed_ = false;
        }
        if (s.menu != menu_) {
            if (!FNV_MUT(1606))
                o.log.push_back("coop: intro scene: " + at(s) + ": " + (s.menu ? "menu " + std::to_string(s.menu) + menuLabel(s.menu) + " opened" : "menu " + std::to_string(menu_) + menuLabel(menu_) + " closed") +
                                " at VCG01 stage " + std::to_string(s.vcg01) + fg(s));
            menu_ = s.menu;
        }
        if (running(s) && s.nowMs - since_ >= cfg_.stallMs && (!stallSaid_ || s.nowMs - lastStall_ >= cfg_.repeatMs)) {
            stallSaid_ = true;
            lastStall_ = s.nowMs;
            std::string why;
            if (s.menu && !FNV_MUT(1602)) why = "menu " + std::to_string(s.menu) + menuLabel(s.menu) + " is open and waits for the player";
            else if (s.menu) why = "a menu is open";
            else why = "no menu is open: the scene waits on its own timer or on a line Doc is still saying";
            if (!s.foreground) why += "; this window is not in front, so it gets no keyboard or mouse (click it)";
            if (!FNV_MUT(1601))
                o.log.push_back("coop: intro scene: STALLED " + at(s) + ": VCG00 stage " + std::to_string(s.vcg00) + ", VCG01 stage " + std::to_string(s.vcg01) + " unchanged for " +
                                secs(s.nowMs - since_) + ": " + why);
            if (s.menu && !s.foreground && (!noticed_ || FNV_MUT(1604)) && !FNV_MUT(1603)) {
                noticed_ = true;
                o.notice = std::string("Doc is waiting for you here (") + (menuName(s.menu)[0] ? menuName(s.menu) : "menu") + "): click this window";
            }
        }
        return o;
    }

    // The scene still has steps to go: the black screen / movie part (VCG00 90 to 99) or Doc's scene up to the player walking off (VCG01 below 55).
    static bool running(const Sample& s) { return (s.vcg00 >= 90 && s.vcg00 < 100) || (s.vcg01 >= 1 && s.vcg01 < 55); }

private:
    static std::string secs(uint64_t ms) {
        char b[32];
        std::snprintf(b, sizeof b, "%.1f s", static_cast<double>(ms) / 1000.0);
        return b;
    }
    std::string at(const Sample& s) const { return "after " + secs(s.nowMs - t0_); }
    static std::string fg(const Sample& s) { return s.foreground ? ", window in front" : ", window NOT in front"; }
    static std::string menuLabel(int id) {
        const char* n = menuName(id);
        return n[0] ? std::string(" (") + n + ")" : std::string();
    }
    static std::string menuText(int id) { return id ? ", menu " + std::to_string(id) + menuLabel(id) + " open" : std::string(); }

    // the wake-up steps, once each
    void wake(const Sample& s, Out& o) {
        if (s.vcg00 >= 100 && !w100_) {
            w100_ = true;
            o.log.push_back("coop: wake-up: " + at(s) + ": VCG00 stage 100, the black screen ends and the player wakes up at Doc's");
        }
        if (s.vcg01 >= 45 && !w45_) {
            w45_ = true;
            o.log.push_back("coop: wake-up: " + at(s) + ": VCG01 stage " + std::to_string(s.vcg01) + ", Doc's scene stands the player up out of the bed");
        }
        if (s.vcg01 >= 55 && !w55_) {
            w55_ = true;
            o.log.push_back("coop: wake-up: " + at(s) + ": VCG01 stage " + std::to_string(s.vcg01) + ", the controls come back, the player can walk");
        }
    }

    Config cfg_;
    bool started_ = false, stallSaid_ = false, noticed_ = false, w100_ = false, w45_ = false, w55_ = false;
    uint64_t t0_ = 0, since_ = 0, lastStall_ = 0;
    int v0_ = -1, v1_ = -1, menu_ = 0;
};

}  // namespace nvcoop::coop::introwatch
