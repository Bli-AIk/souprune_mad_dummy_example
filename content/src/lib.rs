//! Vessel guest entry for generated content in `example_mod`.
//!
//! `example_mod` 生成内容的 Vessel guest 入口。

use anyhow::Result;
use souprune_vessel::prelude::*;

mod performances;

vessel_guest! {
    fn build(reg: &mut Registry) -> Result<()> {
        reg.performance("cotton_first_turn", performances::cotton_first_turn())?;
        Ok(())
    }
}
