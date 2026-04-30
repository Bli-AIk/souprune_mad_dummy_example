//! Typed helpers for example enemy speech bubbles.
//!
//! 示例敌人对话气泡的类型化辅助函数。

use souprune_schema::battle::{
    BattleSpeechBubbleAdvance, BattleSpeechBubbleDef, BattleSpeechBubbleFrame,
};

/// Build a manual Mad Dummy speech bubble request.
///
/// 构建手动推进的 Mad Dummy 对话气泡请求。
pub fn mad_dummy_manual_speech(mortar_node: impl Into<String>) -> BattleSpeechBubbleDef {
    BattleSpeechBubbleDef {
        channel: "battle_enemy_speech".into(),
        mortar_path: "battle/enemies/mad_dummy.mortar".into(),
        mortar_node: mortar_node.into(),
        frame: BattleSpeechBubbleFrame::MadDummyWide,
        advance: BattleSpeechBubbleAdvance::Manual,
        hide_on_finish: true,
        voice: None,
        typewriter_speed: None,
    }
}

/// Build a timed Mad Dummy speech bubble request.
///
/// 构建定时结束的 Mad Dummy 对话气泡请求。
pub fn mad_dummy_timed_speech(
    mortar_node: impl Into<String>,
    duration: f32,
) -> BattleSpeechBubbleDef {
    BattleSpeechBubbleDef {
        advance: BattleSpeechBubbleAdvance::Timed { duration },
        ..mad_dummy_manual_speech(mortar_node)
    }
}
