//! Talking to people: look at someone within reach and press E. They say
//! the first greeting their conditions allow (`world::dialogue`), in their
//! own recorded voice with the text shown. After a line the game's rules
//! (`world::dialogue::after_line`, `00762ff0`) decide: the speaker goes
//! straight on to the line's follow-up (`TCFU`), the menu closes after a
//! Goodbye, or the line's topics come up as numbered choices, or, after a
//! line with none, the main list (`dialogue::menu_topics`: the player's
//! top-level and learned topics the speaker answers). Space skips a line.
//! Only the stand-in text panel (used when the game's menu files can't be
//! read) also ends the conversation on Tab or Esc; the game's dialogue
//! menu takes only its "A"/Enter key (`007628c0`).
//!
//! The menu zooms in as it opens and out before it goes, the player's
//! view held on the speaker's head meanwhile ([`focus_camera`],
//! `world::dialogue_view`).

use bevy::audio::AudioPlayer;
use bevy::prelude::*;
use bevy::render::mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
use bevy::render::mesh::VertexAttributeValues;
use cellview::ActorData;
use esm::FormId;
use world::dialogue::{self, Choice, GameState, Info, Speaker, Topic};
use world::dialogue_view::{self as view_rules, Focus, FocusInput, MenuZoom, ViewSettings};
use world::scripting::{Runner, ScriptCache};

use crate::scripts::{ScriptedTalk, Scripts};
use crate::walk::{game_point, Player, Prompt};
use crate::{FlyCamera, GameFiles};

/// The `GREETING` topic (a fixed form in every game).
const GREETING: FormId = FormId(0xC8);

/// People who can be talked to in what's loaded: reference, base, feet.
#[derive(Resource, Default)]
pub struct Talkers(pub Vec<Talker>);

#[derive(Debug, Clone, Copy)]
pub struct Talker {
    pub reference: FormId,
    pub base: FormId,
    pub position: [f32; 3],
}

impl Talker {
    pub fn from_actor(a: &ActorData) -> Talker {
        Talker {
            reference: FormId(a.reference),
            base: FormId(a.base),
            position: a.position,
        }
    }
}

/// What dialogue conditions ask about (quest stages, globals).
#[derive(Resource)]
pub struct DialogueState(pub GameState);

/// `--talk`: start talking to the nearest person once loaded.
#[derive(Resource, Default)]
pub struct AutoTalk(pub bool, pub std::collections::VecDeque<usize>);

/// `--say`: for testing, topics to choose when the menu offers some, in
/// order: each the first offered whose text contains it (ignoring case).
#[derive(Resource, Default)]
pub struct AutoSay(pub std::collections::VecDeque<String>);

impl AutoSay {
    /// The offered topic (by its text) the next `--say` names, which is
    /// then used up; none if it names none of them.
    pub fn pick(&mut self, offered: &[&str]) -> Option<usize> {
        let wanted = self.0.front()?.to_lowercase();
        let i = offered
            .iter()
            .position(|t| t.to_lowercase().contains(&wanted))?;
        self.0.pop_front();
        Some(i)
    }
}

/// The person looked at, and their name.
#[derive(Resource, Default)]
pub struct TalkTarget(pub Option<(Talker, String)>);

/// The conversation going on, if any, and every top-level topic in the
/// game (read the first time someone is talked to).
#[derive(Resource, Default)]
pub struct Conversation(pub Option<Talk>, Option<Vec<Topic>>);

impl Conversation {
    /// Forget playback and topic state without running the line's end script.
    pub fn discard(&mut self) {
        self.0 = None;
        self.1 = None;
    }

    #[cfg(test)]
    pub(crate) fn test_active() -> Self {
        let speaker = Speaker {
            reference: FormId(10),
            base: FormId(11),
            name: Some("Test speaker".into()),
            voice: None,
            race: None,
            female: false,
            factions: vec![],
        };
        let info = Info {
            form_id: FormId(12),
            topic: None,
            quest: None,
            previous: None,
            flags: 0,
            flags2: 0,
            responses: vec![world::dialogue::Response {
                emotion: 0,
                emotion_value: 0,
                number: 0,
                text: "pending line".into(),
                use_emotion: false,
                speaker_idle: None,
                listener_idle: None,
                sound: None,
            }],
            conditions: vec![],
            prompt: None,
            check: None,
            choices: vec![],
            add_topics: vec![],
            follow_ups: vec![],
            begin_script: Some("BeginScript".into()),
            end_script: Some("EndScript".into()),
        };
        Self(
            Some(Talk {
                speaker,
                name: "Test speaker".into(),
                info,
                response: 0,
                since: 0.0,
                voice: None,
                skipped_at: None,
                choices: None,
                line_only: true,
                say_to: None,
                shown_line: None,
                shown_topics: false,
                zoom: MenuZoom::opening(),
                package_zoom: None,
            }),
            Some(vec![]),
        )
    }
}

pub struct Talk {
    speaker: Speaker,
    name: String,
    info: Info,
    /// The response being said, and since when (seconds).
    response: usize,
    since: f32,
    voice: Option<Entity>,
    /// When the player clicked the line away, while its voice plays.
    skipped_at: Option<f32>,
    /// Once the line is said: what the player can answer.
    choices: Option<Vec<Choice>>,
    /// Only a line said (`SayTo`): no dialogue menu, no choices after it,
    /// and the player and the game carry on meanwhile.
    line_only: bool,
    /// Said with `SayTo`, of this topic: the speaker's `SayToDone` blocks
    /// run once it's said.
    say_to: Option<FormId>,
    /// What the game's dialogue menu shows now (`game_menus::dialog`): the
    /// line and response, or the topics.
    shown_line: Option<(FormId, usize)>,
    shown_topics: bool,
    /// The menu's zoom in, and out once it's to close
    /// (`world::dialogue_view::MenuZoom`).
    zoom: MenuZoom,
    /// The speaker's dialogue package's zoom, when a dialogue package of
    /// theirs is running as the menu opens (`00761a20`).
    package_zoom: Option<f32>,
}

impl Talk {
    pub fn is_line_only(&self) -> bool {
        self.line_only
    }

    /// Who's talking (their reference).
    pub fn speaker(&self) -> FormId {
        self.speaker.reference
    }

    /// The response being said now, as ((line, response number, when it
    /// began), response), while it's said (not once the topics are up):
    /// what the speaker's say (`008a20d0`) hands the animation
    /// (`sitting::dialogue_frame`).
    pub fn said(&self) -> Option<((FormId, usize, f32), &world::dialogue::Response)> {
        if self.choices.is_some() {
            return None;
        }
        let r = self.info.responses.get(self.response)?;
        Some(((self.info.form_id, self.response, self.since), r))
    }
}

/// The text panel at the bottom of the screen.
#[derive(Component)]
pub struct DialogueText;

pub fn setup_dialogue_text(mut commands: Commands) {
    commands.spawn((
        Text::new(String::new()),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.85, 0.55)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(40.0),
            left: Val::Percent(15.0),
            width: Val::Percent(70.0),
            padding: UiRect::all(Val::Px(12.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        Visibility::Hidden,
        DialogueText,
    ));
}

/// Plays a response's voice, if its file is found.
fn play_voice(
    commands: &mut Commands,
    audio: &mut Assets<crate::sounds::PcmSound>,
    game: &cellview::Game,
    talk: &Talk,
) -> Option<Entity> {
    let response = talk.info.responses.get(talk.response)?;
    let voice = talk.speaker.voice?;
    let path = dialogue::voice_path(&game.order, &talk.info, response, voice)?;
    let Some(bytes) = game.assets.read(&path).ok().flatten() else {
        println!("  no voice file {path}");
        return None;
    };

    let source = crate::sounds::voice_handle(&path, &bytes, audio)?;
    // With a lip sync file beside it, the voice waits out its lead-in and
    // the speaker's face says it (`faces`).
    let (settings, voice) = crate::faces::voice_playback(game, &path, talk.speaker.reference);
    let entity = commands.spawn((AudioPlayer(source), settings, voice)).id();
    println!("  voice started: {path} ({entity})");
    Some(entity)
}

/// How long a response stays up without a voice file:
/// `fDialogSpeechDelaySeconds` (exe default 2, setting `011d32c4`; the
/// speaker's say function `008a20d0` sets the dialogue menu's line timer
/// to it when the voice file isn't found, and the menu waits it out in
/// state 3, `00762950`). Whether the speaker is marked done sooner is not
/// traced.
fn silent_line_seconds(order: &esm::LoadOrder) -> f32 {
    world::scripting::game_setting(order, "fDialogSpeechDelaySeconds").unwrap_or(2.0)
}

/// After the player clicks a line away the voice goes on this long before
/// it's stopped (`00762950` state 1: 500 ms, `010301a8`, then `008bc590`).
const SKIP_CUT_SECONDS: f32 = 0.5;

/// Whether the response being said is over: a voiced one when its voice
/// ends or half a second after it was clicked away; a silent one after
/// `silent` seconds, or at once when clicked away.
fn response_done(
    voiced: bool,
    voice_done: bool,
    skipped_at: Option<f32>,
    skip: bool,
    shown_for: f32,
    now: f32,
    silent: f32,
) -> bool {
    if voiced {
        voice_done || skipped_at.is_some_and(|t| now - t >= SKIP_CUT_SECONDS)
    } else {
        skip || shown_for > silent
    }
}

/// Runs one of a line's result scripts, on the speaker (whose script
/// variables are its own).
fn run_line_script(
    order: &esm::LoadOrder,
    scripts: &ScriptCache,
    state: &mut GameState,
    source: Option<&str>,
    speaker: FormId,
) {
    if let Some(source) = source {
        let lines: Vec<&str> = source
            .lines()
            .map(|l| l.split(';').next().unwrap_or("").trim())
            .filter(|l| !l.is_empty())
            .collect();
        if lines.is_empty() {
            return;
        }
        println!("  result script: {}", lines.join(" / "));
        let flow =
            Runner::new(order, scripts, state).run_source(source, Some(speaker), Some(speaker));
        if flow == world::scripting::Flow::Stopped {
            println!("  (stopped: it needs a function that isn't carried out yet)");
        }
    }
}

/// A line starts: it's been said, the speaker has talked to the player,
/// the topics it names are learned, and its first result script runs (in
/// the dialogue menu, unless the line is flagged not to: `0083ebb0`).
fn begin_line(
    order: &esm::LoadOrder,
    scripts: &ScriptCache,
    state: &mut GameState,
    info: &Info,
    speaker: FormId,
    menu: bool,
) {
    dialogue::line_begins(state, info, speaker);
    let said: Vec<&str> = info.responses.iter().map(|r| r.text.as_str()).collect();
    println!("{} ({}): {}", speaker, info.form_id, said.join(" "));
    if !menu || dialogue::menu_runs_begin_script(info) {
        run_line_script(order, scripts, state, info.begin_script.as_deref(), speaker);
    }
}

/// What `talk` uses besides: `--talk`, scripts, scripted talking, the
/// menus, and the people (who flees).
type TalkExtras<'w, 's> = (
    ResMut<'w, AutoTalk>,
    Res<'w, Scripts>,
    ResMut<'w, ScriptedTalk>,
    ResMut<'w, crate::menus::Menus>,
    Query<'w, 's, &'static crate::ai::Walker>,
    ResMut<'w, crate::game_menus::GameMenus>,
    ResMut<'w, DialogueView>,
    ResMut<'w, AutoSay>,
    ResMut<'w, crate::chatter::Lines>,
);

/// Looking for someone to talk to, and the conversation itself.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn talk(
    mut commands: Commands,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    talkers: Res<Talkers>,
    crosshair: Res<crate::crosshair::Crosshair>,
    mut target: ResMut<TalkTarget>,
    (
        mut auto_talk,
        scripts,
        mut scripted,
        mut menus,
        walkers,
        mut game_menus,
        mut view,
        mut auto_say,
        mut lines,
    ): TalkExtras<'_, '_>,
    mut conversation: ResMut<Conversation>,
    mut player: ResMut<Player>,
    mut audio: ResMut<Assets<crate::sounds::PcmSound>>,
    voices: Query<(), With<AudioPlayer<crate::sounds::PcmSound>>>,
    mut prompt: Query<&mut Text, (With<Prompt>, Without<DialogueText>)>,
    mut panel: Query<(&mut Text, &mut Visibility), With<DialogueText>>,
) {
    let now = time.elapsed_secs();
    let order = &game.0.order;
    // Who is saying a line now, for scripts' `IsTalking`.
    let speaking = conversation
        .0
        .as_ref()
        .filter(|t| t.choices.is_none() && !t.zoom.closing())
        .map(|t| t.speaker.reference);
    if state.0.speaking.iter().copied().ne(speaking) {
        state.0.speaking = speaking.into_iter().collect();
    }
    if conversation.0.is_some() {
        let line_only = conversation.0.as_ref().is_some_and(|t| t.line_only);
        // The game's dialogue menu shows the conversation when its files
        // can be read (`game_menus::dialog`); it has the mouse and keys.
        let mut screen = if line_only { None } else { game_menus.screen() };
        if let (Some(s), Some(talk)) = (screen.as_deref_mut(), conversation.0.as_ref()) {
            if crate::game_menus::dialog::menu(s).is_none()
                && !crate::game_menus::dialog::open(s, &game.0, &talk.name)
            {
                screen = None;
            }
        }
        let answer = screen
            .as_deref_mut()
            .and_then(crate::game_menus::dialog::take_answer);
        // Under a service menu (barter, recipes) the menu is hidden and
        // nothing is chosen in it; the line being said when it opened was
        // cut short (`00763ff0`).
        let hidden = screen
            .as_deref_mut()
            .is_some_and(crate::game_menus::dialog::hidden);
        let cut = screen
            .as_deref_mut()
            .is_some_and(crate::game_menus::dialog::take_cut_line);
        let ended = screen.is_none()
            && !line_only
            && (keys.just_pressed(KeyCode::Tab) || keys.just_pressed(KeyCode::Escape));
        keys.clear_just_pressed(KeyCode::Escape);
        if ended {
            end(&mut commands, &mut conversation.0, &mut player, &mut panel);
            return;
        }
        let Conversation(Some(talk), top_level) = &mut *conversation else {
            return;
        };
        // The menu's zoom (`00762950`): in as it opens, the first line
        // already being said (`00761a20` starts it); out once it's to close,
        // the menu going only when that's over, nothing more said meanwhile.
        if !talk.line_only {
            let s = view.settings(&game.0);
            if talk.zoom.step(time.delta_secs(), &s) {
                if let Some(s) = screen.as_deref_mut() {
                    crate::game_menus::dialog::end(s);
                }
                end(&mut commands, &mut conversation.0, &mut player, &mut panel);
                return;
            }
            if talk.zoom.closing() {
                return;
            }
        }
        let skip = !hidden
            && (keys.just_pressed(KeyCode::Space)
                || answer == Some(ui::menus::dialog::Answer::Skip));
        if talk.choices.is_none() {
            // Saying the line: next response when the voice ends (or, with
            // no voice file, after `fDialogSpeechDelaySeconds`); a click or
            // Space moves on, the voice cut half a second later.
            let voice_done = talk.voice.is_none_or(|v| voices.get(v).is_err());
            if skip {
                println!(
                    "  line skipped ({}, response {})",
                    talk.info.form_id, talk.response
                );
            }
            if skip && talk.voice.is_some() && talk.skipped_at.is_none() {
                talk.skipped_at = Some(now);
            }
            // Cut short: the rest of its responses dropped (`0083e4c0(0)`),
            // the speaker stopping now.
            if cut && !talk.line_only {
                talk.response = talk.info.responses.len().saturating_sub(1);
            }
            let done = (cut && !talk.line_only)
                || response_done(
                    talk.voice.is_some(),
                    voice_done,
                    talk.skipped_at,
                    skip,
                    now - talk.since,
                    now,
                    silent_line_seconds(order),
                );
            if done {
                if let Some(v) = talk.voice.take() {
                    if let Ok(mut e) = commands.get_entity(v) {
                        println!("  voice stopped: {v}");
                        e.despawn();
                        // Skipped: the face stops saying it too.
                        crate::faces::cut_short(&mut commands, talk.speaker.reference);
                    } else {
                        println!("  voice ended: {v}");
                    }
                }
                talk.skipped_at = None;
                talk.response += 1;
                talk.since = now;
                if talk.response < talk.info.responses.len() {
                    talk.voice = play_voice(&mut commands, &mut audio, &game.0, talk);
                } else {
                    // The line is said: its second result script (the menu
                    // skips it for "run immediately" lines, `00762ff0`).
                    if talk.line_only || dialogue::menu_runs_end_script(&talk.info) {
                        run_line_script(
                            order,
                            &scripts.0,
                            &mut state.0,
                            talk.info.end_script.as_deref(),
                            talk.speaker.reference,
                        );
                    }
                    let next = if talk.line_only {
                        dialogue::AfterLine::Close
                    } else {
                        // A fresh draw for a run of random follow-ups.
                        state.0.roll();
                        let top =
                            top_level.get_or_insert_with(|| dialogue::top_level_topics(order));
                        dialogue::after_line(order, &talk.info, top, &talk.speaker, &state.0)
                    };
                    match next {
                        dialogue::AfterLine::FollowUp(info) => {
                            // The speaker goes straight on (`00762ff0`).
                            begin_line(
                                order,
                                &scripts.0,
                                &mut state.0,
                                &info,
                                talk.speaker.reference,
                                true,
                            );
                            talk.info = *info;
                            talk.response = 0;
                            talk.since = now;
                            talk.voice = play_voice(&mut commands, &mut audio, &game.0, talk);
                        }
                        dialogue::AfterLine::Topics(list) => {
                            if !auto_say.0.is_empty() {
                                let offered: Vec<&str> =
                                    list.iter().map(|c| c.label.as_str()).collect();
                                println!("Topics offered: {}", offered.join(" | "));
                            }
                            talk.choices = Some(list);
                        }
                        // A line said on its own just ends; the menu zooms
                        // out first (state 4, `00762ff0`, `00762950`).
                        dialogue::AfterLine::Close if !talk.line_only => {
                            talk.zoom.close();
                            return;
                        }
                        dialogue::AfterLine::Close => {
                            let (speaker, said) = (talk.speaker.reference, talk.say_to);
                            if let Some(s) = screen.as_deref_mut() {
                                crate::game_menus::dialog::end(s);
                            }
                            end(&mut commands, &mut conversation.0, &mut player, &mut panel);
                            // `SayTo`'s line is said: the speaker's
                            // `SayToDone` blocks (which may start the next).
                            if let Some(topic) = said {
                                Runner::new(order, &scripts.0, &mut state.0)
                                    .say_to_done(speaker, topic);
                            }
                            return;
                        }
                    }
                }
            }
        } else if let (false, Some(list)) = (hidden, &talk.choices) {
            // Choosing: the game's menu, else number keys.
            let digits = [
                KeyCode::Digit1,
                KeyCode::Digit2,
                KeyCode::Digit3,
                KeyCode::Digit4,
                KeyCode::Digit5,
                KeyCode::Digit6,
                KeyCode::Digit7,
                KeyCode::Digit8,
                KeyCode::Digit9,
            ];
            let picked = match answer {
                Some(ui::menus::dialog::Answer::Topic(i)) => Some(i),
                // `--choose`: the next reply on the list.
                _ if !auto_talk.1.is_empty() => auto_talk.1.pop_front().map(|n| n - 1),
                _ if screen.is_some() => None,
                _ => digits.iter().position(|k| keys.just_pressed(*k)),
            };
            // `--say` (testing).
            let picked = picked.or_else(|| {
                let offered: Vec<&str> = list.iter().map(|c| c.label.as_str()).collect();
                let i = auto_say.pick(&offered)?;
                println!("--say: {} (of {})", offered[i], offered.join(" | "));
                Some(i)
            });
            match picked {
                Some(i) if i < list.len() => {
                    let info = list[i].info.clone();
                    begin_line(
                        order,
                        &scripts.0,
                        &mut state.0,
                        &info,
                        talk.speaker.reference,
                        true,
                    );
                    talk.info = info;
                    talk.response = 0;
                    talk.since = now;
                    talk.choices = None;
                    talk.voice = play_voice(&mut commands, &mut audio, &game.0, talk);
                }
                Some(i) if i == list.len() && screen.is_none() && needs_leave(list) => {
                    end(&mut commands, &mut conversation.0, &mut player, &mut panel);
                    return;
                }
                _ => {}
            }
        }
        // The game's menu shows the line or the topics.
        if let Some(s) = screen {
            let talk = conversation.0.as_mut().unwrap();
            match &talk.choices {
                None => {
                    let key = (talk.info.form_id, talk.response);
                    if talk.shown_line != Some(key) {
                        let text = talk
                            .info
                            .responses
                            .get(talk.response)
                            .map(|r| r.text.clone())
                            .unwrap_or_default();
                        crate::game_menus::dialog::show_line(s, &text);
                        talk.shown_line = Some(key);
                        talk.shown_topics = false;
                    }
                }
                Some(list) => {
                    if !talk.shown_topics {
                        crate::game_menus::dialog::show_topics(s, list, f64::from(now));
                        talk.shown_topics = true;
                        talk.shown_line = None;
                    }
                }
            }
            for (_, mut visible) in &mut panel {
                *visible = Visibility::Hidden;
            }
            return;
        }
        let talk = conversation.0.as_ref().unwrap();
        let mut lines = String::new();
        match &talk.choices {
            None => {
                if let Some(r) = talk.info.responses.get(talk.response) {
                    lines = format!("{}: {}", talk.name, r.text);
                }
            }
            Some(list) => {
                for (i, c) in list.iter().enumerate() {
                    lines.push_str(&format!("{}) {}\n", i + 1, c.label));
                }
                if needs_leave(list) {
                    lines.push_str(&format!("{}) (Leave)", list.len() + 1));
                }
            }
        }
        for (mut text, mut visible) in &mut panel {
            if text.0 != lines {
                text.0 = lines.clone();
            }
            *visible = Visibility::Visible;
        }
        return;
    }

    // Not talking: the person the crosshair is on, within reach
    // (`crosshair`, the game's view caster). A body whose critical stage
    // culled it (disintegrated, gooified) isn't there to search: its ash
    // or goo pile is (`scripts`).
    let best = crosshair
        .target()
        .filter(|&r| !world::more_functions::body_gone(&state.0, r))
        .and_then(|r| talkers.0.iter().find(|t| t.reference == r).copied());
    target.0 = best.and_then(|t| {
        if target
            .0
            .as_ref()
            .is_some_and(|(old, _)| old.reference == t.reference)
        {
            return target.0.take();
        }
        let name = order.get(t.base)?.record().ok()?.full_name()?;
        Some((t, name))
    });
    // A script asked someone to talk to the player (`SayTo`,
    // `StartConversation`): they start, about the topic given.
    if let Some((speaker, topic, menu, say_to)) = scripted.0.take() {
        let found = talkers
            .0
            .iter()
            .find(|t| t.reference == speaker)
            .copied()
            // A talking activator isn't an actor: its own base and place.
            .or_else(|| {
                let base = world::scripting::base_of(order, speaker)?;
                (order.get(base)?.entry.header.kind.as_bytes() == b"TACT").then_some(Talker {
                    reference: speaker,
                    base,
                    position: [0.0; 3],
                })
            })
            .and_then(|t| {
                let name = order.get(t.base)?.record().ok()?.full_name()?;
                Some((t, name))
            });
        match found {
            Some((talker, name)) => {
                let topic = topic.unwrap_or(GREETING);
                // Saying stops the speech in progress first (`005c9100`
                // and the say, `008a20d0`, call `00934250`).
                crate::chatter::hush(&mut commands, &mut lines, talker.reference);
                if let Some(talk) = start_talk(
                    order,
                    &scripts.0,
                    &mut state.0,
                    talker,
                    name,
                    topic,
                    now,
                    menu,
                ) {
                    let mut talk = talk;
                    talk.say_to = say_to.then_some(topic);
                    talk.voice = play_voice(&mut commands, &mut audio, &game.0, &talk);
                    if menu {
                        player.ready = false;
                        for mut text in &mut prompt {
                            text.0.clear();
                        }
                    }
                    conversation.0 = Some(talk);
                }
            }
            None => {
                println!("A script has {speaker} talk to the player, but they aren't loaded here.")
            }
        }
        return;
    }
    // Scripts can turn the roll-over text, and with it talking to people,
    // off (`DisablePlayerControls`).
    if state.0.controls_off[world::scripting::controls::ROLLOVER] && !auto_talk.0 {
        return;
    }
    // `--talk`: the nearest person, once the place has loaded.
    let auto = auto_talk.0 && player.ready && !talkers.0.is_empty();
    let chosen = if auto {
        auto_talk.0 = false;
        // (Nearest across the floor: the eye stands over the feet.)
        let eye = player.character.feet;
        talkers
            .0
            .iter()
            .min_by(|a, b| {
                let d = |t: &Talker| {
                    (t.position[0] - eye[0]).powi(2) + (t.position[1] - eye[1]).powi(2)
                };
                d(a).total_cmp(&d(b))
            })
            .and_then(|t| {
                let name = order.get(t.base)?.record().ok()?.full_name()?;
                Some((*t, name))
            })
    } else {
        target.0.clone()
    };
    let Some((talker, name)) = chosen else {
        return;
    };
    // Using a person (`world::living::pickpocket::use_person`): the dead
    // are searched (their inventory opens as a container); sneaking, the
    // living's pockets picked; the unconscious and fleeing refuse; a
    // teammate's wheel of orders; else talking.
    use world::living::pickpocket::{use_person, Use};
    let fleeing = walkers
        .iter()
        .any(|w| w.reference == talker.reference && w.fleeing.is_some());
    let using = if auto {
        Use::Talk
    } else {
        use_person(order, &state.0, talker.reference, fleeing)
    };
    if !auto {
        let verb = |setting: &str, exe: &str| {
            world::scripting::game_setting_text(order, setting).unwrap_or_else(|| exe.into())
        };
        for mut text in &mut prompt {
            let line = match &using {
                Use::Search => format!("E) {} {name}", verb("sSearch", "Search")),
                Use::Pickpocket => format!("E) {} {name}", verb("sPickpocket", "Pickpocket")),
                _ => format!("E) Talk to {name}"),
            };
            if text.0 != line {
                text.0 = line;
            }
        }
        if !keys.just_pressed(KeyCode::KeyE) {
            return;
        }
        keys.clear_just_pressed(KeyCode::KeyE);
    }
    match using {
        Use::Search => {
            menus.push(crate::menus::Menu::Container(talker.reference, name));
            return;
        }
        Use::Pickpocket => {
            // The game's container menu in its pickpocket mode
            // (`menus\container_menu.xml`) isn't drawn here yet; its rules
            // are `world::living::pickpocket`.
            println!("The pickpocket menu for {name} would open here (not drawn yet).");
            return;
        }
        Use::Refused(why, icon) => {
            // Shown as the game's notice (`world::scripting::Event`).
            println!("{why}");
            state.0.events.push(world::scripting::Event::Message {
                title: None,
                text: why,
                buttons: Vec::new(),
                icon: Some(icon.to_string()),
            });
            return;
        }
        Use::Wheel => {
            menus.push(crate::menus::Menu::CompanionWheel(talker.reference));
            return;
        }
        // A teammate who can't take orders now: nothing (`00754d90`).
        Use::Nothing => return,
        Use::Talk => {}
    }
    // An NPC's `Activate` (`005fa330`): the greeting found for them decides
    // between the dialogue menu and a line said on its own, through the
    // GREET procedure (`world::dialogue::activation_says_a_line`); the line
    // is then picked again (`0057b7c0` asks `0061b320` itself), as the menu
    // picks its own. Either way the speech in progress stops (`00934250`).
    let menu = Speaker::load(order, talker.reference, talker.base).is_none_or(|speaker| {
        state.0.roll();
        dialogue::pick(order, GREETING, &speaker, &state.0)
            .is_none_or(|info| !dialogue::activation_says_a_line(&info))
    });
    crate::chatter::hush(&mut commands, &mut lines, talker.reference);
    if !menu {
        println!(
            "{now:.1} s: {name} only says a line when activated (a one-response Goodbye greeting)."
        );
    }
    let Some(mut talk) = start_talk(
        order,
        &scripts.0,
        &mut state.0,
        talker,
        name,
        GREETING,
        now,
        menu,
    ) else {
        return;
    };
    talk.voice = play_voice(&mut commands, &mut audio, &game.0, &talk);
    if menu {
        player.ready = false;
        for mut text in &mut prompt {
            text.0.clear();
        }
    }
    conversation.0 = Some(talk);
}

/// Someone starts talking about a topic: the first line their conditions
/// allow, its first result script run.
#[allow(clippy::too_many_arguments)]
fn start_talk(
    order: &esm::LoadOrder,
    scripts: &ScriptCache,
    state: &mut GameState,
    talker: Talker,
    name: String,
    topic: FormId,
    now: f32,
    menu: bool,
) -> Option<Talk> {
    let speaker = Speaker::load(order, talker.reference, talker.base)?;
    // A fresh draw for a run of random lines (`dialogue::choose`).
    state.roll();
    let Some(info) = dialogue::pick(order, topic, &speaker, state) else {
        println!("{name} has nothing to say about {topic}.");
        return None;
    };
    begin_line(order, scripts, state, &info, talker.reference, menu);
    if !menu {
        let said: Vec<&str> = info.responses.iter().map(|r| r.text.as_str()).collect();
        println!(
            "{now:.1} s: {name} says to the player, no menu ({}): {}",
            info.form_id,
            said.join(" ")
        );
    }
    // The menu's zoom follows a running dialogue package's (`00761a20`:
    // package type 15, `00672850` reads its `PKDD` float).
    let package_zoom = world::ai::current_package(order, state, talker.reference)
        .filter(|p| p.kind == world::ai::kinds::DIALOGUE)
        .and_then(|p| world::ai::dialogue_data(order, p.form_id))
        .map(|d| d.fov);
    Some(Talk {
        package_zoom,
        speaker,
        name,
        info,
        response: 0,
        since: now,
        voice: None,
        skipped_at: None,
        choices: None,
        line_only: !menu,
        say_to: None,
        shown_line: None,
        shown_topics: false,
        zoom: MenuZoom::opening(),
    })
}

/// Whether the menu needs a way out of its own: none of its choices ends
/// the conversation (the game's people mostly have a "Goodbye." topic;
/// Tab leaves too).
fn needs_leave(list: &[Choice]) -> bool {
    !list.iter().any(|c| c.info.flags & dialogue::GOODBYE != 0)
}

fn end(
    commands: &mut Commands,
    conversation: &mut Option<Talk>,
    player: &mut Player,
    panel: &mut Query<(&mut Text, &mut Visibility), With<DialogueText>>,
) {
    if let Some(talk) = conversation.take() {
        if let Some(v) = talk.voice {
            if let Ok(mut e) = commands.get_entity(v) {
                e.despawn();
                crate::faces::cut_short(commands, talk.speaker.reference);
            }
        }
    }
    player.ready = true;
    for (mut text, mut visible) in panel.iter_mut() {
        text.0.clear();
        *visible = Visibility::Hidden;
    }
}

/// The dialogue menu's view (`world::dialogue_view`): its settings, the
/// player's focus on the speaker, and the fields of view.
#[derive(Resource, Default)]
pub struct DialogueView {
    settings: Option<ViewSettings>,
    focus: Focus,
    /// The world's and the first-person view's fields of view while the
    /// menu has the view, then on their way back to the default after.
    fovs: Option<(f32, f32)>,
    /// The menu had the view last frame.
    active: bool,
    /// Where the head was found has been printed for this conversation.
    logged: bool,
}

impl DialogueView {
    fn settings(&mut self, game: &cellview::Game) -> ViewSettings {
        *self.settings.get_or_insert_with(|| {
            ViewSettings::read(&game.order, |s, k| game.settings.float(s, k))
        })
    }
}

/// The speaker's head as `00953060` measures it: the bound of their face
/// node, here the head parts' vertices as skinned now (a box's middle and
/// the farthest vertex from it; how Gamebryo merges the skinned pieces'
/// own bounds into the node's isn't reproduced). Without head parts, the
/// `Bip01 Head` bone (else `Bip01 Speaker`) with radius 32 (`0101e340`).
#[allow(clippy::type_complexity)]
fn head_bound(
    speaker: FormId,
    roots: &Query<(
        Entity,
        &crate::scripts::PlacedRef,
        Option<(&crate::ai::Walker, &crate::actors::ActorRig)>,
    )>,
    pieces: &Query<(&Mesh3d, &SkinnedMesh, &ChildOf), With<crate::faces::FacePiece>>,
    joints: &Query<&GlobalTransform>,
    meshes: &Assets<Mesh>,
    binds: &Assets<SkinnedMeshInverseBindposes>,
    now: f32,
) -> Option<([f32; 3], f32)> {
    let (root, _, rig) = roots.iter().find(|(_, p, _)| p.0 == speaker.0)?;
    // Just placed (talked to as the place loads, `--talk`): the bones
    // aren't in the world yet (Bevy places them after the frame's
    // updates), so wait a frame.
    if joints
        .get(root)
        .is_ok_and(|g| *g == GlobalTransform::IDENTITY)
    {
        return None;
    }
    let mut points: Vec<[f32; 3]> = Vec::new();
    for (mesh, skin, child_of) in pieces {
        if child_of.parent() != root {
            continue;
        }
        let (Some(mesh), Some(inverse)) = (meshes.get(&mesh.0), binds.get(&skin.inverse_bindposes))
        else {
            continue;
        };
        let (
            Some(VertexAttributeValues::Float32x3(positions)),
            Some(VertexAttributeValues::Uint16x4(indices)),
            Some(VertexAttributeValues::Float32x4(weights)),
        ) = (
            mesh.attribute(Mesh::ATTRIBUTE_POSITION),
            mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX),
            mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT),
        )
        else {
            continue;
        };
        let matrices: Vec<Option<Mat4>> = skin
            .joints
            .iter()
            .zip(inverse.iter())
            .map(|(&e, bind)| joints.get(e).ok().map(|g| g.compute_matrix() * *bind))
            .collect();
        for ((p, js), ws) in positions.iter().zip(indices).zip(weights) {
            let v = Vec3::from(*p);
            let mut world = Vec3::ZERO;
            for k in 0..4 {
                if ws[k] > 0.0 {
                    if let Some(Some(m)) = matrices.get(usize::from(js[k])) {
                        world += m.transform_point3(v) * ws[k];
                    }
                }
            }
            points.push(game_point(world));
        }
    }
    if points.is_empty() {
        let (walker, rig) = rig?;
        let pose = rig.pose_now(now);
        let bone = ["Bip01 Head", "Bip01 Speaker"].iter().find_map(|name| {
            rig.skeleton
                .bones
                .iter()
                .position(|b| b.name.eq_ignore_ascii_case(name))
        })?;
        let at = walker.placement().apply_point(pose.get(bone)?.translation);
        return Some((at, 32.0));
    }
    let mut lo = [f32::MAX; 3];
    let mut hi = [f32::MIN; 3];
    for p in &points {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    let center = [0, 1, 2].map(|k| (lo[k] + hi[k]) * 0.5);
    let radius = points
        .iter()
        .map(|p| {
            (0..3)
                .map(|k| (p[k] - center[k]).powi(2))
                .sum::<f32>()
                .sqrt()
        })
        .fold(0.0f32, f32::max);
    Some((center, radius))
}

/// Every frame of the dialogue menu the player's view is on the speaker
/// (`world::dialogue_view::Focus`, `00953060`, run by the menu's update
/// `00762950` with the menu's zoom); after it, the field of view goes back
/// to the default (`0095de30`). Lines said on their own (`SayTo`) leave the
/// view alone.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn focus_camera(
    time: Res<Time>,
    game: Res<GameFiles>,
    conversation: Res<Conversation>,
    mut view: ResMut<DialogueView>,
    roots: Query<(
        Entity,
        &crate::scripts::PlacedRef,
        Option<(&crate::ai::Walker, &crate::actors::ActorRig)>,
    )>,
    pieces: Query<(&Mesh3d, &SkinnedMesh, &ChildOf), With<crate::faces::FacePiece>>,
    joints: Query<&GlobalTransform>,
    (meshes, binds): (Res<Assets<Mesh>>, Res<Assets<SkinnedMeshInverseBindposes>>),
    mut cameras: Query<(&mut FlyCamera, &mut Transform, &mut Projection)>,
) {
    let Ok((mut fly, mut transform, mut projection)) = cameras.single_mut() else {
        return;
    };
    let s = view.settings(&game.0);
    let dt = time.delta_secs();
    let defaults = (
        cellview::GAME_FOV_DEGREES,
        crate::viewmodel::FIRST_PERSON_FOV_DEGREES,
    );
    let talk = conversation.0.as_ref().filter(|t| !t.line_only);
    let Some(talk) = talk else {
        // After the menu: back toward the defaults.
        view.active = false;
        let Some((world_fov, first_fov)) = view.fovs else {
            return;
        };
        let back = (
            view_rules::fov_back(world_fov, defaults.0, dt, &s),
            view_rules::fov_back(first_fov, defaults.1, dt, &s),
        );
        view.fovs = (back != defaults).then_some(back);
        if let Projection::Perspective(p) = &mut *projection {
            p.fov = cellview::vertical_fov(back.0);
        }
        return;
    };
    if !view.active {
        // The menu opens (`00761a20` focuses with 0: nothing zoomed yet).
        view.active = true;
        view.logged = false;
        view.focus = Focus::default();
        if view.fovs.is_none() {
            view.fovs = Some(defaults);
        }
    }
    let Some(head) = head_bound(
        talk.speaker.reference,
        &roots,
        &pieces,
        &joints,
        &meshes,
        &binds,
        time.elapsed_secs(),
    ) else {
        return;
    };
    let input = FocusInput {
        eye: game_point(transform.translation),
        head,
        percent: view_rules::focus_percent(talk.zoom.percent, talk.package_zoom),
        zooming_out: talk.zoom.closing(),
        dt,
        heading: -fly.yaw,
        pitch: fly.pitch,
        fovs: view.fovs.unwrap_or(defaults),
    };
    let out = view.focus.frame(&s, &input);
    if !view.logged {
        view.logged = true;
        let (c, r) = head;
        println!(
            "Dialogue view: eye {:.0},{:.0},{:.0}; the speaker's head {:.0},{:.0},{:.0}, radius {r:.1}.",
            input.eye[0], input.eye[1], input.eye[2], c[0], c[1], c[2]
        );
    }
    view.fovs = Some(out.fovs);
    fly.yaw = cellview::space::heading_to_yaw(out.heading);
    fly.pitch = out.pitch.clamp(-1.54, 1.54);
    transform.rotation = Quat::from_euler(EulerRot::YXZ, fly.yaw, fly.pitch, 0.0);
    if let Projection::Perspective(p) = &mut *projection {
        p.fov = cellview::vertical_fov(out.fovs.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn say_picks_the_named_topic_once_in_order() {
        let mut say = AutoSay(["i'm IN".to_string(), "sure".to_string()].into());
        let offered = [
            "Okay, I'm in.",
            "[END TUTORIAL] I think I've learned enough.",
        ];
        // Not offered: nothing chosen, the wish kept.
        assert_eq!(say.pick(&["Goodbye."]), None);
        assert_eq!(say.pick(&offered), Some(0));
        assert_eq!(say.pick(&offered), None);
        assert_eq!(say.pick(&["No.", "Sure, I'll come with you."]), Some(1));
        assert!(say.0.is_empty());
    }

    /// `00762950`: a clicked-away voice plays on for 500 ms; `008a20d0`: a
    /// line without a voice file lasts `fDialogSpeechDelaySeconds`.
    #[test]
    fn responses_end_with_the_voice_or_the_speech_delay() {
        // Voiced: not before the voice ends, unless skipped 0.5 s ago.
        assert!(!response_done(true, false, None, false, 9.0, 10.0, 2.0));
        assert!(response_done(true, true, None, false, 9.0, 10.0, 2.0));
        assert!(!response_done(true, false, Some(9.7), true, 9.0, 10.0, 2.0));
        assert!(response_done(true, false, Some(9.5), true, 9.0, 10.0, 2.0));
        // Silent: the speech delay, or a click.
        assert!(!response_done(false, true, None, false, 1.9, 10.0, 2.0));
        assert!(response_done(false, true, None, false, 2.1, 10.0, 2.0));
        assert!(response_done(false, true, None, true, 0.1, 10.0, 2.0));
    }
}
