//! Code representation of `battle/chapters/demo_turn_narration.sequence.ron`.
//!
//! `battle/chapters/demo_turn_narration.sequence.ron` 的代码表示。

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
            Chapter::SetViewFact {
                key: "narration_visible".into(),
                value: FactValueMatch::Bool(true),
            },
            Chapter::SetViewFact {
                key: "dialogue:replay_on_resume".into(),
                value: FactValueMatch::Bool(true),
            },
            Chapter::RandomPick {
                candidates: vec![
                    Chapter::ModifyFact {
                        modifications: vec![FactModificationDef::Set {
                            key: "dialogue:pending_mortar_node".into(),
                            value: FactValueMatch::String("flavor_1".into()),
                        }],
                    },
                    Chapter::ModifyFact {
                        modifications: vec![FactModificationDef::Set {
                            key: "dialogue:pending_mortar_node".into(),
                            value: FactValueMatch::String("flavor_2".into()),
                        }],
                    },
                    Chapter::ModifyFact {
                        modifications: vec![FactModificationDef::Set {
                            key: "dialogue:pending_mortar_node".into(),
                            value: FactValueMatch::String("flavor_3".into()),
                        }],
                    },
                ],
                count: 1,
                allow_repeat: false,
            },
            Chapter::Conditional {
                condition: FactCondition::Equals {
                    key: "battle_turn_count".into(),
                    value: FactValueMatch::Int(0),
                },
                then_branch: Box::new(Chapter::ModifyFact {
                    modifications: vec![FactModificationDef::Set {
                        key: "dialogue:pending_mortar_node".into(),
                        value: FactValueMatch::String("encounter_intro".into()),
                    }],
                }),
                else_branch: None,
            },
            Chapter::ModifyFact {
                modifications: vec![
                    FactModificationDef::Set {
                        key: "dialogue:pending_channel".into(),
                        value: FactValueMatch::String("battle_narration".into()),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:pending_mortar_path".into(),
                        value: FactValueMatch::String("battle/ui.mortar".into()),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:pending_start".into(),
                        value: FactValueMatch::Bool(true),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:battle_narration:has_typewriter".into(),
                        value: FactValueMatch::Bool(true),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:battle_narration:has_focus".into(),
                        value: FactValueMatch::Bool(false),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:battle_narration:voice".into(),
                        value: FactValueMatch::String(
                            "assets/audios/voice/voice_typewriter_default.wav".into(),
                        ),
                    },
                ],
            },
        ],
    }
}
