//! Bootstrapped code asset for `battle/chapters/demo_turn_narration.sequence.ron`.
//!
//! `battle/chapters/demo_turn_narration.sequence.ron` 的 bootstrap 代码资源。

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
                        key: "dialogue:pending_mortar_path".into(),
                        value: FactValueMatch::String("battle/ui.mortar".into()),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:pending_start".into(),
                        value: FactValueMatch::Bool(true),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:has_typewriter".into(),
                        value: FactValueMatch::Bool(true),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:has_focus".into(),
                        value: FactValueMatch::Bool(false),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:voice".into(),
                        value: FactValueMatch::String(
                            "assets/audios/voice/voice_typewriter_default.wav".into(),
                        ),
                    },
                ],
            },
        ],
    }
}
