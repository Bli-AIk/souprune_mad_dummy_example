//! Battle box command helpers for the Mad Dummy example.
//!
//! Mad Dummy 示例的战斗框命令辅助函数。

use souprune_schema::sequence::Chapter;
use std::collections::HashMap;

pub(crate) fn set_battle_box_bounds(
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
    duration: f32,
) -> Chapter {
    Chapter::Custom {
        action_type: "SetBattleBoxBounds".into(),
        params: HashMap::from([
            ("id".into(), "main".into()),
            ("center_x".into(), center_x.to_string()),
            ("center_y".into(), center_y.to_string()),
            ("width".into(), width.to_string()),
            ("height".into(), height.to_string()),
            ("duration".into(), duration.to_string()),
        ]),
    }
}
