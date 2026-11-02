//! A line's own speaker (`ANAM`) chooses the voice type of its voice file
//! (`00616fa0`): Dead Money's narrator says lines whose speaker is Elijah,
//! recorded in Elijah's voice type folder.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::{group, record, sub, zstr};
use world::dialogue::{self, Info};

const NARRATOR_VOICE: u32 = 0x100;
const ELIJAH_VOICE: u32 = 0x101;
const NARRATOR: u32 = 0x200;
const ELIJAH: u32 = 0x201;
const QUEST: u32 = 0x300;
const TOPIC: u32 = 0x400;
const OWN_LINE: u32 = 0x500;
const ELIJAHS_LINE: u32 = 0x501;

#[test]
fn a_line_with_its_own_speaker_uses_that_speakers_voice_type() {
    let data = testdata::TempData::empty("voice-speaker");
    let mut plugin = record(
        b"TES4",
        0,
        &sub(b"HEDR", &{
            let mut h = 1.34f32.to_le_bytes().to_vec();
            h.extend([0; 8]);
            h
        }),
    );
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let mut voices = record(b"VTYP", NARRATOR_VOICE, &edid("NarratorVoice"));
    voices.extend(record(b"VTYP", ELIJAH_VOICE, &edid("ElijahVoice")));
    plugin.extend(group(*b"VTYP", 0, &voices));
    let npc = |id: u32, name: &str, voice: u32| {
        let mut d = edid(name);
        d.extend(sub(b"ACBS", &[0; 24]));
        d.extend(sub(b"VTCK", &voice.to_le_bytes()));
        record(b"NPC_", id, &d)
    };
    let mut people = npc(NARRATOR, "Narrator", NARRATOR_VOICE);
    people.extend(npc(ELIJAH, "Elijah", ELIJAH_VOICE));
    plugin.extend(group(*b"NPC_", 0, &people));
    plugin.extend(group(
        *b"QUST",
        0,
        &record(b"QUST", QUEST, &edid("TestIntro")),
    ));
    let line = |id: u32, speaker: Option<u32>| {
        let mut d = sub(b"DATA", &[0, 0, 0, 0]);
        d.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
        d.extend(sub(b"TRDT", &{
            let mut t = [0u8; 24];
            t[12] = 1;
            t
        }));
        d.extend(sub(b"NAM1", &zstr("A chance to begin again.")));
        if let Some(s) = speaker {
            d.extend(sub(b"ANAM", &s.to_le_bytes()));
        }
        record(b"INFO", id, &d)
    };
    let mut topic = record(b"DIAL", TOPIC, &edid("IntroTopic"));
    let mut lines = line(OWN_LINE, None);
    lines.extend(line(ELIJAHS_LINE, Some(ELIJAH)));
    topic.extend(group(TOPIC.to_le_bytes(), 7, &lines));
    plugin.extend(group(*b"DIAL", 0, &topic));
    data.write("FalloutNV.esm", &plugin);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();

    let path = |id: u32| {
        let rr = order.get(FormId(id)).unwrap();
        let record = rr.record().unwrap();
        let mut info = Info::parse(&order, &rr, &record);
        info.topic = Some(FormId(TOPIC));
        let response = info.responses[0].clone();
        dialogue::voice_path(&order, &info, &response, FormId(NARRATOR_VOICE)).unwrap()
    };
    // No speaker of its own: the one saying it gives the voice type.
    assert!(
        path(OWN_LINE).contains("\\narratorvoice\\"),
        "{}",
        path(OWN_LINE)
    );
    // Elijah's line, said by the narrator: Elijah's voice type.
    assert!(
        path(ELIJAHS_LINE).contains("\\elijahvoice\\"),
        "{}",
        path(ELIJAHS_LINE)
    );
}
