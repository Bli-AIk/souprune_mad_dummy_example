# souprune_example_mods

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE) <img src="https://img.shields.io/github/repo-size/Bli-AIk/souprune_example_mods.svg"/> <img src="https://img.shields.io/github/last-commit/Bli-AIk/souprune_example_mods.svg"/>

**souprune_example_mods** — Example mods for SoupRune, demonstrating overworld, battle, and Alight Motion integration.

| English         | Simplified Chinese                 |
|-----------------|-----------------------------------|
| English         | [简体中文](./readme_zh-hans.md)    |

## Introduction

`souprune_example_mods` is a collection of example mods for the SoupRune game engine.  
It provides ready-to-use templates for creating your own mods, demonstrating:

- Overworld gameplay with tile-based maps
- Battle system integration
- Alight Motion animation support

With these examples, you only need to copy the relevant mod structure and customize the content.  
Each mod is maintained in a separate branch for easy cloning.

## Included Mods

| Branch | Description |
|--------|-------------|
| `mod/example_mod` | Full-featured example with overworld exploration and battle sequences |
| `mod/example_battle_mod` | Battle-focused example demonstrating the combat system |
| `mod/example_am_mod` | Alight Motion animation integration example |

## How to Use

### Clone a specific mod

```bash
# Clone example_mod
git clone -b mod/example_mod --single-branch https://github.com/Bli-AIk/souprune_example_mods.git example_mod

# Clone example_battle_mod
git clone -b mod/example_battle_mod --single-branch https://github.com/Bli-AIk/souprune_example_mods.git example_battle_mod

# Clone example_am_mod
git clone -b mod/example_am_mod --single-branch https://github.com/Bli-AIk/souprune_example_mods.git example_am_mod
```

### Add to SoupRune project

1. Place the cloned directory in SoupRune's `projects/` folder
2. Update `projects/config.toml` to point to your mod:
   ```toml
   [project]
   mod_name = "example_mod"
   language = "en-US"
   ```

### Build C# code (if modified)

```bash
cd example_mod
./build.sh        # Linux/macOS
./build.ps1       # Windows PowerShell
```

## Mod Structure

```
example_mod/
├── mod.toml              # Mod configuration
├── runtime/              # Runtime WASM mod
├── content/              # Vessel content guest
├── .build/               # Built runtime/content wasm artifacts
├── app/                  # Project flow and input config
├── battle/               # Battle content
├── overworld/            # Overworld content (if used)
├── actors/               # Actor definitions (if used)
├── narrative/            # Dialogue and narrative config
├── view/                 # View structures and touch layout
└── assets/               # Textures, audio, maps, locales, etc.
```

## Contributing

Contributions are welcome!
Whether you want to fix a bug, add a feature, or improve documentation:

* Submit an **Issue** or **Pull Request**.
* Share ideas and discuss design or architecture.

## License

This project is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
