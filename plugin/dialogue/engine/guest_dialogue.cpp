#include "guest_dialogue.h"

#include <windows.h>

#include <cstdio>
#include <fstream>
#include <sstream>

#include "dialogue_embedded.h"
#include "engine_xnvse.h"
#include "fnvcoop_dlg/dialogue_data.h"
#include "glue.h"
#include "ui_hooks.h"

namespace fndlg {

struct GuestDialogue::Impl {
    GuestDialogueConfig cfg;
    bool ready = false;
    std::unique_ptr<IEngineApi> engine;
    std::unique_ptr<EngineGlue> glue;
    fnvcoop_dlg::DialogueData data;
    ResponseLayout layout;
    fnvcoop_dlg::DialogueNode* node = nullptr;
    fnvcoop_dlg::FollowerRulesStats followers;
    uint64_t sinkDropped = 0, nodesConfigured = 0;
    bool probed = false;
    std::string loadNote;

    void note(const std::string& s) {
        if (cfg.report) cfg.report(s.c_str());
    }
};

namespace {
EngineGlue* g_glue = nullptr;   // the answer sink of the origin-side menus (nvui::ops) reaches the glue through this
}

GuestDialogue::GuestDialogue() : p_(new Impl) {}
GuestDialogue::~GuestDialogue() = default;
GuestDialogue& GuestDialogue::instance() {
    static GuestDialogue g;
    return g;
}

bool GuestDialogue::init(const GuestDialogueConfig& c) {
    Impl& s = *p_;
    if (s.ready) return true;
    s.cfg = c;
    // data tables: a file next to the DLL wins, else the copy compiled in
    auto provider = [&](const std::string& name, std::string& text) {
        if (!s.cfg.dataDir.empty()) {
            std::ifstream f(s.cfg.dataDir + "nvcoop_" + name, std::ios::binary);
            if (f) {
                std::ostringstream o;
                o << f.rdbuf();
                text = o.str();
                return true;
            }
        }
        for (size_t i = 0; i < kEmbeddedDialogueDataCount; ++i)
            if (name == kEmbeddedDialogueData[i].name) {
                text.assign(reinterpret_cast<const char*>(kEmbeddedDialogueData[i].data), kEmbeddedDialogueData[i].size);
                return true;
            }
        return false;
    };
    fnvcoop_dlg::loadDialogueData(provider, s.data);
    SetFactTable(&s.data.facts);
    char buf[256];
    std::snprintf(buf, sizeof buf, "tables: %u owner-only topics, %u per-player say-once lines, facts for %u INFOs, follower rules %u bytes%s%s",
                  unsigned(s.data.report.ownerOnlyTopics), unsigned(s.data.report.sayOncePerPlayer), unsigned(s.data.report.facts),
                  unsigned(s.data.report.followerBytes), s.data.report.missing.empty() ? "" : ", MISSING ", s.data.report.missing.c_str());
    s.loadNote = buf;
    s.note(s.loadNote);
    // the report callback is a plain function pointer; unsupported() calls it through the engine object
    static void (*g_report)(const char*) = nullptr;
    g_report = s.cfg.report;
    s.engine.reset(CreateXnvseEngine([](const char* what) {
        if (g_report) {
            char b[300];
            std::snprintf(b, sizeof b, "engine call not supported: %s", what);
            g_report(b);
        }
    }));
    s.glue.reset(new EngineGlue(*s.engine, s.layout));
    GlueConfig gc;
    gc.broadcastHostTalk = s.cfg.broadcastHostTalk;
    s.glue->setConfig(gc);
    s.glue->setSayOnce(&s.data.sayOnce);
    g_glue = s.glue.get();
    // the engine's own say-once marks (TESTopicInfo::MarkSaid detour in hooks_ui): a guest reports them to the host, the host commits them (D11)
    nvui::setSaidSink([](uint32_t info) {
        if (g_glue) g_glue->onEngineMark(Addr32(info));
    });
    // the player answered a box the host script raised to them (or a consent prompt): back to the dialogue host. Nothing else wires this.
    nvui::ops::setAnswerSink([](uint32_t menu, int32_t choice) {
        if (g_glue) g_glue->onMenuAnswered(menu, choice, uint32_t(GetTickCount64()));
    });
    s.ready = true;
    return true;
}

// Once per process, the first time a line is being spoken: the live proof of the response layout. The INFO is expected to have NO head
// field (statically verified: the chain is lazy); the structural probe confirms that against live memory and, if it does find one, keeps the
// offset as the reader's last-resort fallback. Logged either way.
void GuestDialogue::probeResponseLayout(Impl& s) {
    struct ProcMem : IMemory {
        bool read(Addr32 a, void* out, size_t n) const override {
            SIZE_T got = 0;
            return a >= 0x10000 && ReadProcessMemory(GetCurrentProcess(), reinterpret_cast<LPCVOID>(uintptr_t(a)), out, n, &got) && got == n;
        }
    } mem;
    Addr32 menu = 0, spoken = 0;
    if (!readU32(mem, dialog_menu::kSingletonPtr, menu) || !menu || !readU32(mem, dialog_menu::kSpokenInfoGlobal, spoken) || !spoken) return;
    std::vector<fnvcoop_dlg::Line> lines;
    if (!readCachedLines(mem, spoken, lines) || lines.empty()) return;   // the engine has not loaded the chain yet: try again next frame
    s.probed = true;
    const int32_t off = probeResponseOffset(mem, spoken, lines[0].text.substr(0, 12));
    s.layout.headOffset = off;
    s.glue->setLayout(s.layout);
    char buf[200];
    std::snprintf(buf, sizeof buf, "response layout probe: %s; %u line(s) read from the engine cache (cache hits %llu, loader calls %llu)",
                  off < 0 ? "the INFO has no response head field, as expected (chain = engine cache + loader)" : "a head field exists at an INFO offset: kept as a fallback",
                  unsigned(lines.size()), static_cast<unsigned long long>(readerStats().cacheHits), static_cast<unsigned long long>(readerStats().loaderCalls));
    s.note(buf);
}

bool GuestDialogue::ready() const { return p_->ready; }

fnvcoop_dlg::DialogueNode::Sink GuestDialogue::sink() {
    return [this](const fnvcoop_dlg::Event& e) {
        Impl& s = *p_;
        if (s.ready && s.node) s.glue->onEvent(e);
        else {
            ++s.sinkDropped;
            s.note("dialogue event " + std::to_string(int(e.kind)) + " DROPPED: the glue is " + (s.ready ? "ready" : "not ready") + ", node " + (s.node ? "attached" : "not attached"));
        }
    };
}

std::function<std::vector<fnvcoop_dlg::Line>(uint32_t)> GuestDialogue::infoReader() {
    Impl& s = *p_;
    // the engine object may not exist yet when the wiring asks (init order): read process memory directly in that case
    struct ProcMem : IMemory {
        bool read(Addr32 a, void* out, size_t n) const override {
            SIZE_T got = 0;
            return a >= 0x10000 && ReadProcessMemory(GetCurrentProcess(), reinterpret_cast<LPCVOID>(uintptr_t(a)), out, n, &got) && got == n;
        }
    };
    static ProcMem mem;
    return makeInfoReader(mem, &s.layout);
}

void GuestDialogue::frame(fnvcoop_dlg::DialogueNode* node, uint32_t nowMs) {
    Impl& s = *p_;
    if (!s.ready) return;
    if (node != s.node) {
        s.node = node;
        s.glue->attach(node);
        s.data.sayOnce.reset();
        if (node) {
            s.followers = fnvcoop_dlg::configureNode(*node, s.data);
            ++s.nodesConfigured;
            char buf[200];
            std::snprintf(buf, sizeof buf, "node attached: %u follower quests, %u companions, %u var rules (%u bad lines)", unsigned(s.followers.quests),
                          unsigned(s.followers.companions), unsigned(s.followers.rules), unsigned(s.followers.bad));
            s.note(buf);
        }
    }
    if (node) s.glue->pollMenu(nowMs);
    if (!s.probed) probeResponseLayout(s);
}

bool GuestDialogue::onTalkRequested(const fnvcoop_dlg::FormRef& npc, uint32_t nowMs) {
    Impl& s = *p_;
    const bool r = s.ready && s.node && s.glue->onTalkRequested(npc, nowMs);
    if (!r) s.note(std::string("talk request not taken: glue ") + (s.ready ? "ready" : "NOT ready") + ", node " + (s.node ? "attached" : "NOT attached") + (s.ready && s.node ? ", a conversation role is already set" : ""));
    return r;
}

std::string GuestDialogue::statusJson() const {
    const Impl& s = *p_;
    std::ostringstream o;
    o << "{\"ready\":" << (s.ready ? "true" : "false") << ",\"attached\":" << (s.node ? "true" : "false");
    if (s.ready) {
        const auto& st = s.glue->stats();
        const auto& rs = readerStats();
        o << ",\"polls\":" << st.polls << ",\"offers\":" << st.offers << ",\"picks\":" << st.picks << ",\"pickRefused\":" << st.pickRefused << ",\"pickLatched\":" << st.pickLatched << ",\"lineDones\":" << st.lineDones
          << ",\"leaves\":" << st.leaves << ",\"hostBroadcasts\":" << st.hostBroadcasts << ",\"sayMarked\":" << st.sayMarked
          << ",\"sayCleared\":" << st.sayCleared << ",\"engineMarks\":" << st.engineMarks << ",\"sayReported\":" << st.sayReported
          << ",\"sayReports\":" << st.sayReports << ",\"sayCommitted\":" << st.sayCommitted << ",\"commandRefused\":" << st.commandRefused << ",\"unreadable\":" << st.unreadable
          << ",\"reader\":{\"cacheHits\":" << rs.cacheHits << ",\"loaderCalls\":" << rs.loaderCalls << ",\"loaderRefused\":" << rs.loaderRefused
          << ",\"failed\":" << rs.failed << "},\"nodesConfigured\":" << s.nodesConfigured << ",\"sinkDropped\":" << s.sinkDropped
          << ",\"companions\":" << s.followers.companions;
        if (s.node && s.node->host()) {   // the dialogue host's own numbers: script runs (Begin + End per pick), picks answered without a run, rejected messages
            const auto* h = s.node->host();
            o << ",\"host\":{\"scriptRuns\":" << h->scriptRuns() << ",\"repeatPicks\":" << h->repeatPicks() << ",\"rejected\":" << h->rejected() << ",\"open\":" << h->open() << "}";
        }
        // the "said once" byte of the INFOs a story run depends on, on THIS machine (story run S03: "Doc Mitchell said you could teach me" never listed)
        static const uint32_t kProbe[] = {0x10A1E4, 0x10A1E5, 0x10A1EB, 0x10B754, 0x15F531, 0x10721E, 0x10721F, 0x1057E7};
        o << ",\"saidProbe\":{";
        for (size_t i = 0; i < sizeof kProbe / sizeof kProbe[0]; ++i) {
            char k[16];
            std::snprintf(k, sizeof k, "%X", unsigned(kProbe[i]));
            o << (i ? "," : "") << "\"" << k << "\":" << s.engine->saidState(fnvcoop_dlg::FormRef("falloutnv.esm", kProbe[i]));
        }
        o << "}";
    }
    o << "}";
    return o.str();
}

}  // namespace fndlg
