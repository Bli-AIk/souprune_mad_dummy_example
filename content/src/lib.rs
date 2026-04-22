//! Vessel guest entry for generated content in `example_mod`.
//!
//! `example_mod` 生成内容的 Vessel guest 入口。

use anyhow::Result;
use souprune_vessel::prelude::*;

mod demo_attack;
mod performances;
mod static_assets;

vessel_guest! {
    fn build(reg: &mut Registry) -> Result<()> {
        static_assets::emit_all(&mut reg)?;
        reg.emit_ron(
            "battle/danmaku/demo_attack.performance.ron",
            &demo_attack::demo_attack(),
        )?;
        reg.emit_ron(
            "overworld/danmaku/demo_attack_ow.performance.ron",
            &demo_attack::demo_attack_overworld(),
        )?;
        reg.emit_ron(
            "battle/danmaku/cotton_top_sweep.performance.ron",
            &performances::cotton_top_sweep(),
        )?;
        reg.emit_ron(
            "battle/danmaku/cotton_surround.performance.ron",
            &performances::cotton_surround(),
        )?;
        reg.emit_ron(
            "battle/danmaku/cotton_side_pincer.performance.ron",
            &performances::cotton_side_pincer(),
        )?;
        reg.emit_ron(
            "battle/danmaku/cotton_bottom_wave.performance.ron",
            &performances::cotton_bottom_wave(),
        )?;
        reg.emit_ron(
            "battle/danmaku/cotton_first_turn.performance.ron",
            &performances::cotton_first_turn(),
        )?;
        Ok(())
    }
}
