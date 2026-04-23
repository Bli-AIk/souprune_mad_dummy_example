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
                selector: ElementSelector::LocalName("BattleBox".into()),
                target: TweenTarget::Anchor(0.0, -1.0),
                duration: None,
                easing: EaseKindRepr::Linear,
                wait_for_completion: false,
            },
            Chapter::SetViewElement {
                selector: ElementSelector::LocalName("BattleBox".into()),
                target: TweenTarget::BoxSize {
                    from: None,
                    to: Vec2Tuple::Positional(Val::Static(175.0), Val::Expr("@current".into())),
                },
                duration: Some(0.85),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: true,
            },
            Chapter::SetViewElement {
                selector: ElementSelector::LocalName("BattleBox".into()),
                target: TweenTarget::BoxSize {
                    from: None,
                    to: Vec2Tuple::Positional(Val::Expr("@current".into()), Val::Static(180.0)),
                },
                duration: Some(0.85),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: true,
            },
            Chapter::DanmakuPerformance {
                performance: "battle/danmaku/cotton_first_turn.performance.ron".into(),
                translation: Some((0.0, 50.0)),
            },
            Chapter::SetViewElement {
                selector: ElementSelector::LocalName("BattleBox".into()),
                target: TweenTarget::BoxSize {
                    from: None,
                    to: Vec2Tuple::Positional(Val::Static(566.0), Val::Static(130.0)),
                },
                duration: Some(0.6),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: true,
            },
            Chapter::SetViewElement {
                selector: ElementSelector::LocalName("BattleBox".into()),
                target: TweenTarget::Anchor(0.0, 0.0),
                duration: None,
                easing: EaseKindRepr::Linear,
                wait_for_completion: false,
            },
        ],
    }
}
