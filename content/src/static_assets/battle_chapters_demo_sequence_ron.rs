//! Bootstrapped code asset for `battle/chapters/demo.sequence.ron`.
//!
//! `battle/chapters/demo.sequence.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_ron("battle/chapters/demo.sequence.ron", &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> SequenceAsset {
    SequenceAsset {
        mode: Some("battle".into()),
        rules_file: None,
        exits: vec![].into_iter().collect(),
        chapters: vec![
            Chapter::SetBgm {
                path: Some("mus_dummybattle.ogg".into()),
                fade_in: Some(0.5),
            },
            Chapter::LoadEnemies {
                enemies: vec!["actors/enemies/mad_dummy.enemy.ron".into()],
            },
            Chapter::SpawnView {
                view_layout: "battle/view/mad_dummy.view.ron".into(),
                bindings: vec![].into_iter().collect(),
            },
            Chapter::SpawnView {
                view_layout: "battle/view/battle_bg.view.ron".into(),
                bindings: vec![].into_iter().collect(),
            },
            Chapter::SpawnView {
                view_layout: "battle/view/undertale.view.ron".into(),
                bindings: vec![("enemies".into(), DataBinding::LocalLayer)]
                    .into_iter()
                    .collect(),
            },
            Chapter::AwaitFact {
                condition: "$view_rules_loaded == true".into(),
                local: true,
            },
            Chapter::EmitFactEvent {
                event_id: "battle:show_intro".into(),
                data: vec![].into_iter().collect(),
            },
            Chapter::RunSequence {
                path: Some("battle/templates/undertale_battle.sequence.ron".into()),
                path_fact: None,
                params: vec![
                    (
                        "enemy_id".into(),
                        FactValueMatch::String("mad_dummy".into()),
                    ),
                    (
                        "turn_narration".into(),
                        FactValueMatch::String(
                            "battle/chapters/demo_turn_narration.sequence.ron".into(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            },
        ],
    }
}
