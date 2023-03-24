# Speedrunapi  

*Rust Edition

The speedrunapi crate intends to help make working with speedrun.coms REST API easier.

Currently this project is in its very early stages, with hope to cover everything possible efficently, and easily. Any help on the project is very much appreiciated!

This project was made by me for fun and to help me pratice rust, so I hope someone can find use for this.

**NOTE:** 0.3.0 is a major rewrite of the project, causing breaking changes to mostly every thing.

## Basic Usage:

```rust
use speedrunapi::GameData;
let result = GameData::new("Mc").run();
assert_eq!(result.name(), "Minecraft: Java Edition");
```
This gets the name of a game.

```rust
use speedrunapi::UserData;
let result = UserData::new("fishin_rod").run();
assert_eq!(result.id(), "jonryvl8");
```
This gets the id of a user.

For more information about this crate check out the [documentation!](https://docs.rs/crate/speedrunapi/latest)
