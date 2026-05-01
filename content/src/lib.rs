//! Cauld-ron content guest for `mad_dummy_example`.
//!
//! `mad_dummy_example` 的 Cauld-ron 内容 guest。

use anyhow::Result;
use souprune_cauld_ron::prelude::*;

mod support;
include!(concat!(env!("OUT_DIR"), "/cauld_ron_content_registry.rs"));

cauld_ron_guest! {
    fn build(reg: &mut Registry) -> Result<()> {
        reg.set_float_output_for_path(
            "battle/danmaku/cotton_first_turn.performance.ron",
            FloatOutputConfig::fixed_fractional_digits(2),
        );
        emit_all(&mut reg)
    }
}
