//! Auto-mapped content asset for `battle/danmaku/cotton_bottom_wave.performance.ron`.
//!
//! `battle/danmaku/cotton_bottom_wave.performance.ron` 的自动映射内容资源。

use anyhow::Result;
use souprune_schema::danmaku::*;
use souprune_vessel::prelude::*;
use std::collections::HashMap;

use crate::support::cotton_danmaku::{cotton_prototype, dummy_prototype, relative_tween};

/// Emit this asset through convention-first path mapping.
///
/// 通过“约定优先”的路径映射生成该资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
pub fn asset() -> DanmakuPerformance {
    DanmakuPerformance {
        prototypes: HashMap::from([
            (
                "dummy".to_string(),
                dummy_prototype("battle/bullets/dummy_bullet/up.png", 3.5),
            ),
            ("cotton".to_string(), cotton_prototype()),
        ]),
        behaviors: HashMap::from([
            ("aim_heart".to_string(), BulletBehavior::aimed(90.0)),
            (
                "enter_row1".to_string(),
                relative_tween(
                    DanmakuTweenTarget::PositionY,
                    0.7,
                    Easing::OutQuad,
                    150.0,
                    0.0,
                ),
            ),
            (
                "exit_row1".to_string(),
                relative_tween(
                    DanmakuTweenTarget::PositionY,
                    0.5,
                    Easing::InQuad,
                    -150.0,
                    1.3,
                ),
            ),
            (
                "enter_row2".to_string(),
                relative_tween(
                    DanmakuTweenTarget::PositionY,
                    0.7,
                    Easing::OutQuad,
                    120.0,
                    0.0,
                ),
            ),
            (
                "exit_row2".to_string(),
                relative_tween(
                    DanmakuTweenTarget::PositionY,
                    0.5,
                    Easing::InQuad,
                    -120.0,
                    1.3,
                ),
            ),
        ]),
        timeline: vec![
            TimelineEvent::delta(
                0.0,
                "dummy",
                SpawnPattern::edge(EdgeSide::Bottom, 9, 22.0, 230.0),
                ["enter_row1", "exit_row1"],
            ),
            TimelineEvent::delta(
                0.15,
                "dummy",
                SpawnPattern::edge(EdgeSide::Bottom, 9, 22.0, 280.0),
                ["enter_row2", "exit_row2"],
            ),
            TimelineEvent::delta(
                0.85,
                "cotton",
                SpawnPattern::edge(EdgeSide::Bottom, 9, 22.0, 80.0),
                ["aim_heart"],
            ),
            TimelineEvent::delta(
                0.15,
                "cotton",
                SpawnPattern::edge(EdgeSide::Bottom, 9, 22.0, 110.0),
                ["aim_heart"],
            ),
        ],
        duration: Some(6.0),
    }
}
