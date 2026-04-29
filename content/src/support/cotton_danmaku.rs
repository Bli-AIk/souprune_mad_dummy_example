//! Shared helpers for cotton danmaku assets.
//!
//! cotton 弹幕资源的共享辅助函数。

use souprune_schema::danmaku::*;

pub(crate) fn dummy_prototype(visual: &str, lifetime: f32) -> BulletPrototype {
    BulletPrototype {
        visual: visual.into(),
        collider: ColliderShape::circle(0.1),
        damage: 0.0,
        lifetime,
        z_index: 14.0,
        ..Default::default()
    }
}

pub(crate) fn cotton_prototype() -> BulletPrototype {
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

pub(crate) fn relative_tween(
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
