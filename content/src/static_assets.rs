//! Canonical static RON assets emitted by the content guest.
//!
//! 由 content guest 发射的 canonical 静态 RON 资产。

use anyhow::Result;
use souprune_vessel::prelude::*;

/// Emit all static canonical RON assets owned by this mod.
///
/// 发射当前 mod 拥有的全部静态 canonical RON 资产。
pub fn emit_all(reg: &mut Registry) -> Result<()> {
    reg.emit_canonical_source(
        "actors/enemies/mad_dummy.enemy.ron",
        include_str!("../ron/actors/enemies/mad_dummy.enemy.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/chapters/demo.sequence.ron",
        include_str!("../ron/battle/chapters/demo.sequence.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/chapters/demo_turn_narration.sequence.ron",
        include_str!("../ron/battle/chapters/demo_turn_narration.sequence.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/rules/dialogue_test.fre.ron",
        include_str!("../ron/battle/rules/dialogue_test.fre.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/turns/cotton_bottom_wave.sequence.ron",
        include_str!("../ron/battle/turns/cotton_bottom_wave.sequence.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/turns/cotton_first_turn.sequence.ron",
        include_str!("../ron/battle/turns/cotton_first_turn.sequence.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/turns/cotton_side_pincer.sequence.ron",
        include_str!("../ron/battle/turns/cotton_side_pincer.sequence.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/turns/cotton_surround.sequence.ron",
        include_str!("../ron/battle/turns/cotton_surround.sequence.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/turns/cotton_top_sweep.sequence.ron",
        include_str!("../ron/battle/turns/cotton_top_sweep.sequence.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/view/battle_bg.view.ron",
        include_str!("../ron/battle/view/battle_bg.view.ron"),
    )?;
    reg.emit_canonical_source(
        "battle/view/mad_dummy.view.ron",
        include_str!("../ron/battle/view/mad_dummy.view.ron"),
    )?;
    reg.emit_canonical_source(
        "overworld/characters/frisk/animations.animation_config.ron",
        include_str!("../ron/overworld/characters/frisk/animations.animation_config.ron"),
    )?;
    reg.emit_canonical_source(
        "overworld/characters/frisk.character.ron",
        include_str!("../ron/overworld/characters/frisk.character.ron"),
    )?;
    reg.emit_canonical_source(
        "overworld/chase_config.ron",
        include_str!("../ron/overworld/chase_config.ron"),
    )?;
    reg.emit_canonical_source(
        "overworld/entry.sequence.ron",
        include_str!("../ron/overworld/entry.sequence.ron"),
    )?;
    reg.emit_canonical_source(
        "overworld/rules/demo.fre.ron",
        include_str!("../ron/overworld/rules/demo.fre.ron"),
    )?;
    Ok(())
}
