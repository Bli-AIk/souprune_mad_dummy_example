//! Code representation of `overworld/rules/demo.fre.ron`.
//!
//! `overworld/rules/demo.fre.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::Local,
        enums: vec![].into_iter().collect(),
        facts: vec![
            ("button_1_pressed".into(), FactValueDef::Bool(false)),
            ("button_0_pressed".into(), FactValueDef::Bool(false)),
            ("demo_area_visit_count".into(), FactValueDef::Int(0)),
        ]
        .into_iter()
        .collect(),
        rules: vec![
            RuleDef {
                id: "demo_increment_visit".into(),
                event: RuleEventDef::Event("trigger_enter_demo_trigger".into()),
                conditions: vec![],
                actions: vec![],
                modifications: vec![FreFactModificationDef::Increment {
                    key: "demo_area_visit_count".into(),
                    amount: 1,
                }],
                outputs: vec!["demo_visit_updated".into()],
                enabled: true,
                priority: 10,
                consume_event: true,
            },
            RuleDef {
                id: "demo_enter_chase".into(),
                event: RuleEventDef::Event("demo_visit_updated".into()),
                conditions: vec!["$demo_area_visit_count == 2".into()],
                actions: vec![RuleActionDef::Custom {
                    action_type: "EnterChaseState".into(),
                    params: vec![].into_iter().collect(),
                }],
                modifications: vec![],
                outputs: vec!["chase_state_entered".into()],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
            RuleDef {
                id: "demo_play_danmaku".into(),
                event: RuleEventDef::Event("demo_visit_updated".into()),
                conditions: vec!["$demo_area_visit_count == 3".into()],
                actions: vec![RuleActionDef::Custom {
                    action_type: "PlayDanmaku".into(),
                    params: vec![(
                        "path".into(),
                        "overworld/danmaku/demo_attack_ow.performance.ron".into(),
                    )]
                    .into_iter()
                    .collect(),
                }],
                modifications: vec![],
                outputs: vec!["danmaku_played".into()],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
            RuleDef {
                id: "demo_exit_chase".into(),
                event: RuleEventDef::Event("demo_visit_updated".into()),
                conditions: vec!["$demo_area_visit_count == 5".into()],
                actions: vec![RuleActionDef::Custom {
                    action_type: "ExitChaseState".into(),
                    params: vec![].into_iter().collect(),
                }],
                modifications: vec![],
                outputs: vec!["chase_state_exited".into()],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
            RuleDef {
                id: "demo_start_battle".into(),
                event: RuleEventDef::Event("demo_visit_updated".into()),
                conditions: vec!["$demo_area_visit_count == 6".into()],
                actions: vec![RuleActionDef::Custom {
                    action_type: "SetMode".into(),
                    params: vec![("mode".into(), "battle".into())].into_iter().collect(),
                }],
                modifications: vec![],
                outputs: vec!["battle_started".into()],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
            RuleDef {
                id: "puzzle_button_0_press".into(),
                event: RuleEventDef::Event("interact_button_trigger_0".into()),
                conditions: vec!["$button_0_pressed != true".into()],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "SetSprite".into(),
                        params: vec![
                            ("target".into(), "button_0".into()),
                            (
                                "image".into(),
                                "assets/maps/ruins/tiles/objects/switch_1.png".into(),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::Log {
                        message: "Button 0 pressed".into(),
                    },
                ],
                modifications: vec![FreFactModificationDef::Set {
                    key: "button_0_pressed".into(),
                    value: FactValueDef::Bool(true),
                }],
                outputs: vec!["puzzle_state_changed".into()],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "puzzle_button_0_unpress".into(),
                event: RuleEventDef::Event("interact_button_trigger_0".into()),
                conditions: vec!["$button_0_pressed == true".into()],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "SetSprite".into(),
                        params: vec![
                            (
                                "image".into(),
                                "assets/maps/ruins/tiles/objects/switch_0.png".into(),
                            ),
                            ("target".into(), "button_0".into()),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::Log {
                        message: "Button 0 unpressed".into(),
                    },
                ],
                modifications: vec![FreFactModificationDef::Set {
                    key: "button_0_pressed".into(),
                    value: FactValueDef::Bool(false),
                }],
                outputs: vec!["puzzle_state_changed".into()],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "puzzle_button_1_press".into(),
                event: RuleEventDef::Event("interact_button_trigger_1".into()),
                conditions: vec!["$button_1_pressed != true".into()],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "SetSprite".into(),
                        params: vec![
                            (
                                "image".into(),
                                "assets/maps/ruins/tiles/objects/faceswitch_1.png".into(),
                            ),
                            ("target".into(), "button_1".into()),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::Log {
                        message: "Button 1 pressed".into(),
                    },
                ],
                modifications: vec![FreFactModificationDef::Set {
                    key: "button_1_pressed".into(),
                    value: FactValueDef::Bool(true),
                }],
                outputs: vec!["puzzle_state_changed".into()],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "puzzle_button_1_unpress".into(),
                event: RuleEventDef::Event("interact_button_trigger_1".into()),
                conditions: vec!["$button_1_pressed == true".into()],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "SetSprite".into(),
                        params: vec![
                            (
                                "image".into(),
                                "assets/maps/ruins/tiles/objects/faceswitch_0.png".into(),
                            ),
                            ("target".into(), "button_1".into()),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::Log {
                        message: "Button 1 unpressed".into(),
                    },
                ],
                modifications: vec![FreFactModificationDef::Set {
                    key: "button_1_pressed".into(),
                    value: FactValueDef::Bool(false),
                }],
                outputs: vec!["puzzle_state_changed".into()],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "puzzle_door_open".into(),
                event: RuleEventDef::Event("puzzle_state_changed".into()),
                conditions: vec![
                    "$button_0_pressed == true".into(),
                    "$button_1_pressed == true".into(),
                ],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "SetSprite".into(),
                        params: vec![
                            ("target".into(), "block_0".into()),
                            (
                                "image".into(),
                                "assets/maps/ruins/tiles/objects/spiketile_1.png".into(),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::Custom {
                        action_type: "SetSprite".into(),
                        params: vec![
                            ("target".into(), "block_1".into()),
                            (
                                "image".into(),
                                "assets/maps/ruins/tiles/objects/spiketile_1.png".into(),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::Custom {
                        action_type: "SetCollision".into(),
                        params: vec![
                            ("isEnabled".into(), "false".into()),
                            ("target".into(), "block_main".into()),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::Log {
                        message: "Door opened!".into(),
                    },
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 10,
                consume_event: true,
            },
            RuleDef {
                id: "puzzle_door_close".into(),
                event: RuleEventDef::Event("puzzle_state_changed".into()),
                conditions: vec!["$button_0_pressed + $button_1_pressed != 2".into()],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "SetSprite".into(),
                        params: vec![
                            (
                                "image".into(),
                                "assets/maps/ruins/tiles/objects/spiketile_0.png".into(),
                            ),
                            ("target".into(), "block_0".into()),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::Custom {
                        action_type: "SetSprite".into(),
                        params: vec![
                            (
                                "image".into(),
                                "assets/maps/ruins/tiles/objects/spiketile_0.png".into(),
                            ),
                            ("target".into(), "block_1".into()),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::Custom {
                        action_type: "SetCollision".into(),
                        params: vec![
                            ("target".into(), "block_main".into()),
                            ("isEnabled".into(), "true".into()),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::Log {
                        message: "Door closed".into(),
                    },
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
        ],
    }
}
