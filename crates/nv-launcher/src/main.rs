use nv_launcher::app::App;
use nv_launcher::cli::{self, Cli};
use nv_launcher::portcheck;
use nv_launcher::settings::{self, Settings};
use nv_launcher::ui::Ui;
use nv_launcher::{diag, joincode};
use std::path::PathBuf;
use std::process::ExitCode;

fn env_get(k: &str) -> Option<String> {
    std::env::var(k).ok()
}

fn windows_user() -> Option<String> {
    env_get("USERNAME")
        .or_else(|| env_get("USER"))
        .filter(|u| !u.is_empty())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let c: Cli = match cli::parse(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("nv-launcher: {e}");
            return ExitCode::from(2);
        }
    };
    if c.help {
        print!("{}", cli::HELP);
        return ExitCode::SUCCESS;
    }
    if c.version {
        println!("nv-launcher {}", nv_launcher::VERSION);
        return ExitCode::SUCCESS;
    }
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let home = c
        .home
        .clone()
        .unwrap_or_else(|| settings::home_dir(&env_get));
    let (settings, warn): (Settings, Option<String>) = Settings::load(&home.join("settings.json"));
    let ui = Ui::new(env_get("NO_COLOR").is_some());
    let user = windows_user();

    if let Some(w) = &warn {
        eprintln!("nv-launcher: {w}");
    }
    if c.selftest {
        let items = diag::selftest(&std::env::temp_dir(), &exe_dir);
        let mut bad = 0;
        for i in &items {
            if i.ok {
                ui.ok(&format!(
                    "{}: {}",
                    i.name,
                    nv_launcher::redact::redact(&i.detail, user.as_deref())
                ));
            } else {
                // A missing viewer is only informational when running the launcher alone.
                if i.name == "viewer present" {
                    ui.note(&format!(
                        "{}: {}",
                        i.name,
                        nv_launcher::redact::redact(&i.detail, user.as_deref())
                    ));
                } else {
                    bad += 1;
                    ui.fail(&format!(
                        "{}: {}",
                        i.name,
                        nv_launcher::redact::redact(&i.detail, user.as_deref())
                    ));
                }
            }
        }
        println!("selftest: {}", if bad == 0 { "PASS" } else { "FAIL" });
        return if bad == 0 {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        };
    }
    if c.print_command.is_some() {
        let mut random = [0u8; 8];
        random.copy_from_slice(&coop_net::random_token()[..8]);
        return match cli::print_command(&c, settings, &home, &exe_dir, random) {
            Ok(t) => {
                print!("{t}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("nv-launcher: {e}");
                ExitCode::from(2)
            }
        };
    }
    if let Some(p) = c.port_check {
        let port = p.unwrap_or(settings.port);
        let mut ok = true;
        for ch in [portcheck::bind_test(port), portcheck::loopback_test(port)] {
            ok &= ch.ok;
            println!(
                "{} {}: {}",
                if ch.ok { "[ok]" } else { "[!!]" },
                ch.name,
                ch.detail
            );
        }
        for l in portcheck::instructions(port, portcheck::local_address()) {
            println!("- {l}");
        }
        return if ok {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        };
    }
    if let Some(p) = c.port_listen {
        let port = p.unwrap_or(settings.port);
        println!("Listening on UDP {port} for 60 seconds.");
        return match portcheck::listen_echo(port, 60, &|| false, &mut |from| {
            println!("probe from {from}")
        }) {
            Ok(n) => {
                println!("{n} probe(s) answered.");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::from(1)
            }
        };
    }
    if let Some(t) = &c.port_probe {
        return match joincode::parse_join(t) {
            Ok(joincode::JoinInput::Addr(a)) => match a.parse() {
                Ok(sa) => {
                    let r = portcheck::probe(sa, std::time::Duration::from_secs(8));
                    println!("{}", r.detail);
                    if r.ok {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::from(1)
                    }
                }
                Err(_) => {
                    eprintln!("probe needs an IP address");
                    ExitCode::from(2)
                }
            },
            _ => {
                eprintln!("probe needs ip:port");
                ExitCode::from(2)
            }
        };
    }

    let given_viewer = c.viewer.clone().or_else(|| {
        Some(PathBuf::from(&settings.viewer_path)).filter(|p| !p.as_os_str().is_empty())
    });
    let viewer = cli::find_viewer(given_viewer.as_deref(), &exe_dir);
    if c.coop_help {
        return match std::process::Command::new(&viewer)
            .arg("--coop-help")
            .output()
        {
            Ok(o) if o.status.success() => {
                print!("{}", String::from_utf8_lossy(&o.stdout));
                ExitCode::SUCCESS
            }
            _ => {
                eprintln!("nv-launcher: could not ask the viewer at {} (--viewer EXE or settings viewer_path). The flags are in docs/COOP.md.", viewer.display());
                ExitCode::from(1)
            }
        };
    }
    let mut app = App {
        ui,
        home,
        exe_dir,
        viewer,
        settings,
        user,
    };
    if c.start.is_some() {
        let code = app.start_from_cli(&c);
        return ExitCode::from(code.clamp(0, 255) as u8);
    }
    if c.diagnostics {
        return match app.collect_diagnostics() {
            Ok((dir, files)) => {
                println!(
                    "Collected {} files in {}",
                    files.len(),
                    nv_launcher::redact::redact(&dir.to_string_lossy(), windows_user().as_deref())
                );
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::from(1)
            }
        };
    }
    app.main_menu();
    ExitCode::SUCCESS
}
