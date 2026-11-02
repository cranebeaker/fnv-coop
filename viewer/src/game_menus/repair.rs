//! Merchants' repairs (`ui::menus::repair_services`): a script's
//! `ShowRepairMenu` on a vendor (`menus::Menu::RepairServices`). What's
//! listed, the costs and what the vendor mends to are `world::repair`'s.

use cellview::Game;
use esm::{FormId, LoadOrder};
use ui::menus::repair_services::{RepairServicesMenu, Request, FILE};
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use super::{OpenMenu, Screen};
use crate::menus::Menu;

/// The repair menu on screen and the vendor.
pub struct RepairScreen {
    pub menu: RepairServicesMenu,
    pub vendor: FormId,
}

/// Whether this module shows a request.
pub fn takes(menu: &Menu) -> bool {
    matches!(menu, Menu::RepairServices(..))
}

/// The vendor's Repair, what they mend to, the player's lines and caps.
fn now(
    order: &LoadOrder,
    state: &GameState,
    vendor: FormId,
) -> (i32, f32, Vec<world::repair::ServiceLine>, i32) {
    let skill = world::repair::skill(order, state, vendor);
    let target = world::repair::service_target(order, skill);
    let (lines, _) = world::repair::service_lines(order, state, skill);
    let caps = state.item_count(order, PLAYER_REF, world::barter::caps(order));
    (skill, target, lines, caps)
}

/// Opens the menu for a request.
pub fn open(screen: &mut Screen, game: &Game, state: &mut GameState, request: Menu) {
    let Menu::RepairServices(vendor) = request else {
        return;
    };
    let order = &game.order;
    let mut menu = RepairServicesMenu::new(0);
    let tile = match screen.load(game, FILE, &mut menu) {
        Ok(t) => t,
        Err(e) => {
            println!("The repair menu can't be shown: {e}");
            return;
        }
    };
    menu.menu = tile;
    menu.caps_limit = world::scripting::game_setting(order, "iCapsLimit")
        .map_or(ui::menus::repair_services::CAPS_LIMIT, |v| v as i32);
    state.stock(order, PLAYER_REF);
    let (skill, target, lines, caps) = now(order, state, vendor);
    if !menu.open(&mut screen.ui, skill, target, lines, caps) {
        println!("MENUS: Repair Services Menu Creation Failed.");
        screen.ui.detach(tile);
        return;
    }
    println!(
        "Repair services: Repair {skill}, mending to {:.0}%.",
        target * 100.0
    );
    screen
        .open
        .push(OpenMenu::Repair(Box::new(RepairScreen { menu, vendor })));
}

/// What the menu asked for, carried out. Returns sounds to play.
pub fn after(screen: &mut Screen, game: &Game, state: &mut GameState) -> Vec<FormId> {
    let order = &game.order;
    let mut sounds = Vec::new();
    for m in screen.open.iter_mut() {
        let OpenMenu::Repair(r) = m else {
            continue;
        };
        for name in r.menu.sounds.drain(..) {
            if let Some(id) = order.form_by_editor_id(&name) {
                sounds.push(id);
            }
        }
        let vendor = r.vendor;
        let requests: Vec<Request> = std::mem::take(&mut r.menu.requests);
        let mut changed = false;
        for request in requests {
            match request {
                Request::Repair { form, cost } => {
                    world::repair::repair_by(order, state, vendor, FormId(form), cost);
                    println!("Repair services: mended {form:08X} for {cost} caps.");
                    changed = true;
                }
                Request::RepairAll { forms, total } => {
                    for form in &forms {
                        world::repair::repair_by(order, state, vendor, FormId(*form), 0);
                    }
                    world::repair::pay(order, state, vendor, total);
                    println!(
                        "Repair services: mended {} things for {total} caps.",
                        forms.len()
                    );
                    changed = true;
                }
                Request::Close => {}
            }
        }
        if changed {
            let (_, _, lines, caps) = now(order, state, vendor);
            r.menu.fill(&mut screen.ui, lines, caps);
        }
    }
    sounds
}
