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
    view_layout(vec![
        view_node("MadDummy_Base").sprite(
            view_sprite("assets/textures/battle/mad_dummy/base/0.png")
                .translation(vector3(5.0, -75.0, 0.0))
                .rotation(expression("-sin(snap(@time, 1.0/30.0) * 4.0) * 3.0"))
                .scale(vector3(2.0, 2.0, 2.0))
                .pivot(vector2(0.476, 0.0)),
        ),
        view_node("MadDummy_Legs").sprite(
            view_sprite("assets/textures/battle/mad_dummy/leg/0.png")
                .translation(vector3(5.0, -85.0, 0.1))
                .rotation(expression("sin(snap(@time, 1.0/30.0) * 4.0) * 1.0"))
                .scale(vector3(2.0, 2.0, 2.0))
                .pivot(vector2(0.48, 0.462)),
        ),
        view_node("MadDummy_Torso").sprite(
            view_sprite("assets/textures/battle/mad_dummy/torso/0.png")
                .translation(vector3(
                    0.0,
                    expression("-115.0 + sin(snap(@time, 1.0/30.0) * 4.0) * 0.75"),
                    0.2,
                ))
                .rotation(expression("sin(snap(@time, 1.0/30.0) * 4.0) * 1.5"))
                .scale(vector3(2.0, 2.0, 2.0))
                .pivot(vector2(0.474, 0.5)),
        ),
        view_node("MadDummy_Head").sprite(
            view_sprite("assets/textures/battle/mad_dummy/head/0.png")
                .translation(vector3(
                    expression("-sin(snap(@time, 1.0/30.0) * 4.0) * 1.0"),
                    expression("-150.0 + sin(snap(@time, 1.0/30.0) * 4.0) * 1.0"),
                    0.3,
                ))
                .rotation(expression("sin(snap(@time, 1.0/30.0) * 4.0) * 3.0"))
                .scale(vector3(2.0, 2.0, 2.0))
                .pivot(vector2(0.516, 0.471)),
        ),
    ])
    .world_space(true)
    .coordinate_system(CoordinateSystem::YDown)
}
