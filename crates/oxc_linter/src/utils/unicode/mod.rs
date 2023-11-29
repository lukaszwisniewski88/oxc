mod is_combining_character;
mod is_emoji_modifier;
mod is_regional_indicator_symbol;
mod is_surrogate_pair;

pub use self::{
    is_combining_character::*, is_emoji_modifier::*, is_regional_indicator_symbol::*,
    is_surrogate_pair::*,
};
