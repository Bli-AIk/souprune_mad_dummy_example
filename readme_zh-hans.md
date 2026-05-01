# souprune_mad_dummy_example

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE) <img src="https://img.shields.io/github/repo-size/Bli-AIk/souprune_mad_dummy_example.svg"/> <img src="https://img.shields.io/github/last-commit/Bli-AIk/souprune_mad_dummy_example.svg"/>

**souprune_mad_dummy_example** 是 SoupRune 维护中的 Mad Dummy 示例项目。

| 英语 | 简体中文 |
|------|----------|
| [English](./readme.md) | 简体中文 |

## 简介

本项目演示了一个基于 `undertale_preset` 的完整 SoupRune mod。
它包含大地图入口流程、Mad Dummy 战斗、本地化对话、RON 编写的资源，以及 WASM runtime/content guest。

如果你想学习一个可运行的完整项目，请使用此仓库。如果你需要可复用的 Undertale 风格预设库，请使用 `souprune_undertale_preset`。

## 使用方法

推荐克隆主 SoupRune 仓库并初始化子模块：

```bash
git clone https://github.com/Bli-AIk/souprune.git
cd souprune
git submodule update --init --recursive
```

主项目会将此仓库挂载到：

```text
projects/mad_dummy_example
```

要将它设为当前项目，请使用：

```toml
[project]
mod_name = "mad_dummy_example"
language = "en-US"
```

## Mod 结构

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

## 许可证

本项目采用以下任一许可证：

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) 或 [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) 或 [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

任选其一。
