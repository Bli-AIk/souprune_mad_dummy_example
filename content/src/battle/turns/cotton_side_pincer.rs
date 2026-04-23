//! Code representation of `battle/turns/cotton_side_pincer.sequence.ron`.
//!
//! `battle/turns/cotton_side_pincer.sequence.ron` 的代码表示。

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
                target: TweenTarget::box_size(Vec2Tuple::positional(130.0, Val::expr("@current"))),
                duration: Some(0.85),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: false,
            },
            Chapter::DanmakuPerformance {
                performance: "battle/danmaku/cotton_side_pincer.performance.ron".into(),
                translation: Some((0.0, 50.0)),
            },
            Chapter::SetViewElement {
                selector: ElementSelector::local("BattleBox"),
                target: TweenTarget::box_size(Vec2Tuple::positional(566.0, 130.0)),
                duration: Some(0.6),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: false,
            },
        ],
    }
}
