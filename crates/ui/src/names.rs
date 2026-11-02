//! The game's one table of menu names: tile types, operators, traits,
//! `src` selectors and the `&...;` entities the menu files use, each mapped
//! to the number the engine works with. Registered by `009ff970` in
//! FalloutNV.exe (and the system colours by the colour manager, `00718c90`);
//! read value for value from there.

use std::collections::HashMap;

/// What a name that isn't in the table reads as (`0x80000000` read as a
/// float: the tokenizer's "not found").
pub const NOT_FOUND: f32 = -2_147_483_648.0;

/// Tile types.
pub mod kind {
    pub const RECT: i32 = 901;
    pub const IMAGE: i32 = 902;
    pub const TEXT: i32 = 903;
    /// `nif` and `3d`.
    pub const NIF: i32 = 904;
    pub const MENU: i32 = 905;
    pub const HOTRECT: i32 = 906;
    pub const WINDOW: i32 = 907;
    pub const RADIAL: i32 = 908;
    pub const TEMPLATE: i32 = 999;

    /// Whether a number is a tile type that can be created (901 to 908).
    pub fn is_tile(v: f32) -> bool {
        (901.0..=908.0).contains(&v)
    }
}

/// Operators (2000 to 2023), and the two the loader adds for nested ones.
pub mod op {
    pub const COPY: i32 = 2000;
    pub const ADD: i32 = 2001;
    pub const SUB: i32 = 2002;
    pub const MUL: i32 = 2003;
    pub const DIV: i32 = 2004;
    pub const MIN: i32 = 2005;
    pub const MAX: i32 = 2006;
    pub const MOD: i32 = 2007;
    pub const FLOOR: i32 = 2008;
    pub const CEIL: i32 = 2009;
    pub const ABS: i32 = 2010;
    pub const ROUND: i32 = 2011;
    pub const GT: i32 = 2012;
    pub const GTE: i32 = 2013;
    pub const EQ: i32 = 2014;
    pub const NEQ: i32 = 2015;
    pub const LT: i32 = 2016;
    pub const LTE: i32 = 2017;
    pub const AND: i32 = 2018;
    pub const OR: i32 = 2019;
    pub const NOT: i32 = 2020;
    pub const ONLYIF: i32 = 2021;
    pub const ONLYIFNOT: i32 = 2022;
    pub const REF: i32 = 2023;
    /// An operator tag with operators inside it starts a group (the loader
    /// adds this with the operator's number as its constant)...
    pub const GROUP_BEGIN: i32 = 2024;
    /// ...and ends it.
    pub const GROUP_END: i32 = 2025;

    /// Operator numbers, as the tokenizer tests them (2000 to 2025).
    pub fn is_op(id: i32) -> bool {
        (2000..=2025).contains(&id)
    }
}

/// The attributes and the text between tags.
pub mod attr {
    /// Text between an opening and a closing tag.
    pub const VALUE: i32 = 3001;
    pub const NAME: i32 = 3002;
    pub const SRC: i32 = 3003;
    pub const TRAIT: i32 = 3004;
}

/// Traits (4001 to 4124; names starting `_` get numbers from 10000 up).
pub mod t {
    pub const X: i32 = 4001;
    pub const Y: i32 = 4002;
    pub const VISIBLE: i32 = 4003;
    pub const CLASS: i32 = 4004;
    pub const CLIPWINDOW: i32 = 4006;
    pub const STACKINGTYPE: i32 = 4007;
    pub const LOCUS: i32 = 4008;
    pub const ALPHA: i32 = 4009;
    pub const ID: i32 = 4010;
    pub const DISABLEFADE: i32 = 4011;
    pub const LISTINDEX: i32 = 4012;
    pub const DEPTH: i32 = 4013;
    pub const CLIPS: i32 = 4014;
    pub const TARGET: i32 = 4015;
    pub const HEIGHT: i32 = 4016;
    pub const WIDTH: i32 = 4017;
    pub const RED: i32 = 4018;
    pub const GREEN: i32 = 4019;
    pub const BLUE: i32 = 4020;
    pub const TILE: i32 = 4021;
    pub const CHILDCOUNT: i32 = 4022;
    pub const JUSTIFY: i32 = 4023;
    pub const ZOOM: i32 = 4024;
    pub const FONT: i32 = 4025;
    pub const WRAPWIDTH: i32 = 4026;
    pub const WRAPLIMIT: i32 = 4027;
    pub const WRAPLINES: i32 = 4028;
    pub const PAGENUM: i32 = 4029;
    pub const ISHTML: i32 = 4030;
    pub const CROPY: i32 = 4031;
    pub const CROPX: i32 = 4032;
    pub const MENUFADE: i32 = 4033;
    pub const EXPLOREFADE: i32 = 4034;
    pub const MOUSEOVER: i32 = 4035;
    pub const STRING: i32 = 4036;
    pub const SHIFTCLICKED: i32 = 4037;
    pub const CLICKED: i32 = 4039;
    pub const CLICKSOUND: i32 = 4043;
    pub const FILENAME: i32 = 4044;
    pub const FILEWIDTH: i32 = 4045;
    pub const FILEHEIGHT: i32 = 4046;
    pub const REPEATVERTICAL: i32 = 4047;
    pub const REPEATHORIZONTAL: i32 = 4048;
    pub const ANIMATION: i32 = 4050;
    pub const LINECOUNT: i32 = 4052;
    pub const PAGECOUNT: i32 = 4053;
    pub const SYSTEMCOLOR: i32 = 4084;
    pub const BRIGHTNESS: i32 = 4085;
    pub const LINEGAP: i32 = 4087;
    pub const RESOLUTIONCONVERTER: i32 = 4088;
    pub const TEXATLAS: i32 = 4089;
    pub const ROTATEANGLE: i32 = 4090;
    pub const USER0: i32 = 4100;
    /// The tile's menu class number (on menus; trait 6001, set by code).
    pub const MENU_CLASS_INTERNAL: i32 = 6001;

    /// Trait numbers the tokenizer treats as traits: 4001 to 4124, or a
    /// custom one (above 9999).
    pub fn is_trait(id: i32) -> bool {
        (4001..=4124).contains(&id) || id > 9999
    }
}

/// `src` selectors.
pub mod sel {
    pub const PARENT: i32 = 5001;
    pub const ME: i32 = 5002;
    pub const SIBLING: i32 = 5004;
    pub const CHILD: i32 = 5005;
    pub const SCREEN: i32 = 5006;
    pub const GLOBALS: i32 = 5007;
    pub const IO: i32 = 5008;
    pub const GRANDPARENT: i32 = 5009;
}

/// System colours (`00718c90`): the HUD's (`uHUDColor`), the alternative
/// HUD red, terminals' green, the Pip-Boy's (`uPipboyColor`), the main
/// menu's and the system's.
pub mod color {
    pub const HUD_MAIN: i32 = 1;
    pub const HUD_ALT: i32 = 2;
    pub const TERMINAL: i32 = 3;
    pub const PIPBOY: i32 = 4;
    pub const MAIN_MENU: i32 = 5;
    pub const SYSTEM: i32 = 6;
}

/// The first number given to a trait named with a leading `_` (the game
/// counts on from the highest number registered above 9999; the start is a
/// guess, only the order matters).
const FIRST_CUSTOM: i32 = 10_000;

/// The table: names (case ignored) to numbers.
#[derive(Debug, Clone)]
pub struct Names {
    by_name: HashMap<String, i32>,
    /// Custom traits' names, for printing and for indexed lookups.
    custom: HashMap<i32, String>,
    next_custom: i32,
}

impl Names {
    /// The table as the game registers it. `widescreen` is the engine's
    /// `&widescreen;` (whether the display is wider than 4:3).
    pub fn new(widescreen: bool) -> Names {
        let mut names = Names {
            by_name: HashMap::new(),
            custom: HashMap::new(),
            next_custom: FIRST_CUSTOM,
        };
        let entries: &[(&str, i32)] = &[
            ("&generic;", -1),
            ("&false;", 0),
            ("&true;", 1),
            ("&left;", 1),
            ("&center;", 2),
            ("&right;", 4),
            ("&up;", 5),
            ("&down;", 6),
            ("&scale;", -1),
            ("&click_past;", 101),
            ("&no_click_past;", 102),
            ("&does_not_stack;", 6008),
            ("&mixed_menu;", 103),
            // On PC: 0 (`[011d8a84]` is the console build's flag).
            ("&xenon;", 0),
            ("&console;", 0),
            ("&xbox;", 0),
            ("&highdef;", 1),
            ("&widescreen;", i32::from(widescreen)),
            ("&glow_branch;", 110),
            ("&noglow_branch;", 111),
            ("&pipboymenu;", 112),
            ("&uselocalcolor;", -1),
            ("&nosystemcolor;", 0),
            ("&nosound;", -1),
            ("&default_accept;", 9),
            ("&xbuttona;", 9),
            ("&xbuttonb;", 10),
            ("&xbuttonx;", 11),
            ("&xbuttony;", 12),
            ("&xbuttonlt;", 13),
            ("&xbuttonrt;", 14),
            ("&xbuttonlb;", 15),
            ("&xbuttonrb;", 16),
            ("&xbuttonls;", 17),
            ("&xbuttonrs;", 18),
            ("&xbuttonl;", 17),
            ("&xbuttonr;", 18),
            ("image", kind::IMAGE),
            ("menu", kind::MENU),
            ("3d", kind::NIF),
            ("nif", kind::NIF),
            ("rect", kind::RECT),
            ("hotrect", kind::HOTRECT),
            ("template", kind::TEMPLATE),
            ("text", kind::TEXT),
            ("window", kind::WINDOW),
            ("radial", kind::RADIAL),
            ("&AudioMenu;", 1017),
            ("&BookMenu;", 1026),
            ("&CharGenMenu;", 1048),
            ("&ContainerMenu;", 1008),
            ("&DialogMenu;", 1009),
            ("&GameplayMenu;", 1020),
            ("&HUDMainMenu;", 1004),
            ("&InventoryMenu;", 1002),
            ("&LoadingMenu;", 1007),
            ("&LockPickMenu;", 1014),
            ("&MapMenu;", 1023),
            ("&MessageMenu;", 1001),
            ("&TutorialMenu;", 1059),
            ("&TextEditMenu;", 1051),
            ("&StartMenu;", 1013),
            ("&QuantityMenu;", 1016),
            ("&RaceSexMenu;", 1036),
            ("&SleepWaitMenu;", 1012),
            ("&StatsMenu;", 1003),
            ("&VideoMenu;", 1018),
            ("&LevelUpMenu;", 1027),
            ("&RepairMenu;", 1035),
            ("&ItemModMenu;", 1061),
            ("&RepairServicesMenu;", 1058),
            ("&CreditsMenu;", 1047),
            ("&BarterMenu;", 1053),
            ("&SurgeryMenu;", 1054),
            ("&HackingMenu;", 1055),
            ("&ComputersMenu;", 1057),
            ("&VATSMenu;", 1056),
            ("&SPECIALBookMenu;", 1060),
            ("&CompanionWheelMenu;", 1075),
            ("&LoveTesterMenu;", 1074),
            ("&TraitSelectMenu;", 1076),
            ("&SlotMachineMenu;", 1080),
            ("&BlackJackMenu;", 1081),
            ("&RouletteMenu;", 1082),
            ("&RecipeMenu;", 1077),
            ("&CaravanMenu;", 1083),
            ("&TraitMenu;", 1084),
            ("abs", op::ABS),
            ("add", op::ADD),
            ("and", op::AND),
            ("ceil", op::CEIL),
            ("copy", op::COPY),
            ("div", op::DIV),
            ("eq", op::EQ),
            ("floor", op::FLOOR),
            ("gt", op::GT),
            ("gte", op::GTE),
            ("lt", op::LT),
            ("lte", op::LTE),
            ("max", op::MAX),
            ("min", op::MIN),
            ("mod", op::MOD),
            ("mul", op::MUL),
            ("mult", op::MUL),
            ("neq", op::NEQ),
            ("not", op::NOT),
            ("onlyif", op::ONLYIF),
            ("onlyifnot", op::ONLYIFNOT),
            ("or", op::OR),
            ("ref", op::REF),
            ("round", op::ROUND),
            ("sub", op::SUB),
            ("name", attr::NAME),
            ("src", attr::SRC),
            ("trait", attr::TRAIT),
            ("value", attr::VALUE),
            ("alpha", t::ALPHA),
            ("animation", t::ANIMATION),
            ("blue", t::BLUE),
            ("brightness", t::BRIGHTNESS),
            ("child_count", t::CHILDCOUNT),
            ("childcount", t::CHILDCOUNT),
            ("class", t::CLASS),
            ("clicked", t::CLICKED),
            ("clicksound", t::CLICKSOUND),
            ("mouseoversound", 4072),
            ("disablefade", t::DISABLEFADE),
            ("cropoffsetx", t::CROPX),
            ("cropoffsety", t::CROPY),
            ("cropx", t::CROPX),
            ("cropy", t::CROPY),
            ("clips", t::CLIPS),
            ("clipwindow", t::CLIPWINDOW),
            ("depth", t::DEPTH),
            ("draggable", 4073),
            ("dragdeltax", 4078),
            ("dragdeltay", 4079),
            ("dragstartx", 4074),
            ("dragstarty", 4075),
            ("dragoffsetx", 4076),
            ("dragoffsety", 4077),
            ("dragx", 4080),
            ("dragy", 4081),
            ("explorefade", t::EXPLOREFADE),
            ("fileheight", t::FILEHEIGHT),
            ("filename", t::FILENAME),
            ("filewidth", t::FILEWIDTH),
            ("font", t::FONT),
            ("green", t::GREEN),
            ("height", t::HEIGHT),
            ("id", t::ID),
            ("ishtml", t::ISHTML),
            ("justify", t::JUSTIFY),
            ("linecount", t::LINECOUNT),
            ("linegap", t::LINEGAP),
            ("listindex", t::LISTINDEX),
            ("locus", t::LOCUS),
            ("menufade", t::MENUFADE),
            ("mouseover", t::MOUSEOVER),
            ("pagecount", t::PAGECOUNT),
            ("pagenum", t::PAGENUM),
            ("red", t::RED),
            ("repeatvertical", t::REPEATVERTICAL),
            ("repeathorizontal", t::REPEATHORIZONTAL),
            ("rotateangle", t::ROTATEANGLE),
            ("rotateaxisx", 4091),
            ("rotateaxisy", 4092),
            ("shiftclicked", t::SHIFTCLICKED),
            ("stackingtype", t::STACKINGTYPE),
            ("string", t::STRING),
            ("systemcolor", t::SYSTEMCOLOR),
            ("target", t::TARGET),
            ("texatlas", t::TEXATLAS),
            ("tile", t::TILE),
            ("visible", t::VISIBLE),
            ("wheelable", 4082),
            ("wheelmoved", 4083),
            ("width", t::WIDTH),
            ("wraplimit", t::WRAPLIMIT),
            ("wraplines", t::WRAPLINES),
            ("wrapwidth", t::WRAPWIDTH),
            ("xbuttona", 4061),
            ("xbuttonb", 4062),
            ("xbuttonx", 4063),
            ("xbuttony", 4064),
            ("xbuttonlt", 4065),
            ("xbuttonrt", 4066),
            ("xbuttonlb", 4067),
            ("xbuttonrb", 4068),
            ("xbuttonstart", 4071),
            ("xdefault", 4054),
            ("xup", 4055),
            ("xdown", 4056),
            ("xleft", 4057),
            ("xright", 4058),
            ("x", t::X),
            ("y", t::Y),
            ("zoom", t::ZOOM),
            ("resolutionconverter", t::RESOLUTIONCONVERTER),
            ("child", sel::CHILD),
            ("me", sel::ME),
            ("io", sel::IO),
            ("parent", sel::PARENT),
            ("screen", sel::SCREEN),
            ("sibling", sel::SIBLING),
            ("grandparent", sel::GRANDPARENT),
            ("globals", sel::GLOBALS),
            // The system colours, registered as "&%s;" (`00719160`).
            ("&HUDMain;", color::HUD_MAIN),
            ("&HUDAlt;", color::HUD_ALT),
            ("&Terminal;", color::TERMINAL),
            ("&Pipboy;", color::PIPBOY),
            ("&MainMenu;", color::MAIN_MENU),
            ("&System;", color::SYSTEM),
        ];
        for &(name, id) in entries {
            names.by_name.insert(name.to_ascii_lowercase(), id);
        }
        // user0..user16 (4100..4116).
        for i in 0..=16 {
            names.by_name.insert(format!("user{i}"), t::USER0 + i);
        }
        names
    }

    /// A name's number, case ignored (the files write `&hudmain;` for the
    /// registered `&HUDMain;`).
    pub fn lookup(&self, name: &str) -> Option<i32> {
        self.by_name.get(&name.to_ascii_lowercase()).copied()
    }

    /// A name's number, giving a name that starts with `_` (or `&_`) a new
    /// number of its own when it has none (`00a00940`); `None` for other
    /// unknown names.
    pub fn lookup_or_add(&mut self, name: &str) -> Option<i32> {
        if let Some(id) = self.lookup(name) {
            return Some(id);
        }
        if !(name.starts_with('_') || name.starts_with("&_")) {
            return None;
        }
        let id = self.next_custom;
        self.next_custom += 1;
        self.by_name.insert(name.to_ascii_lowercase(), id);
        self.custom.insert(id, name.to_string());
        Some(id)
    }

    /// A custom trait's name (as first written), if the number is one.
    pub fn custom_name(&self, id: i32) -> Option<&str> {
        self.custom.get(&id).map(String::as_str)
    }

    /// Whether a custom trait's name ends with `_`: reading it through a
    /// link reads `<name><n>` instead, `n` the reading trait's value
    /// rounded (`00a01a20` marks such names, `00a0a0b0` formats the
    /// name). `_filename_` with 3 reads `_filename_3`.
    pub fn is_indexed(&self, id: i32) -> bool {
        self.custom_name(id).is_some_and(|n| n.ends_with('_'))
    }

    /// Something printable for a number: the name registered for it.
    pub fn describe(&self, id: i32) -> String {
        if let Some(name) = self.custom_name(id) {
            return name.to_string();
        }
        // The plain names first (traits before entities sharing a number).
        let mut found: Vec<&String> = self
            .by_name
            .iter()
            .filter(|(_, &v)| v == id)
            .map(|(k, _)| k)
            .collect();
        found.sort_by_key(|k| (k.starts_with('&'), k.len(), (*k).clone()));
        found
            .first()
            .map_or_else(|| id.to_string(), |s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_games_numbers_and_case_ignored() {
        let names = Names::new(true);
        assert_eq!(names.lookup("width"), Some(4017));
        assert_eq!(names.lookup("WIDTH"), Some(4017));
        assert_eq!(names.lookup("&hudmain;"), Some(1));
        assert_eq!(names.lookup("&right;"), Some(4));
        assert_eq!(names.lookup("&widescreen;"), Some(1));
        assert_eq!(Names::new(false).lookup("&widescreen;"), Some(0));
        assert_eq!(names.lookup("user16"), Some(4116));
        assert_eq!(names.lookup("onlyif"), Some(op::ONLYIF));
        assert_eq!(names.lookup("&noglow_branch;"), Some(111));
        assert_eq!(names.lookup("bogus"), None);
    }

    #[test]
    fn underscore_names_get_numbers_of_their_own() {
        let mut names = Names::new(true);
        let a = names.lookup_or_add("_TotalWidth").unwrap();
        assert!(a > 9999);
        assert_eq!(names.lookup_or_add("_totalwidth"), Some(a));
        let b = names.lookup_or_add("_filename_").unwrap();
        assert_ne!(a, b);
        assert!(names.is_indexed(b));
        assert!(!names.is_indexed(a));
        assert_eq!(names.lookup_or_add("plain"), None);
        assert_eq!(names.describe(a), "_TotalWidth");
        assert_eq!(names.describe(4017), "width");
    }
}
