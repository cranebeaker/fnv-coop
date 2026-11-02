//! The hacking game's rules (`world::hacking`), on generated words.

use esm::FormId;
use world::grass::Twister;
use world::hacking::{
    attempts, choose_words, dictionary_words, likeness, shuffle, word_count, word_length, Game,
    Outcome, PasswordFile, Selection, FILE_CHARS, GARBAGE, LINE_CHARS,
};

/// The game's generator, run on past its seeding: seeded by multiplying
/// (`00aa51a0`), its words' low bits are all alike for the first few
/// hundred draws; the game's has been drawing since it started.
fn warmed(seed: u32) -> Twister {
    let mut t = Twister::seeded(seed);
    for _ in 0..624 * 3 {
        t.next_u32();
    }
    t
}

/// A clock that moves a millisecond each time it's read.
fn ticking() -> impl FnMut() -> u32 {
    let mut t = 0u32;
    move || {
        t += 1;
        t
    }
}

/// Every word of `len` letters over `letters`.
fn all_words(letters: &[u8], len: usize) -> Vec<Vec<u8>> {
    let mut out = vec![Vec::new()];
    for _ in 0..len {
        out = out
            .into_iter()
            .flat_map(|w: Vec<u8>| {
                letters.iter().map(move |&c| {
                    let mut w = w.clone();
                    w.push(c);
                    w
                })
            })
            .collect();
    }
    out
}

fn texts(name: &str) -> String {
    world::hacking::TEXTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, d)| d.to_string())
        .unwrap()
}

#[test]
fn word_length_follows_difficulty_and_the_form_ids_last_bit() {
    assert_eq!(word_length(FormId(0x100), 0), 4);
    assert_eq!(word_length(FormId(0x101), 0), 5);
    assert_eq!(word_length(FormId(0x100), 2), 8);
    assert_eq!(word_length(FormId(0x101), 3), 11);
    // 12 and 13 are both 12.
    assert_eq!(word_length(FormId(0x100), 4), 12);
    assert_eq!(word_length(FormId(0x101), 4), 12);
}

#[test]
fn more_science_above_the_minimum_means_fewer_words() {
    // Just the minimum: the most; everything above it: the fewest.
    assert_eq!(word_count(25.0, 25.0, 20, 5), 20);
    assert_eq!(word_count(100.0, 25.0, 20, 5), 5);
    // Half the way: 15 × ½ = 7.5, rounded up.
    assert_eq!(word_count(62.5, 25.0, 20, 5), 13);
    // Very hard has no span: half.
    assert_eq!(word_count(100.0, 100.0, 20, 5), 13);
    // The settings are capped: at most 20, the minimum at most the most.
    assert_eq!(word_count(0.0, 0.0, 30, 25), 20);
    // Above 100 (a perk's bonus) the share goes negative: fewer words,
    // and below none the game's unsigned count wraps to the most.
    assert_eq!(word_count(110.0, 50.0, 20, 5), 2);
    assert_eq!(word_count(110.0, 80.0, 20, 5), 20);
}

#[test]
fn likeness_counts_letters_in_place_over_the_shorter() {
    assert_eq!(likeness(b"TERMINAL", b"TERMINAL"), 8);
    assert_eq!(likeness(b"SPRING", b"STRING"), 5);
    assert_eq!(likeness(b"ABCD", b"DCBA"), 0);
    assert_eq!(likeness(b"(", b"ABCD"), 0);
    assert_eq!(likeness(b"AB", b"ABCD"), 2);
}

#[test]
fn the_dictionary_is_read_word_by_word_into_the_games_list_order() {
    let words = dictionary_words(b"ABCD EFG HIJK  LMNOP QRST ", 4);
    // Each added at the front.
    assert_eq!(
        words,
        vec![b"QRST".to_vec(), b"HIJK".to_vec(), b"ABCD".to_vec()]
    );
}

#[test]
fn shuffling_is_a_shell_sort_on_coin_flips() {
    // Two items: one comparison; they swap on an even word.
    for seed in 1..20 {
        let mut probe = warmed(seed);
        let swap = probe.next_u32() % 2 == 0;
        let mut rng = warmed(seed);
        let mut items = [1, 2];
        shuffle(&mut items, &mut rng);
        assert_eq!(items == [2, 1], swap, "seed {seed}");
    }
    // Always a permutation.
    let mut rng = warmed(7);
    let mut items: Vec<u32> = (0..100).collect();
    shuffle(&mut items, &mut rng);
    let mut sorted = items.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, (0..100).collect::<Vec<_>>());
    assert_ne!(items, sorted);
}

#[test]
fn attempts_count_the_guesses_that_narrow_the_words_down() {
    // Five words sharing nothing: one guess rules out one word.
    let words: Vec<Vec<u8>> = [b"AAAA", b"BBBB", b"CCCC", b"DDDD", b"EEEE"]
        .iter()
        .map(|w| w.to_vec())
        .collect();
    assert_eq!(attempts(&words), 5);
    // Every word tells the others apart: at least 4 all the same.
    let words: Vec<Vec<u8>> = [b"ABCD", b"ABCE", b"ABFG", b"AHIJ", b"KLMN"]
        .iter()
        .map(|w| w.to_vec())
        .collect();
    assert_eq!(attempts(&words), 4);
    // Eight words sharing nothing: eight.
    let words: Vec<Vec<u8>> = (b'A'..b'I').map(|c| vec![c; 4]).collect();
    assert_eq!(attempts(&words), 8);
}

#[test]
fn chosen_words_share_letters_but_for_the_last_one_taken() {
    let dictionary = all_words(b"ABC", 4);
    for seed in 1..30 {
        let mut rng = warmed(seed);
        let w = choose_words(&dictionary, 4, 12, &mut rng, &mut ticking()).unwrap();
        assert_eq!(w.words.len(), 12);
        assert!(w.password < 12);
        let mut distinct = w.words.clone();
        distinct.sort();
        distinct.dedup();
        assert_eq!(distinct.len(), 12, "no word twice");
        // Every pair sharing no letter in place involves one word (the
        // last taken, before the final shuffles).
        let zero: Vec<(usize, usize)> = (0..12)
            .flat_map(|i| (i + 1..12).map(move |j| (i, j)))
            .filter(|&(i, j)| likeness(&w.words[i], &w.words[j]) == 0)
            .collect();
        if let Some(&(a, b)) = zero.first() {
            assert!(
                zero.iter().all(|&(i, j)| i == a || j == a)
                    || zero.iter().all(|&(i, j)| i == b || j == b),
                "seed {seed}: {zero:?}"
            );
        }
    }
    assert!(choose_words(&dictionary, 0, 12, &mut warmed(1), &mut ticking()).is_none());
}

#[test]
fn a_short_dictionary_gives_what_it_can_as_time_runs_out() {
    // Three words where twelve are wanted: the tries run out of time and
    // the game carries on with what it has.
    let dictionary = vec![b"ABCD".to_vec(), b"ABCE".to_vec(), b"ABCF".to_vec()];
    let mut rng = warmed(3);
    let w = choose_words(&dictionary, 4, 12, &mut rng, &mut ticking()).unwrap();
    assert_eq!(w.words.len(), 3);
}

#[test]
fn the_screen_is_garbage_with_words_two_apart() {
    let dictionary = all_words(b"ABC", 7);
    for seed in 1..20 {
        let mut rng = warmed(seed);
        let mut words = dictionary.clone();
        shuffle(&mut words, &mut rng);
        words.truncate(20);
        let w = world::hacking::Words { words, password: 0 };
        let file = PasswordFile::build(&w.words, &mut rng);
        assert_eq!(file.chars.len(), FILE_CHARS);
        assert_eq!(file.dropped, 0);
        let mut word_at = vec![false; FILE_CHARS];
        for (word, &at) in w.words.iter().zip(&file.positions) {
            assert!(at + 7 <= 0x197);
            assert_eq!(&file.chars[at..at + 7], &word[..]);
            for flag in &mut word_at[at..at + 7] {
                *flag = true;
            }
        }
        for (i, &c) in file.chars.iter().enumerate() {
            if !word_at[i] {
                assert!(GARBAGE.contains(&c), "seed {seed}: {c}");
            }
        }
        // Two non-letters between words.
        let mut sorted = file.positions.clone();
        sorted.sort_unstable();
        for pair in sorted.windows(2) {
            assert!(pair[1] >= pair[0] + 7 + 2, "seed {seed}: {pair:?}");
        }
    }
    assert_eq!(PasswordFile::address(0xF4F0, 0), "0xF4F0");
    assert_eq!(PasswordFile::address(0xF4F0, 1), "0xF4FC");
    assert_eq!(PasswordFile::address(0xFFFC, 1), "0x0008");
}

/// A game on a screen laid out by hand.
fn game(chars: &[u8], words: &[(&[u8], usize)], password: &[u8], attempts: u32) -> Game {
    let mut screen = vec![b'.'; FILE_CHARS];
    screen[..chars.len()].copy_from_slice(chars);
    for (w, at) in words {
        screen[*at..*at + w.len()].copy_from_slice(w);
    }
    Game {
        length: password.len(),
        words: words.iter().map(|(w, _)| w.to_vec()).collect(),
        positions: words.iter().map(|(_, at)| *at).collect(),
        password: password.to_vec(),
        chars: screen,
        attempts,
        max_attempts: attempts,
        used_reset: false,
        used_brackets: Vec::new(),
        outcome: None,
    }
}

#[test]
fn the_pointer_picks_words_brackets_on_one_line_or_a_character() {
    //            0123456789AB
    let line0 = b"(%%]<..>{A}[";
    let line1 = b"]...........";
    let mut chars = line0.to_vec();
    chars.extend(line1);
    let g = game(&chars, &[(b"WORD", 30)], b"WORD", 4);
    assert_eq!(g.selection(31), Selection::Word(0));
    // `(` closes only with `)`: none on the line.
    assert_eq!(g.selection(0), Selection::Char(0));
    assert_eq!(g.selection(4), Selection::Brackets { start: 4, len: 4 });
    // A letter between.
    assert_eq!(g.selection(8), Selection::Char(8));
    // The closing one on the next line doesn't count.
    assert_eq!(g.selection(11), Selection::Char(11));
    assert_eq!(
        g.text(Selection::Brackets { start: 4, len: 4 }),
        b"<..>".to_vec()
    );
    let mut used = g.clone();
    used.used_brackets.push(4);
    assert_eq!(used.selection(4), Selection::Char(4));
}

#[test]
fn a_wrong_guess_costs_an_attempt_and_says_how_close() {
    let words: &[(&[u8], usize)] = &[(b"SPRING", 0), (b"STRING", 12), (b"THINGS", 24)];
    let mut g = game(b"", words, b"STRING", 3);
    let mut rng = warmed(1);
    let c = g.choose(1, &mut rng, &texts);
    assert_eq!(c.lines, vec![">SPRING", ">Entry denied", ">5/6 correct."]);
    assert_eq!((g.attempts, c.warning, c.outcome), (2, None, None));
    let c = g.choose(25, &mut rng, &texts);
    assert_eq!(c.lines[2], ">0/6 correct.");
    assert_eq!((g.attempts, c.warning), (1, Some(true)));
    // A single character is a guess too.
    let c = g.choose(40, &mut rng, &texts);
    assert_eq!(
        c.lines,
        vec![">.", ">Entry denied", ">Lockout in", "progress."]
    );
    assert_eq!((g.attempts, c.outcome), (0, Some(Outcome::LockedOut)));
    // Nothing more once it's over.
    assert!(g.choose(13, &mut rng, &texts).lines.is_empty());
}

#[test]
fn the_password_opens_it() {
    let words: &[(&[u8], usize)] = &[(b"SPRING", 0), (b"STRING", 12)];
    let mut g = game(b"", words, b"STRING", 4);
    let c = g.choose(17, &mut warmed(1), &texts);
    assert_eq!(
        c.lines,
        vec![
            ">STRING",
            ">Exact match!",
            ">Please wait",
            ">while system",
            ">is accessed."
        ]
    );
    assert_eq!(c.outcome, Some(Outcome::Granted));
    assert_eq!(g.attempts, 4);
}

/// A seed whose first draw of 4 is (or isn't) 0.
fn seed_rolling(zero: bool) -> u32 {
    (1..)
        .find(|&s| (warmed(s).next_u32() % 4 == 0) == zero)
        .unwrap()
}

#[test]
fn brackets_remove_a_dud_or_once_refill_the_attempts() {
    let mut chars = vec![b'.'; 24];
    chars[0..4].copy_from_slice(b"<..>");
    chars[4..8].copy_from_slice(b"[..]");
    let words: &[(&[u8], usize)] = &[(b"SPRING", 12), (b"STRING", 30), (b"THINGS", 50)];

    // A roll other than 0: a dud goes, the bracket is used.
    let mut g = game(&chars, words, b"STRING", 4);
    g.attempts = 2;
    let mut rng = warmed(seed_rolling(false));
    let c = g.choose(0, &mut rng, &texts);
    assert_eq!(c.lines, vec!["><..>", ">Dud removed."]);
    assert_eq!(g.words.len(), 2);
    assert!(g.words.contains(&b"STRING".to_vec()));
    assert_eq!(g.attempts, 2, "no attempt used");
    assert_eq!(g.used_brackets, vec![0]);
    assert_eq!(g.selection(0), Selection::Char(0));
    let gone = if g.words.contains(&b"SPRING".to_vec()) {
        50
    } else {
        12
    };
    assert_eq!(
        &g.chars[gone..gone + 6],
        b"......",
        "the dud's letters are dots"
    );

    // A roll of 0: the attempts come back, once.
    let mut g = game(&chars, words, b"STRING", 4);
    g.attempts = 1;
    let mut rng = warmed(seed_rolling(true));
    let c = g.choose(4, &mut rng, &texts);
    assert_eq!(c.lines, vec![">[..]", ">Allowance", ">replenished."]);
    assert_eq!(
        (g.attempts, g.used_reset, c.warning),
        (4, true, Some(false))
    );
    assert_eq!(g.words.len(), 3);
    // After that, always a dud.
    let c = g.choose(0, &mut warmed(seed_rolling(true)), &texts);
    assert_eq!(c.lines[1], ">Dud removed.");

    // Only the password left: denied, nothing used.
    let mut g = game(&chars, &[(b"STRING", 30)], b"STRING", 4);
    let c = g.choose(0, &mut warmed(1), &texts);
    assert_eq!(c.lines, vec!["><..>", ">Entry denied"]);
    assert!(g.used_brackets.is_empty());
    assert_eq!(g.attempts, 4);
}

#[test]
fn a_whole_game_comes_out_playable() {
    let dictionary = all_words(b"ABCD", 6);
    let mut rng = warmed(99);
    let g = Game::new(&dictionary, 6, 14, &mut rng, &mut ticking()).unwrap();
    assert_eq!(g.words.len(), 14);
    assert!(g.words.contains(&g.password));
    assert!(g.max_attempts >= 4);
    assert_eq!(g.attempts, g.max_attempts);
    for (w, &at) in g.words.iter().zip(&g.positions) {
        assert_eq!(&g.chars[at..at + 6], &w[..]);
        assert_eq!(
            g.selection(at + 5),
            Selection::Word(g.words.iter().position(|x| x == w).unwrap())
        );
    }
    assert_eq!(g.line(1), &g.chars[LINE_CHARS..2 * LINE_CHARS]);
}
