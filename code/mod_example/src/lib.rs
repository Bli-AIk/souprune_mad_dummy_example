//! Example mod demonstrating the Souprune modding API.
//! Contains player behaviors (RedSoul) and danmaku behaviors (AimedSpear, SpiralHoming, etc.).
//!
//! 示例模组，演示 Souprune 模组 API。
//! 包含玩家行为（红魂）和弹幕行为（自机狙长矛、螺旋追踪等）。

mod behaviors;

use behaviors::{AimedSpear, GravityDropDanmaku, RedSoul, SpiralHomingDanmaku, WaveBurstDanmaku};
use souprune_sdk::prelude::*;

export_mod! {
    behaviors: [
        ("soul_red", RedSoul, || RedSoul::new()),
    ],
    danmaku: [
        ("aimed_spear", AimedSpear, || AimedSpear::new()),
        ("spiral_homing", SpiralHomingDanmaku, || SpiralHomingDanmaku::new()),
        ("wave_burst", WaveBurstDanmaku, || WaveBurstDanmaku::new()),
        ("gravity_drop", GravityDropDanmaku, || GravityDropDanmaku::new()),
    ],
}
