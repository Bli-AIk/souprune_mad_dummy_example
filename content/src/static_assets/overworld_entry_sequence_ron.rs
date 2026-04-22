//! Bootstrapped code asset for `overworld/entry.sequence.ron`.
//!
//! `overworld/entry.sequence.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_ron("overworld/entry.sequence.ron", &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
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
