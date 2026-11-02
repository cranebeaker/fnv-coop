//! The player's third-person look (`world::actor::player_look`) from a
//! plugin built from scratch: the player's record gives the skeleton, the
//! game state the sex and the clothes worn.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::{group, record, sub, zstr};

const SHIRT: u32 = 0x900;
const DRESS: u32 = 0x901;

fn plugin() -> Vec<u8> {
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    // Clothes: an upper-body shirt (male and female models) and a dress
    // over the upper body and the hands' slot 0x8.
    let clothes = |id: u32, name: &str, slots: u32, male: &str, female: &str| {
        let mut d = sub(b"EDID", &zstr(name));
        let mut bmdt = slots.to_le_bytes().to_vec();
        bmdt.extend([0; 4]);
        d.extend(sub(b"BMDT", &bmdt));
        d.extend(sub(b"MODL", &zstr(male)));
        d.extend(sub(b"MOD3", &zstr(female)));
        record(b"ARMO", id, &d)
    };
    let mut armour = clothes(
        SHIRT,
        "TestShirt",
        0x04,
        "Armor\\Shirt\\M\\Shirt.nif",
        "Armor\\Shirt\\F\\Shirt.nif",
    );
    armour.extend(clothes(
        DRESS,
        "TestDress",
        0x04 | 0x08,
        "Armor\\Dress\\M\\Dress.nif",
        "Armor\\Dress\\F\\Dress.nif",
    ));
    plugin.extend(group(*b"ARMO", 0, &armour));
    // The player: the male skeleton, carrying the dress in the record's
    // inventory (which the player's look must not use).
    let mut player = sub(b"EDID", &zstr("Player"));
    player.extend(sub(b"MODL", &zstr("Characters\\_Male\\Skeleton.NIF")));
    player.extend(sub(b"ACBS", &[0; 24]));
    let mut cnto = DRESS.to_le_bytes().to_vec();
    cnto.extend(1u32.to_le_bytes());
    player.extend(sub(b"CNTO", &cnto));
    plugin.extend(group(*b"NPC_", 0, &record(b"NPC_", 0x7, &player)));
    plugin
}

#[test]
fn the_player_wears_what_the_game_says_not_the_record() {
    let data = testdata::quests("player-look");
    data.write("FalloutNV.esm", &plugin());
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let models = |look: &world::ActorLook| -> Vec<String> {
        look.parts.iter().map(|p| p.model.clone()).collect()
    };
    // The record's own look carries its dress.
    let npc = world::actor_look(&order, FormId(0x7)).unwrap();
    assert_eq!(models(&npc), ["Armor\\Dress\\M\\Dress.nif"]);
    // As the player: the shirt worn, male.
    let male = world::actor::player_look(&order, false, &[FormId(SHIRT)], None).unwrap();
    assert_eq!(male.skeleton, "Characters\\_Male\\Skeleton.NIF");
    assert!(!male.female);
    assert_eq!(models(&male), ["Armor\\Shirt\\M\\Shirt.nif"]);
    // Made a woman: the female model; nothing worn, nothing drawn (no race
    // in this plugin, so no body).
    let female = world::actor::player_look(&order, true, &[FormId(SHIRT)], None).unwrap();
    assert!(female.female);
    assert_eq!(models(&female), ["Armor\\Shirt\\F\\Shirt.nif"]);
    let bare = world::actor::player_look(&order, false, &[], None).unwrap();
    assert!(bare.parts.is_empty());
}
