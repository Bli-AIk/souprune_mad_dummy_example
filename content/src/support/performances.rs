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

fn dummy_prototype(visual: &str, lifetime: f32) -> BulletPrototype {
    BulletPrototype {
        visual: visual.into(),
        collider: ColliderShape::circle(0.1),
        damage: 0.0,
        lifetime,
        z_index: 14.0,
        ..Default::default()
    }
}

fn cotton_prototype() -> BulletPrototype {
    BulletPrototype {
        visual: "battle/bullets/dummy_shot".into(),
        collider: ColliderShape::rectangle(5.0, 5.0),
        damage: 5.0,
        lifetime: 4.0,
        z_index: 15.0,
        frame_duration: Some(0.08),
        ..Default::default()
    }
}

fn relative_tween(
    target: DanmakuTweenTarget,
    duration: f32,
    ease: Easing,
    to: f32,
    delay: f32,
) -> BulletBehavior {
    BulletBehavior::Tween(TweenConfig {
        target,
        duration,
        ease,
        from: 0.0,
        to,
        delay,
        mode: TweenMode::Relative,
    })
}

fn battle_box_edge_event(
    t: f32,
    spawn: &str,
    side: EdgeSide,
    count: usize,
    spacing: f32,
    outside_margin: f32,
) -> TimelineEvent {
    TimelineEvent::absolute(
        t,
        spawn,
        SpawnPattern::box_edge("BattleBox", side, count, spacing, outside_margin),
        std::iter::empty::<&str>(),
    )
}

/// Build the `cotton_top_sweep` performance.
///
/// 构建 `cotton_top_sweep` 演出。
pub fn cotton_top_sweep() -> DanmakuPerformance {
    performance! {
        prototypes {
            "dummy" => dummy_prototype("battle/bullets/dummy_bullet/down.png", 3.0),
            "cotton" => cotton_prototype(),
        }
        behaviors {
            "aim_heart" => BulletBehavior::aimed(90.0),
            "enter_from_top" => relative_tween(
                DanmakuTweenTarget::PositionY,
                0.7,
                Easing::QuadOut,
                -170.0,
                0.0,
            ),
            "exit_to_top" => relative_tween(
                DanmakuTweenTarget::PositionY,
                0.5,
                Easing::QuadIn,
                170.0,
                1.3,
            ),
        }
        timeline [
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
        ]
        duration: DurationExpr::Expr("@current + 4.0".into()),
    }
}

/// Build the `cotton_surround` performance.
///
/// 构建 `cotton_surround` 演出。
pub fn cotton_surround() -> DanmakuPerformance {
    performance! {
        prototypes {
            "dummy_right" => dummy_prototype("battle/bullets/dummy_bullet/right.png", 3.0),
            "dummy_left" => dummy_prototype("battle/bullets/dummy_bullet/left.png", 3.0),
            "dummy_down" => dummy_prototype("battle/bullets/dummy_bullet/down.png", 3.0),
            "dummy_up" => dummy_prototype("battle/bullets/dummy_bullet/up.png", 3.0),
            "cotton" => cotton_prototype(),
        }
        behaviors {
            "aim_heart" => BulletBehavior::aimed(90.0),
            "enter_from_left" => relative_tween(
                DanmakuTweenTarget::PositionX,
                0.7,
                Easing::QuadOut,
                170.0,
                0.0,
            ),
            "exit_to_left" => relative_tween(
                DanmakuTweenTarget::PositionX,
                0.5,
                Easing::QuadIn,
                -170.0,
                1.3,
            ),
            "enter_from_right" => relative_tween(
                DanmakuTweenTarget::PositionX,
                0.7,
                Easing::QuadOut,
                -170.0,
                0.0,
            ),
            "exit_to_right" => relative_tween(
                DanmakuTweenTarget::PositionX,
                0.5,
                Easing::QuadIn,
                170.0,
                1.3,
            ),
            "enter_from_top" => relative_tween(
                DanmakuTweenTarget::PositionY,
                0.7,
                Easing::QuadOut,
                -170.0,
                0.0,
            ),
            "exit_to_top" => relative_tween(
                DanmakuTweenTarget::PositionY,
                0.5,
                Easing::QuadIn,
                170.0,
                1.3,
            ),
            "enter_from_bottom" => relative_tween(
                DanmakuTweenTarget::PositionY,
                0.7,
                Easing::QuadOut,
                170.0,
                0.0,
            ),
            "exit_to_bottom" => relative_tween(
                DanmakuTweenTarget::PositionY,
                0.5,
                Easing::QuadIn,
                -170.0,
                1.3,
            ),
        }
        timeline [
            TimelineEvent::delta(
                0.0,
                "dummy_right",
                SpawnPattern::edge(EdgeSide::Left, 2, 40.0, 250.0),
                ["enter_from_left", "exit_to_left"],
            ),
            TimelineEvent::delta(
                0.0,
                "dummy_left",
                SpawnPattern::edge(EdgeSide::Right, 2, 40.0, 250.0),
                ["enter_from_right", "exit_to_right"],
            ),
            TimelineEvent::delta(
                0.0,
                "dummy_down",
                SpawnPattern::edge(EdgeSide::Top, 5, 25.0, 250.0),
                ["enter_from_top", "exit_to_top"],
            ),
            TimelineEvent::delta(
                0.0,
                "dummy_up",
                SpawnPattern::edge(EdgeSide::Bottom, 5, 25.0, 250.0),
                ["enter_from_bottom", "exit_to_bottom"],
            ),
            TimelineEvent::delta(
                1.0,
                "cotton",
                SpawnPattern::edge(EdgeSide::Left, 2, 40.0, 80.0),
                ["aim_heart"],
            ),
            TimelineEvent::delta(
                0.0,
                "cotton",
                SpawnPattern::edge(EdgeSide::Right, 2, 40.0, 80.0),
                ["aim_heart"],
            ),
            TimelineEvent::delta(
                0.0,
                "cotton",
                SpawnPattern::edge(EdgeSide::Top, 5, 25.0, 80.0),
                ["aim_heart"],
            ),
            TimelineEvent::delta(
                0.0,
                "cotton",
                SpawnPattern::edge(EdgeSide::Bottom, 5, 25.0, 80.0),
                ["aim_heart"],
            ),
        ]
        duration: DurationExpr::Expr("@current + 6.0".into()),
    }
}

/// Build the `cotton_side_pincer` performance.
///
/// 构建 `cotton_side_pincer` 演出。
pub fn cotton_side_pincer() -> DanmakuPerformance {
    performance! {
        prototypes {
            "dummy_right" => dummy_prototype("battle/bullets/dummy_bullet/right.png", 3.0),
            "dummy_left" => dummy_prototype("battle/bullets/dummy_bullet/left.png", 3.0),
            "cotton" => cotton_prototype(),
        }
        behaviors {
            "aim_heart" => BulletBehavior::aimed(90.0),
            "enter_from_left" => relative_tween(
                DanmakuTweenTarget::PositionX,
                0.7,
                Easing::QuadOut,
                170.0,
                0.0,
            ),
            "exit_to_left" => relative_tween(
                DanmakuTweenTarget::PositionX,
                0.5,
                Easing::QuadIn,
                -170.0,
                1.3,
            ),
            "enter_from_right" => relative_tween(
                DanmakuTweenTarget::PositionX,
                0.7,
                Easing::QuadOut,
                -170.0,
                0.0,
            ),
            "exit_to_right" => relative_tween(
                DanmakuTweenTarget::PositionX,
                0.5,
                Easing::QuadIn,
                170.0,
                1.3,
            ),
        }
        timeline [
            TimelineEvent::delta(
                0.0,
                "dummy_right",
                SpawnPattern::edge(EdgeSide::Left, 5, 25.0, 250.0),
                ["enter_from_left", "exit_to_left"],
            ),
            TimelineEvent::delta(
                0.0,
                "dummy_left",
                SpawnPattern::edge(EdgeSide::Right, 5, 25.0, 250.0),
                ["enter_from_right", "exit_to_right"],
            ),
            TimelineEvent::delta(
                1.0,
                "cotton",
                SpawnPattern::edge(EdgeSide::Left, 5, 25.0, 80.0),
                ["aim_heart"],
            ),
            TimelineEvent::delta(
                0.0,
                "cotton",
                SpawnPattern::edge(EdgeSide::Right, 5, 25.0, 80.0),
                ["aim_heart"],
            ),
        ]
        duration: DurationExpr::Expr("@current + 5.0".into()),
    }
}

/// Build the `cotton_bottom_wave` performance.
///
/// 构建 `cotton_bottom_wave` 演出。
pub fn cotton_bottom_wave() -> DanmakuPerformance {
    performance! {
        prototypes {
            "dummy" => dummy_prototype("battle/bullets/dummy_bullet/up.png", 3.5),
            "cotton" => cotton_prototype(),
        }
        behaviors {
            "aim_heart" => BulletBehavior::aimed(90.0),
            "enter_row1" => relative_tween(
                DanmakuTweenTarget::PositionY,
                0.7,
                Easing::QuadOut,
                150.0,
                0.0,
            ),
            "exit_row1" => relative_tween(
                DanmakuTweenTarget::PositionY,
                0.5,
                Easing::QuadIn,
                -150.0,
                1.3,
            ),
            "enter_row2" => relative_tween(
                DanmakuTweenTarget::PositionY,
                0.7,
                Easing::QuadOut,
                120.0,
                0.0,
            ),
            "exit_row2" => relative_tween(
                DanmakuTweenTarget::PositionY,
                0.5,
                Easing::QuadIn,
                -120.0,
                1.3,
            ),
        }
        timeline [
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
        ]
        duration: DurationExpr::Expr("@current + 4.85".into()),
    }
}

/// Build the reference `cotton_first_turn` performance.
///
/// 构建参考版 `cotton_first_turn` 演出。
pub fn cotton_first_turn() -> DanmakuPerformance {
    let mut top_behaviors = HashMap::new();
    let mut bot_behaviors = HashMap::new();

    for (index, &margin) in TOP_MARGINS.iter().enumerate() {
        let travel = round_2(margin + TOP_PEN);
        let enter_duration = round_2(travel / SPEED);
        let fire_time = round_2(enter_duration + PAUSE);

        top_behaviors.insert(
            format!("top{}_enter", index + 1),
            BulletBehavior::tween(
                DanmakuTweenTarget::PositionY,
                enter_duration,
                Easing::Linear,
                -travel,
            ),
        );
        top_behaviors.insert(
            format!("top{}_exit", index + 1),
            BulletBehavior::tween_delayed(
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

        bot_behaviors.insert(
            format!("bot{}_enter", index + 1),
            BulletBehavior::tween(
                DanmakuTweenTarget::PositionY,
                enter_duration,
                Easing::Linear,
                travel,
            ),
        );
        bot_behaviors.insert(
            format!("bot{}_exit", index + 1),
            BulletBehavior::tween_delayed(
                DanmakuTweenTarget::PositionY,
                EXIT_DURATION,
                Easing::Linear,
                -EXIT_DISTANCE,
                fire_time,
            ),
        );
    }

    let mut top_dummy_events = Vec::new();

    for &_margin in &TOP_MARGINS {
        top_dummy_events.push(battle_box_edge_event(
            0.0,
            "dummy_down",
            EdgeSide::Top,
            3,
            55.0,
            0.0,
        ));
    }

    let mut bot_dummy_events = Vec::new();
    for &margin in &BOT_MARGINS {
        bot_dummy_events.push(battle_box_edge_event(
            0.0,
            "dummy_up",
            EdgeSide::Bottom,
            6,
            20.0,
            margin,
        ));
    }

    let mut top_fire_events = Vec::new();
    for &margin in &TOP_MARGINS {
        let enter_duration = round_2((margin + TOP_PEN) / SPEED);
        let fire_time = round_2(enter_duration + PAUSE);
        top_fire_events.push(battle_box_edge_event(
            fire_time,
            "cotton",
            EdgeSide::Top,
            3,
            55.0,
            -TOP_PEN,
        ));
    }

    let mut bot_fire_events = Vec::new();
    for &margin in &BOT_MARGINS {
        let enter_duration = round_2((margin + BOT_PEN) / SPEED);
        let fire_time = round_2(enter_duration + PAUSE);
        bot_fire_events.push(battle_box_edge_event(
            fire_time,
            "cotton",
            EdgeSide::Bottom,
            6,
            20.0,
            -BOT_PEN,
        ));
    }

    performance! {
        prototypes {
            "dummy_down" => dummy_prototype("battle/bullets/dummy_bullet/down.png", 6.0),
            "dummy_up" => dummy_prototype("battle/bullets/dummy_bullet/up.png", 6.0),
            "cotton" => cotton_prototype(),
        }
        behaviors {
            "aim_heart" => BulletBehavior::aimed(COTTON_SPEED),
            ..top_behaviors,
            ..bot_behaviors,
        }
        timeline [
            ..top_dummy_events,
            ..bot_dummy_events,
            ..top_fire_events,
            ..bot_fire_events,
        ]
        duration: DurationExpr::Expr("@current + 8.0".into()),
    }
}
