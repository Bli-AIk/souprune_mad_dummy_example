//! Bootstrapped static asset emitters for this content guest.
//!
//! 当前内容 guest 的 bootstrap 静态资产发射模块。

use anyhow::Result;
use souprune_vessel::prelude::*;

#[path = "static_assets/actors_enemies_mad_dummy_enemy_ron.rs"]
mod actors_enemies_mad_dummy_enemy_ron;
#[path = "static_assets/battle_chapters_demo_sequence_ron.rs"]
mod battle_chapters_demo_sequence_ron;
#[path = "static_assets/battle_chapters_demo_turn_narration_sequence_ron.rs"]
mod battle_chapters_demo_turn_narration_sequence_ron;
#[path = "static_assets/battle_rules_dialogue_test_fre_ron.rs"]
mod battle_rules_dialogue_test_fre_ron;
#[path = "static_assets/battle_turns_cotton_bottom_wave_sequence_ron.rs"]
mod battle_turns_cotton_bottom_wave_sequence_ron;
#[path = "static_assets/battle_turns_cotton_first_turn_sequence_ron.rs"]
mod battle_turns_cotton_first_turn_sequence_ron;
#[path = "static_assets/battle_turns_cotton_side_pincer_sequence_ron.rs"]
mod battle_turns_cotton_side_pincer_sequence_ron;
#[path = "static_assets/battle_turns_cotton_surround_sequence_ron.rs"]
mod battle_turns_cotton_surround_sequence_ron;
#[path = "static_assets/battle_turns_cotton_top_sweep_sequence_ron.rs"]
mod battle_turns_cotton_top_sweep_sequence_ron;
#[path = "static_assets/battle_view_battle_bg_view_ron.rs"]
mod battle_view_battle_bg_view_ron;
#[path = "static_assets/battle_view_mad_dummy_view_ron.rs"]
mod battle_view_mad_dummy_view_ron;
#[path = "static_assets/overworld_characters_frisk_animations_animation_config_ron.rs"]
mod overworld_characters_frisk_animations_animation_config_ron;
#[path = "static_assets/overworld_characters_frisk_character_ron.rs"]
mod overworld_characters_frisk_character_ron;
#[path = "static_assets/overworld_chase_config_ron.rs"]
mod overworld_chase_config_ron;
#[path = "static_assets/overworld_entry_sequence_ron.rs"]
mod overworld_entry_sequence_ron;
#[path = "static_assets/overworld_rules_demo_fre_ron.rs"]
mod overworld_rules_demo_fre_ron;

/// Emit all bootstrapped static assets for this mod.
///
/// 发射当前 mod 的全部 bootstrap 静态资产。
pub fn emit_all(reg: &mut Registry) -> Result<()> {
    actors_enemies_mad_dummy_enemy_ron::emit(reg)?;
    battle_chapters_demo_sequence_ron::emit(reg)?;
    battle_chapters_demo_turn_narration_sequence_ron::emit(reg)?;
    battle_rules_dialogue_test_fre_ron::emit(reg)?;
    battle_turns_cotton_bottom_wave_sequence_ron::emit(reg)?;
    battle_turns_cotton_first_turn_sequence_ron::emit(reg)?;
    battle_turns_cotton_side_pincer_sequence_ron::emit(reg)?;
    battle_turns_cotton_surround_sequence_ron::emit(reg)?;
    battle_turns_cotton_top_sweep_sequence_ron::emit(reg)?;
    battle_view_battle_bg_view_ron::emit(reg)?;
    battle_view_mad_dummy_view_ron::emit(reg)?;
    overworld_characters_frisk_character_ron::emit(reg)?;
    overworld_characters_frisk_animations_animation_config_ron::emit(reg)?;
    overworld_chase_config_ron::emit(reg)?;
    overworld_entry_sequence_ron::emit(reg)?;
    overworld_rules_demo_fre_ron::emit(reg)?;
    Ok(())
}
