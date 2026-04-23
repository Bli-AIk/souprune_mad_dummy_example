//! Code representation of `battle/rules/dialogue_test.fre.ron`.
//!
//! `battle/rules/dialogue_test.fre.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::Local,
        enums: vec![].into_iter().collect(),
        facts: vec![
            ("battle:intro_shown".into(), FactValueDef::Bool(false)),
            ("test_mortar_shown".into(), FactValueDef::Bool(false)),
        ]
        .into_iter()
        .collect(),
        rules: vec![
            RuleDef {
                id: "battle_encounter_intro".into(),
                event: RuleEventDef::Event("battle:show_intro".into()),
                conditions: vec!["$battle:intro_shown == false".into()],
                actions: vec![
                    RuleActionDef::StartDialogue {
                        mortar: "battle/ui.mortar".into(),
                        node: "encounter_intro".into(),
                        view: None,
                        typewriter: true,
                        focus: false,
                        voice: Some("assets/audios/voice/voice_typewriter_default.wav".into()),
                    },
                    RuleActionDef::Log {
                        message: "Battle: Starting encounter intro".into(),
                    },
                ],
                modifications: vec![FreFactModificationDef::Set {
                    key: "battle:intro_shown".into(),
                    value: FactValueDef::Bool(true),
                }],
                outputs: vec![],
                enabled: true,
                priority: 100,
                consume_event: true,
            },
            RuleDef {
                id: "test_mortar_dialogue".into(),
                event: RuleEventDef::Event("test_mortar_dialogue".into()),
                conditions: vec!["$test_mortar_shown == false".into()],
                actions: vec![RuleActionDef::StartDialogue {
                    mortar: "dialogue/test/battle_test.mortar".into(),
                    node: "act_greeting".into(),
                    view: None,
                    typewriter: true,
                    focus: true,
                    voice: None,
                }],
                modifications: vec![FreFactModificationDef::Set {
                    key: "test_mortar_shown".into(),
                    value: FactValueDef::Bool(true),
                }],
                outputs: vec![],
                enabled: true,
                priority: 10,
                consume_event: true,
            },
            RuleDef {
                id: "battle_dialogue_started".into(),
                event: RuleEventDef::Event("dialogue:started".into()),
                conditions: vec![],
                actions: vec![],
                modifications: vec![FreFactModificationDef::Set {
                    key: "battle:paused_for_dialogue".into(),
                    value: FactValueDef::Bool(true),
                }],
                outputs: vec![],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
            RuleDef {
                id: "battle_dialogue_ended".into(),
                event: RuleEventDef::Event("dialogue:ended".into()),
                conditions: vec![],
                actions: vec![],
                modifications: vec![
                    FreFactModificationDef::Set {
                        key: "dialogue:has_focus".into(),
                        value: FactValueDef::Bool(false),
                    },
                    FreFactModificationDef::Set {
                        key: "battle:paused_for_dialogue".into(),
                        value: FactValueDef::Bool(false),
                    },
                    FreFactModificationDef::Remove("dialogue:pending_mortar_path".into()),
                    FreFactModificationDef::Remove("dialogue:pending_mortar_node".into()),
                    FreFactModificationDef::Remove("dialogue:pending_view".into()),
                ],
                outputs: vec![],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
            RuleDef {
                id: "act_with_dialogue".into(),
                event: RuleEventDef::Event("battle:execute_act".into()),
                conditions: vec!["$act_type == 'check'".into()],
                actions: vec![RuleActionDef::StartDialogue {
                    mortar: "dialogue/enemies/dummy.mortar".into(),
                    node: "check".into(),
                    view: None,
                    typewriter: true,
                    focus: true,
                    voice: None,
                }],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 10,
                consume_event: true,
            },
        ],
    }
}
