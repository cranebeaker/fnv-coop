# Crafting: M2 working evidence

Sunny's campfire tutorial (`VCG03`, "By a Campfire on the Trail") and every
campfire, workbench and reloading bench open the crafting menu with
`ShowRecipeMenu <category>` on the player (`VCG03CraftingCampfireRecipesScript`,
`CraftingWorkbenchRecipesScript`, `CraftingReloadingBenchRecipesScript`,
...; `nvinspect <Data> scripts --grep ShowRecipeMenu`). Until 2026-10-06 the
function was not carried out.

## Sources

`FalloutNV.exe` 1.4.0.525 (SHA-256 `19406942...739F`), read in Ghidra
12.0.4 from the full private export; names, the record structures and the
class layouts from the Xbox 360 prototype's symbols (Xbox PDB, read with
`llvm-pdbutil` after [research/pdb](../research/pdb/README.md)).

| What | PC | Xbox PDB name |
| --- | --- | --- |
| `ShowRecipeMenu` (opcode 0x1250, `craft`, one parameter) | `005deb10` | |
| Opening the menu (`Data\Menus\recipe_menu.xml`, `CM_list_template`, the `HelpCrafting` message once) | `00726ff0` | `RecipeMenu::Create` |
| Filling the list | `00727680` | `RecipeMenu::UpdateLists` |
| How many can be made | `00727920` | `RecipeMenu::CanMakeRecipe` |
| List order | `00728c10` | `RecipeMenu::SortFunc` |
| Filter order | `007278c0` | `RecipeMenu::FilterSortFunction` |
| The selected recipe's details | `00727b10` | |
| The item card | `00728da0` | `RecipeMenu::PopulateItemStatsDisplay` |
| Clicks (filter arrows 0-2, Make 7, Exit 8) | `007274b0` | `RecipeMenu::DoClick` |
| Making (the quantity menu's callback) | `007284f0` | |
| Loading `RCPE` | `005a8110` | `TESRecipe::Load` |

`RecipeMenu`'s vtable is `0107048c` (found through its RTTI).

## The record

`RECIPE_DATA` (Xbox PDB): `ReqSkillId` (i32, −1 for none),
`ReqSkillLevel`, `CategoryID`, `SubCategoryID`. `TESRecipe` (Xbox PDB,
108 bytes on the Xbox): form, full name, `data`, `RecipeConditions`,
`ingredientlist`, `outputlist`, `garbageCollection`, `pRecipeCat`,
`pRecipeSubCat`. The PC's getters read the same fields 16 bytes earlier
(its form header is smaller): data +0x24, conditions +0x34, ingredients
+0x3c, outputs +0x44, category +0x54, subcategory +0x58. A component is
`Quantity` then `pInternalItem` (`TESRecipeComponent`, Xbox PDB).

Loading (`005a8110`): `RCIL` starts an ingredient and `RCOD` an output,
`RCQY` sets the current one's count, and after every subrecord the current
component is added to its list once it has an item and a count other than
0. A component without `RCQY` is never added; a stray subrecord after an
`RCQY` would add the same component again (no official recipe has one).
`RCCT` has one byte of flags (`RECIPE_CATEGORY_DATA`, Xbox PDB).

## The rules (`world::crafting`)

- **Listed** (`00727680`): every recipe whose category is the menu's and
  whose conditions pass with the actor as the subject (`00680c30`). Each
  line is the recipe's name, with " (n)" when its first output comes more
  than one at a time. The filter offers the listed recipes'
  subcategories, each once, sorted by name.
- **How many** (`00727920`): 0 without a category and a subcategory, with
  too little skill, or from another category's menu; otherwise the
  smallest `have / need` over the ingredients, at most 100, and 0 when any
  is short. The skill is the value `GetActorValue` reads (the actor value
  owner's +0xc), clamped to 0..100 (`0066f190`) and floored (`0066ef20`).
- **Order** (`00728c10`): lines that can be made first, then by name with
  `_mbscmp` (byte order, case-sensitive). Lines that can't be made are
  drawn at alpha 0x7f.
- **Making** (`007284f0`, n times; then the menu closes): each ingredient's count × n removed
  (an equipped weapon the actor holds only one of is unequipped first);
  "Items Crafted" (misc stat 0x20) + n unless the first ingredient is
  casino chips (form type 0x6C); each output's count × n added, weapons
  other than grenades, mines and thrown weapons (animation types 10-13,
  `004c0bf0`) and armour and clothing (0x18, 0x1A) at 80 % condition
  (`00482090(0.8)`); a notice per output, "name added" or
  "n name`sPlural` added" (`sAddItemtoInventory`, `sPlural`) with the
  gift-box icon.

## Checks

`crates/world/tests/crafting.rs` (generated recipes): loading including a
count-less ingredient, skill and ingredient limits, the 100 cap, category
and subcategory checks, the condition, list and filter order, and making
(counts, notices, the statistic, chips, condition). On the official data,
`nvinspect <Data> recipes CampfireRecipes XanderRoot:2 BrocFlower:3
NVGeckoMeat:1 44=30` lists 30 campfire recipes (filter Aid, Chems, Food,
Misc) with Gecko Steak (Survival 25) and Healing Powder (no skill, 2) at
the top.

Not compared with the running original game yet.

## The menu (`ui::menus::recipe`, `viewer/src/game_menus/recipe.rs`)

Tile ids from `recipe_menu.xml`: 0 and 2 the filter arrows, 1 the filter's
title, 3 the recipes, 4 the category, 5 the skill, 6 the ingredients, 7
Accept, 8 Exit, 9 the picture, 10 the item card, 11-13 the headings; the
line template `RM_list_template` (id 15). From the code:

- Opening (`00726ff0`): both lists ask for `CM_list_template` (the file
  has `RM_list_template`); `sRecipes` (1), `sSkillRequirement` (13),
  `sIngredients` (11), `sMadeAt` (12), `sAccept` (7), `sExit` (8) (the
  settings' objects `011d2088`, `011d2b08`, `011d3090`, `011d50f8`,
  `011d26f4`, `011d33fc`); Accept and Exit 20 units further left.
- A line's alpha goes on the line's first child, its text (`007b5370`).
- The filter index (`0119fbf4`) starts at -1 in the exe's data and is a
  global, so it is kept from one opening to the next.
- The pointer on a line (`00727b10`, the menu's +0x10, id 15): the
  category (alpha 127 when it isn't the menu's), the heading (the
  subcategory's name and the products, as "name (n)", for a recipe of one
  ingredient and several products), the ingredients as
  "name (have/need)" sorted by name (`00728cd0`, alpha 127 when short), the
  skill "name (current/required)" (alpha 127 when too low), Accept's
  `target` only when it can be made, the first product's picture.
- The item card (10, `item_stats_display.xml`; `00728da0` with the first
  product's item, the recipe's +0x44 output list's first component, then
  the card shown): the file's first 12 cards in order (DR, DPS, weight,
  value, condition, ammunition, effects, three mods, strength, damage; the
  13th, DT, is left alone), titled from the settings (`sModEffects` on the
  first mod card), each shown by a bit of the mask: weapons (form type
  0x28) 0xc3e: the DPS (`00645380`), the damage at full condition
  (`006450f0`, "%d" rounded or "%.1fx%d" with more projectiles a shot,
  `00525b20`), the condition meter full (`user5` 1), the strength needed
  (`00663b60`, the card made opaque) and the ammunition card's title, its
  short name (else name) and "(0/held)", or the name alone for a weapon
  that regenerates (`00709430`), "--" without; armour and clothing (0x18,
  0x1a) 0x1d: the DR at full condition (`004be060(1.0)`, "%d"); anything
  else 0xc; the effects (`00406620`) when there's text (0x40). Weight
  (`0048ebc0`) and value: "--" for nothing, "%.1f" below 1, else "%.0f";
  the value goes through `00647c00` with 1 as the condition in percent
  (the exe passes 1.0 where the inventory passes a percentage): with the
  master's `fItemConditionValueBase` 0 and `fItemConditionValueMult`
  0.0316 that is a thousandth of the value, so the Bottlecap Mine (150)
  reads 0.1 and Healing Powder (4) 0.0 (seen in the viewer; not compared
  with the running game). The effects and mod cards' y: half the card's
  height less 20, twice that with the condition or ammunition card.
  Here: `ui::menus::recipe::Card` and `RecipeMenu::show_card`, worked out
  by `ui::pipboy::gather::recipe_card`; the DPS and the effects' text
  aren't worked out (as on ITEMS' card), so the DPS is empty and the
  effects card stays hidden. Seen in the viewer (`--run "player.
  ShowRecipeMenu WorkbenchRecipes" --menu-pointer 525,215`): on Bottlecap
  Mine the DPS and condition (full) cards, STR 1, WG 0.5, VAL 0.1 and
  "--" for ammunition, beside the Vault Boy picture; on the campfire's
  Healing Powder only WG and VAL.
- Accept (7, `007274b0` case 7): the quantity menu with the most that can be
  made (`007aba00`, which always opens), then `007284f0`, which ends by
  closing the menu (`00727430`: `LEAVE_STACK` 1): every craft needs the
  bench again. 0 makes nothing and the menu stays.
- The settings the menu and notice read are exe defaults (the official
  master sets none of them): `sPlural` is "(s)", so "3 Stimpak(s) added".

Checked: two `ui` tests (opening, filter wrap and memory, details, Accept
only when possible, Exit); live in the viewer on the official data
(`--run "player.ShowRecipeMenu CampfireRecipes"` with ingredients given
and `--menu-pointer` on the first line): the layout, the dim lines, the
highlight, the details and the picture as the file and code give them.
Not compared with the running original game.

## Not done

- The `HelpCrafting` tutorial message the first time (`00726ff0`:
  tutorial flag 0x26); the game's tutorial help messages aren't in the
  viewer at all yet.
- The item card's DPS (`00645380`) and effects text (`00406620`); the
  picture of armour by the player's sex (`00480f80` → `00481280`); the
  ingredient list's own pointer (an ingredient's card and picture,
  `00727b10` with the list switched, `011d8ea5`), the
  controller's list switching (special codes 0xd, 0xe) and the cross-fade
  (`00728a70`).
- The challenge events crafting raises (`005f5950(0xc, ...)` and the misc
  stat's), and the HUD ammo refresh when the ingredient is the equipped
  weapon's ammunition.
- Armour and clothing condition (the world keeps condition for weapons
  only).
