//! Auto-mapped content asset for `battle/danmaku/cotton_first_turn.performance.ron`.
//!
//! `battle/danmaku/cotton_first_turn.performance.ron` 的自动映射内容资源。

use anyhow::Result;
use souprune_schema::danmaku::*;
use souprune_cauld_ron::prelude::*;
use std::collections::HashMap;

use crate::support::cotton_danmaku::{cotton_prototype, dummy_prototype};

const SPEED: f32 = 120.0;
const PAUSE_FRAMES: f32 = 20.0;
const PAUSE: f32 = PAUSE_FRAMES / 30.0;
const EXIT_DURATION: f32 = 1.0;
const EXIT_DISTANCE: f32 = 120.0;
const COTTON_SPEED: f32 = 90.0;
const TOP_PENETRATION: f32 = 4.0;
const BOTTOM_PENETRATION: f32 = 22.0;
const TOP_MARGINS: [f32; 3] = [45.0, 135.0, 225.0];
const BOTTOM_MARGINS: [f32; 3] = [310.0, 360.0, 410.0];

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
    let mut top_behaviors = HashMap::new();
    let mut bottom_behaviors = HashMap::new();

    for (index, &margin) in TOP_MARGINS.iter().enumerate() {
        let travel = margin + TOP_PENETRATION;
        let enter_duration = travel / SPEED;
        let fire_time = enter_duration + PAUSE;

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

    for (index, &margin) in BOTTOM_MARGINS.iter().enumerate() {
        let travel = margin + BOTTOM_PENETRATION;
        let enter_duration = travel / SPEED;
        let fire_time = enter_duration + PAUSE;

        bottom_behaviors.insert(
            format!("bot{}_enter", index + 1),
            BulletBehavior::tween(
                DanmakuTweenTarget::PositionY,
                enter_duration,
                Easing::Linear,
                travel,
            ),
        );
        bottom_behaviors.insert(
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
    for _ in &TOP_MARGINS {
        top_dummy_events.push(battle_box_edge_event(
            0.0,
            "dummy_down",
            EdgeSide::Top,
            3,
            55.0,
            0.0,
        ));
    }

    let mut bottom_dummy_events = Vec::new();
    for &margin in &BOTTOM_MARGINS {
        bottom_dummy_events.push(battle_box_edge_event(
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
        let enter_duration = (margin + TOP_PENETRATION) / SPEED;
        let fire_time = enter_duration + PAUSE;
        top_fire_events.push(battle_box_edge_event(
            fire_time,
            "cotton",
            EdgeSide::Top,
            3,
            55.0,
            -TOP_PENETRATION,
        ));
    }

    let mut bottom_fire_events = Vec::new();
    for &margin in &BOTTOM_MARGINS {
        let enter_duration = (margin + BOTTOM_PENETRATION) / SPEED;
        let fire_time = enter_duration + PAUSE;
        bottom_fire_events.push(battle_box_edge_event(
            fire_time,
            "cotton",
            EdgeSide::Bottom,
            6,
            20.0,
            -BOTTOM_PENETRATION,
        ));
    }

    let mut behaviors =
        HashMap::from([("aim_heart".to_string(), BulletBehavior::aimed(COTTON_SPEED))]);
    behaviors.extend(top_behaviors);
    behaviors.extend(bottom_behaviors);

    let mut timeline = Vec::new();
    timeline.extend(top_dummy_events);
    timeline.extend(bottom_dummy_events);
    timeline.extend(top_fire_events);
    timeline.extend(bottom_fire_events);

    DanmakuPerformance {
        prototypes: HashMap::from([
            (
                "dummy_down".to_string(),
                dummy_prototype("battle/bullets/dummy_bullet/down.png", 6.0),
            ),
            (
                "dummy_up".to_string(),
                dummy_prototype("battle/bullets/dummy_bullet/up.png", 6.0),
            ),
            ("cotton".to_string(), cotton_prototype()),
        ]),
        behaviors,
        timeline,
        duration: Some(12.27),
    }
}
