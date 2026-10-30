// The order of a guest's character creation (coop_module.cpp GuestChargenFrame): name, face, SPECIAL, tag skills, traits, one menu after the other. Game-free:
// the caller says which menus are open and runs the lines this asks for; nothing here touches the game.
//
// What it decides, frame by frame:
//   * a step is issued when no menu of another kind is open (a message box, Doc's talk); the pause menu does not count for most steps;
//   * the step's own menu is the one that appears after the line ran; the step is over when that menu has been closed for a moment;
//   * a line whose menu never appears is tried again (up to maxAttempts) before the step is skipped. The first version skipped it after one attempt: in the owner's
//     two-copy run the SPECIAL and tag skill screens "never opened, skipped" and the guest went on without them;
//   * the steps that open a screen from a script command (SPECIAL, tag skills: the game runs them from a script's GameMode block, which does not run while a menu is
//     open) wait for the pause menu to close too (the engine opens it when the game window loses focus, which two copies on one PC do all the time), for at most
//     escapeWaitMs.
#pragma once
#include <cstdint>
#include <set>
#include <string>
#include <vector>

namespace nvcoop::coop {

struct ChargenFlowConfig {
    uint64_t openWaitMs = 3000;     // a menu that has not appeared this long after its line ran: the line did nothing
    int maxAttempts = 3;            // lines per step before the step is skipped
    uint64_t escapeWaitMs = 20000;  // how long a step that needs a quiet game waits for the pause menu to go away
    int closedFramesNeeded = 20;    // a menu is closed for good after this many frames without it (not between two of its own screens)
    int escapeMenu = 1013;
};

struct ChargenStepRule {
    const char* what = "";
    bool needsNoPauseMenu = false;
};

class ChargenFlow {
public:
    enum class Do { Nothing, Issue, Finish };
    struct Out {
        Do what = Do::Nothing;
        int step = 0;
        int attempt = 1;
    };

    ChargenFlow() = default;
    ChargenFlow(std::vector<ChargenStepRule> steps, ChargenFlowConfig cfg = {}) : steps_(std::move(steps)), cfg_(cfg) {}

    void start(uint64_t startAt) {
        running_ = true;
        done_ = false;
        step_ = 0;
        attempt_ = 1;
        nextAt_ = startAt;
        issuedAt_ = 0;
        menu_ = 0;
        closedFrames_ = 0;
        blockedBy_ = 0;
        escapeSince_ = 0;
        escapeGaveUp_ = false;
        log_.clear();
    }
    bool running() const { return running_; }
    bool done() const { return done_; }
    bool pending() const { return running_ && !done_; }
    int step() const { return step_; }
    // Lines the caller writes to the log, in order (taken once).
    std::vector<std::string> takeLog() {
        std::vector<std::string> v;
        v.swap(log_);
        return v;
    }
    // What the caller reports back after it ran the step's line(s) for an Issue.
    void issued(uint64_t now, const std::set<int>& openBefore) {
        issuedAt_ = now;
        before_ = openBefore;
        menu_ = 0;
        closedFrames_ = 0;
    }

    Out frame(uint64_t now, const std::set<int>& open) {
        Out o;
        if (!running_ || done_ || now < nextAt_) return o;
        const int nSteps = static_cast<int>(steps_.size());
        if (issuedAt_ && step_ < nSteps) {   // the step's line ran: wait for its menu to appear, then to close
            if (!menu_) {
                for (int id : open)
                    if (id != cfg_.escapeMenu && !before_.count(id)) {
                        menu_ = id;
                        break;
                    }
                if (menu_) log_.push_back(std::string("chargen ") + steps_[step_].what + ": menu " + std::to_string(menu_) + " is open" + (attempt_ > 1 ? " (attempt " + std::to_string(attempt_) + ")" : ""));
                else if (now - issuedAt_ < cfg_.openWaitMs) return o;
            }
            if (menu_ && open.count(menu_)) {
                closedFrames_ = 0;
                return o;
            }
            if (menu_ && ++closedFrames_ < cfg_.closedFramesNeeded) return o;
            issuedAt_ = 0;
            if (menu_) {
                log_.push_back(std::string("chargen ") + steps_[step_].what + " done");
                ++step_;
                attempt_ = 1;
            } else if (attempt_ < cfg_.maxAttempts) {
                log_.push_back(std::string("chargen ") + steps_[step_].what + ": no menu after attempt " + std::to_string(attempt_) + " (open menus:" + list(open) + "), trying again");
                ++attempt_;
            } else {
                log_.push_back(std::string("chargen ") + steps_[step_].what + " never opened after " + std::to_string(attempt_) + " attempts (open menus:" + list(open) + "), skipped");
                ++step_;
                attempt_ = 1;
            }
            escapeSince_ = 0;
            escapeGaveUp_ = false;
        }
        for (int id : open)
            if (id != cfg_.escapeMenu) {   // something else is open (a message box, Doc's talk): the next step waits
                if (blockedBy_ != id) {
                    blockedBy_ = id;
                    log_.push_back("chargen waits for menu " + std::to_string(id) + " to close");
                }
                return o;
            }
        blockedBy_ = 0;
        if (step_ >= nSteps) {
            done_ = true;
            o.what = Do::Finish;
            return o;
        }
        if (steps_[step_].needsNoPauseMenu && open.count(cfg_.escapeMenu) && !escapeGaveUp_) {
            if (!escapeSince_) {
                escapeSince_ = now;
                log_.push_back(std::string("chargen ") + steps_[step_].what + " waits for the pause menu to close (the game window has no focus?)");
            }
            if (now - escapeSince_ < cfg_.escapeWaitMs) return o;
            escapeGaveUp_ = true;
            log_.push_back(std::string("chargen ") + steps_[step_].what + ": the pause menu is still open, going on anyway");
        }
        o.what = Do::Issue;
        o.step = step_;
        o.attempt = attempt_;
        return o;
    }

private:
    static std::string list(const std::set<int>& s) {
        std::string r;
        for (int id : s) r += " " + std::to_string(id);
        return r.empty() ? " none" : r;
    }
    std::vector<ChargenStepRule> steps_;
    ChargenFlowConfig cfg_;
    bool running_ = false, done_ = false;
    int step_ = 0, attempt_ = 1;
    uint64_t nextAt_ = 0, issuedAt_ = 0;
    std::set<int> before_;
    int menu_ = 0, closedFrames_ = 0, blockedBy_ = 0;
    uint64_t escapeSince_ = 0;
    bool escapeGaveUp_ = false;
    std::vector<std::string> log_;
};

}  // namespace nvcoop::coop
