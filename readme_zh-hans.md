# souprune_example_mods

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE) <img src="https://img.shields.io/github/repo-size/Bli-AIk/souprune_example_mods.svg"/> <img src="https://img.shields.io/github/last-commit/Bli-AIk/souprune_example_mods.svg"/>

**souprune_example_mods** — SoupRune 示例模组集合，演示大地图、战斗系统和 Alight Motion 动画集成。

| English         | Simplified Chinese                 |
|-----------------|-----------------------------------|
| [English](./readme.md) | 简体中文 |

## 简介

`souprune_example_mods` 是 SoupRune 游戏引擎的示例模组集合。  
它提供了开箱即用的模组模板，演示了：

- 基于瓦片的大地图探索玩法
- 战斗系统集成
- Alight Motion 动画支持

使用这些示例，你只需复制相应的模组结构并自定义内容。  
每个模组都在独立分支中维护，便于克隆。

## 包含的模组

| 分支 | 描述 |
|------|------|
| `mod/example_mod` | 完整示例，包含大地图探索和战斗序列 |
| `mod/example_battle_mod` | 战斗焦点示例，演示战斗系统 |
| `mod/example_am_mod` | Alight Motion 动画集成示例 |

## 如何使用

### 克隆特定模组

```bash
# 克隆 example_mod
git clone -b mod/example_mod --single-branch https://github.com/Bli-AIk/souprune_example_mods.git example_mod

# 克隆 example_battle_mod
git clone -b mod/example_battle_mod --single-branch https://github.com/Bli-AIk/souprune_example_mods.git example_battle_mod

# 克隆 example_am_mod
git clone -b mod/example_am_mod --single-branch https://github.com/Bli-AIk/souprune_example_mods.git example_am_mod
```

### 添加到 SoupRune 项目

1. 将克隆的目录放入 SoupRune 的 `projects/` 文件夹
2. 更新 `projects/config.toml` 指向你的模组：
   ```toml
   [project]
   mod_name = "example_mod"
   language = "en-US"
   ```

### 构建 C# 代码（如已修改）

```bash
cd example_mod
./build.sh        # Linux/macOS
./build.ps1       # Windows PowerShell
```

## 模组结构

```
example_mod/
├── mod.toml              # 模组配置
├── assets/               # 纹理、音频等资源
├── code/                 # C# 源代码
├── config/               # 输入和状态配置
├── shared/               # 共享资源（对话、物品、本地化）
├── states/               # 大地图和战斗状态
├── build.sh              # 构建脚本 (Unix)
├── build.ps1             # 构建脚本 (Windows)
└── global.fre.ron        # 全局 FRE 规则
```

## 贡献

欢迎贡献！
无论是修复 bug、添加功能还是改进文档：

* 提交 **Issue** 或 **Pull Request**
* 分享想法、讨论设计或架构

## 许可证

本项目采用以下任一许可证：

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) 或 [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) 或 [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

任选其一。
