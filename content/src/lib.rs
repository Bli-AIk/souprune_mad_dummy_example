//! Vessel content guest for `example_mod`.
//!
//! `example_mod` 的 Vessel 内容 guest。

use anyhow::Result;
use souprune_vessel::prelude::*;

mod support;
include!(concat!(env!("OUT_DIR"), "/vessel_content_registry.rs"));

vessel_guest! {
    fn build(reg: &mut Registry) -> Result<()> {
        reg.set_float_output_for_path(
            "battle/danmaku/cotton_first_turn.performance.ron",
            FloatOutputConfig::fixed_fractional_digits(2),
        );
        emit_all(&mut reg)
    }
}
