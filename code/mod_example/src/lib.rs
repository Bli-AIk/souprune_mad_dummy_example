//! Example mod — mod-specific danmaku behaviors.
//! RedSoul and FightBar are provided by `undertale_preset` dependency.
//!
//! 示例模组 — 模组特定弹幕行为。
//! RedSoul 和 FightBar 由 `undertale_preset` 依赖提供。

mod behaviors;

use behaviors::{AimedSpear, GravityDropDanmaku, SpiralHomingDanmaku, WaveBurstDanmaku};
use souprune_sdk::prelude::*;

export_mod! {
    behaviors: [],
    danmaku: [
        ("aimed_spear", AimedSpear, || AimedSpear::new()),
        ("spiral_homing", SpiralHomingDanmaku, || SpiralHomingDanmaku::new()),
        ("wave_burst", WaveBurstDanmaku, || WaveBurstDanmaku::new()),
        ("gravity_drop", GravityDropDanmaku, || GravityDropDanmaku::new()),
    ],
}
