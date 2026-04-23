//! Bootstrapped code asset for `overworld/entry.sequence.ron`.
//!
//! `overworld/entry.sequence.ron` 的 bootstrap 代码资源。

use anyhow::Result;
use souprune_schema::sequence::*;
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
pub fn asset() -> SequenceAsset {
    SequenceAsset {
        mode: None,
        rules_file: None,
        exits: vec![].into_iter().collect(),
        chapters: vec![
            Chapter::LoadMap {
                path: "assets/maps/ruins/ruins_3.tmx".into(),
                generate_collision: true,
                process_objects: true,
            },
            Chapter::SetPlayer(PlayerAction::Spawn {
                config_path: "overworld/players/player_behavior.ron".into(),
                position: None,
            }),
        ],
    }
}
