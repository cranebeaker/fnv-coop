// Round 6 scenarios (campaign fix wave 1b, gaps G08 / G27): the votes the RULES ENGINE owns (ReadyCheck INFOs, ChoiceConsent INFOs and message boxes) are carried by the
// dialogue host. An INFO's END script waits at its line until the engine's verdict; a confirmed box waits the same way; the other players are asked with a two-button
// box and their answers go to the vote sink (rules_link.h turns them into ScenePass / choiceDeclineId).

struct RulesWorld {
    World w{3};
    FormRef npc = F(0x2BBB), topic = F(0x1000), ready = F(0x15EB69), plain = F(0x2001), box = F(0x4D0001);
    std::vector<std::pair<FormRef, PlayerId>> asked;                              // gate calls (ref, origin)
    std::vector<std::tuple<PlayerId, FormRef, bool>> votes;                        // vote sink calls
    std::vector<std::tuple<uint32_t, int32_t, bool>> resumed;                      // menu resumes (script menus)
    HostDialogue::Gate verdict = HostDialogue::Gate::Hold;
    RulesWorld() {
        w.host.setMenuResume([this](uint32_t m, int32_t c, bool t) { resumed.emplace_back(m, c, t); });
        w.host.setRulesGates(
            [this](const FormRef& r, PlayerId o) { asked.emplace_back(r, o); return r == ready ? verdict : HostDialogue::Gate::Run; },
            [this](const FormRef& r, PlayerId o) { asked.emplace_back(r, o); return verdict; });
        w.host.setVoteSink([this](PlayerId s, const FormRef& r, bool y) { votes.emplace_back(s, r, y); });
        w.emit(2, w.cl[2].requestStart(npc));
        w.pump();
    }
    void say(const FormRef& info) {   // pick + line end of the initiator (2)
        w.emit(2, w.cl[2].pick(topic, info, 0, false, {L("Yes, it is time.")}, someFacts()));
        w.pump();
        w.emit(2, w.cl[2].lineDone(w.net.now));
        w.pump();
    }
    const MenuRequest* askOf(PlayerId p) const {
        for (size_t i = w.log[p].size(); i-- > 0;)
            if (const auto* rq = std::get_if<MenuRequest>(&w.log[p][i].msg)) return rq;
        return nullptr;
    }
    bool closedWith(PlayerId p, CloseReason r) const {
        for (auto& e : w.log[p]) if (e.kind == Event::CloseMenu && e.reason == r) return true;
        return false;
    }
};

static bool scn_rules_info_end_waits_for_the_verdict() {
    RulesWorld r; World& w = r.w;
    r.say(r.ready);
    CHECK(r.asked.size() == 1 && r.asked[0].first == r.ready && r.asked[0].second == 2);   // the engine is asked once, for the initiator's INFO
    CHECK(w.runsOf(Phase::Begin) == 1 && w.runsOf(Phase::End) == 0);                      // the begin script ran, the irreversible END script waits
    CHECK(w.count(0, Event::MenuShow) == 1 && w.count(1, Event::MenuShow) == 1 && w.count(2, Event::MenuShow) == 0);   // the others are asked, the initiator is not
    const MenuRequest* rq = r.askOf(0);
    CHECK(rq && rq->consent && rq->buttons.size() == 2 && w.host.heldByRules() == 1 && !w.cl[2].topicsReady());
    w.fan(w.host.rulesResolved(r.ready, true, w.net.now));
    w.pump();
    CHECK(w.runsOf(Phase::End) == 1 && w.cl[2].topicsReady() && w.host.heldByRules() == 0);   // released: the END script ran once and the conversation goes on
    CHECK(w.count(0, Event::MenuEnd) == 1 && w.count(1, Event::MenuEnd) == 1);              // the others' boxes close
    w.fan(w.host.rulesResolved(r.ready, true, w.net.now));                                   // a duplicate verdict changes nothing
    CHECK(w.runsOf(Phase::End) == 1);
    return true;
}
static bool scn_rules_info_declined_drops_the_end_script_and_tells_the_origin() {
    RulesWorld r; World& w = r.w;
    r.say(r.ready);
    w.fan(w.host.rulesResolved(r.ready, false, w.net.now));
    w.pump();
    CHECK(w.runsOf(Phase::End) == 0);                                                      // the irreversible script never ran
    CHECK(r.closedWith(2, CloseReason::ConsentDenied) && w.cl[2].role() == ClientDialogue::Role::Idle && !w.host.npcBusy(r.npc));   // the origin is told
    CHECK(w.host.heldByRules() == 0 && w.host.open() == 0);
    return true;
}
static bool scn_rules_info_accepted_release_is_not_lost() {
    RulesWorld r; World& w = r.w;
    r.say(r.ready);
    w.fan(w.host.rulesResolved(r.ready, true, w.net.now));
    w.pump();
    CHECK(w.runsOf(Phase::End) == 1 && w.cl[2].topicsReady());
    // a goodbye line closes the conversation after the released script
    RulesWorld g; World& wg = g.w;
    wg.emit(2, wg.cl[2].pick(g.topic, g.ready, 0, true, {L("Farewell.")}, someFacts()));
    wg.pump();
    wg.emit(2, wg.cl[2].lineDone(wg.net.now));
    wg.pump();
    CHECK(wg.host.open() == 1 && wg.runsOf(Phase::End) == 0);
    wg.fan(wg.host.rulesResolved(g.ready, true, wg.net.now));
    wg.pump();
    CHECK(wg.runsOf(Phase::End) == 1 && wg.host.open() == 0 && g.closedWith(2, CloseReason::Goodbye));
    return true;
}
static bool scn_rules_answers_reach_the_vote_sink() {
    RulesWorld r; World& w = r.w;
    r.say(r.ready);
    const MenuRequest* rq0 = r.askOf(0);
    const MenuRequest* rq1 = r.askOf(1);
    CHECK(rq0 && rq1);
    w.emit(0, w.cl[0].menuReply(rq0->menu, 0));                                            // Agree
    w.emit(1, w.cl[1].menuReply(rq1->menu, 1));                                            // Not yet
    w.pump();
    CHECK(r.votes.size() == 2);
    CHECK(std::get<0>(r.votes[0]) == 0 && std::get<1>(r.votes[0]) == r.ready && std::get<2>(r.votes[0]));
    CHECK(std::get<0>(r.votes[1]) == 1 && std::get<1>(r.votes[1]) == r.ready && !std::get<2>(r.votes[1]));
    CHECK(w.runsOf(Phase::End) == 0 && w.host.heldByRules() == 1);                         // the host does not count: only the engine's verdict releases
    w.fan(w.host.onMessage(2, MenuReply{rq0->menu, 0}, 0));                                // the initiator is not asked and cannot vote
    w.fan(w.host.onMessage(0, MenuReply{rq0->menu, 7}, 0));                                // nor can a bogus button
    CHECK(r.votes.size() == 2);
    return true;
}
static bool scn_rules_gate_run_does_not_hold() {
    RulesWorld r; World& w = r.w;
    r.verdict = HostDialogue::Gate::Run;                                                   // alone / mode initiator: the engine says go
    r.say(r.ready);
    CHECK(w.runsOf(Phase::End) == 1 && w.count(0, Event::MenuShow) == 0 && w.host.heldByRules() == 0);
    // any other INFO is run without a question to anybody
    RulesWorld o; World& wo = o.w;
    o.say(o.plain);
    CHECK(wo.runsOf(Phase::End) == 1 && wo.count(0, Event::MenuShow) == 0 && o.asked.size() == 1 && o.asked[0].first == o.plain);
    return true;
}
static bool scn_rules_box_waits_for_the_verdict() {
    RulesWorld r; World& w = r.w;
    HostDialogue::ModalOptions o;
    o.partyConsent = true; o.confirmButton = 0; o.declineChoice = 1; o.mode = HostDialogue::ConsentMode::Rules; o.rulesChoice = r.box;
    uint32_t id = 0;
    w.fan(w.host.raiseMenu(2, MenuKind::MessageBox, "Launch the missiles on the Legion?", {"Yes", "No"}, 1, FormRef(), w.net.now, &id, o));
    w.pump();
    w.emit(2, w.cl[2].menuReply(id, 0));                                                   // the origin confirms
    w.pump();
    CHECK(r.asked.size() == 1 && r.asked[0].first == r.box && r.asked[0].second == 2);
    CHECK(r.resumed.empty() && w.count(0, Event::MenuShow) == 1 && w.count(1, Event::MenuShow) == 1);   // the other players are asked, the script waits
    w.fan(w.host.rulesResolved(r.box, true, w.net.now));
    w.pump();
    CHECK(r.resumed.size() == 1 && std::get<1>(r.resumed[0]) == 0 && !std::get<2>(r.resumed[0]));        // accepted: the script gets the origin's button
    // declined: the script gets the decline choice
    RulesWorld d; World& wd = d.w;
    uint32_t id2 = 0;
    wd.fan(wd.host.raiseMenu(2, MenuKind::MessageBox, "Launch the missiles on the NCR?", {"Yes", "No"}, 1, FormRef(), wd.net.now, &id2, o));
    wd.pump();
    wd.emit(2, wd.cl[2].menuReply(id2, 0));
    wd.pump();
    wd.fan(wd.host.rulesResolved(d.box, false, wd.net.now));
    wd.pump();
    CHECK(d.resumed.size() == 1 && std::get<1>(d.resumed[0]) == 1);
    // the other button of the box is no point of no return: nobody is asked
    RulesWorld n; World& wn = n.w;
    uint32_t id3 = 0;
    wn.fan(wn.host.raiseMenu(2, MenuKind::MessageBox, "Launch?", {"Yes", "No"}, 1, FormRef(), wn.net.now, &id3, o));
    wn.pump();
    wn.emit(2, wn.cl[2].menuReply(id3, 1));
    wn.pump();
    CHECK(n.asked.empty() && n.resumed.size() == 1 && std::get<1>(n.resumed[0]) == 1);
    return true;
}
static bool scn_rules_box_gate_run_resumes_at_once() {
    RulesWorld r; World& w = r.w;
    r.verdict = HostDialogue::Gate::Run;
    HostDialogue::ModalOptions o;
    o.partyConsent = true; o.confirmButton = 0; o.declineChoice = 1; o.mode = HostDialogue::ConsentMode::Rules; o.rulesChoice = r.box;
    uint32_t id = 0;
    w.fan(w.host.raiseMenu(2, MenuKind::MessageBox, "Launch?", {"Yes", "No"}, 1, FormRef(), w.net.now, &id, o));
    w.pump();
    w.emit(2, w.cl[2].menuReply(id, 0));
    w.pump();
    CHECK(r.resumed.size() == 1 && std::get<1>(r.resumed[0]) == 0 && w.host.heldByRules() == 0);
    return true;
}
static bool scn_rules_owned_info_is_not_held_twice() {
    RulesWorld r; World& w = r.w;
    HostDialogue::ChoiceRule cr; cr.mode = HostDialogue::ConsentMode::Vote;
    std::map<FormRef, HostDialogue::ChoiceRule> table;
    table[r.ready] = cr;
    table[F(0x3333)] = cr;
    w.host.setChoiceRules(table);
    for (const FormRef& i : w.host.choiceRuleInfos())
        if (i == r.ready) w.host.dropChoiceRule(i);                                        // what linkRules does for an INFO the engine owns
    CHECK(w.host.choiceRuleCount() == 1);
    w.emit(2, w.cl[2].pick(r.topic, r.ready, 0, false, {L("Go.")}, someFacts()));
    w.pump();
    CHECK(w.runsOf(Phase::Begin) == 1 && w.count(0, Event::MenuShow) == 0);                // not held at the pick by the old table: the party is asked once, at the END script
    return true;
}
static bool scn_rules_origin_leaves_while_held() {
    RulesWorld r; World& w = r.w;
    r.say(r.ready);
    w.fan(w.host.playerLeft(2, w.net.now));
    w.pump();
    CHECK(w.host.heldByRules() == 0 && w.host.open() == 0);
    w.fan(w.host.rulesResolved(r.ready, true, w.net.now));                                 // the engine's verdict comes late
    CHECK(w.runsOf(Phase::End) == 0);                                                      // nobody to run it for
    return true;
}
static bool scn_rules_failsafe_declines_when_the_engine_never_answers() {
    HostDialogue::Config c; c.messageMenuMs = 1000;
    World w(3, c);
    w.host.setRulesGates([](const FormRef&, PlayerId) { return HostDialogue::Gate::Hold; }, nullptr);
    w.emit(2, w.cl[2].requestStart(F(0x2BBB)));
    w.pump();
    w.emit(2, w.cl[2].pick(F(0x1000), F(0x15EB69), 0, false, {L("Go.")}, someFacts()));
    w.pump();
    w.emit(2, w.cl[2].lineDone(w.net.now));
    w.pump();
    CHECK(w.host.heldByRules() == 1);
    w.fan(w.host.tick(1500));
    CHECK(w.host.heldByRules() == 1 && w.runsOf(Phase::End) == 0);                         // inside the grace window the engine still decides
    w.fan(w.host.tick(1000 + 30000 + 600));
    w.pump();
    CHECK(w.host.heldByRules() == 0 && w.runsOf(Phase::End) == 0 && w.host.open() == 0);  // the failsafe: declined, never run
    return true;
}

// ---------- wave 1b (P7 request): a party companion (ED-E, owner=party) uses nobody's slot and is "hired" for everybody
static bool scn_comp_party_owned() {
    CompWorld c;
    auto& r = c.reg;
    const FormRef ede = npcOf("EDE"), rex = npcOf("Rex"), boone = npcOf("Boone");
    CHECK(r.recruit(ede, 1) == RecruitResult::Ok);
    r.setPartyOwned(ede, true);
    CHECK(r.isPartyOwned(ede) && !r.isPartyOwned(rex));
    CHECK(r.count(1, 1) == 0 && r.count(1) == 0 && r.partyCount() == 0);          // ED-E uses nobody's slot ...
    CHECK(r.recruit(rex, 1) == RecruitResult::Ok);                                // ... so the recruiter can still take Rex next to him (the critter slot is free)
    CHECK(r.recruit(boone, 1) == RecruitResult::Ok && r.partyCount() == 2);
    const uint32_t edeHired = 30, edeAvail = 32;
    CHECK(r.viewVar(kQuest, edeHired, 1, 0) == 1 && r.viewVar(kQuest, edeHired, 2, 0) == 1 && r.viewVar(kQuest, edeHired, 0, 0) == 1);   // everybody sees ED-E in the party
    CHECK(r.viewVar(kQuest, edeAvail, 2, 1) == 1);                                 // and the recruiter's ownership does not hide him from the others' scripts
    CHECK(r.viewVar(kQuest, 6, 2, 1) == 0 && r.viewVar(kQuest, 6, 1, 1) == 1);     // an ordinary companion stays per owner
    CHECK(r.viewVar(kQuest, vnpc::kCritterInParty, 1, 0) == 1 && r.viewVar(kQuest, vnpc::kCritterInParty, 2, 1) == 0);   // player 1 has Rex; player 2's critter slot is free
    // an unowned party companion is not hired for anybody
    r.setPartyOwned(boone, false);
    r.dismiss(ede, 1);
    CHECK(r.viewVar(kQuest, edeHired, 2, 0) == 0 && r.isPartyOwned(ede));          // the flag is a property of the NPC, it survives the dismissal
    r.setPartyOwned(ede, false);
    CHECK(!r.isPartyOwned(ede));
    return true;
}
