//! Code representation of `battle/turns/cotton_bottom_wave.sequence.ron`.
//!
//! `battle/turns/cotton_bottom_wave.sequence.ron` 的代码表示。

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
                target: TweenTarget::BoxSize {
                    from: None,
                    to: Vec2Tuple::Positional(Val::Static(130.0), Val::Expr("@current".into())),
                },
                duration: Some(0.85),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: false,
            },
            Chapter::DanmakuPerformance {
                performance: "battle/danmaku/cotton_bottom_wave.performance.ron".into(),
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
                wait_for_completion: false,
            },
        ],
    }
}
