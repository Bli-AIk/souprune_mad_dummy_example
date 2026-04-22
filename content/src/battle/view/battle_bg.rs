//! Bootstrapped code asset for `battle/view/battle_bg.view.ron`.
//!
//! `battle/view/battle_bg.view.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::val::*;
use souprune_schema::view::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> ViewLayoutAsset {
    ViewLayout {
        roots: vec![ViewNodeDef {
            name: "BattleBG".into(),
            tags: vec![],
            style: StyleDef {
                width: None,
                height: None,
                left: None,
                right: None,
                top: None,
                bottom: None,
                position_type: None,
                flex_direction: None,
                justify_content: None,
                align_items: None,
            },
            visible_when: None,
            background_color: None,
            border_color: None,
            image: None,
            sprite: Some(SpriteDef {
                visual: Visual("assets/textures/battle/bg/0.png".into()),
                initial_state: None,
                color: None,
                flip_x: false,
                flip_y: false,
                transform: Some(SerializableTransform {
                    translation: Some((Val::Static(-305.5), Val::Static(-5.5), Val::Static(-1.0))),
                    rotation: None,
                    scale: Some((Val::Static(1.0), Val::Static(1.0), Val::Static(1.0))),
                }),
                pivot: Some((Val::Static(0.0), Val::Static(0.0))),
                frame_duration: None,
                visible_when: None,
                material: None,
            }),
            state_sprite: None,
            texts: vec![],
            view_box: None,
            children: vec![],
            repeat: None,
        }],
        requires: vec![],
        facts: None,
        world_space: true,
        coordinate_system: CoordinateSystem::Standard,
    }
}
