# YARGE

`YARGE` is a simple game engine written in Rust

## Structure

OS $\xrightarrow{\text{in}}$ Platform Layer $\xrightarrow{\text{in}}$ Application Layer

## Platform

### PS Vita

To build for the PSvita:

- modify the root Cargo.toml, especially the unique id:
```toml
# Cargo.toml in your game
[package.metadata.vita]
# A unique identifier for your project. 9 chars, alphanumeric.
title_id = "RUSTAPP01"
# A title that will be shown on a bubble. Optional, will take the crate name as the default
title_name = "My application"
# Optional. A path to static files relative to the project.
assets = "static"
# Optional, this is the default
build_std = "std,panic_unwind"
# Optional, this is the default
vita_strip_flags = ["-g"]
# Optional, this is the default
vita_make_fself_flags = ["-s"]
# Optional, this is the default
vita_mksfoex_flags = ["-d", "ATTRIBUTE2=12"]
```

- build in release mode
```sh
# In the terminal
cargo vita build vpk --release
# creates a `./target/armv7-sony-vita-newlibeabihf/release/your_game.vpk`
```

- run with
```sh
# In the terminal
# replace `your_game' with your actual cargo project
vita3k ./target/armv7-sony-vita-newlibeabihf/release/your_game.vpk
```