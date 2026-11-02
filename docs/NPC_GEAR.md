# People's clothes and weapons, redrawn as they change

Before this batch the viewer built a person's worn models once, from their
record, and never again: a companion given armour, an `EquipItem` from a
script, armour taken away by trading or `RemoveItem`, a weapon swapped,
all changed the game state but not the picture. Now a person (not a
creature, not the player: the player's body is `viewer/src/player_body.rs`)
is redrawn when what they wear or hold changes, and only the parts that
changed are rebuilt.

## What the game does (FalloutNV.exe 1.4.0.525; Xbox PDB names)

- Equipping: `0088c830` (Xbox `Actor::EquipObject` 826d3a20; from the
  script `EquipItem` `005d0060` → `0088c650`) does nothing when the actor
  carries none of the item; armour (form type 0x18) with health left goes
  to `0088db20` (`Actor::AddWornItem`, Xbox 826cc698): every worn item on
  any biped slot the new one fills is taken off (`UnEquipObject`) and its
  biped parts removed at once (`BipedAnim::RemovePart`); the item is marked
  worn (`InventoryChanges::AddWorn`); then, for a character other than the
  player (vtable +0x218) and not a weapon, `TESNPC::InitWornObject`
  (`006061b0`, Xbox 82440cc8) → `TESBipedModelForm::AddToBiped`
  (`00480bd0`) for the item and each addon of its model list (`BIPL`) →
  `BipedAnim::SetBipedPart` (`004abad0`): the parts of other forms on any
  slot it fills are removed and the form is set in the first slot it
  fills. The female model is used only when its path isn't empty.
- Unequipping: script `UnequipItem` `005d0300` → `0088c790`.
- A worn item leaving the inventory (`RemoveItem`, trading, stealing):
  `MiddleHighProcess::ProcessRemoveWorn` (Xbox 8272e1f8) →
  `TESNPC::InitDefaultWorn` (Xbox 82440908: everything worn taken off,
  then per slot `InventoryChanges::GetBestArmor` put on) and the update-3D
  flag 1.
- The 3D: the process's update-3D flags (Xbox process vfunc 0x468 sets,
  0x478 reads; 1 model, 2 skins, 4/8 head, 0x10 scale).
  `HighProcess::Update3dModel` (Xbox 82708178) on flag 1 →
  `TESObjectREFR::ReplaceModel` → `BipedAnim::LoadBipedParts` (Xbox
  822fd188): per slot, **when the slot's model is the one already loaded
  the loaded 3D is kept**; otherwise the old part is removed and the new
  one loaded (`ModelLoader::LoadFile`, cached), cloned, texture-swapped and
  skinned to the skeleton (`ApplySkinnedObjects`) or hung from its bone;
  the weapon slot (5) goes through `AttachWeapon` (`004ab750`). The work
  can be queued for the task threads (`QueueCharacterReset3D`,
  `ShouldQueue3DTask`).
- `TESNPC::InitParts` (Xbox 824454f0), the full rebuild (the container
  menu closing on an NPC calls it): race parts, then the worn item per slot
  in the order 0, 1, 2, 4, 6..19, then 3 (left hand), then 5 (weapon)
  (`TESNPC::InitWorn`, Xbox 82443a98), then `FixDisplayedHeadParts` (Xbox
  82440dd0: head parts shown only when nothing is worn in slot 0; hair
  hidden when slot 1 is worn; a hat, slot 10, switches the hair to its hat
  variant).

## Here

- `world::outfit` (crate `world`): `start_worn` = what the person's record
  has them wear (the look's own choice, `world::actor::worn_of`: the first
  carried piece per body slot, the same rule the spawn look has always
  used); `worn_armour` = what they wear now: pieces put on since
  (`GameState::equipped`) while carried, and the start's pieces unless
  covered by those, taken off (`GameState::taken_off`, filled by
  `GameState::equip` with what a new piece replaced and by
  `unequip_item`), or, once their inventory is in the state, no longer
  held. `taken_off` is saved (`takenoff` lines).
- `world::actor::npc_look_wearing`: the look (`ActorLook`) a person makes
  wearing given items and holding a given weapon, assembled exactly as the
  spawn look is.
- Each look part's pieces carry their part index
  (`preview::cell::ModelMesh::actor_part` → `cellview::MeshData` →
  the viewer's `dress::Piece`); `cellview::ActorData::look` keeps the look
  an actor was built from.
- `viewer/src/dress.rs`: each frame, after the people are posed, what each
  person wears (`worn_armour`) and holds (`world::combat::weapon_in_hand`,
  as `actors::npc_frame` last saw it) is compared with what they were
  drawn with. On a change, a thread builds the new look, pairs its parts
  with the drawn ones (`match_parts`: equal parts are kept, as
  `LoadBipedParts` keeps an unchanged slot) and builds only the new parts
  (`cellview::Game::actor_scene`, a lone-actor scene of just those parts); the
  old pieces stay drawn until it's done, then the pieces of the parts that
  went are despawned and the new ones skinned to the person's own joints.
  Animation keeps running on the same skeleton; a weapon of another kind
  brings its holster pose, and a new weapon re-hangs where the drawn state
  has it (`ActorRig::weapon_attached`, `004ab750`).

Only `actors.rs` changed on the animation side: `npc_frame` remembers the
weapon it looked at (`ActorRig::held_weapon`), and new actors get the
`dress::Dressed` component.

## Checked

- Tests: `crates/world/tests/outfit.rs` (plugin built from scratch:
  start pieces incl. a leveled list's, equipping not carried does nothing,
  equipping over slots, taking off, taking away and picking again, the
  look made from it; `taken_off` saved and loaded),
  `viewer/src/dress.rs` `only_the_parts_that_changed_are_rebuilt`.
- Live (release viewer, installed data, GSDocMitchellHouse, `--freeze-ai`):
  Doc Mitchell given and made to wear `ArmorLeather` and `GamblerMHat01`
  is drawn in them (7 pieces gone, 4 parts came; built in about 85 ms on
  its own thread, put on in under 2 ms); with the armour then removed
  (`RemoveItem`) and the hat taken off (`UnequipItem`) he is drawn in his
  own clothes again. Weapons: given `WeapNV9mmPistol` and
  `WeapHuntingRifle` he is drawn with the rifle (the best weapon he
  carries) on his back; `EquipItem` the pistol swaps it for the pistol at
  his thigh, `EquipItem` the rifle back again (each a 1-part rebuild, under
  40 ms on the thread, 0.1 ms to put on).

## Not done / differences

- The start's choice is the game's (`actor::pick_worn`,
  `TESNPC::InitDefaultWorn` `006047c0` with
  `InventoryChanges::GetBestArmor` `004c8220` per slot 0 to 19, on the
  record's container: directly named items first, then what leveled lists
  gave, whose order among themselves isn't traced). Picking again after a
  worn item is taken away (`ProcessRemoveWorn` → `InitDefaultWorn`, above)
  isn't applied yet: the start's pieces come back if still carried, where
  the game picks per slot from the whole inventory (as companions do after
  trading, [COMPANIONS.md](COMPANIONS.md)).
- A person whose inventory comes from their template: taking the
  template's clothes away isn't seen (`GameState::stock` copies only the
  person's own record).
- The dead keep the weapon they were drawn with (the redraw follows
  `npc_frame`, which the dead skip); their armour is followed.
- Not compared side by side with the original game.
