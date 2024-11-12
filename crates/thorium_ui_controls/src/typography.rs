use bevy::prelude::EntityCommands;

use crate::InheritableFontSize;

pub const FONT_SIZE: f32 = 14.0;
pub const DEFAULT_FONT: &str =
    "embedded://thorium_ui_controls/assets/fonts/Fira_Sans/FiraSans-Medium.ttf";
pub const STRONG_FONT: &str =
    "embedded://thorium_ui_controls/assets/fonts/Fira_Sans/FiraSans-Bold.ttf";

/// Default text style for UI.
pub fn text_default(ent: &mut EntityCommands) {
    // ent.insert(InheritableFont(DEFAULT_FONT));
    // ss.font(DEFAULT_FONT).font_size(14);
    ent.insert(InheritableFontSize(FONT_SIZE));
}

/// When we need to emphasize a label
pub fn text_strong(ent: &mut EntityCommands) {
    // ent.insert(InheritableFont(STRONG_FONT));
    // ss.font(STRONG_FONT).font_size(14);
    ent.insert(InheritableFontSize(FONT_SIZE));
}
