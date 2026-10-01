# Pocket Throne for Elden Ring

This is a toy project based largely on the [spawn-asset] and [debug-line] examples from [fromsoftware-rs](https://github.com/vswarte/fromsoftware-rs). The only interesting addition is editing the spawned asset's params.

I don't plan to work on it further, since it already does what I want it to do.

Object IDs from [AEG Reference Sheet] (via [?WikiName? Souls Modding Wiki][wiki])

## Installation

[me3](https://me3.help) is the recommended mod loader for Elden Ring and other
FromSoftware games.

1. Extract `pocket_throne.dll` wherever you want.
2. In your [me3 profile](https://me3.help/en/latest/configuration-reference/), add a native mod with the path to `pocket_throne.dll`.

## Usage

Press the hotkey (default **H**) to summon a throne.

### Notes

- The throne spawns directly behind you, facing the same direction as the
  character (not camera).
- The throne has 200 HP; it will not break instantly on jumps/rolls, but is easily
  breakable with attacks.
- The throne has no break animation/debris so you can reposition it [without
  littering the ground](https://en.wikipedia.org/wiki/Leave_No_Trace).
- Thrones can also be cleared by reloading the area (fast travelling).

## Config

The hotkey can be optionally configured in `pocket_throne.ini` (next to the DLL).

Use the hexadecimal value from [Windows virtual-key codes][vk].

## Known issues

- The first throne summoned (per game session) is not breakable. Just summon one
  before you head out to your scenic spot.

## Unknown issues



[spawn-asset]: https://github.com/vswarte/fromsoftware-rs/blob/59fbd3b3b7daaf14aca47c9f73530493dba6bc79/examples/spawn-asset/src/lib.rs
[debug-line]: https://github.com/vswarte/fromsoftware-rs/blob/59fbd3b3b7daaf14aca47c9f73530493dba6bc79/examples/debug-line/src/lib.rs
[AEG Reference Sheet]: https://docs.google.com/spreadsheets/d/1mImhhti5WLenNp7lfY87MPXeI9yT0Z2Rxbx8XcYMjgs/edit
[wiki]: https://soulsmodding.com/doku.php?id=er-refmat:main
[vk]: https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
