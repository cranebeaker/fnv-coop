//! The interactive menus. Thin glue over the pure modules; not unit tested.

use crate::cli::{self, LaunchInput};
use crate::command;
use crate::install::{self, Candidate, RegExe};
use crate::joincode::{self, JoinInput};
use crate::party::{self, Role, SaveEntry};
use crate::portcheck;
use crate::redact::redact;
use crate::run;
use crate::settings::{clean_name, Settings, Transport};
use crate::steam;
use crate::ui::Ui;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct App {
    pub ui: Ui,
    pub home: PathBuf,
    pub exe_dir: PathBuf,
    pub viewer: PathBuf,
    pub settings: Settings,
    pub user: Option<String>,
}

fn random8() -> [u8; 8] {
    let t = coop_net::random_token();
    let mut r = [0u8; 8];
    r.copy_from_slice(&t[..8]);
    r
}

impl App {
    fn settings_path(&self) -> PathBuf {
        self.home.join("settings.json")
    }
    fn saves_root(&self) -> PathBuf {
        self.home.join("saves")
    }
    fn save_settings(&self) {
        if let Err(e) = self.settings.save(&self.settings_path()) {
            self.ui.fail(&format!("Could not save settings: {e}"));
        }
    }

    pub fn main_menu(&mut self) {
        loop {
            let ready = !self.settings.data_path.is_empty();
            println!(
                "\n{}  {}",
                self.ui.bold("nv-rs co-op launcher"),
                self.ui.dim(crate::VERSION)
            );
            if ready {
                println!("  Game data: {}", self.shown(&self.settings.data_path));
            } else {
                self.ui.note(
                    "First time here? Choose 1 (Setup) to find your Fallout: New Vegas install.",
                );
            }
            let items = [
                "Setup: find the game",
                "Host a game",
                "Join a game",
                "Party saves",
                "Port check",
                "Diagnostics",
                "Settings",
            ]
            .map(String::from);
            // Main menu: 0 quits.
            self.ui.title("Main menu");
            for (i, it) in items.iter().enumerate() {
                println!("  {}  {it}", self.ui.accent(&format!("{}", i + 1)));
            }
            println!("  {}  Quit", self.ui.dim("0"));
            let Some(a) = self.ui.ask(">") else { return };
            match a.trim() {
                "0" | "q" | "quit" => return,
                "1" => self.setup(),
                "2" => self.host(None),
                "3" => self.join(),
                "4" => self.saves(),
                "5" => self.port_menu(),
                "6" => self.diagnostics(),
                "7" => self.settings_menu(),
                _ => self.ui.fail("Type one of the numbers above."),
            }
        }
    }

    /// Paths for display: the Windows user name hidden.
    fn shown(&self, s: &str) -> String {
        redact(s, self.user.as_deref())
    }

    // ---- Setup ----

    fn setup(&mut self) {
        self.ui.title("Setup");
        self.ui.line(
            "Looking for Fallout: New Vegas (read only; the game folder is never changed)...",
        );
        let env: HashMap<String, String> = std::env::vars().collect();
        let drives: Vec<PathBuf> = ('C'..='Z')
            .map(|c| PathBuf::from(format!("{c}:\\")))
            .filter(|p| p.exists())
            .collect();
        let found: Vec<Candidate> = install::candidates(&RegExe, &env, &drives);
        let mut items: Vec<String> = Vec::new();
        for c in &found {
            let r = install::check(&c.game_dir);
            let tag = match r.version {
                Some(v) if r.ok() => format!("version {}", install::version_text(v)),
                _ => "has problems".to_string(),
            };
            items.push(format!(
                "{} ({}, {tag})",
                self.shown(&c.game_dir.to_string_lossy()),
                c.source
            ));
        }
        items.push("Type a path myself".into());
        let manual = items.len() - 1;
        if found.is_empty() {
            self.ui.note("No install was found automatically.");
        }
        let Some(pick) = self.ui.menu("Which install?", &items) else {
            return;
        };
        let game_dir = if pick == manual {
            let Some(t) = self.ui.ask("Path to the game folder (the one with FalloutNV.exe), its Data folder, or the exe:") else { return };
            install::normalize_game_dir(&PathBuf::from(t.trim().trim_matches('"')))
        } else {
            found[pick].game_dir.clone()
        };
        let r = install::check(&game_dir);
        for p in &r.problems {
            self.ui.fail(p);
        }
        if let Some(v) = r.version {
            if r.ok() {
                self.ui.ok(&format!(
                    "FalloutNV.exe {} and a Data folder with FalloutNV.esm.",
                    install::version_text(v)
                ));
            }
        }
        if !r.has_data || !r.has_esm {
            self.ui
                .line("Nothing was saved. Pick a folder that holds Data\\FalloutNV.esm.");
            return;
        }
        if !r.ok()
            && !self
                .ui
                .confirm("The game version could not be confirmed. Save this path anyway?")
        {
            return;
        }
        self.settings.data_path = r.data_dir.to_string_lossy().into_owned();
        if let Some(n) = self
            .ui
            .ask_default("Your player name", &self.settings.player_name)
        {
            self.settings.player_name = clean_name(&n);
        }
        self.save_settings();
        self.ui.ok("Saved. You can host or join now.");
    }

    // ---- Launching ----

    /// Returns the Data folder when setup is done and the folder still exists.
    fn require_setup(&mut self) -> bool {
        let d = PathBuf::from(&self.settings.data_path);
        if self.settings.data_path.is_empty() || !d.join("FalloutNV.esm").is_file() {
            self.ui
                .fail("The game's Data folder is not set up (or has moved). Run Setup first.");
            return false;
        }
        true
    }

    fn transport_steam(&self) -> bool {
        if self.settings.transport != Transport::Steam {
            return false;
        }
        let dirs = [
            self.exe_dir.clone(),
            self.viewer
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default(),
        ];
        let dll = steam::find_dll(&dirs);
        let help = if dll.is_some() {
            steam::viewer_help(&self.viewer)
        } else {
            None
        };
        match steam::status(dll.as_deref(), help.as_deref()) {
            steam::Status::Available => true,
            steam::Status::Unavailable(why) => {
                self.ui
                    .note(&format!("Steam friends are not available: {why}"));
                self.ui.note("Using UDP direct instead.");
                false
            }
        }
    }

    /// Starts the viewer for `li` and waits for it. Returns the launcher's exit code: the
    /// viewer's own, or 1 when it could not be started.
    fn launch(&mut self, li: LaunchInput) -> i32 {
        let l = match cli::make_plan(&li) {
            Ok(l) => l,
            Err(e) => {
                self.ui.fail(&e);
                return 2;
            }
        };
        let now = run::now_secs();
        if let Err(e) =
            party::ensure_player_dir(&self.saves_root(), &l.campaign, l.role, &l.player, now)
        {
            self.ui
                .fail(&format!("Could not prepare the save folder: {e}"));
            return 1;
        }
        let mut spec = command::build(&l.plan);
        party::extend_spec(&mut spec, l.role, li.new_game, &self.home, random8());
        let log = self
            .home
            .join("logs")
            .join(format!("launcher-{}.log", li.stamp));
        self.ui.line(&format!(
            "Viewer log: {}",
            self.shown(&log.to_string_lossy())
        ));
        self.ui.line(
            "Starting the game. This window shows its co-op lines and waits until you quit it.",
        );
        let user = self.user.clone();
        let mut shown = |line: &str| {
            if run::interesting(line) {
                println!("  | {}", redact(line, user.as_deref()));
            }
        };
        match run::run_viewer(&spec, &log, self.user.as_deref(), &mut shown) {
            Ok(code) => {
                let n = party::stamp_saves(
                    &l.plan.cwd,
                    &l.campaign,
                    l.role,
                    &l.player,
                    run::now_secs(),
                );
                match code {
                    Some(0) => self.ui.ok("The game closed normally."),
                    Some(c) => self.ui.fail(&format!(
                        "The game exited with code {c}. Use Diagnostics to collect the log."
                    )),
                    None => self
                        .ui
                        .fail("The game was ended abnormally. Use Diagnostics to collect the log."),
                }
                if n > 0 {
                    self.ui.line(&format!(
                        "Party save sidecar written for {n} save(s) of campaign {}.",
                        l.campaign
                    ));
                }
                // the host may have turned this guest away: say why in plain words
                if let Some(why) = std::fs::read_to_string(&l.plan.coop_log)
                    .ok()
                    .and_then(|t| crate::extras::refusal_from_log(&t))
                {
                    self.ui.fail(&why);
                }
                code.unwrap_or(1)
            }
            Err(e) => {
                self.ui.fail(&e);
                1
            }
        }
    }

    /// `--start host|join`: the same launch as the menus, from the command line. Prints the join
    /// code for a host. Returns the exit code.
    pub fn start_from_cli(&mut self, c: &cli::Cli) -> i32 {
        let mode = c.start.clone().unwrap_or_default();
        if let Some(d) = &c.data {
            self.settings.data_path = d.clone();
        }
        if let Some(n) = &c.name {
            self.settings.player_name = clean_name(n);
        }
        if let Some(p) = c.port {
            self.settings.port = p;
        }
        if c.uncap {
            self.settings.uncap = true;
        }
        if !self.require_setup() {
            return 2;
        }
        let token = coop_net::random_token();
        let stamp = run::stamp(run::now_secs());
        let steam = self.transport_steam();
        let (launch, mut li) = match cli::plan_from_cli(
            c,
            &mode,
            self.settings.clone(),
            &self.home,
            &self.exe_dir,
            random8(),
            token,
            stamp,
            false,
        ) {
            Ok(x) => x,
            Err(e) => {
                self.ui.fail(&e);
                return 2;
            }
        };
        li.steam = li.steam || (mode == "host" && steam);
        if mode == "host" {
            let addr = c
                .address
                .clone()
                .or_else(|| Some(self.settings.host_address.clone()).filter(|a| !a.is_empty()))
                .or_else(|| portcheck::local_address().map(|i| i.to_string()))
                .unwrap_or_default();
            match joincode::host_code(&addr, self.settings.port, li.token) {
                Ok(code) => {
                    println!("  Join code:   {}", self.ui.bold(&self.ui.good(&code)));
                    println!("  Campaign id: {}", launch.campaign);
                }
                Err(e) => self.ui.note(&format!("No join code: {e}")),
            }
            if li.new_game {
                self.ui.line("New game: the whole opening runs (Doc Mitchell, name, Vit-o-Matic, tags, traits). Guests make their own characters when they join.");
            }
        }
        self.launch(li)
    }

    fn base_input(&self, host: bool) -> LaunchInput {
        LaunchInput {
            settings: self.settings.clone(),
            home: self.home.clone(),
            viewer: self.viewer.clone(),
            host,
            new_game: false,
            join: None,
            campaign: None,
            save: None,
            random: random8(),
            steam: false,
            token: coop_net::random_token(),
            stamp: run::stamp(run::now_secs()),
            extras: Default::default(),
        }
    }

    /// `preset`: a campaign to continue (from the saves menu).
    fn host(&mut self, preset: Option<SaveEntry>) {
        if !self.require_setup() {
            return;
        }
        self.ui.title("Host a game");
        let (new_game, campaign, save) = match preset {
            Some(s) => (false, Some(s.campaign), Some(s.path)),
            None => {
                let hosts = party::host_saves(&self.saves_root());
                let mut items = vec!["New game (the opening, from the start)".to_string()];
                for s in &hosts {
                    let more = party::list_party_saves(&self.saves_root())
                        .into_iter()
                        .find(|p| p.campaign == s.campaign && p.usable())
                        .map(|p| format!(", {}", party::describe_party(&p)))
                        .unwrap_or_default();
                    items.push(format!(
                        "Continue {} ({}, {}{})",
                        s.campaign,
                        s.player,
                        party::format_time(s.modified),
                        more
                    ));
                }
                let Some(i) = self.ui.menu("Which campaign?", &items) else {
                    return;
                };
                if i == 0 {
                    (true, None, None)
                } else {
                    (
                        false,
                        Some(hosts[i - 1].campaign.clone()),
                        Some(hosts[i - 1].path.clone()),
                    )
                }
            }
        };
        let steam_on = self.transport_steam();
        let port = self.settings.port;
        let b = portcheck::bind_test(port);
        if !b.ok {
            self.ui.fail(&b.detail);
            if !self.ui.confirm("Start anyway?") {
                return;
            }
        }
        let local = portcheck::local_address();
        let default_ip = if !self.settings.host_address.is_empty() {
            self.settings.host_address.clone()
        } else {
            local.map(|i| i.to_string()).unwrap_or_default()
        };
        self.ui.line("Join code address: your LAN IP for friends on your network, your public IP for the internet.");
        let Some(addr) = self
            .ui
            .ask_default("Address to put in the join code", &default_ip)
        else {
            return;
        };
        let mut li = self.base_input(true);
        li.new_game = new_game;
        li.campaign = campaign;
        li.save = save.clone();
        li.steam = steam_on;
        let l = match cli::make_plan(&li) {
            Ok(l) => l,
            Err(e) => return self.ui.fail(&e),
        };
        match joincode::host_code(&addr, port, li.token) {
            Ok(code) => {
                if addr != self.settings.host_address {
                    self.settings.host_address = addr.trim().to_string();
                    self.save_settings();
                }
                println!();
                if steam_on {
                    println!("  Steam:       the game makes a friends-only lobby. Its join code (NVRS-...) is printed in the game's");
                    println!("               console and logged as coop_join_code; you can also invite friends from Steam.");
                    println!("               If Steam cannot start, the game hosts on UDP instead (logged as transport_fallback) and this code works:");
                }
                println!("  Join code:   {}", self.ui.bold(&self.ui.good(&code)));
                if let Ok(ip) = addr.trim().parse::<std::net::IpAddr>() {
                    println!("  Or address:  {ip}:{port}");
                    if portcheck::scope(ip) == portcheck::Scope::Lan {
                        self.ui.note("That is a private address: only people on your network (or VPN) can reach it.");
                    }
                }
                println!("  Campaign id: {}  (guests can enter it to keep their saves with this campaign)", l.campaign);
                println!("  Send the join code to your friends, then start. They join while you are in the game.");
            }
            Err(e) => self.ui.note(&format!("No join code: {e}")),
        }
        match (&save, new_game) {
            (Some(s), false) => self.ui.note(&format!("Continuing from {}.", self.shown(&s.to_string_lossy()))),
            (None, false) => match party::list_party_saves(&self.saves_root()).into_iter().find(|p| Some(&p.campaign) == li.campaign.as_ref() && p.usable()) {
                Some(p) => self.ui.note(&format!("Continuing from the party save: {}", party::describe_party(&p))),
                None => self.ui.note("Continuing: once the game is up, press F9 to load this campaign's quicksave."),
            },
            _ => self.ui.note("New game: the whole opening runs for you. Guests make their own characters when they join."),
        }
        if !self.ui.confirm("Start the game now?") {
            return;
        }
        self.launch(li);
    }

    fn join(&mut self) {
        if !self.require_setup() {
            return;
        }
        self.ui.title("Join a game");
        let join = loop {
            let Some(t) = self.ui.ask("Join code (NVRS-...), or 'friend' to join a Steam friend who is hosting, empty to go back:") else { return };
            if t.is_empty() {
                return;
            }
            if t.trim().eq_ignore_ascii_case("friend") {
                break JoinInput::SteamFriend;
            }
            match joincode::parse_join(&t) {
                Ok(JoinInput::Addr(_)) => self.ui.fail("A plain address has no session token. Ask the host for their join code (NVRS-...)."),
                // a short code: the join service (NVRS_JOIN_SERVICE) says which NVRS- code it is
                Ok(JoinInput::Short(c)) => match coop_net::joinsvc::JoinService::from_env() {
                    Some(svc) => match joincode::resolve_short(&c, &svc) {
                        Ok(j) => break j,
                        Err(e) => self.ui.fail(&e),
                    },
                    None => self.ui.fail(&format!(
                        "{c} is a short join code, and no join service is set ({}). Ask the host for their NVRS- code.",
                        coop_net::joinsvc::SERVICE_ENV
                    )),
                },
                Ok(j) => break j,
                Err(e) => self.ui.fail(&e),
            }
        };
        let wants_steam = matches!(
            join,
            JoinInput::SteamFriend
                | JoinInput::Code {
                    target: coop_net::JoinTarget::SteamLobby(_),
                    ..
                }
        );
        let steam_ok = wants_steam
            && {
                let dirs = [
                    self.exe_dir.clone(),
                    self.viewer
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_default(),
                ];
                let dll = steam::find_dll(&dirs);
                let help = if dll.is_some() {
                    steam::viewer_help(&self.viewer)
                } else {
                    None
                };
                match steam::status(dll.as_deref(), help.as_deref()) {
                    steam::Status::Available => true,
                    steam::Status::Unavailable(why) => {
                        self.ui.fail(&format!(
                            "That join needs Steam, which is not available: {why}"
                        ));
                        self.ui.note("Start Steam and put steam_api64.dll next to the launcher, or ask the host for a UDP join code. Start this launcher before accepting a Steam invite.");
                        return;
                    }
                }
            };
        let campaign = loop {
            let Some(c) = self
                .ui
                .ask("Campaign id from the host (optional, Enter to skip):")
            else {
                return;
            };
            if c.is_empty() {
                break None;
            }
            match party::parse_campaign_id(&c) {
                Some(id) => break Some(id),
                None => self
                    .ui
                    .fail("A campaign id is 16 letters and digits (0-9, a-f)."),
            }
        };
        let mut li = self.base_input(false);
        li.join = Some(join);
        li.steam = steam_ok;
        let player = clean_name(&self.settings.player_name);
        li.save = campaign
            .as_deref()
            .and_then(|c| party::find_save(&self.saves_root(), c, Role::Guest, &player));
        if li.save.is_some() {
            self.ui.note(
                "Your saved character from that campaign is used; the host's world is not touched.",
            );
        } else {
            self.ui.note("You make your own character with the same menus when you arrive (the host does not wait).");
        }
        li.campaign = campaign;
        self.launch(li);
    }

    // ---- Party saves ----

    fn saves(&mut self) {
        loop {
            let all = party::list_saves(&self.saves_root());
            self.ui.title("Party saves");
            if all.is_empty() {
                self.ui.line(
                    "No saves yet. Saves appear here after a session (press F5 in game to save).",
                );
                return;
            }
            let items: Vec<String> = all
                .iter()
                .map(|s| {
                    format!(
                        "{}  {:<5} {:<14} {}  {}  ({} bytes)",
                        s.campaign,
                        s.role.name(),
                        s.player,
                        party::format_time(s.modified),
                        s.file,
                        s.bytes
                    )
                })
                .collect();
            for p in party::list_party_saves(&self.saves_root()) {
                self.ui
                    .line(&format!("party save: {}", party::describe_party(&p)));
            }
            let Some(i) = self.ui.menu("Pick a save", &items) else {
                return;
            };
            let s = all[i].clone();
            let acts = if s.role == Role::Host {
                vec!["Continue as host".to_string(), "Delete".into()]
            } else {
                vec!["Delete".to_string()]
            };
            let Some(a) = self.ui.menu(&format!("{} / {}", s.campaign, s.file), &acts) else {
                continue;
            };
            if s.role == Role::Host && a == 0 {
                self.host(Some(s));
                return;
            }
            self.ui.note(&format!(
                "This deletes {} (and its .party sidecar) only.",
                s.file
            ));
            let Some(t) = self
                .ui
                .ask("To confirm, type the exact file name (anything else cancels):")
            else {
                return;
            };
            match party::delete_save(&self.saves_root(), &s, t.trim()) {
                Ok(()) => self.ui.ok("Deleted."),
                Err(e) => self.ui.fail(&e),
            }
        }
    }

    // ---- Port check ----

    fn port_menu(&mut self) {
        let port = self.settings.port;
        loop {
            let items = vec![
                format!("Test this PC for UDP {port}"),
                "Show steps to let friends in".to_string(),
                format!("Listen on UDP {port} for 60 seconds (a friend probes you)"),
                "Probe another launcher".to_string(),
            ];
            let Some(i) = self.ui.menu("Port check", &items) else {
                return;
            };
            match i {
                0 => {
                    for c in [portcheck::bind_test(port), portcheck::loopback_test(port)] {
                        if c.ok {
                            self.ui.ok(&format!("{}: {}", c.name, c.detail))
                        } else {
                            self.ui.fail(&format!("{}: {}", c.name, c.detail))
                        }
                    }
                    match portcheck::local_address() {
                        Some(ip) => self.ui.line(&format!("LAN address: {ip}")),
                        None => self.ui.note("Could not find a LAN address."),
                    }
                    self.ui.line("These checks cannot see your router or Windows Firewall. Use the probe (option 4) from another PC for that.");
                }
                1 => {
                    for l in portcheck::instructions(port, portcheck::local_address()) {
                        self.ui.line(&format!("- {l}"));
                    }
                }
                2 => {
                    self.ui.line(&format!("Listening on UDP {port}. Ask your friend to choose Probe and enter your address. Waits 60 seconds."));
                    let ui = &self.ui;
                    match portcheck::listen_echo(port, 60, &|| false, &mut |from| {
                        ui.ok(&format!("Probe from {from}, answered."))
                    }) {
                        Ok(0) => self.ui.note("No probe arrived."),
                        Ok(n) => self
                            .ui
                            .ok(&format!("{n} probe(s) answered. That path works.")),
                        Err(e) => self.ui.fail(&e),
                    }
                }
                _ => {
                    let Some(t) = self
                        .ui
                        .ask("Address of the listening launcher (ip or ip:port):")
                    else {
                        return;
                    };
                    match joincode::parse_join(&t) {
                        Ok(JoinInput::Addr(a)) => match a.parse::<std::net::SocketAddr>() {
                            Ok(sa) => {
                                let c = portcheck::probe(sa, std::time::Duration::from_secs(8));
                                if c.ok {
                                    self.ui.ok(&c.detail)
                                } else {
                                    self.ui.fail(&c.detail)
                                }
                            }
                            Err(_) => self.ui.fail("Probes need an IP address, not a name."),
                        },
                        Ok(_) => self.ui.fail("Enter an address, not a join code."),
                        Err(e) => self.ui.fail(&e),
                    }
                }
            }
        }
    }

    // ---- Diagnostics ----

    pub fn diagnostics(&mut self) {
        self.ui.title("Diagnostics");
        match self.collect_diagnostics() {
            Ok((dir, files)) => {
                self.ui.ok(&format!(
                    "Collected {} files in {}",
                    files.len(),
                    self.shown(&dir.to_string_lossy())
                ));
                self.ui.line("Your Windows user name was removed from paths and text. Share that folder as it is.");
            }
            Err(e) => self.ui.fail(&e),
        }
    }

    pub fn collect_diagnostics(&self) -> Result<(PathBuf, Vec<String>), String> {
        let out = self
            .home
            .join(format!("diagnostics-{}", run::stamp(run::now_secs())));
        let report = install::check(
            &PathBuf::from(&self.settings.data_path)
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default(),
        );
        let info = format!(
            "nv-launcher {}\nwindows: {}\ndata_path: {}\ninstall_ok: {}\nexe_version: {}\nproblems: {}\nviewer: {} (present: {})\nsteam_dll: {}\n",
            crate::VERSION,
            std::env::var("OS").unwrap_or_default(),
            self.settings.data_path,
            report.ok(),
            report.version.map(install::version_text).unwrap_or_else(|| "unknown".into()),
            report.problems.join(" | "),
            self.viewer.display(),
            self.viewer.is_file(),
            steam::find_dll(std::slice::from_ref(&self.exe_dir)).is_some(),
        );
        let files = crate::diag::collect(&self.home, &out, self.user.as_deref(), &info)?;
        Ok((out, files))
    }

    // ---- Settings ----

    fn settings_menu(&mut self) {
        loop {
            let s = self.settings.clone();
            let items = vec![
                format!("Player name: {}", s.player_name),
                format!("Port: {}", s.port),
                format!(
                    "Transport: {} (Steam is used only when available)",
                    s.transport.name()
                ),
                format!("Windowed: {} ({}x{})", s.windowed, s.width, s.height),
                format!("Uncap frame rate (NV_UNCAP): {}", s.uncap),
                format!(
                    "Join code address: {}",
                    if s.host_address.is_empty() {
                        "(ask each time)"
                    } else {
                        &s.host_address
                    }
                ),
            ];
            let Some(i) = self.ui.menu("Settings", &items) else {
                return;
            };
            match i {
                0 => {
                    if let Some(v) = self.ui.ask("Player name:") {
                        self.settings.player_name = clean_name(&v);
                    }
                }
                1 => {
                    if let Some(v) = self.ui.ask("UDP port (1-65535):") {
                        match v.trim().parse::<u16>().ok().filter(|p| *p != 0) {
                            Some(p) => self.settings.port = p,
                            None => self.ui.fail("Not a port number."),
                        }
                    }
                }
                2 => {
                    self.settings.transport = if s.transport == Transport::Udp {
                        Transport::Steam
                    } else {
                        Transport::Udp
                    };
                }
                3 => {
                    self.settings.windowed = !s.windowed;
                    if self.settings.windowed {
                        if let Some(v) = self
                            .ui
                            .ask_default("Size WxH", &format!("{}x{}", s.width, s.height))
                        {
                            if let Some((w, h)) = v.split_once(['x', 'X']) {
                                if let (Ok(w), Ok(h)) =
                                    (w.trim().parse::<u32>(), h.trim().parse::<u32>())
                                {
                                    if (320..=16384).contains(&w) && (240..=16384).contains(&h) {
                                        self.settings.width = w;
                                        self.settings.height = h;
                                    }
                                }
                            }
                        }
                    }
                }
                4 => self.settings.uncap = !s.uncap,
                _ => {
                    if let Some(v) = self
                        .ui
                        .ask("Address for join codes (empty to ask each time):")
                    {
                        self.settings.host_address = v.trim().to_string();
                    }
                }
            }
            self.save_settings();
        }
    }
}
