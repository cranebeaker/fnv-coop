//! Finding a sound record's file the way the game does.

use cellview::{Game, Options};
use esm::FormId;
use world::sound::Sound;

fn sound(file: &str) -> Sound {
    Sound {
        form_id: FormId(1),
        file: file.into(),
        flags: 0,
        min_distance: 0.0,
        max_distance: 0.0,
    }
}

#[test]
fn a_missing_wav_plays_its_ogg_and_a_folder_plays_one_of_its_files() {
    let data = testdata::quests("cellview-sounds");
    data.write("sound/fx/test/loop.ogg", b"OggS loop");
    data.write("sound/fx/test/many/a.wav", b"RIFF a");
    data.write("sound/fx/test/many/b.wav", b"RIFF b");
    data.write("sound/fx/test/many/deeper/c.wav", b"RIFF c");
    let options = Options {
        official: true,
        ..Options::default()
    };
    let game = Game::open(data.path(), &options).unwrap();

    let (path, bytes) = game
        .sound_file(&sound("sound\\fx\\test\\loop.wav"), 0)
        .unwrap();
    assert_eq!(path, "sound\\fx\\test\\loop.ogg");
    assert_eq!(bytes, b"OggS loop");

    // A folder: one of the files directly in it, by the pick.
    let many = sound("sound\\fx\\test\\many");
    assert!(many.is_folder());
    let first = game.sound_file(&many, 0).unwrap().1;
    let second = game.sound_file(&many, 1).unwrap().1;
    assert_eq!(
        (first.as_slice(), second.as_slice()),
        (&b"RIFF a"[..], &b"RIFF b"[..])
    );
    assert_eq!(game.sound_file(&many, 2).unwrap().1, b"RIFF a");

    assert!(game
        .sound_file(&sound("sound\\fx\\nothing.wav"), 0)
        .is_none());
}
