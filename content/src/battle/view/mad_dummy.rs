//! View asset for `battle/view/mad_dummy.view.ron`.
//!
//! `battle/view/mad_dummy.view.ron` 的 view 资源。

use anyhow::Result;
use souprune_schema::view::*;
use souprune_cauld_ron::prelude::*;

/// Emit this asset.
///
/// 生成当前资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

fn dummy_sway() -> expr::Expression {
    expr::sin(expr::snap(expr::time(), expr::frame_step(30.0)) * 4.0)
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
pub fn asset() -> ViewLayoutAsset {
    ViewLayout {
        roots: Vec::from([ViewNodeDef {
            name: "obj_maddummy".into(),
            transform: Some(SerializableTransform {
                translation: Some(vector3(270.0, 80.0, 0.0)),
                ..Default::default()
            }),
            children: Vec::from([ViewNodeDef {
                name: "obj_maddum_drawer".into(),
                transform: Some(SerializableTransform {
                    translation: Some(vector3(50.0, 10.0, 0.0)),
                    ..Default::default()
                }),
                children: Vec::from([
                    ViewNodeDef {
                        name: "MadDummy_Base".into(),
                        sprite: Some(SpriteDef {
                            visual: Visual("assets/textures/battle/mad_dummy/base/0.png".into()),
                            transform: Some(SerializableTransform {
                                translation: Some(vector3(5.0, 75.0, 0.0)),
                                rotation: (-dummy_sway() * 3.0).into(),
                                scale: Some(vector3(2.0, 2.0, 2.0)),
                            }),
                            pivot: Some(vector2(0.476, 0.0)),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    ViewNodeDef {
                        name: "MadDummy_Torso".into(),
                        sprite: Some(SpriteDef {
                            visual: Visual("assets/textures/battle/mad_dummy/torso/0.png".into()),
                            transform: Some(SerializableTransform {
                                translation: Some(vector3(0.0, 35.0 + dummy_sway() * 0.75, 0.1)),
                                rotation: (dummy_sway() * 1.5).into(),
                                scale: Some(vector3(2.0, 2.0, 2.0)),
                            }),
                            pivot: Some(vector2(0.474, 0.5)),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    ViewNodeDef {
                        name: "MadDummy_Legs".into(),
                        sprite: Some(SpriteDef {
                            visual: Visual("assets/textures/battle/mad_dummy/leg/0.png".into()),
                            transform: Some(SerializableTransform {
                                translation: Some(vector3(5.0, 65.0, 0.2)),
                                rotation: (dummy_sway() * 1.0).into(),
                                scale: Some(vector3(2.0, 2.0, 2.0)),
                            }),
                            pivot: Some(vector2(0.48, 0.462)),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    ViewNodeDef {
                        name: "MadDummy_Head".into(),
                        sprite: Some(SpriteDef {
                            visual: Visual("assets/textures/battle/mad_dummy/head/0.png".into()),
                            transform: Some(SerializableTransform {
                                translation: Some(vector3(
                                    -dummy_sway() * 1.0,
                                    dummy_sway() * 1.0,
                                    0.3,
                                )),
                                rotation: (dummy_sway() * 3.0).into(),
                                scale: Some(vector3(2.0, 2.0, 2.0)),
                            }),
                            pivot: Some(vector2(0.516, 0.471)),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            }]),
            ..Default::default()
        }]),
        space: Some(ViewSpaceDef::World2d),
        coordinate_system: CoordinateSystem::Standard,
        coordinate_space: Some(CoordinateSpaceDef {
            axis_origin: vector2(0.0, 0.0),
            y_axis: YAxisDirectionDef::Down,
            rotation: RotationDirectionDef::CounterClockwise,
            extent: CoordinateExtentDef::Explicit((640.0, 480.0)),
        }),
        ..Default::default()
    }
}
