//! Code representation of `overworld/entry.sequence.ron`.
//!
//! `overworld/entry.sequence.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

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
