//! Bootstrapped code asset for `battle/view/mad_dummy.view.ron`.
//!
//! `battle/view/mad_dummy.view.ron` 的 bootstrap 代码资源。

use anyhow::Result;
use souprune_schema::val::*;
use souprune_schema::view::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 生成当前 bootstrap 资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
pub fn asset() -> ViewLayoutAsset {
    ViewLayout {
        roots: vec![
            ViewNodeDef {
                name: "MadDummy_Base".into(),
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
                    visual: Visual("assets/textures/battle/mad_dummy/base/0.png".into()),
                    initial_state: None,
                    color: None,
                    flip_x: false,
                    flip_y: false,
                    transform: Some(SerializableTransform {
                        translation: Some((Val::Static(5.0), Val::Static(-75.0), Val::Static(0.0))),
                        rotation: Some(Val::Expr("-sin(snap(@time, 1.0/30.0) * 4.0) * 3.0".into())),
                        scale: Some((Val::Static(2.0), Val::Static(2.0), Val::Static(2.0))),
                    }),
                    pivot: Some((Val::Static(0.476), Val::Static(0.0))),
                    frame_duration: None,
                    visible_when: None,
                    material: None,
                }),
                state_sprite: None,
                texts: vec![],
                view_box: None,
                children: vec![],
                repeat: None,
            },
            ViewNodeDef {
                name: "MadDummy_Legs".into(),
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
                    visual: Visual("assets/textures/battle/mad_dummy/leg/0.png".into()),
                    initial_state: None,
                    color: None,
                    flip_x: false,
                    flip_y: false,
                    transform: Some(SerializableTransform {
                        translation: Some((Val::Static(5.0), Val::Static(-85.0), Val::Static(0.1))),
                        rotation: Some(Val::Expr("sin(snap(@time, 1.0/30.0) * 4.0) * 1.0".into())),
                        scale: Some((Val::Static(2.0), Val::Static(2.0), Val::Static(2.0))),
                    }),
                    pivot: Some((Val::Static(0.48), Val::Static(0.462))),
                    frame_duration: None,
                    visible_when: None,
                    material: None,
                }),
                state_sprite: None,
                texts: vec![],
                view_box: None,
                children: vec![],
                repeat: None,
            },
            ViewNodeDef {
                name: "MadDummy_Torso".into(),
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
                    visual: Visual("assets/textures/battle/mad_dummy/torso/0.png".into()),
                    initial_state: None,
                    color: None,
                    flip_x: false,
                    flip_y: false,
                    transform: Some(SerializableTransform {
                        translation: Some((
                            Val::Static(0.0),
                            Val::Expr("-115.0 + sin(snap(@time, 1.0/30.0) * 4.0) * 0.75".into()),
                            Val::Static(0.2),
                        )),
                        rotation: Some(Val::Expr("sin(snap(@time, 1.0/30.0) * 4.0) * 1.5".into())),
                        scale: Some((Val::Static(2.0), Val::Static(2.0), Val::Static(2.0))),
                    }),
                    pivot: Some((Val::Static(0.474), Val::Static(0.5))),
                    frame_duration: None,
                    visible_when: None,
                    material: None,
                }),
                state_sprite: None,
                texts: vec![],
                view_box: None,
                children: vec![],
                repeat: None,
            },
            ViewNodeDef {
                name: "MadDummy_Head".into(),
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
                    visual: Visual("assets/textures/battle/mad_dummy/head/0.png".into()),
                    initial_state: None,
                    color: None,
                    flip_x: false,
                    flip_y: false,
                    transform: Some(SerializableTransform {
                        translation: Some((
                            Val::Expr("-sin(snap(@time, 1.0/30.0) * 4.0) * 1.0".into()),
                            Val::Expr("-150.0 + sin(snap(@time, 1.0/30.0) * 4.0) * 1.0".into()),
                            Val::Static(0.3),
                        )),
                        rotation: Some(Val::Expr("sin(snap(@time, 1.0/30.0) * 4.0) * 3.0".into())),
                        scale: Some((Val::Static(2.0), Val::Static(2.0), Val::Static(2.0))),
                    }),
                    pivot: Some((Val::Static(0.516), Val::Static(0.471))),
                    frame_duration: None,
                    visible_when: None,
                    material: None,
                }),
                state_sprite: None,
                texts: vec![],
                view_box: None,
                children: vec![],
                repeat: None,
            },
        ],
        requires: vec![],
        facts: None,
        world_space: true,
        coordinate_system: CoordinateSystem::YDown,
    }
}
