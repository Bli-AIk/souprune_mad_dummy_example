//! View asset for `battle/view/battle_bg.view.ron`.
//!
//! `battle/view/battle_bg.view.ron` 的 view 资源。

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
        roots: Vec::from([ViewNodeDef {
            name: "BattleBG".into(),
            sprite: Some(SpriteDef {
                visual: Visual("assets/textures/battle/bg/0.png".into()),
                transform: Some(SerializableTransform {
                    translation: Some(vector3(-305.5, -5.5, -1.0)),
                    scale: Some(vector3(1.0, 1.0, 1.0)),
                    ..Default::default()
                }),
                pivot: Some(vector2(0.0, 0.0)),
                ..Default::default()
            }),
            ..Default::default()
        }]),
        world_space: true,
        ..Default::default()
    }
}
