//! Reference Vessel-generated performances for `example_mod`.
//!
//! `example_mod` 的参考 Vessel 生成演出。

use souprune_schema::danmaku::*;
use souprune_vessel::prelude::*;
use std::collections::HashMap;

/// Physics constants from the original UT engine.
/// UT 原版引擎的物理常量。
const SPEED: f32 = 120.0;
const PAUSE_FRAMES: f32 = 20.0;
const PAUSE: f32 = PAUSE_FRAMES / 30.0;
const EXIT_DURATION: f32 = 1.0;
const EXIT_DISTANCE: f32 = 120.0;
const COTTON_SPEED: f32 = 90.0;

/// Top groups: penetration depth inside box = 4px.
/// 顶部分组：进入盒内深度为 4px。
const TOP_PEN: f32 = 4.0;
/// Bottom groups: penetration depth inside box = 22px.
/// 底部分组：进入盒内深度为 22px。
const BOT_PEN: f32 = 22.0;

/// Top group outside margins.
/// 顶部分组的外侧距离。
const TOP_MARGINS: [f32; 3] = [45.0, 135.0, 225.0];
/// Bottom group outside margins.
/// 底部分组的外侧距离。
const BOT_MARGINS: [f32; 3] = [310.0, 360.0, 410.0];

/// Round to 2 decimal places to match the hand-written asset precision.
///
/// 保留两位小数，以匹配手写资源的精度。
fn round_2(value: f32) -> f32 {
    (value * 100.0).round() / 100.0
}

/// Build the reference `cotton_first_turn` performance.
///
/// 构建参考版 `cotton_first_turn` 演出。
pub fn cotton_first_turn() -> DanmakuPerformance {
    let mut prototypes = HashMap::new();
    let mut behaviors = HashMap::new();

    prototypes.insert(
        "dummy_down".into(),
        prototype("battle/bullets/dummy_bullet/down.png")
            .collider(circle(0.1))
            .damage(0.0)
            .lifetime(6.0)
            .z_index(14.0)
            .build(),
    );

    prototypes.insert(
        "dummy_up".into(),
        prototype("battle/bullets/dummy_bullet/up.png")
            .collider(circle(0.1))
            .damage(0.0)
            .lifetime(6.0)
            .z_index(14.0)
            .build(),
    );

    prototypes.insert(
        "cotton".into(),
        prototype("battle/bullets/dummy_shot")
            .collider(rect(5.0, 5.0))
            .damage(5.0)
            .lifetime(4.0)
            .z_index(15.0)
            .frame_duration(0.08)
            .build(),
    );

    behaviors.insert("aim_heart".into(), aimed(COTTON_SPEED));

    for (index, &margin) in TOP_MARGINS.iter().enumerate() {
        let travel = round_2(margin + TOP_PEN);
        let enter_duration = round_2(travel / SPEED);
        let fire_time = round_2(enter_duration + PAUSE);

        behaviors.insert(
            format!("top{}_enter", index + 1),
            tween(
                DanmakuTweenTarget::PositionY,
                enter_duration,
                Easing::Linear,
                -travel,
            ),
        );
        behaviors.insert(
            format!("top{}_exit", index + 1),
            tween_delayed(
                DanmakuTweenTarget::PositionY,
                EXIT_DURATION,
                Easing::Linear,
                EXIT_DISTANCE,
                fire_time,
            ),
        );
    }

    for (index, &margin) in BOT_MARGINS.iter().enumerate() {
        let travel = round_2(margin + BOT_PEN);
        let enter_duration = round_2(travel / SPEED);
        let fire_time = round_2(enter_duration + PAUSE);

        behaviors.insert(
            format!("bot{}_enter", index + 1),
            tween(
                DanmakuTweenTarget::PositionY,
                enter_duration,
                Easing::Linear,
                travel,
            ),
        );
        behaviors.insert(
            format!("bot{}_exit", index + 1),
            tween_delayed(
                DanmakuTweenTarget::PositionY,
                EXIT_DURATION,
                Easing::Linear,
                -EXIT_DISTANCE,
                fire_time,
            ),
        );
    }

    let mut timeline = Vec::new();

    for &_margin in &TOP_MARGINS {
        timeline.push(
            event_at(
                0.0,
                "dummy_down",
                box_edge("BattleBox", EdgeSide::Top, 3)
                    .spacing(55.0)
                    .outside(0.0)
                    .build(),
            )
            .build(),
        );
    }

    for &margin in &BOT_MARGINS {
        timeline.push(
            event_at(
                0.0,
                "dummy_up",
                box_edge("BattleBox", EdgeSide::Bottom, 6)
                    .spacing(20.0)
                    .outside(margin)
                    .build(),
            )
            .build(),
        );
    }

    for &margin in &TOP_MARGINS {
        let enter_duration = round_2((margin + TOP_PEN) / SPEED);
        let fire_time = round_2(enter_duration + PAUSE);
        timeline.push(
            event_at(
                fire_time,
                "cotton",
                box_edge("BattleBox", EdgeSide::Top, 3)
                    .spacing(55.0)
                    .outside(-TOP_PEN)
                    .build(),
            )
            .build(),
        );
    }

    for &margin in &BOT_MARGINS {
        let enter_duration = round_2((margin + BOT_PEN) / SPEED);
        let fire_time = round_2(enter_duration + PAUSE);
        timeline.push(
            event_at(
                fire_time,
                "cotton",
                box_edge("BattleBox", EdgeSide::Bottom, 6)
                    .spacing(20.0)
                    .outside(-BOT_PEN)
                    .build(),
            )
            .build(),
        );
    }

    DanmakuPerformance {
        prototypes,
        behaviors,
        timeline,
        duration: Some(DurationExpr::Expr("@current + 8.0".into())),
    }
}
