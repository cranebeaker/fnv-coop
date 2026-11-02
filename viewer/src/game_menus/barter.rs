//! Trading (`ui::menus::barter`): a script's `ShowBarterMenu` on a merchant
//! (`menus::Menu::Barter`). The prices and the settling of a trade are
//! `world::barter`'s; "How many?" is the quantity menu over it, "Cancel
//! transaction?" a message box it owns.

use cellview::Game;
use esm::{FormId, LoadOrder};
use ui::menus::barter::{BarterMenu, Goods, Request, FILE};
use ui::menus::container::Item;
use ui::menus::message::MessageBox;
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use super::{OpenMenu, Screen};
use crate::menus::Menu;

/// The number the barter menu's own message boxes carry (their owner).
pub const BOX_OWNER: u32 = 1053;

/// The barter menu on screen and the vendor.
pub struct BarterScreen {
    pub menu: BarterMenu,
    pub vendor: FormId,
}

/// Whether this module shows a request.
pub fn takes(menu: &Menu) -> bool {
    matches!(menu, Menu::Barter(..))
}

fn item_of(l: world::items::InventoryLine) -> Item {
    Item {
        form: l.item.0,
        name: l.name,
        count: l.count,
        form_type: l.form_type,
        equipped: l.equipped,
        quest_item: l.quest_item,
        playable: l.playable,
        regenerating_ammo: l.regenerating_ammo,
        modded: false,
        weightless: l.weightless,
        is_caps: l.item.0 == 0xF,
        icon: l.icon,
    }
}

/// The two sides' things with their prices: the player's, and the
/// vendor's goods (their own and their merchant container's, together,
/// those their services cover).
fn sides(
    order: &LoadOrder,
    state: &GameState,
    vendor: FormId,
) -> (Vec<Goods>, Vec<Goods>, (i32, i32)) {
    let m = world::barter::multipliers(order, state);
    // `ShowBarterMenu`'s price adjustment isn't passed on by the world's
    // event yet: 0.
    let adjustment = 0;
    let priced = |holder: FormId, l: world::items::InventoryLine| {
        let worth = world::barter::item_value(order, state, holder, l.item);
        let item = l.item;
        Goods {
            sell: world::barter::price(order, state, item, worth, true, m, adjustment),
            buy: world::barter::price(order, state, item, worth, false, m, adjustment),
            item: item_of(l),
        }
    };
    let player: Vec<Goods> = world::items::inventory_lines(order, state, PLAYER_REF)
        .into_iter()
        .map(|l| priced(PLAYER_REF, l))
        .collect();
    let services = world::barter::services(order, vendor);
    let mut goods: Vec<Goods> = Vec::new();
    for holder in world::barter::vendor_holders(order, vendor) {
        for l in world::items::inventory_lines(order, state, holder) {
            if !world::barter::deals_in(order, services, l.item) {
                continue;
            }
            match goods.iter_mut().find(|g| g.item.form == l.item.0) {
                Some(g) => g.item.count += l.count,
                None => goods.push(priced(holder, l)),
            }
        }
    }
    let caps = (
        state.item_count(order, PLAYER_REF, world::barter::caps(order)),
        world::barter::vendor_caps(order, state, vendor),
    );
    (player, goods, caps)
}

/// A reference's name (its base's `FULL`).
fn name_of(order: &LoadOrder, reference: FormId) -> String {
    world::scripting::base_of(order, reference)
        .and_then(|b| order.get(b))
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name())
        .unwrap_or_default()
}

/// Opens the barter menu for a request.
pub fn open(screen: &mut Screen, game: &Game, state: &mut GameState, request: Menu) -> Vec<FormId> {
    let Menu::Barter(vendor) = request else {
        return Vec::new();
    };
    let order = &game.order;
    let mut menu = BarterMenu::new(0);
    let tile = match screen.load(game, FILE, &mut menu) {
        Ok(t) => t,
        Err(e) => {
            println!("The barter menu can't be shown: {e}");
            return Vec::new();
        }
    };
    menu.menu = tile;
    menu.vendor_name = name_of(order, vendor);
    menu.english = game
        .settings
        .get("General", "sLanguage")
        .is_none_or(|l| l.trim() == "ENGLISH");
    menu.ask_quantity_at = world::scripting::game_setting(order, "iInventoryAskQuantityAt")
        .map_or(ui::menus::container::ASK_QUANTITY_AT, |v| v as i32);
    for h in world::barter::vendor_holders(order, vendor) {
        state.stock(order, h);
    }
    let (player, goods, caps) = sides(order, state, vendor);
    if !menu.open(&mut screen.ui, player, goods, caps) {
        println!("MENUS: Barter Menu Creation Failed.");
        screen.ui.detach(tile);
        return Vec::new();
    }
    println!("Barter menu: {}.", menu.vendor_name);
    let sounds = menu
        .sounds
        .drain(..)
        .filter_map(|s| order.form_by_editor_id(&s))
        .collect();
    screen
        .open
        .push(OpenMenu::Barter(Box::new(BarterScreen { menu, vendor })));
    sounds
}

/// What the menu asked for, carried out. Returns sounds to play.
pub fn after(screen: &mut Screen, game: &Game, state: &mut GameState) -> Vec<FormId> {
    let order = &game.order;
    let mut sounds = Vec::new();
    // The quantity menu's answer goes to the barter menu below it, and
    // "Cancel transaction?"'s Yes closes it.
    let mut answer = None;
    let mut cancel = false;
    let mut barter_below = false;
    for m in screen.open.iter_mut() {
        match m {
            OpenMenu::Barter(_) => barter_below = true,
            OpenMenu::Quantity(q) if barter_below => {
                if let Some(n) = q.answer.take() {
                    answer = Some(n);
                }
            }
            OpenMenu::Message(msg) => {
                if msg.take_pressed_for(Some(BOX_OWNER)) == Some(0) {
                    cancel = true;
                }
            }
            _ => {}
        }
    }
    let mut ask = None;
    let mut confirm = false;
    for m in screen.open.iter_mut() {
        let OpenMenu::Barter(b) = m else {
            continue;
        };
        if let Some(n) = answer {
            b.menu.quantity_chosen(&mut screen.ui, n);
        }
        if cancel {
            b.menu.close(&mut screen.ui);
        }
        for name in b.menu.sounds.drain(..) {
            if let Some(id) = order.form_by_editor_id(&name) {
                sounds.push(id);
            }
        }
        let vendor = b.vendor;
        let requests: Vec<Request> = std::mem::take(&mut b.menu.requests);
        for request in requests {
            match request {
                Request::AskQuantity { most } => ask = Some(most),
                Request::Sound { form, up } => {
                    if let Some(s) = world::sound::item_sound(order, FormId(form), up) {
                        sounds.push(s);
                    }
                }
                Request::Accept { buys, sells, total } => {
                    let buys: Vec<_> = buys.into_iter().map(|(f, n)| (FormId(f), n)).collect();
                    let sells: Vec<_> = sells.into_iter().map(|(f, n)| (FormId(f), n)).collect();
                    if let Some(s) =
                        world::barter::accept(order, state, vendor, &buys, &sells, total)
                            .and_then(|s| order.form_by_editor_id(s))
                    {
                        sounds.push(s);
                    }
                    println!("Barter: traded ({total:+.1} caps).");
                    let (player, goods, caps) = sides(order, state, vendor);
                    b.menu.settled(&mut screen.ui, player, goods, caps);
                }
                Request::ConfirmExit => confirm = true,
                Request::Close { .. } => {}
            }
        }
    }
    if let Some(most) = ask {
        super::container::open_quantity(screen, game, most);
    }
    if confirm {
        open_confirm(screen, game);
    }
    sounds
}

/// "Cancel transaction?" with Yes and No (`0072d770` case 0x13: kind 0x17,
/// the callback `0072db40`).
fn open_confirm(screen: &mut Screen, game: &Game) {
    let text = |n: &str| screen.ui.setting_text(n).unwrap_or_default();
    let (q, yes, no) = (text("sCancelBarter"), text("sYes"), text("sNo"));
    let alpha = super::message::background_alpha(screen);
    let mut b = MessageBox::new(&q, None, &[&yes, &no], 0x17, alpha);
    b.owner = Some(BOX_OWNER);
    super::message::show(screen, game, b);
}
