//! Bootstrapped code asset for `actors/enemies/mad_dummy.enemy.ron`.
//!
//! `actors/enemies/mad_dummy.enemy.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::enemy::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_ron("actors/enemies/mad_dummy.enemy.ron", &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> EnemyDef {
    EnemyDef {
        id: "mad_dummy".into(),
        locale: LocaleInfo {
            name: "{{battle/ui:ENEMY_MAD_DUMMY}}".into(),
            file: "enemies".into(),
        },
        stats: CombatStats {
            hp: 300,
            attack: 7,
            defense: 999,
        },
        description: "Possesses a mannequin.".into(),
        mortar_path: "battle/enemies/mad_dummy.mortar".into(),
        acts: vec![
            ActionOption {
                label: "{{battle/ui:ACT_MAD_DUMMY_CHECK}}".into(),
                sequence: "battle/common/narration.sequence.ron".into(),
                param: "act_check".into(),
            },
            ActionOption {
                label: "{{battle/ui:ACT_MAD_DUMMY_TALK}}".into(),
                sequence: "battle/common/narration.sequence.ron".into(),
                param: "act_talk".into(),
            },
        ],
        mercies: vec![
            ActionOption {
                label: "{{battle/ui:MERCY_SPARE}}".into(),
                sequence: "battle/mercy/spare.sequence.ron".into(),
                param: "mercy_spare".into(),
            },
            ActionOption {
                label: "{{battle/ui:MERCY_FLEE}}".into(),
                sequence: "battle/mercy/end.sequence.ron".into(),
                param: "".into(),
            },
        ],
        turn_groups: vec![
            (
                "first".into(),
                TurnGroup {
                    turns: vec!["battle/turns/cotton_first_turn.sequence.ron".into()],
                    strategy: TurnStrategy::Sequential,
                },
            ),
            (
                "type2".into(),
                TurnGroup {
                    turns: vec![
                        "battle/turns/cotton_top_sweep.sequence.ron".into(),
                        "battle/turns/cotton_surround.sequence.ron".into(),
                        "battle/turns/cotton_side_pincer.sequence.ron".into(),
                        "battle/turns/cotton_bottom_wave.sequence.ron".into(),
                    ],
                    strategy: TurnStrategy::Sequential,
                },
            ),
        ]
        .into_iter()
        .collect(),
    }
}
