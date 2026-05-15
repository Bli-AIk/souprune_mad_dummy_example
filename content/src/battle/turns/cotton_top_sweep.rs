//! Code representation of `battle/turns/cotton_top_sweep.sequence.ron`.
//!
//! `battle/turns/cotton_top_sweep.sequence.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_cauld_ron::prelude::*;

use crate::support::battle_box::set_battle_box_bounds;

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
            set_battle_box_bounds(-0.5, -80.0, 130.0, 130.0, 0.85),
            Chapter::SetViewElement {
                selector: ElementSelector::local("BattleBox"),
                target: TweenTarget::box_size((130.0, expr::current())),
                duration: Some(0.85),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: false,
            },
            Chapter::DanmakuPerformance {
                performance: "battle/danmaku/cotton_top_sweep.performance.ron".into(),
                translation: Some((0.0, 50.0)),
            },
            set_battle_box_bounds(-0.5, -80.0, 566.0, 130.0, 0.6),
            Chapter::SetViewElement {
                selector: ElementSelector::local("BattleBox"),
                target: TweenTarget::box_size((566.0, 130.0)),
                duration: Some(0.6),
                easing: EaseKindRepr::InOutQuad,
                wait_for_completion: false,
            },
        ],
    }
}
