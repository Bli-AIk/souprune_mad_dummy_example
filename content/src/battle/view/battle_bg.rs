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
    view_layout(vec![view_node("BattleBG").sprite(
        view_sprite("assets/textures/battle/bg/0.png")
            .translation(vector3(-305.5, -5.5, -1.0))
            .scale(vector3(1.0, 1.0, 1.0))
            .pivot(vector2(0.0, 0.0)),
    )])
    .world_space(true)
}
