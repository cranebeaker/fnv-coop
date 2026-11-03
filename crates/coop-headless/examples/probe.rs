//! Prints what the world crate knows about a reference:
//! `cargo run -p coop-headless --example probe -- REF ...` (needs FNV_ESM_DIR).
use coop_headless::data::Data;
use world::scripting::GameState;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let dir = Data::locate(None).expect("FNV_ESM_DIR");
    let d = Data::open(&dir).expect("data");
    let gs = GameState::new(&d.order);
    for w in &a {
        let Some(f) = d.form(w) else {
            println!("{w}: not found");
            continue;
        };
        println!(
            "{w} = {f}: actor_base {:?}, health {:?}, max {:?}, essential {}, place {:?}",
            d.actor_base(f),
            world::combat::health(&d.order, &gs, f),
            world::combat::max_health(&d.order, &gs, f),
            world::more_functions::is_essential(&d.order, &gs, f),
            gs.place(&d.order, f)
        );
        if let Some(b) = d.actor_base(f) {
            let acbs = d
                .order
                .get(b)
                .and_then(|rr| rr.record().ok())
                .and_then(|r| r.get(esm::FourCC::new(b"ACBS")).map(|s| s.data.clone()));
            println!(
                "  base {b} ACBS {:02x?}",
                acbs.as_deref().map(|x| &x[..x.len().min(8)])
            );
            let same = world::more_functions::is_essential(&d.order, &gs, b);
            println!("  is_essential(base) {same}");
        }
    }
}
