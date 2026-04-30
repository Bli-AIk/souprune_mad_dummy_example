//! Code representation of `battle/turns/cotton_first_turn.sequence.ron`.
//!
//! `battle/turns/cotton_first_turn.sequence.ron` 的代码表示。

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
            Chapter::SetViewElement {
                selector: ElementSelector::local("BattleBox"),
                target: TweenTarget::Anchor(0.0, -1.0),
                duration: None,
                easing: EaseKindRepr::Linear,
                wait_for_completion: false,
            },
            Chapter::SetViewElement {
                selector: ElementSelector::local("BattleBox"),
                target: TweenTarget::box_size((175.0, expr::current())),
                duration: Some(0.85),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: true,
            },
            Chapter::Custom {
                action_type: "battle:speech_bubble".into(),
                params: vec![
                    ("channel".into(), "battle_enemy_speech".into()),
                    (
                        "mortar_path".into(),
                        "battle/enemies/mad_dummy.mortar".into(),
                    ),
                    ("mortar_node".into(), "enemy_speech_manual_intro".into()),
                    ("bubble_profile".into(), "mad_dummy_wide".into()),
                    ("advance_mode".into(), "Manual".into()),
                    ("hide_on_finish".into(), "true".into()),
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
            Chapter::SetViewElement {
                selector: ElementSelector::local("BattleBox"),
                target: TweenTarget::box_size((expr::current(), 180.0)),
                duration: Some(0.85),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: true,
            },
            Chapter::DanmakuPerformance {
                performance: "battle/danmaku/cotton_first_turn.performance.ron".into(),
                translation: Some((0.0, 50.0)),
            },
            Chapter::SetViewElement {
                selector: ElementSelector::local("BattleBox"),
                target: TweenTarget::box_size((566.0, 130.0)),
                duration: Some(0.6),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: true,
            },
            Chapter::SetViewElement {
                selector: ElementSelector::local("BattleBox"),
                target: TweenTarget::Anchor(0.0, 0.0),
                duration: None,
                easing: EaseKindRepr::Linear,
                wait_for_completion: false,
            },
        ],
    }
}
