//! Auto-mapped content asset for `battle/danmaku/cotton_top_sweep.performance.ron`.
//!
//! `battle/danmaku/cotton_top_sweep.performance.ron` 的自动映射内容资源。

use anyhow::Result;
use souprune_schema::danmaku::*;
use souprune_cauld_ron::prelude::*;
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
                dummy_prototype("battle/bullets/dummy_bullet/down.png", 3.0),
            ),
            ("cotton".to_string(), cotton_prototype()),
        ]),
        behaviors: HashMap::from([
            ("aim_heart".to_string(), BulletBehavior::aimed(90.0)),
            (
                "enter_from_top".to_string(),
                relative_tween(
                    DanmakuTweenTarget::PositionY,
                    0.7,
                    Easing::OutQuad,
                    -170.0,
                    0.0,
                ),
            ),
            (
                "exit_to_top".to_string(),
                relative_tween(
                    DanmakuTweenTarget::PositionY,
                    0.5,
                    Easing::InQuad,
                    170.0,
                    1.3,
                ),
            ),
        ]),
        timeline: vec![
            TimelineEvent::delta(
                0.0,
                "dummy",
                SpawnPattern::edge(EdgeSide::Top, 9, 25.0, 250.0),
                ["enter_from_top", "exit_to_top"],
            ),
            TimelineEvent::delta(
                1.0,
                "cotton",
                SpawnPattern::edge(EdgeSide::Top, 9, 25.0, 80.0),
                ["aim_heart"],
            ),
        ],
        duration: Some(5.0),
    }
}
