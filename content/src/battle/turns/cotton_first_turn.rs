//! Code representation of `battle/turns/cotton_first_turn.sequence.ron`.
//!
//! `battle/turns/cotton_first_turn.sequence.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_schema::val::*;
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
                target: TweenTarget::box_size(Vec2Tuple::positional(175.0, Val::expr("@current"))),
                duration: Some(0.85),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: true,
            },
            Chapter::SetViewElement {
                selector: ElementSelector::local("BattleBox"),
                target: TweenTarget::box_size(Vec2Tuple::positional(Val::expr("@current"), 180.0)),
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
                target: TweenTarget::box_size(Vec2Tuple::positional(566.0, 130.0)),
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
