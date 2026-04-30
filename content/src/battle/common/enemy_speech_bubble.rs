//! Code representation of `battle/common/enemy_speech_bubble.sequence.ron`.
//!
//! `battle/common/enemy_speech_bubble.sequence.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_vessel::prelude::*;

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
            Chapter::Custom {
                action_type: "battle:speech_bubble".into(),
                params: vec![
                    ("channel".into(), "battle_enemy_speech".into()),
                    ("mortar_path".into(), "$_param_mortar_path".into()),
                    ("mortar_node".into(), "$_param_mortar_node".into()),
                    ("bubble_profile".into(), "mad_dummy_wide".into()),
                    ("advance_mode".into(), "$_param_advance_mode".into()),
                    ("duration".into(), "$_param_duration".into()),
                    ("hide_on_finish".into(), "$_param_hide_on_finish".into()),
                ]
                .into_iter()
                .collect(),
            },
            Chapter::AwaitFact {
                condition: "$dialogue:battle_enemy_speech:active == true".into(),
                local: false,
            },
            Chapter::AwaitFact {
                condition: "$dialogue:battle_enemy_speech:active == false".into(),
                local: false,
            },
        ],
    }
}
