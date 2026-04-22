//! Auto-mapped content asset for `overworld/danmaku/demo_attack_ow.performance.ron`.
//!
//! `overworld/danmaku/demo_attack_ow.performance.ron` 的自动映射内容资产。

use anyhow::Result;
use souprune_schema::danmaku::DanmakuPerformance;
use souprune_vessel::prelude::*;

use crate::support::demo_attack;

/// Emit this asset through convention-first path mapping.
///
/// 通过“约定优先”的路径映射发射该资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> DanmakuPerformance {
    demo_attack::demo_attack_overworld()
}
