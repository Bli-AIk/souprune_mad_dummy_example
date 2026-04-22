//! Vessel content guest for `example_mod`.
//!
//! `example_mod` 的 Vessel 内容 guest。

use anyhow::Result;
use souprune_vessel::prelude::*;

mod support;
include!(concat!(env!("OUT_DIR"), "/vessel_content_registry.rs"));

vessel_guest! {
    fn build(reg: &mut Registry) -> Result<()> {
        emit_all(&mut reg)
    }
}
