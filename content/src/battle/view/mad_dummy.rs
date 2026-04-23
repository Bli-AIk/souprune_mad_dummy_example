//! View asset for `battle/view/mad_dummy.view.ron`.
//!
//! `battle/view/mad_dummy.view.ron` 的 view 资源。

use anyhow::Result;
use souprune_schema::view::*;
use souprune_vessel::prelude::*;

/// Emit this asset.
///
/// 生成当前资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
pub fn asset() -> ViewLayoutAsset {
    ViewLayout {
        roots: Vec::from([
            ViewNodeDef {
                name: "MadDummy_Base".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/mad_dummy/base/0.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(5.0, -75.0, 0.0)),
                        rotation: Some(expression("-sin(snap(@time, 1.0/30.0) * 4.0) * 3.0")),
                        scale: Some(vector3(2.0, 2.0, 2.0)),
                        ..Default::default()
                    }),
                    pivot: Some(vector2(0.476, 0.0)),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "MadDummy_Legs".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/mad_dummy/leg/0.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(5.0, -85.0, 0.1)),
                        rotation: Some(expression("sin(snap(@time, 1.0/30.0) * 4.0) * 1.0")),
                        scale: Some(vector3(2.0, 2.0, 2.0)),
                        ..Default::default()
                    }),
                    pivot: Some(vector2(0.48, 0.462)),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "MadDummy_Torso".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/mad_dummy/torso/0.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3_value(
                            static_float(0.0),
                            expression("-115.0 + sin(snap(@time, 1.0/30.0) * 4.0) * 0.75"),
                            static_float(0.2),
                        )),
                        rotation: Some(expression("sin(snap(@time, 1.0/30.0) * 4.0) * 1.5")),
                        scale: Some(vector3(2.0, 2.0, 2.0)),
                        ..Default::default()
                    }),
                    pivot: Some(vector2(0.474, 0.5)),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "MadDummy_Head".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/mad_dummy/head/0.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3_value(
                            expression("-sin(snap(@time, 1.0/30.0) * 4.0) * 1.0"),
                            expression("-150.0 + sin(snap(@time, 1.0/30.0) * 4.0) * 1.0"),
                            static_float(0.3),
                        )),
                        rotation: Some(expression("sin(snap(@time, 1.0/30.0) * 4.0) * 3.0")),
                        scale: Some(vector3(2.0, 2.0, 2.0)),
                        ..Default::default()
                    }),
                    pivot: Some(vector2(0.516, 0.471)),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ]),
        world_space: true,
        coordinate_system: CoordinateSystem::YDown,
        ..Default::default()
    }
}
