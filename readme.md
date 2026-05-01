# souprune_mad_dummy_example

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE) <img src="https://img.shields.io/github/repo-size/Bli-AIk/souprune_mad_dummy_example.svg"/> <img src="https://img.shields.io/github/last-commit/Bli-AIk/souprune_mad_dummy_example.svg"/>

**souprune_mad_dummy_example** is the maintained Mad Dummy example project for SoupRune.

| English | Simplified Chinese |
|---------|--------------------|
| English | [简体中文](./readme_zh-hans.md) |

## Introduction

This project demonstrates a concrete SoupRune mod built on top of `undertale_preset`.
It includes an overworld entry flow, a Mad Dummy battle, localized dialogue, RON-authored assets, and WASM runtime/content guests.

Use this repository when you want to study a complete runnable project. Use `souprune_undertale_preset` when you need the reusable Undertale-style preset library.

## How to Use

The recommended path is to clone the main SoupRune repository and initialize submodules:

```bash
git clone https://github.com/Bli-AIk/souprune.git
cd souprune
git submodule update --init --recursive
```

This repository is mounted by the main project at:

```text
projects/mad_dummy_example
```

To make it the active project, use:

```toml
[project]
mod_name = "mad_dummy_example"
language = "en-US"
```

## Mod Structure

```text
mad_dummy_example/
├── mod.toml
├── runtime/
├── content/
├── .build/
├── app/
├── battle/
├── overworld/
├── actors/
├── narrative/
├── view/
└── assets/
```

## License and Asset Notice

The original code, configuration, and scripts in this repository are licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

This license applies only to original repository code, configuration, scripts, and other original contributions.
Undertale/Deltarune-related characters, names, visual assets, audio assets, and other original-game materials remain the property of their respective rights holders.
This repository is a fangame-development example and does not grant any rights to Undertale or Deltarune assets.
