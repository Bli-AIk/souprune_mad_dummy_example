//! Code representation of `battle/common/enemy_speech_bubble.sequence.ron`.
//!
//! `battle/common/enemy_speech_bubble.sequence.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_vessel::prelude::*;

use crate::support::enemy_speech::mad_dummy_manual_speech;

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
            Chapter::BattleSpeechBubble(mad_dummy_manual_speech("enemy_speech_manual_intro")),
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
