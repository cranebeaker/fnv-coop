//! Lines people say without the dialogue menu (`world::social`): greetings
//! to the player, idle chatter, the lines of conversations between two
//! people, and "Say To" dialogue packages. The game says these through its
//! GREET procedure (`008dbe30`) and a conversation's playback (`009ee0a0`):
//! each line in its speaker's voice, one response after another, with its
//! lip sync (`faces`); its result scripts run as it starts and after it's
//! said, as the dialogue's do. No text is shown (the game's general
//! subtitles are off by default); the lines are printed.
//!
//! Not done: the voices aren't placed in the world (they play at full
//! volume wherever the speaker is; the game's 3D sound isn't traced here).

use std::collections::HashMap;

use bevy::audio::AudioPlayer;
use bevy::prelude::*;
use esm::FormId;
use world::dialogue::{self, Info, Speaker};
use world::scripting::Runner;

use crate::dialogue::DialogueState;
use crate::scripts::Scripts;
use crate::GameFiles;

/// A line someone is to say.
#[derive(Debug, Clone)]
pub struct LineRequest {
    pub speaker: FormId,
    pub listener: FormId,
    pub info: Info,
    /// Said in a conversation, with whether its says force a tree request
    /// for the speaker (`world::talk_idles::conversation_forces_tree`);
    /// none for the GREET procedure's lines (greetings, chatter, Say To),
    /// which force one unless the speaker's package has idles
    /// (`world::talk_idles::greet_forces_tree`).
    pub conversation: Option<bool>,
    /// A GREET line said to its listener (a greeting, a "Say To"): while
    /// it's said the speaker's greeting timer is held at `fAIGreetingTimer`
    /// and, said to the player, the player counts as spoken to (`008dbe30`
    /// with a listener: process +0x330, player +0x6cc). Idle chatter has
    /// no listener in the game (`008eeec0` passes none).
    pub to_listener: bool,
}

/// A response begun this frame (each one of a line is its own say,
/// `008a20d0`): who says it to whom, and how its caller asks for idles.
#[derive(Debug, Clone)]
pub struct ResponseStarted {
    pub speaker: FormId,
    pub listener: FormId,
    pub response: world::dialogue::Response,
    pub conversation: Option<bool>,
}

/// A line being said: the response and since when (seconds), its voice.
struct Saying {
    info: Info,
    listener: FormId,
    conversation: Option<bool>,
    to_listener: bool,
    response: usize,
    since: f32,
    voice: Option<Entity>,
}

/// Lines said without the dialogue menu: those to start, those being said
/// (by speaker), and those that ended this frame.
#[derive(Resource, Default)]
pub struct Lines {
    pub queue: Vec<LineRequest>,
    saying: HashMap<FormId, Saying>,
    pub done: Vec<(FormId, FormId)>,
    /// The responses begun this frame (for the speakers' and listeners'
    /// gestures, `sitting::idle_requests`).
    pub started: Vec<ResponseStarted>,
}

impl Lines {
    /// Whether someone is saying a line now.
    pub fn is_saying(&self, who: FormId) -> bool {
        self.saying.contains_key(&who) || self.queue.iter().any(|r| r.speaker == who)
    }

    /// Asks someone to say a line with no listener of its own (idle
    /// chatter; `listener` is only whom it's printed as said to).
    pub fn say(&mut self, speaker: FormId, listener: FormId, info: Info) {
        self.queue.push(LineRequest {
            speaker,
            listener,
            info,
            conversation: None,
            to_listener: false,
        });
    }

    /// Asks someone to say a GREET line to `listener` (a greeting, a
    /// "Say To" package's line).
    pub fn say_to(&mut self, speaker: FormId, listener: FormId, info: Info) {
        self.queue.push(LineRequest {
            speaker,
            listener,
            info,
            conversation: None,
            to_listener: true,
        });
    }

    /// Whether someone is saying (or about to say) a GREET line to a
    /// listener.
    pub fn greeting(&self, who: FormId) -> bool {
        self.saying.get(&who).is_some_and(|s| s.to_listener)
            || self.queue.iter().any(|r| r.speaker == who && r.to_listener)
    }

    /// Whether anyone is saying (or about to say) a GREET line to `whom`
    /// (for the player: +0x6cc, `world::social::GreetingCheck`).
    pub fn spoken_to(&self, whom: FormId) -> bool {
        self.saying
            .values()
            .any(|s| s.to_listener && s.listener == whom)
            || self
                .queue
                .iter()
                .any(|r| r.to_listener && r.listener == whom)
    }

    /// Asks someone to say a conversation's line (`009ee0a0`), its says
    /// forcing a tree request or not (`force_tree`).
    pub fn say_in_conversation(
        &mut self,
        speaker: FormId,
        listener: FormId,
        info: Info,
        force_tree: bool,
    ) {
        self.queue.push(LineRequest {
            speaker,
            listener,
            info,
            conversation: Some(force_tree),
            to_listener: false,
        });
    }

    /// Discard lines from the world that was just replaced. Their result
    /// scripts must not run against the restored game state.
    pub fn discard_pending(&mut self) {
        self.queue.clear();
        self.saying.clear();
        self.done.clear();
        self.started.clear();
    }
}

/// How long a response stays without a voice: the dialogue's reading pace
/// (a guess there too).
fn reading_time(text: &str) -> f32 {
    (text.split_whitespace().count() as f32 * 0.35).max(2.0)
}

/// A response's voice, if its file is found, with its lip sync.
fn play_voice(
    commands: &mut Commands,
    audio: &mut Assets<crate::sounds::PcmSound>,
    game: &cellview::Game,
    speaker: FormId,
    info: &Info,
    response: usize,
) -> Option<Entity> {
    let r = info.responses.get(response)?;
    let base = world::scripting::base_of(&game.order, speaker)?;
    let voice = Speaker::load(&game.order, speaker, base)?.voice?;
    let path = dialogue::voice_path(&game.order, info, r, voice)?;
    let bytes = game.assets.read(&path).ok()??;
    let source = crate::sounds::voice_handle(&path, &bytes, audio)?;
    let (settings, voice) = crate::faces::voice_playback(game, &path, speaker);
    Some(commands.spawn((AudioPlayer(source), settings, voice)).id())
}

/// Runs one of a line's result scripts on its speaker.
fn run_script(
    game: &cellview::Game,
    scripts: &Scripts,
    state: &mut world::scripting::GameState,
    source: Option<&str>,
    speaker: FormId,
) {
    let Some(source) = source.filter(|s| {
        s.lines()
            .any(|l| !l.split(';').next().unwrap_or("").trim().is_empty())
    }) else {
        return;
    };
    let flow = Runner::new(&game.order, &scripts.0, state).run_source(
        source,
        Some(speaker),
        Some(speaker),
    );
    if flow == world::scripting::Flow::Stopped {
        println!("  (a line's result script stopped: it needs a function not carried out yet)");
    }
}

/// Every frame: queued lines start (said once, their first result script
/// run, the first response's voice), responses follow one another as their
/// voices end (or after a reading pause without one), and a line that's
/// over runs its second result script and is reported in `done`. Whoever
/// says a line counts as talking (`IsTalking`).
#[allow(clippy::too_many_arguments)]
pub fn say_lines(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    mut state: ResMut<DialogueState>,
    mut lines: ResMut<Lines>,
    mut audio: ResMut<Assets<crate::sounds::PcmSound>>,
    voices: Query<(), With<AudioPlayer<crate::sounds::PcmSound>>>,
) {
    let now = time.elapsed_secs();
    let lines = &mut *lines;
    lines.done.clear();
    lines.started.clear();
    for request in std::mem::take(&mut lines.queue) {
        if lines.saying.contains_key(&request.speaker) {
            continue;
        }
        let info = request.info;
        state.0.said.insert(info.form_id);
        let said: Vec<&str> = info.responses.iter().map(|r| r.text.as_str()).collect();
        println!(
            "{:.1} s: {} says to {} ({}): {}",
            now,
            request.speaker,
            request.listener,
            info.form_id,
            said.join(" ")
        );
        run_script(
            &game.0,
            &scripts,
            &mut state.0,
            info.begin_script.as_deref(),
            request.speaker,
        );
        let voice = play_voice(
            &mut commands,
            &mut audio,
            &game.0,
            request.speaker,
            &info,
            0,
        );
        if let Some(r) = info.responses.first() {
            lines.started.push(ResponseStarted {
                speaker: request.speaker,
                listener: request.listener,
                response: r.clone(),
                conversation: request.conversation,
            });
        }
        lines.saying.insert(
            request.speaker,
            Saying {
                info,
                listener: request.listener,
                conversation: request.conversation,
                to_listener: request.to_listener,
                response: 0,
                since: now,
                voice,
            },
        );
    }
    let mut over = Vec::new();
    let started = &mut lines.started;
    for (&speaker, saying) in lines.saying.iter_mut() {
        let done = match saying.voice {
            Some(v) => voices.get(v).is_err(),
            None => {
                let text = saying
                    .info
                    .responses
                    .get(saying.response)
                    .map_or("", |r| r.text.as_str());
                now - saying.since > reading_time(text)
            }
        };
        if !done {
            continue;
        }
        saying.response += 1;
        saying.since = now;
        if saying.response < saying.info.responses.len() {
            started.push(ResponseStarted {
                speaker,
                listener: saying.listener,
                response: saying.info.responses[saying.response].clone(),
                conversation: saying.conversation,
            });
            saying.voice = play_voice(
                &mut commands,
                &mut audio,
                &game.0,
                speaker,
                &saying.info,
                saying.response,
            );
        } else {
            over.push(speaker);
        }
    }
    for speaker in over {
        if let Some(saying) = lines.saying.remove(&speaker) {
            run_script(
                &game.0,
                &scripts,
                &mut state.0,
                saying.info.end_script.as_deref(),
                speaker,
            );
            lines.done.push((speaker, saying.info.form_id));
        }
    }
    for &speaker in lines.saying.keys() {
        state.0.speaking.insert(speaker);
    }
}

/// A line is cut short (the speaker died, or was talked to): its voice
/// stops and the face eases back.
pub fn hush(commands: &mut Commands, lines: &mut Lines, who: FormId) {
    lines.queue.retain(|r| r.speaker != who);
    if let Some(saying) = lines.saying.remove(&who) {
        if let Some(v) = saying.voice {
            if let Ok(mut e) = commands.get_entity(v) {
                e.despawn();
            }
        }
        crate::faces::cut_short(commands, who);
    }
}
