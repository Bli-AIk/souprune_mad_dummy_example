//! Demo danmaku performance for `example_mod`.
//!
//! `example_mod` 的演示弹幕演出。

use souprune_schema::danmaku::*;
use souprune_vessel::prelude::*;

#[derive(Clone, Copy)]
struct DemoAttackStyle {
    scale: f32,
    z_index: f32,
}

fn custom_behavior(id: &str, props: &[(&str, f32)]) -> BulletBehavior {
    BulletBehavior::Custom {
        id: id.to_string(),
        props: props
            .iter()
            .map(|(key, value)| ((*key).to_string(), *value))
            .collect(),
    }
}

fn fade_in() -> BulletBehavior {
    BulletBehavior::Tween(TweenConfig {
        target: DanmakuTweenTarget::Opacity,
        duration: 2.0,
        ease: Easing::QuadOut,
        from: 0.0,
        to: 1.0,
        delay: 0.0,
        mode: TweenMode::Relative,
    })
}

fn scale_pop() -> BulletBehavior {
    BulletBehavior::Tween(TweenConfig {
        target: DanmakuTweenTarget::Scale,
        duration: 1.0,
        ease: Easing::QuadOut,
        from: 0.5,
        to: 1.0,
        delay: 0.0,
        mode: TweenMode::Relative,
    })
}

fn spear_prototype(style: DemoAttackStyle) -> BulletPrototype {
    BulletPrototype {
        visual: "spear".to_string(),
        collider: rect(3.0, 12.0),
        damage: 2.0,
        lifetime: 4.0,
        z_index: style.z_index,
        scale: style.scale,
        hit_behavior: HitBehaviorPreset::Persistent,
        ..Default::default()
    }
}

fn pellet_prototype(style: DemoAttackStyle) -> BulletPrototype {
    BulletPrototype {
        visual: "flowey_pellet".to_string(),
        collider: rect(6.0, 6.0),
        damage: 1.0,
        lifetime: 3.0,
        z_index: style.z_index,
        scale: style.scale,
        frame_duration: Some(0.05),
        ..Default::default()
    }
}

fn tinted_pellet(
    style: DemoAttackStyle,
    hit_behavior: HitBehaviorPreset,
    hex: &str,
) -> BulletPrototype {
    BulletPrototype {
        hit_behavior,
        color_tint: ColorTint {
            hex: hex.to_string(),
            rgba: None,
        },
        ..pellet_prototype(style)
    }
}

fn build_demo_attack(style: DemoAttackStyle) -> DanmakuPerformance {
    performance! {
        prototypes {
            "spear" => spear_prototype(style),
            "pellet" => pellet_prototype(style),
            "pellet_orange" => tinted_pellet(style, HitBehaviorPreset::DamageWhenStationary, "#FCA600"),
            "pellet_blue" => tinted_pellet(style, HitBehaviorPreset::DamageWhenMoving, "#40FEFE"),
        }
        behaviors {
            "move_right" => linear((1.0, 0.0), 200.0),
            "move_left" => linear((-1.0, 0.0), 200.0),
            "move_down" => linear((0.0, -1.0), 150.0),
            "spiral_in" => orbital(0.8, -60.0),
            "aimed" => custom_behavior("aimed_spear", &[("speed", 180.0), ("smoothness", 0.8)]),
            "fade_in" => fade_in(),
            "spiral_homing" => custom_behavior(
                "spiral_homing",
                &[
                    ("spiral_speed", 80.0),
                    ("angular_velocity", 3.0),
                    ("homing_strength", 0.5),
                    ("homing_delay", 0.5),
                ],
            ),
            "wave_burst" => custom_behavior(
                "wave_burst",
                &[
                    ("base_speed", 120.0),
                    ("wave_amplitude", 30.0),
                    ("wave_frequency", 4.0),
                    ("burst_time", 0.8),
                    ("burst_multiplier", 2.5),
                ],
            ),
            "gravity_drop" => custom_behavior(
                "gravity_drop",
                &[("gravity", 200.0), ("bounce_damping", 0.7)],
            ),
        }
        timeline [
            event_delta(0.0, "spear", edge(EdgeSide::Left, 5).spacing(35.0).margin(250.0).build())
                .apply(&["move_right", "fade_in"])
                .build(),
            event_delta(1.5, "pellet_orange", ring(12, 100.0).build())
                .apply(&["spiral_in", "fade_in"])
                .build(),
            event_delta(2.0, "spear", line(3).spacing(60.0).direction((0.0, -1.0)).build())
                .apply(&["aimed", "fade_in"])
                .build(),
            event_delta(1.5, "pellet_blue", ring(16, 80.0).start_angle(0.785).build())
                .apply(&["spiral_in"])
                .behaviors(vec![scale_pop()])
                .build(),
            event_delta(1.5, "spear", edge(EdgeSide::Right, 7).spacing(30.0).margin(250.0).build())
                .apply(&["move_left"])
                .build(),
            event_delta(2.0, "pellet_orange", ring(8, 60.0).build())
                .apply(&["spiral_homing", "fade_in"])
                .build(),
            event_delta(2.0, "spear", line(5).spacing(40.0).direction((0.0, -1.0)).build())
                .apply(&["wave_burst", "fade_in"])
                .build(),
            event_delta(2.0, "pellet_blue", line(6).spacing(50.0).direction((1.0, 0.0)).build())
                .apply(&["gravity_drop", "fade_in"])
                .build(),
        ]
    }
}

/// Build the demo attack performance used by `example_mod`.
///
/// 构建 `example_mod` 使用的演示攻击演出。
pub fn demo_attack() -> DanmakuPerformance {
    build_demo_attack(DemoAttackStyle {
        scale: 1.0,
        z_index: 15.0,
    })
}

/// Build the overworld-scaled demo attack performance used by `example_mod`.
///
/// 构建 `example_mod` 在大地图里使用的缩放版演示攻击演出。
pub fn demo_attack_overworld() -> DanmakuPerformance {
    build_demo_attack(DemoAttackStyle {
        scale: 0.5,
        z_index: 100.0,
    })
}
